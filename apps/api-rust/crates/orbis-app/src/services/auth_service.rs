use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use orbis_domain::entities::user::{User, Role};
use orbis_domain::entities::person::Person;
use orbis_domain::entities::session::Session;
use orbis_domain::entities::mfa::{MfaFactor, MfaType};
use orbis_domain::entities::device::{UserDevice, DeviceType};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::{
    UserRepository, PersonRepository, SessionRepository, MfaFactorRepository,
    MfaRecoveryCodeRepository, UserDeviceRepository, UserIdentityRepository,
};
use crate::ports::event_bus::EventBus;
use crate::ports::security::{TokenService, PasswordHasher};
use crate::ports::iam::PermissionCheckerPort;

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: User,
}

pub struct AuthService {
    pub user_repo: Arc<dyn UserRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub mfa_factor_repo: Arc<dyn MfaFactorRepository>,
    pub mfa_recovery_code_repo: Arc<dyn MfaRecoveryCodeRepository>,
    pub user_device_repo: Arc<dyn UserDeviceRepository>,
    pub user_identity_repo: Arc<dyn UserIdentityRepository>,
    pub token_service: Arc<dyn TokenService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        person_repo: Arc<dyn PersonRepository>,
        session_repo: Arc<dyn SessionRepository>,
        mfa_factor_repo: Arc<dyn MfaFactorRepository>,
        mfa_recovery_code_repo: Arc<dyn MfaRecoveryCodeRepository>,
        user_device_repo: Arc<dyn UserDeviceRepository>,
        user_identity_repo: Arc<dyn UserIdentityRepository>,
        token_service: Arc<dyn TokenService>,
        password_hasher: Arc<dyn PasswordHasher>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            user_repo,
            person_repo,
            session_repo,
            mfa_factor_repo,
            mfa_recovery_code_repo,
            user_device_repo,
            user_identity_repo,
            token_service,
            password_hasher,
            event_bus,
            permission_checker,
        }
    }

    /// Authenticate user via email and password, creating session and JWT tokens
    pub async fn login(
        &self,
        ctx: &RequestContext,
        email: &str,
        password: &str,
    ) -> Result<LoginResponse, DomainError> {
        let user = self.user_repo.find_by_email(email).await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid email or password".into()))?;

        let hash = user.password_hash.as_ref()
            .ok_or_else(|| DomainError::Unauthorized("Password authentication not supported for this user".into()))?;

        let is_valid = self.password_hasher.verify(password, hash).await?;
        if !is_valid {
            return Err(DomainError::Unauthorized("Invalid email or password".into()));
        }

        let user_id_str = user.id.to_string();
        let org_id_str = user.org_id.to_string();
        let role_str = "org_admin".to_string();

        let access_token = self.token_service.generate_access_token(&user_id_str, &org_id_str, &role_str).await?;
        let refresh_token = self.token_service.generate_refresh_token(&user_id_str, &org_id_str, &role_str).await?;

        let now = Utc::now();
        let session_id = Uuid::new_v4();

        let session = Session {
            id: session_id,
            user_id: user.id,
            token: access_token.clone(),
            refresh_token: Some(refresh_token.clone()),
            device_id: None,
            family_id: None,
            is_revoked: false,
            is_current: true,
            is_trusted: false,
            mfa_verified: false,
            expires_at: now + chrono::Duration::hours(2),
            refresh_expires_at: Some(now + chrono::Duration::days(7)),
            last_active_at: Some(now),
            created_at: now,
        };

        self.session_repo.save(&session).await?;

        let event = DomainEvent::UserLoggedIn {
            user_id: user.id,
            org_id: user.org_id,
            session_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(LoginResponse {
            token: access_token,
            refresh_token,
            user,
        })
    }

    /// Register a new user in the organization
    pub async fn register_user(
        &self,
        ctx: &RequestContext,
        full_name: String,
        email: Option<String>,
        password_plain: Option<String>,
        _role: Role,
        org_id: Option<Uuid>,
    ) -> Result<User, DomainError> {
        let actual_org_id = org_id.unwrap_or(ctx.tenant_id);

        if let Some(ref e) = email {
            if self.user_repo.find_by_email(e).await?.is_some() {
                return Err(DomainError::Conflict(format!("Email '{}' already registered", e)));
            }
        }

        let now = Utc::now();
        let person_id = Uuid::new_v4();

        let name_parts: Vec<&str> = full_name.split_whitespace().collect();
        let first_name = name_parts.first().unwrap_or(&"").to_string();
        let last_name = if name_parts.len() > 1 {
            name_parts[1..].join(" ")
        } else {
            first_name.clone()
        };

        let person = Person {
            id: person_id,
            org_id: Some(actual_org_id),
            first_name,
            last_name,
            full_name: Some(full_name.clone()),
            name_ar: None,
            national_id: None,
            passport_number: None,
            gender: None,
            date_of_birth: None,
            place_of_birth: None,
            nationality: None,
            blood_type: None,
            medical_notes: None,
            special_needs_notes: None,
            phone: None,
            email: email.clone(),
            address: None,
            created_at: now,
            updated_at: now,
        };
        self.person_repo.save(&person).await?;

        let password_hash = match password_plain {
            Some(p) => Some(self.password_hasher.hash(&p).await?),
            None => None,
        };

        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            person_id,
            org_id: actual_org_id,
            email,
            email_verified: false,
            phone: None,
            phone_verified: false,
            password_hash,
            mfa_enabled: false,
            full_name,
            status: "active".to_string(),
            login_count: 0,
            failed_logins: 0,
            locked_until: None,
            is_deleted: false,
            created_at: now,
            updated_at: now,
        };

        self.user_repo.save(&user).await?;

        let event = DomainEvent::UserCreated {
            user_id,
            person_id,
            org_id: actual_org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(user)
    }

    /// Get user by ID
    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        self.user_repo.find_by_id(id).await
    }

    /// List users in an organization
    pub async fn list_users(&self, org_id: Uuid) -> Result<Vec<User>, DomainError> {
        self.user_repo.find_by_org(org_id).await
    }

    /// Update user details
    pub async fn update_user(
        &self,
        ctx: &RequestContext,
        user_id: Uuid,
        full_name: Option<String>,
    ) -> Result<User, DomainError> {
        self.permission_checker.check(ctx, Permission::UserWrite).await?;

        let mut user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("User {} not found", user_id)))?;

        if let Some(name) = full_name {
            user.full_name = name;
        }
        user.updated_at = Utc::now();
        self.user_repo.save(&user).await?;

        Ok(user)
    }

    /// Invalidate session / Logout
    pub async fn logout(&self, token: &str) -> Result<(), DomainError> {
        self.session_repo.invalidate(token).await
    }

    /// Enroll MFA factor
    pub async fn enroll_mfa_totp(
        &self,
        _ctx: &RequestContext,
        user_id: Uuid,
        secret: String,
    ) -> Result<MfaFactor, DomainError> {
        let now = Utc::now();
        let factor = MfaFactor {
            id: Uuid::new_v4(),
            user_id,
            factor_type: MfaType::Totp,
            factor_name: "Authenticator App".into(),
            secret_encrypted: secret,
            is_verified: false,
            is_default: true,
            created_at: now,
            last_used_at: None,
        };
        self.mfa_factor_repo.save(&factor).await?;
        Ok(factor)
    }

    /// List user devices
    pub async fn list_devices(&self, user_id: Uuid) -> Result<Vec<UserDevice>, DomainError> {
        self.user_device_repo.find_by_user(user_id).await
    }

    /// Register or update a user device
    pub async fn register_device(
        &self,
        user_id: Uuid,
        device_id: String,
        device_name: String,
        device_type: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<UserDevice, DomainError> {
        let now = Utc::now();
        let dev_type = DeviceType::from(device_type.as_str());
        let device = match self.user_device_repo.find_by_device_id(user_id, &device_id).await? {
            Some(mut d) => {
                d.device_name = device_name;
                d.client_ip = ip_address;
                d.last_active_at = now;
                self.user_device_repo.save(&d).await?;
                d
            }
            None => {
                let d = UserDevice {
                    id: Uuid::new_v4(),
                    user_id,
                    device_id: device_id.clone(),
                    device_name,
                    device_type: dev_type,
                    os: None,
                    browser: None,
                    client_ip: ip_address,
                    user_agent,
                    fingerprint_hash: device_id,
                    is_trusted: true,
                    trusted_until: None,
                    first_seen_at: now,
                    last_active_at: now,
                };
                self.user_device_repo.save(&d).await?;
                d
            }
        };
        Ok(device)
    }
}
