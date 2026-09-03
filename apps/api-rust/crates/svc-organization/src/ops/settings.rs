//! Settings operations — get/update organization settings via ServiceContext.
//!
//! This module also provides the concrete `SettingsProvider` implementation
//! that other services use through the `ServiceContext`.

use async_trait::async_trait;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use tracing::info;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::settings::{
    OrgSettings, RegistrationSettings, FinanceSettings, GradingSettings,
};
use shared_kernel::traits::{
    AuditEntry, Permission, SettingsProvider,
};

use crate::contracts::commands::{
    UpdateRegistrationSettingsCmd, UpdateFinanceSettingsCmd, UpdateGradingSettingsCmd,
};
use crate::contracts::events::event_types;
use crate::entities::organization;

// ═══════════════════════════════════════════════════════════════
// Concrete SettingsProvider Implementation
// ═══════════════════════════════════════════════════════════════

/// Concrete `SettingsProvider` backed by the `organizations` table.
///
/// Reads the `settings` JSON column and deserializes into typed settings.
/// This struct is created once at startup and injected into `ServiceContext`.
pub struct OrgSettingsProvider {
    db: sea_orm::DatabaseConnection,
}

impl OrgSettingsProvider {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    /// Parse the settings JSON string from the organization row.
    pub fn parse_settings(settings_json: Option<&str>) -> OrgSettings {
        settings_json
            .and_then(|s| serde_json::from_str::<OrgSettings>(s).ok())
            .unwrap_or_default()
    }
}

#[async_trait]
impl SettingsProvider for OrgSettingsProvider {
    async fn get_org_settings(&self, org_id: &str) -> Result<OrgSettings, AppError> {
        let org = organization::Entity::find()
            .filter(organization::Column::Id.eq(org_id))
            .filter(organization::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::not_found("organization", org_id))?;

        Ok(Self::parse_settings(org.settings.as_deref()))
    }
}

// ═══════════════════════════════════════════════════════════════
// Settings Operations (via ServiceContext)
// ═══════════════════════════════════════════════════════════════

/// Get all settings for the current organization.
pub async fn get_all_settings(ctx: &ServiceContext) -> Result<OrgSettings, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;
    ctx.settings.get_org_settings(&ctx.org_id).await
}

/// Get registration settings.
pub async fn get_registration_settings(ctx: &ServiceContext) -> Result<RegistrationSettings, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;
    ctx.settings.get_registration_settings(&ctx.org_id).await
}

/// Get finance settings.
pub async fn get_finance_settings(ctx: &ServiceContext) -> Result<FinanceSettings, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;
    ctx.settings.get_finance_settings(&ctx.org_id).await
}

/// Get grading settings.
pub async fn get_grading_settings(ctx: &ServiceContext) -> Result<GradingSettings, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;
    ctx.settings.get_grading_settings(&ctx.org_id).await
}

/// Update registration settings.
pub async fn update_registration_settings(
    ctx: &ServiceContext,
    cmd: UpdateRegistrationSettingsCmd,
) -> Result<OrgSettings, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationWrite).await?;

    let mut current = ctx.settings.get_org_settings(&ctx.org_id).await?;
    let old_settings = current.clone();
    current.registration = cmd.settings;

    save_settings(ctx, &current).await?;

    // Audit
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "settings.registration.updated".to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        old_data: Some(serde_json::to_value(&old_settings.registration).unwrap_or_default()),
        new_data: Some(serde_json::to_value(&current.registration).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    // Event
    ctx.event_bus.publish(shared_kernel::event_bus::EventEnvelope {
        event_type: event_types::SETTINGS_CHANGED.to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().to_string(),
        actor_type: ctx.actor.actor_type().to_string(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({ "section": "registration" }),
        chain_depth: 0,
    });

    info!(org_id = %ctx.org_id, "Registration settings updated");
    Ok(current)
}

/// Update finance settings.
pub async fn update_finance_settings(
    ctx: &ServiceContext,
    cmd: UpdateFinanceSettingsCmd,
) -> Result<OrgSettings, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationWrite).await?;

    let mut current = ctx.settings.get_org_settings(&ctx.org_id).await?;
    let old_settings = current.clone();
    current.finance = cmd.settings;

    save_settings(ctx, &current).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "settings.finance.updated".to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        old_data: Some(serde_json::to_value(&old_settings.finance).unwrap_or_default()),
        new_data: Some(serde_json::to_value(&current.finance).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(shared_kernel::event_bus::EventEnvelope {
        event_type: event_types::SETTINGS_CHANGED.to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().to_string(),
        actor_type: ctx.actor.actor_type().to_string(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({ "section": "finance" }),
        chain_depth: 0,
    });

    info!(org_id = %ctx.org_id, "Finance settings updated");
    Ok(current)
}

/// Update grading settings.
pub async fn update_grading_settings(
    ctx: &ServiceContext,
    cmd: UpdateGradingSettingsCmd,
) -> Result<OrgSettings, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationWrite).await?;

    let mut current = ctx.settings.get_org_settings(&ctx.org_id).await?;
    let old_settings = current.clone();
    current.grading = cmd.settings;

    save_settings(ctx, &current).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "settings.grading.updated".to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        old_data: Some(serde_json::to_value(&old_settings.grading).unwrap_or_default()),
        new_data: Some(serde_json::to_value(&current.grading).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(shared_kernel::event_bus::EventEnvelope {
        event_type: event_types::SETTINGS_CHANGED.to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().to_string(),
        actor_type: ctx.actor.actor_type().to_string(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({ "section": "grading" }),
        chain_depth: 0,
    });

    info!(org_id = %ctx.org_id, "Grading settings updated");
    Ok(current)
}

// ── Internal Helper ─────────────────────────────────────────────

/// Persist settings JSON to the organization row.
async fn save_settings(ctx: &ServiceContext, settings: &OrgSettings) -> Result<(), AppError> {
    let org = organization::Entity::find()
        .filter(organization::Column::Id.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("organization", &ctx.org_id))?;

    let settings_json = serde_json::to_string(settings)
        .map_err(|e| AppError::internal(format!("Settings serialization failed: {}", e)))?;

    let mut active: organization::ActiveModel = org.into();
    active.settings = Set(Some(settings_json));
    active.updated_at = Set(chrono::Utc::now().naive_utc().to_string());
    active.update(&ctx.db).await?;

    Ok(())
}
