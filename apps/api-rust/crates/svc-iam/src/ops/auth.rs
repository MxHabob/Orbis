//! Auth operations — login, logout, password, session management.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::settings::AuthenticationSettings;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::{user, user_session};

async fn authentication_settings(
    db: &DatabaseConnection,
    org_id: &str,
) -> Result<AuthenticationSettings, AppError> {
    use sea_orm::{ConnectionTrait, Statement};

    let row = db
        .query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT settings FROM organizations WHERE id = $1",
            vec![org_id.into()],
        ))
        .await?;
    let Some(row) = row else {
        return Ok(AuthenticationSettings::default());
    };
    let settings_json = row.try_get::<Option<String>>("", "settings").unwrap_or(None);
    let settings = settings_json
        .and_then(|value| serde_json::from_str::<serde_json::Value>(&value).ok())
        .and_then(|value| value.get("authentication").cloned())
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default();
    Ok(settings)
}

/// Authenticate by email/phone + password.
pub async fn login(
    db: &DatabaseConnection,
    identifier: &str,
    password_hash: &str,
) -> Result<(user::Model, user_session::Model), AppError> {
    let found = user::Entity::find()
        .filter(user::Column::IsDeleted.eq(0))
        .filter(user::Column::Status.eq("active"))
        .filter(
            sea_orm::Condition::any()
                .add(user::Column::Email.eq(identifier))
                .add(user::Column::Phone.eq(identifier))
        )
        .one(db).await?
        .ok_or_else(|| AppError::unauthorized("Invalid credentials"))?;
    let auth_settings = authentication_settings(db, &found.org_id).await?;

    // Check locked
    if let Some(ref locked) = found.locked_until {
        if *locked > Utc::now().to_rfc3339() {
            return Err(AppError::unauthorized("Account is locked"));
        }
    }

    // Verify password
    let valid_password = match &found.password_hash {
        Some(hash) => bcrypt::verify(password_hash, hash).unwrap_or(false),
        None => false,
    };

    if !valid_password {
        let failed_logins = found
            .failed_logins
            .unwrap_or_default()
            .saturating_add(1);
        let mut active: user::ActiveModel = found.clone().into();
        active.failed_logins = Set(Some(failed_logins));
        if failed_logins >= auth_settings.max_failed_logins.min(i32::MAX as u32) as i32 {
            active.locked_until = Set(Some(
                (Utc::now() + chrono::Duration::minutes(auth_settings.lockout_minutes as i64)).to_rfc3339()
            ));
        }
        active.update(db).await?;
        return Err(AppError::unauthorized("Invalid credentials"));
    }

    // Update login stats
    let now = Utc::now().to_rfc3339();
    let mut active: user::ActiveModel = found.clone().into();
    active.last_login_at = Set(Some(now.clone()));
    active.login_count = Set(Some(found.login_count.unwrap_or(0) + 1));
    active.failed_logins = Set(Some(0));
    active.locked_until = Set(None);
    active.updated_at = Set(now.clone());
    let updated_user = active.update(db).await?;

    // Create session
    let session_id = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string();
    let expires = (Utc::now() + chrono::Duration::days(auth_settings.session_days as i64)).to_rfc3339();

    let session = user_session::Model {
        id: session_id.clone(),
        user_id: found.id.clone(),
        token: token.clone(),
        refresh_token: Some(Uuid::new_v4().to_string()),
        device_id: None,
        device_type: None,
        device_name: None,
        os: None,
        browser: None,
        ip_address: None,
        user_agent: None,
        is_current: 1,
        is_trusted: 0,
        mfa_verified: 0,
        expires_at: expires,
        refresh_expires_at: Some(
            (Utc::now() + chrono::Duration::days(auth_settings.refresh_token_days as i64)).to_rfc3339(),
        ),
        last_active_at: Some(now.clone()),
        created_at: now,
    };

    let session_active: user_session::ActiveModel = session.into();
    let created_session = session_active.insert(db).await?;

    Ok((updated_user, created_session))
}

/// Rotate an active session using its refresh token.
pub async fn refresh_session(
    db: &DatabaseConnection,
    refresh_token: &str,
) -> Result<(user::Model, user_session::Model), AppError> {
    let existing = user_session::Entity::find()
        .filter(user_session::Column::RefreshToken.eq(refresh_token))
        .filter(user_session::Column::IsCurrent.eq(1))
        .one(db)
        .await?
        .ok_or_else(|| AppError::unauthorized("Invalid or expired refresh token"))?;

    if let Some(refresh_expires_at) = existing.refresh_expires_at.as_deref() {
        let expires_at = chrono::DateTime::parse_from_rfc3339(refresh_expires_at)
            .map_err(|_| AppError::unauthorized("Invalid refresh token expiry"))?;
        if expires_at.with_timezone(&Utc) <= Utc::now() {
            return Err(AppError::unauthorized("Refresh token expired"));
        }
    }

    let user = user::Entity::find_by_id(existing.user_id.clone())
        .filter(user::Column::IsDeleted.eq(0))
        .filter(user::Column::Status.eq("active"))
        .one(db)
        .await?
        .ok_or_else(|| AppError::unauthorized("User account is inactive"))?;
    let auth_settings = authentication_settings(db, &user.org_id).await?;

    let now = Utc::now();
    let mut active: user_session::ActiveModel = existing.into();
    active.token = Set(Uuid::new_v4().to_string());
    active.refresh_token = Set(Some(Uuid::new_v4().to_string()));
    active.expires_at = Set((now + chrono::Duration::days(auth_settings.session_days as i64)).to_rfc3339());
    active.refresh_expires_at = Set(Some(
        (now + chrono::Duration::days(auth_settings.refresh_token_days as i64)).to_rfc3339(),
    ));
    active.last_active_at = Set(Some(now.to_rfc3339()));
    let session = active.update(db).await?;

    Ok((user, session))
}

/// Logout — invalidate session.
pub async fn logout(db: &DatabaseConnection, token: &str) -> Result<(), AppError> {
    let existing = user_session::Entity::find()
        .filter(user_session::Column::Token.eq(token))
        .one(db).await?
        .ok_or_else(|| AppError::not_found("user_session", token))?;

    let mut active: user_session::ActiveModel = existing.into();
    active.is_current = Set(0);
    active.update(db).await?;

    Ok(())
}

/// Restore session from a token.
pub async fn restore_session(
    db: &DatabaseConnection,
    token: &str,
) -> Result<(user::Model, user_session::Model), AppError> {
    let session = user_session::Entity::find()
        .filter(user_session::Column::Token.eq(token))
        .filter(user_session::Column::IsCurrent.eq(1))
        .one(db)
        .await?
        .ok_or_else(|| AppError::unauthorized("Invalid or expired session token"))?;

    let expires_at = chrono::DateTime::parse_from_rfc3339(&session.expires_at)
        .map_err(|_| AppError::unauthorized("Invalid session expiry"))?;
    if expires_at <= Utc::now() {
        return Err(AppError::unauthorized("Session expired"));
    }
        
    let user = user::Entity::find_by_id(session.user_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| AppError::unauthorized("User not found"))?;

    if user.is_deleted != 0 || user.status.to_lowercase() != "active" {
        return Err(AppError::unauthorized("User account is inactive"));
    }

    Ok((user, session))
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ChangePasswordPayload {
    pub new_password: String,
}

/// Change password.
pub async fn change_password(
    ctx: &ServiceContext,
    user_id: &str,
    req: ChangePasswordPayload,
) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserPasswordWrite).await?;

    let existing = user::Entity::find_by_id(user_id.to_string())
        .filter(user::Column::OrgId.eq(ctx.org_id.clone()))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("user", user_id))?;

    let mut active: user::ActiveModel = existing.into();
    if req.new_password.len() < 12 {
        return Err(AppError::validation("Password must contain at least 12 characters"));
    }
    let password_hash = bcrypt::hash(&req.new_password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::internal("Failed to secure password"))?;
    active.password_hash = Set(Some(password_hash));
    active.password_changed_at = Set(Some(Utc::now().to_rfc3339()));
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;

    user_session::Entity::update_many()
        .col_expr(user_session::Column::IsCurrent, 0.into())
        .filter(user_session::Column::UserId.eq(user_id))
        .filter(user_session::Column::IsCurrent.eq(1))
        .exec(&ctx.db)
        .await?;

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

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct RegisterPayload {
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
}

pub async fn register(
    db: &DatabaseConnection,
    req: RegisterPayload,
) -> Result<user::Model, AppError> {
    if req.password.len() < 12 {
        return Err(AppError::validation("Password must contain at least 12 characters"));
    }
    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::internal("Failed to secure password"))?;

    // Check email/phone uniqueness
    if let Some(ref email) = req.email {
        let existing = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db).await?;
        if existing.is_some() {
            return Err(AppError::conflict(format!("Email '{}' already registered", email)));
        }
    }
    if let Some(ref phone) = req.phone {
        let existing = user::Entity::find()
            .filter(user::Column::Phone.eq(phone))
            .one(db).await?;
        if existing.is_some() {
            return Err(AppError::conflict(format!("Phone '{}' already registered", phone)));
        }
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let org_id = "default".to_string();
    let txn = db.begin().await?;

    // ── 1. Identity Provisioning ──────────────────────────────────
    // Self-registration provisions a new core identity profile first.
    let person_id = Uuid::new_v4().to_string();
    
    let name_parts: Vec<&str> = req.full_name.split_whitespace().collect();
    let first_name = name_parts.first().unwrap_or(&"").to_string();
    let last_name = if name_parts.len() > 1 {
        name_parts[1..].join(" ")
    } else {
        first_name.clone()
    };

    let party_id = Uuid::new_v4().to_string();
    let party_model = crate::entities::party::Model {
        id: party_id.clone(),
        party_type: "Person".to_string(),
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    let party_active: crate::entities::party::ActiveModel = party_model.into();
    party_active.insert(&txn).await?;

    let person_model = crate::entities::person::Model {
        id: person_id.clone(),
        party_id,
        org_id: Some(org_id.clone()),
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
    person_active.insert(&txn).await?;

    let model = user::Model {
        id: id.clone(),
        person_id,
        party_id: None,
        org_id,
        email: req.email,
        email_verified: 0,
        email_verified_at: None,
        phone: req.phone,
        phone_verified: 0,
        phone_verified_at: None,
        password_hash: Some(password_hash),
        password_changed_at: None,
        password_expires_at: None,
        mfa_enabled: 0,
        mfa_secret: None,
        mfa_backup_codes: None,
        full_name: req.full_name,
        primary_role: "student".to_string(),
        status: "active".into(),
        last_login_at: None,
        last_login_ip: None,
        login_count: Some(0),
        failed_logins: Some(0),
        locked_until: None,
        prefs: "{}".into(),
        meta: "{}".into(),
        is_deleted: 0,
        deleted_at: None,
        deleted_by: None,
        created_at: now.clone(),
        updated_at: now,
        created_by: Some("register".to_string()),
    };

    let active: user::ActiveModel = model.into();
    let created = active.insert(&txn).await?;
    txn.commit().await?;
    Ok(created)
}
pub async fn logout_by_user(db: &DatabaseConnection, user_id: &str) -> Result<(), AppError> {
    user_session::Entity::update_many()
        .col_expr(user_session::Column::IsCurrent, sea_orm::sea_query::Expr::value(0))
        .filter(user_session::Column::UserId.eq(user_id))
        .exec(db)
        .await?;
    Ok(())
}
