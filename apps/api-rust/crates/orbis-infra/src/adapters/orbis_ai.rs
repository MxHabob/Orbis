use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::LlmGatewayPort;
use orbis_domain::entities::ai::{AiMessage, AiToolCall, AiToolDefinition, LlmChatOutput};
use orbis_domain::errors::DomainError;
use reqwest::Client;
use serde_json::json;

/// Universal OpenAI-compatible LLM adapter.
/// Supports OpenAI, Azure OpenAI, DeepSeek, Groq, Mistral, Ollama, vLLM, and LM Studio.
pub struct OrbisAiAdapter {
    pub endpoint_url: String,
    pub api_key: Option<String>,
    pub default_model: String,
    pub client: Client,
}

impl OrbisAiAdapter {
    pub fn new(endpoint_url: String) -> Self {
        Self {
            endpoint_url,
            api_key: None,
            default_model: "llama3.2".to_string(),
            client: Client::new(),
        }
    }

    pub fn with_api_key(endpoint_url: String, api_key: Option<String>) -> Self {
        Self {
            endpoint_url,
            api_key,
            default_model: "gpt-4o".to_string(),
            client: Client::new(),
        }
    }

    pub fn with_config(
        endpoint_url: String,
        api_key: Option<String>,
        default_model: String,
    ) -> Self {
        Self {
            endpoint_url,
            api_key,
            default_model,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmGatewayPort for OrbisAiAdapter {
    async fn generate_text(
        &self,
        _ctx: &RequestContext,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f64,
    ) -> Result<String, DomainError> {
        let endpoint = format!("{}/v1/chat/completions", self.endpoint_url.trim_end_matches('/'));
        let selected_model = if model.trim().is_empty() {
            &self.default_model
        } else {
            model
        };

        let payload = json!({
            "model": selected_model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
            ],
            "temperature": temperature
        });

        let mut req = self.client.post(&endpoint).json(&payload);
        if let Some(key) = &self.api_key {
            if !key.trim().is_empty() {
                req = req.bearer_auth(key);
            }
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("AI Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!("AI Provider API Error: {}", error_text)));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from AI Provider: {}", e)))?;

        let generated_text = body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response generated")
            .to_string();

        Ok(generated_text)
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
        let endpoint = format!("{}/v1/chat/completions", self.endpoint_url.trim_end_matches('/'));
        let selected_model = if model.trim().is_empty() {
            &self.default_model
        } else {
            model
        };

        // Construct standard OpenAI messages array
        let mut messages_payload = Vec::new();
        if !system_prompt.trim().is_empty() {
            messages_payload.push(json!({
                "role": "system",
                "content": system_prompt
            }));
        }

        for msg in messages {
            if msg.role == "tool" {
                messages_payload.push(json!({
                    "role": "tool",
                    "tool_call_id": msg.tool_call_id.clone().unwrap_or_default(),
                    "content": msg.content
                }));
            } else if let Some(tool_calls) = &msg.tool_calls {
                let tc_json: Vec<serde_json::Value> = tool_calls
                    .iter()
                    .map(|tc| {
                        json!({
                            "id": tc.id,
                            "type": "function",
                            "function": {
                                "name": tc.tool_name,
                                "arguments": tc.arguments.to_string()
                            }
                        })
                    })
                    .collect();

                messages_payload.push(json!({
                    "role": msg.role,
                    "content": if msg.content.is_empty() { serde_json::Value::Null } else { json!(msg.content) },
                    "tool_calls": tc_json
                }));
            } else {
                messages_payload.push(json!({
                    "role": msg.role,
                    "content": msg.content
                }));
            }
        }

        let mut payload = json!({
            "model": selected_model,
            "messages": messages_payload,
            "temperature": temperature
        });

        // Add tools schema if tools are available
        if !tools.is_empty() {
            let tools_json: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters_schema
                        }
                    })
                })
                .collect();
            payload["tools"] = json!(tools_json);
            payload["tool_choice"] = json!("auto");
        }

        let mut req = self.client.post(&endpoint).json(&payload);
        if let Some(key) = &self.api_key {
            if !key.trim().is_empty() {
                req = req.bearer_auth(key);
            }
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("AI Network Error: {}", e)))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!("AI Provider API Error: {}", error_text)));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from AI Provider: {}", e)))?;

        let choice = &body["choices"][0]["message"];
        let content = choice["content"].as_str().map(|s| s.to_string());

        let mut parsed_tool_calls = Vec::new();
        if let Some(tool_calls_arr) = choice["tool_calls"].as_array() {
            for tc in tool_calls_arr {
                let id = tc["id"].as_str().unwrap_or_default().to_string();
                let tool_name = tc["function"]["name"].as_str().unwrap_or_default().to_string();
                let args_str = tc["function"]["arguments"].as_str().unwrap_or("{}");
                let arguments: serde_json::Value = serde_json::from_str(args_str).unwrap_or(json!({}));

                parsed_tool_calls.push(AiToolCall {
                    id,
                    tool_name,
                    arguments,
                });
            }
        }

        Ok(LlmChatOutput {
            content,
            tool_calls: parsed_tool_calls,
        })
    }
}
