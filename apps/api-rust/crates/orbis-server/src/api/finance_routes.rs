//! Finance routes — Accounts, FeeTemplates, StudentFees, Payments, Currencies.

use axum::{extract::{State, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;
use rust_decimal::Decimal;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::entities::account::AccountType;
use orbis_domain::entities::payment::PaymentMethod;
use orbis_domain::entities::fee_template::FeeFrequency;

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
    pub name_ar: Option<String>,
    pub account_type: String,
    pub sub_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub currency: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct FeeTemplateDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: String,
    pub default_amount: Decimal,
    pub currency: String,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub is_mandatory: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateFeeTemplateRequest {
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: Option<String>,
    pub default_amount: Decimal,
    pub currency: Option<String>,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub applicable_levels: Option<Vec<String>>,
    pub applicable_study_modes: Option<Vec<String>>,
    pub frequency: Option<String>,
    pub is_mandatory: Option<bool>,
}

#[derive(Serialize, ToSchema)]
pub struct StudentFeeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Uuid,
    pub original_amount: Decimal,
    pub discount_amount: Decimal,
    pub net_amount: Decimal,
    pub paid_amount: Decimal,
    pub balance_due: Decimal,
    pub status: String,
    pub due_date: NaiveDate,
}

#[derive(Deserialize, ToSchema)]
pub struct GenerateFeeRequest {
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Uuid,
    pub discount_amount: Option<Decimal>,
    pub due_date: NaiveDate,
}

#[derive(Serialize, ToSchema)]
pub struct PaymentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub fee_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub method: String,
    pub reference: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub paid_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct ProcessPaymentRequest {
    pub fee_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub enrollment_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub method: String,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub reference: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct CurrencyDto {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Deserialize)]
pub struct FeeQuery { pub enrollment_id: Option<Uuid>, pub student_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct PaymentQuery { pub fee_id: Option<Uuid>, pub student_id: Option<Uuid> }

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/accounts", request_body = CreateAccountRequest,
    responses((status = 201, body = AccountDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn create_account(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateAccountRequest>,
) -> Result<Json<ApiResponse<AccountDto>>, ApiError> {
    let account_type = match p.account_type.to_lowercase().as_str() {
        "liability" => AccountType::Liability,
        "equity" => AccountType::Equity,
        "revenue" => AccountType::Revenue,
        "expense" => AccountType::Expense,
        _ => AccountType::Asset,
    };

    let a = state.finance_service.create_account(
        &ctx,
        ctx.tenant_id,
        p.code,
        p.name,
        account_type,
        p.currency.unwrap_or_else(|| "SAR".to_string()),
        p.parent_id,
    ).await?;

    Ok(ApiResponse::ok(AccountDto {
        id: a.id,
        org_id: a.org_id,
        code: a.code,
        name: a.name,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency,
        is_active: a.is_active,
        balances: a.balances,
    }))
}

#[utoipa::path(get, path = "/api/v1/accounts",
    responses((status = 200, body = Vec<AccountDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn list_accounts(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AccountDto>>>, ApiError> {
    let accounts = state.finance_service.list_accounts(ctx.tenant_id).await?;
    let dtos = accounts.into_iter().map(|a| AccountDto {
        id: a.id,
        org_id: a.org_id,
        code: a.code,
        name: a.name,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency,
        is_active: a.is_active,
        balances: a.balances,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/fee-templates", request_body = CreateFeeTemplateRequest,
    responses((status = 201, body = FeeTemplateDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn create_fee_template(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateFeeTemplateRequest>,
) -> Result<Json<ApiResponse<FeeTemplateDto>>, ApiError> {
    let currency = p.currency.unwrap_or_else(|| "SAR".to_string());
    let t = state.finance_service.create_fee_template(
        &ctx,
        ctx.tenant_id,
        p.name,
        p.default_amount,
        currency,
        p.receivable_account_id,
        p.revenue_account_id,
    ).await?;

    Ok(ApiResponse::ok(FeeTemplateDto {
        id: t.id,
        org_id: t.org_id,
        code: t.code,
        name: t.name,
        name_ar: t.name_ar,
        category: t.category,
        default_amount: t.default_amount,
        currency: t.currency,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        is_mandatory: t.is_mandatory,
    }))
}

#[utoipa::path(get, path = "/api/v1/fee-templates",
    responses((status = 200, body = Vec<FeeTemplateDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn list_fee_templates(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<FeeTemplateDto>>>, ApiError> {
    let templates = state.finance_service.list_fee_templates(ctx.tenant_id).await?;
    let dtos = templates.into_iter().map(|t| FeeTemplateDto {
        id: t.id,
        org_id: t.org_id,
        code: t.code,
        name: t.name,
        name_ar: t.name_ar,
        category: t.category,
        default_amount: t.default_amount,
        currency: t.currency,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        is_mandatory: t.is_mandatory,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/student-fees", request_body = GenerateFeeRequest,
    responses((status = 201, body = StudentFeeDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn generate_fee(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<GenerateFeeRequest>,
) -> Result<Json<ApiResponse<StudentFeeDto>>, ApiError> {
    let f = state.finance_service.generate_student_fee(
        &ctx,
        ctx.tenant_id,
        p.period_id,
        p.student_id,
        p.enrollment_id,
        p.fee_template_id,
        p.discount_amount.unwrap_or(Decimal::ZERO),
        p.due_date,
    ).await?;
    Ok(ApiResponse::ok(StudentFeeDto {
        id: f.id,
        org_id: f.org_id,
        period_id: f.period_id,
        student_id: f.student_id,
        enrollment_id: f.enrollment_id,
        fee_template_id: f.fee_template_id,
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        paid_amount: f.paid_amount,
        balance_due: f.balance_due,
        status: f.status.to_string(),
        due_date: f.due_date,
    }))
}

#[utoipa::path(get, path = "/api/v1/student-fees",
    responses((status = 200, body = Vec<StudentFeeDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn list_fees(
    ctx: AuthContext, State(state): State<AppState>, Query(q): Query<FeeQuery>,
) -> Result<Json<ApiResponse<Vec<StudentFeeDto>>>, ApiError> {
    let filter = orbis_domain::queries::filters::StudentFeeFilter {
        student_id: q.student_id,
        enrollment_id: q.enrollment_id,
        fee_template_id: None,
        status: None,
        is_overdue: None,
        pagination: None,
    };
    let fees = state.finance_service.list_student_fees(ctx.tenant_id, filter).await?.items;
    let dtos = fees.into_iter().map(|f| StudentFeeDto {
        id: f.id,
        org_id: f.org_id,
        period_id: f.period_id,
        student_id: f.student_id,
        enrollment_id: f.enrollment_id,
        fee_template_id: f.fee_template_id,
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        paid_amount: f.paid_amount,
        balance_due: f.balance_due,
        status: f.status.to_string(),
        due_date: f.due_date,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/payments", request_body = ProcessPaymentRequest,
    responses((status = 201, body = PaymentDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn process_payment(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<ProcessPaymentRequest>,
) -> Result<Json<ApiResponse<PaymentDto>>, ApiError> {
    let method = match p.method.to_lowercase().as_str() {
        "bank_transfer" => PaymentMethod::BankTransfer,
        "check" => PaymentMethod::Check,
        "credit_card" => PaymentMethod::CreditCard,
        "mada" => PaymentMethod::Mada,
        "apple_pay" => PaymentMethod::ApplePay,
        "cash" => PaymentMethod::Cash,
        _ => PaymentMethod::Other,
    };

    let pay = state.finance_service.process_payment(
        &ctx,
        ctx.tenant_id,
        p.fee_id,
        p.student_id,
        p.enrollment_id,
        p.account_id,
        p.amount,
        method,
        p.payer_name,
        p.payer_phone,
        p.reference,
        p.source,
    ).await?;

    Ok(ApiResponse::ok(PaymentDto {
        id: pay.id,
        org_id: pay.org_id,
        fee_id: pay.fee_id,
        account_id: pay.account_id,
        amount: pay.amount,
        method: pay.method.to_string(),
        reference: pay.reference,
        journal_entry_id: pay.journal_entry_id,
        paid_at: pay.paid_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/payments",
    responses((status = 200, body = Vec<PaymentDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn list_payments(
    ctx: AuthContext, State(state): State<AppState>, Query(q): Query<PaymentQuery>,
) -> Result<Json<ApiResponse<Vec<PaymentDto>>>, ApiError> {
    let filter = orbis_domain::queries::filters::PaymentFilter {
        student_id: q.student_id,
        enrollment_id: None,
        fee_id: q.fee_id,
        method: None,
        from_date: None,
        to_date: None,
        pagination: None,
    };
    let payments = state.payment_repo.list(ctx.tenant_id, filter).await?.items;
    let dtos = payments.into_iter().map(|p| PaymentDto {
        id: p.id,
        org_id: p.org_id,
        fee_id: p.fee_id,
        account_id: p.account_id,
        amount: p.amount,
        method: p.method.to_string(),
        reference: p.reference,
        journal_entry_id: p.journal_entry_id,
        paid_at: p.paid_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/currencies",
    responses((status = 200, body = Vec<CurrencyDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn list_currencies(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<CurrencyDto>>>, ApiError> {
    let currencies = state.currency_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = currencies.into_iter().map(|c| CurrencyDto { code: c.code, name: c.name, symbol: c.symbol }).collect();
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
