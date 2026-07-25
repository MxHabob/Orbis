//! Idempotent schema migrations applied after baseline on existing databases.

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};
use tracing::info;

/// Apply P0 migrations (safe to call on every startup).
pub async fn run_p0_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    migrate_academic_policies_branch_overrides(db).await?;
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
