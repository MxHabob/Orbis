use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Types ──────────────────────────────────────────

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CreateCurrencyRequest {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub symbol: String,
    pub exchange_rate: f64,
    pub is_base: bool,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UpdateCurrencyRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub symbol: String,
    pub exchange_rate: f64,
    pub is_base: bool,
}

struct FinanceServices {
    ledger: Arc<domain_finance::services::ledger::LedgerService>,
    treasury: domain_finance::services::treasury::TreasuryService,
    fee: domain_finance::services::fee::FeeService,
    payment: domain_finance::services::payment::PaymentService,
    discount: domain_finance::services::discount::DiscountService,
}

fn get_finance_services(db: sea_orm::DatabaseConnection) -> FinanceServices {
    let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(db.clone()));
    let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(db));
    let ledger = Arc::new(domain_finance::services::ledger::LedgerService::new(cmd.clone(), query.clone()));
    FinanceServices {
        ledger: ledger.clone(),
        treasury: domain_finance::services::treasury::TreasuryService::new(cmd.clone(), query.clone(), ledger.clone()),
        fee: domain_finance::services::fee::FeeService::new(cmd.clone(), query.clone(), ledger.clone()),
        payment: domain_finance::services::payment::PaymentService::new(cmd.clone(), query.clone(), ledger),
        discount: domain_finance::services::discount::DiscountService::new(cmd, query),
    }
}

// ── Ledger Account Commands ─────────────────────────────────────────

#[tauri::command]
pub async fn get_ledger_accounts(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::services::ledger::LedgerAccountResponse>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.ledger.get_ledger_accounts(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_transaction_accounts(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::services::ledger::LedgerAccountResponse>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.ledger.get_transaction_accounts(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_ledger_account(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_finance::services::ledger::CreateLedgerAccountRequest,
) -> CmdResult<domain_finance::entities::ledger_account::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.ledger.create_ledger_account(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Treasury Commands ───────────────────────────────────────────────

#[tauri::command]
pub async fn get_treasuries(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::treasury::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.treasury.get_treasuries(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_treasury(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_finance::services::treasury::CreateTreasuryRequest,
) -> CmdResult<domain_finance::entities::treasury::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.treasury.create_treasury(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Student Fee Commands ────────────────────────────────────────────

#[tauri::command]
pub async fn get_student_fees(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    enrollment_id: Option<String>,
) -> CmdResult<Vec<domain_finance::entities::student_fee::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.fee.get_student_fees(&org_id, enrollment_id.as_deref())
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_active_fee_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::fee_template::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.fee.get_active_fee_templates(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_fee_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::fee_template::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.fee.get_fee_templates(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_fee_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_finance::services::fee::CreateFeeTemplateRequest,
) -> CmdResult<domain_finance::entities::fee_template::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.fee.create_fee_template(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_fee_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: domain_finance::services::fee::CreateFeeTemplateRequest,
) -> CmdResult<domain_finance::entities::fee_template::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.fee.update_fee_template(&org_id, &id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_fee_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    let _ = services.fee.delete_fee_template(&org_id, &id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn assign_student_fee(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_finance::services::fee::AssignStudentFeeRequest,
) -> CmdResult<domain_finance::entities::student_fee::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.fee.assign_student_fee(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Payment Commands ────────────────────────────────────────────────

#[tauri::command]
pub async fn get_payments(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::payment::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.payment.get_payments(&org_id, None)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn record_payment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    recorded_by: String,
    request: domain_finance::services::payment::RecordPaymentRequest,
) -> CmdResult<domain_finance::entities::payment::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.payment.record_payment(&org_id, request, &recorded_by)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Journal Entry Commands ──────────────────────────────────────────

#[tauri::command]
pub async fn get_journal_entries(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::journal_entry::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.ledger.get_journal_entries(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_journal_entry(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_finance::services::ledger::CreateJournalEntryRequest,
) -> CmdResult<domain_finance::entities::journal_entry::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.ledger.create_journal_entry(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Discount Commands ───────────────────────────────────────────────

#[tauri::command]
pub async fn get_active_discount_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::discount_template::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.discount.get_active_discount_templates(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_discount_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::discount_template::Model>> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.discount.get_discount_templates(&org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_discount_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_finance::services::discount::CreateDiscountTemplateRequest,
) -> CmdResult<domain_finance::entities::discount_template::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.discount.create_discount_template(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_discount_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: domain_finance::services::discount::CreateDiscountTemplateRequest,
) -> CmdResult<domain_finance::entities::discount_template::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.discount.update_discount_template(&org_id, &id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_discount_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let services = get_finance_services(db);
    let _ = services.discount.delete_discount_template(&org_id, &id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

// ── Currency Commands ───────────────────────────────────────────────

#[tauri::command]
pub async fn get_currencies(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_finance::entities::currency::Model>> {
    let db = state.active_db().await;
    domain_finance::entities::currency::Entity::find()
        .filter(domain_finance::entities::currency::Column::OrgId.eq(org_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_currency(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    code: String,
) -> CmdResult<Option<domain_finance::entities::currency::Model>> {
    let db = state.active_db().await;
    domain_finance::entities::currency::Entity::find()
        .filter(domain_finance::entities::currency::Column::OrgId.eq(org_id))
        .filter(domain_finance::entities::currency::Column::Code.eq(code))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_currency(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateCurrencyRequest,
) -> CmdResult<domain_finance::entities::currency::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let model = domain_finance::entities::currency::Model {
        org_id,
        code: request.code,
        name: request.name,
        name_ar: request.name_ar,
        symbol: request.symbol,
        symbol_position: "before".to_string(),
        decimal_places: 2,
        exchange_rate: request.exchange_rate,
        is_base: if request.is_base { 1 } else { 0 },
        is_active: 1,
        updated_at: now,
    };
    let active: domain_finance::entities::currency::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_currency(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    code: String,
    request: UpdateCurrencyRequest,
) -> CmdResult<domain_finance::entities::currency::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let curr = domain_finance::entities::currency::Entity::find()
        .filter(domain_finance::entities::currency::Column::OrgId.eq(org_id))
        .filter(domain_finance::entities::currency::Column::Code.eq(code))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Currency not found".to_string()))?;
        
    let mut active: domain_finance::entities::currency::ActiveModel = curr.into();
    active.name = Set(request.name);
    active.name_ar = Set(request.name_ar);
    active.symbol = Set(request.symbol);
    active.exchange_rate = Set(request.exchange_rate);
    active.is_base = Set(if request.is_base { 1 } else { 0 });
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_currency(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    code: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let curr = domain_finance::entities::currency::Entity::find()
        .filter(domain_finance::entities::currency::Column::OrgId.eq(org_id))
        .filter(domain_finance::entities::currency::Column::Code.eq(code))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Currency not found".to_string()))?;
        
    let _ = domain_finance::entities::currency::Entity::delete_by_id(curr.code)
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

// ── Account Statement Commands ──────────────────────────────────────

#[tauri::command]
pub async fn get_account_statement(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    account_id: String,
    start_date: String,
    end_date: String,
) -> CmdResult<domain_finance::services::ledger::AccountStatement> {
    let db = state.active_db().await;
    let services = get_finance_services(db);
    services.ledger.get_account_statement(&org_id, &account_id, &start_date, &end_date)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn seed_default_financials(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    
    let default_accounts = vec![
        ("110301", "Student Accounts Receivable", Some("ذمم الطلاب المدنية")),
        ("210401", "Employee Accounts Payable", Some("ذمم الموظفين الدائنة")),
        ("110101", "Treasury Cash", Some("نقدية الصناديق")),
        ("110102", "Treasury Bank", Some("نقدية البنوك")),
        ("210106", "Vendor Accounts Payable", Some("ذمم الموردين الدائنة")),
        ("310101", "Partner Accounts", Some("حسابات الشركاء")),
        ("110308", "Guardian Accounts", Some("حسابات أولياء الأمور")),
        ("110604", "Branch Accounts", Some("حسابات الفروع")),
    ];

    for (code, name, name_ar) in default_accounts {
        let id = format!("{}_{}", org_id, code);
        let existing = domain_finance::entities::ledger_account::Entity::find_by_id(&id)
            .one(&db)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;
            
        if existing.is_none() {
            let model = domain_finance::entities::ledger_account::Model {
                id,
                org_id: org_id.clone(),
                code: code.to_string(),
                name: name.to_string(),
                name_ar: name_ar.map(|s| s.to_string()),
                item_type: "asset".to_string(),
                sub_type: None,
                parent_id: None,
                path: code.to_string(),
                depth: 0,
                currency: "USD".to_string(),
                opening_balance: Some(0.0),
                current_balance: Some(0.0),
                entity_type: None,
                entity_id: None,
                is_control_account: 1,
                is_system_account: 1,
                is_active: 1,
                created_at: now.clone(),
                updated_at: now.clone(),
            };
            let active: domain_finance::entities::ledger_account::ActiveModel = model.into();
            let _ = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
        }
    }
    
    Ok(())
}
