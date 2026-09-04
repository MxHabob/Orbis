use crate::ports::repositories::WorkflowRepository;
use crate::context::RequestContext;
use orbis_domain::events::DomainEvent;
use orbis_domain::entities::workflow::WorkflowExecution;
use orbis_domain::errors::DomainError;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct WorkflowEngine {
    pub workflow_repo: Arc<dyn WorkflowRepository>,
    // In a real system we'd have an ActionExecutor interface for the various integrations
}

impl WorkflowEngine {
    pub async fn handle_event(&self, ctx: &RequestContext, event: &DomainEvent) -> Result<(), DomainError> {
        let trigger_name = match event {
            DomainEvent::UserCreated { .. } => "UserCreated",
            DomainEvent::StudentFeeGenerated { .. } => "StudentFeeGenerated",
            DomainEvent::PaymentReceived { .. } => "PaymentReceived",
            DomainEvent::DocumentUploaded { .. } => "DocumentUploaded",
            _ => return Ok(()), // Ignore unhandled events
        };

        // Fetch rules for this trigger
        let rules = self.workflow_repo.find_rules_by_trigger(ctx.tenant_id, trigger_name).await?;

        for rule in rules {
            // Find actions for the rule
            let actions = self.workflow_repo.find_actions_by_rule(rule.id).await?;

            for action in actions {
                // Execute action (Mocked for now)
                println!("Executing Workflow Action: {} for Rule: {}", action.action_type, rule.name);

                let execution = WorkflowExecution {
                    id: Uuid::new_v4(),
                    rule_id: rule.id,
                    entity_id: Uuid::nil(), // We'd map this properly from event payload
                    status: "success".to_string(),
                    error_msg: None,
                    executed_at: Utc::now(),
                };

                self.workflow_repo.save_execution(&execution).await?;
            }
        }

        Ok(())
    }
}
