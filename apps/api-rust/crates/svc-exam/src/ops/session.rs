//! Exam session operations — CRUD for exam periods.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::exam_session;

pub async fn list(ctx: &ServiceContext, period_id: &str) -> Result<Vec<exam_session::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(exam_session::Entity::find()
        .filter(exam_session::Column::OrgId.eq(&ctx.org_id))
        .filter(exam_session::Column::PeriodId.eq(period_id))
        .order_by_desc(exam_session::Column::StartDate)
        .all(&ctx.db).await?)
}

pub async fn get(ctx: &ServiceContext, session_id: &str) -> Result<exam_session::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    exam_session::Entity::find_by_id(session_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("exam_session", session_id))
}

#[derive(serde::Deserialize)]
pub struct CreateExamSessionRequest {
    pub period_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub session_type: String,
    pub start_date: String,
    pub end_date: String,
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<exam_session::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let req: CreateExamSessionRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid session payload: {e}")))?;

    // ── Period Gate: check exam window ──
    {
        use shared_kernel::period_gate::GateType;
        let verdict = ctx.period_gate
            .check(&ctx.db, &ctx.org_id, &req.period_id, GateType::ExamEntry)
            .await?;
        verdict.enforce()?;
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = exam_session::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), period_id: req.period_id,
        name: req.name, name_ar: req.name_ar, code: None, session_type: req.session_type,
        status: "draft".into(), start_date: req.start_date, end_date: req.end_date,
        hijri_start_date: None, hijri_end_date: None,
        applicable_levels: None, applicable_class_ids: None,
        settings: None, grading_deadline: None,
        results_publish_date: None, is_results_published: 0,
        created_by: ctx.actor.as_str().to_string(),
        approved_by: None, approved_at: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: exam_session::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::EXAM_CREATED.into(),
        entity_type: "exam_session".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "exam_session.create".into(), entity_type: "exam_session".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext, session_id: &str,
    name: Option<String>, status: Option<String>,
    start_date: Option<String>, end_date: Option<String>,
) -> Result<exam_session::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let existing = get(ctx, session_id).await?;
    let mut active: exam_session::ActiveModel = existing.into();

    if let Some(n) = name { active.name = Set(n); }
    if let Some(s) = status { active.status = Set(s); }
    if let Some(sd) = start_date { active.start_date = Set(sd); }
    if let Some(ed) = end_date { active.end_date = Set(ed); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}
