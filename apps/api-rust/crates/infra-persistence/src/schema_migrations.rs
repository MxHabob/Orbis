//! Idempotent schema migrations applied after baseline on existing databases.

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};
use tracing::info;

/// Apply P0 migrations (safe to call on every startup).
pub async fn run_p0_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    migrate_academic_policies_branch_overrides(db).await?;
    migrate_process_log_schema(db).await?;
    migrate_grading_architecture(db).await?;
    Ok(())
}

/// Replace org_id UNIQUE with partial unique indexes so branch overrides work.
async fn migrate_academic_policies_branch_overrides(db: &DatabaseConnection) -> Result<(), DbErr> {
    let row = db
        .query_one(Statement::from_string(
            db.get_database_backend(),
            "SELECT name FROM sqlite_master WHERE type = 'index' AND name = 'idx_ap_org_default'",
        ))
        .await?;

    if row.is_some() {
        return Ok(());
    }

    info!("Applying P0 migration: academic_policies branch override indexes");

    db.execute_unprepared("PRAGMA foreign_keys=OFF").await?;

    db.execute_unprepared(
        r#"
        CREATE TABLE IF NOT EXISTS academic_policies__p0 (
          id TEXT PRIMARY KEY,
          party_id TEXT REFERENCES parties(id),
          org_id TEXT NOT NULL,
          org_unit_id TEXT,
          default_grading_system TEXT NOT NULL DEFAULT 'percentage',
          default_passing_grade INTEGER NOT NULL DEFAULT 50,
          default_max_grade INTEGER NOT NULL DEFAULT 100,
          enrollment_auto_close INTEGER NOT NULL DEFAULT 1,
          allow_late_enrollment INTEGER NOT NULL DEFAULT 0,
          late_enrollment_approval INTEGER NOT NULL DEFAULT 1,
          grade_entry_auto_close INTEGER NOT NULL DEFAULT 1,
          allow_grade_edit_after_close INTEGER NOT NULL DEFAULT 0,
          grade_edit_approval INTEGER NOT NULL DEFAULT 1,
          passing_score INTEGER NOT NULL DEFAULT 50,
          min_attendance_rate REAL NOT NULL DEFAULT 75.0,
          auto_graduate_final_level INTEGER NOT NULL DEFAULT 0,
          final_level TEXT,
          exclude_withdrawn INTEGER NOT NULL DEFAULT 1,
          autopilot_mode TEXT NOT NULL DEFAULT 'suggested'
            CHECK (autopilot_mode IN ('automatic','suggested','manual')),
          autopilot_enabled INTEGER NOT NULL DEFAULT 1,
          max_auto_retries INTEGER NOT NULL DEFAULT 3,
          pause_on_error INTEGER NOT NULL DEFAULT 0,
          auto_open_enrollment INTEGER NOT NULL DEFAULT 1,
          auto_close_enrollment INTEGER NOT NULL DEFAULT 1,
          auto_open_grade_entry INTEGER NOT NULL DEFAULT 1,
          auto_close_grade_entry INTEGER NOT NULL DEFAULT 1,
          auto_lock_periods INTEGER NOT NULL DEFAULT 1,
          auto_advance_periods INTEGER NOT NULL DEFAULT 0,
          auto_archive_years INTEGER NOT NULL DEFAULT 0,
          grace_days_after_deadline INTEGER NOT NULL DEFAULT 3,
          notify_before_deadline_days INTEGER NOT NULL DEFAULT 7,
          default_template_id TEXT,
          created_at TEXT NOT NULL,
          updated_at TEXT NOT NULL,
          FOREIGN KEY (org_id) REFERENCES organizations(id)
        )
        "#,
    )
    .await?;

    db.execute_unprepared(
        r#"
        INSERT OR REPLACE INTO academic_policies__p0
        SELECT * FROM academic_policies
        "#,
    )
    .await?;

    db.execute_unprepared("DROP TABLE academic_policies").await?;
    db.execute_unprepared("ALTER TABLE academic_policies__p0 RENAME TO academic_policies")
        .await?;

    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_ap_org_default \
         ON academic_policies(org_id) WHERE org_unit_id IS NULL",
    )
    .await?;

    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_ap_org_unit \
         ON academic_policies(org_id, org_unit_id) WHERE org_unit_id IS NOT NULL",
    )
    .await?;

    db.execute_unprepared("PRAGMA foreign_keys=ON").await?;

    info!("P0 migration complete: academic_policies branch overrides enabled");
    Ok(())
}

pub async fn run_p4_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    info!("Running P4 migrations for Party/Person model...");

    // Create parties and persons from existing users
    let migrate_sql = "
        INSERT INTO parties (id, party_type, created_at, updated_at)
        SELECT 
            'party_' || id, 
            'Person', 
            created_at, 
            updated_at 
        FROM users 
        WHERE party_id IS NULL;

        INSERT INTO persons (id, party_id, first_name, last_name, created_at, updated_at)
        SELECT 
            'person_' || id, 
            'party_' || id, 
            COALESCE(full_name, 'Unknown'), 
            '', 
            created_at, 
            updated_at 
        FROM users 
        WHERE party_id IS NULL;

        UPDATE users SET party_id = 'party_' || id WHERE party_id IS NULL;
    ";

    db.execute_unprepared(migrate_sql).await?;
    info!("P4 migration complete: users migrated to parties and persons");
    Ok(())
}

/// Ensure process_log table has steps and current_step columns on existing databases.
async fn migrate_process_log_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
    let backend = db.get_database_backend();
    if backend == sea_orm::DbBackend::Sqlite {
        let has_steps = db
            .query_one(Statement::from_string(
                backend,
                "SELECT 1 FROM pragma_table_info('process_log') WHERE name = 'steps'",
            ))
            .await?;

        if has_steps.is_none() {
            info!("Applying P0 migration: process_log steps and current_step columns");
            db.execute_unprepared("PRAGMA foreign_keys=OFF").await?;

            db.execute_unprepared(
                r#"
                CREATE TABLE IF NOT EXISTS process_log__p0 (
                    id              TEXT PRIMARY KEY,
                    process_type    TEXT NOT NULL,
                    org_id          TEXT NOT NULL,
                    status          TEXT NOT NULL,
                    steps           TEXT NOT NULL,
                    current_step    INTEGER NOT NULL DEFAULT 0,
                    started_at      TEXT NOT NULL,
                    completed_at    TEXT,
                    error           TEXT
                )
                "#,
            )
            .await?;

            let has_old = db
                .query_one(Statement::from_string(
                    backend,
                    "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'process_log'",
                ))
                .await?;

            if has_old.is_some() {
                let _ = db
                    .execute_unprepared(
                        "INSERT OR IGNORE INTO process_log__p0 (id, process_type, org_id, status, steps, current_step, started_at, completed_at, error) \
                         SELECT id, process_type, org_id, status, steps_json, 0, started_at, completed_at, error FROM process_log",
                    )
                    .await;
                let _ = db.execute_unprepared("DROP TABLE process_log").await;
            }

            db.execute_unprepared("ALTER TABLE process_log__p0 RENAME TO process_log").await?;
            db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_pl_org ON process_log(org_id, process_type, started_at)").await?;
            db.execute_unprepared("PRAGMA foreign_keys=ON").await?;
            info!("P0 migration complete: process_log columns updated");
        }
    }
    Ok(())
}

/// P5: Grading architecture — ranking policies, term/annual results, entity extensions.
/// Idempotent: all CREATE IF NOT EXISTS / column-existence checks.
async fn migrate_grading_architecture(db: &DatabaseConnection) -> Result<(), DbErr> {
    let backend = db.get_database_backend();

    // Guard: skip if already applied
    let has_ranking = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'ranking_policies'",
        ))
        .await?;

    if has_ranking.is_some() {
        return Ok(());
    }

    info!("Applying P5 migration: grading architecture (ranking, term results, annual results)");

    // ── ranking_policies ────────────────────────────────────────
    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS ranking_policies (
            id                      TEXT PRIMARY KEY,
            org_id                  TEXT NOT NULL,
            period_id               TEXT,
            name                    TEXT NOT NULL,
            is_enabled              INTEGER NOT NULL DEFAULT 1,
            ranking_scope           TEXT NOT NULL DEFAULT 'class',
            ranking_method          TEXT NOT NULL DEFAULT 'standard',
            tie_handling            TEXT NOT NULL DEFAULT 'same_rank',
            include_failed_students INTEGER NOT NULL DEFAULT 0,
            ranking_metric          TEXT NOT NULL DEFAULT 'total_score',
            created_at              TEXT NOT NULL,
            updated_at              TEXT NOT NULL,
            FOREIGN KEY (org_id) REFERENCES organizations(id)
        )"#,
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_rp_org ON ranking_policies(org_id)",
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_rp_org_period ON ranking_policies(org_id, period_id)",
    )
    .await?;

    // ── term_results ────────────────────────────────────────────
    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS term_results (
            id                      TEXT PRIMARY KEY,
            org_id                  TEXT NOT NULL,
            student_id              TEXT NOT NULL,
            term_period_id          TEXT NOT NULL,
            enrollment_id           TEXT NOT NULL,
            class_subject_id        TEXT NOT NULL,
            coursework_score        REAL NOT NULL DEFAULT 0,
            coursework_max          REAL NOT NULL DEFAULT 0,
            exam_score              REAL NOT NULL DEFAULT 0,
            exam_max                REAL NOT NULL DEFAULT 0,
            total_score             REAL NOT NULL DEFAULT 0,
            total_max               REAL NOT NULL DEFAULT 0,
            percentage              REAL NOT NULL DEFAULT 0,
            grade_label             TEXT,
            gpa_points              REAL,
            is_passing              INTEGER NOT NULL DEFAULT 0,
            rank                    INTEGER,
            rank_scope              TEXT,
            calculation_policy_id   TEXT,
            calculation_version     INTEGER NOT NULL DEFAULT 1,
            policy_snapshot         TEXT,
            status                  TEXT NOT NULL DEFAULT 'draft',
            finalized_at            TEXT,
            finalized_by            TEXT,
            created_at              TEXT NOT NULL,
            updated_at              TEXT NOT NULL,
            FOREIGN KEY (org_id) REFERENCES organizations(id)
        )"#,
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_tr_student_period ON term_results(org_id, student_id, term_period_id)",
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_tr_enrollment ON term_results(enrollment_id, term_period_id)",
    )
    .await?;

    // ── annual_results ──────────────────────────────────────────
    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS annual_results (
            id                      TEXT PRIMARY KEY,
            org_id                  TEXT NOT NULL,
            student_id              TEXT NOT NULL,
            year_period_id          TEXT NOT NULL,
            enrollment_id           TEXT NOT NULL,
            class_subject_id        TEXT NOT NULL,
            term_scores_json        TEXT NOT NULL DEFAULT '[]',
            annual_total            REAL NOT NULL DEFAULT 0,
            annual_max              REAL NOT NULL DEFAULT 0,
            percentage              REAL NOT NULL DEFAULT 0,
            grade_label             TEXT,
            gpa_points              REAL,
            is_passing              INTEGER NOT NULL DEFAULT 0,
            rank                    INTEGER,
            rank_scope              TEXT,
            calculation_policy_id   TEXT,
            calculation_version     INTEGER NOT NULL DEFAULT 1,
            policy_snapshot         TEXT,
            status                  TEXT NOT NULL DEFAULT 'draft',
            finalized_at            TEXT,
            finalized_by            TEXT,
            created_at              TEXT NOT NULL,
            updated_at              TEXT NOT NULL,
            FOREIGN KEY (org_id) REFERENCES organizations(id)
        )"#,
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_ar_student_year ON annual_results(org_id, student_id, year_period_id)",
    )
    .await?;

    // ── ALTER existing tables (add columns if missing) ──────────

    // assessment_schemes.period_id
    let has_period_id = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM pragma_table_info('assessment_schemes') WHERE name = 'period_id'",
        ))
        .await?;
    if has_period_id.is_none() {
        db.execute_unprepared("ALTER TABLE assessment_schemes ADD COLUMN period_id TEXT")
            .await?;
    }

    // assessment_schemes.is_archived
    let has_archived_as = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM pragma_table_info('assessment_schemes') WHERE name = 'is_archived'",
        ))
        .await?;
    if has_archived_as.is_none() {
        db.execute_unprepared("ALTER TABLE assessment_schemes ADD COLUMN is_archived INTEGER NOT NULL DEFAULT 0")
            .await?;
    }

    // calculation_policies.is_archived
    let has_archived_cp = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM pragma_table_info('calculation_policies') WHERE name = 'is_archived'",
        ))
        .await?;
    if has_archived_cp.is_none() {
        db.execute_unprepared("ALTER TABLE calculation_policies ADD COLUMN is_archived INTEGER NOT NULL DEFAULT 0")
            .await?;
    }

    info!("P5 migration complete: grading architecture tables created");
    Ok(())
}
