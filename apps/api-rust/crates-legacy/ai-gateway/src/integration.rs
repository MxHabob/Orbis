use std::sync::Arc;
use infra_integration::traits::{LlmProvider, ChatMessage, MessageRole, InferenceConfig, LlmResponse};
use shared_kernel::errors::AppError;
use tracing::info;
use std::collections::HashMap;

pub struct PromptTemplates {
    templates: HashMap<String, &'static str>,
}

impl PromptTemplates {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        // Add predefined templates
        templates.insert(
            "student_behavior_analysis".to_string(),
            "Analyze the following student behavior logs and provide a structured summary including risk factors, support recommendations, and an overall risk category (low, medium, high).\nContext:\n{context}",
        );
        templates.insert(
            "autopilot_decision_making".to_string(),
            "Given the academic calendar event '{event_type}' and period context, determine if this event can be automatically executed or requires human approval. Context:\n{context}",
        );
        templates.insert(
            "data_quality_anomalies".to_string(),
            "Analyze the dataset for anomalies or quality issues. Highlight fields with inconsistencies. Data:\n{context}",
        );
        Self { templates }
    }

    pub fn get(&self, key: &str) -> Option<&&'static str> {
        self.templates.get(key)
    }

    pub fn format(&self, key: &str, context: &str) -> Result<String, AppError> {
        let template = self.get(key)
            .ok_or_else(|| AppError::not_found("prompt_template", key))?;
        Ok(template.replace("{context}", context))
    }
}

pub struct AiIntegrationService {
    pub provider: Arc<dyn LlmProvider>,
    pub templates: PromptTemplates,
}

impl AiIntegrationService {
    pub fn new(provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            provider,
            templates: PromptTemplates::new(),
        }
    }

    /// Run chat completion with automatic token usage tracking and logging
    pub async fn chat(
        &self,
        system_prompt: Option<&str>,
        user_message: &str,
        config: Option<&InferenceConfig>,
    ) -> Result<LlmResponse, AppError> {
        let default_config = InferenceConfig::default();
        let config = config.unwrap_or(&default_config);

        let mut messages = Vec::new();
        if let Some(sys) = system_prompt {
            messages.push(ChatMessage {
                role: MessageRole::System,
                content: sys.to_string(),
                tool_call_id: None,
            });
        }
        messages.push(ChatMessage {
            role: MessageRole::User,
            content: user_message.to_string(),
            tool_call_id: None,
        });

        // Call the LLM provider
        let response = self.provider.chat(&messages, None, config).await
            .map_err(|e| AppError::internal(format!("LLM invocation failed: {}", e)))?;

        // Track and log token usage
        info!(
            provider = %self.provider.provider_name(),
            prompt_tokens = %response.usage.prompt_tokens,
            completion_tokens = %response.usage.completion_tokens,
            total_tokens = %response.usage.total_tokens,
            "AI Integration Service LLM Call Completed"
        );

        Ok(response)
    }

    /// Formats a prompt using a registered template and calls the LLM
    pub async fn generate_from_template(
        &self,
        template_key: &str,
        context: &str,
        system_prompt: Option<&str>,
        config: Option<&InferenceConfig>,
    ) -> Result<LlmResponse, AppError> {
        let user_message = self.templates.format(template_key, context)?;
        self.chat(system_prompt, &user_message, config).await
    }
}
