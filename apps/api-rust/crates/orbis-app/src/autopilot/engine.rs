use std::sync::Arc;
use orbis_domain::autopilot::task::{AutopilotTask, TaskType};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::PolicyRepository;

pub struct AutopilotEngine {
    pub policy_repo: Arc<dyn PolicyRepository>,
    // In a real scenario, it would also hold an AutopilotTaskRepository 
    // and all the required UseCases (e.g., ProcessPaymentUseCase).
}

impl AutopilotEngine {
    pub fn new(policy_repo: Arc<dyn PolicyRepository>) -> Self {
        Self { policy_repo }
    }

    /// Entry point for processing a scheduled task.
    pub async fn process_task(&self, ctx: &RequestContext, task: AutopilotTask) -> Result<(), DomainError> {
        let settings = self.policy_repo.get_settings(task.org_id).await?;

        if !settings.autopilot.enabled {
            println!("Autopilot is disabled for org {}", task.org_id);
            return Ok(());
        }

        if settings.autopilot.mode == "manual" {
            println!("Autopilot in manual mode for org {}, keeping task {} as suggested", task.org_id, task.id);
            // Here we would update task status to "Suggested" in DB.
            return Ok(());
        }

        println!("Executing Autopilot Task: {:?} for org {}", task.task_type, task.org_id);
        
        match task.task_type {
            TaskType::SendDelayNotification => {
                crate::autopilot::handlers::handle_late_fee::LateFeeHandler::handle(ctx, &task).await?;
            }
            TaskType::TransitionPeriod => {
                // Call academic period handler (e.g., lock grade entry)
                println!("Autopilot: Period transitioned.");
            }
            TaskType::GenericWorkflow => {
                // Call Workflow Engine
                println!("Autopilot: Workflow triggered.");
            }
        }

        // Update task status to Completed in DB.
        Ok(())
    }

    /// Evaluates domain events on-the-fly to decide if an immediate task should be scheduled or executed.
    pub async fn handle_event(&self, _ctx: &RequestContext, event: &DomainEvent) -> Result<(), DomainError> {
        // e.g. DomainEvent::StudentFeeGenerated -> Schedule a TaskType::SendDelayNotification if unpaid by due_date
        println!("Autopilot Engine analyzed event: {:?}", event);
        Ok(())
    }
}
