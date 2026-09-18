use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::LlmGatewayPort;
use orbis_domain::entities::ai::{AiMessage, AiToolCall, AiToolDefinition, LlmChatOutput};
use orbis_domain::errors::DomainError;
use reqwest::Client;
use serde_json::json;

pub struct GeminiAdapter {
    pub api_key: String,
    pub client: Client,
}

impl GeminiAdapter {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmGatewayPort for GeminiAdapter {
    async fn generate_text(
        &self,
        _ctx: &RequestContext,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f64,
    ) -> Result<String, DomainError> {
        const CURRENT_FLASH_MODEL: &str = "gemini-2.5-flash";
        let mut selected_model = if model.trim().is_empty() {
            CURRENT_FLASH_MODEL.to_string()
        } else {
            model.to_string()
        };

        let prompt_text = format!("{}\n\n{}", system_prompt, user_prompt);
        let payload = json!({
            "contents": [{
                "parts": [{"text": prompt_text}]
            }],
            "generationConfig": {"temperature": temperature.clamp(0.0, 2.0)}
        });

        for attempt in 0..2 {
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                selected_model
            );
            let res = self
                .client
                .post(&url)
                .header("x-goog-api-key", &self.api_key)
                .json(&payload)
                .send()
                .await
                .map_err(|e| DomainError::Validation(format!("Gemini API Network Error: {}", e)))?;

            if !res.status().is_success() {
                let status = res.status();
                let error_text = res.text().await.unwrap_or_default();
                let is_retired_model =
                    status.as_u16() == 404 && error_text.contains("no longer available");
                if attempt == 0 && is_retired_model && selected_model != CURRENT_FLASH_MODEL {
                    selected_model = CURRENT_FLASH_MODEL.to_string();
                    continue;
                }
                return Err(DomainError::Validation(format!("Gemini API Error: {}", error_text)));
            }

            let body: serde_json::Value = res
                .json()
                .await
                .map_err(|e| DomainError::Validation(format!("Invalid JSON from Gemini: {}", e)))?;

            let generated_text = body["candidates"][0]["content"]["parts"][0]["text"]
                .as_str()
                .unwrap_or("No response generated")
                .to_string();

            return Ok(generated_text);
        }

        Err(DomainError::Validation("Gemini model retry failed".into()))
    }

    async fn chat_with_tools(
        &self,
        _ctx: &RequestContext,
        model: &str,
        system_prompt: &str,
        messages: &[AiMessage],
        tools: &[AiToolDefinition],
        temperature: f64,
    ) -> Result<LlmChatOutput, DomainError> {
        const DEFAULT_MODEL: &str = "gemini-2.5-flash";
        let selected_model = if model.trim().is_empty() {
            DEFAULT_MODEL
        } else {
            model
        };

        // Build contents for Gemini API
        let mut contents = Vec::new();
        for msg in messages {
            let role = match msg.role.as_str() {
                "assistant" => "model",
                "tool" => "user",
                _ => "user",
            };

            let mut parts = Vec::new();
            if !msg.content.is_empty() {
                parts.push(json!({"text": msg.content}));
            }
            if let Some(tool_calls) = &msg.tool_calls {
                for tc in tool_calls {
                    parts.push(json!({
                        "functionCall": {
                            "name": tc.tool_name,
                            "args": tc.arguments
                        }
                    }));
                }
            }

            if !parts.is_empty() {
                contents.push(json!({
                    "role": role,
                    "parts": parts
                }));
            }
        }

        let mut payload = json!({
            "contents": contents,
            "generationConfig": {
                "temperature": temperature.clamp(0.0, 2.0)
            }
        });

        if !system_prompt.trim().is_empty() {
            payload["systemInstruction"] = json!({
                "parts": [{"text": system_prompt}]
            });
        }

        if !tools.is_empty() {
            let function_declarations: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters_schema
                    })
                })
                .collect();

            payload["tools"] = json!([{
                "functionDeclarations": function_declarations
            }]);
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            selected_model
        );

        let res = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Gemini API Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!("Gemini API Error: {}", error_text)));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Gemini: {}", e)))?;

        let candidate = &body["candidates"][0]["content"];
        let parts = candidate["parts"].as_array();

        let mut content_parts = Vec::new();
        let mut tool_calls = Vec::new();

        if let Some(parts_arr) = parts {
            for (idx, part) in parts_arr.iter().enumerate() {
                if let Some(text) = part["text"].as_str() {
                    content_parts.push(text);
                }
                if let Some(fc) = part.get("functionCall") {
                    let tool_name = fc["name"].as_str().unwrap_or_default().to_string();
                    let arguments = fc.get("args").cloned().unwrap_or(json!({}));
                    tool_calls.push(AiToolCall {
                        id: format!("call_gemini_{}_{}", tool_name, idx),
                        tool_name,
                        arguments,
                    });
                }
            }
        }

        let content = if content_parts.is_empty() {
            None
        } else {
            Some(content_parts.join("\n"))
        };

        Ok(LlmChatOutput {
            content,
            tool_calls,
        })
    }
}
