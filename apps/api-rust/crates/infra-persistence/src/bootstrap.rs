use sea_orm::{DatabaseConnection, Statement, ConnectionTrait};
use shared_kernel::event_store::EventStore;
use shared_kernel::traits::Actor;
use shared_kernel::errors::AppError;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;
use tracing::info;

pub enum BlueprintType {
    School,
    University,
    TrainingCenter,
}

pub struct BootstrapParams {
    pub org_name: String,
    pub org_slug: String,
    pub admin_name: String,
    pub admin_username: String,
    pub admin_password_hash: String,
    pub blueprint: BlueprintType,
}

pub struct BootstrapResult {
    pub org_id: String,
    pub admin_id: String,
}

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
        db.get_database_backend(),
        "INSERT INTO organizations (id, name, slug, is_active, subscription_plan, created_at, updated_at) VALUES ($1, $2, $3, 1, 'basic', $4, $5)",
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
        db.get_database_backend(),
        "INSERT INTO users (id, org_id, email, full_name, password_hash, status, primary_role, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'active', 'super_admin', $6, $7)",
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
        db.get_database_backend(),
        "INSERT INTO user_roles (id, user_id, role, org_id, scope_type, is_primary, assigned_by, assigned_at) VALUES ($1, $2, 'super_admin', $3, 'global', 1, 'system', $4)",
        vec![
            user_role_id.into(),
            user_id.clone().into(),
            org_id.clone().into(),
            now.clone().into()
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to assign admin role: {}", e)))?;

    // 3. Apply Blueprint
    match params.blueprint {
        BlueprintType::School => apply_school_blueprint(db, &org_id, &now).await?,
        BlueprintType::University => apply_university_blueprint(db, &org_id, &now).await?,
        BlueprintType::TrainingCenter => apply_training_blueprint(db, &org_id, &now).await?,
    }

    // 4. Emit Domain Events
    let event_store = EventStore::new(db.clone());
    let system_actor = Actor::System;

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

    info!("System bootstrap completed successfully. OrgID: {}", org_id);

    Ok(BootstrapResult {
        org_id,
        admin_id: user_id,
    })
}

async fn apply_school_blueprint(db: &DatabaseConnection, org_id: &str, now: &str) -> Result<(), AppError> {
    info!("Applying School Blueprint for {}", org_id);
    let policy_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_policies (id, org_id, grading_system, pass_mark, is_active, created_at, updated_at) VALUES ($1, $2, 'Percentage', 50.0, 1, $3, $4)",
        vec![policy_id.into(), org_id.into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    // Seed default period templates
    let s1_id = Uuid::new_v4().to_string();
    let s2_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO period_templates (id, org_id, name, type_name, duration_weeks, is_active) VALUES ($1, $2, 'Semester 1', 'Semester', 16, 1), ($3, $4, 'Semester 2', 'Semester', 16, 1)",
        vec![s1_id.into(), org_id.into(), s2_id.into(), org_id.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    Ok(())
}

async fn apply_university_blueprint(db: &DatabaseConnection, org_id: &str, now: &str) -> Result<(), AppError> {
    info!("Applying University Blueprint for {}", org_id);
    let policy_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_policies (id, org_id, grading_system, pass_mark, is_active, created_at, updated_at) VALUES ($1, $2, 'GPA_4_0', 60.0, 1, $3, $4)",
        vec![policy_id.into(), org_id.into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    let fall_id = Uuid::new_v4().to_string();
    let spring_id = Uuid::new_v4().to_string();
    let summer_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO period_templates (id, org_id, name, type_name, duration_weeks, is_active) VALUES ($1, $2, 'Fall Semester', 'Semester', 16, 1), ($3, $4, 'Spring Semester', 'Semester', 16, 1), ($5, $6, 'Summer Term', 'Term', 8, 1)",
        vec![fall_id.into(), org_id.into(), spring_id.into(), org_id.into(), summer_id.into(), org_id.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    Ok(())
}

async fn apply_training_blueprint(db: &DatabaseConnection, org_id: &str, now: &str) -> Result<(), AppError> {
    info!("Applying Training Center Blueprint for {}", org_id);
    let policy_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_policies (id, org_id, grading_system, pass_mark, is_active, created_at, updated_at) VALUES ($1, $2, 'PassFail', 70.0, 1, $3, $4)",
        vec![policy_id.into(), org_id.into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    let q1_id = Uuid::new_v4().to_string();
    let q2_id = Uuid::new_v4().to_string();
    let q3_id = Uuid::new_v4().to_string();
    let q4_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO period_templates (id, org_id, name, type_name, duration_weeks, is_active) VALUES ($1, $2, 'Q1', 'Quarter', 12, 1), ($3, $4, 'Q2', 'Quarter', 12, 1), ($5, $6, 'Q3', 'Quarter', 12, 1), ($7, $8, 'Q4', 'Quarter', 12, 1)",
        vec![q1_id.into(), org_id.into(), q2_id.into(), org_id.into(), q3_id.into(), org_id.into(), q4_id.into(), org_id.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    Ok(())
}



