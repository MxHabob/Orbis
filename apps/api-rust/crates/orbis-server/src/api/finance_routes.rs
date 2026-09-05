//! Finance routes — Accounts, FeeTemplates, StudentFees, Payments, Currencies.

use axum::{extract::{State, Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::finance::{
    account::CreateAccountUseCase,
    fee_template::CreateFeeTemplateUseCase,
    student_fee::GenerateStudentFeeUseCase,
    payment::ProcessPaymentUseCase,
};

#[derive(Serialize, ToSchema)]
pub struct AccountDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub balance: f64,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: String,
}

#[derive(Serialize, ToSchema)]
pub struct FeeTemplateDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub amount: f64,
    pub fee_type: String,
    pub is_active: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateFeeTemplateRequest {
    pub name: String,
    pub amount: f64,
    pub fee_type: String,
}

#[derive(Serialize, ToSchema)]
pub struct StudentFeeDto {
    pub id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub template_id: Uuid,
    pub net_amount: f64,
    pub paid_amount: f64,
    pub status: String,
}

#[derive(Deserialize, ToSchema)]
pub struct GenerateFeeRequest {
    pub enrollment_id: Uuid,
    pub template_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct PaymentDto {
    pub id: Uuid,
    pub fee_id: Option<Uuid>,
    pub amount: f64,
    pub method: String,
    pub status: String,
    pub paid_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema)]
pub struct ProcessPaymentRequest {
    pub fee_id: Uuid,
    pub amount: f64,
    pub method: String,
}

#[derive(Serialize, ToSchema)]
pub struct CurrencyDto {
    pub id: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Deserialize)]
pub struct FeeQuery { pub enrollment_id: Option<Uuid>, pub student_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct PaymentQuery { pub fee_id: Option<Uuid> }

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/accounts", request_body = CreateAccountRequest,
    responses((status = 201, body = AccountDto)), security(("bearer" = [])), tag = "Finance")]
async fn create_account(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateAccountRequest>,
) -> Result<Json<ApiResponse<AccountDto>>, ApiError> {
    let uc = CreateAccountUseCase { account_repo: state.account_repo.clone() };
    let a = uc.execute(&ctx, ctx.tenant_id, p.code, p.name, p.account_type).await?;
    Ok(ApiResponse::ok(AccountDto { id: a.id, org_id: a.org_id, code: a.code, name: a.name, account_type: a.account_type, balance: a.balance }))
}

#[utoipa::path(get, path = "/api/v1/accounts",
    responses((status = 200, body = Vec<AccountDto>)), security(("bearer" = [])), tag = "Finance")]
async fn list_accounts(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AccountDto>>>, ApiError> {
    let accounts = state.account_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = accounts.into_iter().map(|a| AccountDto { id: a.id, org_id: a.org_id, code: a.code, name: a.name, account_type: a.account_type, balance: a.balance }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/fee-templates", request_body = CreateFeeTemplateRequest,
    responses((status = 201, body = FeeTemplateDto)), security(("bearer" = [])), tag = "Finance")]
async fn create_fee_template(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateFeeTemplateRequest>,
) -> Result<Json<ApiResponse<FeeTemplateDto>>, ApiError> {
    let uc = CreateFeeTemplateUseCase { fee_template_repo: state.fee_template_repo.clone() };
    let t = uc.execute(&ctx, ctx.tenant_id, p.name, p.amount, p.fee_type).await?;
    Ok(ApiResponse::ok(FeeTemplateDto { id: t.id, org_id: t.org_id, name: t.name, amount: t.amount, fee_type: t.fee_type, is_active: t.is_active }))
}

#[utoipa::path(get, path = "/api/v1/fee-templates",
    responses((status = 200, body = Vec<FeeTemplateDto>)), security(("bearer" = [])), tag = "Finance")]
async fn list_fee_templates(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<FeeTemplateDto>>>, ApiError> {
    let templates = state.fee_template_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = templates.into_iter().map(|t| FeeTemplateDto { id: t.id, org_id: t.org_id, name: t.name, amount: t.amount, fee_type: t.fee_type, is_active: t.is_active }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/student-fees", request_body = GenerateFeeRequest,
    responses((status = 201, body = StudentFeeDto)), security(("bearer" = [])), tag = "Finance")]
async fn generate_fee(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<GenerateFeeRequest>,
) -> Result<Json<ApiResponse<StudentFeeDto>>, ApiError> {
    let uc = GenerateStudentFeeUseCase {
        student_fee_repo: state.student_fee_repo.clone(),
        fee_template_repo: state.fee_template_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let f = uc.execute(&ctx, p.enrollment_id, p.template_id).await?;
    Ok(ApiResponse::ok(StudentFeeDto {
        id: f.id, student_id: f.student_id, enrollment_id: f.enrollment_id,
        template_id: f.template_id, net_amount: f.net_amount, paid_amount: f.paid_amount,
        status: f.status,
    }))
}

#[utoipa::path(get, path = "/api/v1/student-fees",
    responses((status = 200, body = Vec<StudentFeeDto>)), security(("bearer" = [])), tag = "Finance")]
async fn list_fees(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<FeeQuery>,
) -> Result<Json<ApiResponse<Vec<StudentFeeDto>>>, ApiError> {
    let fees = if let Some(eid) = q.enrollment_id {
        state.student_fee_repo.find_by_enrollment(eid).await?
    } else if let Some(sid) = q.student_id {
        state.student_fee_repo.find_by_student(sid).await?
    } else { vec![] };
    let dtos = fees.into_iter().map(|f| StudentFeeDto {
        id: f.id, student_id: f.student_id, enrollment_id: f.enrollment_id,
        template_id: f.template_id, net_amount: f.net_amount, paid_amount: f.paid_amount,
        status: f.status,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/payments", request_body = ProcessPaymentRequest,
    responses((status = 201, body = PaymentDto)), security(("bearer" = [])), tag = "Finance")]
async fn process_payment(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<ProcessPaymentRequest>,
) -> Result<Json<ApiResponse<PaymentDto>>, ApiError> {
    let uc = ProcessPaymentUseCase {
        payment_repo: state.payment_repo.clone(),
        student_fee_repo: state.student_fee_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let pay = uc.execute(&ctx, p.fee_id, p.amount, p.method).await?;
    Ok(ApiResponse::ok(PaymentDto {
        id: pay.id, fee_id: pay.fee_id, amount: pay.amount,
        method: pay.method, status: pay.status, paid_at: pay.paid_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/payments",
    responses((status = 200, body = Vec<PaymentDto>)), security(("bearer" = [])), tag = "Finance")]
async fn list_payments(
    ctx: AuthContext, State(state): State<AppState>, Query(q): Query<PaymentQuery>,
) -> Result<Json<ApiResponse<Vec<PaymentDto>>>, ApiError> {
    let payments = if let Some(fid) = q.fee_id {
        state.payment_repo.find_by_fee(fid).await?
    } else {
        state.payment_repo.find_by_org(ctx.tenant_id).await?
    };
    let dtos = payments.into_iter().map(|p| PaymentDto {
        id: p.id, fee_id: p.fee_id, amount: p.amount,
        method: p.method, status: p.status, paid_at: p.paid_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/currencies",
    responses((status = 200, body = Vec<CurrencyDto>)), security(("bearer" = [])), tag = "Finance")]
async fn list_currencies(
    _ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<CurrencyDto>>>, ApiError> {
    let currencies = state.currency_repo.find_all().await?;
    let dtos = currencies.into_iter().map(|c| CurrencyDto { id: c.id, name: c.name, symbol: c.symbol }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/accounts", post(create_account).get(list_accounts))
        .route("/api/v1/fee-templates", post(create_fee_template).get(list_fee_templates))
        .route("/api/v1/student-fees", post(generate_fee).get(list_fees))
        .route("/api/v1/payments", post(process_payment).get(list_payments))
        .route("/api/v1/currencies", get(list_currencies))
}
