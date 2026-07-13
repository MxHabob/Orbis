//! Organization operations — CRUD via ServiceContext.

use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use tracing::info;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::{CreateOrganizationCmd, UpdateOrganizationCmd};
use crate::contracts::responses::OrganizationResponse;
use crate::contracts::events::event_types;
use crate::entities::{organization, period_template};
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

pub async fn initialize_school_setup(ctx: &ServiceContext, payload: serde_json::Value) -> Result<(), AppError> {
    use sea_orm::{ConnectionTrait, TransactionTrait, Statement, DatabaseBackend};
    
    let txn = ctx.db.begin().await?;

    let org_name = payload.get("org_name").and_then(|v| v.as_str()).unwrap_or("Organization");
    let org_name_ar = payload.get("org_name_ar").and_then(|v| v.as_str()).unwrap_or(org_name);
    let currency = payload.get("currency").and_then(|v| v.as_str()).unwrap_or("YER");
    let now = chrono::Utc::now().naive_utc().to_string();
    
    // 1. Update Organization
    txn.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        format!(
            "UPDATE organizations SET name = '{}', name_ar = '{}', currency = '{}' WHERE id = '{}';",
            org_name.replace("'", "''"), org_name_ar.replace("'", "''"), currency.replace("'", "''"), ctx.org_id.replace("'", "''")
        )
    )).await?;

    // 2. Insert Root Unit
    let root_unit_id = payload.get("root_unit_id").and_then(|v| v.as_str()).unwrap_or_else(|| "unit-default");
    let root_name = payload.get("root_unit_name").and_then(|v| v.as_str()).unwrap_or("Main Unit");
    let root_name_ar = payload.get("root_unit_name_ar").and_then(|v| v.as_str()).unwrap_or(root_name);
    
    txn.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        format!(
            "INSERT INTO org_units (id, org_id, name, name_ar, unit_type, is_active, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', 'root', 1, '{}', '{}') ON CONFLICT DO NOTHING;",
            root_unit_id.replace("'", "''"), ctx.org_id.replace("'", "''"), root_name.replace("'", "''"), root_name_ar.replace("'", "''"), now, now
        )
    )).await?;

    let mut period_id_val = String::from("default-year");

    // 3. Insert periods
    if let Some(periods) = payload.get("periods").and_then(|v| v.as_array()) {
        for p in periods {
            let id = p.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let parent_id = p.get("parent_id").and_then(|v| v.as_str());
            let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let name_ar = p.get("name_ar").and_then(|v| v.as_str()).unwrap_or(name);
            let start_date = p.get("start_date").and_then(|v| v.as_str()).unwrap_or("2026-01-01");
            let end_date = p.get("end_date").and_then(|v| v.as_str()).unwrap_or("2026-12-31");
            let ptype = p.get("period_type").and_then(|v| v.as_str()).unwrap_or("term");

            if ptype == "year" {
                period_id_val = id.to_string();
                txn.execute(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    format!(
                        "INSERT INTO academic_years (id, org_id, name, name_ar, start_date, end_date, status, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', 'active', '{}', '{}') ON CONFLICT DO NOTHING;",
                        id.replace("'", "''"), ctx.org_id.replace("'", "''"), name.replace("'", "''"), name_ar.replace("'", "''"), start_date, end_date, now, now
                    )
                )).await?;
            } else {
                let mapped_ptype = if ptype == "term" { "semester" } else { "custom" };
                let parent_val = if let Some(pid) = parent_id { format!("'{}'", pid.replace("'", "''")) } else { "NULL".to_string() };
                txn.execute(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    format!(
                        "INSERT INTO academic_periods (id, org_id, period_id, parent_id, name, name_ar, period_type, start_date, end_date, status, created_at, updated_at) VALUES ('{}', '{}', '{}', {}, '{}', '{}', '{}', '{}', '{}', 'active', '{}', '{}') ON CONFLICT DO NOTHING;",
                        id.replace("'", "''"), ctx.org_id.replace("'", "''"), period_id_val.replace("'", "''"), parent_val, name.replace("'", "''"), name_ar.replace("'", "''"), mapped_ptype, start_date, end_date, now, now
                    )
                )).await?;
            }
        }
    }

    // 4. Insert classes
    // Note: Schema expects period_id, not period_id. Since we don't have exactly the term period ID here easily, we fallback to period_id if needed,
    // wait, actually we can just pass period_id from the frontend payload or use period_id_val as a fallback if the column allows. 
    // In schema_baseline.sql, classes has: period_id TEXT NOT NULL. So let's use period_id_val as period_id to ensure it inserts without error.
    if let Some(classes) = payload.get("classes").and_then(|v| v.as_array()) {
        for c in classes {
            let id = c.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let name = c.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let name_ar = c.get("name_ar").and_then(|v| v.as_str()).unwrap_or(name);
            let level = c.get("level").and_then(|v| v.as_str()).unwrap_or("grade1");
            let div = c.get("division").and_then(|v| v.as_str()).unwrap_or("A");
            txn.execute(Statement::from_string(
                DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO classes (id, org_id, period_id, unit_id, name, name_ar, level, division, capacity, is_active, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', 30, 1, '{}', '{}') ON CONFLICT DO NOTHING;",
                    id.replace("'", "''"), ctx.org_id.replace("'", "''"), period_id_val.replace("'", "''"), root_unit_id.replace("'", "''"), name.replace("'", "''"), name_ar.replace("'", "''"), level.replace("'", "''"), div.replace("'", "''"), now, now
                )
            )).await?;
        }
    }

    // 5. Insert subjects
    if let Some(subjects) = payload.get("subjects").and_then(|v| v.as_array()) {
        for s in subjects {
            let id = s.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let name = s.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let name_ar = s.get("name_ar").and_then(|v| v.as_str()).unwrap_or(name);
            let code = s.get("code").and_then(|v| v.as_str()).unwrap_or("");
            txn.execute(Statement::from_string(
                DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO subjects (id, org_id, unit_id, code, name, name_ar, is_core, is_elective, requires_lab, requires_practical, is_active, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', 1, 0, 0, 0, 1, '{}', '{}') ON CONFLICT DO NOTHING;",
                    id.replace("'", "''"), ctx.org_id.replace("'", "''"), root_unit_id.replace("'", "''"), code.replace("'", "''"), name.replace("'", "''"), name_ar.replace("'", "''"), now, now
                )
            )).await?;
        }
    }

    // 6. Insert time_slots
    if let Some(slots) = payload.get("time_slots").and_then(|v| v.as_array()) {
        for (i, s) in slots.iter().enumerate() {
            let name = s.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let name_ar = s.get("name_ar").and_then(|v| v.as_str()).unwrap_or(name);
            let start = s.get("start_time").and_then(|v| v.as_str()).unwrap_or("08:00");
            let end = s.get("end_time").and_then(|v| v.as_str()).unwrap_or("09:00");
            let id = format!("ts-{}-{}", ctx.org_id, i);
            txn.execute(Statement::from_string(
                DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO time_slots (id, org_id, period_id, slot_number, name, name_ar, start_time, end_time, duration_minutes, is_break, is_active, created_at, updated_at) VALUES ('{}', '{}', '{}', {}, '{}', '{}', '{}', '{}', 45, 0, 1, '{}', '{}') ON CONFLICT DO NOTHING;",
                    id.replace("'", "''"), ctx.org_id.replace("'", "''"), period_id_val.replace("'", "''"), i+1, name.replace("'", "''"), name_ar.replace("'", "''"), start.replace("'", "''"), end.replace("'", "''"), now, now
                )
            )).await?;
        }
    }

    // 7. Insert rooms
    if let Some(rooms) = payload.get("rooms").and_then(|v| v.as_array()) {
        for r in rooms {
            let id = r.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let name = r.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let name_ar = r.get("name_ar").and_then(|v| v.as_str()).unwrap_or(name);
            txn.execute(Statement::from_string(
                DatabaseBackend::Sqlite,
                format!(
                    "INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, is_active, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', 30, 'classroom', 1, '{}', '{}') ON CONFLICT DO NOTHING;",
                    id.replace("'", "''"), ctx.org_id.replace("'", "''"), name.replace("'", "''"), name_ar.replace("'", "''"), now, now
                )
            )).await?;
        }
    }

    // 8. Fiscal Year and Fiscal Periods (Default)
    let current_year = chrono::Utc::now().format("%Y").to_string();
    let created_at = chrono::Utc::now().to_rfc3339();
    let fy_id = format!("{}-fy", ctx.org_id);
    txn.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        format!(
            "INSERT INTO fiscal_years (id, org_id, name, start_date, end_date, status, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}-01-01', '{}-12-31', 'open', '{}', '{}') ON CONFLICT DO NOTHING;",
            fy_id, ctx.org_id, current_year, current_year, current_year, created_at, created_at
        )
    )).await?;

    for month in 1..=12 {
        let fp_id = format!("{}-fp-{:02}", ctx.org_id, month);
        let name = format!("Month {}", month);
        let start_date = format!("{}-{:02}-01", current_year, month);
        let end_day = match month {
            4 | 6 | 9 | 11 => 30,
            2 => 28,
            _ => 31,
        };
        let end_date = format!("{}-{:02}-{:02}", current_year, month, end_day);
        
        txn.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            format!(
                "INSERT INTO fiscal_periods (id, org_id, fiscal_period_id, name, start_date, end_date, status, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', 'open', '{}', '{}') ON CONFLICT DO NOTHING;",
                fp_id, ctx.org_id, fy_id, name, start_date, end_date, created_at, created_at
            )
        )).await?;
    }


    txn.commit().await?;
    
    Ok(())
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
