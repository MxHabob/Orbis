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
    gemini::GeminiAdapter,
    hikvision::HikvisionAdapter,
    mada::MadaAdapter,
    moyasar::MoyasarAdapter,
    orbis_ai::OrbisAiAdapter,
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
                let provider = config.provider_name.to_lowercase();

                if provider == "gemini" || provider == "gemini-ai" || provider == "google" || provider == "google-gemini" {
                    let api_key = secrets
                        .into_iter()
                        .find(|s| s.key_name == "api_key")
                        .map(|s| s.secret_key)
                        .or_else(|| std::env::var("GEMINI_API_KEY").ok())
                        .unwrap_or_default();
                    Ok(Box::new(GeminiAdapter::new(api_key)))
                } else if provider == "openai" || provider == "deepseek" || provider == "groq" || provider == "mistral" {
                    let api_key = secrets
                        .into_iter()
                        .find(|s| s.key_name == "api_key" || s.key_name == "secret_key")
                        .map(|s| s.secret_key)
                        .or_else(|| std::env::var("OPENAI_API_KEY").ok());
                    let default_url = if provider == "deepseek" {
                        "https://api.deepseek.com"
                    } else if provider == "groq" {
                        "https://api.groq.com/openai"
                    } else {
                        "https://api.openai.com"
                    };
                    let endpoint_url = config
                        .settings
                        .as_ref()
                        .and_then(|s| s.get("endpoint_url"))
                        .and_then(|v| v.as_str())
                        .filter(|s| !s.trim().is_empty())
                        .unwrap_or(default_url)
                        .to_string();
                    Ok(Box::new(OrbisAiAdapter::with_api_key(endpoint_url, api_key)))
                } else if provider == "orbis" || provider == "ollama" || provider == "local" || provider == "vllm" || provider == "lmstudio" {
                    let endpoint_url = config
                        .settings
                        .as_ref()
                        .and_then(|s| s.get("endpoint_url"))
                        .and_then(|v| v.as_str())
                        .filter(|s| !s.trim().is_empty())
                        .unwrap_or("http://localhost:11434")
                        .to_string();
                    let api_key = secrets
                        .into_iter()
                        .find(|s| s.key_name == "api_key")
                        .map(|s| s.secret_key);
                    Ok(Box::new(OrbisAiAdapter::with_api_key(endpoint_url, api_key)))
                } else {
                    Err(DomainError::Validation(format!(
                        "Unsupported AI Provider: '{}'. Supported providers: gemini, openai, deepseek, groq, orbis (local ollama).",
                        config.provider_name
                    )))
                }
            }
            None => {
                // Auto Fallback: If environment keys exist, use them; otherwise fallback to local Ollama default
                if let Ok(gemini_key) = std::env::var("GEMINI_API_KEY") {
                    if !gemini_key.trim().is_empty() {
                        return Ok(Box::new(GeminiAdapter::new(gemini_key)));
                    }
                }
                if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
                    if !openai_key.trim().is_empty() {
                        return Ok(Box::new(OrbisAiAdapter::with_api_key("https://api.openai.com".into(), Some(openai_key))));
                    }
                }
                // Fallback to local Ollama / Orbis Local AI
                Ok(Box::new(OrbisAiAdapter::new("http://localhost:11434".into())))
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
