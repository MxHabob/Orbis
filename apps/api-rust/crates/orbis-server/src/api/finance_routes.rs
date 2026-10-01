//! Finance routes — Accounts, FeeTemplates, StudentFees, Payments, Currencies, Journal Entries, Installments, Refunds.

use axum::{extract::{State, Query, Path}, routing::{get, post, delete}, Json, Router};
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
use orbis_domain::entities::journal_entry::{JournalEntry, JournalEntryLine, JournalPostingStatus};
use orbis_app::services::finance_service::{
    DisbursementInput, CreateSubAccountInput, StatementFilter,
    PayrollItemInput, PostPayrollInput, SettleGatewayPayoutInput,
};

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AccountDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub currency: String,
    pub is_active: bool,
    pub balances: serde_json::Value,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Deserialize, ToSchema)]
pub struct UpdateAccountRequest {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub account_type: String,
    pub sub_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub currency: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
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
    pub applicable_levels: Option<Vec<String>>,
    pub applicable_study_modes: Option<Vec<String>>,
    pub frequency: String,
    pub is_mandatory: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Deserialize, ToSchema)]
pub struct UpdateFeeTemplateRequest {
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: Option<String>,
    pub default_amount: Decimal,
    pub currency: Option<String>,
    pub revenue_account_id: Option<Uuid>,
    pub receivable_account_id: Option<Uuid>,
    pub applicable_levels: Option<Vec<String>>,
    pub frequency: Option<String>,
    pub is_mandatory: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct DiscountTemplateDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub discount_type: String,
    pub value: Decimal,
    pub eligibility_rules: Option<serde_json::Value>,
    pub is_approval_required: bool,
    pub is_auto_apply: bool,
    pub is_stackable: bool,
    pub priority: u32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateDiscountTemplateRequest {
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub discount_type: String,
    pub value: Decimal,
    pub eligibility_rules: Option<serde_json::Value>,
    pub is_approval_required: Option<bool>,
    pub is_auto_apply: Option<bool>,
    pub is_stackable: Option<bool>,
    pub priority: Option<u32>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
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
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct PaymentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub fee_id: Option<Uuid>,
    pub enrollment_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub method: String,
    pub reference: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub paid_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct JournalEntryLineDto {
    pub id: Uuid,
    pub account_id: Uuid,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct JournalEntryDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub reference_number: String,
    pub accounting_date: NaiveDate,
    pub amount: Decimal,
    pub currency_code: String,
    pub exchange_rate: Decimal,
    pub base_amount: Decimal,
    pub debit_account_id: Uuid,
    pub credit_account_id: Uuid,
    pub description: String,
    pub description_ar: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub status: String,
    pub lines: Vec<JournalEntryLineDto>,
    pub is_reversed: bool,
    pub reversed_by: Option<Uuid>,
    pub reversal_reason: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub version: i32,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateJournalEntryLineRequest {
    pub account_id: Uuid,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateJournalEntryRequest {
    pub reference_number: Option<String>,
    pub accounting_date: Option<NaiveDate>,
    pub amount: Decimal,
    pub currency_code: Option<String>,
    pub exchange_rate: Option<Decimal>,
    pub debit_account_id: Uuid,
    pub credit_account_id: Uuid,
    pub description: String,
    pub description_ar: Option<String>,
    pub lines: Option<Vec<CreateJournalEntryLineRequest>>,
}

#[derive(Deserialize, ToSchema)]
pub struct ReverseJournalRequest {
    pub reason: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefundStudentFeeRequest {
    pub refund_amount: Decimal,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct SplitFeeRequest {
    pub num_installments: u32,
    pub due_dates: Vec<NaiveDate>,
}

#[derive(Deserialize, ToSchema)]
pub struct ProcessDisbursementRequest {
    pub treasury_account_id: Uuid,
    pub beneficiary_account_id: Uuid,
    pub amount: Decimal,
    pub currency_code: Option<String>,
    pub payment_method: Option<String>,
    pub reference: Option<String>,
    pub beneficiary_name: Option<String>,
    pub beneficiary_type: Option<String>,
    pub beneficiary_id: Option<Uuid>,
    pub notes: Option<String>,
    pub voucher_date: Option<NaiveDate>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSubAccountRequest {
    pub parent_id: Option<Uuid>,
    pub parent_code: Option<String>,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub currency_code: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct StatementFilterQuery {
    pub account_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub currency_code: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct StatementTransactionItemDto {
    pub id: Uuid,
    pub date: NaiveDate,
    pub reference_number: String,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub description: String,
    pub description_ar: Option<String>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub running_balance: Decimal,
    pub contra_account_name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct StatementResponseDto {
    pub account_id: Option<Uuid>,
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub account_name_ar: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub currency: String,
    pub opening_balance: Decimal,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub closing_balance: Decimal,
    pub transactions: Vec<StatementTransactionItemDto>,
}

#[derive(Deserialize, ToSchema)]
pub struct PayrollItemDto {
    pub employee_id: Uuid,
    pub employee_name: String,
    pub is_academic: bool,
    pub base_salary: Decimal,
    pub allowances: Decimal,
    pub deductions: Decimal,
    pub gosi_deduction: Decimal,
    pub net_salary: Decimal,
}

#[derive(Deserialize, ToSchema)]
pub struct PostPayrollRequest {
    pub payroll_period: String,
    pub accounting_date: NaiveDate,
    pub items: Vec<PayrollItemDto>,
}

#[derive(Deserialize, ToSchema)]
pub struct DisbursePayrollRequest {
    pub bank_account_id: Uuid,
    pub payroll_period: String,
    pub amount: Decimal,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct SettleGatewayPayoutRequest {
    pub clearing_account_id: Option<Uuid>,
    pub bank_account_id: Uuid,
    pub gross_amount: Decimal,
    pub fee_amount: Decimal,
    pub net_amount: Decimal,
    pub payout_reference: String,
    pub accounting_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct FeeQuery { pub enrollment_id: Option<Uuid>, pub student_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct PaymentQuery { pub fee_id: Option<Uuid>, pub student_id: Option<Uuid> }

fn map_journal_entry_to_dto(entry: JournalEntry) -> JournalEntryDto {
    JournalEntryDto {
        id: entry.id,
        org_id: entry.org_id,
        reference_number: entry.reference_number,
        accounting_date: entry.accounting_date,
        amount: entry.amount,
        currency_code: entry.currency_code,
        exchange_rate: entry.exchange_rate,
        base_amount: entry.base_amount,
        debit_account_id: entry.debit_account_id,
        credit_account_id: entry.credit_account_id,
        description: entry.description,
        description_ar: entry.description_ar,
        source_type: entry.source_type,
        source_id: entry.source_id,
        status: entry.status.to_string(),
        lines: entry.lines.into_iter().map(|l| JournalEntryLineDto {
            id: l.id,
            account_id: l.account_id,
            debit_amount: l.debit_amount,
            credit_amount: l.credit_amount,
            description: l.description,
            entity_type: l.entity_type,
            entity_id: l.entity_id,
        }).collect(),
        is_reversed: entry.is_reversed,
        reversed_by: entry.reversed_by,
        reversal_reason: entry.reversal_reason,
        created_by: entry.created_by,
        created_at: entry.created_at,
        updated_at: entry.updated_at,
        updated_by: entry.updated_by,
        org_unit_id: entry.org_unit_id,
        version: entry.version,
    }
}

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
        name_ar: a.name_ar,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency_code,
        is_active: a.is_active,
        balances: a.balances,
        created_by: a.created_by,
        updated_by: a.updated_by,
        version: a.version,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/accounts/{id}",
    params(("id" = Uuid, Path, description = "Account ID")),
    responses((status = 200, body = AccountDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn get_account(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AccountDto>>, ApiError> {
    let a = state.finance_service.get_account(ctx.tenant_id, id).await?
        .ok_or_else(|| ApiError::not_found(format!("Account {} not found", id)))?;

    Ok(ApiResponse::ok(AccountDto {
        id: a.id,
        org_id: a.org_id,
        code: a.code,
        name: a.name,
        name_ar: a.name_ar,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency_code,
        is_active: a.is_active,
        balances: a.balances,
        created_by: a.created_by,
        updated_by: a.updated_by,
        version: a.version,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }))
}

#[utoipa::path(patch, path = "/api/v1/accounts/{id}",
    params(("id" = Uuid, Path, description = "Account ID")),
    request_body = UpdateAccountRequest,
    responses((status = 200, body = AccountDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn update_account(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>, Json(p): Json<UpdateAccountRequest>,
) -> Result<Json<ApiResponse<AccountDto>>, ApiError> {
    let account_type = match p.account_type.to_lowercase().as_str() {
        "liability" => AccountType::Liability,
        "equity" => AccountType::Equity,
        "revenue" => AccountType::Revenue,
        "expense" => AccountType::Expense,
        _ => AccountType::Asset,
    };

    let a = state.finance_service.update_account(
        &ctx,
        ctx.tenant_id,
        id,
        p.code,
        p.name,
        p.name_ar,
        account_type,
        p.sub_type,
        p.parent_id,
        p.currency.unwrap_or_else(|| "SAR".to_string()),
        p.is_active.unwrap_or(true),
    ).await?;

    Ok(ApiResponse::ok(AccountDto {
        id: a.id,
        org_id: a.org_id,
        code: a.code,
        name: a.name,
        name_ar: a.name_ar,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency_code,
        is_active: a.is_active,
        balances: a.balances,
        created_by: a.created_by,
        updated_by: a.updated_by,
        version: a.version,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/accounts/{id}",
    params(("id" = Uuid, Path, description = "Account ID")),
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Finance")]
pub async fn delete_account(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.finance_service.delete_account(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(true))
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
        name_ar: a.name_ar,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency_code,
        is_active: a.is_active,
        balances: a.balances,
        created_by: a.created_by,
        updated_by: a.updated_by,
        version: a.version,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/fee-templates", request_body = CreateFeeTemplateRequest,
    responses((status = 201, body = FeeTemplateDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn create_fee_template(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateFeeTemplateRequest>,
) -> Result<Json<ApiResponse<FeeTemplateDto>>, ApiError> {
    let currency_code = p.currency.unwrap_or_else(|| "SAR".to_string());
    let t = state.finance_service.create_fee_template(
        &ctx,
        ctx.tenant_id,
        p.name,
        p.default_amount,
        currency_code,
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
        currency: t.currency_code,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        applicable_levels: t.applicable_levels,
        applicable_study_modes: t.applicable_study_modes,
        frequency: t.frequency.to_string(),
        is_mandatory: t.is_mandatory,
        is_active: t.is_active,
        created_at: t.created_at,
        updated_at: t.updated_at,
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
        currency: t.currency_code,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        applicable_levels: t.applicable_levels,
        applicable_study_modes: t.applicable_study_modes,
        frequency: t.frequency.to_string(),
        is_mandatory: t.is_mandatory,
        is_active: t.is_active,
        created_at: t.created_at,
        updated_at: t.updated_at,
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
        p.discount_amount,
        Some(p.due_date),
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
        created_by: f.created_by,
        created_at: f.created_at,
        updated_at: f.updated_at,
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
        created_by: f.created_by,
        created_at: f.created_at,
        updated_at: f.updated_at,
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
        enrollment_id: pay.enrollment_id,
        student_id: pay.student_id,
        account_id: pay.account_id,
        amount: pay.amount,
        method: pay.method.to_string(),
        reference: pay.reference,
        journal_entry_id: pay.journal_entry_id,
        paid_at: pay.paid_at,
        created_by: pay.created_by,
        created_at: pay.created_at,
        updated_at: pay.updated_at,
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
        enrollment_id: p.enrollment_id,
        student_id: p.student_id,
        account_id: p.account_id,
        amount: p.amount,
        method: p.method.to_string(),
        reference: p.reference,
        journal_entry_id: p.journal_entry_id,
        paid_at: p.paid_at,
        created_by: p.created_by,
        created_at: p.created_at,
        updated_at: p.updated_at,
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

#[utoipa::path(get, path = "/api/v1/journal-entries",
    responses((status = 200, body = Vec<JournalEntryDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn list_journal_entries(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<JournalEntryDto>>>, ApiError> {
    let entries = state.journal_entry_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = entries.into_iter().map(map_journal_entry_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/journal-entries/{id}",
    params(("id" = Uuid, Path, description = "Journal Entry ID")),
    responses((status = 200, body = JournalEntryDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn get_journal_entry(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<JournalEntryDto>>, ApiError> {
    let entry = state.journal_entry_repo.find_by_id(id).await?
        .ok_or_else(|| ApiError::not_found(format!("Journal entry {} not found", id)))?;

    if entry.org_id != ctx.tenant_id {
        return Err(ApiError::unauthorized("Organization mismatch"));
    }

    Ok(ApiResponse::ok(map_journal_entry_to_dto(entry)))
}

#[utoipa::path(post, path = "/api/v1/journal-entries", request_body = CreateJournalEntryRequest,
    responses((status = 201, body = JournalEntryDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn create_journal_entry(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateJournalEntryRequest>,
) -> Result<Json<ApiResponse<JournalEntryDto>>, ApiError> {
    let now = Utc::now();
    let user_id = ctx.user_id();

    let lines: Vec<JournalEntryLine> = match p.lines {
        Some(ls) if !ls.is_empty() => ls.into_iter().map(|l| {
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: l.account_id,
                debit_amount: l.debit_amount,
                credit_amount: l.credit_amount,
                description: l.description,
                entity_type: l.entity_type,
                entity_id: l.entity_id,
            }
        }).collect(),
        _ => vec![
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: p.debit_account_id,
                debit_amount: p.amount,
                credit_amount: Decimal::ZERO,
                description: Some(p.description.clone()),
                entity_type: None,
                entity_id: None,
            },
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: p.credit_account_id,
                debit_amount: Decimal::ZERO,
                credit_amount: p.amount,
                description: Some(p.description.clone()),
                entity_type: None,
                entity_id: None,
            },
        ],
    };

    let entry = JournalEntry {
        id: Uuid::new_v4(),
        org_id: ctx.tenant_id,
        reference_number: p.reference_number.unwrap_or_else(|| format!("JV-{}", Utc::now().timestamp_millis())),
        accounting_date: p.accounting_date.unwrap_or_else(|| now.date_naive()),
        amount: p.amount,
        currency_code: p.currency_code.unwrap_or_else(|| "SAR".into()),
        exchange_rate: p.exchange_rate.unwrap_or(Decimal::ONE),
        base_amount: p.amount * p.exchange_rate.unwrap_or(Decimal::ONE),
        debit_account_id: p.debit_account_id,
        credit_account_id: p.credit_account_id,
        description: p.description,
        description_ar: p.description_ar,
        source_type: "manual".into(),
        source_id: None,
        status: JournalPostingStatus::Posted,
        lines,
        is_reversed: false,
        reversed_by: None,
        reversal_reason: None,
        created_by: user_id,
        created_at: now,
        updated_at: now,
        updated_by: user_id,
        org_unit_id: None,
        version: 1,
    };

    entry.validate_balance().map_err(ApiError::bad_request)?;
    state.journal_entry_repo.save(&entry).await?;

    Ok(ApiResponse::ok(map_journal_entry_to_dto(entry)))
}

#[utoipa::path(post, path = "/api/v1/journal-entries/{id}/reverse",
    params(("id" = Uuid, Path, description = "Journal Entry ID")),
    request_body = ReverseJournalRequest,
    responses((status = 200, body = JournalEntryDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn reverse_journal_entry(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>, Json(p): Json<ReverseJournalRequest>,
) -> Result<Json<ApiResponse<JournalEntryDto>>, ApiError> {
    let reversing = state.finance_service.reverse_journal_entry(
        &ctx,
        ctx.tenant_id,
        id,
        p.reason,
    ).await?;

    Ok(ApiResponse::ok(map_journal_entry_to_dto(reversing)))
}

#[utoipa::path(post, path = "/api/v1/student-fees/{id}/split",
    params(("id" = Uuid, Path, description = "Student Fee ID")),
    request_body = SplitFeeRequest,
    responses((status = 200, body = Vec<StudentFeeDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn split_fee(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>, Json(p): Json<SplitFeeRequest>,
) -> Result<Json<ApiResponse<Vec<StudentFeeDto>>>, ApiError> {
    let created = state.finance_service.split_fee_into_installments(
        &ctx,
        ctx.tenant_id,
        id,
        p.num_installments,
        p.due_dates,
    ).await?;

    let dtos = created.into_iter().map(|f| StudentFeeDto {
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
        created_by: f.created_by,
        created_at: f.created_at,
        updated_at: f.updated_at,
    }).collect();

    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/student-fees/{id}/refund",
    params(("id" = Uuid, Path, description = "Student Fee ID")),
    request_body = RefundStudentFeeRequest,
    responses((status = 200, body = StudentFeeDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn refund_fee(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>, Json(p): Json<RefundStudentFeeRequest>,
) -> Result<Json<ApiResponse<StudentFeeDto>>, ApiError> {
    let f = state.finance_service.refund_student_fee(
        &ctx,
        ctx.tenant_id,
        id,
        p.refund_amount,
        p.notes,
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
        created_by: f.created_by,
        created_at: f.created_at,
        updated_at: f.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/fee-templates/{id}",
    params(("id" = Uuid, Path, description = "Fee Template ID")),
    responses((status = 200, body = FeeTemplateDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn get_fee_template(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<FeeTemplateDto>>, ApiError> {
    let t = state.finance_service.get_fee_template(ctx.tenant_id, id).await?
        .ok_or_else(|| ApiError::not_found(format!("Fee template {} not found", id)))?;

    Ok(ApiResponse::ok(FeeTemplateDto {
        id: t.id,
        org_id: t.org_id,
        code: t.code,
        name: t.name,
        name_ar: t.name_ar,
        category: t.category,
        default_amount: t.default_amount,
        currency: t.currency_code,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        applicable_levels: t.applicable_levels,
        applicable_study_modes: t.applicable_study_modes,
        frequency: t.frequency.to_string(),
        is_mandatory: t.is_mandatory,
        is_active: t.is_active,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }))
}

#[utoipa::path(patch, path = "/api/v1/fee-templates/{id}",
    params(("id" = Uuid, Path, description = "Fee Template ID")),
    request_body = UpdateFeeTemplateRequest,
    responses((status = 200, body = FeeTemplateDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn update_fee_template(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>, Json(p): Json<UpdateFeeTemplateRequest>,
) -> Result<Json<ApiResponse<FeeTemplateDto>>, ApiError> {
    let t = state.finance_service.update_fee_template(
        &ctx,
        ctx.tenant_id,
        id,
        p.code,
        p.name,
        p.name_ar,
        p.category,
        p.default_amount,
        p.currency,
        p.revenue_account_id,
        p.receivable_account_id,
        p.applicable_levels,
        p.frequency,
        p.is_mandatory,
        p.is_active,
    ).await?;

    Ok(ApiResponse::ok(FeeTemplateDto {
        id: t.id,
        org_id: t.org_id,
        code: t.code,
        name: t.name,
        name_ar: t.name_ar,
        category: t.category,
        default_amount: t.default_amount,
        currency: t.currency_code,
        revenue_account_id: t.revenue_account_id,
        receivable_account_id: t.receivable_account_id,
        applicable_levels: t.applicable_levels,
        applicable_study_modes: t.applicable_study_modes,
        frequency: t.frequency.to_string(),
        is_mandatory: t.is_mandatory,
        is_active: t.is_active,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/fee-templates/{id}",
    params(("id" = Uuid, Path, description = "Fee Template ID")),
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Finance")]
pub async fn delete_fee_template(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.finance_service.delete_fee_template(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(get, path = "/api/v1/discount-templates",
    responses((status = 200, body = Vec<DiscountTemplateDto>)), security(("bearer" = [])), tag = "Finance")]
pub async fn list_discount_templates(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DiscountTemplateDto>>>, ApiError> {
    let templates = state.finance_service.list_discount_templates(ctx.tenant_id).await?;
    let dtos = templates.into_iter().map(|t| DiscountTemplateDto {
        id: t.id,
        org_id: t.org_id,
        code: t.code,
        name: t.name,
        name_ar: t.name_ar,
        discount_type: match t.discount_type {
            orbis_domain::entities::discount_template::DiscountType::Percentage => "percentage".into(),
            orbis_domain::entities::discount_template::DiscountType::FixedAmount => "fixed_amount".into(),
        },
        value: t.value,
        eligibility_rules: t.eligibility_rules,
        is_approval_required: t.is_approval_required,
        is_auto_apply: t.is_auto_apply,
        is_stackable: t.is_stackable,
        priority: t.priority,
        is_active: t.is_active,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/discount-templates", request_body = CreateDiscountTemplateRequest,
    responses((status = 201, body = DiscountTemplateDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn create_discount_template(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateDiscountTemplateRequest>,
) -> Result<Json<ApiResponse<DiscountTemplateDto>>, ApiError> {
    let dtype = match p.discount_type.to_lowercase().as_str() {
        "percentage" | "percent" => orbis_domain::entities::discount_template::DiscountType::Percentage,
        _ => orbis_domain::entities::discount_template::DiscountType::FixedAmount,
    };

    let t = state.finance_service.create_discount_template(
        &ctx,
        ctx.tenant_id,
        p.code,
        p.name,
        p.name_ar,
        dtype,
        p.value,
        p.eligibility_rules,
        p.is_approval_required.unwrap_or(false),
        p.is_auto_apply.unwrap_or(false),
        p.is_stackable.unwrap_or(false),
        p.priority.unwrap_or(1),
    ).await?;

    Ok(ApiResponse::ok(DiscountTemplateDto {
        id: t.id,
        org_id: t.org_id,
        code: t.code,
        name: t.name,
        name_ar: t.name_ar,
        discount_type: match t.discount_type {
            orbis_domain::entities::discount_template::DiscountType::Percentage => "percentage".into(),
            orbis_domain::entities::discount_template::DiscountType::FixedAmount => "fixed_amount".into(),
        },
        value: t.value,
        eligibility_rules: t.eligibility_rules,
        is_approval_required: t.is_approval_required,
        is_auto_apply: t.is_auto_apply,
        is_stackable: t.is_stackable,
        priority: t.priority,
        is_active: t.is_active,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/discount-templates/{id}",
    params(("id" = Uuid, Path, description = "Discount Template ID")),
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Finance")]
pub async fn delete_discount_template(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.finance_service.delete_discount_template(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(delete, path = "/api/v1/student-fees/{id}",
    params(("id" = Uuid, Path, description = "Student Fee ID")),
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Finance")]
pub async fn delete_student_fee(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.finance_service.delete_student_fee(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(delete, path = "/api/v1/payments/{id}",
    params(("id" = Uuid, Path, description = "Payment ID")),
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Finance")]
pub async fn delete_payment(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.finance_service.delete_payment(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(delete, path = "/api/v1/journal-entries/{id}",
    params(("id" = Uuid, Path, description = "Journal Entry ID")),
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Finance")]
pub async fn delete_journal_entry(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.finance_service.delete_journal_entry(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(get, path = "/api/v1/finance/statement",
    params(
        ("account_id" = Option<Uuid>, Query, description = "Account ID"),
        ("entity_type" = Option<String>, Query, description = "Entity Type: student, vendor, employee, sponsor"),
        ("entity_id" = Option<Uuid>, Query, description = "Entity ID"),
        ("start_date" = Option<NaiveDate>, Query, description = "Start Date"),
        ("end_date" = Option<NaiveDate>, Query, description = "End Date"),
        ("currency" = Option<String>, Query, description = "Currency"),
    ),
    responses((status = 200, body = StatementResponseDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn get_account_statement(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<StatementFilterQuery>,
) -> Result<Json<ApiResponse<StatementResponseDto>>, ApiError> {
    let filter = StatementFilter {
        account_id: q.account_id,
        entity_type: q.entity_type,
        entity_id: q.entity_id,
        start_date: q.start_date,
        end_date: q.end_date,
        currency_code: q.currency_code.or(q.currency),
    };
    let statement = state.finance_service.get_account_statement(&ctx, ctx.tenant_id, filter).await?;
    let dto = StatementResponseDto {
        account_id: statement.account_id,
        account_code: statement.account_code,
        account_name: statement.account_name,
        account_name_ar: statement.account_name_ar,
        entity_type: statement.entity_type,
        entity_id: statement.entity_id,
        currency: statement.currency,
        opening_balance: statement.opening_balance,
        total_debit: statement.total_debit,
        total_credit: statement.total_credit,
        closing_balance: statement.closing_balance,
        transactions: statement.transactions.into_iter().map(|t| StatementTransactionItemDto {
            id: t.id,
            date: t.date,
            reference_number: t.reference_number,
            source_type: t.source_type,
            source_id: t.source_id,
            description: t.description,
            description_ar: t.description_ar,
            debit: t.debit,
            credit: t.credit,
            running_balance: t.running_balance,
            contra_account_name: t.contra_account_name,
        }).collect(),
    };
    Ok(ApiResponse::ok(dto))
}

#[utoipa::path(post, path = "/api/v1/finance/disbursements", request_body = ProcessDisbursementRequest,
    responses((status = 201, body = JournalEntryDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn process_disbursement(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<ProcessDisbursementRequest>,
) -> Result<Json<ApiResponse<JournalEntryDto>>, ApiError> {
    let method = match p.payment_method.as_deref().unwrap_or("cash").to_lowercase().as_str() {
        "bank" | "bank_transfer" => PaymentMethod::BankTransfer,
        "check" => PaymentMethod::Check,
        "mada" => PaymentMethod::Mada,
        "credit_card" => PaymentMethod::CreditCard,
        "apple_pay" => PaymentMethod::ApplePay,
        _ => PaymentMethod::Cash,
    };

    let input = DisbursementInput {
        treasury_account_id: p.treasury_account_id,
        beneficiary_account_id: p.beneficiary_account_id,
        amount: p.amount,
        currency_code: p.currency_code,
        payment_method: method,
        reference: p.reference,
        beneficiary_name: p.beneficiary_name,
        beneficiary_type: p.beneficiary_type,
        beneficiary_id: p.beneficiary_id,
        notes: p.notes,
        voucher_date: p.voucher_date,
    };

    let entry = state.finance_service.process_disbursement(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(map_journal_entry_to_dto(entry)))
}

#[utoipa::path(post, path = "/api/v1/finance/sub-accounts", request_body = CreateSubAccountRequest,
    responses((status = 201, body = AccountDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn create_sub_account(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateSubAccountRequest>,
) -> Result<Json<ApiResponse<AccountDto>>, ApiError> {
    let input = CreateSubAccountInput {
        parent_id: p.parent_id,
        parent_code: p.parent_code,
        code: p.code,
        name: p.name,
        name_ar: p.name_ar,
        entity_type: p.entity_type,
        entity_id: p.entity_id,
        currency_code: p.currency_code,
    };

    let a = state.finance_service.create_sub_account(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(AccountDto {
        id: a.id,
        org_id: a.org_id,
        code: a.code,
        name: a.name,
        name_ar: a.name_ar,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency_code,
        is_active: a.is_active,
        balances: a.balances,
        created_by: a.created_by,
        updated_by: a.updated_by,
        version: a.version,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }))
}

#[utoipa::path(post, path = "/api/v1/finance/gateway/settlement", request_body = SettleGatewayPayoutRequest,
    responses((status = 201, body = JournalEntryDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn settle_gateway_payout(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<SettleGatewayPayoutRequest>,
) -> Result<Json<ApiResponse<JournalEntryDto>>, ApiError> {
    let input = SettleGatewayPayoutInput {
        clearing_account_id: p.clearing_account_id,
        bank_account_id: p.bank_account_id,
        gross_amount: p.gross_amount,
        fee_amount: p.fee_amount,
        net_amount: p.net_amount,
        payout_reference: p.payout_reference,
        accounting_date: p.accounting_date,
    };

    let entry = state.finance_service.settle_gateway_payout(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(map_journal_entry_to_dto(entry)))
}

#[utoipa::path(post, path = "/api/v1/finance/payroll/accrual", request_body = PostPayrollRequest,
    responses((status = 201, body = JournalEntryDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn post_payroll_accrual(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<PostPayrollRequest>,
) -> Result<Json<ApiResponse<JournalEntryDto>>, ApiError> {
    let input = PostPayrollInput {
        payroll_period: p.payroll_period,
        accounting_date: p.accounting_date,
        items: p.items.into_iter().map(|i| PayrollItemInput {
            employee_id: i.employee_id,
            employee_name: i.employee_name,
            is_academic: i.is_academic,
            base_salary: i.base_salary,
            allowances: i.allowances,
            deductions: i.deductions,
            gosi_deduction: i.gosi_deduction,
            net_salary: i.net_salary,
        }).collect(),
    };

    let entry = state.finance_service.post_payroll_journal(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(map_journal_entry_to_dto(entry)))
}

#[utoipa::path(post, path = "/api/v1/finance/payroll/disbursement", request_body = DisbursePayrollRequest,
    responses((status = 201, body = JournalEntryDto)), security(("bearer" = [])), tag = "Finance")]
pub async fn disburse_payroll_handler(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<DisbursePayrollRequest>,
) -> Result<Json<ApiResponse<JournalEntryDto>>, ApiError> {
    let entry = state.finance_service.disburse_payroll(
        &ctx,
        ctx.tenant_id,
        p.bank_account_id,
        p.payroll_period,
        p.amount,
        p.notes,
    ).await?;
    Ok(ApiResponse::ok(map_journal_entry_to_dto(entry)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/accounts", post(create_account).get(list_accounts))
        .route("/api/v1/accounts/:id", get(get_account).patch(update_account).delete(delete_account))
        .route("/api/v1/fee-templates", post(create_fee_template).get(list_fee_templates))
        .route("/api/v1/fee-templates/:id", get(get_fee_template).patch(update_fee_template).delete(delete_fee_template))
        .route("/api/v1/discount-templates", post(create_discount_template).get(list_discount_templates))
        .route("/api/v1/discount-templates/:id", delete(delete_discount_template))
        .route("/api/v1/student-fees", post(generate_fee).get(list_fees))
        .route("/api/v1/student-fees/:id", delete(delete_student_fee))
        .route("/api/v1/student-fees/:id/split", post(split_fee))
        .route("/api/v1/student-fees/:id/refund", post(refund_fee))
        .route("/api/v1/payments", post(process_payment).get(list_payments))
        .route("/api/v1/payments/:id", delete(delete_payment))
        .route("/api/v1/currencies", get(list_currencies))
        .route("/api/v1/journal-entries", post(create_journal_entry).get(list_journal_entries))
        .route("/api/v1/journal-entries/:id", get(get_journal_entry).delete(delete_journal_entry))
        .route("/api/v1/journal-entries/:id/reverse", post(reverse_journal_entry))
        .route("/api/v1/finance/statement", get(get_account_statement))
        .route("/api/v1/finance/disbursements", post(process_disbursement))
        .route("/api/v1/finance/sub-accounts", post(create_sub_account))
        .route("/api/v1/finance/gateway/settlement", post(settle_gateway_payout))
        .route("/api/v1/finance/payroll/accrual", post(post_payroll_accrual))
        .route("/api/v1/finance/payroll/disbursement", post(disburse_payroll_handler))
}
