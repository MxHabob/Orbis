//! Organization operations — CRUD via ServiceContext.

use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use tracing::info;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::{CreateOrganizationCmd, UpdateOrganizationCmd};
use crate::contracts::responses::OrganizationResponse;
use crate::contracts::events::event_types;
use crate::entities::organization;
use svc_calendar::entities::period_template;
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
        current_period_id: Set(None),
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
        updated_at: Set(now.clone()),
    };

    let inserted = org.insert(&ctx.db).await?;

    // Seed default period templates
    let default_templates = vec![
        period_template::ActiveModel {
            id: Set(format!("tpl-2-semesters-{}", org_id)),
            org_id: Set(org_id.clone()),
            name: Set("Two Semesters (4 Months each)".to_string()),
            name_ar: Set(Some("نظام فصلين دراسيين".to_string())),
            institution_type: Set("school".to_string()),
            structure: Set(r#"[{"name":"Semester 1","name_ar":"الفصل الدراسي الأول","type":"term","weight":50,"children":[{"name":"Month 1","name_ar":"الشهر الأول","type":"month","weight":25},{"name":"Month 2","name_ar":"الشهر الثاني","type":"month","weight":25},{"name":"Month 3","name_ar":"الشهر الثالث","type":"month","weight":25},{"name":"Month 4 (Final)","name_ar":"الشهر الرابع (النهائي)","type":"month","weight":25}]},{"name":"Semester 2","name_ar":"الفصل الدراسي الثاني","type":"term","weight":50,"children":[{"name":"Month 1","name_ar":"الشهر الأول","type":"month","weight":25},{"name":"Month 2","name_ar":"الشهر الثاني","type":"month","weight":25},{"name":"Month 3","name_ar":"الشهر الثالث","type":"month","weight":25},{"name":"Month 4 (Final)","name_ar":"الشهر الرابع (النهائي)","type":"month","weight":25}]}]"#.to_string()),
            is_default: Set(1),
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
        },
        period_template::ActiveModel {
            id: Set(format!("tpl-3-trimesters-{}", org_id)),
            org_id: Set(org_id.clone()),
            name: Set("Three Trimesters".to_string()),
            name_ar: Set(Some("نظام 3 فصول دراسية".to_string())),
            institution_type: Set("school".to_string()),
            structure: Set(r#"[{"name":"Trimester 1","name_ar":"الفصل الدراسي الأول","type":"term","weight":33.33,"children":[{"name":"Mid-Term","name_ar":"منتصف الفصل","type":"quarter","weight":50},{"name":"Final","name_ar":"النهائي","type":"quarter","weight":50}]},{"name":"Trimester 2","name_ar":"الفصل الدراسي الثاني","type":"term","weight":33.33,"children":[{"name":"Mid-Term","name_ar":"منتصف الفصل","type":"quarter","weight":50},{"name":"Final","name_ar":"النهائي","type":"quarter","weight":50}]},{"name":"Trimester 3","name_ar":"الفصل الدراسي الثالث","type":"term","weight":33.34,"children":[{"name":"Mid-Term","name_ar":"منتصف الفصل","type":"quarter","weight":50},{"name":"Final","name_ar":"النهائي","type":"quarter","weight":50}]}]"#.to_string()),
            is_default: Set(0),
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
        },
        period_template::ActiveModel {
            id: Set(format!("tpl-4-quarters-{}", org_id)),
            org_id: Set(org_id.clone()),
            name: Set("Four Quarters".to_string()),
            name_ar: Set(Some("نظام 4 أرباع".to_string())),
            institution_type: Set("school".to_string()),
            structure: Set(r#"[{"name":"Quarter 1","name_ar":"الربع الأول","type":"quarter","weight":25},{"name":"Quarter 2","name_ar":"الربع الثاني","type":"quarter","weight":25},{"name":"Quarter 3","name_ar":"الربع الثالث","type":"quarter","weight":25},{"name":"Quarter 4","name_ar":"الربع الرابع","type":"quarter","weight":25}]"#.to_string()),
            is_default: Set(0),
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
        }
    ];

    for t in default_templates {
        t.insert(&ctx.db).await.ok();
    }

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
    if let Some(cur) = cmd.currency {
        active.currency = Set(cur.clone());
        let db_backend = ctx.db.get_database_backend();
        let now = chrono::Utc::now().to_rfc3339();
        
        let reset_stmt = sea_orm::Statement::from_string(
            db_backend,
            format!("UPDATE currencies SET is_base = 0 WHERE org_id = '{}'", ctx.org_id),
        );
        ctx.db.execute(reset_stmt).await.ok();

        let update_stmt = sea_orm::Statement::from_string(
            db_backend,
            format!(
                "UPDATE currencies SET is_base = 1, is_active = 1, updated_at = '{}' WHERE org_id = '{}' AND code = '{}'",
                now, ctx.org_id, cur
            ),
        );
        if let Ok(result) = ctx.db.execute(update_stmt).await {
            if result.rows_affected() == 0 {
                let insert_stmt = sea_orm::Statement::from_string(
                    db_backend,
                    format!(
                        "INSERT INTO currencies (code, org_id, name, symbol, symbol_position, decimal_places, exchange_rate, is_base, is_active, updated_at) VALUES ('{}', '{}', '{}', '{}', 'after', 2, 1.0, 1, 1, '{}')",
                        cur, ctx.org_id, cur, cur, now
                    ),
                );
                ctx.db.execute(insert_stmt).await.ok();
            }
        }
    }
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
    mut data: serde_json::Value,
) -> Result<OrganizationResponse, AppError> {
    if let Some(obj) = data.as_object_mut() {
        if !obj.contains_key("org_id") {
            obj.insert("org_id".to_string(), serde_json::Value::String(ctx.org_id.clone()));
        }
    }
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
    use svc_calendar::entities::academic_period;
    use svc_academic::entities::class;
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
    
    // Check if there are any academic periods
    let periods_count = academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .count(&ctx.db)
        .await?;

    // Check if there are any classes
    let classes_count = class::Entity::find()
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .filter(class::Column::IsDeleted.eq(0))
        .count(&ctx.db)
        .await?;

    let is_complete = periods_count > 0 && classes_count > 0;

    Ok(serde_json::json!({
        "setup_completed": is_complete,
        "step": if is_complete { "completed" } else { "onboarding" }
    }))
}



/// Fetch period templates for the organization
pub async fn get_period_templates(ctx: &ServiceContext) -> Result<serde_json::Value, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;

    let templates = period_template::Entity::find()
        .filter(period_template::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db)
        .await?;

    // We can map it directly to JSON
    let mut mapped = Vec::new();
    for t in templates {
        let structure_val: serde_json::Value = serde_json::from_str(&t.structure).unwrap_or(serde_json::json!([]));
        mapped.push(serde_json::json!({
            "id": t.id,
            "org_id": t.org_id,
            "name": t.name,
            "name_ar": t.name_ar,
            "institution_type": t.institution_type,
            "structure": structure_val,
            "is_default": t.is_default,
        }));
    }

    Ok(serde_json::Value::Array(mapped))
}
