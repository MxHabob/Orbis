use async_trait::async_trait;
use orbis_app::ports::repositories::AiRepository;
use orbis_domain::entities::ai::{AiAgent as DomainAiAgent, AiInsight as DomainAiInsight};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::ai::{ai_agent, ai_insight};

pub struct AiRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl AiRepository for AiRepoImpl {
    async fn find_agent_by_id(&self, id: Uuid) -> Result<Option<DomainAiAgent>, DomainError> {
        let model = ai_agent::Entity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainAiAgent {
            id: m.id, org_id: m.org_id, name: m.name, description: m.description,
            model_name: m.model_name, temperature: m.temperature, system_prompt: m.system_prompt,
            is_active: m.is_active, created_at: m.created_at.into(), updated_at: m.updated_at.into()
        }))
    }

    async fn find_active_agents(&self, org_id: Uuid) -> Result<Vec<DomainAiAgent>, DomainError> {
        let models = ai_agent::Entity::find()
            .filter(ai_agent::Column::OrgId.eq(org_id))
            .filter(ai_agent::Column::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(|m| DomainAiAgent {
            id: m.id, org_id: m.org_id, name: m.name, description: m.description,
            model_name: m.model_name, temperature: m.temperature, system_prompt: m.system_prompt,
            is_active: m.is_active, created_at: m.created_at.into(), updated_at: m.updated_at.into()
        }).collect())
    }

    async fn save_insight(&self, insight: &DomainAiInsight) -> Result<(), DomainError> {
        let active = ai_insight::ActiveModel {
            id: Set(insight.id), org_id: Set(insight.org_id), agent_id: Set(insight.agent_id),
            entity_type: Set(insight.entity_type.clone()), entity_id: Set(insight.entity_id),
            insight_type: Set(insight.insight_type.clone()), content: Set(insight.content.clone()),
            confidence_score: Set(insight.confidence_score), created_at: Set(insight.created_at.into()),
        };

        if ai_insight::Entity::find_by_id(insight.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
