//! Schedule solver operations — AI/constraint-based schedule generation.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::solve_job;
use crate::repository::{SchedulingCommandRepo, SchedulingQueryRepo};
use crate::services::{schedule::ScheduleService, solver::{StartSolveRequest, TimetableSolverService}};
use infra_scheduler::RemoteTimefoldSolver;

pub async fn start(
    ctx: &ServiceContext, period_id: &str,
) -> Result<crate::services::solver::SolveJobResponse, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let query_repo = std::sync::Arc::new(SchedulingQueryRepo::new(ctx.db.clone()));
    let cmd_repo = std::sync::Arc::new(SchedulingCommandRepo::new(ctx.db.clone()));
    let schedule_service = std::sync::Arc::new(ScheduleService::new(cmd_repo.clone(), query_repo.clone()));
    let solver_url = std::env::var("SOLVER_SERVICE_URL")
        .ok()
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| "http://localhost:8081".to_string());
    let solver = std::sync::Arc::new(RemoteTimefoldSolver::new(solver_url));
    let service = TimetableSolverService::new(cmd_repo, query_repo, schedule_service, solver);

    service.start_schedule_solve(
        &ctx.org_id,
        ctx.actor.as_str(),
        StartSolveRequest { period_id: period_id.to_string(), mode: Default::default() },
    ).await
}

pub async fn get_job(ctx: &ServiceContext, job_id: &str) -> Result<solve_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    let job = solve_job::Entity::find_by_id(job_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("solve_job", job_id))
        ?;
    if job.org_id != ctx.org_id {
        return Err(AppError::not_found("solve_job", job_id));
    }
    Ok(job)
}

pub async fn list_jobs(ctx: &ServiceContext, period_id: &str) -> Result<Vec<solve_job::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(solve_job::Entity::find()
        .filter(solve_job::Column::OrgId.eq(&ctx.org_id))
        .filter(solve_job::Column::JobType.eq("timetable"))
        .filter(solve_job::Column::TargetId.eq(period_id))
        .order_by_desc(solve_job::Column::CreatedAt)
        .all(&ctx.db).await?)
}

pub async fn cancel(ctx: &ServiceContext, job_id: &str) -> Result<solve_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let existing = get_job(ctx, job_id).await?;
    let solver_url = std::env::var("SOLVER_SERVICE_URL")
        .ok()
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| "http://localhost:8081".to_string());
    let solver = RemoteTimefoldSolver::new(solver_url);
    let _ = infra_scheduler::ScheduleSolver::cancel(&solver, job_id).await;
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
    let solver_url = std::env::var("SOLVER_SERVICE_URL")
        .ok()
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| "http://localhost:8081".to_string());
    let scheduler = std::sync::Arc::new(RemoteTimefoldSolver::new(solver_url));
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
