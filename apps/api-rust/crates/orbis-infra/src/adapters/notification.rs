use async_trait::async_trait;
use orbis_app::{
    context::RequestContext,
    ports::{
        gateways::{NotificationPort, NotificationRequest},
        repositories::{IntegrationRepository, SettingsRepository},
    },
};
use orbis_domain::{
    entities::integration::{IntegrationConfig, IntegrationSecret},
    errors::DomainError,
};
use std::sync::Arc;

use crate::adapters::{email::EmailAdapter, whatsapp::WhatsAppAdapter};

#[async_trait]
pub trait NotificationProviderAdapter: Send + Sync {
    fn supports(&self, integration: &IntegrationConfig, channel: &str) -> bool;

    async fn send(
        &self,
        ctx: &RequestContext,
        integration: &IntegrationConfig,
        secrets: &[IntegrationSecret],
        request: &NotificationRequest,
    ) -> Result<(), DomainError>;
}

pub struct NotificationAdapter {
    pub integration_repo: Arc<dyn IntegrationRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
    providers: Vec<Arc<dyn NotificationProviderAdapter>>,
}

impl NotificationAdapter {
    pub fn new(
        integration_repo: Arc<dyn IntegrationRepository>,
        settings_repo: Arc<dyn SettingsRepository>,
    ) -> Self {
        Self {
            integration_repo,
            settings_repo,
            providers: vec![
                Arc::new(SendGridNotificationProvider),
                Arc::new(TwilioNotificationProvider),
            ],
        }
    }

    pub fn with_provider(mut self, provider: Arc<dyn NotificationProviderAdapter>) -> Self {
        self.providers.push(provider);
        self
    }

    async fn resolve_integration(
        &self,
        ctx: &RequestContext,
        channel: &str,
    ) -> Result<(IntegrationConfig, Arc<dyn NotificationProviderAdapter>), DomainError> {
        let channel = channel.trim().to_ascii_lowercase();
        if channel.is_empty() {
            return Err(DomainError::Validation(
                "Notification channel is required".into(),
            ));
        }

        let settings = self.settings_repo.get_settings(ctx.tenant_id).await?;
        let default_id = settings
            .notifications
            .default_integration_ids
            .get(&channel)
            .copied()
            .or_else(|| legacy_default_integration_id(&settings.notifications, &channel));
        let configs = self
            .integration_repo
            .find_configs_by_org(ctx.tenant_id)
            .await?;

        if let Some(default_id) = default_id {
            let integration = configs
                .into_iter()
                .find(|integration| {
                    integration.id == default_id && integration.org_id == ctx.tenant_id
                })
                .ok_or_else(|| {
                    DomainError::Validation(format!(
                        "The organization's default integration for '{}' is not connected",
                        channel
                    ))
                })?;
            if !integration.is_enabled {
                return Err(DomainError::Validation(format!(
                    "The organization's default integration for '{}' is disabled",
                    channel
                )));
            }
            let provider = self
                .providers
                .iter()
                .find(|provider| provider.supports(&integration, &channel))
                .cloned()
                .ok_or_else(|| {
                    DomainError::Validation(format!(
                        "The organization's default integration has no adapter for '{}'",
                        channel
                    ))
                })?;
            return Ok((integration, provider));
        }

        let mut eligible = configs.into_iter().filter_map(|integration| {
            if integration.org_id != ctx.tenant_id || !integration.is_enabled {
                return None;
            }
            self.providers
                .iter()
                .find(|provider| provider.supports(&integration, &channel))
                .cloned()
                .map(|provider| (integration, provider))
        });
        let selected = eligible.next().ok_or_else(|| {
            DomainError::Validation(format!(
                "No active integration with a '{}' adapter is configured",
                channel
            ))
        })?;
        if eligible.next().is_some() {
            return Err(DomainError::Validation(format!(
                "More than one '{}' provider is active. Choose the organization's default in notification settings",
                channel
            )));
        }
        Ok(selected)
    }
}

fn legacy_default_integration_id(
    settings: &orbis_domain::entities::settings::NotificationSettings,
    channel: &str,
) -> Option<uuid::Uuid> {
    match channel {
        "email" => settings.email_integration_id,
        "sms" => settings.sms_integration_id,
        "whatsapp" => settings.whatsapp_integration_id,
        _ => None,
    }
}

#[async_trait]
impl NotificationPort for NotificationAdapter {
    async fn send(
        &self,
        ctx: &RequestContext,
        request: &NotificationRequest,
    ) -> Result<(), DomainError> {
        if request.recipient.trim().is_empty() {
            return Err(DomainError::Validation(
                "Notification recipient is required".into(),
            ));
        }
        if request.body.trim().is_empty() {
            return Err(DomainError::Validation(
                "Notification body is required".into(),
            ));
        }

        let (integration, provider) = self.resolve_integration(ctx, &request.channel).await?;
        let secrets = self
            .integration_repo
            .get_secrets_by_config(integration.id)
            .await?;
        provider.send(ctx, &integration, &secrets, request).await
    }
}

struct SendGridNotificationProvider;

#[async_trait]
impl NotificationProviderAdapter for SendGridNotificationProvider {
    fn supports(&self, integration: &IntegrationConfig, channel: &str) -> bool {
        channel.eq_ignore_ascii_case("email")
            && provider_name(integration).as_deref() == Some("sendgrid")
    }

    async fn send(
        &self,
        ctx: &RequestContext,
        _integration: &IntegrationConfig,
        secrets: &[IntegrationSecret],
        request: &NotificationRequest,
    ) -> Result<(), DomainError> {
        let api_key = required_secret(secrets, &["api_key", "secret_key", "token"], "SendGrid")?;
        let subject = request.subject.as_deref().ok_or_else(|| {
            DomainError::Validation("Email notification subject is required".into())
        })?;
        EmailAdapter::new(api_key)
            .send_email(ctx, &request.recipient, subject, &request.body)
            .await
    }
}

struct TwilioNotificationProvider;

#[async_trait]
impl NotificationProviderAdapter for TwilioNotificationProvider {
    fn supports(&self, integration: &IntegrationConfig, channel: &str) -> bool {
        matches!(channel.to_ascii_lowercase().as_str(), "sms" | "whatsapp")
            && provider_name(integration).as_deref() == Some("twilio")
    }

    async fn send(
        &self,
        ctx: &RequestContext,
        integration: &IntegrationConfig,
        secrets: &[IntegrationSecret],
        request: &NotificationRequest,
    ) -> Result<(), DomainError> {
        let sid = required_secret(secrets, &["account_sid"], "Twilio")?;
        let token = required_secret(secrets, &["auth_token"], "Twilio")?;
        let from_number = integration
            .settings
            .as_ref()
            .and_then(|settings| settings.get("from_number"))
            .and_then(|value| value.as_str())
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
            .ok_or_else(|| {
                DomainError::Validation("The Twilio integration is missing a sender number".into())
            })?;
        let adapter = WhatsAppAdapter::new(sid, token, from_number);
        match request.channel.trim().to_ascii_lowercase().as_str() {
            "sms" => {
                adapter
                    .send_sms(ctx, &request.recipient, &request.body)
                    .await
            }
            "whatsapp" => {
                adapter
                    .send_whatsapp(ctx, &request.recipient, &request.body)
                    .await
            }
            _ => Err(DomainError::Validation(format!(
                "Twilio does not support '{}' notifications",
                request.channel
            ))),
        }
    }
}

fn provider_name(integration: &IntegrationConfig) -> Option<String> {
    integration
        .settings
        .as_ref()
        .and_then(|settings| settings.get("provider"))
        .and_then(|value| value.as_str())
        .filter(|provider| !provider.trim().is_empty())
        .or_else(|| Some(integration.provider_name.as_str()))
        .map(|provider| provider.trim().to_ascii_lowercase())
}

fn required_secret(
    secrets: &[IntegrationSecret],
    names: &[&str],
    provider: &str,
) -> Result<String, DomainError> {
    secrets
        .iter()
        .find(|secret| names.iter().any(|name| secret.key_name == *name))
        .map(|secret| secret.secret_key.trim().to_string())
        .filter(|secret| !secret.is_empty())
        .ok_or_else(|| {
            DomainError::Validation(format!(
                "The {} integration is missing a required credential",
                provider
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;
    use std::collections::HashMap;

    fn integration(org_id: uuid::Uuid, provider_name: &str) -> IntegrationConfig {
        IntegrationConfig {
            id: uuid::Uuid::new_v4(),
            org_id,
            provider_name: provider_name.to_string(),
            category: "communication".to_string(),
            is_enabled: true,
            env: "production".to_string(),
            settings: Some(json!({})),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn provider_adapters_declare_supported_channels_without_a_channel_enum() {
        let org_id = uuid::Uuid::new_v4();
        let twilio = integration(org_id, "sms-provider");
        let mut settings = HashMap::new();
        settings.insert("provider".to_string(), json!("twilio"));
        let twilio = IntegrationConfig {
            settings: Some(json!(settings)),
            ..twilio
        };
        let provider = TwilioNotificationProvider;
        assert!(provider.supports(&twilio, "sms"));
        assert!(provider.supports(&twilio, "whatsapp"));
        assert!(!provider.supports(&twilio, "push"));
    }

    #[test]
    fn sendgrid_provider_only_claims_email_delivery() {
        let sendgrid = integration(uuid::Uuid::new_v4(), "sendgrid");
        let provider = SendGridNotificationProvider;
        assert!(provider.supports(&sendgrid, "email"));
        assert!(!provider.supports(&sendgrid, "push"));
    }
}
