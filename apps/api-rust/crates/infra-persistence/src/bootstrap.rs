use sea_orm::{DatabaseConnection, Statement, ConnectionTrait, DatabaseBackend};
use shared_kernel::event_store::EventStore;
use shared_kernel::traits::Actor;
use shared_kernel::errors::AppError;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;
use tracing::info;

pub struct BootstrapParams {
    pub org_name: String,
    pub org_slug: String,
    pub admin_name: String,
    pub admin_username: String,
    pub admin_password_hash: String,
}

pub struct BootstrapResult {
    pub org_id: String,
    pub admin_id: String,
}

/// Safely bootstraps the initial system state (Root Organization and Admin User).
/// This replaces scattered raw SQL queries in the CLI, ensuring Domain Events are recorded
/// and basic structure is correctly initialized.
pub async fn bootstrap_system(
    db: &DatabaseConnection,
    params: BootstrapParams,
) -> Result<BootstrapResult, AppError> {
    let now = Utc::now().to_rfc3339();
    let org_id = Uuid::new_v4().to_string();
    let user_id = Uuid::new_v4().to_string();

    info!("Bootstrapping system for organization '{}'", params.org_name);

    // 1. Insert Organization
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO organizations (id, name, slug, is_active, subscription_plan, created_at, updated_at) VALUES (?, ?, ?, 1, 'basic', ?, ?)",
        vec![
            org_id.clone().into(),
            params.org_name.clone().into(),
            params.org_slug.clone().into(),
            now.clone().into(),
            now.clone().into()
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create organization: {}", e)))?;

    // 2. Insert Admin User
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO users (id, org_id, email, full_name, password_hash, status, primary_role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'active', 'super_admin', ?, ?)",
        vec![
            user_id.clone().into(),
            org_id.clone().into(),
            params.admin_username.clone().into(),
            params.admin_name.clone().into(),
            params.admin_password_hash.into(),
            now.clone().into(),
            now.clone().into()
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create admin user: {}", e)))?;

    // 2.5 Insert Admin Role into user_roles
    let user_role_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO user_roles (id, user_id, role, org_id, scope_type, is_primary, assigned_by, assigned_at) VALUES (?, ?, 'super_admin', ?, 'global', 1, 'system', ?)",
        vec![
            user_role_id.into(),
            user_id.clone().into(),
            org_id.clone().into(),
            now.clone().into()
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to assign admin role: {}", e)))?;

    // 3. Emit Domain Events
    let event_store = EventStore::new(db.clone());
    let system_actor = Actor::System;

    // Emit OrganizationCreated Event
    event_store.append(
        "organization",
        &org_id,
        "organization.created",
        &json!({
            "name": params.org_name,
            "slug": params.org_slug,
            "subscription_plan": "basic"
        }),
        &system_actor,
        None,
    ).await?;

    // Emit UserCreated Event
    event_store.append(
        "user",
        &user_id,
        "user.created",
        &json!({
            "email": params.admin_username,
            "full_name": params.admin_name,
            "primary_role": "super_admin",
            "org_id": org_id
        }),
        &system_actor,
        None,
    ).await?;

    info!("System bootstrap completed successfully. OrgID: {}", org_id);

    Ok(BootstrapResult {
        org_id,
        admin_id: user_id,
    })
}
