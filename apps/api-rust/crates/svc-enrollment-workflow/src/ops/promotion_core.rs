//! Promotion Engine — orchestrates the multi-phase academic promotion workflow.


use sea_orm::{DatabaseConnection, ConnectionTrait, ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use tracing::info;

use shared_kernel::errors::AppError;
use crate::entities::{promotion_job, promotion_detail};


use svc_academic::entities::{class, enrollment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionConfig {
    pub source_period_id: String,
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
    pub distribution_strategy: String, // "balanced" | "keep_order" | "manual"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionPreview {
    pub job_id: String,
    pub total: i32,
    pub promoted: i32,
    pub retained: i32,
    pub graduated: i32,
    pub excluded: i32,
    pub details: Vec<PromotionDetailPreview>,
    pub class_plan: Vec<ClassPlanEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionDetailPreview {
    pub student_id: String,
    pub student_name: String,
    pub student_name_ar: Option<String>,
    pub student_number: Option<String>,
    pub source_class: String,
    pub source_level: String,
    pub decision: String,
    pub reason: String,
    pub target_level: Option<String>,
    pub target_class: Option<String>,
    pub final_score: Option<i32>,
    pub attendance_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPlanEntry {
    pub source_level: String,
    pub target_level: String,
    pub target_classes: Vec<String>,
    pub student_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionResult {
    pub job_id: String,
    pub target_period_id: String,
    pub target_year_name: String,
    pub total: i32,
    pub promoted: i32,
    pub retained: i32,
    pub graduated: i32,
    pub excluded: i32,
}

#[derive(Debug)]
pub struct EnrollmentData {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub student_name_ar: Option<String>,
    pub student_number: Option<String>,
    pub class_id: String,
    pub class_name: String,
    pub level: String,
    pub division: Option<String>,
    pub status: String,
    pub final_score: Option<i32>,
    pub attendance_rate: f64,
}

pub struct PromotionService {
    
    
}

impl PromotionService {
    pub fn new() -> Self {
        Self {}
    }

    /// Phase 1: Dry Run — analyze all students and generate a preview.
    pub async fn dry_run(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        user_id: &str,
        config: PromotionConfig,
    ) -> Result<PromotionPreview, AppError> {
        let config_json = serde_json::to_string(&config)
            .map_err(|e| AppError::internal(format!("Config serialization failed: {}", e)))?;

        let now = chrono::Utc::now().to_rfc3339();
        let job = promotion_job::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            source_period_id: config.source_period_id.clone(),
            target_period_id: None,
            status: "validating".to_string(),
            initiated_by: user_id.to_string(),
            initiated_by_name: None,
            config: config_json,
            dry_run_result: None,
            dry_run_at: None,
            total_students: None,
            promoted_count: None,
            retained_count: None,
            graduated_count: None,
            withdrawn_count: None,
            excluded_count: None,
            error_log: None,
            rollback_available: 1,
            started_at: None,
            completed_at: None,
            rolled_back_at: None,
            rolled_back_by: None,
            created_at: now.clone(),
            updated_at: now,
        };

        let active: promotion_job::ActiveModel = job.into(); let job = active.insert(db).await?;

        let enrollments = Self::fetch_enrollment_data(db, org_id, &config.source_period_id).await?;

        let passing_score = config.passing_score.unwrap_or(50);
        let min_attendance = config.min_attendance_rate.unwrap_or(75.0);

        let mut details = Vec::new();
        let mut promoted = 0i32;
        let mut retained = 0i32;
        let mut graduated = 0i32;
        let mut excluded = 0i32;

        for e in &enrollments {
            let (decision, reason) = Self::evaluate_student(
                e, &config, passing_score, min_attendance,
            );

            let target_level = if decision == "promoted" {
                config.class_mapping.iter()
                    .find(|m| m.source_level == e.level)
                    .map(|m| m.target_level.clone())
            } else {
                None
            };

            match decision.as_str() {
                "promoted" => promoted += 1,
                "retained" => retained += 1,
                "graduated" => graduated += 1,
                _ => excluded += 1,
            }

            details.push(promotion_detail::Model {
                id: uuid::Uuid::new_v4().to_string(),
                job_id: job.id.clone(),
                student_id: e.student_id.clone(),
                student_name: e.student_name.clone(),
                student_name_ar: e.student_name_ar.clone(),
                student_number: e.student_number.clone(),
                source_enrollment_id: e.enrollment_id.clone(),
                source_class_id: e.class_id.clone(),
                source_class_name: Some(e.class_name.clone()),
                source_level: e.level.clone(),
                source_division: e.division.clone(),
                target_enrollment_id: None,
                target_class_id: None,
                target_class_name: None,
                target_level,
                decision,
                reason,
                final_score: e.final_score,
                attendance_rate: Some(e.attendance_rate),
                manual_override: 0,
                override_by: None,
                override_reason: None,
                original_decision: None,
                created_at: chrono::Utc::now().to_rfc3339(),
            });
        }

        if !details.is_empty() { promotion_detail::Entity::insert_many(details.into_iter().map(Into::<promotion_detail::ActiveModel>::into)).exec(db).await?; }

        let total = enrollments.len() as i32;

        let class_plan: Vec<ClassPlanEntry> = config.class_mapping.iter().map(|m| {
            let count = enrollments.iter()
                .filter(|e| e.level == m.source_level)
                .count() as i32;
            ClassPlanEntry {
                source_level: m.source_level.clone(),
                target_level: m.target_level.clone(),
                target_classes: m.target_class_names.clone(),
                student_count: count,
            }
        }).collect();

        let summary_json = serde_json::json!({
            "total": total, "promoted": promoted,
            "retained": retained, "graduated": graduated, "excluded": excluded,
        });

        let mut updated_job = job;
        updated_job.status = "dry_run".to_string();
        updated_job.dry_run_result = Some(summary_json.to_string());
        updated_job.dry_run_at = Some(chrono::Utc::now().to_rfc3339());
        updated_job.updated_at = chrono::Utc::now().to_rfc3339();
        let active: promotion_job::ActiveModel = updated_job.clone().into(); active.update(db).await?;

        let preview_details: Vec<PromotionDetailPreview> = enrollments.iter().enumerate().map(|(i, e)| {
            PromotionDetailPreview {
                student_id: e.student_id.clone(),
                student_name: e.student_name.clone(),
                student_name_ar: e.student_name_ar.clone(),
                student_number: e.student_number.clone(),
                source_class: e.class_name.clone(),
                source_level: e.level.clone(),
                decision: if i < promoted as usize { "promoted" }
                    else if i < (promoted + retained) as usize { "retained" }
                    else if i < (promoted + retained + graduated) as usize { "graduated" }
                    else { "excluded" }.to_string(),
                reason: String::new(),
                target_level: None,
                target_class: None,
                final_score: e.final_score,
                attendance_rate: Some(e.attendance_rate),
            }
        }).collect();

        Ok(PromotionPreview {
            job_id: updated_job.id,
            total, promoted, retained, graduated, excluded,
            details: preview_details,
            class_plan,
        })
    }

    /// Phase 3: Execute — create target year and copy classes/enrollments.
    pub async fn execute(
        &self,
        db: &DatabaseConnection,
        job_id: &str,
    ) -> Result<PromotionResult, AppError> {
        let mut job = promotion_job::Entity::find_by_id(job_id).one(db).await?.ok_or_else(|| AppError::not_found("job", job_id))?;

        if job.status != "dry_run" && job.status != "confirmed" {
            return Err(AppError::validation(format!("Cannot execute job in status '{}'", job.status)));
        }

        let config: PromotionConfig = serde_json::from_str(&job.config)
            .map_err(|e| AppError::internal(format!("Invalid config: {}", e)))?;

        job.status = "executing".to_string();
        job.started_at = Some(chrono::Utc::now().to_rfc3339());
        job.updated_at = chrono::Utc::now().to_rfc3339();
        let active: promotion_job::ActiveModel = job.clone().into(); active.update(db).await?;

        // 1. Create target year
        let now = chrono::Utc::now().to_rfc3339();
        let target_year = svc_calendar::entities::academic_year::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: job.org_id.clone(),
            name: config.target_year_name.clone(),
            name_ar: config.target_year_name_ar.clone(),
            start_date: config.target_start_date.clone(),
            end_date: config.target_end_date.clone(),
            hijri_start: None,
            hijri_end: None,
            status: "active".to_string(),
            is_locked: 0,
            locked_at: None,
            locked_by: None,
            lock_reason: None,
            current_period_id: None,
            archived_at: None,
            archived_by: None,
            notes: None,
            created_at: now.clone(),
            updated_at: now,
        };

        let active: svc_calendar::entities::academic_year::ActiveModel = target_year.into(); let target_year = active.insert(db).await?;
        
        // 2. Create target classes
        Self::create_target_classes(db, db, &config, &target_year.id).await?;
        info!("🏫 Target classes created");

        // 3. Create target enrollments
        let details = promotion_detail::Entity::find().filter(promotion_detail::Column::JobId.eq(job_id)).all(db).await?;
        let promoted_details: Vec<_> = details.iter()
            .filter(|d| d.decision == "promoted")
            .collect();

        Self::create_target_enrollments(db, &promoted_details, &config, &target_year.id).await?;
        info!("📝 {} enrollments created in target year", promoted_details.len());

        // 4. Save execution stats
        let total = details.len() as i32;
        let promoted_count = details.iter().filter(|d| d.decision == "promoted").count() as i32;
        let retained_count = details.iter().filter(|d| d.decision == "retained").count() as i32;
        let graduated_count = details.iter().filter(|d| d.decision == "graduated").count() as i32;
        let excluded_count = details.iter().filter(|d| d.decision == "excluded").count() as i32;

        let mut final_job = job;
        final_job.status = "completed".to_string();
        final_job.completed_at = Some(chrono::Utc::now().to_rfc3339());
        final_job.total_students = Some(total);
        final_job.promoted_count = Some(promoted_count);
        final_job.retained_count = Some(retained_count);
        final_job.graduated_count = Some(graduated_count);
        final_job.excluded_count = Some(excluded_count);
        final_job.target_period_id = Some(target_year.id.clone());
        final_job.updated_at = chrono::Utc::now().to_rfc3339();

        let active: promotion_job::ActiveModel = final_job.into(); active.update(db).await?;

        info!("✅ Promotion completed: {} promoted, {} retained, {} graduated",
              promoted_count, retained_count, graduated_count);

        Ok(PromotionResult {
            job_id: job_id.to_string(),
            target_period_id: target_year.id,
            target_year_name: config.target_year_name,
            total,
            promoted: promoted_count,
            retained: retained_count,
            graduated: graduated_count,
            excluded: excluded_count,
        })
    }

    /// Rollback a completed promotion.
    pub async fn rollback(
        &self,
        db: &DatabaseConnection,
        job_id: &str,
        user_id: &str,
    ) -> Result<(), AppError> {
        let mut job = promotion_job::Entity::find_by_id(job_id).one(db).await?.ok_or_else(|| AppError::not_found("job", job_id))?;

        if job.status != "completed" {
            return Err(AppError::validation(format!("Cannot rollback job in status '{}'", job.status)));
        }
        if job.rollback_available != 1 {
            return Err(AppError::validation("Rollback is no longer available"));
        }

        if let Some(target_period_id) = &job.target_period_id {
            // In a single DB, we delete the enrollments, classes, periods, and the academic year itself.
            let _ = db.execute(
                sea_orm::Statement::from_sql_and_values(
                    sea_orm::DatabaseBackend::Sqlite,
                    "DELETE FROM enrollments WHERE enrollment_type = 'promotion' AND org_id = ?",
                    vec![job.org_id.clone().into()],
                )
            ).await;
            
            // Delete the academic year
            svc_calendar::entities::academic_year::Entity::delete_by_id(target_period_id.to_string()).exec(db).await?;
        }

        job.status = "rolled_back".to_string();
        job.rolled_back_at = Some(chrono::Utc::now().to_rfc3339());
        job.rolled_back_by = Some(user_id.to_string());
        job.rollback_available = 0;
        job.updated_at = chrono::Utc::now().to_rfc3339();
        let active: promotion_job::ActiveModel = job.into(); active.update(db).await?;

        info!("↩️ Promotion {} rolled back by {}", job_id, user_id);
        Ok(())
    }

    // ── Internal Helpers ──

    async fn fetch_enrollment_data(
        db: &DatabaseConnection,
        org_id: &str,
        period_id: &str,
    ) -> Result<Vec<EnrollmentData>, AppError> {
        let sql = r#"
            SELECT
                e.id AS enrollment_id,
                e.student_id,
                p.full_name AS student_name,
                p.name_ar AS student_name_ar,
                e.student_number,
                e.class_id,
                c.name AS class_name,
                c.level,
                c.division,
                e.status,
                e.final_score,
                COALESCE(
                    (SELECT CAST(SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) AS FLOAT)
                     / NULLIF(COUNT(*), 0) * 100
                     FROM attendance a WHERE a.student_id = e.student_id),
                    100.0
                ) AS attendance_rate
            FROM enrollments e
            JOIN persons p ON p.id = e.student_id
            JOIN classes c ON c.id = e.class_id
            WHERE e.org_id = ?1
              AND e.period_id = ?2
              AND e.status = 'active'
              AND e.is_deleted = 0
            ORDER BY c.level, c.name, p.full_name
        "#;

        let results = db.query_all(
            sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                sql,
                vec![org_id.into(), period_id.into()],
            )
        ).await.map_err(|e| AppError::Database(e.to_string()))?;

        let mut data = Vec::new();
        for row in results {
            data.push(EnrollmentData {
                enrollment_id: row.try_get::<String>("", "enrollment_id").unwrap_or_default(),
                student_id: row.try_get::<String>("", "student_id").unwrap_or_default(),
                student_name: row.try_get::<String>("", "student_name").unwrap_or_default(),
                student_name_ar: row.try_get::<Option<String>>("", "student_name_ar").unwrap_or(None),
                student_number: row.try_get::<Option<String>>("", "student_number").unwrap_or(None),
                class_id: row.try_get::<String>("", "class_id").unwrap_or_default(),
                class_name: row.try_get::<String>("", "class_name").unwrap_or_default(),
                level: row.try_get::<String>("", "level").unwrap_or_default(),
                division: row.try_get::<Option<String>>("", "division").unwrap_or(None),
                status: row.try_get::<String>("", "status").unwrap_or_default(),
                final_score: row.try_get::<Option<i32>>("", "final_score").unwrap_or(None),
                attendance_rate: row.try_get::<f64>("", "attendance_rate").unwrap_or(100.0),
            });
        }

        Ok(data)
    }

    fn evaluate_student(
        e: &EnrollmentData,
        config: &PromotionConfig,
        passing_score: i32,
        min_attendance: f64,
    ) -> (String, String) {
        if config.exclude_withdrawn && (e.status == "withdrawn" || e.status == "inactive") {
            return ("excluded".to_string(), "Student is withdrawn/inactive".to_string());
        }

        if config.auto_graduate_final_level {
            if let Some(ref final_level) = config.final_level {
                if e.level == *final_level {
                    return ("graduated".to_string(), format!("Completed final level ({})", e.level));
                }
            }
        }

        let score_pass = e.final_score.map(|s| s >= passing_score).unwrap_or(false);
        let attendance_pass = e.attendance_rate >= min_attendance;

        if score_pass && attendance_pass {
            ("promoted".to_string(), format!(
                "Score: {}/100 (pass >= {}), Attendance: {:.1}% (min >= {:.1}%)",
                e.final_score.unwrap_or(0), passing_score, e.attendance_rate, min_attendance
            ))
        } else {
            let mut reasons = Vec::new();
            if !score_pass {
                reasons.push(format!("Score {} < {}", e.final_score.unwrap_or(0), passing_score));
            }
            if !attendance_pass {
                reasons.push(format!("Attendance {:.1}% < {:.1}%", e.attendance_rate, min_attendance));
            }
            ("retained".to_string(), format!("Retained: {}", reasons.join(", ")))
        }
    }


    async fn create_target_classes(
        target_db: &DatabaseConnection,
        source_db: &DatabaseConnection,
        config: &PromotionConfig,
        target_period_id: &str,
    ) -> Result<(), AppError> {
        for mapping in &config.class_mapping {
            let source_classes: Vec<class::Model> = class::Entity::find()
                .filter(class::Column::Level.eq(&mapping.source_level))
                .filter(class::Column::PeriodId.eq(&config.source_period_id))
                .filter(class::Column::IsDeleted.eq(0))
                .all(source_db)
                .await?;

            let reference = source_classes.first();

            for class_name in &mapping.target_class_names {
                let now = chrono::Utc::now().to_rfc3339();
                let new_class = class::ActiveModel {
                    id: sea_orm::Set(uuid::Uuid::new_v4().to_string()),
                    org_id: sea_orm::Set(reference.map(|r| r.org_id.clone()).unwrap_or_default()),
                    unit_id: sea_orm::Set(reference.map(|r| r.unit_id.clone()).unwrap_or_default()),
                    period_id: sea_orm::Set(target_period_id.to_string()),
                    name: sea_orm::Set(class_name.clone()),
                    name_ar: sea_orm::Set(None),
                    level: sea_orm::Set(mapping.target_level.clone()),
                    division: sea_orm::Set(None),
                    homeroom_teacher_id: sea_orm::Set(None),
                    capacity: sea_orm::Set(reference.and_then(|r| r.capacity)),
                    current_students: sea_orm::Set(Some(0)),
                    room_id: sea_orm::Set(None),
                    color: sea_orm::Set(reference.and_then(|r| r.color.clone())),
                    is_active: sea_orm::Set(1),
                    is_deleted: sea_orm::Set(0),
                    deleted_at: sea_orm::Set(None),
                    created_at: sea_orm::Set(now.clone()),
                    updated_at: sea_orm::Set(now),
                };
                new_class.insert(target_db).await?;
            }
        }
        Ok(())
    }

    async fn create_target_enrollments(
        target_db: &DatabaseConnection,
        promoted: &[&promotion_detail::Model],
        config: &PromotionConfig,
        target_period_id: &str,
    ) -> Result<(), AppError> {
        let mut level_classes: std::collections::HashMap<String, Vec<class::Model>> =
            std::collections::HashMap::new();

        for mapping in &config.class_mapping {
            let classes: Vec<class::Model> = class::Entity::find()
                .filter(class::Column::Level.eq(&mapping.target_level))
                .filter(class::Column::PeriodId.eq(target_period_id))
                .filter(class::Column::IsDeleted.eq(0))
                .all(target_db)
                .await?;

            level_classes.insert(mapping.target_level.clone(), classes);
        }

        for detail in promoted {
            let target_level = detail.target_level.as_deref().unwrap_or("");
            let classes = level_classes.get(target_level).cloned().unwrap_or_default();

            let target_class = if classes.is_empty() {
                return Err(AppError::validation(format!("No target classes for level '{}'", target_level)));
            } else {
                let idx = promoted.iter()
                    .filter(|d| d.target_level.as_deref() == Some(target_level))
                    .position(|d| d.id == detail.id)
                    .unwrap_or(0);
                classes[idx % classes.len()].clone()
            };

            let now = chrono::Utc::now().to_rfc3339();
            let enr = enrollment::ActiveModel {
                id: sea_orm::Set(uuid::Uuid::new_v4().to_string()),
                org_id: sea_orm::Set(target_class.org_id.clone()),
                period_id: sea_orm::Set(target_class.period_id.clone()),
                student_id: sea_orm::Set(detail.student_id.clone()),
                class_id: sea_orm::Set(target_class.id.clone()),
                student_number: sea_orm::Set(detail.student_number.clone().unwrap_or_default()),
                status: sea_orm::Set("active".to_string()),
                enrollment_date: sea_orm::Set(now.clone()),
                enrollment_type: sea_orm::Set("promotion".to_string()),
                withdrawal_date: sea_orm::Set(None),
                withdrawal_reason: sea_orm::Set(None),
                meta: sea_orm::Set(None),
                created_at: sea_orm::Set(now.clone()),
                updated_at: sea_orm::Set(now),
            };

            let _ = enr.insert(target_db).await;
        }

        Ok(())
    }


}

