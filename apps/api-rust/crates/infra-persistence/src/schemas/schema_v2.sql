-- ═══════════════════════════════════════════════════════════════
-- Orbis v2.0 — Schema Migration
-- Enterprise Architecture Rebuild — Phase 5
--
-- This migration:
--   1. Adds 7 new tables (autopilot + AI + data quality)
--   2. Modifies audit_log (actor tracking)
--   3. Drops duplicate/obsolete tables
--   4. Creates performance indexes
--
-- Run via: infra_persistence::migrations::run_sql_migration()
-- ═══════════════════════════════════════════════════════════════

-- ── 1. autopilot_config ─────────────────────────────────────
-- REMOVED: Merged into `academic_policies` in schema_baseline.sql.
-- Autopilot mode, toggles, and grace days are now part of the
-- unified academic_policies table (one row per org).


-- ── 2. autopilot_tasks ──────────────────────────────────────
CREATE TABLE IF NOT EXISTS autopilot_tasks (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  calendar_event_id TEXT,
  task_type TEXT NOT NULL,
  scheduled_at TEXT NOT NULL,
  deadline_at TEXT,
  target_entity_type TEXT,
  target_entity_id TEXT,
  params TEXT,
  prerequisites TEXT,
  status TEXT NOT NULL DEFAULT 'pending'
    CHECK (status IN ('pending','ready','executing','completed',
                      'failed','skipped','suggested','approved','cancelled')),
  execution_mode TEXT NOT NULL,
  attempts INTEGER NOT NULL DEFAULT 0,
  last_attempt_at TEXT,
  last_error TEXT,
  suggested_at TEXT,
  approved_by TEXT,
  approved_at TEXT,
  rejection_reason TEXT,
  completed_at TEXT,
  result TEXT,
  idempotency_key TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_at_org_status ON autopilot_tasks(org_id, status);
CREATE INDEX IF NOT EXISTS idx_at_scheduled ON autopilot_tasks(scheduled_at)
  WHERE status IN ('pending','ready');

-- ── 3. autopilot_execution_log ──────────────────────────────
CREATE TABLE IF NOT EXISTS autopilot_execution_log (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  task_id TEXT,
  action TEXT NOT NULL,
  status TEXT NOT NULL,
  details TEXT,
  error TEXT,
  duration_ms INTEGER,
  executed_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_ael_org_date ON autopilot_execution_log(org_id, executed_at);

-- ── 4. ai_agent_sessions ────────────────────────────────────
CREATE TABLE IF NOT EXISTS ai_agent_sessions (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  agent_type TEXT NOT NULL
    CHECK (agent_type IN ('assistant','autopilot','analyst','guardian','external')),
  agent_version TEXT,
  acting_as_user_id TEXT NOT NULL,
  acting_as_role TEXT NOT NULL,
  conversation_id TEXT,
  status TEXT NOT NULL DEFAULT 'active'
    CHECK (status IN ('active','completed','suspended','revoked')),
  max_risk_level TEXT NOT NULL DEFAULT 'low',
  requires_approval_above TEXT NOT NULL DEFAULT 'medium',
  actions_executed INTEGER NOT NULL DEFAULT 0,
  actions_suggested INTEGER NOT NULL DEFAULT 0,
  started_at TEXT NOT NULL,
  last_active_at TEXT NOT NULL,
  ended_at TEXT,
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_aas_org_status ON ai_agent_sessions(org_id, status);

-- ── 5. ai_action_log ────────────────────────────────────────
CREATE TABLE IF NOT EXISTS ai_action_log (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  session_id TEXT NOT NULL,
  correlation_id TEXT,
  action_code TEXT NOT NULL,
  input_data TEXT,
  output_data TEXT,
  reasoning TEXT NOT NULL,
  confidence REAL,
  status TEXT NOT NULL
    CHECK (status IN ('pending','approved','executed','rejected','failed','rolled_back')),
  risk_level TEXT NOT NULL,
  approval_required INTEGER NOT NULL DEFAULT 0,
  approved_by TEXT,
  approved_at TEXT,
  rejection_reason TEXT,
  affected_entities TEXT,
  idempotency_key TEXT UNIQUE,
  duration_ms INTEGER,
  error TEXT,
  is_reversible INTEGER NOT NULL DEFAULT 0,
  rollback_data TEXT,
  reversed_at TEXT,
  reversed_by TEXT,
  executed_at TEXT NOT NULL,
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_aal_session ON ai_action_log(session_id);
CREATE INDEX IF NOT EXISTS idx_aal_org_date ON ai_action_log(org_id, executed_at);
CREATE INDEX IF NOT EXISTS idx_aal_pending ON ai_action_log(status)
  WHERE status = 'pending';

-- ── 6. ai_insights ──────────────────────────────────────────
CREATE TABLE IF NOT EXISTS ai_insights (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  insight_type TEXT NOT NULL,
  entity_type TEXT,
  entity_id TEXT,
  scope TEXT NOT NULL DEFAULT 'organization',
  title TEXT NOT NULL,
  title_ar TEXT,
  summary TEXT NOT NULL,
  summary_ar TEXT,
  details TEXT,
  severity TEXT NOT NULL DEFAULT 'info'
    CHECK (severity IN ('info','suggestion','warning','alert','critical')),
  confidence REAL NOT NULL,
  impact_score REAL,
  suggested_action TEXT,
  suggested_params TEXT,
  status TEXT NOT NULL DEFAULT 'new'
    CHECK (status IN ('new','seen','acknowledged','acted_upon','dismissed','expired')),
  acknowledged_by TEXT,
  acknowledged_at TEXT,
  generated_by TEXT NOT NULL,
  valid_from TEXT NOT NULL,
  valid_until TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_ai_org_status ON ai_insights(org_id, status);
CREATE INDEX IF NOT EXISTS idx_ai_entity ON ai_insights(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_ai_severity ON ai_insights(severity)
  WHERE status = 'new';

-- ── 7. data_quality_findings ────────────────────────────────
CREATE TABLE IF NOT EXISTS data_quality_findings (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  rule_code TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  field_name TEXT,
  severity TEXT NOT NULL DEFAULT 'warning',
  current_value TEXT,
  expected_description TEXT,
  suggested_fix TEXT,
  status TEXT NOT NULL DEFAULT 'open'
    CHECK (status IN ('open','auto_fixed','manually_fixed','ignored','false_positive')),
  fixed_by TEXT,
  fixed_at TEXT,
  detected_at TEXT NOT NULL,
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_dqf_org_status ON data_quality_findings(org_id, status);
CREATE INDEX IF NOT EXISTS idx_dqf_entity ON data_quality_findings(entity_type, entity_id);

-- ═══════════════════════════════════════════════════════════════
-- Schema Modifications — Existing Tables
-- ═══════════════════════════════════════════════════════════════

-- ── audit_log — Add AI actor tracking columns ───────────────
-- These columns allow distinguishing human vs AI vs autopilot actions
-- and correlating related operations.
-- Using ALTER TABLE ADD COLUMN (SQLite ignores if column exists via IF NOT EXISTS workaround)

ALTER TABLE audit_log ADD COLUMN actor_type TEXT DEFAULT 'user';
ALTER TABLE audit_log ADD COLUMN actor_session_id TEXT;
ALTER TABLE audit_log ADD COLUMN correlation_id TEXT;
