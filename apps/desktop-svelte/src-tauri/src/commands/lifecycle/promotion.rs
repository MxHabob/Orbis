use tauri::State;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use domain_lifecycle::entities::{promotion_job, promotion_detail};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Types ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionConfig {
    pub source_year_id: String,
    pub target_year_name: String,
    pub target_year_name_ar: Option<String>,
    pub target_start_date: String,
    pub target_end_date: String,
    pub passing_score: Option<i32>,
    pub min_attendance_rate: Option<f64>,
    pub auto_graduate_final_level: bool,
    pub final_level: Option<String>,
    pub exclude_withdrawn: bool,
    pub carry_over_outstanding_fees: bool,
    pub carry_over_guardians: bool,
    pub class_mapping: Vec<ClassMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMapping {
    pub source_level: String,
    pub target_level: String,
    pub target_class_names: Vec<String>,
    pub distribution_strategy: String,
}

#[derive(Debug, Deserialize)]
pub struct OverrideRequest {
    pub detail_id: String,
    pub new_decision: String,
    pub reason: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePromotionPolicyRequest {
    pub passing_score: i32,
    pub min_attendance_rate: Option<f64>,
    pub auto_graduate_final_level: bool,
    pub final_level: Option<String>,
    pub exclude_withdrawn: bool,
    pub carry_over_outstanding_fees: bool,
    pub carry_over_guardians: bool,
}

// ── Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn promotion_dry_run(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    config: PromotionConfig,
) -> CmdResult<promotion_job::Model> {
    let registry_db = state.provider.registry();
    let year_db = state.active_db().await;
    
    let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(registry_db.clone()));
    let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(registry_db.clone()));
    let service = domain_lifecycle::services::promotion::PromotionService::new(cmd, query);
    
    // Map to domain PromotionConfig
    let domain_config = domain_lifecycle::services::promotion::PromotionConfig {
        source_year_id: config.source_year_id.clone(),
        target_year_name: config.target_year_name.clone(),
        target_year_name_ar: config.target_year_name_ar.clone(),
        target_start_date: config.target_start_date.clone(),
        target_end_date: config.target_end_date.clone(),
        passing_score: config.passing_score,
        min_attendance_rate: config.min_attendance_rate,
        auto_graduate_final_level: config.auto_graduate_final_level,
        final_level: config.final_level.clone(),
        exclude_withdrawn: config.exclude_withdrawn,
        carry_over_outstanding_fees: config.carry_over_outstanding_fees,
        carry_over_guardians: config.carry_over_guardians,
        class_mapping: config.class_mapping.iter().map(|m| {
            domain_lifecycle::services::promotion::ClassMapping {
                source_level: m.source_level.clone(),
                target_level: m.target_level.clone(),
                target_class_names: m.target_class_names.clone(),
                distribution_strategy: m.distribution_strategy.clone(),
            }
        }).collect(),
    };

    let preview = service.dry_run(&year_db, &org_id, "system", domain_config)
        .await
        .map_err(CommandError::from)?;

    // Fetch the updated job model to return
    let job = domain_lifecycle::entities::promotion_job::Entity::find_by_id(&preview.job_id)
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Promotion job not found after dry run".to_string()))?;
        
    Ok(job)
}

#[tauri::command]
pub async fn get_promotion_jobs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<promotion_job::Model>> {
    let registry_db = state.provider.registry();
    let jobs = promotion_job::Entity::find()
        .filter(promotion_job::Column::OrgId.eq(org_id))
        .all(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(jobs)
}

#[tauri::command]
pub async fn get_promotion_job(
    state: State<'_, Arc<AppState>>,
    job_id: String,
) -> CmdResult<Option<promotion_job::Model>> {
    let registry_db = state.provider.registry();
    let job = promotion_job::Entity::find_by_id(job_id)
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(job)
}

#[tauri::command]
pub async fn get_promotion_details(
    state: State<'_, Arc<AppState>>,
    job_id: String,
) -> CmdResult<Vec<promotion_detail::Model>> {
    let registry_db = state.provider.registry();
    let details = promotion_detail::Entity::find()
        .filter(promotion_detail::Column::JobId.eq(job_id))
        .all(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(details)
}

#[tauri::command]
pub async fn override_promotion_decision(
    state: State<'_, Arc<AppState>>,
    request: OverrideRequest,
) -> CmdResult<promotion_detail::Model> {
    let registry_db = state.provider.registry();

    let detail = promotion_detail::Entity::find_by_id(&request.detail_id)
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Promotion detail not found".to_string()))?;

    let valid_decisions = ["promoted", "retained", "graduated", "excluded"];
    if !valid_decisions.contains(&request.new_decision.as_str()) {
        return Err(CommandError::from(format!(
            "Invalid decision: {}. Valid: {:?}",
            request.new_decision, valid_decisions
        )));
    }

    let mut active: promotion_detail::ActiveModel = detail.into();
    active.original_decision = Set(Some(active.decision.unwrap()));
    active.decision = Set(request.new_decision);
    active.manual_override = Set(1);
    active.override_by = Set(Some(request.user_id));
    active.override_reason = Set(Some(request.reason));
    
    let updated = active.update(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(updated)
}

#[tauri::command]
pub async fn execute_promotion(
    state: State<'_, Arc<AppState>>,
    job_id: String,
) -> CmdResult<promotion_job::Model> {
    let registry_db = state.provider.registry();
    let year_db = state.active_db().await;
    
    let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(registry_db.clone()));
    let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(registry_db.clone()));
    let service = domain_lifecycle::services::promotion::PromotionService::new(cmd, query);
    
    let _result = service.execute(&year_db, &job_id, state.provider.app_data_dir())
        .await
        .map_err(CommandError::from)?;
        
    let job = promotion_job::Entity::find_by_id(&job_id)
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Promotion job not found after execution".to_string()))?;
        
    Ok(job)
}

#[tauri::command]
pub async fn rollback_promotion(
    state: State<'_, Arc<AppState>>,
    job_id: String,
    user_id: String,
) -> CmdResult<()> {
    let registry_db = state.provider.registry();
    let year_db = state.active_db().await;
    
    let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(registry_db.clone()));
    let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(registry_db.clone()));
    let service = domain_lifecycle::services::promotion::PromotionService::new(cmd, query);
    
    service.rollback(&job_id, &user_id, state.provider.app_data_dir())
        .await
        .map_err(CommandError::from)?;
        
    Ok(())
}

// ── Promotion Policies ─────────────────────────────────────────────

#[tauri::command]
pub async fn get_promotion_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<domain_lifecycle::entities::promotion_policy::Model> {
    let db = state.active_db().await;
    let query = domain_lifecycle::repository::LifecycleQueryRepo::new(db.clone());
    let policy = query.get_promotion_policy(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .unwrap_or(domain_lifecycle::entities::promotion_policy::Model {
            id: format!("{}_policy", org_id),
            org_id: org_id.clone(),
            passing_score: 50,
            min_attendance_rate: 75.0,
            auto_graduate_final_level: 0,
            final_level: None,
            exclude_withdrawn: 1,
            fee_carry_over_mode: "ignore".to_string(),
            fee_carry_over_categories: None,
            carry_over_guardians: 1,
            carry_over_time_slots: 0,
            carry_over_class_subjects: 0,
            auto_create_cycles: 0,
            cycle_template: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        });
    Ok(policy)
}

#[tauri::command]
pub async fn update_promotion_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: UpdatePromotionPolicyRequest,
) -> CmdResult<domain_lifecycle::entities::promotion_policy::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = domain_lifecycle::repository::LifecycleCommandRepo::new(db.clone());
    
    let now = chrono::Utc::now().to_rfc3339();
    let model = domain_lifecycle::entities::promotion_policy::Model {
        id: format!("{}_policy", org_id),
        org_id: org_id.clone(),
        passing_score: request.passing_score,
        min_attendance_rate: request.min_attendance_rate.unwrap_or(75.0),
        auto_graduate_final_level: if request.auto_graduate_final_level { 1 } else { 0 },
        final_level: request.final_level,
        exclude_withdrawn: if request.exclude_withdrawn { 1 } else { 0 },
        fee_carry_over_mode: if request.carry_over_outstanding_fees { "carry_all".to_string() } else { "ignore".to_string() },
        fee_carry_over_categories: None,
        carry_over_guardians: if request.carry_over_guardians { 1 } else { 0 },
        carry_over_time_slots: 0,
        carry_over_class_subjects: 0,
        auto_create_cycles: 0,
        cycle_template: None,
        created_at: now.clone(),
        updated_at: now,
    };
    
    let policy = cmd.upsert_promotion_policy(model)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    Ok(policy)
}
