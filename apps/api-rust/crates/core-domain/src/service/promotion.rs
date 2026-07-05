//! Promotion Engine — orchestrates the multi-phase academic promotion workflow.
//!
//! # Workflow
//!
//! 1. **Dry Run** — Analyze students, apply rules, generate preview
//! 2. **Override** — Manual adjustments to individual decisions
//! 3. **Execute** — Create backup → Create target DB → Copy data → Create enrollments
//! 4. **Rollback** — Restore backup, delete target year
//!
//! All state transitions are recorded in the registry database.

use std::path::Path;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set, ConnectionTrait};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::database::registry;
use crate::entities::{promotion_job, promotion_detail, student_registry_entry};
use crate::repository::promotion::PromotionRepo;

// ── Types ────────────────────────────────────────────────────────

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
    pub target_year_id: String,
    pub target_year_name: String,
    pub total: i32,
    pub promoted: i32,
    pub retained: i32,
    pub graduated: i32,
    pub excluded: i32,
    pub backup_path: String,
}

/// Enrollment data extracted from the year database for analysis.
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

// ── Service ──────────────────────────────────────────────────────

pub struct PromotionService;

impl PromotionService {
    /// Phase 1: Dry Run — analyze all students and generate a preview.
    /// No data is modified in the year database.
    pub async fn dry_run(
        registry_db: &DatabaseConnection,
        year_db: &DatabaseConnection,
        org_id: &str,
        user_id: &str,
        config: PromotionConfig,
    ) -> Result<PromotionPreview, String> {
        let config_json = serde_json::to_string(&config)
            .map_err(|e| format!("Config serialization failed: {}", e))?;

        // Create the job record
        let now = chrono::Utc::now().to_rfc3339();
        let job = promotion_job::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            source_year_id: config.source_year_id.clone(),
            target_year_id: None,
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
            .map_err(|e| format!("Failed to create job: {}", e))?;

        // Fetch all active enrollments with class/student info
        let enrollments = Self::fetch_enrollment_data(year_db, org_id).await?;

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
                target_level: target_level.clone(),
                decision: decision.clone(),
                reason: reason.clone(),
                final_score: e.final_score,
                attendance_rate: Some(e.attendance_rate),
                manual_override: 0,
                override_by: None,
                override_reason: None,
                original_decision: None,
                created_at: chrono::Utc::now().to_rfc3339(),
            });
        }

        // Batch insert details
        PromotionRepo::create_details_batch(registry_db, details)
            .await
            .map_err(|e| format!("Failed to save details: {}", e))?;

        let total = enrollments.len() as i32;

        // Build class plan
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

        // Save dry run summary
        let summary_json = serde_json::json!({
            "total": total, "promoted": promoted,
            "retained": retained, "graduated": graduated, "excluded": excluded,
        });
        PromotionRepo::save_dry_run_result(registry_db, &job.id, &summary_json.to_string())
            .await
            .map_err(|e| format!("Failed to save dry run result: {}", e))?;

        // Build preview details for response
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
            job_id: job.id,
            total, promoted, retained, graduated, excluded,
            details: preview_details,
            class_plan,
        })
    }

    /// Phase 3: Execute — create target year DB, copy data, create enrollments.
    pub async fn execute(
        registry_db: &DatabaseConnection,
        source_db: &DatabaseConnection,
        job_id: &str,
        app_data_dir: &Path,
    ) -> Result<PromotionResult, String> {
        // 1. Load job and config
        let job = PromotionRepo::get_job_by_id(registry_db, job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Job not found")?;

        if job.status != "dry_run" && job.status != "confirmed" {
            return Err(format!("Cannot execute job in status '{}'", job.status));
        }

        let config: PromotionConfig = serde_json::from_str(&job.config)
            .map_err(|e| format!("Invalid config: {}", e))?;

        // 2. Update status to executing
        PromotionRepo::update_job_status(registry_db, job_id, "executing")
            .await
            .map_err(|e| e.to_string())?;

        // 3. Backup the source year database
        let source_year = crate::repository::academic_year::AcademicYearRepo::get_by_id(
            registry_db, &config.source_year_id,
        ).await.map_err(|e| e.to_string())?.ok_or("Source year not found")?;

        let backup_filename = registry::backup_filename(&source_year.name);
        let backup_path = registry::backups_dir(app_data_dir).join(&backup_filename);
        let source_db_path = registry::year_db_path(app_data_dir, &source_year.db_filename);

        registry::backup_database(&source_db_path, &backup_path).await?;
        info!("💾 Backup created: {:?}", backup_path);

        // 4. Create target year database with schema
        let target_db_filename = registry::year_db_filename(&config.target_year_name);
        let target_db_path = registry::year_db_path(app_data_dir, &target_db_filename);
        let target_db = registry::create_year_database(&target_db_path)
            .await
            .map_err(|e| format!("Failed to create target DB: {}", e))?;

        // 5. Copy reference data from source to target
        Self::copy_reference_data(source_db, &target_db, &config).await?;
        info!("📋 Reference data copied to target year");

        // 6. Create classes in target DB based on class_mapping
        Self::create_target_classes(&target_db, source_db, &config).await?;
        info!("🏫 Target classes created");

        // 7. Create enrollments for promoted students
        let details = PromotionRepo::get_details(registry_db, job_id)
            .await
            .map_err(|e| e.to_string())?;

        let promoted_details: Vec<_> = details.iter()
            .filter(|d| d.decision == "promoted")
            .collect();

        Self::create_target_enrollments(&target_db, &promoted_details, &config).await?;
        info!("📝 {} enrollments created in target year", promoted_details.len());

        // 8. Register target year in registry
        let now = chrono::Utc::now().to_rfc3339();
        let target_year = crate::entities::academic_year::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: job.org_id.clone(),
            name: config.target_year_name.clone(),
            name_ar: config.target_year_name_ar.clone(),
            start_date: config.target_start_date.clone(),
            end_date: config.target_end_date.clone(),
            hijri_start_date: None,
            hijri_end_date: None,
            db_filename: target_db_filename,
            status: "active".to_string(),
            is_current: 0,
            archived_at: None,
            archived_by: None,
            promotion_source_year_id: Some(config.source_year_id.clone()),
            db_size_bytes: None,
            student_count: Some(promoted_details.len() as i32),
            teacher_count: None,
            class_count: Some(config.class_mapping.iter().flat_map(|m| &m.target_class_names).count() as i32),
            notes: None,
            created_at: now.clone(),
            updated_at: now,
        };

        let target_year = crate::repository::academic_year::AcademicYearRepo::create(
            registry_db, target_year,
        ).await.map_err(|e| e.to_string())?;

        // 9. Update student registry entries
        Self::update_student_registry(registry_db, &details, &target_year.id).await?;

        // 10. Save execution stats
        let total = details.len() as i32;
        let promoted = details.iter().filter(|d| d.decision == "promoted").count() as i32;
        let retained_count = details.iter().filter(|d| d.decision == "retained").count() as i32;
        let graduated_count = details.iter().filter(|d| d.decision == "graduated").count() as i32;
        let excluded_count = details.iter().filter(|d| d.decision == "excluded").count() as i32;

        PromotionRepo::save_execution_stats(
            registry_db, job_id, total, promoted, retained_count,
            graduated_count, 0, excluded_count,
            &target_year.id, &backup_path.to_string_lossy(),
        ).await.map_err(|e| e.to_string())?;

        // 11. Mark job as completed
        PromotionRepo::update_job_status(registry_db, job_id, "completed")
            .await
            .map_err(|e| e.to_string())?;

        info!("✅ Promotion completed: {} promoted, {} retained, {} graduated",
              promoted, retained_count, graduated_count);

        Ok(PromotionResult {
            job_id: job_id.to_string(),
            target_year_id: target_year.id,
            target_year_name: config.target_year_name,
            total,
            promoted,
            retained: retained_count,
            graduated: graduated_count,
            excluded: excluded_count,
            backup_path: backup_path.to_string_lossy().to_string(),
        })
    }

    /// Rollback a completed promotion.
    pub async fn rollback(
        registry_db: &DatabaseConnection,
        job_id: &str,
        user_id: &str,
        app_data_dir: &Path,
    ) -> Result<(), String> {
        let job = PromotionRepo::get_job_by_id(registry_db, job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Job not found")?;

        if job.status != "completed" {
            return Err(format!("Cannot rollback job in status '{}'", job.status));
        }
        if job.rollback_available != 1 {
            return Err("Rollback is no longer available".to_string());
        }

        // Delete the target year's database file
        if let Some(target_year_id) = &job.target_year_id {
            let target_year = crate::repository::academic_year::AcademicYearRepo::get_by_id(
                registry_db, target_year_id,
            ).await.map_err(|e| e.to_string())?;

            if let Some(year) = target_year {
                let target_path = registry::year_db_path(app_data_dir, &year.db_filename);
                if target_path.exists() {
                    std::fs::remove_file(&target_path)
                        .map_err(|e| format!("Failed to delete target DB: {}", e))?;
                }
                // Also clean WAL/SHM
                let _ = std::fs::remove_file(target_path.with_extension("db-wal"));
                let _ = std::fs::remove_file(target_path.with_extension("db-shm"));
            }
        }

        // Mark job as rolled back
        PromotionRepo::set_rolled_back(registry_db, job_id, user_id)
            .await
            .map_err(|e| e.to_string())?;

        info!("↩️ Promotion {} rolled back by {}", job_id, user_id);
        Ok(())
    }

    // ── Internal helpers ────────────────────────────────────────

    async fn fetch_enrollment_data(
        year_db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<EnrollmentData>, String> {
        // Fetch active enrollments with joined user/class data
        let sql = r#"
            SELECT
                e.id AS enrollment_id,
                e.student_id,
                u.full_name AS student_name,
                u.name_ar AS student_name_ar,
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
            JOIN users u ON u.id = e.student_id
            JOIN classes c ON c.id = e.class_id
            WHERE e.org_id = ?1
              AND e.status = 'active'
              AND e.is_deleted = 0
            ORDER BY c.level, c.name, u.full_name
        "#;

        let results = year_db.query_all(
            sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                sql,
                vec![org_id.into()],
            )
        ).await.map_err(|e| format!("Query failed: {}", e))?;

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
        // Exclude withdrawn/inactive
        if config.exclude_withdrawn && (e.status == "withdrawn" || e.status == "inactive") {
            return ("excluded".to_string(), "Student is withdrawn/inactive".to_string());
        }

        // Graduated if final level
        if config.auto_graduate_final_level {
            if let Some(ref final_level) = config.final_level {
                if e.level == *final_level {
                    return ("graduated".to_string(), format!("Completed final level ({})", e.level));
                }
            }
        }

        // Check score and attendance
        let score_pass = e.final_score.map(|s| s >= passing_score).unwrap_or(false);
        let attendance_pass = e.attendance_rate >= min_attendance;

        if score_pass && attendance_pass {
            ("promoted".to_string(), format!(
                "Score: {}/100 (pass ≥{}), Attendance: {:.1}% (min ≥{:.1}%)",
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

    /// Copy reference/structural data from source to target year database.
    /// Copies: organizations, org_units, users (all staff + promoted students),
    /// subjects, fee_templates, discount_templates, rooms, currencies,
    /// ledger_accounts (structure), permissions, role_permissions, document_templates.
    ///
    /// Uses raw SQL to read all rows from the source and insert them into the target,
    /// because SeaORM cannot ATTACH two separate SQLite connections.
    async fn copy_reference_data(
        source_db: &DatabaseConnection,
        target_db: &DatabaseConnection,
        _config: &PromotionConfig,
    ) -> Result<(), String> {
        // Tables to copy in dependency order (parents first)
        let tables = vec![
            "organizations",
            "org_units",
            "subjects",
            "rooms",
            "currencies",
            "ledger_accounts",
            "fee_templates",
            "discount_templates",
            "permissions",
            "role_permissions",
            "document_templates",
        ];

        for table in &tables {
            Self::copy_table_data(source_db, target_db, table).await?;
        }

        // Copy all non-deleted users (staff are always carried forward)
        Self::copy_table_data_where(
            source_db, target_db, "users", "WHERE is_deleted = 0"
        ).await?;

        // Copy user roles and permissions
        Self::copy_table_data(source_db, target_db, "user_roles").await?;
        Self::copy_table_data(source_db, target_db, "user_permissions").await?;

        // Copy guardians if configured
        if _config.carry_over_guardians {
            Self::copy_table_data(source_db, target_db, "student_guardians").await?;
        }

        // Copy academic_cycles (structure only — not enrollment data)
        Self::copy_table_data(source_db, target_db, "academic_cycles").await?;

        // Copy org_academic_policies and promotion_policies if they exist
        let _ = Self::copy_table_data(source_db, target_db, "org_academic_policies").await;
        let _ = Self::copy_table_data(source_db, target_db, "promotion_policies").await;

        info!("📋 Reference data copied: {} structural tables + users + roles", tables.len());
        Ok(())
    }

    /// Copy all rows from a table in source_db to target_db.
    async fn copy_table_data(
        source_db: &DatabaseConnection,
        target_db: &DatabaseConnection,
        table: &str,
    ) -> Result<(), String> {
        Self::copy_table_data_where(source_db, target_db, table, "").await
    }

    /// Copy rows from a table with an optional WHERE clause.
    async fn copy_table_data_where(
        source_db: &DatabaseConnection,
        target_db: &DatabaseConnection,
        table: &str,
        where_clause: &str,
    ) -> Result<(), String> {
        // 1. Get column names from the table
        let pragma_sql = format!("PRAGMA table_info({})", table);
        let columns_result = source_db.query_all(
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Sqlite, pragma_sql)
        ).await.map_err(|e| format!("PRAGMA for {} failed: {}", table, e))?;

        if columns_result.is_empty() {
            // Table doesn't exist in source — skip silently
            return Ok(());
        }

        let column_names: Vec<String> = columns_result.iter()
            .filter_map(|row| row.try_get::<String>("", "name").ok())
            .collect();

        if column_names.is_empty() {
            return Ok(());
        }

        let cols_csv = column_names.join(", ");
        let placeholders: Vec<String> = (1..=column_names.len())
            .map(|i| format!("?{}", i))
            .collect();
        let placeholders_csv = placeholders.join(", ");

        // 2. Read all rows from source
        let select_sql = format!("SELECT {} FROM {} {}", cols_csv, table, where_clause);
        let source_rows = source_db.query_all(
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::Sqlite, select_sql)
        ).await.map_err(|e| format!("SELECT from {} failed: {}", table, e))?;

        if source_rows.is_empty() {
            return Ok(());
        }

        // 3. Insert into target one row at a time using raw SQL values
        let insert_prefix = format!(
            "INSERT OR IGNORE INTO {} ({}) VALUES ({})",
            table, cols_csv, placeholders_csv
        );

        for row in &source_rows {
            let mut values: Vec<sea_orm::Value> = Vec::new();
            for col in &column_names {
                // Try string first (covers TEXT, which is most SQLite data)
                if let Ok(v) = row.try_get::<Option<String>>("", col.as_str()) {
                    values.push(v.into());
                } else if let Ok(v) = row.try_get::<Option<i64>>("", col.as_str()) {
                    values.push(v.into());
                } else if let Ok(v) = row.try_get::<Option<f64>>("", col.as_str()) {
                    values.push(v.into());
                } else {
                    values.push(sea_orm::Value::String(None));
                }
            }

            let _ = target_db.execute(
                sea_orm::Statement::from_sql_and_values(
                    sea_orm::DatabaseBackend::Sqlite,
                    &insert_prefix,
                    values,
                )
            ).await;
            // INSERT OR IGNORE — skip duplicates silently
        }

        info!("  → Copied {} rows from {}", source_rows.len(), table);
        Ok(())
    }

    /// Auto-generate classes in the target year based on class_mapping config.
    async fn create_target_classes(
        target_db: &DatabaseConnection,
        source_db: &DatabaseConnection,
        config: &PromotionConfig,
    ) -> Result<(), String> {
        for mapping in &config.class_mapping {
            // Get source classes at this level for reference (org_id, unit_id, cycle_id)
            let source_classes: Vec<crate::entities::class::Model> =
                crate::entities::class::Entity::find()
                    .filter(crate::entities::class::Column::Level.eq(&mapping.source_level))
                    .filter(crate::entities::class::Column::IsDeleted.eq(0))
                    .all(source_db)
                    .await
                    .map_err(|e| format!("Failed to read source classes: {}", e))?;

            let reference = source_classes.first();

            for class_name in &mapping.target_class_names {
                let now = chrono::Utc::now().to_rfc3339();
                let new_class = crate::entities::class::ActiveModel {
                    id: Set(uuid::Uuid::new_v4().to_string()),
                    org_id: Set(reference.map(|r| r.org_id.clone()).unwrap_or_default()),
                    unit_id: Set(reference.map(|r| r.unit_id.clone()).unwrap_or_default()),
                    cycle_id: Set(reference.map(|r| r.cycle_id.clone()).unwrap_or_default()),
                    name: Set(class_name.clone()),
                    name_ar: Set(None),
                    level: Set(mapping.target_level.clone()),
                    division: Set(None),
                    homeroom_teacher_id: Set(None), // Fresh — no assignments carried
                    capacity: Set(reference.and_then(|r| r.capacity)),
                    current_students: Set(Some(0)),
                    room_id: Set(None),
                    color: Set(reference.and_then(|r| r.color.clone())),
                    is_active: Set(1),
                    is_deleted: Set(0),
                    deleted_at: Set(None),
                    created_at: Set(now.clone()),
                    updated_at: Set(now),
                };
                new_class.insert(target_db).await
                    .map_err(|e| format!("Failed to create class '{}': {}", class_name, e))?;
            }
        }
        Ok(())
    }

    /// Create enrollment records for promoted students in the target year.
    async fn create_target_enrollments(
        target_db: &DatabaseConnection,
        promoted: &[&promotion_detail::Model],
        config: &PromotionConfig,
    ) -> Result<(), String> {
        // Build a map of target_level → classes for distribution
        let mut level_classes: std::collections::HashMap<String, Vec<crate::entities::class::Model>> =
            std::collections::HashMap::new();

        for mapping in &config.class_mapping {
            // Find classes at the target level in the target DB
            let classes: Vec<crate::entities::class::Model> =
                crate::entities::class::Entity::find()
                    .filter(crate::entities::class::Column::Level.eq(&mapping.target_level))
                    .filter(crate::entities::class::Column::IsDeleted.eq(0))
                    .all(target_db)
                    .await
                    .map_err(|e| format!("Failed to find target classes: {}", e))?;

            level_classes.insert(mapping.target_level.clone(), classes);
        }

        for detail in promoted {
            let target_level = detail.target_level.as_deref().unwrap_or("");
            let classes = level_classes.get(target_level).cloned().unwrap_or_default();

            // Balanced distribution: assign to class with fewest students
            let target_class = if classes.is_empty() {
                return Err(format!("No target classes for level '{}'", target_level));
            } else {
                // Simple round-robin for now
                let idx = promoted.iter()
                    .filter(|d| d.target_level.as_deref() == Some(target_level))
                    .position(|d| d.id == detail.id)
                    .unwrap_or(0);
                classes[idx % classes.len()].clone()
            };

            let now = chrono::Utc::now().to_rfc3339();
            let enrollment = crate::entities::enrollment::ActiveModel {
                id: Set(uuid::Uuid::new_v4().to_string()),
                org_id: Set(target_class.org_id.clone()),
                cycle_id: Set(target_class.cycle_id.clone()),
                student_id: Set(detail.student_id.clone()),
                class_id: Set(target_class.id.clone()),
                student_number: Set(detail.student_number.clone().unwrap_or_default()),
                status: Set("active".to_string()),
                enrollment_date: Set(now.clone()),
                enrollment_type: Set("promotion".to_string()),
                withdrawal_date: Set(None),
                withdrawal_reason: Set(None),
                final_grade: Set(None),
                final_score: Set(None),
                final_rank: Set(None),
                is_promoted: Set(None),
                promotion_decision: Set(None),
                promotion_notes: Set(None),
                meta: Set(None),
                created_at: Set(now.clone()),
                updated_at: Set(now),
            };

            let _ = enrollment.insert(target_db).await;
        }

        Ok(())
    }

    /// Update the central student registry with new year/level info.
    async fn update_student_registry(
        registry_db: &DatabaseConnection,
        details: &[promotion_detail::Model],
        target_year_id: &str,
    ) -> Result<(), String> {
        for detail in details {
            let now = chrono::Utc::now().to_rfc3339();
            let status = match detail.decision.as_str() {
                "promoted" => "active",
                "graduated" => "graduated",
                "excluded" => "withdrawn",
                _ => "active",
            };

            let entry = student_registry_entry::Model {
                id: uuid::Uuid::new_v4().to_string(),
                org_id: String::new(), // Will be set from context
                user_id: detail.student_id.clone(),
                student_number: detail.student_number.clone().unwrap_or_default(),
                full_name: detail.student_name.clone(),
                name_ar: detail.student_name_ar.clone(),
                national_id: None,
                status: status.to_string(),
                first_enrollment_year_id: None,
                current_year_id: if detail.decision == "promoted" { Some(target_year_id.to_string()) } else { None },
                current_level: detail.target_level.clone(),
                current_class_name: detail.target_class_name.clone(),
                years_enrolled: 1,
                created_at: now.clone(),
                updated_at: now,
            };

            PromotionRepo::upsert_student_registry(registry_db, entry)
                .await
                .map_err(|e| format!("Failed to update student registry: {}", e))?;
        }

        Ok(())
    }
}
