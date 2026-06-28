use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entities::user;
use crate::repository::user_mgmt::UserUpdate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub national_id: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub emergency_contact: Option<String>,
    pub blood_type: Option<String>,
    pub medical_conditions: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChangeRoleRequest {
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub name_ar: Option<String>,
    pub primary_role: String,
    pub sex: Option<String>,
    pub phone: Option<String>,
    pub national_id: Option<String>,
    pub date_of_birth: Option<String>,
}

impl super::Service {
    pub async fn get_users(
        &self,
        org_id: &str,
        params: crate::params::UserParams,
    ) -> Result<Vec<user::Model>, String> {
        self.repo.get_users(org_id, params).await
            .map_err(|e| e.to_string())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<user::Model, String> {
        self.repo.get_user_by_id(user_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "user not found".to_string())
    }

    pub async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<crate::entities::user_session::Model>, String> {
        self.repo.get_user_sessions(user_id).await
            .map_err(|e| e.to_string())
    }

    pub async fn create_user_in_org(&self, org_id: &str, req: CreateUserRequest, created_by: &str) -> Result<user::Model, String> {
        // Check if user email already exists
        let existing = self.repo.get_user_by_email(&req.email).await
            .map_err(|e| e.to_string())?;
        if existing.is_some() {
            return Err("email already registered".to_string());
        }

        // Validate role
        let valid_roles = ["admin", "super_admin", "principal", "teacher", "student", "parent", "staff"];
        if !valid_roles.contains(&req.primary_role.as_str()) {
            return Err(format!("invalid role: {}. Valid roles: {:?}", req.primary_role, valid_roles));
        }

        let pwd_hash = hash(&req.password, DEFAULT_COST)
            .map_err(|e| e.to_string())?;

        let new_user = user::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            email: Some(req.email),
            email_verified: 0,
            email_verified_at: None,
            phone: req.phone,
            phone_verified: 0,
            phone_verified_at: None,
            password_hash: Some(pwd_hash),
            password_changed_at: None,
            password_expires_at: None,
            mfa_enabled: 0,
            mfa_secret: None,
            mfa_backup_codes: None,
            full_name: req.full_name,
            name_ar: req.name_ar,
            name_en: None,
            avatar: None,
            sex: req.sex,
            date_of_birth: req.date_of_birth,
            place_of_birth: None,
            nationality: None,
            national_id: req.national_id,
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
            primary_role: req.primary_role,
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
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            created_by: Some(created_by.to_string()),
        };

        self.repo.create_user(new_user).await
            .map_err(|e| e.to_string())
    }

    pub async fn update_user(&self, user_id: &str, req: UpdateUserRequest) -> Result<user::Model, String> {
        let updates = UserUpdate {
            full_name: req.full_name,
            name_ar: req.name_ar,
            name_en: req.name_en,
            email: req.email,
            phone: req.phone,
            sex: req.sex,
            date_of_birth: req.date_of_birth,
            nationality: req.nationality,
            national_id: req.national_id,
            address: req.address,
            city: req.city,
            country: req.country,
            emergency_contact: req.emergency_contact,
            blood_type: req.blood_type,
            medical_conditions: req.medical_conditions,
            avatar: req.avatar,
            status: req.status,
            ..Default::default()
        };

        self.repo.update_user(user_id, updates).await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_user(&self, user_id: &str, deleted_by: &str) -> Result<(), String> {
        let affected = self.repo.soft_delete_user(user_id, deleted_by).await
            .map_err(|e| e.to_string())?;
        if affected == 0 {
            return Err("user not found or already deleted".to_string());
        }
        Ok(())
    }

    pub async fn change_user_role(&self, user_id: &str, role: &str) -> Result<user::Model, String> {
        let valid_roles = ["admin", "super_admin", "principal", "teacher", "student", "parent", "staff"];
        if !valid_roles.contains(&role) {
            return Err(format!("invalid role: {}. Valid roles: {:?}", role, valid_roles));
        }

        let updates = UserUpdate {
            primary_role: Some(role.to_string()),
            ..Default::default()
        };

        self.repo.update_user(user_id, updates).await
            .map_err(|e| e.to_string())
    }

    pub async fn change_user_password(&self, user_id: &str, new_password: &str) -> Result<user::Model, String> {
        if new_password.len() < 6 {
            return Err("password must be at least 6 characters".to_string());
        }

        let pwd_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| e.to_string())?;

        let updates = UserUpdate {
            password_hash: Some(pwd_hash),
            ..Default::default()
        };

        self.repo.update_user(user_id, updates).await
            .map_err(|e| e.to_string())
    }

    pub async fn get_user_roles(&self, user_id: &str) -> Result<Vec<crate::entities::user_role::Model>, String> {
        self.repo.get_user_roles(user_id).await.map_err(|e| e.to_string())
    }

    pub async fn save_user_roles(&self, user_id: &str, roles: Vec<String>) -> Result<(), String> {
        let u = self.repo.get_user_by_id(user_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "User not found".to_string())?;

        self.repo.save_user_roles(user_id, &u.org_id, roles).await
            .map_err(|e| e.to_string())
    }
}

