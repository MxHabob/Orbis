-- Orbis Registry Schema — Central database for multi-year management
-- This schema lives in orbis_registry.db (separate from year databases).
PRAGMA foreign_keys=ON;

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Academic Years ──
-- Each row maps to a separate SQLite database file for that academic year.
-- ══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS academic_years (
    id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL,
    name TEXT NOT NULL,                           -- "2024-2025"
    name_ar TEXT,                                  -- "العام الدراسي 2024-2025"
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    hijri_start_date TEXT,
    hijri_end_date TEXT,
    db_filename TEXT NOT NULL UNIQUE,              -- "2024-2025_active.db"
    status TEXT NOT NULL DEFAULT 'active',
        -- active     : current working year (only one allowed)
        -- promoting  : promotion in progress (locked)
        -- archived   : read-only, promotion completed
    is_current INTEGER NOT NULL DEFAULT 0,
    archived_at TEXT,
    archived_by TEXT,
    promotion_source_year_id TEXT,                 -- which year was promoted into this one
    db_size_bytes INTEGER,
    student_count INTEGER,
    teacher_count INTEGER,
    class_count INTEGER,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ay_org ON academic_years(org_id);
CREATE INDEX IF NOT EXISTS idx_ay_status ON academic_years(org_id, status);
CREATE UNIQUE INDEX IF NOT EXISTS idx_ay_current ON academic_years(org_id, is_current) WHERE is_current = 1;

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Promotion Jobs ──
-- Each promotion operation is tracked as a job with full lifecycle.
-- Status flow: pending → validating → dry_run → confirmed → executing → completed | failed | rolled_back
-- ══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS promotion_jobs (
    id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL,
    source_year_id TEXT NOT NULL,
    target_year_id TEXT,                           -- NULL until execution creates the year
    status TEXT NOT NULL DEFAULT 'pending',
    initiated_by TEXT NOT NULL,
    initiated_by_name TEXT,

    -- Configuration (serialized JSON)
    config TEXT NOT NULL,
    -- JSON shape: {
    --   passing_score: i32,
    --   min_attendance_rate: f64,
    --   auto_graduate_final_level: bool,
    --   final_level: string?,
    --   exclude_withdrawn: bool,
    --   carry_over_outstanding_fees: bool,
    --   carry_over_guardians: bool,
    --   target_year_name: string,
    --   target_start_date: string,
    --   target_end_date: string,
    --   class_mapping: ClassMapping[],
    -- }

    -- Dry run results (populated after dry_run phase)
    dry_run_result TEXT,                           -- JSON: full preview
    dry_run_at TEXT,

    -- Execution statistics
    total_students INTEGER,
    promoted_count INTEGER,
    retained_count INTEGER,
    graduated_count INTEGER,
    withdrawn_count INTEGER,
    excluded_count INTEGER,

    -- Safety & recovery
    backup_path TEXT,                              -- path to pre-promotion backup file
    error_log TEXT,
    rollback_available INTEGER NOT NULL DEFAULT 1,

    -- Timestamps
    started_at TEXT,
    completed_at TEXT,
    rolled_back_at TEXT,
    rolled_back_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    FOREIGN KEY (source_year_id) REFERENCES academic_years(id),
    FOREIGN KEY (target_year_id) REFERENCES academic_years(id)
);

CREATE INDEX IF NOT EXISTS idx_pj_org ON promotion_jobs(org_id);
CREATE INDEX IF NOT EXISTS idx_pj_source ON promotion_jobs(source_year_id);
CREATE INDEX IF NOT EXISTS idx_pj_status ON promotion_jobs(status);

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Promotion Details ──
-- Per-student promotion decision and outcome.
-- ══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS promotion_details (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    student_name TEXT NOT NULL,
    student_name_ar TEXT,
    student_number TEXT,

    -- Source (current year)
    source_enrollment_id TEXT NOT NULL,
    source_class_id TEXT NOT NULL,
    source_class_name TEXT,
    source_level TEXT NOT NULL,
    source_division TEXT,

    -- Target (next year) — populated after execution
    target_enrollment_id TEXT,
    target_class_id TEXT,
    target_class_name TEXT,
    target_level TEXT,

    -- Decision
    decision TEXT NOT NULL,
        -- promoted   : moves to next level
        -- retained   : stays in same level
        -- graduated  : completed final level
        -- excluded   : withdrawn/inactive, not processed
    reason TEXT NOT NULL,
    final_score INTEGER,
    attendance_rate REAL,

    -- Manual override
    manual_override INTEGER NOT NULL DEFAULT 0,
    override_by TEXT,
    override_reason TEXT,
    original_decision TEXT,                         -- decision before override

    created_at TEXT NOT NULL,

    FOREIGN KEY (job_id) REFERENCES promotion_jobs(id)
);

CREATE INDEX IF NOT EXISTS idx_pd_job ON promotion_details(job_id);
CREATE INDEX IF NOT EXISTS idx_pd_student ON promotion_details(student_id);
CREATE INDEX IF NOT EXISTS idx_pd_decision ON promotion_details(job_id, decision);

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Student Registry ──
-- Cross-year student identity. Tracks a student across all academic years.
-- The authoritative student data lives in each year's DB - this is a summary.
-- ══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS student_registry (
    id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    student_number TEXT NOT NULL,
    full_name TEXT NOT NULL,
    name_ar TEXT,
    national_id TEXT,
    status TEXT NOT NULL DEFAULT 'active',
        -- active         : currently enrolled
        -- graduated      : completed all levels
        -- withdrawn      : left the school
        -- transferred_out: moved to another school
    first_enrollment_year_id TEXT,
    current_year_id TEXT,
    current_level TEXT,
    current_class_name TEXT,
    years_enrolled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sr_org ON student_registry(org_id);
CREATE INDEX IF NOT EXISTS idx_sr_user ON student_registry(user_id);
CREATE INDEX IF NOT EXISTS idx_sr_number ON student_registry(student_number);
CREATE INDEX IF NOT EXISTS idx_sr_national ON student_registry(national_id);
CREATE INDEX IF NOT EXISTS idx_sr_status ON student_registry(org_id, status);

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Registry Audit Log ──
-- Tracks all year management and promotion actions.
-- ══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS registry_audit_log (
    id TEXT PRIMARY KEY,
    org_id TEXT NOT NULL,
    user_id TEXT,
    action TEXT NOT NULL,
        -- year.created, year.archived, year.switched,
        -- promotion.started, promotion.dry_run, promotion.executed,
        -- promotion.rolled_back, promotion.override
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    details TEXT,                                   -- JSON with action-specific data
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ral_org ON registry_audit_log(org_id);
CREATE INDEX IF NOT EXISTS idx_ral_action ON registry_audit_log(action);
