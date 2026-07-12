//! Enrollment operations — student enrollment lifecycle.
//!
//! Handles: enroll, withdraw, transfer + duplicate/capacity checks.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::{enrollment, class};

// ── Queries ─────────────────────────────────────────────────────

/// List enrollments, optionally filtered by class and/or period.
pub async fn list(
    ctx: &ServiceContext,
) -> Result<Vec<enrollment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    Ok(enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(enrollment::Column::StudentNumber)
        .all(&ctx.db)
        .await?)
}

/// Get a single enrollment by ID.
pub async fn get(ctx: &ServiceContext, id: &str) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    enrollment::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("enrollment", id))
}

// ── Commands ────────────────────────────────────────────────────

/// Enroll a student in a class.
///
/// Business rules enforced:
/// - No duplicate enrollment in the same period
/// - Class capacity check
/// - Mandatory fee assignment (via FeeAssigner trait)
#[derive(serde::Deserialize)]
pub struct CreateEnrollmentPayload {
    pub org_id: Option<String>,
    pub period_id: String,
    pub student_id: String,
    pub class_id: String,
    pub student_number: String,
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let req: CreateEnrollmentPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create enrollment payload: {e}")))?;

    let org_id = req.org_id.unwrap_or_else(|| ctx.org_id.clone());

    // Business rule: no duplicate enrollment in the same period
    let existing = enrollment::Entity::find()
        .filter(enrollment::Column::StudentId.eq(&req.student_id))
        .filter(enrollment::Column::PeriodId.eq(&req.period_id))
        .filter(enrollment::Column::Status.ne("withdrawn"))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::conflict("Student is already enrolled in this period"));
    }

    // Business rule: class capacity check
    let target_class = class::Entity::find_by_id(req.class_id.clone())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", &req.class_id))?;

    if let Some(cap) = target_class.capacity {
        let current = enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(&req.class_id))
            .filter(enrollment::Column::PeriodId.eq(&req.period_id))
            .filter(enrollment::Column::Status.ne("withdrawn"))
            .count(&ctx.db)
            .await?;
        if current as i32 >= cap {
            return Err(AppError::conflict(format!(
                "Class '{}' is at capacity ({}/{})", target_class.name, current, cap
            )));
        }
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = enrollment::Model {
        id: id.clone(),
        org_id: org_id.clone(),
        period_id: req.period_id,
        student_id: req.student_id,
        class_id: req.class_id.clone(),
        student_number: req.student_number,
        status: "active".into(),
        enrollment_date: now.clone(),
        enrollment_type: "regular".into(),
        withdrawal_date: None,
        withdrawal_reason: None,
        final_grade: None,
        final_score: None,
        final_rank: None,
        is_promoted: None,
        promotion_decision: None,
        promotion_notes: None,
        meta: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: enrollment::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    // Cross-domain: assign mandatory fees
    if let Err(e) = ctx.fee_assigner.assign_mandatory_fees(&org_id, &id, &req.class_id).await {
        tracing::warn!("Fee assignment failed for enrollment {}: {}", id, e);
        // Non-blocking — enrollment succeeds even if fee assignment fails
    }

    // Audit
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.create".into(),
        entity_type: "enrollment".into(),
        entity_id: id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    // Domain event
    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_CREATED.into(),
        entity_type: "enrollment".into(),
        entity_id: id,
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(created)
}

/// Withdraw a student from enrollment.
pub async fn withdraw(
    ctx: &ServiceContext,
    enrollment_id: &str,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: enrollment::ActiveModel = existing.into();
    active.status = Set("withdrawn".into());
    active.withdrawal_date = Set(Some(Utc::now().to_rfc3339()));
    active.withdrawal_reason = Set(Some("Not specified".to_string()));
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.withdraw".into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_WITHDRAWN.into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(updated)
}

/// Transfer a student to a different class.
pub async fn transfer(
    ctx: &ServiceContext,
    enrollment_id: &str,
    new_class_id: &str,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentTransfer).await?;

    // Verify new class exists
    class::Entity::find_by_id(new_class_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", new_class_id))?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: enrollment::ActiveModel = existing.into();
    active.class_id = Set(new_class_id.to_string());
    active.enrollment_type = Set("transfer".into());
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.transfer".into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_TRANSFERRED.into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(updated)
}

/// Check if a student is already enrolled (for duplicate detection UI).
pub async fn check_duplicate(
    ctx: &ServiceContext,
    national_id: Option<String>,
    email: Option<String>,
    phone: Option<String>,
) -> Result<Option<enrollment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let mut user_ids = Vec::new();

    if let Some(nid) = national_id {
        if !nid.is_empty() {
            let u = svc_iam::entities::user::Entity::find()
                .filter(svc_iam::entities::user::Column::NationalId.eq(nid))
                .one(&ctx.db)
                .await?;
            if let Some(user) = u {
                user_ids.push(user.id);
            }
        }
    }
    if let Some(em) = email {
        if !em.is_empty() {
            let u = svc_iam::entities::user::Entity::find()
                .filter(svc_iam::entities::user::Column::Email.eq(em))
                .one(&ctx.db)
                .await?;
            if let Some(user) = u {
                user_ids.push(user.id);
            }
        }
    }
    if let Some(ph) = phone {
        if !ph.is_empty() {
            let u = svc_iam::entities::user::Entity::find()
                .filter(svc_iam::entities::user::Column::Phone.eq(ph))
                .one(&ctx.db)
                .await?;
            if let Some(user) = u {
                user_ids.push(user.id);
            }
        }
    }

    if user_ids.is_empty() {
        return Ok(None);
    }

    Ok(enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .filter(enrollment::Column::StudentId.is_in(user_ids))
        .filter(enrollment::Column::Status.ne("withdrawn"))
        .one(&ctx.db)
        .await?)
}

#[derive(serde::Deserialize)]
pub struct PromotePayload {
    pub is_promoted: i32,
    pub promotion_decision: Option<String>,
    pub promotion_notes: Option<String>,
}

pub async fn promote(
    ctx: &ServiceContext,
    enrollment_id: &str,
    data: serde_json::Value,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let req: PromotePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid promote payload: {e}")))?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: enrollment::ActiveModel = existing.into();
    active.is_promoted = Set(Some(req.is_promoted));
    active.promotion_decision = Set(req.promotion_decision);
    active.promotion_notes = Set(req.promotion_notes);
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "enrollment.promote".into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ENROLLMENT_PROMOTED.into(),
        entity_type: "enrollment".into(),
        entity_id: enrollment_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(updated)
}
