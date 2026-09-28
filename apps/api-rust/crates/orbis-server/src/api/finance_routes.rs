//! Finance routes — Accounts, FeeTemplates, StudentFees, Payments, Currencies, Journal Entries, Installments, Refunds.

use axum::{extract::{State, Query, Path}, routing::{get, post}, Json, Router};
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
    pub applicable_levels: Option<Vec<String>>,
    pub applicable_study_modes: Option<Vec<String>>,
    pub frequency: String,
    pub is_mandatory: bool,
    pub is_active: bool,
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
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id,
        currency: a.currency_code,
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
        currency: a.currency_code,
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
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/accounts", post(create_account).get(list_accounts))
        .route("/api/v1/fee-templates", post(create_fee_template).get(list_fee_templates))
        .route("/api/v1/student-fees", post(generate_fee).get(list_fees))
        .route("/api/v1/student-fees/:id/split", post(split_fee))
        .route("/api/v1/student-fees/:id/refund", post(refund_fee))
        .route("/api/v1/payments", post(process_payment).get(list_payments))
        .route("/api/v1/currencies", get(list_currencies))
        .route("/api/v1/journal-entries", post(create_journal_entry).get(list_journal_entries))
        .route("/api/v1/journal-entries/:id", get(get_journal_entry))
        .route("/api/v1/journal-entries/:id/reverse", post(reverse_journal_entry))
}
