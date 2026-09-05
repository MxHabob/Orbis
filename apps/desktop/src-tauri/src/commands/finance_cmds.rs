//! Tauri IPC commands — Finance (Accounts, FeeTemplates, StudentFees, Payments, Currencies).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct AccountDto { pub id: String, pub org_id: String, pub code: String, pub name: String, pub account_type: String, pub balance: f64 }
#[derive(Serialize)]
pub struct FeeTemplateDto { pub id: String, pub org_id: String, pub name: String, pub amount: f64, pub fee_type: String, pub is_active: bool }
#[derive(Serialize)]
pub struct StudentFeeDto { pub id: String, pub student_id: String, pub enrollment_id: Option<String>, pub template_id: String, pub net_amount: f64, pub paid_amount: f64, pub status: String }
#[derive(Serialize)]
pub struct PaymentDto { pub id: String, pub fee_id: Option<String>, pub amount: f64, pub method: String, pub status: String, pub paid_at: Option<String> }
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
pub async fn finance_list_accounts(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AccountDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.account_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|a| AccountDto {
        id: a.id.to_string(), org_id: a.org_id.to_string(), code: a.code, name: a.name, account_type: a.account_type, balance: a.balance,
    }).collect()))
}

#[tauri::command]
pub async fn finance_list_fee_templates(state: State<'_, AppState>) -> Result<ApiResponse<Vec<FeeTemplateDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.fee_template_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|t| FeeTemplateDto {
        id: t.id.to_string(), org_id: t.org_id.to_string(), name: t.name, amount: t.amount, fee_type: t.fee_type, is_active: t.is_active,
    }).collect()))
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
        id: f.id.to_string(), student_id: f.student_id.to_string(),
        enrollment_id: f.enrollment_id.map(|e| e.to_string()), template_id: f.template_id.to_string(),
        net_amount: f.net_amount, paid_amount: f.paid_amount, status: f.status,
    }).collect()))
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
        id: p.id.to_string(), fee_id: p.fee_id.map(|f| f.to_string()),
        amount: p.amount, method: p.method, status: p.status,
        paid_at: p.paid_at.map(|d| d.to_rfc3339()),
    }).collect()))
}

#[tauri::command]
pub async fn finance_list_currencies(state: State<'_, AppState>) -> Result<ApiResponse<Vec<CurrencyDto>>, String> {
    let items = state.currency_repo.find_all().await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|c| CurrencyDto { id: c.id, name: c.name, symbol: c.symbol }).collect()))
}
