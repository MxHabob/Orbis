use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use orbis_domain::entities::group::UserGroupMembership;
use orbis_domain::entities::identity::{IdentityProvider, UserIdentity};
pub use orbis_domain::entities::identity_provisioning::{
    AccountProvisioningResult, ActorType, CredentialDeliveryStatus, DeliveryChannel,
};
use orbis_domain::entities::person::Person;
use orbis_domain::entities::user::User;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::gateways::NotificationPort;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    GroupRepository, IntegrationRepository, OrganizationRepository, PersonRepository,
    SessionRepository, UserIdentityRepository, UserRepository,
};
use crate::ports::security::PasswordHasher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionAccountInput {
    pub person_id: Uuid,
    pub actor_type: ActorType,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub roles: Vec<String>,
    pub group_ids: Vec<Uuid>,
    pub delivery_channel: Option<DeliveryChannel>,
    pub custom_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordResult {
    pub user_id: Uuid,
    pub username: String,
    pub temporary_password: Option<String>,
    pub delivery_channel: DeliveryChannel,
    pub delivery_status: CredentialDeliveryStatus,
}

pub struct IdentityProvisioningService {
    pub user_repo: Arc<dyn UserRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub user_identity_repo: Arc<dyn UserIdentityRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub integration_repo: Option<Arc<dyn IntegrationRepository>>,
    pub session_repo: Option<Arc<dyn SessionRepository>>,
    pub org_repo: Option<Arc<dyn OrganizationRepository>>,
    pub notification_port: Option<Arc<dyn NotificationPort>>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl IdentityProvisioningService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        person_repo: Arc<dyn PersonRepository>,
        user_identity_repo: Arc<dyn UserIdentityRepository>,
        group_repo: Arc<dyn GroupRepository>,
        integration_repo: Option<Arc<dyn IntegrationRepository>>,
        session_repo: Option<Arc<dyn SessionRepository>>,
        org_repo: Option<Arc<dyn OrganizationRepository>>,
        notification_port: Option<Arc<dyn NotificationPort>>,
        password_hasher: Arc<dyn PasswordHasher>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            user_repo,
            person_repo,
            user_identity_repo,
            group_repo,
            integration_repo,
            session_repo,
            org_repo,
            notification_port,
            password_hasher,
            event_bus,
            permission_checker,
        }
    }

    /// Generate a cryptographically secure random temporary password (12 characters, alphanumeric + symbols)
    pub fn generate_temporary_password() -> String {
        const CHARSET: &[u8] = b"abcdefghjkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789!@#$%&*";
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let b1 = u1.as_bytes();
        let b2 = u2.as_bytes();
        (0..12)
            .map(|i| {
                let idx = ((b1[i] as usize) ^ (b2[i + 2] as usize)) % CHARSET.len();
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Generate a normalized, deterministic, unique username based on person details
    pub async fn generate_unique_username(
        &self,
        org_id: Uuid,
        preferred: Option<&str>,
        person: &Person,
    ) -> Result<String, DomainError> {
        let base_name = if let Some(pref) = preferred {
            let clean = pref.trim().to_lowercase().replace(' ', ".");
            if !clean.is_empty() {
                clean
            } else {
                Self::fallback_username(person)
            }
        } else if let Some(ref email) = person.email {
            if let Some(user_part) = email.split('@').next() {
                user_part.to_lowercase().trim().to_string()
            } else {
                Self::fallback_username(person)
            }
        } else {
            Self::fallback_username(person)
        };

        let mut candidate = base_name.clone();
        let mut counter = 1;

        // Ensure uniqueness across existing identities
        while self
            .user_identity_repo
            .find_by_provider("password", &candidate)
            .await?
            .is_some()
            || self.user_repo.find_by_email(&candidate).await?.is_some()
        {
            candidate = format!("{}.{}", base_name, counter);
            counter += 1;
            if counter > 1000 {
                candidate = format!(
                    "{}.{}",
                    base_name,
                    Uuid::new_v4()
                        .to_string()
                        .chars()
                        .take(6)
                        .collect::<String>()
                );
                break;
            }
        }

        let _ = org_id;
        Ok(candidate)
    }

    fn fallback_username(person: &Person) -> String {
        let first = person
            .first_name
            .trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        let last = person
            .last_name
            .trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        if !first.is_empty() && !last.is_empty() {
            format!("{}.{}", first, last)
        } else if !first.is_empty() {
            first
        } else {
            format!(
                "user.{}",
                Uuid::new_v4()
                    .to_string()
                    .chars()
                    .take(8)
                    .collect::<String>()
            )
        }
    }

    /// Unified Account Provisioning Pipeline
    pub async fn provision_account(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: ProvisionAccountInput,
    ) -> Result<AccountProvisioningResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::UserWrite)
            .await?;

        // 1. Fetch Person
        let person = self
            .person_repo
            .find_by_id(input.person_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Person {} not found", input.person_id))
            })?;

        // 2. Check if user already exists for this person
        let existing_users = self.user_repo.find_by_org(org_id).await?;
        if let Some(existing) = existing_users
            .into_iter()
            .find(|u| u.person_id == person.id && !u.is_deleted)
        {
            let primary_identity = self
                .user_identity_repo
                .find_by_user(existing.id)
                .await?
                .into_iter()
                .find(|i| i.is_primary)
                .map(|i| i.provider_user_id)
                .or(existing.email)
                .unwrap_or_else(|| existing.id.to_string());

            return Ok(AccountProvisioningResult {
                user_id: existing.id,
                username: primary_identity,
                temporary_password: None,
                must_change_password: false,
                delivery_channel: DeliveryChannel::Manual,
                delivery_status: CredentialDeliveryStatus::Skipped,
                created_at: existing.created_at,
            });
        }

        // 3. Resolve username
        let username = self
            .generate_unique_username(org_id, input.preferred_username.as_deref(), &person)
            .await?;

        // 4. Resolve / Generate Temporary Password
        let raw_password = input
            .custom_password
            .unwrap_or_else(Self::generate_temporary_password);
        let password_hash = self.password_hasher.hash(&raw_password).await?;

        // 5. Create User Record
        let now = Utc::now();
        let user_id = Uuid::new_v4();
        let user_email = input.email.or(person.email.clone());
        let user_phone = input.phone.or(person.phone.clone());

        let user = User {
            id: user_id,
            person_id: person.id,
            org_id,
            email: user_email.clone(),
            is_email_verified: false,
            phone: user_phone.clone(),
            is_phone_verified: false,
            password_hash: Some(password_hash),
            is_mfa_enabled: false,
            status: "active".to_string(),
            login_count: 0,
            failed_login_attempts: 0,
            locked_until: None,
            is_deleted: false,
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.user_repo.save(&user).await?;

        // 6. Create UserIdentity for Password authentication
        let identity = UserIdentity {
            id: Uuid::new_v4(),
            user_id,
            provider: IdentityProvider::Password,
            provider_user_id: username.clone(),
            email: user_email.clone(),
            is_primary: true,
            profile_data: None,
            connected_at: now,
            last_used_at: None,
        };
        self.user_identity_repo.save(&identity).await?;

        // 7. Assign requested Groups
        for group_id in &input.group_ids {
            let membership = UserGroupMembership {
                id: Uuid::new_v4(),
                org_id,
                user_id,
                group_id: *group_id,
                created_at: now,
            };
            let _ = self.group_repo.add_member(&membership).await;
            let _ = self
                .event_bus
                .publish(
                    ctx,
                    &DomainEvent::UserGroupAssigned {
                        user_id,
                        group_id: *group_id,
                        org_id,
                        timestamp: now,
                    },
                )
                .await;
        }

        // 8. Publish Domain Event
        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::UserAccountProvisioned {
                    user_id,
                    person_id: person.id,
                    org_id,
                    actor_type: input.actor_type.to_string(),
                    timestamp: now,
                },
            )
            .await;

        // 9. Deliver credentials via Notification integrations
        let channel = input.delivery_channel.unwrap_or(DeliveryChannel::Auto);
        let (delivery_status, expose_password) = self
            .deliver_credentials(ctx, org_id, &person, &username, &raw_password, channel)
            .await;

        Ok(AccountProvisioningResult {
            user_id,
            username,
            temporary_password: if expose_password {
                Some(raw_password)
            } else {
                None
            },
            must_change_password: true,
            delivery_channel: channel,
            delivery_status,
            created_at: now,
        })
    }

    /// Reset password and deliver / return new temporary credentials
    pub async fn reset_password(
        &self,
        ctx: &RequestContext,
        user_id: Uuid,
        channel_override: Option<DeliveryChannel>,
    ) -> Result<ResetPasswordResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::UserPasswordWrite)
            .await?;

        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User {} not found", user_id)))?;

        let person = self
            .person_repo
            .find_by_id(user.person_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Person associated with user not found".into()))?;

        let identities = self.user_identity_repo.find_by_user(user.id).await?;
        let username = identities
            .into_iter()
            .find(|i| i.is_primary)
            .map(|i| i.provider_user_id)
            .or(user.email.clone())
            .unwrap_or_else(|| user.id.to_string());

        let raw_password = Self::generate_temporary_password();
        let password_hash = self.password_hasher.hash(&raw_password).await?;

        user.password_hash = Some(password_hash);
        user.failed_login_attempts = 0;
        user.locked_until = None;
        user.updated_at = Utc::now();
        self.user_repo.save(&user).await?;

        let channel = channel_override.unwrap_or(DeliveryChannel::Auto);
        let (delivery_status, expose_password) = self
            .deliver_credentials(ctx, user.org_id, &person, &username, &raw_password, channel)
            .await;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::UserPasswordReset {
                    user_id: user.id,
                    org_id: user.org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(ResetPasswordResult {
            user_id: user.id,
            username,
            temporary_password: if expose_password {
                Some(raw_password)
            } else {
                None
            },
            delivery_channel: channel,
            delivery_status,
        })
    }

    /// Enable or Disable a user account
    pub async fn toggle_user_status(
        &self,
        ctx: &RequestContext,
        user_id: Uuid,
        is_active: bool,
    ) -> Result<User, DomainError> {
        self.permission_checker
            .check(ctx, Permission::UserWrite)
            .await?;

        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User {} not found", user_id)))?;

        let old_status = user.status.clone();
        let new_status = if is_active {
            "active".to_string()
        } else {
            "disabled".to_string()
        };
        user.status = new_status.clone();
        user.updated_at = Utc::now();
        self.user_repo.save(&user).await?;

        // If disabling user, invalidate any active sessions
        if !is_active {
            if let Some(ref sr) = self.session_repo {
                // Invalidate all tokens for user
                let _ = sr.invalidate(&user_id.to_string()).await;
            }
        }

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::UserAccountStatusChanged {
                    user_id,
                    org_id: user.org_id,
                    old_status,
                    new_status,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(user)
    }

    /// Internal credential delivery helper with dynamic Integration inspection & Org policies
    async fn deliver_credentials(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        person: &Person,
        username: &str,
        temporary_password: &str,
        channel: DeliveryChannel,
    ) -> (CredentialDeliveryStatus, bool) {
        if matches!(channel, DeliveryChannel::Manual) {
            return (CredentialDeliveryStatus::Skipped, true);
        }

        // 1. Inspect organization active messaging integrations
        let (has_active_email, has_active_whatsapp, _has_active_sms) =
            if let Some(ref int_repo) = self.integration_repo {
                let configs = int_repo
                    .find_configs_by_org(org_id)
                    .await
                    .unwrap_or_default();
                let email = configs
                    .iter()
                    .any(|c| c.category == "email" && c.is_enabled);
                let wa = configs
                    .iter()
                    .any(|c| c.category == "whatsapp" && c.is_enabled);
                let sms = configs.iter().any(|c| c.category == "sms" && c.is_enabled);
                (email, wa, sms)
            } else {
                // Default to checking notification port directly if no integration repo
                (true, true, false)
            };

        let org_name = if let Some(ref o_repo) = self.org_repo {
            o_repo
                .find_by_id(org_id)
                .await
                .ok()
                .flatten()
                .map(|o| o.name)
                .unwrap_or_else(|| "Orbis School".into())
        } else {
            "Orbis School".into()
        };

        let person_name = person.full_name_ar().unwrap_or_else(|| person.full_name());
        let login_url = "https://app.orbis-edu.com/login";

        // Template message in Arabic and English
        let email_subject = format!("بيانات الدخول إلى نظام Orbis - {}", org_name);
        let email_body = format!(
            r#"<div style="font-family: sans-serif; direction: rtl; text-align: right; padding: 20px; line-height: 1.6;">
                <h2>مرحباً {}،</h2>
                <p>تم إنشاء حسابك في نظام <strong>{}</strong> بنجاح.</p>
                <div style="background: #f4f4f5; border-radius: 8px; padding: 16px; margin: 16px 0; border: 1px solid #e4e4e7;">
                    <p style="margin: 4px 0;"><strong>اسم المستخدم / البريد:</strong> <code style="background: #fff; padding: 2px 6px; border-radius: 4px;">{}</code></p>
                    <p style="margin: 4px 0;"><strong>كلمة المرور المؤقتة:</strong> <code style="background: #fff; padding: 2px 6px; border-radius: 4px; color: #d97706; font-weight: bold;">{}</code></p>
                </div>
                <p>يمكنك تسجيل الدخول وتغيير كلمة المرور عبر الرابط التالي:</p>
                <p><a href="{}" style="display: inline-block; background: #2563eb; color: #fff; padding: 10px 20px; text-decoration: none; border-radius: 6px; font-weight: bold;">تسجيل الدخول</a></p>
                <p style="color: #71717a; font-size: 12px; margin-top: 24px;">يرجى عدم مشاركة كلمة المرور المؤقتة مع أي شخص.</p>
            </div>"#,
            person_name, org_name, username, temporary_password, login_url
        );

        let whatsapp_message = format!(
            "مرحباً {}،\nتم إنشاء حسابك في نظام {}.\nاسم المستخدم: {}\nكلمة المرور المؤقتة: {}\nرابط الدخول: {}\nيرجى تغيير كلمة المرور عند أول تسجيل دخول.",
            person_name, org_name, username, temporary_password, login_url
        );

        let notification_port = match self.notification_port {
            Some(ref np) => np,
            None => return (CredentialDeliveryStatus::NotConfigured, true),
        };

        // Determine effective channels based on requested channel and active integrations
        let can_email = has_active_email
            && person
                .email
                .as_ref()
                .map_or(false, |e| !e.trim().is_empty());
        let can_whatsapp = has_active_whatsapp
            && person
                .phone
                .as_ref()
                .map_or(false, |p| !p.trim().is_empty());
        let can_sms = _has_active_sms
            && person
                .phone
                .as_ref()
                .map_or(false, |p| !p.trim().is_empty());

        let target_email = matches!(channel, DeliveryChannel::Email)
            || (matches!(channel, DeliveryChannel::Auto) && can_email);
        let target_whatsapp = matches!(channel, DeliveryChannel::WhatsApp)
            || (matches!(channel, DeliveryChannel::Auto) && !can_email && can_whatsapp);
        let target_sms = matches!(channel, DeliveryChannel::Sms)
            || (matches!(channel, DeliveryChannel::Auto) && !can_email && !can_whatsapp && can_sms);

        if target_email {
            if let Some(ref email) = person.email {
                if !email.trim().is_empty() {
                    match notification_port
                        .send_email(ctx, email.trim(), &email_subject, &email_body)
                        .await
                    {
                        Ok(_) => {
                            let _ = self
                                .event_bus
                                .publish(
                                    ctx,
                                    &DomainEvent::CredentialDeliveryAttempted {
                                        user_id: person.id,
                                        channel: "email".into(),
                                        success: true,
                                        timestamp: Utc::now(),
                                    },
                                )
                                .await;
                            return (CredentialDeliveryStatus::Delivered, false);
                        }
                        Err(e) => {
                            let _ = self
                                .event_bus
                                .publish(
                                    ctx,
                                    &DomainEvent::CredentialDeliveryAttempted {
                                        user_id: person.id,
                                        channel: "email".into(),
                                        success: false,
                                        timestamp: Utc::now(),
                                    },
                                )
                                .await;
                            // Sending failed: fallback to exposing temporary password so administrator can hand it over
                            return (CredentialDeliveryStatus::Failed(e.to_string()), true);
                        }
                    }
                }
            }
        }

        if target_whatsapp {
            if let Some(ref phone) = person.phone {
                if !phone.trim().is_empty() {
                    match notification_port
                        .send_whatsapp(ctx, phone.trim(), &whatsapp_message)
                        .await
                    {
                        Ok(_) => {
                            let _ = self
                                .event_bus
                                .publish(
                                    ctx,
                                    &DomainEvent::CredentialDeliveryAttempted {
                                        user_id: person.id,
                                        channel: "whatsapp".into(),
                                        success: true,
                                        timestamp: Utc::now(),
                                    },
                                )
                                .await;
                            return (CredentialDeliveryStatus::Delivered, false);
                        }
                        Err(e) => {
                            let _ = self
                                .event_bus
                                .publish(
                                    ctx,
                                    &DomainEvent::CredentialDeliveryAttempted {
                                        user_id: person.id,
                                        channel: "whatsapp".into(),
                                        success: false,
                                        timestamp: Utc::now(),
                                    },
                                )
                                .await;
                            return (CredentialDeliveryStatus::Failed(e.to_string()), true);
                        }
                    }
                }
            }
        }

        if target_sms {
            if let Some(ref phone) = person.phone {
                if !phone.trim().is_empty() {
                    let sms_text = format!(
                        "Orbis: تم إنشاء حسابك في {}. اسم المستخدم: {} كلمة المرور: {} الرابط: {}",
                        org_name, username, temporary_password, login_url
                    );
                    match notification_port
                        .send_sms(ctx, phone.trim(), &sms_text)
                        .await
                    {
                        Ok(_) => {
                            let _ = self
                                .event_bus
                                .publish(
                                    ctx,
                                    &DomainEvent::CredentialDeliveryAttempted {
                                        user_id: person.id,
                                        channel: "sms".into(),
                                        success: true,
                                        timestamp: Utc::now(),
                                    },
                                )
                                .await;
                            return (CredentialDeliveryStatus::Delivered, false);
                        }
                        Err(e) => {
                            let _ = self
                                .event_bus
                                .publish(
                                    ctx,
                                    &DomainEvent::CredentialDeliveryAttempted {
                                        user_id: person.id,
                                        channel: "sms".into(),
                                        success: false,
                                        timestamp: Utc::now(),
                                    },
                                )
                                .await;
                            return (CredentialDeliveryStatus::Failed(e.to_string()), true);
                        }
                    }
                }
            }
        }

        // No active delivery channel or integration configured -> Graceful fallback to Manual screen display
        (CredentialDeliveryStatus::NotConfigured, true)
    }
}
