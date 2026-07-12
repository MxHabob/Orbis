//! Organization service — setup, branding, tenant management.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use shared_kernel::errors::AppError;
use crate::entities::organization;
use crate::repository::{LifecycleCommandRepo, LifecycleQueryRepo};

/// Organization service.
pub struct OrganizationService {
    pub cmd: Arc<LifecycleCommandRepo>,
    pub query: Arc<LifecycleQueryRepo>,
}

impl OrganizationService {
    pub fn new(cmd: Arc<LifecycleCommandRepo>, query: Arc<LifecycleQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Get organization by ID.
    pub async fn get(&self, id: &str) -> Result<organization::Model, AppError> {
        self.query.get_organization_by_id(id).await?
            .ok_or_else(|| AppError::not_found("organization", id))
    }

    /// Get organization by slug.
    pub async fn get_by_slug(&self, slug: &str) -> Result<organization::Model, AppError> {
        self.query.get_organization_by_slug(slug).await?
            .ok_or_else(|| AppError::not_found("organization", slug))
    }

    /// Create a new organization (during first registration).
    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        timezone: &str,
        locale: &str,
        currency: &str,
    ) -> Result<organization::Model, AppError> {
        // Check for duplicate slug
        if self.query.get_organization_by_slug(slug).await?.is_some() {
            return Err(AppError::conflict(format!("Slug '{}' already exists", slug)));
        }

        let now = Utc::now().naive_utc().to_string();
        let org = organization::Model {
            current_year_id: None,
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            name_ar: None,
            slug: Some(slug.to_string()),
            code: None,
            timezone: timezone.to_string(),
            locale: locale.to_string(),
            currency: currency.to_string(),
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
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.create_organization(org).await?)
    }

    /// Update organization settings.
    pub async fn update_settings(
        &self,
        id: &str,
        settings: serde_json::Value,
    ) -> Result<organization::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let existing = self.get(id).await?;
        let mut active: organization::ActiveModel = existing.into();
        active.settings = Set(Some(settings.to_string()));
        active.updated_at = Set(Utc::now().naive_utc().to_string());

        Ok(self.cmd.update_organization(active).await?)
    }
}
