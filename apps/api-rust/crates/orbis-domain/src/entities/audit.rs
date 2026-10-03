use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single audit trail entry, stored in the `audit_events` table.
///
/// Represents a recorded action performed by an actor on an entity.
/// Audit events are append-only — they are never updated or deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent {
    pub id: Uuid,
    pub org_id: Uuid,

    // Who
    pub actor_id: Option<Uuid>,
    pub actor_type: String,

    // What
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,

    // Where (scope)
    pub org_unit_id: Option<Uuid>,

    // Context
    pub request_id: Option<String>,
    pub source: String,

    // Details (optional)
    /// Field-level changes for updates: `{"field": {"old": ..., "new": ...}}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<serde_json::Value>,
    /// Extra context: reason, notes, batch metadata, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    // When
    pub created_at: DateTime<Utc>,
}

/// Filter parameters for querying audit events.
#[derive(Debug, Clone, Default)]
pub struct AuditEventFilter {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub actor_id: Option<Uuid>,
    pub action: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
