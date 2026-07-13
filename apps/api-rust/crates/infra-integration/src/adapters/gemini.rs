use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::traits::{
    ChatMessage, FinishReason, InferenceConfig, LlmError, LlmProvider, LlmResponse, MessageRole,
    TokenUsage, ToolCall, ToolDef,
};

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GeminiTool>>,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_call: Option<GeminiFunctionCall>,
}

#[derive(Serialize)]
struct GeminiFunctionCall {
    name: String,
    args: serde_json::Value,
}

#[derive(Serialize)]
struct GeminiTool {
    function_declarations: Vec<GeminiFunctionDeclaration>,
}

#[derive(Serialize)]
struct GeminiFunctionDeclaration {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponseData {
    candidates: Option<Vec<Candidate>>,
    usage_metadata: Option<GeminiUsage>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    content: Option<CandidateContent>,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Option<Vec<CandidatePart>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CandidatePart {
    text: Option<String>,
    function_call: Option<GeminiFunctionCallResponse>,
}

#[derive(Deserialize)]
struct GeminiFunctionCallResponse {
    name: String,
    args: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiUsage {
    prompt_token_count: Option<u32>,
    candidates_token_count: Option<u32>,
    total_token_count: Option<u32>,
}

pub struct GeminiAdapter {
    api_key: String,
    client: reqwest::Client,
}

impl GeminiAdapter {
    pub fn new(api_key: String) -> Self {
        let req_client = reqwest::Client::new();
        
        if api_key.is_empty() {
            warn!("GEMINI_API_KEY is not set. GeminiAdapter will return mock responses.");
        }
        Self {
            api_key,
            client: req_client,
        }
    }
}

#[async_trait]
impl LlmProvider for GeminiAdapter {
    fn provider_name(&self) -> &str {
        "gemini"
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_vision(&self) -> bool {
        false // Keep false for now
    }

    async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: Option<&[ToolDef]>,
        _config: &InferenceConfig,
    ) -> Result<LlmResponse, LlmError> {
        if self.api_key.is_empty() {
            // Return mock response
            info!("GEMINI_API_KEY is missing, returning mock LLM response");
            return Ok(LlmResponse {
                content: "هذا رد تجريبي من Gemini Adapter لأن مفتاح الـ API غير متوفر.".to_string(),
                tool_calls: vec![],
                usage: TokenUsage::default(),
                finish_reason: FinishReason::Stop,
            });
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro:generateContent?key={}",
            self.api_key
        );

        let mut contents = Vec::new();
        for msg in messages {
            let role = match msg.role {
                MessageRole::System => "user".to_string(), // Gemini handles system differently in v1beta, we treat as user for simple cases
                MessageRole::User => "user".to_string(),
                MessageRole::Assistant => "model".to_string(),
                MessageRole::Tool => "user".to_string(), // Tools results go here
            };

            contents.push(Content {
                role,
                parts: vec![Part {
                    text: Some(msg.content.clone()),
                    function_call: None,
                }],
            });
        }

        let gemini_tools = tools.map(|ts| {
            vec![GeminiTool {
                function_declarations: ts
                    .iter()
                    .map(|t| GeminiFunctionDeclaration {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    })
                    .collect(),
            }]
        });

        let req_body = GeminiRequest {
            contents,
            tools: gemini_tools,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&req_body)
            .send()
            .await
            .map_err(|e| LlmError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            error!("Gemini HTTP {} error: {}", status, text);
            return Err(LlmError::RequestFailed(text));
        }

        let data: GeminiResponseData = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        let candidate = data
            .candidates
            .and_then(|mut c| c.pop())
            .ok_or_else(|| LlmError::ParseError("No candidates in response".to_string()))?;

        let mut content_text = String::new();
        let mut tool_calls = Vec::new();

        if let Some(content) = candidate.content {
            if let Some(parts) = content.parts {
                for part in parts {
                    if let Some(text) = part.text {
                        content_text.push_str(&text);
                    }
                    if let Some(fc) = part.function_call {
                        tool_calls.push(ToolCall {
                            id: uuid::Uuid::new_v4().to_string(),
                            name: fc.name,
                            arguments: fc.args,
                        });
                    }
                }
            }
        }

        let finish_reason = match candidate.finish_reason.as_deref() {
            Some("STOP") => FinishReason::Stop,
            Some("MAX_TOKENS") => FinishReason::Length,
            Some(_) if !tool_calls.is_empty() => FinishReason::ToolCalls,
            _ => FinishReason::Stop,
        };

        let usage = data
            .usage_metadata
            .map(|u| TokenUsage {
                prompt_tokens: u.prompt_token_count.unwrap_or(0),
                completion_tokens: u.candidates_token_count.unwrap_or(0),
                total_tokens: u.total_token_count.unwrap_or(0),
            })
            .unwrap_or_default();

        Ok(LlmResponse {
            content: content_text,
            tool_calls,
            usage,
            finish_reason,
        })
    }

    async fn embed(&self, _texts: &[&str]) -> Result<Vec<Vec<f32>>, LlmError> {
        Err(LlmError::Unavailable("Embeddings not implemented for Gemini yet".into()))
    }

    async fn health_check(&self) -> Result<bool, LlmError> {
        Ok(!self.api_key.is_empty())
    }
}
