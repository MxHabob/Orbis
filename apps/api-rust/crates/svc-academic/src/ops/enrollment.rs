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

/// List enrollments, optionally filtered by class and/or cycle.
pub async fn list(
    ctx: &ServiceContext,
    class_id: Option<String>,
    cycle_id: Option<String>,
) -> Result<Vec<enrollment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    let mut query = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id));

    if let Some(cid) = class_id {
        query = query.filter(enrollment::Column::ClassId.eq(cid));
    }
    if let Some(cyc) = cycle_id {
        query = query.filter(enrollment::Column::CycleId.eq(cyc));
    }

    Ok(query
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
/// - No duplicate enrollment in the same cycle
/// - Class capacity check
/// - Mandatory fee assignment (via FeeAssigner trait)
pub async fn create(
    ctx: &ServiceContext,
    org_id: String,
    cycle_id: String,
    student_id: String,
    class_id: String,
    student_number: String,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    // Business rule: no duplicate enrollment in the same cycle
    let existing = enrollment::Entity::find()
        .filter(enrollment::Column::StudentId.eq(&student_id))
        .filter(enrollment::Column::CycleId.eq(&cycle_id))
        .filter(enrollment::Column::Status.ne("withdrawn"))
        .one(&ctx.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::conflict("Student is already enrolled in this cycle"));
    }

    // Business rule: class capacity check
    let target_class = class::Entity::find_by_id(class_id.clone())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", &class_id))?;

    if let Some(cap) = target_class.capacity {
        let current = enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(&class_id))
            .filter(enrollment::Column::CycleId.eq(&cycle_id))
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
        cycle_id,
        student_id,
        class_id: class_id.clone(),
        student_number,
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
    if let Err(e) = ctx.fee_assigner.assign_mandatory_fees(&org_id, &id, &class_id).await {
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
    reason: String,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentWrite).await?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: enrollment::ActiveModel = existing.into();
    active.status = Set("withdrawn".into());
    active.withdrawal_date = Set(Some(Utc::now().to_rfc3339()));
    active.withdrawal_reason = Set(Some(reason));
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
    new_class_id: String,
) -> Result<enrollment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentTransfer).await?;

    // Verify new class exists
    class::Entity::find_by_id(new_class_id.clone())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", &new_class_id))?;

    let existing = get(ctx, enrollment_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: enrollment::ActiveModel = existing.into();
    active.class_id = Set(new_class_id);
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
    student_id: &str,
    cycle_id: &str,
) -> Result<Option<enrollment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::EnrollmentRead).await?;

    Ok(enrollment::Entity::find()
        .filter(enrollment::Column::StudentId.eq(student_id))
        .filter(enrollment::Column::CycleId.eq(cycle_id))
        .filter(enrollment::Column::Status.ne("withdrawn"))
        .one(&ctx.db)
        .await?)
}
