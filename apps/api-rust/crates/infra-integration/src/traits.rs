//! Core traits for all external integrations.
//!
//! Every external system (LLM, camera, fingerprint, email, etc.)
//! is abstracted behind a trait. This allows:
//!   - Swapping providers without changing core logic
//!   - Testing with mock implementations
//!   - Graceful degradation when providers are unavailable

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════
// LLM Provider (ADQ-3: Ollama local-first)
// ═══════════════════════════════════════════════════════════════

/// Chat message for LLM interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    /// Optional tool call results (for function calling).
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Tool/function definition for LLM function calling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// LLM inference configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: Option<f32>,
    pub stop_sequences: Vec<String>,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            model: "llama3.2".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            top_p: None,
            stop_sequences: vec![],
        }
    }
}

/// Response from an LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub usage: TokenUsage,
    pub finish_reason: FinishReason,
}

/// A tool/function call requested by the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Token usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Reason the LLM stopped generating.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    Error,
}

/// LLM-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("LLM provider unavailable: {0}")]
    Unavailable(String),
    #[error("LLM request failed: {0}")]
    RequestFailed(String),
    #[error("LLM response parse error: {0}")]
    ParseError(String),
    #[error("LLM rate limited: retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },
    #[error("LLM context too long: {tokens} tokens (max {max_tokens})")]
    ContextTooLong { tokens: u32, max_tokens: u32 },
    #[error("LLM configuration error: {0}")]
    ConfigError(String),
}

/// Pluggable LLM provider interface.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Provider name (e.g., "ollama", "openai", "anthropic").
    fn provider_name(&self) -> &str;

    /// Whether this provider supports tool/function calling.
    fn supports_tools(&self) -> bool;

    /// Whether this provider supports vision (image input).
    fn supports_vision(&self) -> bool;

    /// Send a chat completion request.
    async fn chat(
        &self,
        messages: &[ChatMessage],
        tools: Option<&[ToolDef]>,
        config: &InferenceConfig,
    ) -> Result<LlmResponse, LlmError>;

    /// Generate embeddings for text.
    async fn embed(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, LlmError>;

    /// Check if the provider is reachable / healthy.
    async fn health_check(&self) -> Result<bool, LlmError>;
}

// ═══════════════════════════════════════════════════════════════
// Integration Provider (cameras, fingerprint, IoT, etc.)
// ═══════════════════════════════════════════════════════════════

/// An event from an external integration source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationEvent {
    pub id: String,
    pub source: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp: String,
    pub org_id: String,
}

/// Integration adapter status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AdapterStatus {
    Connected,
    Disconnected,
    Error,
    Disabled,
}

/// Generic integration provider trait.
#[async_trait]
pub trait IntegrationProvider: Send + Sync {
    /// Adapter name (e.g., "hikvision_camera", "zkteco_fingerprint").
    fn adapter_name(&self) -> &str;

    /// Adapter category (e.g., "camera", "biometric", "iot").
    fn category(&self) -> &str;

    /// Current adapter status.
    fn status(&self) -> AdapterStatus;

    /// Start receiving events from this adapter.
    async fn connect(&mut self) -> Result<(), String>;

    /// Stop receiving events.
    async fn disconnect(&mut self) -> Result<(), String>;

    /// Poll for new events (for adapters that don't push).
    async fn poll_events(&self) -> Result<Vec<IntegrationEvent>, String>;
}

// ═══════════════════════════════════════════════════════════════
// Notification Provider (email, SMS, push)
// ═══════════════════════════════════════════════════════════════

/// A notification to be sent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub recipient: String,
    pub subject: String,
    pub body: String,
    pub body_html: Option<String>,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    InApp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Notification provider trait.
#[async_trait]
pub trait NotificationProvider: Send + Sync {
    fn provider_name(&self) -> &str;
    fn supported_channels(&self) -> Vec<NotificationChannel>;

    async fn send(&self, notification: &Notification) -> Result<String, String>;
    async fn send_bulk(&self, notifications: &[Notification]) -> Result<Vec<String>, String>;
}
