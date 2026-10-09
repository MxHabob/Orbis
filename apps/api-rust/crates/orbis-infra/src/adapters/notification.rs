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

    async fn check_connection(
        &self,
        integration: &IntegrationConfig,
        secrets: &[IntegrationSecret],
    ) -> Result<(), DomainError>;

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
        selected_integration_id: Option<uuid::Uuid>,
    ) -> Result<(IntegrationConfig, Arc<dyn NotificationProviderAdapter>), DomainError> {
        let channel = channel.trim().to_ascii_lowercase();
        if channel.is_empty() {
            return Err(DomainError::Validation(
                "Notification channel is required".into(),
            ));
        }

        let settings = self.settings_repo.get_settings(ctx.tenant_id).await?;
        let default_id = selected_integration_id.or_else(|| {
            settings
                .notifications
                .default_integration_ids
                .get(&channel)
                .copied()
                .or_else(|| legacy_default_integration_id(&settings.notifications, &channel))
        });
        let configs = self
            .integration_repo
            .find_configs_by_org(ctx.tenant_id)
            .await?;

        select_integration(
            configs,
            &self.providers,
            ctx.tenant_id,
            &channel,
            default_id,
        )
    }

    pub async fn check_integrations(
        &self,
        org_id: uuid::Uuid,
        channel: &str,
    ) -> Result<Vec<NotificationIntegrationCheck>, DomainError> {
        let channel = channel.trim().to_ascii_lowercase();
        if channel.is_empty() {
            return Err(DomainError::Validation(
                "Notification channel is required".into(),
            ));
        }

        let configs = self.integration_repo.find_configs_by_org(org_id).await?;
        let mut checks = Vec::new();
        for integration in configs
            .into_iter()
            .filter(|integration| integration.org_id == org_id)
        {
            let provider = self
                .providers
                .iter()
                .find(|provider| provider.supports(&integration, &channel))
                .cloned();

            let (is_supported, is_healthy, reason) = if !integration.is_enabled {
                (provider.is_some(), None, Some("disabled"))
            } else if let Some(provider) = provider {
                let secrets = self
                    .integration_repo
                    .get_secrets_by_config(integration.id)
                    .await?;
                match provider.check_connection(&integration, &secrets).await {
                    Ok(()) => (true, Some(true), None),
                    Err(error) => {
                        let reason = notification_check_failure_reason(&error);
                        (true, Some(false), Some(reason))
                    }
                }
            } else {
                (false, None, Some("unsupported_channel"))
            };

            checks.push(NotificationIntegrationCheck {
                id: integration.id,
                provider_name: integration.provider_name,
                channel: channel.clone(),
                is_enabled: integration.is_enabled,
                is_supported,
                is_healthy,
                is_eligible: integration.is_enabled && is_supported && is_healthy == Some(true),
                reason,
            });
        }
        Ok(checks)
    }
}

fn notification_check_failure_reason(error: &DomainError) -> &'static str {
    let error = error.to_string();
    if error.contains("missing a required credential") {
        "missing_credentials"
    } else if error.contains("sender number is required") {
        "missing_sender_number"
    } else if error.contains("HTTP 401") || error.contains("HTTP 403") {
        "provider_rejected_credentials"
    } else {
        "connection_failed"
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NotificationIntegrationCheck {
    pub id: uuid::Uuid,
    pub provider_name: String,
    pub channel: String,
    pub is_enabled: bool,
    pub is_supported: bool,
    pub is_healthy: Option<bool>,
    pub is_eligible: bool,
    pub reason: Option<&'static str>,
}

fn select_integration(
    configs: Vec<IntegrationConfig>,
    providers: &[Arc<dyn NotificationProviderAdapter>],
    org_id: uuid::Uuid,
    channel: &str,
    selected_id: Option<uuid::Uuid>,
) -> Result<(IntegrationConfig, Arc<dyn NotificationProviderAdapter>), DomainError> {
    if let Some(selected_id) = selected_id {
        let integration = configs
            .into_iter()
            .find(|integration| integration.id == selected_id && integration.org_id == org_id)
            .ok_or_else(|| {
                DomainError::Validation(format!(
                    "The selected integration for '{}' is not connected to this organization",
                    channel
                ))
            })?;
        if !integration.is_enabled {
            return Err(DomainError::Validation(format!(
                "The selected integration for '{}' is disabled",
                channel
            )));
        }
        let provider = providers
            .iter()
            .find(|provider| provider.supports(&integration, channel))
            .cloned()
            .ok_or_else(|| {
                DomainError::Validation(format!(
                    "The selected integration has no active adapter for '{}'",
                    channel
                ))
            })?;
        return Ok((integration, provider));
    }

    let mut eligible = configs.into_iter().filter_map(|integration| {
        if integration.org_id != org_id || !integration.is_enabled {
            return None;
        }
        providers
            .iter()
            .find(|provider| provider.supports(&integration, channel))
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

        let (integration, provider) = self
            .resolve_integration(ctx, &request.channel, request.integration_id)
            .await?;
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

    async fn check_connection(
        &self,
        _integration: &IntegrationConfig,
        secrets: &[IntegrationSecret],
    ) -> Result<(), DomainError> {
        let api_key = required_secret(secrets, &["api_key", "secret_key", "token"], "SendGrid")?;
        let response = reqwest::Client::new()
            .get("https://api.sendgrid.com/v3/user/profile")
            .bearer_auth(api_key)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|_| DomainError::Validation("SendGrid connection check failed".into()))?;
        if !response.status().is_success() {
            return Err(DomainError::Validation(format!(
                "SendGrid rejected connection check (HTTP {})",
                response.status().as_u16()
            )));
        }
        Ok(())
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

    async fn check_connection(
        &self,
        integration: &IntegrationConfig,
        secrets: &[IntegrationSecret],
    ) -> Result<(), DomainError> {
        let sid = required_secret(secrets, &["account_sid"], "Twilio")?;
        let token = required_secret(secrets, &["auth_token"], "Twilio")?;
        if integration
            .settings
            .as_ref()
            .and_then(|settings| settings.get("from_number"))
            .and_then(|value| value.as_str())
            .filter(|value| !value.trim().is_empty())
            .is_none()
        {
            return Err(DomainError::Validation(
                "Twilio sender number is required".into(),
            ));
        }
        let endpoint = format!("https://api.twilio.com/2010-04-01/Accounts/{sid}.json");
        let response = reqwest::Client::new()
            .get(endpoint)
            .basic_auth(sid, Some(token))
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|_| DomainError::Validation("Twilio connection check failed".into()))?;
        if !response.status().is_success() {
            return Err(DomainError::Validation(format!(
                "Twilio rejected connection check (HTTP {})",
                response.status().as_u16()
            )));
        }
        Ok(())
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

    struct TestIntegrationRepository {
        configs: Vec<IntegrationConfig>,
    }

    #[async_trait::async_trait]
    impl IntegrationRepository for TestIntegrationRepository {
        async fn find_configs_by_org(
            &self,
            org_id: uuid::Uuid,
        ) -> Result<Vec<IntegrationConfig>, DomainError> {
            Ok(self
                .configs
                .iter()
                .filter(|config| config.org_id == org_id)
                .cloned()
                .collect())
        }

        async fn get_secrets_by_config(
            &self,
            _config_id: uuid::Uuid,
        ) -> Result<Vec<IntegrationSecret>, DomainError> {
            Ok(Vec::new())
        }

        async fn save_config(&self, _config: &IntegrationConfig) -> Result<(), DomainError> {
            Ok(())
        }

        async fn save_secret(&self, _secret: &IntegrationSecret) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_config_by_provider(
            &self,
            org_id: uuid::Uuid,
            provider_name: &str,
        ) -> Result<Option<IntegrationConfig>, DomainError> {
            Ok(self
                .configs
                .iter()
                .find(|config| config.org_id == org_id && config.provider_name == provider_name)
                .cloned())
        }

        async fn delete_config(&self, _config_id: uuid::Uuid) -> Result<(), DomainError> {
            Ok(())
        }

        async fn delete_by_provider(
            &self,
            _org_id: uuid::Uuid,
            _provider_name: &str,
        ) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct TestSettingsRepository;

    #[async_trait::async_trait]
    impl SettingsRepository for TestSettingsRepository {
        async fn get_settings(
            &self,
            _org_id: uuid::Uuid,
        ) -> Result<orbis_domain::entities::settings::OrganizationSettings, DomainError> {
            Ok(orbis_domain::entities::settings::OrganizationSettings::default())
        }

        async fn save_settings(
            &self,
            _org_id: uuid::Uuid,
            _settings: &orbis_domain::entities::settings::OrganizationSettings,
        ) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct TestHealthProvider;

    #[async_trait::async_trait]
    impl NotificationProviderAdapter for TestHealthProvider {
        fn supports(&self, integration: &IntegrationConfig, channel: &str) -> bool {
            channel == "email"
                && matches!(
                    integration.provider_name.as_str(),
                    "healthy" | "rejected" | "missing"
                )
        }

        async fn check_connection(
            &self,
            integration: &IntegrationConfig,
            _secrets: &[IntegrationSecret],
        ) -> Result<(), DomainError> {
            match integration.provider_name.as_str() {
                "healthy" => Ok(()),
                "rejected" => Err(DomainError::Validation(
                    "Provider rejected connection check (HTTP 401)".into(),
                )),
                _ => Err(DomainError::Validation(
                    "Provider integration is missing a required credential".into(),
                )),
            }
        }

        async fn send(
            &self,
            _ctx: &RequestContext,
            _integration: &IntegrationConfig,
            _secrets: &[IntegrationSecret],
            _request: &NotificationRequest,
        ) -> Result<(), DomainError> {
            Ok(())
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

    #[tokio::test]
    async fn provider_connection_checks_reject_missing_credentials_without_network_access() {
        let org_id = uuid::Uuid::new_v4();
        let sendgrid = integration(org_id, "sendgrid");
        let twilio = integration(org_id, "twilio");
        let sendgrid_error = SendGridNotificationProvider
            .check_connection(&sendgrid, &[])
            .await
            .expect_err("SendGrid must require an API key");
        let twilio_error = TwilioNotificationProvider
            .check_connection(&twilio, &[])
            .await
            .expect_err("Twilio must require account credentials");

        assert_eq!(
            notification_check_failure_reason(&sendgrid_error),
            "missing_credentials"
        );
        assert_eq!(
            notification_check_failure_reason(&twilio_error),
            "missing_credentials"
        );
    }

    #[tokio::test]
    async fn integration_health_check_reports_healthy_rejected_and_missing_credentials() {
        let org_id = uuid::Uuid::new_v4();
        let repository = Arc::new(TestIntegrationRepository {
            configs: ["healthy", "rejected", "missing"]
                .into_iter()
                .map(|name| integration(org_id, name))
                .collect(),
        });
        let adapter = NotificationAdapter::new(repository, Arc::new(TestSettingsRepository))
            .with_provider(Arc::new(TestHealthProvider));

        let checks = adapter
            .check_integrations(org_id, "email")
            .await
            .expect("integration health checks should return per-provider outcomes");

        assert_eq!(checks.len(), 3);
        assert!(checks.iter().any(|check| {
            check.provider_name == "healthy" && check.is_eligible && check.is_healthy == Some(true)
        }));
        assert!(checks.iter().any(|check| {
            check.provider_name == "rejected"
                && !check.is_eligible
                && check.reason == Some("provider_rejected_credentials")
        }));
        assert!(checks.iter().any(|check| {
            check.provider_name == "missing"
                && !check.is_eligible
                && check.reason == Some("missing_credentials")
        }));
    }

    #[test]
    fn connection_failures_are_classified_without_exposing_provider_responses() {
        assert_eq!(
            notification_check_failure_reason(&DomainError::Validation(
                "provider rejected connection check (HTTP 401)".into()
            )),
            "provider_rejected_credentials"
        );
        assert_eq!(
            notification_check_failure_reason(&DomainError::Validation(
                "provider connection check failed".into()
            )),
            "connection_failed"
        );
    }

    #[test]
    fn explicitly_selected_integration_never_falls_back_to_another_provider() {
        let org_id = uuid::Uuid::new_v4();
        let configured = integration(org_id, "sendgrid");
        let requested_id = uuid::Uuid::new_v4();
        let providers: Vec<Arc<dyn NotificationProviderAdapter>> =
            vec![Arc::new(SendGridNotificationProvider)];

        let error = select_integration(
            vec![configured],
            &providers,
            org_id,
            "email",
            Some(requested_id),
        )
        .err()
        .expect("a missing explicit integration must be rejected");

        assert!(error.to_string().contains("not connected"));
    }

    #[test]
    fn selected_integration_must_belong_to_the_request_organization_and_support_channel() {
        let org_id = uuid::Uuid::new_v4();
        let other_org_id = uuid::Uuid::new_v4();
        let foreign = integration(other_org_id, "sendgrid");
        let unsupported = integration(org_id, "mail-provider");
        let providers: Vec<Arc<dyn NotificationProviderAdapter>> =
            vec![Arc::new(SendGridNotificationProvider)];

        let foreign_error = select_integration(
            vec![foreign.clone()],
            &providers,
            org_id,
            "email",
            Some(foreign.id),
        )
        .err()
        .expect("foreign integration must be rejected");
        assert!(foreign_error.to_string().contains("not connected"));

        let unsupported_error = select_integration(
            vec![unsupported.clone()],
            &providers,
            org_id,
            "email",
            Some(unsupported.id),
        )
        .err()
        .expect("integration without a registered channel adapter must be rejected");
        assert!(unsupported_error.to_string().contains("no active adapter"));
    }
}
