//! Tauri IPC commands — Finance (Accounts, FeeTemplates, StudentFees, Payments, Currencies).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::NaiveDate;
use orbis_app::usecases::finance::{
    create_account::CreateAccountUseCase,
    create_fee_template::CreateFeeTemplateUseCase,
    generate_student_fee::GenerateStudentFeeUseCase,
    process_payment::ProcessPaymentUseCase,
};
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
    pub name: String,
    pub default_amount: f64,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub is_mandatory: bool,
}

#[derive(Deserialize)]
pub struct CreateFeeTemplateInput {
    pub name: String,
    pub default_amount: f64,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub is_mandatory: Option<bool>,
}

#[derive(Serialize)]
pub struct StudentFeeDto {
    pub id: String,
    pub org_id: String,
    pub student_id: String,
    pub enrollment_id: Option<String>,
    pub fee_template_id: String,
    pub original_amount: f64,
    pub discount_amount: f64,
    pub net_amount: f64,
    pub status: String,
    pub due_date: String,
}

#[derive(Deserialize)]
pub struct GenerateFeeInput {
    pub student_id: String,
    pub enrollment_id: Option<String>,
    pub fee_template_id: String,
    pub discount_amount: Option<f64>,
    pub due_date: String,
}

#[derive(Serialize)]
pub struct PaymentDto {
    pub id: String,
    pub org_id: String,
    pub fee_id: Option<String>,
    pub account_id: String,
    pub amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub journal_entry_id: Option<String>,
    pub paid_at: String,
}

#[derive(Deserialize)]
pub struct ProcessPaymentInput {
    pub fee_id: Option<String>,
    pub account_id: String,
    pub amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct CurrencyDto { pub id: String, pub name: String, pub symbol: String }

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
}

#[tauri::command]
pub async fn finance_create_account(request: CreateAccountInput, state: State<'_, AppState>) -> Result<ApiResponse<AccountDto>, String> {
    let ctx = make_ctx(&state);
    let uc = CreateAccountUseCase { account_repo: state.account_repo.clone() };
    let parent_id = request.parent_id.and_then(|p| Uuid::parse_str(&p).ok());
    let a = uc.execute(
        &ctx,
        ctx.tenant_id,
        request.code,
        request.name,
        request.account_type,
        parent_id,
        request.currency.unwrap_or_else(|| "SAR".to_string()),
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(AccountDto {
        id: a.id.to_string(),
        org_id: a.org_id.to_string(),
        code: a.code,
        name: a.name,
        account_type: a.account_type,
        parent_id: a.parent_id.map(|p| p.to_string()),
        currency: a.currency,
        is_active: a.is_active,
        balances: a.balances,
    }))
}

#[tauri::command]
pub async fn finance_list_accounts(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AccountDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.account_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|a| AccountDto {
        id: a.id.to_string(), org_id: a.org_id.to_string(), code: a.code, name: a.name, account_type: a.account_type,
        parent_id: a.parent_id.map(|p| p.to_string()),
        currency: a.currency, is_active: a.is_active, balances: a.balances,
    }).collect()))
}

#[tauri::command]
pub async fn finance_create_fee_template(request: CreateFeeTemplateInput, state: State<'_, AppState>) -> Result<ApiResponse<FeeTemplateDto>, String> {
    let ctx = make_ctx(&state);
    let uc = CreateFeeTemplateUseCase { fee_template_repo: state.fee_template_repo.clone() };
    let rev_id = Uuid::parse_str(&request.revenue_account_id).map_err(|_| "Invalid revenue_account_id")?;
    let rec_id = Uuid::parse_str(&request.receivable_account_id).map_err(|_| "Invalid receivable_account_id")?;

    let t = uc.execute(
        &ctx,
        ctx.tenant_id,
        request.name,
        request.default_amount,
        rev_id,
        rec_id,
        request.is_mandatory.unwrap_or(false),
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(FeeTemplateDto {
        id: t.id.to_string(),
        org_id: t.org_id.to_string(),
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
    let items = state.fee_template_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|t| FeeTemplateDto {
        id: t.id.to_string(), org_id: t.org_id.to_string(), name: t.name, default_amount: t.default_amount, revenue_account_id: t.revenue_account_id.to_string(), receivable_account_id: t.receivable_account_id.to_string(), is_mandatory: t.is_mandatory,
    }).collect()))
}

#[tauri::command]
pub async fn finance_generate_fee(request: GenerateFeeInput, state: State<'_, AppState>) -> Result<ApiResponse<StudentFeeDto>, String> {
    let ctx = make_ctx(&state);
    let uc = GenerateStudentFeeUseCase {
        student_fee_repo: state.student_fee_repo.clone(),
        fee_template_repo: state.fee_template_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid student_id")?;
    let enrollment_id = request.enrollment_id.and_then(|e| Uuid::parse_str(&e).ok());
    let fee_template_id = Uuid::parse_str(&request.fee_template_id).map_err(|_| "Invalid fee_template_id")?;
    let due_date = NaiveDate::parse_from_str(&request.due_date, "%Y-%m-%d").map_err(|_| "Invalid due_date (expected YYYY-MM-DD)")?;

    let f = uc.execute(
        &ctx,
        ctx.tenant_id,
        student_id,
        enrollment_id,
        fee_template_id,
        request.discount_amount.unwrap_or(0.0),
        due_date,
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(StudentFeeDto {
        id: f.id.to_string(),
        org_id: f.org_id.to_string(),
        student_id: f.student_id.to_string(),
        enrollment_id: f.enrollment_id.map(|e| e.to_string()),
        fee_template_id: f.fee_template_id.to_string(),
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        status: f.status,
        due_date: f.due_date.to_string(),
    }))
}

#[tauri::command]
pub async fn finance_list_student_fees(filter: Option<FeeFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<StudentFeeDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(eid) = f.enrollment_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.student_fee_repo.find_by_enrollment(eid).await
    } else if let Some(sid) = f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.student_fee_repo.find_by_student(sid).await
    } else { Ok(vec![]) }.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|f| StudentFeeDto {
        id: f.id.to_string(),
        org_id: f.org_id.to_string(),
        student_id: f.student_id.to_string(),
        enrollment_id: f.enrollment_id.map(|e| e.to_string()),
        fee_template_id: f.fee_template_id.to_string(),
        original_amount: f.original_amount,
        discount_amount: f.discount_amount,
        net_amount: f.net_amount,
        status: f.status,
        due_date: f.due_date.to_string(),
    }).collect()))
}

#[tauri::command]
pub async fn finance_process_payment(request: ProcessPaymentInput, state: State<'_, AppState>) -> Result<ApiResponse<PaymentDto>, String> {
    let ctx = make_ctx(&state);
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
    let fee_id = request.fee_id.and_then(|f| Uuid::parse_str(&f).ok());
    let account_id = Uuid::parse_str(&request.account_id).map_err(|_| "Invalid account_id")?;

    let pay = uc.execute(
        &ctx,
        ctx.tenant_id,
        fee_id,
        account_id,
        request.amount,
        request.method,
        request.reference,
        request.source,
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(PaymentDto {
        id: pay.id.to_string(),
        org_id: pay.org_id.to_string(),
        fee_id: pay.fee_id.map(|f| f.to_string()),
        account_id: pay.account_id.to_string(),
        amount: pay.amount,
        method: pay.method,
        reference: pay.reference,
        journal_entry_id: pay.journal_entry_id.map(|j| j.to_string()),
        paid_at: pay.paid_at.to_rfc3339(),
    }))
}

#[tauri::command]
pub async fn finance_list_payments(filter: Option<PaymentFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<PaymentDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let items = if let Some(fid) = f.fee_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.payment_repo.find_by_fee(fid).await
    } else {
        state.payment_repo.find_by_org(ctx.tenant_id).await
    }.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|p| PaymentDto {
        id: p.id.to_string(),
        org_id: p.org_id.to_string(),
        fee_id: p.fee_id.map(|f| f.to_string()),
        account_id: p.account_id.to_string(),
        amount: p.amount,
        method: p.method,
        reference: p.reference,
        journal_entry_id: p.journal_entry_id.map(|j| j.to_string()),
        paid_at: p.paid_at.to_rfc3339(),
    }).collect()))
}

#[tauri::command]
pub async fn finance_list_currencies(state: State<'_, AppState>) -> Result<ApiResponse<Vec<CurrencyDto>>, String> {
    let items = state.currency_repo.find_all().await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|c| CurrencyDto { id: c.id, name: c.name, symbol: c.symbol }).collect()))
}
