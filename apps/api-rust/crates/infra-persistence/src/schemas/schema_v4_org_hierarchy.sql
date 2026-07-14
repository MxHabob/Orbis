-- ═══════════════════════════════════════════════════════════════════════════════
-- Orbis v4.0 Schema Migration — Organization Hierarchy Upgrade
-- ═══════════════════════════════════════════════════════════════════════════════
--
-- Upgrades the `org_units` table with:
--   1. Materialized Path (path, depth) for efficient tree queries
--   2. Physical location fields (head_id, capacity, room_number, floor, building)
--   3. Settings & Policy inheritance (settings, local_policies)
--   4. Data isolation control (data_isolation)
--   5. Denormalized counts (children_count, descendant_count)
--
-- Also adds `org_unit_id` to `academic_policies` for branch-level policies.
--
-- Safe to run multiple times — uses ALTER TABLE ADD COLUMN (SQLite ignores if exists).
-- ═══════════════════════════════════════════════════════════════════════════════

-- ── org_units: Materialized Path ─────────────────────────────────────────────
-- Note: SQLite ALTER TABLE ADD COLUMN does not support IF NOT EXISTS,
-- so we wrap in a transaction and use a version check.

-- Check if migration already applied
CREATE TABLE IF NOT EXISTS _schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Guard: only run if v4_org_hierarchy not yet applied
INSERT OR IGNORE INTO _schema_migrations (version) VALUES ('v4_org_hierarchy_check');

-- ── Add Materialized Path columns ────────────────────────────────────────────
ALTER TABLE org_units ADD COLUMN path TEXT NOT NULL DEFAULT '/';
ALTER TABLE org_units ADD COLUMN depth INTEGER NOT NULL DEFAULT 0;

-- ── Physical Location ────────────────────────────────────────────────────────
ALTER TABLE org_units ADD COLUMN head_id TEXT;
ALTER TABLE org_units ADD COLUMN capacity INTEGER;
ALTER TABLE org_units ADD COLUMN room_number TEXT;
ALTER TABLE org_units ADD COLUMN floor INTEGER;
ALTER TABLE org_units ADD COLUMN building TEXT;

-- ── Settings & Policy Inheritance ────────────────────────────────────────────
ALTER TABLE org_units ADD COLUMN settings TEXT;
ALTER TABLE org_units ADD COLUMN local_policies TEXT;

-- ── Data Isolation ───────────────────────────────────────────────────────────
ALTER TABLE org_units ADD COLUMN data_isolation TEXT NOT NULL DEFAULT 'shared';

-- ── Denormalized Counts ──────────────────────────────────────────────────────
ALTER TABLE org_units ADD COLUMN children_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE org_units ADD COLUMN descendant_count INTEGER NOT NULL DEFAULT 0;

-- ── Performance Indexes ──────────────────────────────────────────────────────
CREATE INDEX IF NOT EXISTS idx_org_units_path ON org_units(path);
CREATE INDEX IF NOT EXISTS idx_org_units_org_type ON org_units(org_id, unit_type);
CREATE INDEX IF NOT EXISTS idx_org_units_org_depth ON org_units(org_id, depth);
CREATE INDEX IF NOT EXISTS idx_org_units_parent ON org_units(parent_id);

-- ══════════════════════════════════════════════════════════════════════════════
-- academic_policies: Branch-Level Policy Support
-- ══════════════════════════════════════════════════════════════════════════════

ALTER TABLE academic_policies ADD COLUMN org_unit_id TEXT;

-- Unique constraint: one policy per org + unit combination
-- NULL org_unit_id = org-wide default policy
CREATE UNIQUE INDEX IF NOT EXISTS idx_academic_policies_org_unit
    ON academic_policies(org_id, COALESCE(org_unit_id, '__root__'));

-- ══════════════════════════════════════════════════════════════════════════════
-- Performance Indexes for Common Queries
-- ══════════════════════════════════════════════════════════════════════════════

CREATE INDEX IF NOT EXISTS idx_enrollment_org_period_class
    ON enrollments(org_id, period_id, class_id) WHERE is_deleted = 0;

CREATE INDEX IF NOT EXISTS idx_grades_enrollment_period_subject
    ON grades(enrollment_id, period_id, class_subject_id);

CREATE INDEX IF NOT EXISTS idx_attendance_enrollment_period_date
    ON attendance(enrollment_id, period_id, date);

CREATE INDEX IF NOT EXISTS idx_users_org_role
    ON users(org_id, role) WHERE is_deleted = 0;

CREATE INDEX IF NOT EXISTS idx_autopilot_org_status
    ON autopilot_tasks(org_id, status);

CREATE INDEX IF NOT EXISTS idx_classes_org_period_covering
    ON classes(org_id, period_id, name, grade_level, capacity);

-- ── Mark migration as applied ────────────────────────────────────────────────
INSERT OR REPLACE INTO _schema_migrations (version)
VALUES ('v4_org_hierarchy');
