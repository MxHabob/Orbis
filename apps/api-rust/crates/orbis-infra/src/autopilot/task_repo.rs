use async_trait::async_trait;
use orbis_domain::autopilot::task::{AutopilotTask, TaskStatus, TaskType};
use orbis_domain::errors::DomainError;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveValue, ActiveModelTrait};
use crate::models::autopilot_task;
use chrono::Utc;

#[async_trait]
pub trait AutopilotTaskRepository: Send + Sync {
    async fn find_pending_tasks(&self) -> Result<Vec<AutopilotTask>, DomainError>;
    async fn save_task(&self, task: &AutopilotTask) -> Result<(), DomainError>;
}

pub struct PgAutopilotTaskRepo {
    pub db: DatabaseConnection,
}

impl PgAutopilotTaskRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AutopilotTaskRepository for PgAutopilotTaskRepo {
    async fn find_pending_tasks(&self) -> Result<Vec<AutopilotTask>, DomainError> {
        let now = Utc::now();
        let models = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::Status.eq("Pending"))
            .filter(autopilot_task::Column::ScheduledAt.lte(now))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let tasks = models.into_iter().map(|m| {
            let task_type = match m.task_type.as_str() {
                "SendDelayNotification" => TaskType::SendDelayNotification,
                "TransitionPeriod" => TaskType::TransitionPeriod,
                _ => TaskType::GenericWorkflow,
            };
            AutopilotTask {
                id: m.id,
                org_id: m.org_id,
                task_type,
                status: TaskStatus::Pending,
                parameters: m.parameters,
                scheduled_at: m.scheduled_at,
                executed_at: m.executed_at,
                error_log: m.error_log,
            }
        }).collect();

        Ok(tasks)
    }

    async fn save_task(&self, task: &AutopilotTask) -> Result<(), DomainError> {
        let status_str = match task.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::Executing => "Executing",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
            TaskStatus::Suggested => "Suggested",
        }.to_string();

        let type_str = match task.task_type {
            TaskType::SendDelayNotification => "SendDelayNotification",
            TaskType::TransitionPeriod => "TransitionPeriod",
            TaskType::GenericWorkflow => "GenericWorkflow",
        }.to_string();

        let model = autopilot_task::ActiveModel {
            id: ActiveValue::Set(task.id),
            org_id: ActiveValue::Set(task.org_id),
            task_type: ActiveValue::Set(type_str),
            status: ActiveValue::Set(status_str),
            parameters: ActiveValue::Set(task.parameters.clone()),
            scheduled_at: ActiveValue::Set(task.scheduled_at),
            executed_at: ActiveValue::Set(task.executed_at),
            error_log: ActiveValue::Set(task.error_log.clone()),
        };

        // If it exists, update it. If not, insert it.
        // We can just use `save` which does upsert if primary key is set.
        model.save(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }
}
