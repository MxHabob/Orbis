use async_trait::async_trait;
use orbis_app::ports::repositories::WorkflowRepository;
use orbis_domain::entities::workflow::{
    WorkflowAction as DomainWorkflowAction, WorkflowExecution as DomainWorkflowExecution,
    WorkflowRule as DomainWorkflowRule,
};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::workflow::{workflow_action, workflow_execution, workflow_rule};

pub struct WorkflowRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl WorkflowRepository for WorkflowRepoImpl {
    async fn find_rules_by_trigger(
        &self,
        org_id: Uuid,
        trigger_event: &str,
    ) -> Result<Vec<DomainWorkflowRule>, DomainError> {
        let models = workflow_rule::Entity::find()
            .filter(workflow_rule::Column::OrgId.eq(org_id))
            .filter(workflow_rule::Column::TriggerEvent.eq(trigger_event))
            .filter(workflow_rule::Column::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| DomainWorkflowRule {
                id: m.id,
                org_id: m.org_id,
                name: m.name,
                description: m.description,
                trigger_event: m.trigger_event,
                conditions: m.conditions.map(|v| v.into()),
                is_active: m.is_active,
                created_at: m.created_at.into(),
                updated_at: m.updated_at.into(),
            })
            .collect())
    }

    async fn find_actions_by_rule(
        &self,
        rule_id: Uuid,
    ) -> Result<Vec<DomainWorkflowAction>, DomainError> {
        let models = workflow_action::Entity::find()
            .filter(workflow_action::Column::RuleId.eq(rule_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| DomainWorkflowAction {
                id: m.id,
                rule_id: m.rule_id,
                action_type: m.action_type,
                config: m.config.into(),
                order_index: m.order_index,
                created_at: m.created_at.into(),
            })
            .collect())
    }

    async fn save_execution(&self, execution: &DomainWorkflowExecution) -> Result<(), DomainError> {
        let active = workflow_execution::ActiveModel {
            id: Set(execution.id),
            rule_id: Set(execution.rule_id),
            entity_id: Set(execution.entity_id),
            status: Set(execution.status.clone()),
            error_msg: Set(execution.error_msg.clone()),
            executed_at: Set(execution.executed_at.into()),
        };

        if workflow_execution::Entity::find_by_id(execution.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
