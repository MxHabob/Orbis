use async_trait::async_trait;
use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::models::autopilot_task;
use orbis_app::ports::repositories::AutopilotTaskRepository;
use orbis_domain::autopilot::task::{AutopilotTask, RiskLevel, TaskStatus, TaskType};
use orbis_domain::errors::DomainError;

pub struct PgAutopilotTaskRepo {
    pub db: DatabaseConnection,
}

impl PgAutopilotTaskRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(m: autopilot_task::Model) -> AutopilotTask {
        AutopilotTask {
            id: m.id,
            org_id: m.org_id,
            task_type: TaskType::from_str(&m.task_type),
            status: TaskStatus::from_str(&m.status),
            target_id: m.target_id,
            target_type: m.target_type,
            risk_level: RiskLevel::from_str(&m.risk_level),
            reason: m.reason,
            parameters: m.parameters,
            scheduled_at: m.scheduled_at,
            executed_at: m.executed_at,
            attempts: m.attempts,
            max_retries: m.max_retries,
            error_log: m.error_log,
            idempotency_key: m.idempotency_key,
            triggered_by: m.triggered_by,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[async_trait]
impl AutopilotTaskRepository for PgAutopilotTaskRepo {
    async fn update_task_state(
        &self,
        task: &AutopilotTask,
        expected_status: TaskStatus,
    ) -> Result<(), DomainError> {
        let result = autopilot_task::Entity::update_many()
            .col_expr(
                autopilot_task::Column::Status,
                Expr::value(task.status.as_str()),
            )
            .col_expr(
                autopilot_task::Column::Reason,
                Expr::value(task.reason.clone()),
            )
            .col_expr(
                autopilot_task::Column::ExecutedAt,
                Expr::value(task.executed_at),
            )
            .col_expr(
                autopilot_task::Column::ErrorLog,
                Expr::value(task.error_log.clone()),
            )
            .col_expr(
                autopilot_task::Column::UpdatedAt,
                Expr::value(task.updated_at),
            )
            .filter(autopilot_task::Column::Id.eq(task.id))
            .filter(autopilot_task::Column::OrgId.eq(task.org_id))
            .filter(autopilot_task::Column::Status.eq(expected_status.as_str()))
            .exec(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;

        if result.rows_affected != 1 {
            return Err(DomainError::Conflict("Autopilot task state changed".into()));
        }
        Ok(())
    }

    async fn find_pending_tasks(
        &self,
        org_id: Option<Uuid>,
    ) -> Result<Vec<AutopilotTask>, DomainError> {
        let now = Utc::now();
        let mut query = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::Status.eq("pending"))
            .filter(autopilot_task::Column::ScheduledAt.lte(now));

        if let Some(oid) = org_id {
            query = query.filter(autopilot_task::Column::OrgId.eq(oid));
        }

        let models = query
            .order_by_asc(autopilot_task::Column::ScheduledAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(Self::model_to_domain).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AutopilotTask>, DomainError> {
        let model = autopilot_task::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(Self::model_to_domain))
    }

    async fn claim_task(
        &self,
        id: Uuid,
        org_id: Uuid,
        expected_status: TaskStatus,
        expected_attempts: i32,
    ) -> Result<bool, DomainError> {
        if !matches!(
            expected_status,
            TaskStatus::Pending | TaskStatus::Suggested | TaskStatus::Failed
        ) {
            return Err(DomainError::Validation(
                "Only pending, suggested, or failed tasks can be claimed".into(),
            ));
        }

        let attempts = if expected_status == TaskStatus::Failed {
            Expr::value(1)
        } else {
            Expr::col(autopilot_task::Column::Attempts).add(1)
        };
        let result = autopilot_task::Entity::update_many()
            .col_expr(
                autopilot_task::Column::Status,
                Expr::value(TaskStatus::Executing.as_str()),
            )
            .col_expr(autopilot_task::Column::Attempts, attempts)
            .col_expr(autopilot_task::Column::UpdatedAt, Expr::value(Utc::now()))
            .filter(autopilot_task::Column::Id.eq(id))
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .filter(autopilot_task::Column::Status.eq(expected_status.as_str()))
            .filter(autopilot_task::Column::Attempts.eq(expected_attempts))
            .exec(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;

        Ok(result.rows_affected == 1)
    }

    async fn count_by_status(&self, org_id: Uuid) -> Result<Vec<(TaskStatus, u64)>, DomainError> {
        let rows = autopilot_task::Entity::find()
            .select_only()
            .column(autopilot_task::Column::Status)
            .column_as(autopilot_task::Column::Id.count(), "task_count")
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .group_by(autopilot_task::Column::Status)
            .into_tuple::<(String, i64)>()
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        rows.into_iter()
            .map(|(status, count)| {
                u64::try_from(count)
                    .map(|count| (TaskStatus::from_str(&status), count))
                    .map_err(|error| DomainError::Internal(error.to_string()))
            })
            .collect()
    }

    async fn find_by_org(
        &self,
        org_id: Uuid,
        status: Option<&str>,
        task_type: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<AutopilotTask>, u64), DomainError> {
        let mut query =
            autopilot_task::Entity::find().filter(autopilot_task::Column::OrgId.eq(org_id));

        if let Some(s) = status {
            query = query.filter(autopilot_task::Column::Status.eq(s.to_lowercase()));
        }

        if let Some(tt) = task_type {
            query = query.filter(autopilot_task::Column::TaskType.eq(tt.to_lowercase()));
        }

        let total = query
            .clone()
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let models = query
            .order_by_desc(autopilot_task::Column::ScheduledAt)
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let tasks = models.into_iter().map(Self::model_to_domain).collect();
        Ok((tasks, total))
    }

    async fn schedule_task(&self, task: &AutopilotTask) -> Result<AutopilotTask, DomainError> {
        let key = task.idempotency_key.as_deref().ok_or_else(|| {
            DomainError::Validation("A scheduled Autopilot task requires an idempotency key".into())
        })?;
        let scoped_tasks =
            autopilot_task::Entity::find().filter(autopilot_task::Column::OrgId.eq(task.org_id));
        // Honor actionable rows written before scheduled tasks had a stable key.
        if let Some(target_id) = task.target_id {
            let existing = scoped_tasks
                .clone()
                .filter(autopilot_task::Column::TaskType.eq(task.task_type.as_str()))
                .filter(autopilot_task::Column::TargetId.eq(target_id))
                .filter(autopilot_task::Column::IdempotencyKey.is_null())
                .filter(autopilot_task::Column::Status.is_in([
                    "pending",
                    "suggested",
                    "executing",
                    "failed",
                ]))
                .order_by_asc(autopilot_task::Column::CreatedAt)
                .order_by_asc(autopilot_task::Column::Id)
                .one(&self.db)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            if let Some(existing) = existing {
                return Ok(Self::model_to_domain(existing));
            }
        }

        let active = autopilot_task::ActiveModel {
            id: ActiveValue::Set(task.id),
            org_id: ActiveValue::Set(task.org_id),
            task_type: ActiveValue::Set(task.task_type.as_str().to_owned()),
            status: ActiveValue::Set(task.status.as_str().to_owned()),
            target_id: ActiveValue::Set(task.target_id),
            target_type: ActiveValue::Set(task.target_type.clone()),
            risk_level: ActiveValue::Set(task.risk_level.as_str().to_owned()),
            reason: ActiveValue::Set(task.reason.clone()),
            parameters: ActiveValue::Set(task.parameters.clone()),
            scheduled_at: ActiveValue::Set(task.scheduled_at),
            executed_at: ActiveValue::Set(task.executed_at),
            attempts: ActiveValue::Set(task.attempts),
            max_retries: ActiveValue::Set(task.max_retries),
            error_log: ActiveValue::Set(task.error_log.clone()),
            idempotency_key: ActiveValue::Set(task.idempotency_key.clone()),
            triggered_by: ActiveValue::Set(task.triggered_by.clone()),
            created_at: ActiveValue::Set(task.created_at),
            updated_at: ActiveValue::Set(task.updated_at),
        };
        let inserted = autopilot_task::Entity::insert(active)
            .on_conflict(
                sea_orm::sea_query::OnConflict::columns([
                    autopilot_task::Column::OrgId,
                    autopilot_task::Column::IdempotencyKey,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec_without_returning(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if inserted == 1 {
            return Ok(task.clone());
        }

        scoped_tasks
            .filter(autopilot_task::Column::IdempotencyKey.eq(key))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .map(Self::model_to_domain)
            .ok_or_else(|| DomainError::Conflict("Scheduled Autopilot task disappeared".into()))
    }

    async fn delete_task(&self, id: Uuid) -> Result<(), DomainError> {
        autopilot_task::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn find_stuck_tasks(
        &self,
        org_id: Uuid,
        threshold_seconds: i64,
    ) -> Result<Vec<AutopilotTask>, DomainError> {
        let cutoff = Utc::now() - chrono::Duration::seconds(threshold_seconds);
        let models = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .filter(autopilot_task::Column::Status.eq("executing"))
            .filter(autopilot_task::Column::UpdatedAt.lt(cutoff))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(Self::model_to_domain).collect())
    }

    async fn find_overdue_tasks(&self, org_id: Uuid) -> Result<Vec<AutopilotTask>, DomainError> {
        let now = Utc::now();
        let models = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .filter(autopilot_task::Column::Status.eq("pending"))
            .filter(autopilot_task::Column::ScheduledAt.lt(now))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(Self::model_to_domain).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ConnectionTrait, Database, DatabaseBackend, Statement};

    #[tokio::test]
    async fn task_claim_is_scoped_and_only_succeeds_once() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE autopilot_tasks (
                id TEXT PRIMARY KEY NOT NULL,
                org_id TEXT NOT NULL,
                status TEXT NOT NULL,
                attempts INTEGER NOT NULL,
                updated_at TEXT NOT NULL
            )",
        )
        .await
        .unwrap();

        let task_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "INSERT INTO autopilot_tasks (id, org_id, status, attempts, updated_at) VALUES (?, ?, 'pending', 0, ?)",
            [
                task_id.into(),
                org_id.into(),
                Utc::now().into(),
            ],
        ))
        .await
        .unwrap();

        let repo = PgAutopilotTaskRepo::new(db.clone());
        assert!(
            !repo
                .claim_task(task_id, Uuid::new_v4(), TaskStatus::Pending, 0)
                .await
                .unwrap()
        );
        assert!(
            repo.claim_task(task_id, org_id, TaskStatus::Pending, 0)
                .await
                .unwrap()
        );
        assert!(
            !repo
                .claim_task(task_id, org_id, TaskStatus::Pending, 0)
                .await
                .unwrap()
        );

        let row = db
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "SELECT status, attempts FROM autopilot_tasks WHERE id = ?",
                [task_id.into()],
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(row.try_get_by::<String, _>("status").unwrap(), "executing");
        assert_eq!(row.try_get_by::<i32, _>("attempts").unwrap(), 1);
    }

    #[tokio::test]
    async fn retry_claim_resets_the_attempt_counter_atomically() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE autopilot_tasks (
                id TEXT PRIMARY KEY NOT NULL,
                org_id TEXT NOT NULL,
                status TEXT NOT NULL,
                attempts INTEGER NOT NULL,
                updated_at TEXT NOT NULL
            )",
        )
        .await
        .unwrap();

        let task_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "INSERT INTO autopilot_tasks (id, org_id, status, attempts, updated_at) VALUES (?, ?, 'failed', 4, ?)",
            [
                task_id.into(),
                org_id.into(),
                Utc::now().into(),
            ],
        ))
        .await
        .unwrap();

        let repo = PgAutopilotTaskRepo::new(db.clone());
        assert!(
            repo.claim_task(task_id, org_id, TaskStatus::Failed, 4)
                .await
                .unwrap()
        );

        let row = db
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "SELECT status, attempts FROM autopilot_tasks WHERE id = ?",
                [task_id.into()],
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(row.try_get_by::<String, _>("status").unwrap(), "executing");
        assert_eq!(row.try_get_by::<i32, _>("attempts").unwrap(), 1);
    }
}
