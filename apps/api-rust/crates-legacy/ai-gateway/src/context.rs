use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The context of the user's current session in the Frontend.
/// This allows the AI agent to know what page the user is currently looking at
/// to provide highly contextual and relevant assistance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentContext {
    /// The domain or module the user is in (e.g. academic, finance, exam).
    pub domain: String,
    
    /// The specific page or view name (e.g. StudentProfile, GradeEntry).
    pub page_name: String,
    
    /// The ID of the specific record the user is viewing, if any (e.g. a student ID).
    pub record_id: Option<String>,
    
    /// Any extra contextual metadata (e.g. current academic period, selected filters).
    pub metadata: Option<Value>,
}

impl AgentContext {
    /// Formats the context into a human-readable prompt string for the LLM.
    pub fn to_prompt_string(&self) -> String {
        let mut prompt = format!(
            "معلومات السياق الحالية:\n- الوحدة: {}\n- الصفحة الحالية: {}",
            self.domain, self.page_name
        );
        
        if let Some(ref id) = self.record_id {
            prompt.push_str(&format!("\n- المعرف النشط (Record ID): {}", id));
        }
        
        if let Some(ref meta) = self.metadata {
            if let Ok(meta_str) = serde_json::to_string_pretty(meta) {
                prompt.push_str(&format!("\n- تفاصيل إضافية:\n{}", meta_str));
            }
        }
        
        prompt
    }
}
