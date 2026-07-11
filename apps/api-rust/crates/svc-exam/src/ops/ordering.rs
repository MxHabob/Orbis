//! Student ordering operations — seating numbers, secret numbers.

use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::{seating_number, secret_number, student_ordering};

/// List seating numbers for an exam session.
pub async fn list_seating(
    ctx: &ServiceContext, exam_session_id: &str,
) -> Result<Vec<seating_number::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(seating_number::Entity::find()
        .filter(seating_number::Column::ExamSessionId.eq(exam_session_id))
        .order_by_asc(seating_number::Column::SeatingNumber)
        .all(&ctx.db).await?)
}

/// List secret numbers for an exam session.
pub async fn list_secret_numbers(
    ctx: &ServiceContext, session_id: &str,
) -> Result<Vec<secret_number::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(secret_number::Entity::find()
        .filter(secret_number::Column::ExamSessionId.eq(session_id))
        .order_by_asc(secret_number::Column::SecretNumber)
        .all(&ctx.db).await?)
}

/// List student ordering for a cycle.
pub async fn list_ordering(
    ctx: &ServiceContext, cycle_id: &str,
) -> Result<Vec<student_ordering::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(student_ordering::Entity::find()
        .filter(student_ordering::Column::CycleId.eq(cycle_id))
        .order_by_asc(student_ordering::Column::OrderInClass)
        .all(&ctx.db).await?)
}

/// Generate seating & secret numbers for a session (delegated to solver service).
pub async fn generate_ordering(
    ctx: &ServiceContext, session_id: &str,
) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let query_repo = crate::repository::ExamQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::ExamCommandRepo::new(ctx.db.clone());
    let svc = crate::services::exam_ordering::ExamOrderingService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.generate_ordering(&ctx.org_id, session_id).await?;

    ctx.audit.log(shared_kernel::traits::AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "exam_ordering.generate".into(), entity_type: "exam_session".into(),
        entity_id: session_id.into(), old_data: None, new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
