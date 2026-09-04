use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::finance::{
    create_account::CreateAccountUseCase,
    create_fee_template::CreateFeeTemplateUseCase,
    generate_student_fee::GenerateStudentFeeUseCase,
    process_payment::ProcessPaymentUseCase,
};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub currency: String,
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateFeeTemplateRequest {
    pub org_id: Uuid,
    pub name: String,
    pub default_amount: f64,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub is_mandatory: bool,
}

#[derive(Serialize)]
pub struct FeeTemplateResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct GenerateStudentFeeRequest {
    pub org_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Uuid,
    pub discount_amount: f64,
    pub due_date: String,
}

#[derive(Serialize)]
pub struct StudentFeeResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct ProcessPaymentRequest {
    pub org_id: Uuid,
    pub fee_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
}

async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, String> {
    let ctx = RequestContext { tenant_id: payload.org_id, actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    let use_case = CreateAccountUseCase { account_repo: state.account_repo.clone() };
    match use_case.execute(&ctx, payload.org_id, payload.code, payload.name, payload.account_type, payload.parent_id, payload.currency).await {
        Ok(acc) => Ok(Json(AccountResponse { id: acc.id })),
        Err(e) => Err(e.to_string()),
    }
}

async fn create_fee_template(
    State(state): State<AppState>,
    Json(payload): Json<CreateFeeTemplateRequest>,
) -> Result<Json<FeeTemplateResponse>, String> {
    let ctx = RequestContext { tenant_id: payload.org_id, actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    let use_case = CreateFeeTemplateUseCase { fee_template_repo: state.fee_template_repo.clone() };
    match use_case.execute(&ctx, payload.org_id, payload.name, payload.default_amount, payload.revenue_account_id, payload.receivable_account_id, payload.is_mandatory).await {
        Ok(tpl) => Ok(Json(FeeTemplateResponse { id: tpl.id })),
        Err(e) => Err(e.to_string()),
    }
}

async fn generate_student_fee(
    State(state): State<AppState>,
    Json(payload): Json<GenerateStudentFeeRequest>,
) -> Result<Json<StudentFeeResponse>, String> {
    let ctx = RequestContext { tenant_id: payload.org_id, actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    let due_date = NaiveDate::parse_from_str(&payload.due_date, "%Y-%m-%d").map_err(|_| "Invalid due date")?;
    
    let use_case = GenerateStudentFeeUseCase { 
        student_fee_repo: state.student_fee_repo.clone(),
        fee_template_repo: state.fee_template_repo.clone(),
        event_bus: state.event_bus.clone()
    };
    match use_case.execute(&ctx, payload.org_id, payload.student_id, payload.enrollment_id, payload.fee_template_id, payload.discount_amount, due_date).await {
        Ok(fee) => Ok(Json(StudentFeeResponse { id: fee.id })),
        Err(e) => Err(e.to_string()),
    }
}

async fn process_payment(
    State(state): State<AppState>,
    Json(payload): Json<ProcessPaymentRequest>,
) -> Result<Json<PaymentResponse>, String> {
    let ctx = RequestContext { tenant_id: payload.org_id, actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    
    let use_case = ProcessPaymentUseCase { 
        payment_repo: state.payment_repo.clone(),
        journal_repo: state.journal_entry_repo.clone(),
        student_fee_repo: state.student_fee_repo.clone(),
        fee_template_repo: state.fee_template_repo.clone(),
        event_bus: state.event_bus.clone(),
        payment_gateway: state.integration_resolver.clone(),
        notification_gateway: state.integration_resolver.clone(),
        permission_checker: state.permission_checker.clone(),
        currency_repo: state.currency_repo.clone(),
        policy_repo: state.policy_repo.clone(),
        account_repo: state.account_repo.clone(),
    };
    match use_case.execute(&ctx, payload.org_id, payload.fee_id, payload.account_id, payload.amount, payload.method, payload.reference, payload.source).await {
        Ok(payment) => Ok(Json(PaymentResponse { id: payment.id })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/finance/accounts", post(create_account))
        .route("/finance/fee-templates", post(create_fee_template))
        .route("/finance/student-fees", post(generate_student_fee))
        .route("/finance/payments", post(process_payment))
}
