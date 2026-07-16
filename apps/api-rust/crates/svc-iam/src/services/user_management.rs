//! User management service — CRUD operations for users.
//!
//! Uses `AppError` for structured errors and CQRS repos.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use shared_kernel::errors::AppError;
use crate::entities::user;
use crate::repository::{IamCommandRepo, IamQueryRepo};

/// User management service.
pub struct UserService {
    pub cmd: Arc<IamCommandRepo>,
    pub query: Arc<IamQueryRepo>,
}

impl UserService {
    pub fn new(cmd: Arc<IamCommandRepo>, query: Arc<IamQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Get a user by ID.
    pub async fn get_user(&self, id: &str) -> Result<user::Model, AppError> {
        self.query.get_user_by_id(id).await?
            .ok_or_else(|| AppError::not_found("user", id))
    }

    /// List users in an organization with optional role filter.
    pub async fn list_users(
        &self,
        org_id: &str,
        role: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<user::Model>, AppError> {
        Ok(self.query.list_users_by_org(org_id, role, limit, offset).await?)
    }

    /// Create a new user.
    pub async fn create_user(
        &self,
        org_id: &str,
        email: Option<&str>,
        full_name: &str,
        role: &str,
        password_hash: Option<String>,
    ) -> Result<user::Model, AppError> {
        // Check for duplicate email
        if let Some(email) = email {
            if self.query.get_user_by_email(email).await?.is_some() {
                return Err(AppError::conflict(format!("Email '{}' already registered", email)));
            }
        }

        let now = Utc::now().to_rfc3339();
        let new_user = user::Model {
            id: Uuid::new_v4().to_string(),
            party_id: None,
            org_id: org_id.to_string(),
            email: email.map(String::from),
            email_verified: 0,
            email_verified_at: None,
            phone: None,
            phone_verified: 0,
            phone_verified_at: None,
            password_hash,
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
            primary_role: role.to_string(),
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
            created_at: now.clone(),
            updated_at: now,
            created_by: None,
        };

        Ok(self.cmd.create_user(new_user).await?)
    }

    /// Soft-delete a user.
    pub async fn soft_delete(
        &self,
        user_id: &str,
        deleted_by: &str,
    ) -> Result<user::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let existing = self.get_user(user_id).await?;
        let now = Utc::now().to_rfc3339();

        let mut active: user::ActiveModel = existing.into();
        active.is_deleted = Set(1);
        active.deleted_at = Set(Some(now.clone()));
        active.deleted_by = Set(Some(deleted_by.to_string()));
        active.updated_at = Set(now);

        Ok(self.cmd.update_user(active).await?)
    }
}
