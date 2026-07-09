-- ═══════════════════════════════════════════════════════════════
-- Orbis v2.0 — Cleanup Migration (Destructive)
--
-- ⚠️  WARNING: This migration DROPS tables and columns.
--     Run AFTER verifying data has been migrated.
--     Use infra_persistence::migrations::run_sql_cleanup()
--
-- Plan Section 3.1 — حذف الجداول المكرّرة
-- ═══════════════════════════════════════════════════════════════

-- ── Drop duplicate attendance table ─────────────────────────
-- attendance_daily is the authoritative source.
-- attendance (per-period) is redundant.
DROP TABLE IF EXISTS attendance;

-- ── Drop computed attendance_summary ────────────────────────
-- Summary is now computed on-the-fly from attendance_daily.
DROP TABLE IF EXISTS attendance_summary;

-- ── Drop duplicate system_log ───────────────────────────────
-- All logging now goes through audit_log with actor_type column.
DROP TABLE IF EXISTS system_log;

-- ── Drop registry_audit_log ─────────────────────────────────
-- Merged into main audit_log with source='registry'.
DROP TABLE IF EXISTS registry_audit_log;

-- ── Drop redundant permissions table ────────────────────────
-- Unified into role_permissions table.
DROP TABLE IF EXISTS permissions;

-- ── Clean up execution columns from academic_calendar_events ──
-- Removed execution mode, actions, and execution tracking columns.
-- Ignore errors in cleanup in case these columns don't exist anymore.
ALTER TABLE academic_calendar_events DROP COLUMN execution_mode;
ALTER TABLE academic_calendar_events DROP COLUMN auto_action;
ALTER TABLE academic_calendar_events DROP COLUMN prerequisites;
ALTER TABLE academic_calendar_events DROP COLUMN executed_at;
ALTER TABLE academic_calendar_events DROP COLUMN executed_by;
ALTER TABLE academic_calendar_events DROP COLUMN execution_result;
ALTER TABLE academic_calendar_events DROP COLUMN execution_notes;
