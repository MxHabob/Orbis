// ═══════════════════════════════════════════════════════════════
// docforge-orbis — Database Schema Migration
// Creates all 10 template management tables in SQLite.
// ═══════════════════════════════════════════════════════════════

use sea_orm::{ConnectionTrait, DbErr};

/// Run the template management schema migration.
/// Creates all tables if they don't exist. Safe to run multiple times.
pub async fn migrate_template_schema(db: &impl ConnectionTrait) -> Result<(), DbErr> {
    // Execute all CREATE TABLE statements
    for statement in MIGRATION_STATEMENTS {
        db.execute_unprepared(statement).await?;
    }

    tracing::info!("docforge-orbis: Template schema migration completed");
    Ok(())
}

const MIGRATION_STATEMENTS: &[&str] = &[
    // ── Template Categories ─────────────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_categories (
        id           TEXT PRIMARY KEY,
        org_id       TEXT NOT NULL,
        name         TEXT NOT NULL,
        name_ar      TEXT,
        slug         TEXT NOT NULL,
        description  TEXT,
        icon         TEXT,
        sort_order   INTEGER DEFAULT 0,
        parent_id    TEXT REFERENCES template_categories(id),
        is_system    INTEGER DEFAULT 0,
        is_active    INTEGER DEFAULT 1,
        created_at   TEXT NOT NULL,
        updated_at   TEXT NOT NULL
    )"#,
    // ── Templates (Master Record) ───────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS templates (
        id                 TEXT PRIMARY KEY,
        org_id             TEXT NOT NULL,
        category_id        TEXT REFERENCES template_categories(id),
        code               TEXT,
        name               TEXT NOT NULL,
        name_ar            TEXT,
        description        TEXT,
        description_ar     TEXT,
        document_type      TEXT NOT NULL,
        entity_type        TEXT NOT NULL,
        page_size          TEXT DEFAULT 'A4',
        page_width_mm      REAL,
        page_height_mm     REAL,
        orientation        TEXT DEFAULT 'portrait',
        margins_json       TEXT DEFAULT '{"top":10,"bottom":10,"left":10,"right":10}',
        document_json      TEXT NOT NULL DEFAULT '{}',
        status             TEXT DEFAULT 'draft',
        version            INTEGER DEFAULT 1,
        published_version  INTEGER,
        published_at       TEXT,
        published_by       TEXT,
        created_by         TEXT NOT NULL,
        is_system          INTEGER DEFAULT 0,
        is_default         INTEGER DEFAULT 0,
        is_locked          INTEGER DEFAULT 0,
        tags               TEXT,
        locale             TEXT DEFAULT 'ar',
        thumbnail_url      TEXT,
        meta               TEXT,
        deleted_at         TEXT,
        created_at         TEXT NOT NULL,
        updated_at         TEXT NOT NULL
    )"#,
    r#"CREATE INDEX IF NOT EXISTS idx_templates_org_status ON templates(org_id, status, document_type)"#,
    r#"CREATE UNIQUE INDEX IF NOT EXISTS idx_templates_org_code ON templates(org_id, code) WHERE code IS NOT NULL"#,
    // ── Template Versions (Immutable Snapshots) ──────────────────
    r#"CREATE TABLE IF NOT EXISTS template_versions (
        id              TEXT PRIMARY KEY,
        template_id     TEXT NOT NULL REFERENCES templates(id) ON DELETE CASCADE,
        version         INTEGER NOT NULL,
        document_json   TEXT NOT NULL,
        page_size       TEXT,
        page_width_mm   REAL,
        page_height_mm  REAL,
        orientation     TEXT,
        margins_json    TEXT,
        change_summary  TEXT,
        created_by      TEXT NOT NULL,
        created_at      TEXT NOT NULL,
        UNIQUE(template_id, version)
    )"#,
    // ── Template Components (Reusable Blocks) ────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_components (
        id              TEXT PRIMARY KEY,
        org_id          TEXT NOT NULL,
        name            TEXT NOT NULL,
        name_ar         TEXT,
        description     TEXT,
        component_type  TEXT NOT NULL,
        document_json   TEXT NOT NULL,
        thumbnail_url   TEXT,
        is_system       INTEGER DEFAULT 0,
        is_shared       INTEGER DEFAULT 1,
        tags            TEXT,
        created_by      TEXT NOT NULL,
        created_at      TEXT NOT NULL,
        updated_at      TEXT NOT NULL
    )"#,
    // ── Template Assets ──────────────────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_assets (
        id               TEXT PRIMARY KEY,
        org_id           TEXT NOT NULL,
        name             TEXT NOT NULL,
        asset_type       TEXT NOT NULL,
        mime_type        TEXT NOT NULL,
        file_size        INTEGER,
        width            INTEGER,
        height           INTEGER,
        storage_key      TEXT NOT NULL,
        storage_provider TEXT DEFAULT 'local',
        url              TEXT,
        thumbnail_url    TEXT,
        font_family      TEXT,
        font_weight      TEXT,
        font_style       TEXT,
        tags             TEXT,
        is_system        INTEGER DEFAULT 0,
        created_by       TEXT NOT NULL,
        created_at       TEXT NOT NULL,
        updated_at       TEXT NOT NULL
    )"#,
    r#"CREATE INDEX IF NOT EXISTS idx_assets_org_type ON template_assets(org_id, asset_type)"#,
    // ── Template ↔ Asset Usage ───────────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_asset_usages (
        id           TEXT PRIMARY KEY,
        template_id  TEXT NOT NULL REFERENCES templates(id) ON DELETE CASCADE,
        asset_id     TEXT NOT NULL REFERENCES template_assets(id),
        usage_context TEXT,
        created_at   TEXT NOT NULL
    )"#,
    // ── Template ↔ Component Usage ───────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_component_usages (
        id              TEXT PRIMARY KEY,
        template_id     TEXT NOT NULL REFERENCES templates(id) ON DELETE CASCADE,
        component_id    TEXT NOT NULL REFERENCES template_components(id),
        node_id         TEXT NOT NULL,
        overrides_json  TEXT,
        created_at      TEXT NOT NULL
    )"#,
    // ── Data Source Definitions ──────────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_data_sources (
        id               TEXT PRIMARY KEY,
        org_id           TEXT NOT NULL,
        name             TEXT NOT NULL,
        name_ar          TEXT,
        entity_type      TEXT NOT NULL,
        description      TEXT,
        schema_json      TEXT NOT NULL,
        sample_data_json TEXT,
        is_system        INTEGER DEFAULT 1,
        created_at       TEXT NOT NULL,
        updated_at       TEXT NOT NULL
    )"#,
    // ── Generation Jobs ──────────────────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_generation_jobs (
        id                TEXT PRIMARY KEY,
        org_id            TEXT NOT NULL,
        template_id       TEXT NOT NULL REFERENCES templates(id),
        template_version  INTEGER NOT NULL,
        job_type          TEXT NOT NULL,
        output_format     TEXT DEFAULT 'pdf',
        data_source_id    TEXT,
        filter_json       TEXT,
        record_ids_json   TEXT,
        total_records     INTEGER DEFAULT 0,
        status            TEXT DEFAULT 'pending',
        progress          INTEGER DEFAULT 0,
        processed_count   INTEGER DEFAULT 0,
        succeeded_count   INTEGER DEFAULT 0,
        failed_count      INTEGER DEFAULT 0,
        retry_count       INTEGER DEFAULT 0,
        max_retries       INTEGER DEFAULT 3,
        last_checkpoint   TEXT,
        checkpoint_data   TEXT,
        output_key        TEXT,
        output_url        TEXT,
        output_size       INTEGER,
        error_log         TEXT,
        started_at        TEXT,
        completed_at      TEXT,
        cancelled_at      TEXT,
        cancelled_by      TEXT,
        estimated_eta     TEXT,
        priority          INTEGER DEFAULT 5,
        created_by        TEXT NOT NULL,
        created_at        TEXT NOT NULL
    )"#,
    r#"CREATE INDEX IF NOT EXISTS idx_jobs_org_status ON template_generation_jobs(org_id, status)"#,
    r#"CREATE INDEX IF NOT EXISTS idx_jobs_active ON template_generation_jobs(status) WHERE status IN ('pending', 'processing', 'paused')"#,
    // ── Generation Job Items ─────────────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_generation_items (
        id              TEXT PRIMARY KEY,
        job_id          TEXT NOT NULL REFERENCES template_generation_jobs(id) ON DELETE CASCADE,
        record_id       TEXT NOT NULL,
        sequence        INTEGER NOT NULL,
        status          TEXT DEFAULT 'pending',
        retry_count     INTEGER DEFAULT 0,
        error_message   TEXT,
        output_key      TEXT,
        rendered_at     TEXT,
        duration_ms     INTEGER
    )"#,
    r#"CREATE INDEX IF NOT EXISTS idx_items_job_status ON template_generation_items(job_id, status)"#,
    r#"CREATE INDEX IF NOT EXISTS idx_items_job_seq ON template_generation_items(job_id, sequence)"#,
    // ── Template Audit Trail ─────────────────────────────────────
    r#"CREATE TABLE IF NOT EXISTS template_audit_log (
        id           TEXT PRIMARY KEY,
        org_id       TEXT NOT NULL,
        template_id  TEXT,
        user_id      TEXT NOT NULL,
        action       TEXT NOT NULL,
        entity_type  TEXT NOT NULL,
        entity_id    TEXT NOT NULL,
        old_data     TEXT,
        new_data     TEXT,
        metadata     TEXT,
        created_at   TEXT NOT NULL
    )"#,
    r#"CREATE INDEX IF NOT EXISTS idx_tpl_audit_org ON template_audit_log(org_id, created_at)"#,
    r#"CREATE INDEX IF NOT EXISTS idx_tpl_audit_template ON template_audit_log(template_id, created_at)"#,
];
