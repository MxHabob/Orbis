//! Schedule solver operations — AI/constraint-based schedule generation.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::solve_job;

pub async fn start(
    ctx: &ServiceContext, cycle_id: &str,
) -> Result<solve_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = solve_job::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        job_type: "schedule".into(), target_id: cycle_id.to_string(),
        mode: None, status: "pending".into(),
        problem_snapshot: None, best_solution: None, score: None,
        items_created: 0, error_message: None,
        created_by: Some(ctx.actor.as_str().to_string()),
        started_at: None, completed_at: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: solve_job::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "schedule_solve.start".into(), entity_type: "solve_job".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn get_job(ctx: &ServiceContext, job_id: &str) -> Result<solve_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    solve_job::Entity::find_by_id(job_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("solve_job", job_id))
}

pub async fn list_jobs(ctx: &ServiceContext, cycle_id: &str) -> Result<Vec<solve_job::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(solve_job::Entity::find()
        .filter(solve_job::Column::OrgId.eq(&ctx.org_id))
        .filter(solve_job::Column::JobType.eq("schedule"))
        .filter(solve_job::Column::TargetId.eq(cycle_id))
        .order_by_desc(solve_job::Column::CreatedAt)
        .all(&ctx.db).await?)
}

pub async fn cancel(ctx: &ServiceContext, job_id: &str) -> Result<solve_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let existing = get_job(ctx, job_id).await?;
    let mut active: solve_job::ActiveModel = existing.into();
    active.status = Set("cancelled".into());
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete_job(ctx: &ServiceContext, job_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;
    let existing = get_job(ctx, job_id).await?;
    existing.delete(&ctx.db).await?;
    Ok(())
}

/// Apply solved schedule — creates lesson records from solution.
pub async fn apply(ctx: &ServiceContext, job_id: &str) -> Result<solve_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let existing = get_job(ctx, job_id).await?;
    if existing.status != "completed" {
        return Err(AppError::validation("Can only apply completed solve jobs"));
    }

    // Delegate to service layer for complex solution parsing
    let query_repo = std::sync::Arc::new(crate::repository::SchedulingQueryRepo::new(ctx.db.clone()));
    let cmd_repo = std::sync::Arc::new(crate::repository::SchedulingCommandRepo::new(ctx.db.clone()));
    let schedule_service = std::sync::Arc::new(crate::services::schedule::ScheduleService::new(
        cmd_repo.clone(),
        query_repo.clone(),
    ));
    let scheduler = std::sync::Arc::new(infra_scheduler::NoOpScheduleSolver);
    let svc = crate::services::solver::TimetableSolverService::new(
        cmd_repo.clone(),
        query_repo.clone(),
        schedule_service,
        scheduler,
    );
    svc.apply_schedule_solve(&ctx.org_id, job_id).await?;

    let updated = get_job(ctx, job_id).await?;
    Ok(updated)
}
