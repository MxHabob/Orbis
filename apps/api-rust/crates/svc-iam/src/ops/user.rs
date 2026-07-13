//! User operations — CRUD, profile, search.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Condition};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::user;

#[derive(serde::Deserialize)]
pub struct UserFilter {
    pub role: Option<String>,
    pub roles: Option<Vec<String>>,
    pub status: Option<String>,
}

pub async fn list(
    ctx: &ServiceContext,
    filter: Option<UserFilter>,
) -> Result<Vec<user::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;

    let mut query = user::Entity::find()
        .filter(user::Column::OrgId.eq(&ctx.org_id))
        .filter(user::Column::IsDeleted.eq(0));

    if let Some(f) = filter {
        if let Some(role) = f.role {
            query = query.filter(user::Column::PrimaryRole.eq(role));
        } else if let Some(roles) = f.roles {
            if !roles.is_empty() {
                query = query.filter(user::Column::PrimaryRole.is_in(roles));
            }
        }
        
        if let Some(status) = f.status {
            query = query.filter(user::Column::Status.eq(status));
        }
    }

    Ok(query.order_by_asc(user::Column::FullName).all(&ctx.db).await?)
}

pub async fn get(ctx: &ServiceContext, user_id: &str) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;
    user::Entity::find_by_id(user_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("user", user_id))
}

pub async fn search(
    ctx: &ServiceContext, query_str: &str,
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
                .add(user::Column::NationalId.contains(&pattern))
        )
        .all(&ctx.db).await?)
}

#[derive(serde::Deserialize)]
pub struct CreateUserPayload {
    pub full_name: String,
    pub name_ar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub primary_role: String,
    pub password_hash: Option<String>,
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserWrite).await?;

    let req: CreateUserPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create user payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = user::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        email: req.email, email_verified: 0, email_verified_at: None,
        phone: req.phone, phone_verified: 0, phone_verified_at: None,
        password_hash: req.password_hash, password_changed_at: None, password_expires_at: None,
        mfa_enabled: 0, mfa_secret: None, mfa_backup_codes: None,
        full_name: req.full_name, name_ar: req.name_ar, name_en: None, avatar: None,
        sex: None, date_of_birth: None, place_of_birth: None,
        nationality: None, national_id: None, passport_number: None,
        address: None, city: None, country: None, postal_code: None,
        emergency_contact: None, blood_type: None, allergies: None,
        medical_conditions: None, medical_notes: None,
        social_links: None, bio: None,
        primary_role: req.primary_role, status: "active".into(),
        last_login_at: None, last_login_ip: None,
        login_count: Some(0), failed_logins: Some(0), locked_until: None,
        prefs: "{}".into(), meta: "{}".into(),
        is_deleted: 0, deleted_at: None, deleted_by: None,
        created_at: now.clone(), updated_at: now,
        created_by: Some(ctx.actor.as_str().to_string()),
    };

    let active: user::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

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

#[derive(serde::Deserialize)]
pub struct UpdateUserPayload {
    pub full_name: Option<String>,
    pub name_ar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}

pub async fn update(
    ctx: &ServiceContext,
    user_id: &str,
    data: serde_json::Value,
) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserWrite).await?;

    let req: UpdateUserPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update user payload: {e}")))?;

    let existing = get(ctx, user_id).await?;
    let old_data = serde_json::to_value(&existing).ok();
    let mut active: user::ActiveModel = existing.into();

    if let Some(n) = req.full_name { active.full_name = Set(n); }
    if let Some(na) = req.name_ar { active.name_ar = Set(Some(na)); }
    if let Some(e) = req.email { active.email = Set(Some(e)); }
    if let Some(p) = req.phone { active.phone = Set(Some(p)); }
    if let Some(s) = req.status { active.status = Set(s); }
    active.updated_at = Set(Utc::now().to_rfc3339());

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

#[derive(serde::Deserialize)]
pub struct ChangeRolePayload {
    pub role: String,
}

pub async fn change_role(
    ctx: &ServiceContext,
    user_id: &str,
    data: serde_json::Value,
) -> Result<user::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserWrite).await?;

    let req: ChangeRolePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid change role payload: {e}")))?;

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
