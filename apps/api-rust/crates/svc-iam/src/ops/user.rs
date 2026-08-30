//! User operations — CRUD, profile, search.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Condition, QuerySelect, ConnectionTrait};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::user;

use shared_kernel::query_filter::UniversalFilter;

pub async fn list(
    ctx: &ServiceContext,
    filter: Option<UniversalFilter>,
) -> Result<Vec<user::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;

    let mut query = user::Entity::find()
        .filter(user::Column::OrgId.eq(&ctx.org_id))
        .filter(user::Column::IsDeleted.eq(0));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "full_name" => Some(user::Column::FullName.into_expr().into()),
            "email" => Some(user::Column::Email.into_expr().into()),
            "phone" => Some(user::Column::Phone.into_expr().into()),
            "primary_role" => Some(user::Column::PrimaryRole.into_expr().into()),
            "status" => Some(user::Column::Status.into_expr().into()),
            "created_at" => Some(user::Column::CreatedAt.into_expr().into()),
            "last_login_at" => Some(user::Column::LastLoginAt.into_expr().into()),
            "login_count" => Some(user::Column::LoginCount.into_expr().into()),
            "mfa_enabled" => Some(user::Column::MfaEnabled.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.order_by_asc(user::Column::FullName).all(&ctx.db).await?)
}

pub async fn list_paginated(
    ctx: &ServiceContext,
    request: shared_kernel::pagination::CursorRequest,
    filter: Option<UniversalFilter>,
) -> Result<shared_kernel::pagination::CursorPage<user::Model>, AppError> {
    use shared_kernel::pagination::CursorPage;
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;

    let limit = request.effective_limit();
    let mut query = user::Entity::find()
        .filter(user::Column::OrgId.eq(&ctx.org_id))
        .filter(user::Column::IsDeleted.eq(0))
        .order_by_asc(user::Column::Id);

    // Filter provided in the request body supersedes the one in CursorRequest
    // Or we merge them, but for now we just use the one provided as parameter.
    let active_filter = filter.or(request.filter.clone());

    if let Some(f) = active_filter {
        let condition = f.build_condition(|field| match field {
            "full_name" => Some(user::Column::FullName.into_expr().into()),
            "email" => Some(user::Column::Email.into_expr().into()),
            "phone" => Some(user::Column::Phone.into_expr().into()),
            "primary_role" => Some(user::Column::PrimaryRole.into_expr().into()),
            "status" => Some(user::Column::Status.into_expr().into()),
            "created_at" => Some(user::Column::CreatedAt.into_expr().into()),
            "last_login_at" => Some(user::Column::LastLoginAt.into_expr().into()),
            "login_count" => Some(user::Column::LoginCount.into_expr().into()),
            "mfa_enabled" => Some(user::Column::MfaEnabled.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    if let Some(ref cursor) = request.cursor {
        query = query.filter(user::Column::Id.gt(cursor));
    }

    let items = query.limit(limit + 1).all(&ctx.db).await?;
    Ok(CursorPage::from_items_with(items, limit, |u| u.id.clone()))
}

pub async fn get(ctx: &ServiceContext, user_id: &str) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;
    user::Entity::find_by_id(user_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("user", user_id))
}
pub async fn search(
    ctx: &ServiceContext,
    query_str: &str,
) -> Result<Vec<user::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;

    let pattern = format!("%{}%", query_str);
    Ok(user::Entity::find()
        .filter(user::Column::OrgId.eq(&ctx.org_id))
        .filter(user::Column::IsDeleted.eq(0))
        .filter(
            Condition::any()
                .add(user::Column::FullName.contains(&pattern))
                .add(user::Column::Email.contains(&pattern))
                .add(user::Column::Phone.contains(&pattern))
        )
        .all(&ctx.db).await?)
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CreateUserPayload {
    pub person_id: Option<String>,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub primary_role: String,
    pub password_hash: Option<String>,
}

pub async fn create(
    ctx: &ServiceContext,
    req: CreateUserPayload,
) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    // ── 1. Identity Resolution ─────────────────────────────────────
    // If a person_id is not provided, provision a new core identity profile.
    let person_id = match req.person_id {
        Some(pid) => pid,
        None => {
            let new_person_id = Uuid::new_v4().to_string();
            
            // Split full_name into first and last name for the identity profile
            let name_parts: Vec<&str> = req.full_name.split_whitespace().collect();
            let first_name = name_parts.first().unwrap_or(&"").to_string();
            let last_name = if name_parts.len() > 1 {
                name_parts[1..].join(" ")
            } else {
                first_name.clone()
            };

            let new_party_id = uuid::Uuid::new_v4().to_string();
            let _ = ctx.db.execute(sea_orm::Statement::from_sql_and_values(
                sea_orm::DbBackend::Sqlite,
                "INSERT INTO parties (id, party_type, created_at, updated_at) VALUES ($1, $2, $3, $4)",
                vec![new_party_id.clone().into(), "person".into(), now.clone().into(), now.clone().into()],
            )).await;

            let person_model = crate::entities::person::Model {
                id: new_person_id.clone(),
                party_id: new_party_id,
                org_id: Some(ctx.org_id.clone()),
                first_name,
                last_name,
                full_name: Some(req.full_name.clone()),
                name_ar: None,
                name_en: None, father_name: None, grandfather_name: None,
                gender: None, date_of_birth: None, place_of_birth: None,
                nationality: None, national_id: None, passport_number: None,
                address: None, city: None, country: None, postal_code: None,
                phone: req.phone.clone(), emergency_contact: None,
                blood_type: None, allergies: None, medical_conditions: None,
                medical_notes: None, avatar: None, bio: None, social_links: None,
                created_at: now.clone(), updated_at: now.clone(),
            };
            
            let person_active: crate::entities::person::ActiveModel = person_model.into();
            person_active.insert(&ctx.db).await?;
            
            new_person_id
        }
    };
    let model = user::Model {
        id: id.clone(), person_id, party_id: None, org_id: ctx.org_id.clone(),
        email: req.email, email_verified: 0, email_verified_at: None,
        phone: req.phone, phone_verified: 0, phone_verified_at: None,
        password_hash: req.password_hash, password_changed_at: None, password_expires_at: None,
        mfa_enabled: 0, mfa_secret: None, mfa_backup_codes: None,
        full_name: req.full_name,
        primary_role: req.primary_role, status: "active".into(),
        last_login_at: None, last_login_ip: None,
        login_count: Some(0), failed_logins: Some(0), locked_until: None,
        prefs: "{}".into(), meta: "{}".into(),
        is_deleted: 0, deleted_at: None, deleted_by: None,
        created_at: now.clone(), updated_at: now.clone(),
        created_by: Some(ctx.actor.as_str().to_string()),
    };

    let active: user::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    let user_role_model = crate::entities::user_role::Model {
        id: Uuid::new_v4().to_string(),
        user_id: id.clone(),
        role: created.primary_role.clone(),
        org_id: ctx.org_id.clone(),
        scope_type: "global".to_string(),
        scope_id: None,
        valid_from: None,
        valid_until: None,
        is_primary: 1,
        assigned_by: Some(ctx.actor.as_str().to_string()),
        assigned_at: now.clone(),
    };
    let user_role_active: crate::entities::user_role::ActiveModel = user_role_model.into();
    user_role_active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::USER_CREATED.into(),
        entity_type: "user".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "user.create".into(), entity_type: "user".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct UpdateUserPayload {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}

pub async fn update(
    ctx: &ServiceContext,
    user_id: &str,
    req: UpdateUserPayload,
) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserWrite).await?;

    let existing = get(ctx, user_id).await?;
    let old_data = serde_json::to_value(&existing).ok();
    
    // Save person_id before moving existing
    let person_id = existing.person_id.clone();
    
    let mut active: user::ActiveModel = existing.into();

    if let Some(n) = &req.full_name { active.full_name = Set(n.clone()); }
    if let Some(e) = req.email { active.email = Set(Some(e)); }
    if let Some(p) = req.phone { active.phone = Set(Some(p)); }
    if let Some(s) = req.status { active.status = Set(s); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    // Also update the associated person
    if let Some(person_model) = crate::entities::person::Entity::find_by_id(&person_id)
        .one(&ctx.db).await? 
    {
        let mut person_active: crate::entities::person::ActiveModel = person_model.into();
        if let Some(n) = &req.full_name {
            person_active.full_name = Set(Some(n.clone()));
            person_active.first_name = Set(n.clone());
            person_active.last_name = Set(n.clone());
        }
        person_active.updated_at = Set(Utc::now().to_rfc3339());
        person_active.update(&ctx.db).await?;
    }

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "user.update".into(), entity_type: "user".into(),
        entity_id: user_id.into(), old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

pub async fn delete(ctx: &ServiceContext, user_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserDelete).await?;

    let existing = get(ctx, user_id).await?;
    let mut active: user::ActiveModel = existing.into();
    active.is_deleted = Set(1);
    active.deleted_at = Set(Some(Utc::now().to_rfc3339()));
    active.deleted_by = Set(Some(ctx.actor.as_str().to_string()));
    active.status = Set("deleted".into());
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::USER_DELETED.into(),
        entity_type: "user".into(), entity_id: user_id.into(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({}),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "user.delete".into(), entity_type: "user".into(),
        entity_id: user_id.into(), old_data: None, new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ChangeRolePayload {
    pub role: String,
}

pub async fn change_role(
    ctx: &ServiceContext,
    user_id: &str,
    req: ChangeRolePayload,
) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserWrite).await?;

    let existing = get(ctx, user_id).await?;
    let old_data = serde_json::to_value(&existing).ok();
    let mut active: user::ActiveModel = existing.into();

    active.primary_role = Set(req.role);
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "user.change_role".into(), entity_type: "user".into(),
        entity_id: user_id.into(), old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}
