use std::sync::Arc;
use serde_json::Value;
use uuid::Uuid;
use infra_integration::traits::{LlmProvider, ChatMessage, MessageRole, InferenceConfig, ToolDef};
use shared_kernel::errors::AppError;
use crate::action_registry::ActionRegistry;
use crate::action_executor::{ActionExecutor, ActionExecutionResult};
use crate::entities::ai_agent_session;
use tracing::info;

/// Unified Gateway for all AI interactions in Orbis.
/// This file acts as the general external AI integration layer for all agents.
pub struct LlmGateway {
    provider: Box<dyn LlmProvider>,
    registry: Arc<ActionRegistry>,
    executor: ActionExecutor,
}

impl LlmGateway {
    /// Initialize the gateway with a specific provider and the global action registry.
    pub fn new(provider: Box<dyn LlmProvider>, registry: Arc<ActionRegistry>, executor: ActionExecutor) -> Self {
        Self { provider, registry, executor }
    }

    /// Dispatch a query for any agent type.
    pub async fn dispatch_agent_query(
        &self,
        session: &ai_agent_session::Model,
        user_query: &str,
        context: Option<Value>,
    ) -> Result<String, AppError> {
        let agent_type = &session.agent_type;
        let org_id = &session.org_id;
        let session_id = &session.id;

        info!(
            agent_type,
            session_id,
            org_id,
            "Dispatching generic agent query through LLM Gateway"
        );

        // 1. Build the System Prompt dynamically based on the agent type
        let mut system_msg = self.build_system_prompt(agent_type, org_id);

        if let Some(ctx_val) = context {
            if let Ok(agent_ctx) = serde_json::from_value::<crate::context::AgentContext>(ctx_val) {
                system_msg.push_str("\n\n");
                system_msg.push_str(&agent_ctx.to_prompt_string());
            }
        }

        // 2. Resolve available tools for this specific agent domain
        let tools = self.resolve_agent_tools(agent_type);

        // 3. Construct the message history
        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: system_msg,
                tool_call_id: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: user_query.to_string(),
                tool_call_id: None,
            },
        ];

        // 4. Configure inference parameters
        let config = InferenceConfig::default();

        // 5. Call the external AI provider
        let mut response = self
            .provider
            .chat(&messages, Some(&tools), &config)
            .await
            .map_err(|e| AppError::internal(&e.to_string()))?;

        // 6. Handle Tool Calls
        if !response.tool_calls.is_empty() {
            info!("LLM requested {} tool calls", response.tool_calls.len());
            let mut final_content = response.content.clone();
            
            for tool_call in response.tool_calls {
                info!("Executing tool: {}", tool_call.name);
                
                // Execute using ActionExecutor
                let exec_result = self.executor.execute(
                    session,
                    &tool_call.name,
                    tool_call.arguments.clone(),
                    "LLM requested execution during chat",
                ).await?;
                
                let result_summary = match exec_result {
                    ActionExecutionResult::Executed { output, duration_ms } => {
                        format!("✅ تم التنفيذ بنجاح في {}ms. النتيجة: {}", duration_ms, output)
                    },
                    ActionExecutionResult::PendingApproval { approval_id } => {
                        format!("⏳ الإجراء يتطلب موافقة يدوية من المسؤول. رقم الطلب: {}", approval_id)
                    },
                    ActionExecutionResult::RiskExceeded { action_risk, session_max } => {
                        format!("❌ تم الرفض: مستوى الخطر للإجراء ({}) يتجاوز الحد المسموح للجلسة ({}).", action_risk, session_max)
                    },
                    ActionExecutionResult::AlreadyExecuted => {
                        "⚠️ تم تنفيذ هذا الإجراء مسبقاً.".to_string()
                    }
                };

                // Append the result summary to the response content so the user knows what happened.
                final_content.push_str(&format!("\n\n[نتيجة الأداة {}: {}]", tool_call.name, result_summary));
            }
            
            return Ok(final_content);
        }

        Ok(response.content)
    }

    /// Build a custom system prompt depending on the agent's persona.
    fn build_system_prompt(&self, agent_type: &str, org_id: &str) -> String {
        match agent_type {
            "academic" => format!(
                "أنت وكيل الشؤون الأكاديمية في نظام Orbis (مؤسسة {}). دورك هو مساعدة المعلمين والطلاب في إدارة الجداول، الدرجات، والمواد الأكاديمية. أجب بشكل احترافي ومباشر.",
                org_id
            ),
            "finance" => format!(
                "أنت الوكيل المالي في نظام Orbis (مؤسسة {}). دورك هو إدارة الرسوم، الخزينة، والمدفوعات. التزم بالدقة المالية الصارمة في إجاباتك.",
                org_id
            ),
            "exam" => format!(
                "أنت وكيل الاختبارات في نظام Orbis (مؤسسة {}). دورك هو تنظيم قاعات الامتحانات وتوزيع أرقام الجلوس.",
                org_id
            ),
            "admin" | "default" | _ => format!(
                "أنت المساعد الذكي الموحد (Ask Orbis) لمؤسسة {}. دورك هو تلبية طلبات المستخدم وتنفيذ الإجراءات اللازمة بناءً على صلاحياته.",
                org_id
            ),
        }
    }

    /// Retrieve the permitted tools from the registry for this agent type.
    fn resolve_agent_tools(&self, agent_type: &str) -> Vec<ToolDef> {
        // Map agent type to domain names in the ActionRegistry
        let domains = match agent_type {
            "academic" => vec!["academic", "scheduling"],
            "finance" => vec!["finance"],
            "exam" => vec!["exam"],
            _ => vec!["iam", "academic", "assessment", "finance", "exam", "lifecycle"], // All
        };

        let mut tools = Vec::new();

        for domain in domains {
            let actions = self.registry.discover(Some(domain), None);
            for action in actions {
                tools.push(ToolDef {
                    name: action.code.clone(),
                    description: action.description.clone(),
                    parameters: action.input_schema.clone().unwrap_or_else(|| serde_json::json!({})),
                });
            }
        }

        tools
    }
}
