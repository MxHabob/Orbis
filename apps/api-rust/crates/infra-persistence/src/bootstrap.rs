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
    pub currency: Option<String>,
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
    let cur = params.currency.clone().unwrap_or_else(|| "SAR".to_string());

    info!("Bootstrapping system for organization '{}'", params.org_name);

    // 1. Insert Organization
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO organizations (id, name, slug, is_active, created_at, updated_at) VALUES ($1, $2, $3, 1, $4, $5)",
        vec![
            org_id.clone().into(),
            params.org_name.clone().into(),
            params.org_slug.clone().into(),
            now.clone().into(),
            now.clone().into()
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create organization: {}", e)))?;

    // 1b. Insert Base Currency
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO currencies (code, org_id, name, symbol, symbol_position, decimal_places, exchange_rate, is_base, is_active, updated_at) VALUES ($1, $2, $3, $4, 'after', 2, 1.0, 1, 1, $5)",
        vec![
            cur.clone().into(),
            org_id.clone().into(),
            cur.clone().into(),
            cur.clone().into(),
            now.clone().into(),
        ]
    )).await.ok();

    // 1.5 Insert Party for Admin
    let party_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO parties (id, party_type, created_at, updated_at) VALUES ($1, 'person', $2, $3)",
        vec![
            party_id.clone().into(),
            now.clone().into(),
            now.clone().into()
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create admin party: {}", e)))?;

    // 1.6 Insert Person for Admin
    let person_id = Uuid::new_v4().to_string();
    let name_parts: Vec<&str> = params.admin_name.split_whitespace().collect();
    let first_name = name_parts.first().unwrap_or(&"").to_string();
    let last_name = if name_parts.len() > 1 { name_parts[1..].join(" ") } else { first_name.clone() };
    
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO persons (id, party_id, org_id, first_name, last_name, full_name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        vec![
            person_id.clone().into(),
            party_id.clone().into(),
            org_id.clone().into(),
            first_name.into(),
            last_name.into(),
            params.admin_name.clone().into(),
            now.clone().into(),
            now.clone().into()
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create admin person: {}", e)))?;

    // 2. Insert Admin User
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO users (id, person_id, org_id, email, full_name, password_hash, status, primary_role, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, 'active', 'super_admin', $7, $8)",
        vec![
            user_id.clone().into(),
            person_id.clone().into(),
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
            "slug": params.org_slug
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
        "INSERT INTO academic_policies (id, org_id, default_grading_system, default_passing_score, default_max_score, created_at, updated_at) VALUES ($1, $2, 'percentage', 50.0, 100.0, $3, $4)",
        vec![policy_id.into(), org_id.into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    // Seed default period template (2 semesters, 2 quarters each)
    let tmpl_id = Uuid::new_v4().to_string();
    let structure = serde_json::json!([
        {
            "name": "الفصل الأول", "name_ar": "الفصل الأول", "type": "semester", "weight": 50,
            "children": [
                { "name": "الفترة الأولى", "type": "quarter", "weight": 25 },
                { "name": "الفترة الثانية", "type": "quarter", "weight": 25 }
            ]
        },
        {
            "name": "الفصل الثاني", "name_ar": "الفصل الثاني", "type": "semester", "weight": 50,
            "children": [
                { "name": "الفترة الثالثة", "type": "quarter", "weight": 25 },
                { "name": "الفترة الرابعة", "type": "quarter", "weight": 25 }
            ]
        }
    ]);
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) VALUES ($1, $2, 'School Year', 'العام الدراسي', 'school', $3, 1, $4, $5)",
        vec![tmpl_id.into(), org_id.into(), structure.to_string().into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    Ok(())
}

async fn apply_university_blueprint(db: &DatabaseConnection, org_id: &str, now: &str) -> Result<(), AppError> {
    info!("Applying University Blueprint for {}", org_id);
    let policy_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_policies (id, org_id, default_grading_system, default_passing_score, default_max_score, created_at, updated_at) VALUES ($1, $2, 'gpa_4_0', 60.0, 100.0, $3, $4)",
        vec![policy_id.into(), org_id.into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    let tmpl_id = Uuid::new_v4().to_string();
    let structure = serde_json::json!([
        { "name": "Fall Semester", "type": "semester", "weight": 40 },
        { "name": "Spring Semester", "type": "semester", "weight": 40 },
        { "name": "Summer Term", "type": "term", "weight": 20 }
    ]);
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO period_templates (id, org_id, name, institution_type, structure, is_default, created_at, updated_at) VALUES ($1, $2, 'University Year', 'university', $3, 1, $4, $5)",
        vec![tmpl_id.into(), org_id.into(), structure.to_string().into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    Ok(())
}

async fn apply_training_blueprint(db: &DatabaseConnection, org_id: &str, now: &str) -> Result<(), AppError> {
    info!("Applying Training Center Blueprint for {}", org_id);
    let policy_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_policies (id, org_id, default_grading_system, default_passing_score, default_max_score, created_at, updated_at) VALUES ($1, $2, 'pass_fail', 70.0, 100.0, $3, $4)",
        vec![policy_id.into(), org_id.into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    let tmpl_id = Uuid::new_v4().to_string();
    let structure = serde_json::json!([
        { "name": "Q1", "type": "quarter", "weight": 25 },
        { "name": "Q2", "type": "quarter", "weight": 25 },
        { "name": "Q3", "type": "quarter", "weight": 25 },
        { "name": "Q4", "type": "quarter", "weight": 25 }
    ]);
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO period_templates (id, org_id, name, institution_type, structure, is_default, created_at, updated_at) VALUES ($1, $2, 'Training Year', 'institute', $3, 1, $4, $5)",
        vec![tmpl_id.into(), org_id.into(), structure.to_string().into(), now.into(), now.into()]
    )).await.map_err(|e| AppError::internal(e.to_string()))?;

    Ok(())
}



