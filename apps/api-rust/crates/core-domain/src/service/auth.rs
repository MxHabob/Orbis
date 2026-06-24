use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entities::{user, organization, user_session};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginResponse {
    #[schema(value_type = User)]
    pub user: user::Model,
    #[serde(rename = "access_token")]
    pub token: String,
    pub refresh_token: String,
}

impl super::Service {
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        full_name: &str,
        org_slug: &str,
    ) -> Result<user::Model, String> {
        let email_owned = email.to_string();

        // 1. Check if organization exists, if not create one
        let org = self.repo.get_organization_by_slug(org_slug).await
            .map_err(|e| e.to_string())?;

        let org_id = match org {
            Some(o) => o.id,
            None => {
                let new_org = organization::Model {
                    id: Uuid::new_v4().to_string(),
                    name: format!("{} School", org_slug),
                    name_ar: None,
                    slug: org_slug.to_string(),
                    code: None,
                    timezone: "Asia/Aden".to_string(),
                    locale: "ar-YE".to_string(),
                    currency: "YER".to_string(),
                    date_format: Some("DD/MM/YYYY".to_string()),
                    hijri_enabled: 1,
                    max_users: 1000,
                    max_students: 5000,
                    max_storage_gb: 100,
                    settings: Some("{}".to_string()),
                    features: Some("{}".to_string()),
                    subscription_plan: "free".to_string(),
                    subscription_expires_at: None,
                    logo: None,
                    favicon: None,
                    primary_color: Some("#6366f1".to_string()),
                    is_active: 1,
                    is_verified: 0,
                    is_deleted: 0,
                    deleted_at: None,
                    deleted_by: None,
                    created_at: Utc::now().naive_utc().to_string(),
                    updated_at: Utc::now().naive_utc().to_string(),
                };
                let created_org = self.repo.create_organization(new_org).await
                    .map_err(|e| e.to_string())?;
                created_org.id
            }
        };

        // 2. Check if user already exists
        let existing = self.repo.get_user_by_email(email).await
            .map_err(|e| e.to_string())?;
        if existing.is_some() {
            return Err("email already registered".to_string());
        }

        // 3. Hash password
        let pwd_hash = hash(password, DEFAULT_COST)
            .map_err(|e| e.to_string())?;

        // 4. Create user
        let new_user = user::Model {
            id: Uuid::new_v4().to_string(),
            org_id,
            email: Some(email_owned),
            email_verified: 0,
            email_verified_at: None,
            phone: None,
            phone_verified: 0,
            phone_verified_at: None,
            password_hash: Some(pwd_hash),
            password_changed_at: None,
            password_expires_at: None,
            mfa_enabled: 0,
            mfa_secret: None,
            mfa_backup_codes: None,
            full_name: full_name.to_string(),
            name_ar: None,
            name_en: None,
            avatar: None,
            sex: None,
            date_of_birth: None,
            place_of_birth: None,
            nationality: None,
            national_id: None,
            passport_number: None,
            address: None,
            city: None,
            country: None,
            postal_code: None,
            emergency_contact: None,
            blood_type: None,
            allergies: None,
            medical_conditions: None,
            medical_notes: None,
            social_links: None,
            bio: None,
            primary_role: "admin".to_string(),
            status: "active".to_string(),
            last_login_at: None,
            last_login_ip: None,
            login_count: Some(0),
            failed_logins: Some(0),
            locked_until: None,
            prefs: "{}".to_string(),
            meta: "{}".to_string(),
            is_deleted: 0,
            deleted_at: None,
            deleted_by: None,
            created_at: Utc::now().naive_utc().to_string(),
            updated_at: Utc::now().naive_utc().to_string(),
            created_by: None,
        };

        self.repo.create_user(new_user).await.map_err(|e| e.to_string())
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, String> {
        // 1. Get user by email
        let user = self.repo.get_user_by_email(email).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "invalid email or password".to_string())?;

        // 2. Verify password
        let hash_str = user.password_hash.as_ref()
            .ok_or_else(|| "password authentication not supported for this user".to_string())?;
        
        let valid = verify(password, hash_str)
            .map_err(|e| e.to_string())?;
        if !valid {
            return Err("invalid email or password".to_string());
        }

        // 3. Generate tokens
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let exp = now + (self.cfg.jwt_expiry_hours as usize * 3600);
        let refresh_exp = now + (self.cfg.jwt_refresh_days as usize * 24 * 3600);

        let claims = Claims {
            sub: user.id.clone(),
            org_id: user.org_id.clone(),
            role: user.primary_role.clone(),
            exp,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.cfg.jwt_secret.as_bytes()),
        ).map_err(|e| e.to_string())?;

        let refresh_claims = Claims {
            sub: user.id.clone(),
            org_id: user.org_id.clone(),
            role: user.primary_role.clone(),
            exp: refresh_exp,
            iat: now,
        };

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.cfg.jwt_secret.as_bytes()),
        ).map_err(|e| e.to_string())?;

        // 4. Store user session
        let session_id = Uuid::new_v4().to_string();
        let expires_at_str = Utc::now()
            .checked_add_signed(chrono::Duration::hours(self.cfg.jwt_expiry_hours))
            .unwrap()
            .to_rfc3339();
        
        let refresh_expires_at_str = Utc::now()
            .checked_add_signed(chrono::Duration::days(self.cfg.jwt_refresh_days))
            .unwrap()
            .to_rfc3339();

        let session = user_session::Model {
            id: session_id,
            user_id: user.id.clone(),
            token: token.clone(),
            refresh_token: Some(refresh_token.clone()),
            device_id: None,
            device_type: None,
            device_name: None,
            os: None,
            browser: None,
            ip_address: None,
            user_agent: None,
            is_current: 1,
            is_trusted: 0,
            mfa_verified: 0,
            expires_at: expires_at_str,
            refresh_expires_at: Some(refresh_expires_at_str),
            last_active_at: Some(Utc::now().to_rfc3339()),
            created_at: Utc::now().to_rfc3339(),
        };

        self.repo.create_session(session).await.map_err(|e| e.to_string())?;

        // 5. Audit Log (mock/ignore audit fail)
        let audit = crate::entities::audit_log::Model {
            id: Uuid::new_v4().to_string(),
            org_id: user.org_id.clone(),
            user_id: Some(user.id.clone()),
            action: "user.login".to_string(),
            entity_type: "user".to_string(),
            entity_id: user.id.clone(),
            old_data: None,
            new_data: None,
            ip_address: None,
            user_agent: None,
            session_id: Some(user.id.clone()), // mapping to session
            metadata: "{}".to_string(),
            created_at: Utc::now().to_rfc3339(),
        };
        let _ = self.repo.create_audit_log(audit).await;

        Ok(LoginResponse {
            user,
            token,
            refresh_token,
        })
    }

    pub async fn validate_token(&self, token_str: &str) -> Result<user::Model, String> {
        // 1. Check session in database
        self.repo.get_session_by_token(token_str).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "session not found or inactive".to_string())?;

        // 2. Parse and validate JWT
        let token_data = decode::<Claims>(
            token_str,
            &DecodingKey::from_secret(self.cfg.jwt_secret.as_bytes()),
            &Validation::default(),
        ).map_err(|e| format!("invalid or expired token: {}", e))?;

        // 3. Fetch user
        let user = self.repo.get_user_by_id(&token_data.claims.sub).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "user not found".to_string())?;

        Ok(user)
    }

    pub async fn logout(&self, token_str: &str) -> Result<(), String> {
        self.repo.invalidate_session(token_str).await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
