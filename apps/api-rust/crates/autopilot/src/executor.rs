use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventBus;
use shared_kernel::period_gate::PeriodGate;
use std::sync::Arc;

use crate::entities::autopilot_task;

#[async_trait]
pub trait AutopilotExecutor: Send + Sync {
    fn can_handle(&self, task_type: &str) -> bool;
    
    async fn execute(
        &self,
        db: &DatabaseConnection,
        task: &autopilot_task::Model,
        event_bus: &Arc<EventBus>,
        period_gate: Option<&PeriodGate>,
    ) -> Result<String, AppError>;
}
