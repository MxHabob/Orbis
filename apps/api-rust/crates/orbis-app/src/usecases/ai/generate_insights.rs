use crate::ports::gateways::LlmGatewayPort;
use crate::ports::repositories::AiRepository;
use crate::context::RequestContext;
use orbis_domain::entities::ai::AiInsight;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct GenerateInsightsUseCase {
    pub ai_repo: Arc<dyn AiRepository>,
    pub llm_gateway: Arc<dyn LlmGatewayPort>,
}

impl GenerateInsightsUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        agent_id: Uuid,
        entity_type: String,
        entity_id: Uuid,
        insight_type: String,
        context_data: String,
    ) -> Result<AiInsight, DomainError> {
        // 1. Fetch Agent
        let agent = self.ai_repo.find_agent_by_id(agent_id).await?
            .ok_or_else(|| DomainError::NotFound("AI Agent not found".into()))?;

        if !agent.is_active {
            return Err(DomainError::Validation("Agent is not active".into()));
        }

        // 2. Generate Insight via LLM
        let user_prompt = format!("Analyze the following data for entity {} ({}):\n{}", entity_type, entity_id, context_data);
        
        let generated_text = self.llm_gateway.generate_text(
            _ctx,
            &agent.model_name,
            &agent.system_prompt,
            &user_prompt,
            agent.temperature,
        ).await?;

        // 3. Save Insight
        let insight = AiInsight {
            id: Uuid::new_v4(),
            org_id: _ctx.tenant_id,
            agent_id,
            entity_type,
            entity_id,
            insight_type,
            content: generated_text,
            confidence_score: 0.9, // Default/Mock score for now
            created_at: Utc::now(),
        };

        self.ai_repo.save_insight(&insight).await?;

        Ok(insight)
    }
}
