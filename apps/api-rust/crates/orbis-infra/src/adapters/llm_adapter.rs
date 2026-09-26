//! Universal LLM Gateway Adapter (2026 Clean Architecture).
//!
//! Provides a unified, highly optimized, multi-protocol integration for all Large Language Model providers:
//! - **OpenAI Compatible**: OpenAI (GPT-4o, o1, o3, GPT-4.5), DeepSeek, Groq, Mistral, Perplexity, OpenRouter, Together AI, vLLM, Ollama, LM Studio, Azure OpenAI.
//! - **Google Gemini**: Gemini 2.5/3.0/3.8 Flash, Pro, Flash-Lite via native Google Generative Language API.
//! - **Anthropic Claude**: Claude 3.5/3.7 Sonnet, Haiku, Opus via Anthropic Messages API.
//!
//! # Token Efficiency & Precision Principles:
//! 1. **Zero Forced Defaults**: No hardcoded default models are assumed. The integration configuration or request must explicitly supply the model.
//! 2. **Token Optimization**: Compact JSON serialization, prompt whitespace trimming, strict schema pruning, and max output token budgeting.
//! 3. **Robust Resiliency**: Reusable HTTP connection pooling, configurable timeouts, and detailed error mapping into DomainError.

use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::LlmGatewayPort;
use orbis_domain::entities::ai::{AiMessage, AiToolCall, AiToolDefinition, LlmChatOutput};
use orbis_domain::errors::DomainError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

/// Supported LLM API wire protocols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmProviderProtocol {
    /// OpenAI-compatible chat completions (/v1/chat/completions).
    /// Used by OpenAI, DeepSeek, Groq, Mistral, Ollama, LM Studio, vLLM, OpenRouter, etc.
    OpenAiCompatible,
    /// Google Generative AI REST API v1beta (/v1beta/models/{model}:generateContent).
    Gemini,
    /// Anthropic Messages REST API v1 (/v1/messages).
    AnthropicClaude,
}

impl Default for LlmProviderProtocol {
    fn default() -> Self {
        Self::OpenAiCompatible
    }
}

/// Precise configuration for the universal LLM Adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmAdapterConfig {
    /// Wire protocol to use
    pub protocol: LlmProviderProtocol,
    /// Base or full endpoint URL
    pub endpoint_url: String,
    /// API key / authentication token
    pub api_key: Option<String>,
    /// Configured default model for this integration instance (if any)
    pub default_model: Option<String>,
    /// Optional max tokens limit to prevent runaway token expenditure
    pub max_tokens: Option<u32>,
    /// Network timeout in seconds
    pub timeout_secs: u64,
    /// Optional custom HTTP headers (e.g. organization, beta flags, proxy auth)
    pub extra_headers: HashMap<String, String>,
}

impl LlmAdapterConfig {
    /// Create a new configuration for a specific protocol and endpoint.
    pub fn new(protocol: LlmProviderProtocol, endpoint_url: impl Into<String>) -> Self {
        Self {
            protocol,
            endpoint_url: endpoint_url.into(),
            api_key: None,
            default_model: None,
            max_tokens: None,
            timeout_secs: 60,
            extra_headers: HashMap::new(),
        }
    }

    /// Builder: Set API key.
    pub fn with_api_key(mut self, key: Option<String>) -> Self {
        self.api_key = key.filter(|k| !k.trim().is_empty());
        self
    }

    /// Builder: Set default model.
    pub fn with_default_model(mut self, model: Option<String>) -> Self {
        self.default_model = model.filter(|m| !m.trim().is_empty());
        self
    }

    /// Builder: Set max completion tokens.
    pub fn with_max_tokens(mut self, max_tokens: Option<u32>) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Builder: Set network timeout.
    pub fn with_timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs.max(5);
        self
    }

    /// Builder: Add custom header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_headers.insert(key.into(), value.into());
        self
    }
}

/// Unified, high-performance LLM Adapter.
pub struct LlmAdapter {
    config: LlmAdapterConfig,
    client: Client,
}

impl LlmAdapter {
    /// Create a new instance of `LlmAdapter` with connection pooling and timeouts.
    pub fn new(config: LlmAdapterConfig) -> Result<Self, DomainError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .build()
            .map_err(|e| DomainError::Internal(format!("Failed to initialize HTTP client for LLM adapter: {}", e)))?;

        Ok(Self { config, client })
    }

    /// Inspect configuration.
    pub fn config(&self) -> &LlmAdapterConfig {
        &self.config
    }

    /// Resolve model name strictly: no arbitrary fallback or guessed model names.
    fn resolve_model<'a>(&'a self, requested_model: &'a str) -> Result<&'a str, DomainError> {
        let trimmed_req = requested_model.trim();
        if !trimmed_req.is_empty() {
            return Ok(trimmed_req);
        }

        if let Some(ref def) = self.config.default_model {
            let trimmed_def = def.trim();
            if !trimmed_def.is_empty() {
                return Ok(trimmed_def);
            }
        }

        Err(DomainError::Validation(
            "No LLM model was specified in the request, and no default_model is configured in the integration settings."
                .into(),
        ))
    }

    // =========================================================================
    // OPENAI COMPATIBLE PROTOCOL IMPLEMENTATION
    // =========================================================================

    async fn openai_generate_text(
        &self,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f64,
    ) -> Result<String, DomainError> {
        let endpoint = self.build_openai_chat_url();
        let mut messages = Vec::with_capacity(2);

        let sys_trimmed = system_prompt.trim();
        if !sys_trimmed.is_empty() {
            messages.push(json!({
                "role": "system",
                "content": sys_trimmed
            }));
        }

        messages.push(json!({
            "role": "user",
            "content": user_prompt.trim()
        }));

        let mut payload = json!({
            "model": model,
            "messages": messages,
            "temperature": temperature.clamp(0.0, 2.0)
        });

        if let Some(max_tokens) = self.config.max_tokens {
            payload["max_tokens"] = json!(max_tokens);
        }

        let mut req = self.client.post(&endpoint).json(&payload);
        if let Some(ref key) = self.config.api_key {
            req = req.bearer_auth(key);
        }
        for (k, v) in &self.config.extra_headers {
            req = req.header(k, v);
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("LLM Network Error (OpenAI-compatible): {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "LLM API Error [HTTP {}]: {}",
                status, error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON response from LLM provider: {}", e)))?;

        let generated_text = body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string();

        if generated_text.is_empty() {
            return Err(DomainError::Validation("LLM returned empty completion content.".into()));
        }

        Ok(generated_text)
    }

    async fn openai_chat_with_tools(
        &self,
        model: &str,
        system_prompt: &str,
        messages: &[AiMessage],
        tools: &[AiToolDefinition],
        temperature: f64,
    ) -> Result<LlmChatOutput, DomainError> {
        let endpoint = self.build_openai_chat_url();
        let mut messages_payload = Vec::with_capacity(messages.len() + 1);

        let sys_trimmed = system_prompt.trim();
        if !sys_trimmed.is_empty() {
            messages_payload.push(json!({
                "role": "system",
                "content": sys_trimmed
            }));
        }

        for msg in messages {
            if msg.role == "tool" {
                messages_payload.push(json!({
                    "role": "tool",
                    "tool_call_id": msg.tool_call_id.clone().unwrap_or_default(),
                    "content": msg.content.trim()
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

                let content_val = if msg.content.trim().is_empty() {
                    serde_json::Value::Null
                } else {
                    json!(msg.content.trim())
                };

                messages_payload.push(json!({
                    "role": msg.role,
                    "content": content_val,
                    "tool_calls": tc_json
                }));
            } else {
                messages_payload.push(json!({
                    "role": msg.role,
                    "content": msg.content.trim()
                }));
            }
        }

        let mut payload = json!({
            "model": model,
            "messages": messages_payload,
            "temperature": temperature.clamp(0.0, 2.0)
        });

        if let Some(max_tokens) = self.config.max_tokens {
            payload["max_tokens"] = json!(max_tokens);
        }

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
        if let Some(ref key) = self.config.api_key {
            req = req.bearer_auth(key);
        }
        for (k, v) in &self.config.extra_headers {
            req = req.header(k, v);
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("LLM Network Error (OpenAI-compatible): {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "LLM API Error [HTTP {}]: {}",
                status, error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON response from LLM: {}", e)))?;

        let choice = &body["choices"][0]["message"];
        let content = choice["content"].as_str().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

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

    fn build_openai_chat_url(&self) -> String {
        let base = self.config.endpoint_url.trim().trim_end_matches('/');
        if base.ends_with("/chat/completions") {
            base.to_string()
        } else if base.ends_with("/v1") {
            format!("{}/chat/completions", base)
        } else {
            format!("{}/v1/chat/completions", base)
        }
    }

    // =========================================================================
    // GOOGLE GEMINI PROTOCOL IMPLEMENTATION
    // =========================================================================

    async fn gemini_generate_text(
        &self,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f64,
    ) -> Result<String, DomainError> {
        let url = self.build_gemini_url(model);
        let mut payload = json!({
            "contents": [{
                "role": "user",
                "parts": [{"text": user_prompt.trim()}]
            }],
            "generationConfig": {
                "temperature": temperature.clamp(0.0, 2.0)
            }
        });

        if let Some(max_tokens) = self.config.max_tokens {
            payload["generationConfig"]["maxOutputTokens"] = json!(max_tokens);
        }

        let sys_trimmed = system_prompt.trim();
        if !sys_trimmed.is_empty() {
            payload["systemInstruction"] = json!({
                "parts": [{"text": sys_trimmed}]
            });
        }

        let mut req = self.client.post(&url).json(&payload);
        if let Some(ref key) = self.config.api_key {
            req = req.header("x-goog-api-key", key);
        }
        for (k, v) in &self.config.extra_headers {
            req = req.header(k, v);
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Gemini API Network Error: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Gemini API Error [HTTP {}]: {}",
                status, error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Gemini: {}", e)))?;

        let generated_text = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string();

        if generated_text.is_empty() {
            return Err(DomainError::Validation("Gemini returned empty candidate content.".into()));
        }

        Ok(generated_text)
    }

    async fn gemini_chat_with_tools(
        &self,
        model: &str,
        system_prompt: &str,
        messages: &[AiMessage],
        tools: &[AiToolDefinition],
        temperature: f64,
    ) -> Result<LlmChatOutput, DomainError> {
        let url = self.build_gemini_url(model);
        let mut contents = Vec::with_capacity(messages.len());

        for msg in messages {
            let role = match msg.role.as_str() {
                "assistant" => "model",
                "tool" => "user",
                _ => "user",
            };

            let mut parts = Vec::new();

            if msg.role == "tool" {
                let name = msg.tool_call_id.as_deref().unwrap_or("tool");
                parts.push(json!({
                    "functionResponse": {
                        "name": name,
                        "response": {
                            "output": msg.content.trim()
                        }
                    }
                }));
            } else {
                if !msg.content.trim().is_empty() {
                    parts.push(json!({"text": msg.content.trim()}));
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

        if let Some(max_tokens) = self.config.max_tokens {
            payload["generationConfig"]["maxOutputTokens"] = json!(max_tokens);
        }

        let sys_trimmed = system_prompt.trim();
        if !sys_trimmed.is_empty() {
            payload["systemInstruction"] = json!({
                "parts": [{"text": sys_trimmed}]
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

        let mut req = self.client.post(&url).json(&payload);
        if let Some(ref key) = self.config.api_key {
            req = req.header("x-goog-api-key", key);
        }
        for (k, v) in &self.config.extra_headers {
            req = req.header(k, v);
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Gemini API Network Error: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Gemini API Error [HTTP {}]: {}",
                status, error_text
            )));
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
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        content_parts.push(trimmed);
                    }
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

    fn build_gemini_url(&self, model: &str) -> String {
        let base = if self.config.endpoint_url.trim().is_empty() {
            "https://generativelanguage.googleapis.com"
        } else {
            self.config.endpoint_url.trim().trim_end_matches('/')
        };

        format!("{}/v1beta/models/{}:generateContent", base, model)
    }

    // =========================================================================
    // ANTHROPIC CLAUDE PROTOCOL IMPLEMENTATION
    // =========================================================================

    async fn anthropic_generate_text(
        &self,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f64,
    ) -> Result<String, DomainError> {
        let url = self.build_anthropic_url();
        let mut payload = json!({
            "model": model,
            "max_tokens": self.config.max_tokens.unwrap_or(4096),
            "messages": [{
                "role": "user",
                "content": user_prompt.trim()
            }],
            "temperature": temperature.clamp(0.0, 1.0)
        });

        let sys_trimmed = system_prompt.trim();
        if !sys_trimmed.is_empty() {
            payload["system"] = json!(sys_trimmed);
        }

        let mut req = self
            .client
            .post(&url)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&payload);

        if let Some(ref key) = self.config.api_key {
            req = req.header("x-api-key", key);
        }
        for (k, v) in &self.config.extra_headers {
            req = req.header(k, v);
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Anthropic API Network Error: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Anthropic API Error [HTTP {}]: {}",
                status, error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Anthropic: {}", e)))?;

        let mut output_text = String::new();
        if let Some(content_arr) = body["content"].as_array() {
            for block in content_arr {
                if block["type"].as_str() == Some("text") {
                    if let Some(t) = block["text"].as_str() {
                        output_text.push_str(t.trim());
                    }
                }
            }
        }

        if output_text.is_empty() {
            return Err(DomainError::Validation("Anthropic returned empty content.".into()));
        }

        Ok(output_text)
    }

    async fn anthropic_chat_with_tools(
        &self,
        model: &str,
        system_prompt: &str,
        messages: &[AiMessage],
        tools: &[AiToolDefinition],
        temperature: f64,
    ) -> Result<LlmChatOutput, DomainError> {
        let url = self.build_anthropic_url();
        let mut messages_payload = Vec::with_capacity(messages.len());

        for msg in messages {
            if msg.role == "tool" {
                messages_payload.push(json!({
                    "role": "user",
                    "content": [{
                        "type": "tool_result",
                        "tool_use_id": msg.tool_call_id.clone().unwrap_or_default(),
                        "content": msg.content.trim()
                    }]
                }));
            } else if let Some(tool_calls) = &msg.tool_calls {
                let mut blocks = Vec::new();
                if !msg.content.trim().is_empty() {
                    blocks.push(json!({
                        "type": "text",
                        "text": msg.content.trim()
                    }));
                }
                for tc in tool_calls {
                    blocks.push(json!({
                        "type": "tool_use",
                        "id": tc.id,
                        "name": tc.tool_name,
                        "input": tc.arguments
                    }));
                }
                messages_payload.push(json!({
                    "role": "assistant",
                    "content": blocks
                }));
            } else {
                messages_payload.push(json!({
                    "role": if msg.role == "assistant" { "assistant" } else { "user" },
                    "content": msg.content.trim()
                }));
            }
        }

        let mut payload = json!({
            "model": model,
            "max_tokens": self.config.max_tokens.unwrap_or(4096),
            "messages": messages_payload,
            "temperature": temperature.clamp(0.0, 1.0)
        });

        let sys_trimmed = system_prompt.trim();
        if !sys_trimmed.is_empty() {
            payload["system"] = json!(sys_trimmed);
        }

        if !tools.is_empty() {
            let tools_json: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "input_schema": t.parameters_schema
                    })
                })
                .collect();
            payload["tools"] = json!(tools_json);
        }

        let mut req = self
            .client
            .post(&url)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&payload);

        if let Some(ref key) = self.config.api_key {
            req = req.header("x-api-key", key);
        }
        for (k, v) in &self.config.extra_headers {
            req = req.header(k, v);
        }

        let res = req
            .send()
            .await
            .map_err(|e| DomainError::Validation(format!("Anthropic API Network Error: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(DomainError::Validation(format!(
                "Anthropic API Error [HTTP {}]: {}",
                status, error_text
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| DomainError::Validation(format!("Invalid JSON from Anthropic: {}", e)))?;

        let mut text_parts = Vec::new();
        let mut tool_calls = Vec::new();

        if let Some(content_arr) = body["content"].as_array() {
            for block in content_arr {
                match block["type"].as_str() {
                    Some("text") => {
                        if let Some(t) = block["text"].as_str() {
                            let trimmed = t.trim();
                            if !trimmed.is_empty() {
                                text_parts.push(trimmed);
                            }
                        }
                    }
                    Some("tool_use") => {
                        let id = block["id"].as_str().unwrap_or_default().to_string();
                        let tool_name = block["name"].as_str().unwrap_or_default().to_string();
                        let arguments = block.get("input").cloned().unwrap_or(json!({}));
                        tool_calls.push(AiToolCall {
                            id,
                            tool_name,
                            arguments,
                        });
                    }
                    _ => {}
                }
            }
        }

        let content = if text_parts.is_empty() {
            None
        } else {
            Some(text_parts.join("\n"))
        };

        Ok(LlmChatOutput {
            content,
            tool_calls,
        })
    }

    fn build_anthropic_url(&self) -> String {
        let base = if self.config.endpoint_url.trim().is_empty() {
            "https://api.anthropic.com"
        } else {
            self.config.endpoint_url.trim().trim_end_matches('/')
        };

        if base.ends_with("/v1/messages") {
            base.to_string()
        } else if base.ends_with("/v1") {
            format!("{}/messages", base)
        } else {
            format!("{}/v1/messages", base)
        }
    }
}

#[async_trait]
impl LlmGatewayPort for LlmAdapter {
    async fn generate_text(
        &self,
        _ctx: &RequestContext,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f64,
    ) -> Result<String, DomainError> {
        let selected_model = self.resolve_model(model)?;

        match self.config.protocol {
            LlmProviderProtocol::OpenAiCompatible => {
                self.openai_generate_text(selected_model, system_prompt, user_prompt, temperature)
                    .await
            }
            LlmProviderProtocol::Gemini => {
                self.gemini_generate_text(selected_model, system_prompt, user_prompt, temperature)
                    .await
            }
            LlmProviderProtocol::AnthropicClaude => {
                self.anthropic_generate_text(selected_model, system_prompt, user_prompt, temperature)
                    .await
            }
        }
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
        let selected_model = self.resolve_model(model)?;

        match self.config.protocol {
            LlmProviderProtocol::OpenAiCompatible => {
                self.openai_chat_with_tools(selected_model, system_prompt, messages, tools, temperature)
                    .await
            }
            LlmProviderProtocol::Gemini => {
                self.gemini_chat_with_tools(selected_model, system_prompt, messages, tools, temperature)
                    .await
            }
            LlmProviderProtocol::AnthropicClaude => {
                self.anthropic_chat_with_tools(selected_model, system_prompt, messages, tools, temperature)
                    .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_resolution_strict() {
        // 1. Config with no default model
        let config = LlmAdapterConfig::new(LlmProviderProtocol::OpenAiCompatible, "https://api.openai.com");
        let adapter = LlmAdapter::new(config).unwrap();

        // Error if requested model is empty
        assert!(adapter.resolve_model("").is_err());
        assert!(adapter.resolve_model("   ").is_err());

        // OK if requested model is provided
        assert_eq!(adapter.resolve_model("gpt-4o").unwrap(), "gpt-4o");

        // 2. Config with explicit default model
        let config_with_def = LlmAdapterConfig::new(LlmProviderProtocol::Gemini, "https://generativelanguage.googleapis.com")
            .with_default_model(Some("gemini-2.5-pro".into()));
        let adapter_with_def = LlmAdapter::new(config_with_def).unwrap();

        // Returns configured default when empty
        assert_eq!(adapter_with_def.resolve_model("").unwrap(), "gemini-2.5-pro");
        // Overrides configured default when caller explicitly passes a model
        assert_eq!(adapter_with_def.resolve_model("gemini-2.5-flash").unwrap(), "gemini-2.5-flash");
    }

    #[test]
    fn test_url_building() {
        let open_ai = LlmAdapter::new(LlmAdapterConfig::new(
            LlmProviderProtocol::OpenAiCompatible,
            "https://api.deepseek.com",
        ))
        .unwrap();
        assert_eq!(open_ai.build_openai_chat_url(), "https://api.deepseek.com/v1/chat/completions");

        let gemini = LlmAdapter::new(LlmAdapterConfig::new(
            LlmProviderProtocol::Gemini,
            "https://generativelanguage.googleapis.com",
        ))
        .unwrap();
        assert_eq!(
            gemini.build_gemini_url("gemini-2.5-flash"),
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent"
        );

        let anthropic = LlmAdapter::new(LlmAdapterConfig::new(
            LlmProviderProtocol::AnthropicClaude,
            "https://api.anthropic.com",
        ))
        .unwrap();
        assert_eq!(anthropic.build_anthropic_url(), "https://api.anthropic.com/v1/messages");
    }
}
