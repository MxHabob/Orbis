//! Promotion operations — year-end student promotion workflow.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;
use std::sync::Arc;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::promotion_job;
use crate::services::promotion::{PromotionConfig, PromotionPreview, PromotionResult, PromotionService};

pub async fn list_jobs(ctx: &ServiceContext) -> Result<Vec<promotion_job::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PromotionRead).await?;
    Ok(promotion_job::Entity::find()
        .filter(promotion_job::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(promotion_job::Column::CreatedAt)
        .all(&ctx.db).await?)
}

pub async fn get_job(ctx: &ServiceContext, job_id: &str) -> Result<promotion_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PromotionRead).await?;
    promotion_job::Entity::find_by_id(job_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("promotion_job", job_id))
}

pub async fn create_job(
    ctx: &ServiceContext,
    source_year_id: String, config: String,
) -> Result<promotion_job::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PromotionWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = promotion_job::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        source_year_id, target_year_id: None,
        status: "pending".into(),
        initiated_by: ctx.actor.as_str().to_string(),
        initiated_by_name: None, config,
        dry_run_result: None, dry_run_at: None,
        total_students: None, promoted_count: None,
        retained_count: None, graduated_count: None,
        withdrawn_count: None, excluded_count: None,
        backup_path: None, error_log: None, rollback_available: 0,
        started_at: None, completed_at: None,
        rolled_back_at: None, rolled_back_by: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: promotion_job::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "promotion.create".into(), entity_type: "promotion_job".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

fn build_service(ctx: &ServiceContext) -> PromotionService {
    let query_repo = crate::repository::LifeperiodQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::LifeperiodCommandRepo::new(ctx.db.clone());
    PromotionService::new(Arc::new(cmd_repo), Arc::new(query_repo))
}

/// dry_run(&self, year_db, org_id, user_id, config) → PromotionPreview
pub async fn start_dry_run(
    ctx: &ServiceContext,
    job_id: &str,
    config: PromotionConfig,
) -> Result<PromotionPreview, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PromotionWrite).await?;

    let existing = get_job(ctx, job_id).await?;
    if existing.status != "pending" {
        return Err(AppError::validation("Can only dry-run pending promotion jobs"));
    }

    let svc = build_service(ctx);
    svc.dry_run(&ctx.db, &ctx.org_id, ctx.actor.as_str(), config).await
}

/// execute(&self, source_db, job_id, app_data_dir) → PromotionResult
pub async fn execute(
    ctx: &ServiceContext,
    job_id: &str,
    app_data_dir: &std::path::Path,
) -> Result<PromotionResult, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PromotionExecute).await?;

    let existing = get_job(ctx, job_id).await?;
    if existing.status != "dry_run_complete" && existing.status != "dry_run" && existing.status != "confirmed" {
        return Err(AppError::validation("Must complete dry-run before executing"));
    }

    let svc = build_service(ctx);
    let result = svc.execute(&ctx.db, job_id, app_data_dir).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "promotion.execute".into(), entity_type: "promotion_job".into(),
        entity_id: job_id.into(), old_data: None,
        new_data: Some(serde_json::to_value(&result).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(result)
}

/// rollback(&self, job_id, user_id, app_data_dir) → ()
pub async fn rollback(
    ctx: &ServiceContext,
    job_id: &str,
    app_data_dir: &std::path::Path,
) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PromotionExecute).await?;

    let existing = get_job(ctx, job_id).await?;
    if existing.rollback_available != 1 {
        return Err(AppError::validation("Rollback not available for this job"));
    }

    let svc = build_service(ctx);
    svc.rollback(job_id, ctx.actor.as_str(), app_data_dir).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "promotion.rollback".into(), entity_type: "promotion_job".into(),
        entity_id: job_id.into(), old_data: None, new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
