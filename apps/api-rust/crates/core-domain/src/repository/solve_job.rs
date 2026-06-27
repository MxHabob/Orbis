use crate::entities::solve_job;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    pub async fn insert_solve_job(
        &self,
        job: solve_job::Model,
    ) -> Result<solve_job::Model, DbErr> {
        let active = solve_job::ActiveModel {
            id: Set(job.id),
            org_id: Set(job.org_id),
            job_type: Set(job.job_type),
            target_id: Set(job.target_id),
            mode: Set(job.mode),
            status: Set(job.status),
            problem_snapshot: Set(job.problem_snapshot),
            best_solution: Set(job.best_solution),
            score: Set(job.score),
            items_created: Set(job.items_created),
            error_message: Set(job.error_message),
            created_by: Set(job.created_by),
            started_at: Set(job.started_at),
            completed_at: Set(job.completed_at),
            created_at: Set(job.created_at),
            updated_at: Set(job.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn get_solve_job_by_id(
        &self,
        id: &str,
    ) -> Result<Option<solve_job::Model>, DbErr> {
        solve_job::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn get_solve_jobs(
        &self,
        org_id: &str,
        job_type: &str,
        target_id: Option<&str>,
    ) -> Result<Vec<solve_job::Model>, DbErr> {
        let mut query = solve_job::Entity::find()
            .filter(solve_job::Column::OrgId.eq(org_id))
            .filter(solve_job::Column::JobType.eq(job_type))
            .order_by_desc(solve_job::Column::CreatedAt);

        if let Some(tgt) = target_id {
            query = query.filter(solve_job::Column::TargetId.eq(tgt));
        }

        query.all(&self.db).await
    }

    pub async fn update_solve_job(
        &self,
        job: solve_job::Model,
    ) -> Result<solve_job::Model, DbErr> {
        let active = solve_job::ActiveModel {
            id: Set(job.id),
            org_id: Set(job.org_id),
            job_type: Set(job.job_type),
            target_id: Set(job.target_id),
            mode: Set(job.mode),
            status: Set(job.status),
            problem_snapshot: Set(job.problem_snapshot),
            best_solution: Set(job.best_solution),
            score: Set(job.score),
            items_created: Set(job.items_created),
            error_message: Set(job.error_message),
            created_by: Set(job.created_by),
            started_at: Set(job.started_at),
            completed_at: Set(job.completed_at),
            created_at: Set(job.created_at),
            updated_at: Set(chrono::Utc::now().to_rfc3339()),
        };
        active.update(&self.db).await
    }

    pub async fn count_active_solve_jobs(&self, org_id: &str, job_type: &str) -> Result<u64, DbErr> {
        solve_job::Entity::find()
            .filter(solve_job::Column::OrgId.eq(org_id))
            .filter(solve_job::Column::JobType.eq(job_type))
            .filter(
                solve_job::Column::Status
                    .is_in(["pending", "running"]),
            )
            .count(&self.db)
            .await
    }
}
