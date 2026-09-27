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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCategory {
    Read,
    Analysis,
    Generation,
    Action,
}

impl Default for ToolCategory {
    fn default() -> Self {
        ToolCategory::Read
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingAssignmentInfo {
    pub class_subject_id: uuid::Uuid,
    pub class_id: uuid::Uuid,
    pub class_name: String,
    pub subject_id: uuid::Uuid,
    pub subject_name: String,
    pub period_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAiContext {
    pub user_id: Option<uuid::Uuid>,
    pub user_name: Option<String>,
    pub effective_role: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub org_id: uuid::Uuid,
    pub org_name: Option<String>,
    pub org_unit_id: Option<uuid::Uuid>,
    pub org_unit_path: Option<String>,
    pub employee_id: Option<uuid::Uuid>,
    pub is_teacher: bool,
    pub teaching_assignments: Vec<TeachingAssignmentInfo>,
    pub is_parent: bool,
    pub dependent_student_ids: Vec<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcademicAiContext {
    pub academic_year_id: Option<uuid::Uuid>,
    pub academic_year_name: Option<String>,
    pub period_id: Option<uuid::Uuid>,
    pub period_name: Option<String>,
    pub period_state: Option<String>,
    pub class_id: Option<uuid::Uuid>,
    pub class_name: Option<String>,
    pub subject_id: Option<uuid::Uuid>,
    pub subject_name: Option<String>,
    pub class_subject_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiContext {
    pub route: Option<String>,
    pub screen: Option<String>,
    pub domain: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<uuid::Uuid>,
    #[serde(default)]
    pub selected_entity_ids: Vec<uuid::Uuid>,
    pub active_filters: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    // Academic hierarchy from current UI view
    pub period_id: Option<uuid::Uuid>,
    pub class_id: Option<uuid::Uuid>,
    pub subject_id: Option<uuid::Uuid>,
    pub class_subject_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
    pub required_permission: Option<String>,
    pub is_consequential: bool,
    #[serde(default)]
    pub category: ToolCategory,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ui_context: Option<UiContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub academic_context: Option<AcademicAiContext>,
    pub agent_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatResponse {
    pub response: String,
    pub executed_tools: Vec<AiToolResult>,
    pub proposed_actions: Vec<AiProposedAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_user_context: Option<UserAiContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_academic_context: Option<AcademicAiContext>,
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
