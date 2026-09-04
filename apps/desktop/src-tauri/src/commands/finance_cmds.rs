use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::finance::{
    create_account::CreateAccountUseCase,
    create_fee_template::CreateFeeTemplateUseCase,
    generate_student_fee::GenerateStudentFeeUseCase,
    process_payment::ProcessPaymentUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;
use chrono::NaiveDate;

#[tauri::command]
pub async fn create_account(
    org_id: String,
    code: String,
    name: String,
    account_type: String,
    parent_id: Option<String>,
    currency: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_parent_id = match parent_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|_| "Invalid parent ID")?),
        None => None,
    };

    let ctx = RequestContext { tenant_id: parsed_org_id, actor_id: Uuid::new_v4(), request_id: Uuid::new_v4().to_string() };
    
    let usecase = CreateAccountUseCase { account_repo: state.account_repo.clone() };
    
    match usecase.execute(&ctx, parsed_org_id, code, name, account_type, parsed_parent_id, currency).await {
        Ok(account) => Ok(account.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn create_fee_template(
    org_id: String,
    name: String,
    default_amount: f64,
    revenue_account_id: String,
    receivable_account_id: String,
    is_mandatory: bool,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let rev_acc = Uuid::parse_str(&revenue_account_id).map_err(|_| "Invalid revenue account ID")?;
    let rec_acc = Uuid::parse_str(&receivable_account_id).map_err(|_| "Invalid receivable account ID")?;

    let ctx = RequestContext { tenant_id: parsed_org_id, actor_id: Uuid::new_v4(), request_id: Uuid::new_v4().to_string() };
    
    let usecase = CreateFeeTemplateUseCase { fee_template_repo: state.fee_template_repo.clone() };
    
    match usecase.execute(&ctx, parsed_org_id, name, default_amount, rev_acc, rec_acc, is_mandatory).await {
        Ok(template) => Ok(template.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn generate_student_fee(
    org_id: String,
    enrollment_id: String,
    fee_template_id: String,
    discount_amount: f64,
    due_date: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let enr_id = Uuid::parse_str(&enrollment_id).map_err(|_| "Invalid enrollment ID")?;
    let fee_tpl_id = Uuid::parse_str(&fee_template_id).map_err(|_| "Invalid fee template ID")?;
    let d_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d").map_err(|_| "Invalid date")?;

    let ctx = RequestContext { tenant_id: parsed_org_id, actor_id: Uuid::new_v4(), request_id: Uuid::new_v4().to_string() };
    
    let usecase = GenerateStudentFeeUseCase { 
        student_fee_repo: state.student_fee_repo.clone(),
        fee_template_repo: state.fee_template_repo.clone(),
        event_bus: state.event_bus.clone()
    };
    
    match usecase.execute(&ctx, parsed_org_id, enr_id, fee_tpl_id, discount_amount, d_date).await {
        Ok(fee) => Ok(fee.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn process_payment(
    org_id: String,
    fee_id: Option<String>,
    account_id: String,
    amount: f64,
    method: String,
    reference: Option<String>,
    source: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_fee_id = match fee_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|_| "Invalid fee ID")?),
        None => None,
    };
    let acc_id = Uuid::parse_str(&account_id).map_err(|_| "Invalid account ID")?;

    let ctx = RequestContext { tenant_id: parsed_org_id, actor_id: Uuid::new_v4(), request_id: Uuid::new_v4().to_string() };
    
    let usecase = ProcessPaymentUseCase { 
        payment_repo: state.payment_repo.clone(),
        journal_repo: state.journal_entry_repo.clone(),
        student_fee_repo: state.student_fee_repo.clone(),
        fee_template_repo: state.fee_template_repo.clone(),
        event_bus: state.event_bus.clone(),
        payment_gateway: state.integration_resolver.clone(),
        notification_gateway: state.integration_resolver.clone(),
    };
    
    match usecase.execute(&ctx, parsed_org_id, parsed_fee_id, acc_id, amount, method, reference, source).await {
        Ok(payment) => Ok(payment.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
