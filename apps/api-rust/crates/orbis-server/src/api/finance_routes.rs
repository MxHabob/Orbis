//! Finance routes — Accounts, FeeTemplates, StudentFees, Payments, Currencies.

use axum::{extract::{State, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::finance::{
    create_account::CreateAccountUseCase,
    create_fee_template::CreateFeeTemplateUseCase,
    generate_student_fee::GenerateStudentFeeUseCase,
    process_payment::ProcessPaymentUseCase,
};

#[derive(Serialize, ToSchema)]
pub struct AccountDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub currency: String,
    pub is_active: bool,
    pub balances: serde_json::Value,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub currency: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct FeeTemplateDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub default_amount: f64,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub is_mandatory: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateFeeTemplateRequest {
    pub name: String,
    pub default_amount: f64,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub is_mandatory: Option<bool>,
}

#[derive(Serialize, ToSchema)]
pub struct StudentFeeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Uuid,
    pub original_amount: f64,
    pub discount_amount: f64,
    pub net_amount: f64,
    pub status: String,
    pub due_date: NaiveDate,
}

#[derive(Deserialize, ToSchema)]
pub struct GenerateFeeRequest {
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Uuid,
    pub discount_amount: Option<f64>,
    pub due_date: NaiveDate,
}

#[derive(Serialize, ToSchema)]
pub struct PaymentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub fee_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub paid_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct ProcessPaymentRequest {
    pub fee_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub source: Option<String>,
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
    let a = uc.execute(
        &ctx,
        ctx.tenant_id,
        p.code,
        p.name,
        p.account_type,
        p.parent_id,
        p.currency.unwrap_or_else(|| "SAR".to_string()),
    ).await?;
    Ok(ApiResponse::ok(AccountDto {
        id: a.id,
        org_id: a.org_id,
        code: a.code,
        name: a.name,
        account_type: a.account_type,
        parent_id: a.parent_id,
        currency: a.currency,
        is_active: a.is_active,
        balances: a.balances,
    }))
}

#[utoipa::path(get, path = "/api/v1/accounts",
    responses((status = 200, body = Vec<AccountDto>)), security(("bearer" = [])), tag = "Finance")]
async fn list_accounts(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AccountDto>>>, ApiError> {
    let accounts = state.account_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = accounts.into_iter().map(|a| AccountDto {
        id: a.id,
        org_id: a.org_id,
        code: a.code,
        name: a.name,
        account_type: a.account_type,
        parent_id: a.parent_id,
        currency: a.currency,
        is_active: a.is_active,
        balances: a.balances,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/fee-templates", request_body = CreateFeeTemplateRequest,
    responses((status = 201, body = FeeTemplateDto)), security(("bearer" = [])), tag = "Finance")]
async fn create_fee_template(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateFeeTemplateRequest>,
) -> Result<Json<ApiResponse<FeeTemplateDto>>, ApiError> {
    let uc = CreateFeeTemplateUseCase { fee_template_repo: state.fee_template_repo.clone() };
    let t = uc.execute(
        &ctx,
        ctx.tenant_id,
        p.name,
        p.default_amount,
        p.revenue_account_id,
        p.receivable_account_id,
        p.is_mandatory.unwrap_or(false),
    ).await?;
    Ok(ApiResponse::ok(FeeTemplateDto {
        id: t.id,
        org_id: t.org_id,
        name: t.name,
        default_amount: t.default_amount,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        is_mandatory: t.is_mandatory,
    }))
}

#[utoipa::path(get, path = "/api/v1/fee-templates",
    responses((status = 200, body = Vec<FeeTemplateDto>)), security(("bearer" = [])), tag = "Finance")]
async fn list_fee_templates(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<FeeTemplateDto>>>, ApiError> {
    let templates = state.fee_template_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = templates.into_iter().map(|t| FeeTemplateDto {
        id: t.id,
        org_id: t.org_id,
        name: t.name,
        default_amount: t.default_amount,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        is_mandatory: t.is_mandatory,
    }).collect();
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
    let f = uc.execute(
        &ctx,
        ctx.tenant_id,
        p.student_id,
        p.enrollment_id,
        p.fee_template_id,
        p.discount_amount.unwrap_or(0.0),
        p.due_date,
    ).await?;
    Ok(ApiResponse::ok(StudentFeeDto {
        id: f.id,
        org_id: f.org_id,
        student_id: f.student_id,
        enrollment_id: f.enrollment_id,
        fee_template_id: f.fee_template_id,
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        status: f.status,
        due_date: f.due_date,
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
        id: f.id,
        org_id: f.org_id,
        student_id: f.student_id,
        enrollment_id: f.enrollment_id,
        fee_template_id: f.fee_template_id,
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        status: f.status,
        due_date: f.due_date,
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
    let pay = uc.execute(
        &ctx,
        ctx.tenant_id,
        p.fee_id,
        p.account_id,
        p.amount,
        p.method,
        p.reference,
        p.source,
    ).await?;
    Ok(ApiResponse::ok(PaymentDto {
        id: pay.id,
        org_id: pay.org_id,
        fee_id: pay.fee_id,
        account_id: pay.account_id,
        amount: pay.amount,
        method: pay.method,
        reference: pay.reference,
        journal_entry_id: pay.journal_entry_id,
        paid_at: pay.paid_at,
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
        id: p.id,
        org_id: p.org_id,
        fee_id: p.fee_id,
        account_id: p.account_id,
        amount: p.amount,
        method: p.method,
        reference: p.reference,
        journal_entry_id: p.journal_entry_id,
        paid_at: p.paid_at,
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

