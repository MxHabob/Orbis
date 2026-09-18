use async_trait::async_trait;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use orbis_domain::entities::ai::{AiMessage, AiToolDefinition, LlmChatOutput};

#[async_trait]
pub trait LlmGatewayPort: Send + Sync {
    async fn generate_text(&self, ctx: &RequestContext, model: &str, system_prompt: &str, user_prompt: &str, temperature: f64) -> Result<String, DomainError>;

    async fn chat_with_tools(
        &self,
        ctx: &RequestContext,
        model: &str,
        system_prompt: &str,
        messages: &[AiMessage],
        tools: &[AiToolDefinition],
        temperature: f64,
    ) -> Result<LlmChatOutput, DomainError> {
        let _ = tools;
        let last_user_msg = messages.iter().rev().find(|m| m.role == "user").map(|m| m.content.as_str()).unwrap_or_default();
        let text = self.generate_text(ctx, model, system_prompt, last_user_msg, temperature).await?;
        Ok(LlmChatOutput {
            content: Some(text),
            tool_calls: vec![],
        })
    }
}




#[async_trait]
pub trait PaymentGatewayPort: Send + Sync {
    async fn process_payment(&self, ctx: &RequestContext, amount: f64, currency: &str, source: &str) -> Result<String, DomainError>;
}

#[async_trait]
pub trait SurveillancePort: Send + Sync {
    async fn detect_face(&self, ctx: &RequestContext, image_data: &[u8]) -> Result<Option<String>, DomainError>;
}

use orbis_domain::entities::solver::{TimetableProblem, SolveJobStatus, ExamProblem, ExamSolveJobStatus};

#[async_trait]
pub trait NotificationPort: Send + Sync {
    async fn send_email(&self, ctx: &RequestContext, to: &str, subject: &str, body: &str) -> Result<(), DomainError>;
    async fn send_whatsapp(&self, ctx: &RequestContext, to: &str, message: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SchedulingSolverPort: Send + Sync {
    async fn submit_timetable(&self, ctx: &RequestContext, problem: &TimetableProblem) -> Result<(), DomainError>;
    async fn get_timetable_status(&self, ctx: &RequestContext, job_id: &str) -> Result<SolveJobStatus, DomainError>;
    async fn submit_exam(&self, ctx: &RequestContext, problem: &ExamProblem) -> Result<(), DomainError>;
    async fn get_exam_status(&self, ctx: &RequestContext, job_id: &str) -> Result<ExamSolveJobStatus, DomainError>;
}

