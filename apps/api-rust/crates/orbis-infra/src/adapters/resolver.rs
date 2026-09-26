use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::{
    LlmGatewayPort, NotificationPort, PaymentGatewayPort, SchedulingSolverPort, SurveillancePort,
};
use orbis_app::ports::repositories::IntegrationRepository;
use orbis_domain::entities::solver::{
    ExamProblem, ExamSolveJobStatus, SolveJobStatus, TimetableProblem,
};
use orbis_domain::errors::DomainError;
use std::sync::Arc;

use crate::adapters::{
    authen::AuthenAdapter,
    email::EmailAdapter,
    hikvision::HikvisionAdapter,
    llm_adapter::{LlmAdapter, LlmAdapterConfig, LlmProviderProtocol},
    mada::MadaAdapter,
    moyasar::MoyasarAdapter,
    solver_adapter::RemoteTimefoldSolver,
    whatsapp::WhatsAppAdapter,
};

pub struct IntegrationResolver {
    pub integration_repo: Arc<dyn IntegrationRepository>,
}

impl IntegrationResolver {
    pub fn new(integration_repo: Arc<dyn IntegrationRepository>) -> Self {
        Self { integration_repo }
    }

    async fn resolve_solver_adapter(&self, ctx: &RequestContext) -> Result<RemoteTimefoldSolver, DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_solver = configs.into_iter().find(|c| {
            (c.category == "automation" || c.category == "scheduling")
                && (c.provider_name == "solver_service" || c.provider_name == "solver-service")
                && c.is_enabled
        });

        match active_solver {
            Some(config) => {
                let endpoint = config
                    .settings
                    .as_ref()
                    .and_then(|s| s.get("endpoint_url"))
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or("http://localhost:8081")
                    .to_string();

                Ok(RemoteTimefoldSolver::new(endpoint))
            }
            None => Err(DomainError::Validation(
                "No active Timetable & Schedule Solver integration found. Please enable and configure Solver Service in App Store.".into(),
            )),
        }
    }

    async fn resolve_llm_adapter(&self, ctx: &RequestContext) -> Result<Box<dyn LlmGatewayPort>, DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_ai = configs.into_iter().find(|c| {
            (c.category == "ai" || c.category == "llm" || c.category == "ai-provider" || c.category == "ai_provider" || c.category == "ai-gateway") && c.is_enabled
        });

        match active_ai {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let provider = config.provider_name.trim().to_lowercase();
                let settings = config.settings.as_ref();

                // Extract settings without forcing arbitrary defaults
                let custom_endpoint = settings
                    .and_then(|s| s.get("endpoint_url"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty());

                let configured_model = settings
                    .and_then(|s| s.get("default_model").or_else(|| s.get("model")))
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty());

                let max_tokens = settings
                    .and_then(|s| s.get("max_tokens"))
                    .and_then(|v| v.as_u64())
                    .map(|n| n as u32);

                let timeout_secs = settings
                    .and_then(|s| s.get("timeout_secs"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(60);

                let protocol_override = settings
                    .and_then(|s| s.get("protocol"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim().to_lowercase());

                // Find API secret key
                let api_key = secrets
                    .into_iter()
                    .find(|s| s.key_name == "api_key" || s.key_name == "secret_key" || s.key_name == "token")
                    .map(|s| s.secret_key)
                    .filter(|k| !k.trim().is_empty());

                // Resolve protocol and standard base endpoint
                let (protocol, default_base_url) = match protocol_override.as_deref() {
                    Some("gemini") => (LlmProviderProtocol::Gemini, "https://generativelanguage.googleapis.com"),
                    Some("anthropic") | Some("claude") => (LlmProviderProtocol::AnthropicClaude, "https://api.anthropic.com"),
                    Some("openai") | Some("openai_compatible") => (LlmProviderProtocol::OpenAiCompatible, "https://api.openai.com"),
                    _ => {
                        // Infer from provider name
                        if provider == "gemini" || provider == "gemini-ai" || provider == "google" || provider == "google-gemini" {
                            (LlmProviderProtocol::Gemini, "https://generativelanguage.googleapis.com")
                        } else if provider == "anthropic" || provider == "claude" {
                            (LlmProviderProtocol::AnthropicClaude, "https://api.anthropic.com")
                        } else if provider == "deepseek" {
                            (LlmProviderProtocol::OpenAiCompatible, "https://api.deepseek.com")
                        } else if provider == "groq" {
                            (LlmProviderProtocol::OpenAiCompatible, "https://api.groq.com/openai")
                        } else if provider == "mistral" {
                            (LlmProviderProtocol::OpenAiCompatible, "https://api.mistral.ai")
                        } else if provider == "orbis" || provider == "ollama" || provider == "local" || provider == "vllm" || provider == "lmstudio" {
                            (LlmProviderProtocol::OpenAiCompatible, "http://localhost:11434")
                        } else {
                            (LlmProviderProtocol::OpenAiCompatible, "https://api.openai.com")
                        }
                    }
                };

                let endpoint_url = custom_endpoint.unwrap_or_else(|| default_base_url.to_string());

                let mut adapter_config = LlmAdapterConfig::new(protocol, endpoint_url)
                    .with_api_key(api_key)
                    .with_default_model(configured_model)
                    .with_max_tokens(max_tokens)
                    .with_timeout_secs(timeout_secs);

                if let Some(org_id) = settings.and_then(|s| s.get("organization_id")).and_then(|v| v.as_str()) {
                    if !org_id.trim().is_empty() {
                        adapter_config = adapter_config.with_header("OpenAI-Organization", org_id.trim());
                    }
                }

                let adapter = LlmAdapter::new(adapter_config)?;
                Ok(Box::new(adapter))
            }
            None => {
                // Environment variables fallback if configured on the host
                if let Ok(gemini_key) = std::env::var("GEMINI_API_KEY") {
                    if !gemini_key.trim().is_empty() {
                        let env_model = std::env::var("GEMINI_MODEL").ok();
                        let config = LlmAdapterConfig::new(
                            LlmProviderProtocol::Gemini,
                            "https://generativelanguage.googleapis.com",
                        )
                        .with_api_key(Some(gemini_key))
                        .with_default_model(env_model);
                        let adapter = LlmAdapter::new(config)?;
                        return Ok(Box::new(adapter));
                    }
                }

                if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
                    if !openai_key.trim().is_empty() {
                        let env_url = std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());
                        let env_model = std::env::var("OPENAI_MODEL").ok();
                        let config = LlmAdapterConfig::new(LlmProviderProtocol::OpenAiCompatible, env_url)
                            .with_api_key(Some(openai_key))
                            .with_default_model(env_model);
                        let adapter = LlmAdapter::new(config)?;
                        return Ok(Box::new(adapter));
                    }
                }

                if let Ok(anthropic_key) = std::env::var("ANTHROPIC_API_KEY") {
                    if !anthropic_key.trim().is_empty() {
                        let env_model = std::env::var("ANTHROPIC_MODEL").ok();
                        let config = LlmAdapterConfig::new(
                            LlmProviderProtocol::AnthropicClaude,
                            "https://api.anthropic.com",
                        )
                        .with_api_key(Some(anthropic_key))
                        .with_default_model(env_model);
                        let adapter = LlmAdapter::new(config)?;
                        return Ok(Box::new(adapter));
                    }
                }

                // If local Ollama is running / specified in env
                if let Ok(ollama_url) = std::env::var("OLLAMA_HOST").or_else(|_| std::env::var("OLLAMA_URL")) {
                    if !ollama_url.trim().is_empty() {
                        let env_model = std::env::var("OLLAMA_MODEL").ok();
                        let config = LlmAdapterConfig::new(LlmProviderProtocol::OpenAiCompatible, ollama_url)
                            .with_default_model(env_model);
                        let adapter = LlmAdapter::new(config)?;
                        return Ok(Box::new(adapter));
                    }
                }

                Err(DomainError::Validation(
                    "No active AI / LLM integration configured for this organization. Please enable and configure an AI provider in the App Store.".into(),
                ))
            }
        }
    }
}

#[async_trait]
impl LlmGatewayPort for IntegrationResolver {
    async fn generate_text(
        &self,
        ctx: &RequestContext,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f64,
    ) -> Result<String, DomainError> {
        let adapter = self.resolve_llm_adapter(ctx).await?;
        adapter.generate_text(ctx, model, system_prompt, user_prompt, temperature).await
    }

    async fn chat_with_tools(
        &self,
        ctx: &RequestContext,
        model: &str,
        system_prompt: &str,
        messages: &[orbis_domain::entities::ai::AiMessage],
        tools: &[orbis_domain::entities::ai::AiToolDefinition],
        temperature: f64,
    ) -> Result<orbis_domain::entities::ai::LlmChatOutput, DomainError> {
        let adapter = self.resolve_llm_adapter(ctx).await?;
        adapter.chat_with_tools(ctx, model, system_prompt, messages, tools, temperature).await
    }
}

#[async_trait]
impl PaymentGatewayPort for IntegrationResolver {
    async fn process_payment(
        &self,
        ctx: &RequestContext,
        amount: f64,
        currency: &str,
        source: &str,
    ) -> Result<String, DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_payment = configs.into_iter().find(|c| c.category == "payment-gateway" && c.is_enabled);

        match active_payment {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let secret_key = secrets
                    .into_iter()
                    .find(|s| s.key_name == "secret_key")
                    .map(|s| s.secret_key)
                    .unwrap_or_default();

                if config.provider_name == "mada" {
                    let adapter = MadaAdapter::new(secret_key);
                    adapter.process_payment(ctx, amount, currency, source).await
                } else if config.provider_name == "moyasar" {
                    let adapter = MoyasarAdapter::new(secret_key);
                    adapter.process_payment(ctx, amount, currency, source).await
                } else {
                    Err(DomainError::Validation(format!(
                        "Unsupported Payment Provider: {}",
                        config.provider_name
                    )))
                }
            }
            None => Err(DomainError::Validation(
                "No active Payment Gateway integration found".into(),
            )),
        }
    }
}

#[async_trait]
impl SurveillancePort for IntegrationResolver {
    async fn detect_face(&self, ctx: &RequestContext, image_data: &[u8]) -> Result<Option<String>, DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_camera = configs.into_iter().find(|c| c.category == "surveillance" && c.is_enabled);

        match active_camera {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let api_key = secrets
                    .into_iter()
                    .find(|s| s.key_name == "api_key")
                    .map(|s| s.secret_key)
                    .unwrap_or_default();

                if config.provider_name == "hikvision" {
                    let api_url = config
                        .settings
                        .as_ref()
                        .and_then(|s| s.get("api_url"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    let adapter = HikvisionAdapter::new(api_url, api_key);
                    adapter.detect_face(ctx, image_data).await
                } else if config.provider_name == "authen" {
                    let api_url = config
                        .settings
                        .as_ref()
                        .and_then(|s| s.get("api_url"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    let adapter = AuthenAdapter::new(api_url, api_key);
                    adapter.detect_face(ctx, image_data).await
                } else {
                    Err(DomainError::Validation(format!(
                        "Unsupported Surveillance Provider: {}",
                        config.provider_name
                    )))
                }
            }
            None => Err(DomainError::Validation(
                "No active Surveillance integration found".into(),
            )),
        }
    }
}

#[async_trait]
impl NotificationPort for IntegrationResolver {
    async fn send_email(&self, ctx: &RequestContext, to: &str, subject: &str, body: &str) -> Result<(), DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_email = configs.into_iter().find(|c| c.category == "email" && c.is_enabled);

        match active_email {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let api_key = secrets
                    .into_iter()
                    .find(|s| s.key_name == "api_key")
                    .map(|s| s.secret_key)
                    .unwrap_or_default();

                if config.provider_name == "sendgrid" {
                    let adapter = EmailAdapter::new(api_key);
                    adapter.send_email(ctx, to, subject, body).await
                } else {
                    Err(DomainError::Validation(format!(
                        "Unsupported Email Provider: {}",
                        config.provider_name
                    )))
                }
            }
            None => Err(DomainError::Validation(
                "No active Email integration found".into(),
            )),
        }
    }

    async fn send_whatsapp(&self, ctx: &RequestContext, to: &str, message: &str) -> Result<(), DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_wa = configs.into_iter().find(|c| c.category == "whatsapp" && c.is_enabled);

        match active_wa {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let sid = secrets
                    .iter()
                    .find(|s| s.key_name == "account_sid")
                    .map(|s| s.secret_key.clone())
                    .unwrap_or_default();
                let token = secrets
                    .iter()
                    .find(|s| s.key_name == "auth_token")
                    .map(|s| s.secret_key.clone())
                    .unwrap_or_default();

                let from_num = config
                    .settings
                    .as_ref()
                    .and_then(|s| s.get("from_number"))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                if config.provider_name == "twilio" {
                    let adapter = WhatsAppAdapter::new(sid, token, from_num);
                    adapter.send_whatsapp(ctx, to, message).await
                } else {
                    Err(DomainError::Validation(format!(
                        "Unsupported WhatsApp Provider: {}",
                        config.provider_name
                    )))
                }
            }
            None => Err(DomainError::Validation(
                "No active WhatsApp integration found".into(),
            )),
        }
    }
}

#[async_trait]
impl SchedulingSolverPort for IntegrationResolver {
    async fn submit_timetable(&self, ctx: &RequestContext, problem: &TimetableProblem) -> Result<(), DomainError> {
        let solver = self.resolve_solver_adapter(ctx).await?;
        solver.submit(problem).await
    }

    async fn get_timetable_status(&self, ctx: &RequestContext, job_id: &str) -> Result<SolveJobStatus, DomainError> {
        let solver = self.resolve_solver_adapter(ctx).await?;
        solver.status(job_id).await
    }

    async fn submit_exam(&self, ctx: &RequestContext, problem: &ExamProblem) -> Result<(), DomainError> {
        let solver = self.resolve_solver_adapter(ctx).await?;
        solver.submit_exam(problem).await
    }

    async fn get_exam_status(&self, ctx: &RequestContext, job_id: &str) -> Result<ExamSolveJobStatus, DomainError> {
        let solver = self.resolve_solver_adapter(ctx).await?;
        solver.status_exam(job_id).await
    }
}
