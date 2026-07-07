//! Ollama LLM Provider — local-first AI inference (ADQ-3).
//!
//! Connects to a locally running Ollama instance.
//! No internet dependency — ideal for desktop app deployment.
//!
//! Default: http://localhost:11434

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::traits::*;

/// Ollama provider configuration.
#[derive(Debug, Clone)]
pub struct OllamaConfig {
    pub base_url: String,
    pub default_model: String,
    pub timeout_secs: u64,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            default_model: "llama3.2".to_string(),
            timeout_secs: 120,
        }
    }
}

pub struct OllamaProvider {
    client: Client,
    config: OllamaConfig,
}

impl OllamaProvider {
    pub fn new(config: OllamaConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub fn with_defaults() -> Self {
        Self::new(OllamaConfig::default())
    }
}

// ── Ollama API types ────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stop: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    eval_count: u32,
    #[serde(default)]
    prompt_eval_count: u32,
}

#[derive(Debug, Serialize)]
struct OllamaEmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaEmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModelInfo>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelInfo {
    name: String,
}

// ── LlmProvider implementation ──────────────────────────────────

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn provider_name(&self) -> &str {
        "ollama"
    }

    fn supports_tools(&self) -> bool {
        // Ollama supports tool calling with compatible models
        true
    }

    fn supports_vision(&self) -> bool {
        // Depends on model (llava, etc.)
        false
    }

    async fn chat(
        &self,
        messages: &[ChatMessage],
        _tools: Option<&[ToolDef]>,
        config: &InferenceConfig,
    ) -> Result<LlmResponse, LlmError> {
        let model = if config.model.is_empty() {
            &self.config.default_model
        } else {
            &config.model
        };

        let ollama_messages: Vec<OllamaMessage> = messages.iter().map(|m| {
            OllamaMessage {
                role: match m.role {
                    MessageRole::System => "system".into(),
                    MessageRole::User => "user".into(),
                    MessageRole::Assistant => "assistant".into(),
                    MessageRole::Tool => "tool".into(),
                },
                content: m.content.clone(),
            }
        }).collect();

        let request = OllamaChatRequest {
            model: model.clone(),
            messages: ollama_messages,
            stream: false,
            options: Some(OllamaOptions {
                temperature: config.temperature,
                num_predict: config.max_tokens,
                top_p: config.top_p,
                stop: config.stop_sequences.clone(),
            }),
        };

        let url = format!("{}/api/chat", self.config.base_url);

        debug!(model, url = %url, "Sending chat request to Ollama");

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    LlmError::Unavailable(format!(
                        "Cannot connect to Ollama at {}. Is it running?",
                        self.config.base_url
                    ))
                } else if e.is_timeout() {
                    LlmError::RequestFailed("Ollama request timed out".into())
                } else {
                    LlmError::RequestFailed(e.to_string())
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::RequestFailed(
                format!("Ollama returned {}: {}", status, body)
            ));
        }

        let chat_response: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        Ok(LlmResponse {
            content: chat_response.message.content,
            tool_calls: vec![], // Tool calls parsed from content if needed
            usage: TokenUsage {
                prompt_tokens: chat_response.prompt_eval_count,
                completion_tokens: chat_response.eval_count,
                total_tokens: chat_response.prompt_eval_count + chat_response.eval_count,
            },
            finish_reason: FinishReason::Stop,
        })
    }

    async fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, LlmError> {
        let request = OllamaEmbedRequest {
            model: self.config.default_model.clone(),
            input: texts.iter().map(|t| t.to_string()).collect(),
        };

        let url = format!("{}/api/embed", self.config.base_url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Unavailable(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::RequestFailed("Embed request failed".into()));
        }

        let embed_response: OllamaEmbedResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        Ok(embed_response.embeddings)
    }

    async fn health_check(&self) -> Result<bool, LlmError> {
        let url = format!("{}/api/tags", self.config.base_url);

        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let tags: OllamaTagsResponse = resp.json().await
                    .map_err(|e| LlmError::ParseError(e.to_string()))?;

                let has_model = tags.models.iter()
                    .any(|m| m.name.starts_with(&self.config.default_model));

                if !has_model {
                    warn!(
                        model = %self.config.default_model,
                        available = ?tags.models.iter().map(|m| &m.name).collect::<Vec<_>>(),
                        "Default model not found in Ollama"
                    );
                }

                Ok(true)
            }
            Ok(resp) => Err(LlmError::Unavailable(
                format!("Ollama returned status {}", resp.status())
            )),
            Err(e) => Err(LlmError::Unavailable(e.to_string())),
        }
    }
}

/// Factory function for creating an LLM provider based on config.
pub fn create_llm_provider(provider: &str, config_json: &str) -> Box<dyn LlmProvider> {
    match provider {
        "ollama" => {
            let config: OllamaConfig = serde_json::from_str(config_json)
                .unwrap_or_default();
            Box::new(OllamaProvider::new(config))
        }
        _ => {
            // Default to Ollama (ADQ-3: local-first)
            Box::new(OllamaProvider::with_defaults())
        }
    }
}

// Allow OllamaConfig to be deserialized from JSON
impl<'de> Deserialize<'de> for OllamaConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(default = "default_base_url")]
            base_url: String,
            #[serde(default = "default_model")]
            default_model: String,
            #[serde(default = "default_timeout")]
            timeout_secs: u64,
        }

        fn default_base_url() -> String { "http://localhost:11434".into() }
        fn default_model() -> String { "llama3.2".into() }
        fn default_timeout() -> u64 { 120 }

        let helper = Helper::deserialize(deserializer)?;
        Ok(OllamaConfig {
            base_url: helper.base_url,
            default_model: helper.default_model,
            timeout_secs: helper.timeout_secs,
        })
    }
}
