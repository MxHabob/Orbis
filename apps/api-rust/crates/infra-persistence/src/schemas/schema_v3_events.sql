-- ═══════════════════════════════════════════════════════════════
-- Orbis v3.0 — Event Store Schema (domain_events)
-- ═══════════════════════════════════════════════════════════════
-- Append-only event log for critical aggregates.
-- Used by: Event Sourcing, AI analysis, audit trail.
-- ═══════════════════════════════════════════════════════════════

-- ── Domain Events ───────────────────────────────────────────────
-- Stores the complete history of critical operations.
-- Each row is an immutable fact — never updated or deleted.
CREATE TABLE IF NOT EXISTS domain_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    aggregate_type  TEXT NOT NULL,   -- "enrollment", "grade", "payment", "attendance"
    aggregate_id    TEXT NOT NULL,   -- ID of the aggregate this event belongs to
    event_type      TEXT NOT NULL,   -- "enrollment.created", "grade.submitted", "payment.recorded"
    payload         TEXT NOT NULL,   -- JSON — full event data
    actor_id        TEXT NOT NULL,   -- who performed the action
    actor_type      TEXT NOT NULL,   -- "user", "system", "ai_agent", "autopilot"
    correlation_id  TEXT DEFAULT '', -- links related operations across services
    version         INTEGER NOT NULL, -- per-aggregate version (optimistic concurrency)
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Primary query: get all events for an aggregate, ordered by version
CREATE INDEX IF NOT EXISTS idx_de_aggregate
    ON domain_events(aggregate_type, aggregate_id, version);

-- AI analysis: get events by type since a timestamp
CREATE INDEX IF NOT EXISTS idx_de_type_time
    ON domain_events(event_type, created_at);

-- Correlation: find all events related to a multi-step operation
CREATE INDEX IF NOT EXISTS idx_de_correlation
    ON domain_events(correlation_id)
    WHERE correlation_id != '';

-- Actor audit: find all events by a specific actor
CREATE INDEX IF NOT EXISTS idx_de_actor
    ON domain_events(actor_id, created_at);

-- ── Process State (optional — for future persistence) ───────────
-- Currently Process Manager is in-memory only.
-- This table is created as a placeholder for future upgrade.
CREATE TABLE IF NOT EXISTS process_log (
    id              TEXT PRIMARY KEY,
    process_type    TEXT NOT NULL,   -- "student_registration", "promotion", "initial_setup"
    org_id          TEXT NOT NULL,
    status          TEXT NOT NULL,   -- "completed", "failed", "compensated"
    steps_json      TEXT NOT NULL,   -- JSON array of step results
    started_at      TEXT NOT NULL,
    completed_at    TEXT,
    error           TEXT
);

CREATE INDEX IF NOT EXISTS idx_pl_org
    ON process_log(org_id, process_type, started_at);
