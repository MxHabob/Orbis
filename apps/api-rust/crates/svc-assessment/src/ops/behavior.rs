//! Behavior record operations — student behavior tracking (manual + AI).

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::behavior_record;

// ── Queries ─────────────────────────────────────────────────────

/// List behavior records for an org, optionally filtered.
pub async fn list(
    ctx: &ServiceContext,
    cycle_id: &str,
    enrollment_id: Option<String>,
) -> Result<Vec<behavior_record::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::BehaviorRead).await?;

    let mut query = behavior_record::Entity::find()
        .filter(behavior_record::Column::OrgId.eq(&ctx.org_id))
        .filter(behavior_record::Column::CycleId.eq(cycle_id));

    if let Some(eid) = enrollment_id {
        query = query.filter(behavior_record::Column::EnrollmentId.eq(eid));
    }

    Ok(query
        .order_by_desc(behavior_record::Column::IncidentDate)
        .all(&ctx.db)
        .await?)
}

// ── Commands ────────────────────────────────────────────────────

/// Submit an AI-detected behavior record.
pub async fn submit_ai(
    ctx: &ServiceContext,
    cycle_id: String,
    enrollment_id: String,
    behavior_type: String,
    severity: String,
    description: String,
    confidence: Option<f64>,
    camera_id: Option<String>,
    model_version: Option<String>,
    location: Option<String>,
    image_evidence: Option<String>,
) -> Result<behavior_record::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::BehaviorAI).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = behavior_record::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        cycle_id,
        enrollment_id,
        behavior_type,
        severity,
        category: None,
        title: "AI Detected Behavior".into(),
        title_ar: Some("سلوك مكتشف بالذكاء الاصطناعي".into()),
        description,
        incident_date: now.clone(),
        incident_time: Some(Utc::now().format("%H:%M:%S").to_string()),
        location,
        points: None,
        action_taken: None,
        action_details: None,
        action_completed: None,
        reported_by: ctx.actor.as_str().to_string(),
        reviewed_by: None,
        reviewed_at: None,
        witnesses: None,
        parent_notified: 0,
        parent_notified_at: None,
        parent_response: None,
        evidence_urls: image_evidence,
        status: "pending_review".into(),
        resolution_notes: None,
        resolved_at: None,
        resolved_by: None,
        is_private: 0,
        ai_source: Some("orbis_vision".into()),
        ai_confidence: confidence,
        ai_model_version: model_version,
        camera_id,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: behavior_record::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "behavior.submit_ai".into(),
        entity_type: "behavior_record".into(),
        entity_id: id,
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

/// Update a behavior record (e.g. review, resolve).
pub async fn update(
    ctx: &ServiceContext,
    record_id: &str,
    action_taken: Option<String>,
    status: Option<String>,
    resolution_notes: Option<String>,
) -> Result<behavior_record::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::BehaviorWrite).await?;

    let existing = behavior_record::Entity::find_by_id(record_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("behavior_record", record_id))?;

    let mut active: behavior_record::ActiveModel = existing.into();
    if let Some(action) = action_taken {
        active.action_taken = Set(Some(action));
    }
    if let Some(s) = status {
        if s == "resolved" {
            active.resolved_at = Set(Some(Utc::now().to_rfc3339()));
            active.resolved_by = Set(Some(ctx.actor.as_str().to_string()));
        }
        active.status = Set(s);
    }
    if let Some(notes) = resolution_notes {
        active.resolution_notes = Set(Some(notes));
    }
    active.reviewed_by = Set(Some(ctx.actor.as_str().to_string()));
    active.reviewed_at = Set(Some(Utc::now().to_rfc3339()));
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "behavior.update".into(),
        entity_type: "behavior_record".into(),
        entity_id: record_id.into(),
        old_data: None,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

/// Delete a behavior record.
pub async fn delete(ctx: &ServiceContext, record_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::BehaviorDelete).await?;

    let existing = behavior_record::Entity::find_by_id(record_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("behavior_record", record_id))?;

    existing.delete(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "behavior.delete".into(),
        entity_type: "behavior_record".into(),
        entity_id: record_id.into(),
        old_data: None,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
