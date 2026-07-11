//! Auth operations — login, logout, password, session management.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::{user, user_session};

/// Authenticate by email/phone + password.
pub async fn login(
    ctx: &ServiceContext,
    identifier: &str,
    password_hash: &str,
    ip: Option<String>,
    user_agent: Option<String>,
) -> Result<(user::Model, user_session::Model), AppError> {
    let found = user::Entity::find()
        .filter(user::Column::OrgId.eq(&ctx.org_id))
        .filter(user::Column::IsDeleted.eq(0))
        .filter(
            sea_orm::Condition::any()
                .add(user::Column::Email.eq(identifier))
                .add(user::Column::Phone.eq(identifier))
        )
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::unauthorized("Invalid credentials"))?;

    // Check locked
    if let Some(ref locked) = found.locked_until {
        if *locked > Utc::now().to_rfc3339() {
            return Err(AppError::unauthorized("Account is locked"));
        }
    }

    // Verify password
    match &found.password_hash {
        Some(hash) if hash == password_hash => {},
        _ => {
            // Increment failed logins
            let mut active: user::ActiveModel = found.clone().into();
            active.failed_logins = Set(Some(found.failed_logins.unwrap_or(0) + 1));
            if found.failed_logins.unwrap_or(0) >= 4 {
                active.locked_until = Set(Some(
                    (Utc::now() + chrono::Duration::minutes(15)).to_rfc3339()
                ));
            }
            active.update(&ctx.db).await?;
            return Err(AppError::unauthorized("Invalid credentials"));
        }
    }

    // Update login stats
    let now = Utc::now().to_rfc3339();
    let mut active: user::ActiveModel = found.clone().into();
    active.last_login_at = Set(Some(now.clone()));
    active.last_login_ip = Set(ip.clone());
    active.login_count = Set(Some(found.login_count.unwrap_or(0) + 1));
    active.failed_logins = Set(Some(0));
    active.locked_until = Set(None);
    active.updated_at = Set(now.clone());
    let updated_user = active.update(&ctx.db).await?;

    // Create session
    let session_id = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string();
    let expires = (Utc::now() + chrono::Duration::days(30)).to_rfc3339();

    let session = user_session::Model {
        id: session_id.clone(),
        user_id: found.id.clone(),
        token: token.clone(),
        refresh_token: Some(Uuid::new_v4().to_string()),
        device_id: None,
        device_type: None,
        device_name: user_agent.clone(),
        os: None,
        browser: None,
        ip_address: ip,
        user_agent,
        is_current: 1,
        is_trusted: 0,
        mfa_verified: 0,
        expires_at: expires,
        refresh_expires_at: None,
        last_active_at: Some(now.clone()),
        created_at: now,
    };

    let session_active: user_session::ActiveModel = session.into();
    let created_session = session_active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::USER_LOGIN.into(),
        entity_type: "user".into(), entity_id: found.id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: "user".into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({ "session_id": session_id }),
        chain_depth: 0,
    });

    Ok((updated_user, created_session))
}

/// Logout — invalidate session.
pub async fn logout(ctx: &ServiceContext, session_id: &str) -> Result<(), AppError> {
    let existing = user_session::Entity::find_by_id(session_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("user_session", session_id))?;

    let mut active: user_session::ActiveModel = existing.into();
    active.is_current = Set(0);
    active.update(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::USER_LOGOUT.into(),
        entity_type: "user".into(), entity_id: session_id.into(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: "user".into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({}),
        chain_depth: 0,
    });

    Ok(())
}

/// Change password.
pub async fn change_password(
    ctx: &ServiceContext,
    user_id: &str,
    new_password_hash: String,
) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserWrite).await?;

    let existing = user::Entity::find_by_id(user_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("user", user_id))?;

    let mut active: user::ActiveModel = existing.into();
    active.password_hash = Set(Some(new_password_hash));
    active.password_changed_at = Set(Some(Utc::now().to_rfc3339()));
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::PASSWORD_CHANGED.into(),
        entity_type: "user".into(), entity_id: user_id.into(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({}),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "user.change_password".into(), entity_type: "user".into(),
        entity_id: user_id.into(), old_data: None, new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}

/// List active sessions for a user.
pub async fn list_sessions(
    ctx: &ServiceContext, user_id: &str,
) -> Result<Vec<user_session::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;
    Ok(user_session::Entity::find()
        .filter(user_session::Column::UserId.eq(user_id))
        .filter(user_session::Column::IsCurrent.eq(1))
        .all(&ctx.db).await?)
}
