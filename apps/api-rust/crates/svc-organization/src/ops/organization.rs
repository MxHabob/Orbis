//! Organization operations — CRUD via ServiceContext.

use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use tracing::info;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::{CreateOrganizationCmd, UpdateOrganizationCmd};
use crate::contracts::responses::OrganizationResponse;
use crate::contracts::events::event_types;
use crate::entities::organization;
use crate::ops::settings::OrgSettingsProvider;

/// Get organization by ID.
pub async fn get_organization(ctx: &ServiceContext) -> Result<OrganizationResponse, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;

    let org = organization::Entity::find()
        .filter(organization::Column::Id.eq(&ctx.org_id))
        .filter(organization::Column::IsDeleted.eq(0))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("organization", &ctx.org_id))?;

    let settings = OrgSettingsProvider::parse_settings(org.settings.as_deref());

    Ok(OrganizationResponse {
        id: org.id,
        name: org.name,
        name_ar: org.name_ar,
        slug: org.slug.unwrap_or_default(),
        timezone: org.timezone,
        locale: org.locale,
        currency: org.currency,
        settings,
        is_active: org.is_active == 1,
        is_verified: org.is_verified == 1,
    })
}

/// Create a new organization.
pub async fn create_organization(
    ctx: &ServiceContext,
    cmd: CreateOrganizationCmd,
) -> Result<OrganizationResponse, AppError> {
    // Check for duplicate slug
    let existing = organization::Entity::find()
        .filter(organization::Column::Slug.eq(&cmd.slug))
        .filter(organization::Column::IsDeleted.eq(0))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::conflict(format!("Slug '{}' already exists", cmd.slug)));
    }

    let now = chrono::Utc::now().naive_utc().to_string();
    let org_id = uuid::Uuid::new_v4().to_string();

    let org = organization::ActiveModel {
        current_year_id: Set(None),
        id: Set(org_id.clone()),
        name: Set(cmd.name),
        name_ar: Set(None),
        slug: Set(Some(cmd.slug)),
        code: Set(None),
        timezone: Set(cmd.timezone),
        locale: Set(cmd.locale),
        currency: Set(cmd.currency),
        date_format: Set(Some("DD/MM/YYYY".to_string())),
        hijri_enabled: Set(1),
        max_users: Set(1000),
        max_students: Set(5000),
        max_storage_gb: Set(100),
        settings: Set(Some("{}".to_string())),
        features: Set(Some("{}".to_string())),
        subscription_plan: Set("free".to_string()),
        subscription_expires_at: Set(None),
        logo: Set(None),
        favicon: Set(None),
        primary_color: Set(Some("#6366f1".to_string())),
        is_active: Set(1),
        is_verified: Set(0),
        is_deleted: Set(0),
        deleted_at: Set(None),
        deleted_by: Set(None),
        created_at: Set(now.clone()),
        updated_at: Set(now),
    };

    let inserted = org.insert(&ctx.db).await?;

    // Audit
    ctx.audit.log(AuditEntry {
        org_id: org_id.clone(),
        actor: ctx.actor.clone(),
        action: "organization.created".to_string(),
        entity_type: "organization".to_string(),
        entity_id: org_id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&inserted).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    // Event
    ctx.event_bus.publish(shared_kernel::event_bus::EventEnvelope {
        event_type: event_types::ORGANIZATION_CREATED.to_string(),
        entity_type: "organization".to_string(),
        entity_id: org_id.clone(),
        org_id: org_id.clone(),
        actor: ctx.actor.as_str().to_string(),
        actor_type: ctx.actor.actor_type().to_string(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&inserted).unwrap_or_default(),
        chain_depth: 0,
    });

    info!(org_id = %org_id, "Organization created");

    Ok(OrganizationResponse {
        id: inserted.id,
        name: inserted.name,
        name_ar: inserted.name_ar,
        slug: inserted.slug.unwrap_or_default(),
        timezone: inserted.timezone,
        locale: inserted.locale,
        currency: inserted.currency,
        settings: Default::default(),
        is_active: true,
        is_verified: false,
    })
}

/// Update organization basic info.
pub async fn update_organization(
    ctx: &ServiceContext,
    cmd: UpdateOrganizationCmd,
) -> Result<OrganizationResponse, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationWrite).await?;

    let org = organization::Entity::find()
        .filter(organization::Column::Id.eq(&ctx.org_id))
        .filter(organization::Column::IsDeleted.eq(0))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("organization", &ctx.org_id))?;

    let old_data = serde_json::to_value(&org).unwrap_or_default();
    let mut active: organization::ActiveModel = org.into();

    if let Some(name) = cmd.name { active.name = Set(name); }
    if let Some(name_ar) = cmd.name_ar { active.name_ar = Set(Some(name_ar)); }
    if let Some(tz) = cmd.timezone { active.timezone = Set(tz); }
    if let Some(loc) = cmd.locale { active.locale = Set(loc); }
    if let Some(cur) = cmd.currency { active.currency = Set(cur); }
    if let Some(df) = cmd.date_format { active.date_format = Set(Some(df)); }
    if let Some(h) = cmd.hijri_enabled { active.hijri_enabled = Set(if h { 1 } else { 0 }); }
    if let Some(logo) = cmd.logo { active.logo = Set(Some(logo)); }
    if let Some(favicon) = cmd.favicon { active.favicon = Set(Some(favicon)); }
    if let Some(color) = cmd.primary_color { active.primary_color = Set(Some(color)); }

    active.updated_at = Set(chrono::Utc::now().naive_utc().to_string());
    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "organization.updated".to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        old_data: Some(old_data),
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(shared_kernel::event_bus::EventEnvelope {
        event_type: event_types::ORGANIZATION_UPDATED.to_string(),
        entity_type: "organization".to_string(),
        entity_id: ctx.org_id.clone(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().to_string(),
        actor_type: ctx.actor.actor_type().to_string(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    info!(org_id = %ctx.org_id, "Organization updated");

    let settings = OrgSettingsProvider::parse_settings(updated.settings.as_deref());

    Ok(OrganizationResponse {
        id: updated.id,
        name: updated.name,
        name_ar: updated.name_ar,
        slug: updated.slug.unwrap_or_default(),
        timezone: updated.timezone,
        locale: updated.locale,
        currency: updated.currency,
        settings,
        is_active: updated.is_active == 1,
        is_verified: updated.is_verified == 1,
    })
}

pub async fn get(ctx: &ServiceContext) -> Result<OrganizationResponse, AppError> {
    get_organization(ctx).await
}

pub async fn get_main(db: &sea_orm::DatabaseConnection) -> Result<organization::Model, AppError> {
    let first = organization::Entity::find()
        .filter(organization::Column::IsDeleted.eq(0))
        .one(db)
        .await?;
    if let Some(org) = first {
        Ok(org)
    } else {
        Err(AppError::not_found("organization", "main"))
    }
}

pub async fn update(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<OrganizationResponse, AppError> {
    let cmd: UpdateOrganizationCmd = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update organization payload: {e}")))?;
    update_organization(ctx, cmd).await
}

pub async fn get_settings(ctx: &ServiceContext) -> Result<serde_json::Value, AppError> {
    let org = organization::Entity::find()
        .filter(organization::Column::Id.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("organization", &ctx.org_id))?;
    let settings = org.settings.unwrap_or_else(|| "{}".to_string());
    let val: serde_json::Value = serde_json::from_str(&settings)
        .unwrap_or_else(|_| serde_json::json!({}));
    Ok(val)
}

pub async fn update_settings(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationWrite).await?;
    let org = organization::Entity::find()
        .filter(organization::Column::Id.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("organization", &ctx.org_id))?;
    let mut active: organization::ActiveModel = org.into();
    active.settings = Set(Some(data.to_string()));
    active.updated_at = Set(chrono::Utc::now().naive_utc().to_string());
    active.update(&ctx.db).await?;
    Ok(())
}

pub async fn check_setup(ctx: &ServiceContext) -> Result<serde_json::Value, AppError> {
    use crate::entities::academic_year;
    use svc_academic::entities::class;
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
    
    // Check if there are any academic years
    let periods_count = academic_year::Entity::find()
        .filter(academic_year::Column::OrgId.eq(&ctx.org_id))
        .count(&ctx.db)
        .await?;

    // Check if there are any classes
    let classes_count = class::Entity::find()
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .count(&ctx.db)
        .await?;

    let is_complete = periods_count > 0 && classes_count > 0;

    Ok(serde_json::json!({
        "setup_completed": is_complete,
        "step": if is_complete { "completed" } else { "onboarding" }
    }))
}
