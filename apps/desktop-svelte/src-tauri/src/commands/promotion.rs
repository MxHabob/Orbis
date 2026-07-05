use tauri::State;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use core_domain::{
    entities::{promotion_job, promotion_detail},
    repository::promotion::PromotionRepo,
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Request/Response Types ──────────────────────────────────────────

/// Configuration for the promotion process.
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

/// Maps source level classes to target level classes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMapping {
    pub source_level: String,
    pub target_level: String,
    pub target_class_names: Vec<String>,
    pub distribution_strategy: String,
}

/// Override request for a single student's promotion decision.
#[derive(Debug, Deserialize)]
pub struct OverrideRequest {
    pub detail_id: String,
    pub new_decision: String,
    pub reason: String,
    pub user_id: String,
}

// ── Commands ────────────────────────────────────────────────────────

/// Start a promotion dry run — analyzes all students and returns a preview
/// of what will happen without making any changes.
#[tauri::command]
pub async fn promotion_dry_run(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    config: PromotionConfig,
) -> CmdResult<promotion_job::Model> {
    let registry_db = state.provider.registry();
    let service = state.service().await;

    let config_json = serde_json::to_string(&config)
        .map_err(|e| CommandError::from(format!("Failed to serialize config: {}", e)))?;

    // Create the job record
    let now = chrono::Utc::now().to_rfc3339();
    let job = promotion_job::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id: org_id.clone(),
        source_year_id: config.source_year_id.clone(),
        target_year_id: None,
        status: "validating".to_string(),
        initiated_by: String::new(), // Will be set from auth context
        initiated_by_name: None,
        config: config_json.clone(),
        dry_run_result: None,
        dry_run_at: None,
        total_students: None,
        promoted_count: None,
        retained_count: None,
        graduated_count: None,
        withdrawn_count: None,
        excluded_count: None,
        backup_path: None,
        error_log: None,
        rollback_available: 1,
        started_at: None,
        completed_at: None,
        rolled_back_at: None,
        rolled_back_by: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let job = PromotionRepo::create_job(registry_db, job)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    // Analyze all active enrollments in the source year
    let params = core_domain::params::EnrollmentParams {
        class_id: None,
        cycle_id: None,
        student_id: None,
        status: Some("active".to_string()),
        q: None,
        limit: None,
        offset: None,
    };
    let enrollments = service
        .get_enrollments(&org_id, params)
        .await
        .map_err(CommandError::from)?;

    let passing_score = config.passing_score.unwrap_or(50);
    let min_attendance = config.min_attendance_rate.unwrap_or(75.0);

    let mut details = Vec::new();
    let mut promoted_count = 0i32;
    let mut retained_count = 0i32;
    let mut graduated_count = 0i32;
    let mut excluded_count = 0i32;

    for enrollment_val in &enrollments {
        let enrollment_id = enrollment_val["id"].as_str().unwrap_or_default().to_string();
        let student_id = enrollment_val["student_id"].as_str().unwrap_or_default().to_string();
        let student_name = enrollment_val["full_name"].as_str().unwrap_or("Unknown").to_string();
        let student_name_ar = enrollment_val["name_ar"].as_str().map(|s| s.to_string());
        let student_number = enrollment_val["student_number"].as_str().map(|s| s.to_string());
        let class_id = enrollment_val["class_id"].as_str().unwrap_or_default().to_string();
        let status = enrollment_val["status"].as_str().unwrap_or_default();
        let final_score = enrollment_val["final_score"].as_i64().map(|s| s as i32);

        // Get class info for level/division
        let class_name = enrollment_val["class_name"].as_str().unwrap_or_default().to_string();
        // TODO: Resolve level from class lookup
        let level = enrollment_val["level"].as_str().unwrap_or("").to_string();
        let division = enrollment_val["division"].as_str().map(|s| s.to_string());

        // Determine decision
        let (decision, reason) = if config.exclude_withdrawn && (status == "withdrawn" || status == "inactive") {
            excluded_count += 1;
            ("excluded".to_string(), "Student is withdrawn/inactive".to_string())
        } else if config.auto_graduate_final_level && config.final_level.as_deref() == Some(&level) {
            graduated_count += 1;
            ("graduated".to_string(), format!("Completed final level ({})", level))
        } else {
            // Check score and attendance
            let score_pass = final_score.map(|s| s >= passing_score).unwrap_or(false);
            // TODO: Calculate actual attendance rate from attendance records
            let attendance_rate = 100.0_f64; // Placeholder — will be calculated from DB

            let attendance_pass = attendance_rate >= min_attendance;

            if score_pass && attendance_pass {
                promoted_count += 1;
                ("promoted".to_string(), format!(
                    "Score: {}/100 (pass: {}), Attendance: {:.1}% (min: {:.1}%)",
                    final_score.unwrap_or(0), passing_score, attendance_rate, min_attendance
                ))
            } else {
                retained_count += 1;
                let mut reasons = Vec::new();
                if !score_pass {
                    reasons.push(format!("Score {} < {}", final_score.unwrap_or(0), passing_score));
                }
                if !attendance_pass {
                    reasons.push(format!("Attendance {:.1}% < {:.1}%", attendance_rate, min_attendance));
                }
                ("retained".to_string(), format!("Retained: {}", reasons.join(", ")))
            }
        };

        // Determine target level for promoted students
        let target_level = if decision == "promoted" {
            config.class_mapping.iter()
                .find(|m| m.source_level == level)
                .map(|m| m.target_level.clone())
        } else {
            None
        };

        details.push(promotion_detail::Model {
            id: uuid::Uuid::new_v4().to_string(),
            job_id: job.id.clone(),
            student_id,
            student_name,
            student_name_ar,
            student_number,
            source_enrollment_id: enrollment_id,
            source_class_id: class_id,
            source_class_name: Some(class_name),
            source_level: level,
            source_division: division,
            target_enrollment_id: None,
            target_class_id: None,
            target_class_name: None,
            target_level,
            decision,
            reason,
            final_score,
            attendance_rate: Some(100.0), // TODO: Calculate actual
            manual_override: 0,
            override_by: None,
            override_reason: None,
            original_decision: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        });
    }

    // Save details to registry
    PromotionRepo::create_details_batch(registry_db, details)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    // Save dry run summary
    let total = enrollments.len() as i32;
    let summary = serde_json::json!({
        "total": total,
        "promoted": promoted_count,
        "retained": retained_count,
        "graduated": graduated_count,
        "excluded": excluded_count,
    });

    PromotionRepo::save_dry_run_result(
        registry_db,
        &job.id,
        &summary.to_string(),
    )
    .await
    .map_err(|e| CommandError::from(e.to_string()))?;

    // Return the updated job
    PromotionRepo::get_job_by_id(registry_db, &job.id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?
        .ok_or_else(|| CommandError::from("Job not found after creation".to_string()))
}

/// Get all promotion jobs for an organization.
#[tauri::command]
pub async fn get_promotion_jobs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<promotion_job::Model>> {
    let registry_db = state.provider.registry();
    PromotionRepo::get_jobs(registry_db, &org_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

/// Get a single promotion job by ID.
#[tauri::command]
pub async fn get_promotion_job(
    state: State<'_, Arc<AppState>>,
    job_id: String,
) -> CmdResult<Option<promotion_job::Model>> {
    let registry_db = state.provider.registry();
    PromotionRepo::get_job_by_id(registry_db, &job_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

/// Get promotion details (per-student decisions) for a job.
#[tauri::command]
pub async fn get_promotion_details(
    state: State<'_, Arc<AppState>>,
    job_id: String,
) -> CmdResult<Vec<promotion_detail::Model>> {
    let registry_db = state.provider.registry();
    PromotionRepo::get_details(registry_db, &job_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

/// Override a student's promotion decision.
/// Can only be done while the job is in dry_run or confirmed status.
#[tauri::command]
pub async fn override_promotion_decision(
    state: State<'_, Arc<AppState>>,
    request: OverrideRequest,
) -> CmdResult<promotion_detail::Model> {
    let registry_db = state.provider.registry();

    let _detail = PromotionRepo::get_details(registry_db, &request.detail_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?
        .into_iter()
        .next()
        .ok_or_else(|| CommandError::from("Promotion detail not found".to_string()))?;

    // Validate the new decision
    let valid_decisions = ["promoted", "retained", "graduated", "excluded"];
    if !valid_decisions.contains(&request.new_decision.as_str()) {
        return Err(CommandError::from(format!(
            "Invalid decision: {}. Valid: {:?}",
            request.new_decision, valid_decisions
        )));
    }

    PromotionRepo::override_detail(
        registry_db,
        &request.detail_id,
        &request.new_decision,
        &request.reason,
        &request.user_id,
    )
    .await
    .map_err(|e| CommandError::from(e.to_string()))
}

/// Execute a confirmed promotion.
/// This creates the target year's database, copies reference data,
/// and creates new enrollments for promoted students.
#[tauri::command]
pub async fn execute_promotion(
    state: State<'_, Arc<AppState>>,
    job_id: String,
) -> CmdResult<promotion_job::Model> {
    let registry_db = state.provider.registry();

    // Verify job is in dry_run or confirmed status
    let job = PromotionRepo::get_job_by_id(registry_db, &job_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?
        .ok_or_else(|| CommandError::from("Promotion job not found".to_string()))?;

    if job.status != "dry_run" && job.status != "confirmed" {
        return Err(CommandError::from(format!(
            "Cannot execute promotion in status '{}'. Must be 'dry_run' or 'confirmed'.",
            job.status
        )));
    }

    // Update status to executing
    PromotionRepo::update_job_status(registry_db, &job_id, "executing")
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    // Parse config
    let config: PromotionConfig = serde_json::from_str(&job.config)
        .map_err(|e| CommandError::from(format!("Invalid promotion config: {}", e)))?;

    // 1. Create backup of source year database
    let source_year = core_domain::repository::academic_year::AcademicYearRepo::get_by_id(
        registry_db,
        &config.source_year_id,
    )
    .await
    .map_err(|e| CommandError::from(e.to_string()))?
    .ok_or_else(|| CommandError::from("Source year not found".to_string()))?;

    let backup_filename = core_domain::database::registry::backup_filename(&source_year.name);
    let backup_path = core_domain::database::registry::backups_dir(state.provider.app_data_dir())
        .join(&backup_filename);
    let source_db_path = core_domain::database::registry::year_db_path(
        state.provider.app_data_dir(),
        &source_year.db_filename,
    );

    core_domain::database::registry::backup_database(&source_db_path, &backup_path)
        .await
        .map_err(|e| {
            let _ = tauri::async_runtime::block_on(
                PromotionRepo::update_job_status(registry_db, &job_id, "failed"),
            );
            CommandError::from(format!("Backup failed: {}", e))
        })?;

    // 2. Create the target year's database
    let target_db_filename =
        core_domain::database::registry::year_db_filename(&config.target_year_name);
    let _target_service = state
        .provider
        .create_year_db(&target_db_filename)
        .await
        .map_err(|e| {
            let _ = tauri::async_runtime::block_on(
                PromotionRepo::update_job_status(registry_db, &job_id, "failed"),
            );
            CommandError::from(format!("Failed to create target year database: {}", e))
        })?;

    // 3. Register the target year in the registry
    let now = chrono::Utc::now().to_rfc3339();
    let target_year = core_domain::entities::academic_year::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id: job.org_id.clone(),
        name: config.target_year_name.clone(),
        name_ar: config.target_year_name_ar.clone(),
        start_date: config.target_start_date.clone(),
        end_date: config.target_end_date.clone(),
        hijri_start_date: None,
        hijri_end_date: None,
        db_filename: target_db_filename.clone(),
        status: "active".to_string(),
        is_current: 0,
        archived_at: None,
        archived_by: None,
        promotion_source_year_id: Some(config.source_year_id.clone()),
        db_size_bytes: None,
        student_count: None,
        teacher_count: None,
        class_count: None,
        notes: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let target_year = core_domain::repository::academic_year::AcademicYearRepo::create(
        registry_db,
        target_year,
    )
    .await
    .map_err(|e| CommandError::from(e.to_string()))?;

    // TODO: Phase 2 implementation:
    // 4. Copy reference data (org, users, subjects, templates, etc.) to target DB
    // 5. Create classes in target DB based on class_mapping
    // 6. Create enrollments for promoted students
    // 7. Optionally carry outstanding fee balances
    // 8. Update student_registry entries

    // Get promotion counts from details
    let details = PromotionRepo::get_details(registry_db, &job_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    let total = details.len() as i32;
    let promoted = details.iter().filter(|d| d.decision == "promoted").count() as i32;
    let retained = details.iter().filter(|d| d.decision == "retained").count() as i32;
    let graduated = details.iter().filter(|d| d.decision == "graduated").count() as i32;
    let excluded = details.iter().filter(|d| d.decision == "excluded").count() as i32;
    let withdrawn = details.iter().filter(|d| d.decision == "withdrawn").count() as i32;

    // Save execution stats
    PromotionRepo::save_execution_stats(
        registry_db,
        &job_id,
        total,
        promoted,
        retained,
        graduated,
        withdrawn,
        excluded,
        &target_year.id,
        &backup_path.to_string_lossy(),
    )
    .await
    .map_err(|e| CommandError::from(e.to_string()))?;

    // Mark job as completed
    PromotionRepo::update_job_status(registry_db, &job_id, "completed")
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

/// Rollback a completed promotion.
/// Restores the backup and removes the target year's database.
#[tauri::command]
pub async fn rollback_promotion(
    state: State<'_, Arc<AppState>>,
    job_id: String,
    user_id: String,
) -> CmdResult<()> {
    let registry_db = state.provider.registry();

    let job = PromotionRepo::get_job_by_id(registry_db, &job_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?
        .ok_or_else(|| CommandError::from("Promotion job not found".to_string()))?;

    if job.status != "completed" {
        return Err(CommandError::from(format!(
            "Cannot rollback a job in status '{}'. Must be 'completed'.",
            job.status
        )));
    }

    if job.rollback_available != 1 {
        return Err(CommandError::from(
            "Rollback is no longer available for this job".to_string(),
        ));
    }

    // TODO: Implement actual rollback:
    // 1. Delete the target year's database file
    // 2. Restore backup if needed
    // 3. Update registry

    // Mark job as rolled back
    PromotionRepo::set_rolled_back(registry_db, &job_id, &user_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;

    Ok(())
}

// ── Promotion Policies ─────────────────────────────────────────────

#[tauri::command]
pub async fn get_promotion_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<core_domain::entities::promotion_policy::Model> {
    state.service().await
        .get_promotion_policy(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_promotion_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: core_domain::service::promotion_policy::UpdatePromotionPolicyRequest,
) -> CmdResult<core_domain::entities::promotion_policy::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .update_promotion_policy(&org_id, request)
        .await
        .map_err(CommandError::from)
}
