//! Tauri IPC commands — Finance (Accounts, FeeTemplates, StudentFees, Payments, Currencies).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use orbis_domain::entities::account::AccountType;
use orbis_domain::entities::payment::PaymentMethod;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct AccountDto {
    pub id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<String>,
    pub currency: String,
    pub is_active: bool,
    pub balances: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CreateAccountInput {
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize)]
pub struct FeeTemplateDto {
    pub id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub default_amount: Decimal,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub is_mandatory: bool,
}

#[derive(Deserialize)]
pub struct CreateFeeTemplateInput {
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: Option<String>,
    pub default_amount: Decimal,
    pub currency: Option<String>,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub is_mandatory: Option<bool>,
}

#[derive(Serialize)]
pub struct StudentFeeDto {
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub student_id: String,
    pub enrollment_id: Option<String>,
    pub fee_template_id: String,
    pub original_amount: Decimal,
    pub discount_amount: Decimal,
    pub net_amount: Decimal,
    pub paid_amount: Decimal,
    pub balance_due: Decimal,
    pub status: String,
    pub due_date: String,
}

#[derive(Deserialize)]
pub struct GenerateFeeInput {
    pub period_id: String,
    pub student_id: String,
    pub enrollment_id: Option<String>,
    pub fee_template_id: String,
    pub discount_amount: Option<Decimal>,
    pub due_date: String,
}

#[derive(Serialize)]
pub struct PaymentDto {
    pub id: String,
    pub org_id: String,
    pub fee_id: Option<String>,
    pub student_id: Option<String>,
    pub account_id: String,
    pub amount: Decimal,
    pub method: String,
    pub reference: Option<String>,
    pub journal_entry_id: Option<String>,
    pub paid_at: String,
}

#[derive(Deserialize)]
pub struct ProcessPaymentInput {
    pub fee_id: Option<String>,
    pub student_id: Option<String>,
    pub enrollment_id: Option<String>,
    pub account_id: String,
    pub amount: Decimal,
    pub method: String,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub reference: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct CurrencyDto { pub code: String, pub name: String, pub symbol: String }

#[derive(Deserialize, Default)]
pub struct FeeFilter {
    #[serde(rename = "enrollmentId")]
    pub enrollment_id: Option<String>,
    #[serde(rename = "studentId")]
    pub student_id: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct PaymentFilter {
    #[serde(rename = "feeId")]
    pub fee_id: Option<String>,
    #[serde(rename = "studentId")]
    pub student_id: Option<String>,
}

#[tauri::command]
pub async fn finance_create_account(request: CreateAccountInput, state: State<'_, AppState>) -> Result<ApiResponse<AccountDto>, String> {
    let ctx = make_ctx(&state);
    let parent_id = request.parent_id.and_then(|p| Uuid::parse_str(&p).ok());
    let account_type = match request.account_type.to_lowercase().as_str() {
        "asset" => AccountType::Asset,
        "liability" => AccountType::Liability,
        "equity" => AccountType::Equity,
        "revenue" => AccountType::Revenue,
        "expense" => AccountType::Expense,
        _ => AccountType::Asset,
    };

    let a = state.finance_service.create_account(
        &ctx,
        ctx.tenant_id,
        request.code,
        request.name,
        account_type,
        request.currency.unwrap_or_else(|| "SAR".to_string()),
        parent_id,
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(AccountDto {
        id: a.id.to_string(),
        org_id: a.org_id.to_string(),
        code: a.code,
        name: a.name,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id.map(|p| p.to_string()),
        currency: a.currency,
        is_active: a.is_active,
        balances: a.balances,
    }))
}

#[tauri::command]
pub async fn finance_list_accounts(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AccountDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.finance_service.list_accounts(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|a| AccountDto {
        id: a.id.to_string(),
        org_id: a.org_id.to_string(),
        code: a.code,
        name: a.name,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id.map(|p| p.to_string()),
        currency: a.currency,
        is_active: a.is_active,
        balances: a.balances,
    }).collect()))
}

#[tauri::command]
pub async fn finance_create_fee_template(request: CreateFeeTemplateInput, state: State<'_, AppState>) -> Result<ApiResponse<FeeTemplateDto>, String> {
    let ctx = make_ctx(&state);
    let rev_id = Uuid::parse_str(&request.revenue_account_id).map_err(|_| "Invalid revenue_account_id")?;
    let rec_id = Uuid::parse_str(&request.receivable_account_id).map_err(|_| "Invalid receivable_account_id")?;
    let currency = request.currency.unwrap_or_else(|| "SAR".to_string());

    let t = state.finance_service.create_fee_template(
        &ctx,
        ctx.tenant_id,
        request.name,
        request.default_amount,
        currency,
        rec_id,
        rev_id,
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(FeeTemplateDto {
        id: t.id.to_string(),
        org_id: t.org_id.to_string(),
        code: t.code,
        name: t.name,
        default_amount: t.default_amount,
        revenue_account_id: t.revenue_account_id.to_string(),
        receivable_account_id: t.receivable_account_id.to_string(),
        is_mandatory: t.is_mandatory,
    }))
}

#[tauri::command]
pub async fn finance_list_fee_templates(state: State<'_, AppState>) -> Result<ApiResponse<Vec<FeeTemplateDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.finance_service.list_fee_templates(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|t| FeeTemplateDto {
        id: t.id.to_string(),
        org_id: t.org_id.to_string(),
        code: t.code,
        name: t.name,
        default_amount: t.default_amount,
        revenue_account_id: t.revenue_account_id.to_string(),
        receivable_account_id: t.receivable_account_id.to_string(),
        is_mandatory: t.is_mandatory,
    }).collect()))
}

#[tauri::command]
pub async fn finance_generate_fee(request: GenerateFeeInput, state: State<'_, AppState>) -> Result<ApiResponse<StudentFeeDto>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid period_id")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid student_id")?;
    let enrollment_id = request.enrollment_id.and_then(|e| Uuid::parse_str(&e).ok());
    let fee_template_id = Uuid::parse_str(&request.fee_template_id).map_err(|_| "Invalid fee_template_id")?;
    let due_date = NaiveDate::parse_from_str(&request.due_date, "%Y-%m-%d").map_err(|_| "Invalid due_date (expected YYYY-MM-DD)")?;

    let f = state.finance_service.generate_student_fee(
        &ctx,
        ctx.tenant_id,
        period_id,
        student_id,
        enrollment_id,
        fee_template_id,
        request.discount_amount,
        Some(due_date),
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(StudentFeeDto {
        id: f.id.to_string(),
        org_id: f.org_id.to_string(),
        period_id: f.period_id.to_string(),
        student_id: f.student_id.to_string(),
        enrollment_id: f.enrollment_id.map(|e| e.to_string()),
        fee_template_id: f.fee_template_id.to_string(),
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        paid_amount: f.paid_amount,
        balance_due: f.balance_due,
        status: f.status.to_string(),
        due_date: f.due_date.to_string(),
    }))
}

#[tauri::command]
pub async fn finance_list_student_fees(filter: Option<FeeFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<StudentFeeDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let query_filter = orbis_domain::queries::filters::StudentFeeFilter {
        student_id: f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        enrollment_id: f.enrollment_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        fee_template_id: None,
        status: None,
        is_overdue: None,
        pagination: None,
    };
    let items = state.finance_service.list_student_fees(ctx.tenant_id, query_filter).await.map_err(|e| e.to_string())?.items;
    Ok(ok(items.into_iter().map(|f| StudentFeeDto {
        id: f.id.to_string(),
        org_id: f.org_id.to_string(),
        period_id: f.period_id.to_string(),
        student_id: f.student_id.to_string(),
        enrollment_id: f.enrollment_id.map(|e| e.to_string()),
        fee_template_id: f.fee_template_id.to_string(),
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        paid_amount: f.paid_amount,
        balance_due: f.balance_due,
        status: f.status.to_string(),
        due_date: f.due_date.to_string(),
    }).collect()))
}

#[tauri::command]
pub async fn finance_process_payment(request: ProcessPaymentInput, state: State<'_, AppState>) -> Result<ApiResponse<PaymentDto>, String> {
    let ctx = make_ctx(&state);
    let fee_id = request.fee_id.and_then(|f| Uuid::parse_str(&f).ok());
    let student_id = request.student_id.and_then(|s| Uuid::parse_str(&s).ok());
    let enrollment_id = request.enrollment_id.and_then(|e| Uuid::parse_str(&e).ok());
    let account_id = Uuid::parse_str(&request.account_id).map_err(|_| "Invalid account_id")?;

    let method = match request.method.to_lowercase().as_str() {
        "cash" => PaymentMethod::Cash,
        "bank_transfer" | "bank" => PaymentMethod::BankTransfer,
        "check" | "cheque" => PaymentMethod::Check,
        "credit_card" | "card" => PaymentMethod::CreditCard,
        "mada" => PaymentMethod::Mada,
        "apple_pay" | "applepay" => PaymentMethod::ApplePay,
        _ => PaymentMethod::Other,
    };

    let pay = state.finance_service.process_payment(
        &ctx,
        ctx.tenant_id,
        fee_id,
        student_id,
        enrollment_id,
        account_id,
        request.amount,
        method,
        request.payer_name,
        request.payer_phone,
        request.reference,
        request.source,
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(PaymentDto {
        id: pay.id.to_string(),
        org_id: pay.org_id.to_string(),
        fee_id: pay.fee_id.map(|f| f.to_string()),
        student_id: pay.student_id.map(|s| s.to_string()),
        account_id: pay.account_id.to_string(),
        amount: pay.amount,
        method: pay.method.to_string(),
        reference: pay.reference,
        journal_entry_id: pay.journal_entry_id.map(|j| j.to_string()),
        paid_at: pay.paid_at.to_rfc3339(),
    }))
}

#[tauri::command]
pub async fn finance_list_payments(filter: Option<PaymentFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<PaymentDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let query_filter = orbis_domain::queries::filters::PaymentFilter {
        student_id: f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        enrollment_id: None,
        fee_id: f.fee_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        method: None,
        from_date: None,
        to_date: None,
        pagination: None,
    };
    let items = state.payment_repo.list(ctx.tenant_id, query_filter).await.map_err(|e| e.to_string())?.items;

    Ok(ok(items.into_iter().map(|p| PaymentDto {
        id: p.id.to_string(),
        org_id: p.org_id.to_string(),
        fee_id: p.fee_id.map(|f| f.to_string()),
        student_id: p.student_id.map(|s| s.to_string()),
        account_id: p.account_id.to_string(),
        amount: p.amount,
        method: p.method.to_string(),
        reference: p.reference,
        journal_entry_id: p.journal_entry_id.map(|j| j.to_string()),
        paid_at: p.paid_at.to_rfc3339(),
    }).collect()))
}

#[tauri::command]
pub async fn finance_list_currencies(state: State<'_, AppState>) -> Result<ApiResponse<Vec<CurrencyDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.currency_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|c| CurrencyDto { code: c.code, name: c.name, symbol: c.symbol }).collect()))
}
