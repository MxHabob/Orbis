//! Idempotent schema migrations applied after baseline on existing databases.

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};
use tracing::info;

/// Apply P0 migrations (safe to call on every startup).
pub async fn run_p0_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // P0 is the legacy SQLite schema-repair bundle. PostgreSQL uses the
    // dedicated baseline schema and must not execute SQLite catalog queries.
    if db.get_database_backend() == sea_orm::DbBackend::Postgres {
        info!("Skipping SQLite-only P0 migrations for PostgreSQL");
        return Ok(());
    }

    migrate_academic_policies_branch_overrides(db).await?;
    migrate_process_log_schema(db).await?;
    migrate_grading_architecture(db).await?;
    migrate_p6_schema_corrections(db).await?;
    migrate_p7_grading_bindings(db).await?;
    migrate_p8_report_definitions(db).await?;
    Ok(())
}

/// Enable RLS on every table exposed through the public schema.
///
/// No policies are created here intentionally: the application connects as
/// the database owner, while PostgREST roles remain deny-by-default until
/// policies are defined for their authentication model.
pub async fn run_postgres_security_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    if db.get_database_backend() != sea_orm::DbBackend::Postgres {
        return Ok(());
    }

    db.execute_unprepared(
        r#"
        DO $$
        DECLARE
            table_name text;
        BEGIN
            FOR table_name IN
                SELECT tablename
                FROM pg_catalog.pg_tables
                WHERE schemaname = 'public'
            LOOP
                EXECUTE format('ALTER TABLE public.%I ENABLE ROW LEVEL SECURITY', table_name);
            END LOOP;
        END
        $$;
        "#,
    )
    .await?;

    info!("PostgreSQL security migration complete: RLS enabled on public tables");
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
            FOREIGN KEY (org_id) REFERENCES organizations(id),
            FOREIGN KEY (enrollment_id) REFERENCES enrollments(id),
            FOREIGN KEY (class_subject_id) REFERENCES class_subjects(id)
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
            FOREIGN KEY (org_id) REFERENCES organizations(id),
            FOREIGN KEY (enrollment_id) REFERENCES enrollments(id),
            FOREIGN KEY (class_subject_id) REFERENCES class_subjects(id)
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

/// P6: Schema corrections from architecture review.
///
/// 1. Add `org_id` to `attendance_summary` (tenant isolation fix).
/// 2. Rename `attendance.period` → `attendance.lesson_slot` (naming collision fix).
/// 3. Update enrollment uniqueness index to include `org_id` and exclude 'cancelled'.
///
/// All operations are idempotent.
async fn migrate_p6_schema_corrections(db: &DatabaseConnection) -> Result<(), DbErr> {
    let backend = db.get_database_backend();

    // ── 1. attendance_summary: add org_id if missing ──

    let has_org_id = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM pragma_table_info('attendance_summary') WHERE name = 'org_id'",
        ))
        .await?;

    if has_org_id.is_none() {
        info!("P6: Adding org_id to attendance_summary");

        // SQLite cannot add NOT NULL without default to existing table, so we rebuild.
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS attendance_summary__p6 (
                id TEXT PRIMARY KEY,
                org_id TEXT NOT NULL DEFAULT '',
                enrollment_id TEXT NOT NULL,
                period_id TEXT NOT NULL,
                total_lessons INTEGER,
                present_lessons INTEGER,
                absent_lessons INTEGER,
                late_lessons INTEGER,
                excused_lessons INTEGER,
                late_minutes INTEGER,
                attendance_rate REAL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (org_id) REFERENCES organizations(id),
                FOREIGN KEY (enrollment_id) REFERENCES enrollments(id)
            )"#,
        )
        .await?;

        // Copy existing data, backfilling org_id from enrollments
        db.execute_unprepared(
            r#"INSERT OR IGNORE INTO attendance_summary__p6
               (id, org_id, enrollment_id, period_id, total_lessons, present_lessons,
                absent_lessons, late_lessons, excused_lessons, late_minutes,
                attendance_rate, created_at, updated_at)
               SELECT s.id,
                      COALESCE(e.org_id, ''),
                      s.enrollment_id, s.period_id, s.total_lessons, s.present_lessons,
                      s.absent_lessons, s.late_lessons, s.excused_lessons, s.late_minutes,
                      s.attendance_rate, s.created_at, s.updated_at
               FROM attendance_summary s
               LEFT JOIN enrollments e ON e.id = s.enrollment_id"#,
        )
        .await?;

        db.execute_unprepared("DROP TABLE attendance_summary").await?;
        db.execute_unprepared("ALTER TABLE attendance_summary__p6 RENAME TO attendance_summary")
            .await?;

        info!("P6: attendance_summary rebuilt with org_id");
    }

    // ── 2. attendance: rename `period` → `lesson_slot` if needed ──

    let has_lesson_slot = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM pragma_table_info('attendance') WHERE name = 'lesson_slot'",
        ))
        .await?;

    if has_lesson_slot.is_none() {
        info!("P6: Renaming attendance.period → attendance.lesson_slot");

        // SQLite cannot rename columns (pre-3.25), so we rebuild the table.
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS attendance__p6 (
                id TEXT PRIMARY KEY,
                org_id TEXT NOT NULL,
                period_id TEXT NOT NULL,
                enrollment_id TEXT NOT NULL,
                class_id TEXT NOT NULL,
                date TEXT NOT NULL,
                status TEXT NOT NULL,
                check_in_time TEXT,
                check_out_time TEXT,
                lesson_slot TEXT,
                subject_name TEXT,
                recorded_by TEXT NOT NULL,
                notes TEXT,
                excuse_type TEXT,
                excuse_document TEXT,
                is_excused INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                FOREIGN KEY (enrollment_id) REFERENCES enrollments(id),
                FOREIGN KEY (org_id) REFERENCES organizations(id),
                FOREIGN KEY (period_id) REFERENCES academic_periods(id)
            )"#,
        )
        .await?;

        // Copy data, mapping old `period` column to new `lesson_slot`
        db.execute_unprepared(
            r#"INSERT OR IGNORE INTO attendance__p6
               (id, org_id, period_id, enrollment_id, class_id, date, status,
                check_in_time, check_out_time, lesson_slot, subject_name,
                recorded_by, notes, excuse_type, excuse_document, is_excused, created_at)
               SELECT id, org_id, period_id, enrollment_id, class_id, date, status,
                      check_in_time, check_out_time, period, subject_name,
                      recorded_by, notes, excuse_type, excuse_document, is_excused, created_at
               FROM attendance"#,
        )
        .await?;

        db.execute_unprepared("DROP TABLE attendance").await?;
        db.execute_unprepared("ALTER TABLE attendance__p6 RENAME TO attendance").await?;

        // Recreate indexes
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_attendance_enr_date ON attendance(enrollment_id, date)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_attendance_class_date ON attendance(class_id, date)",
        )
        .await?;

        info!("P6: attendance table rebuilt with lesson_slot");
    }

    // ── 3. Rename student_assessment_snapshots → period_scores ──

    let has_old_table = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'student_assessment_snapshots'",
        ))
        .await?;

    if has_old_table.is_some() {
        info!("P6: Renaming student_assessment_snapshots → period_scores");

        db.execute_unprepared(
            "ALTER TABLE student_assessment_snapshots RENAME TO period_scores",
        )
        .await?;

        // Drop old indexes and recreate with new names
        db.execute_unprepared("DROP INDEX IF EXISTS idx_sas_student_period").await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_sas_org_period_status").await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_ps_student_period ON period_scores(student_id, period_id)",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_ps_org_period_status ON period_scores(org_id, period_id, status)",
        )
        .await?;

        info!("P6: period_scores table renamed and indexes recreated");
    }

    // ── 4. Enrollment uniqueness index: update to include org_id ──

    // Drop old index if it exists (the old one was on (student_id, period_id) only)
    let old_idx = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM sqlite_master WHERE type = 'index' AND name = 'idx_enr_unique_active'",
        ))
        .await?;

    if old_idx.is_some() {
        // Drop and recreate with the correct columns
        db.execute_unprepared("DROP INDEX IF EXISTS idx_enr_unique_active").await?;
    }

    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_enr_unique_active ON enrollments(org_id, student_id, period_id) WHERE status NOT IN ('withdrawn', 'cancelled')",
    )
    .await?;

    // ── 5. Expand persons table with identity columns ──

    let persons_columns = [
        ("org_id", "TEXT"),
        ("full_name", "TEXT"),
        ("name_ar", "TEXT"),
        ("name_en", "TEXT"),
        ("father_name", "TEXT"),
        ("grandfather_name", "TEXT"),
        ("date_of_birth", "TEXT"),
        ("place_of_birth", "TEXT"),
        ("nationality", "TEXT"),
        ("national_id", "TEXT"),
        ("passport_number", "TEXT"),
        ("address", "TEXT"),
        ("city", "TEXT"),
        ("country", "TEXT"),
        ("postal_code", "TEXT"),
        ("phone", "TEXT"),
        ("emergency_contact", "TEXT"),
        ("blood_type", "TEXT"),
        ("allergies", "TEXT"),
        ("medical_conditions", "TEXT"),
        ("medical_notes", "TEXT"),
        ("avatar", "TEXT"),
        ("bio", "TEXT"),
        ("social_links", "TEXT"),
    ];

    for (col, col_type) in &persons_columns {
        let check = format!(
            "SELECT 1 FROM pragma_table_info('persons') WHERE name = '{}'",
            col
        );
        let exists = db
            .query_one(Statement::from_string(backend, check))
            .await?;
        if exists.is_none() {
            let alter = format!("ALTER TABLE persons ADD COLUMN {} {}", col, col_type);
            db.execute_unprepared(&alter).await?;
        }
    }

    // Rename birth_date → date_of_birth if old column exists
    let has_birth_date = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM pragma_table_info('persons') WHERE name = 'birth_date'",
        ))
        .await?;
    if has_birth_date.is_some() {
        // Copy data from birth_date to date_of_birth then leave birth_date (SQLite can't drop columns)
        db.execute_unprepared("UPDATE persons SET date_of_birth = birth_date WHERE date_of_birth IS NULL AND birth_date IS NOT NULL")
            .await?;
    }

    // ── 6. Data Migration: users → persons ──

    // Check if users table still has the old demographic columns
    let has_sex_col = db
        .query_one(Statement::from_string(
            backend,
            "SELECT 1 FROM pragma_table_info('users') WHERE name = 'sex'",
        ))
        .await?;

    if has_sex_col.is_some() {
        info!("P6: Migrating demographic data from users to persons");

        // Disable foreign keys temporarily for table rebuilds
        db.execute_unprepared("PRAGMA foreign_keys = OFF").await?;

        // 6.1 Create persons for existing users (using user id as person id)
        db.execute_unprepared(
            r#"INSERT OR IGNORE INTO persons 
               (id, party_id, org_id, first_name, last_name, full_name, gender, date_of_birth,
                place_of_birth, nationality, national_id, passport_number, address, city, country,
                postal_code, phone, emergency_contact, blood_type, allergies, medical_conditions,
                medical_notes, avatar, bio, social_links, created_at, updated_at)
               SELECT 
                 u.id, 
                 COALESCE(u.party_id, (SELECT id FROM parties LIMIT 1)), -- fallback if missing
                 u.org_id, 
                 COALESCE(u.name_en, u.full_name), -- fallback first_name
                 COALESCE(u.name_ar, u.full_name), -- fallback last_name
                 u.full_name, u.sex, u.date_of_birth,
                 u.place_of_birth, u.nationality, u.national_id, u.passport_number, u.address, u.city, u.country,
                 u.postal_code, u.phone, u.emergency_contact, u.blood_type, u.allergies, u.medical_conditions,
                 u.medical_notes, u.avatar, u.bio, u.social_links, u.created_at, u.updated_at
               FROM users u"#
        ).await?;

        // 6.2 Rebuild users table (auth only)
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS users__p6 (
                id TEXT PRIMARY KEY,
                person_id TEXT NOT NULL,
                party_id TEXT,
                org_id TEXT NOT NULL,
                email TEXT,
                email_verified INTEGER NOT NULL DEFAULT 0,
                email_verified_at TEXT,
                phone TEXT,
                phone_verified INTEGER NOT NULL DEFAULT 0,
                phone_verified_at TEXT,
                password_hash TEXT,
                password_changed_at TEXT,
                password_expires_at TEXT,
                mfa_enabled INTEGER NOT NULL DEFAULT 0,
                mfa_secret TEXT,
                mfa_backup_codes TEXT,
                full_name TEXT NOT NULL,
                primary_role TEXT NOT NULL DEFAULT 'user',
                status TEXT NOT NULL DEFAULT 'active',
                last_login_at TEXT,
                last_login_ip TEXT,
                login_count INTEGER,
                failed_logins INTEGER,
                locked_until TEXT,
                prefs TEXT NOT NULL DEFAULT '{}',
                meta TEXT NOT NULL DEFAULT '{}',
                is_deleted INTEGER NOT NULL DEFAULT 0,
                deleted_at TEXT,
                deleted_by TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                created_by TEXT,
                FOREIGN KEY (person_id) REFERENCES persons(id),
                FOREIGN KEY (org_id) REFERENCES organizations(id),
                FOREIGN KEY (party_id) REFERENCES parties(id)
            )"#,
        ).await?;

        db.execute_unprepared(
            r#"INSERT OR IGNORE INTO users__p6
               (id, person_id, party_id, org_id, email, email_verified, email_verified_at,
                phone, phone_verified, phone_verified_at, password_hash, password_changed_at,
                password_expires_at, mfa_enabled, mfa_secret, mfa_backup_codes, full_name,
                primary_role, status, last_login_at, last_login_ip, login_count, failed_logins,
                locked_until, prefs, meta, is_deleted, deleted_at, deleted_by, created_at, updated_at, created_by)
               SELECT 
                id, id as person_id, party_id, org_id, email, email_verified, email_verified_at,
                phone, phone_verified, phone_verified_at, password_hash, password_changed_at,
                password_expires_at, mfa_enabled, mfa_secret, mfa_backup_codes, full_name,
                primary_role, status, last_login_at, last_login_ip, login_count, failed_logins,
                locked_until, prefs, meta, is_deleted, deleted_at, deleted_by, created_at, updated_at, created_by
               FROM users"#
        ).await?;

        db.execute_unprepared("DROP TABLE users").await?;
        db.execute_unprepared("ALTER TABLE users__p6 RENAME TO users").await?;
        info!("P6: users table rebuilt to auth-only structure");

        // 6.3 Rebuild enrollments to point FK to persons instead of users
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS enrollments__p6 (
                id TEXT PRIMARY KEY,
                org_id TEXT NOT NULL,
                period_id TEXT NOT NULL,
                student_id TEXT NOT NULL,
                class_id TEXT NOT NULL,
                student_number TEXT UNIQUE,
                status TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active','transferred','withdrawn','graduated','suspended')),
                enrollment_date TEXT NOT NULL,
                enrollment_type TEXT NOT NULL DEFAULT 'regular',
                withdrawal_date TEXT,
                withdrawal_reason TEXT,
                meta TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (period_id) REFERENCES academic_periods(id),
                FOREIGN KEY (student_id) REFERENCES persons(id),
                FOREIGN KEY (class_id) REFERENCES classes(id),
                FOREIGN KEY (org_id) REFERENCES organizations(id)
            )"#,
        ).await?;

        db.execute_unprepared("INSERT OR IGNORE INTO enrollments__p6 SELECT * FROM enrollments").await?;
        db.execute_unprepared("DROP TABLE enrollments").await?;
        db.execute_unprepared("ALTER TABLE enrollments__p6 RENAME TO enrollments").await?;

        // Recreate enrollments indexes
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_enr_org_period ON enrollments(org_id, period_id)").await?;
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_enr_student ON enrollments(student_id)").await?;
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_enr_class ON enrollments(class_id)").await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_enr_unique_active ON enrollments(org_id, student_id, period_id) WHERE status NOT IN ('withdrawn', 'cancelled')",
        ).await?;
        db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_enrollment_org_period_class ON enrollments(org_id, period_id, class_id)").await?;

        // Turn foreign keys back on
        db.execute_unprepared("PRAGMA foreign_keys = ON").await?;
    }

    info!("P6 migration complete: schema corrections applied");
    Ok(())
}

/// P7: Bind assessment schemes to offerings, unique computed results, overall pass/fail.
async fn migrate_p7_grading_bindings(db: &DatabaseConnection) -> Result<(), DbErr> {
    let backend = db.get_database_backend();
    info!("Applying P7 migration: grading bindings and unique computed results");

    async fn has_col(db: &DatabaseConnection, backend: sea_orm::DatabaseBackend, table: &str, col: &str) -> Result<bool, DbErr> {
        let sql = if backend == sea_orm::DatabaseBackend::Postgres {
            format!(
                "SELECT 1 FROM information_schema.columns WHERE table_name = '{table}' AND column_name = '{col}' LIMIT 1"
            )
        } else {
            format!("SELECT 1 FROM pragma_table_info('{table}') WHERE name = '{col}'")
        };
        Ok(db.query_one(Statement::from_string(backend, sql)).await?.is_some())
    }

    if !has_col(db, backend, "class_subjects", "scheme_id").await? {
        db.execute_unprepared("ALTER TABLE class_subjects ADD COLUMN scheme_id TEXT").await?;
    }
    if !has_col(db, backend, "grades", "component_id").await? {
        db.execute_unprepared("ALTER TABLE grades ADD COLUMN component_id TEXT").await?;
    }
    if !has_col(db, backend, "period_scores", "enrollment_id").await? {
        db.execute_unprepared("ALTER TABLE period_scores ADD COLUMN enrollment_id TEXT").await?;
    }

    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS ranking_policies (
            id TEXT PRIMARY KEY,
            org_id TEXT NOT NULL,
            period_id TEXT,
            name TEXT NOT NULL,
            is_enabled INTEGER NOT NULL DEFAULT 1,
            ranking_scope TEXT NOT NULL DEFAULT 'class',
            ranking_method TEXT NOT NULL DEFAULT 'standard',
            tie_handling TEXT NOT NULL DEFAULT 'same_rank',
            include_failed_students INTEGER NOT NULL DEFAULT 0,
            ranking_metric TEXT NOT NULL DEFAULT 'total_score',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"#,
    )
    .await?;

    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS term_results (
            id TEXT PRIMARY KEY,
            org_id TEXT NOT NULL,
            student_id TEXT NOT NULL,
            term_period_id TEXT NOT NULL,
            enrollment_id TEXT NOT NULL,
            class_subject_id TEXT NOT NULL,
            coursework_score REAL NOT NULL DEFAULT 0,
            coursework_max REAL NOT NULL DEFAULT 0,
            exam_score REAL NOT NULL DEFAULT 0,
            exam_max REAL NOT NULL DEFAULT 0,
            total_score REAL NOT NULL DEFAULT 0,
            total_max REAL NOT NULL DEFAULT 0,
            percentage REAL NOT NULL DEFAULT 0,
            grade_label TEXT,
            gpa_points REAL,
            is_passing INTEGER NOT NULL DEFAULT 0,
            rank INTEGER,
            rank_scope TEXT,
            calculation_policy_id TEXT,
            calculation_version INTEGER NOT NULL DEFAULT 1,
            policy_snapshot TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            finalized_at TEXT,
            finalized_by TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"#,
    )
    .await?;

    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS annual_results (
            id TEXT PRIMARY KEY,
            org_id TEXT NOT NULL,
            student_id TEXT NOT NULL,
            year_period_id TEXT NOT NULL,
            enrollment_id TEXT NOT NULL,
            class_subject_id TEXT NOT NULL,
            term_scores_json TEXT NOT NULL DEFAULT '[]',
            annual_total REAL NOT NULL DEFAULT 0,
            annual_max REAL NOT NULL DEFAULT 0,
            percentage REAL NOT NULL DEFAULT 0,
            grade_label TEXT,
            gpa_points REAL,
            is_passing INTEGER NOT NULL DEFAULT 0,
            rank INTEGER,
            rank_scope TEXT,
            calculation_policy_id TEXT,
            calculation_version INTEGER NOT NULL DEFAULT 1,
            policy_snapshot TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            finalized_at TEXT,
            finalized_by TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"#,
    )
    .await?;

    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS period_overall_results (
            id TEXT PRIMARY KEY,
            org_id TEXT NOT NULL,
            student_id TEXT NOT NULL,
            enrollment_id TEXT NOT NULL,
            period_id TEXT NOT NULL,
            passed INTEGER NOT NULL DEFAULT 0,
            failed_subject_count INTEGER NOT NULL DEFAULT 0,
            policy_id TEXT,
            decision TEXT NOT NULL DEFAULT 'failed',
            percentage REAL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"#,
    )
    .await?;

    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_ps_unique_offering ON period_scores(org_id, student_id, period_id, class_subject_id)",
    )
    .await?;
    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_tr_unique_offering ON term_results(enrollment_id, term_period_id, class_subject_id)",
    )
    .await?;
    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_ar_unique_offering ON annual_results(enrollment_id, year_period_id, class_subject_id)",
    )
    .await?;
    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_por_unique_student_period ON period_overall_results(org_id, student_id, period_id)",
    )
    .await?;
    db.execute_unprepared(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_grades_unique_component ON grades(enrollment_id, class_subject_id, period_id, component_id) WHERE component_id IS NOT NULL AND class_subject_id IS NOT NULL",
    )
    .await?;
    db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_cs_scheme ON class_subjects(scheme_id)").await?;
    db.execute_unprepared("CREATE INDEX IF NOT EXISTS idx_grades_component ON grades(component_id)").await?;

    info!("P7 migration complete: grading bindings applied");
    Ok(())
}

/// P8: Report definitions and template bindings tables.
async fn migrate_p8_report_definitions(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Check if already applied
    let row = db
        .query_one(Statement::from_string(
            db.get_database_backend(),
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'report_definitions'",
        ))
        .await?;

    if row.is_some() {
        return Ok(());
    }

    info!("Applying P8 migration: report definitions and template bindings");

    // Report Definitions — metadata for every available report
    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS report_definitions (
            code                  TEXT PRIMARY KEY,
            org_id                TEXT,
            domain                TEXT NOT NULL,
            category              TEXT NOT NULL,
            name                  TEXT NOT NULL,
            name_ar               TEXT NOT NULL,
            description           TEXT,
            description_ar        TEXT,
            parameters_schema     TEXT NOT NULL DEFAULT '[]',
            required_permissions  TEXT DEFAULT '[]',
            supported_formats     TEXT DEFAULT '["data_only"]',
            default_template_id   TEXT,
            is_system             INTEGER DEFAULT 1,
            is_active             INTEGER DEFAULT 1,
            sort_order            INTEGER DEFAULT 0,
            version               INTEGER DEFAULT 1,
            created_at            TEXT NOT NULL,
            updated_at            TEXT NOT NULL
        )"#,
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_rd_domain ON report_definitions(domain, category)",
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_rd_org ON report_definitions(org_id) WHERE org_id IS NOT NULL",
    )
    .await?;

    // Report Template Bindings — links report definitions to DocForge templates
    db.execute_unprepared(
        r#"CREATE TABLE IF NOT EXISTS report_template_bindings (
            id             TEXT PRIMARY KEY,
            report_code    TEXT NOT NULL REFERENCES report_definitions(code) ON DELETE CASCADE,
            template_id    TEXT REFERENCES templates(id),
            locale         TEXT DEFAULT 'ar',
            format         TEXT DEFAULT 'pdf',
            is_default     INTEGER DEFAULT 1,
            created_at     TEXT NOT NULL
        )"#,
    )
    .await?;

    db.execute_unprepared(
        "CREATE INDEX IF NOT EXISTS idx_rtb_report ON report_template_bindings(report_code, locale, format)",
    )
    .await?;

    info!("P8 migration complete: report definitions and template bindings");
    Ok(())
}
