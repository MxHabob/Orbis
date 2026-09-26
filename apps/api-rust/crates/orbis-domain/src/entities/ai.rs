use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiToolCall {
    pub id: String,
    pub tool_name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiToolResult {
    pub tool_call_id: String,
    pub tool_name: String,
    pub content: String,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String, // "user" | "assistant" | "system" | "tool"
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<AiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
    pub required_permission: Option<String>,
    pub is_consequential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProposedAction {
    pub tool_name: String,
    pub title: String,
    pub description: String,
    pub payload: serde_json::Value,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatRequest {
    pub messages: Vec<AiMessage>,
    pub screen_context: Option<serde_json::Value>,
    pub agent_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatResponse {
    pub response: String,
    pub executed_tools: Vec<AiToolResult>,
    pub proposed_actions: Vec<AiProposedAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LlmChatOutput {
    pub content: Option<String>,
    pub tool_calls: Vec<AiToolCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStoredMessage {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_tools: Option<Vec<AiToolResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_actions: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStoredSession {
    pub id: String,
    pub title: String,
    pub org_id: String,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    pub messages: Vec<AiStoredMessage>,
}
