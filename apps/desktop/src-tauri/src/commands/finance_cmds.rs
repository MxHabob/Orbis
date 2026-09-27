//! Tauri IPC commands — Finance (Accounts, FeeTemplates, StudentFees, Payments, Currencies, JournalEntries, DiscountTemplates).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use orbis_domain::entities::account::{Account, AccountType};
use orbis_domain::entities::payment::{Payment, PaymentMethod};
use orbis_domain::entities::fee_template::{FeeTemplate, FeeFrequency};
use orbis_domain::entities::student_fee::StudentFee;
use orbis_domain::entities::discount_template::{DiscountTemplate, DiscountType};
use orbis_domain::entities::journal_entry::JournalEntry;
use orbis_domain::queries::filters::{PaymentFilter, StudentFeeFilter, PaginationParams};
use super::make_ctx;
use super::org_cmds::ApiResponse;

fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

// ── DTOs ─────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountDto {
    pub id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub account_type: String,
    pub parent_id: Option<String>,
    pub currency: String,
    pub is_active: bool,
    pub balances: serde_json::Value,
}

pub fn account_to_dto(a: Account) -> AccountDto {
    AccountDto {
        id: a.id.to_string(),
        org_id: a.org_id.to_string(),
        code: a.code,
        name: a.name,
        name_ar: a.name_ar,
        account_type: a.account_type.to_string(),
        parent_id: a.parent_id.map(|p| p.to_string()),
        currency: a.currency_code,
        is_active: a.is_active,
        balances: a.balances,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeeTemplateDto {
    pub id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: String,
    pub default_amount: Decimal,
    pub currency: String,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub applicable_levels: Option<String>,
    pub frequency: String,
    pub is_mandatory: bool,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub fn fee_template_to_dto(t: FeeTemplate) -> FeeTemplateDto {
    let applicable_levels = t.applicable_levels.map(|levels| {
        serde_json::to_string(&serde_json::json!({ "levels": levels })).unwrap_or_default()
    });

    FeeTemplateDto {
        id: t.id.to_string(),
        org_id: t.org_id.to_string(),
        code: t.code,
        name: t.name,
        name_ar: t.name_ar,
        category: t.category,
        default_amount: t.default_amount,
        currency: t.currency_code,
        revenue_account_id: t.revenue_account_id.to_string(),
        receivable_account_id: t.receivable_account_id.to_string(),
        applicable_levels,
        frequency: t.frequency.to_string(),
        is_mandatory: t.is_mandatory,
        is_active: t.is_active,
        created_at: t.created_at.to_rfc3339(),
        updated_at: t.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn student_fee_to_dto(f: StudentFee) -> StudentFeeDto {
    StudentFeeDto {
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
        notes: f.notes,
        created_at: f.created_at.to_rfc3339(),
        updated_at: f.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentDto {
    pub id: String,
    pub org_id: String,
    pub fee_id: Option<String>,
    pub student_id: Option<String>,
    pub enrollment_id: Option<String>,
    pub account_id: String,
    pub amount: Decimal,
    pub currency: String,
    pub exchange_rate: Decimal,
    pub base_amount: Decimal,
    pub method: String,
    pub reference: Option<String>,
    pub receipt_number: Option<String>,
    pub journal_entry_id: Option<String>,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub notes: Option<String>,
    pub paid_at: String,
    pub created_by: Option<String>,
    pub created_at: String,
}

pub fn payment_to_dto(p: Payment) -> PaymentDto {
    PaymentDto {
        id: p.id.to_string(),
        org_id: p.org_id.to_string(),
        fee_id: p.fee_id.map(|f| f.to_string()),
        student_id: p.student_id.map(|s| s.to_string()),
        enrollment_id: p.enrollment_id.map(|e| e.to_string()),
        account_id: p.account_id.to_string(),
        amount: p.amount,
        currency: p.currency_code,
        exchange_rate: p.exchange_rate,
        base_amount: p.base_amount,
        method: p.method.to_string(),
        reference: p.reference,
        receipt_number: p.receipt_number,
        journal_entry_id: p.journal_entry_id.map(|j| j.to_string()),
        payer_name: p.payer_name,
        payer_phone: p.payer_phone,
        notes: p.notes,
        paid_at: p.paid_at.to_rfc3339(),
        created_by: p.created_by.map(|c| c.to_string()),
        created_at: p.created_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalEntryLineDto {
    pub id: String,
    pub account_id: String,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalEntryDto {
    pub id: String,
    pub org_id: String,
    pub reference_number: String,
    pub accounting_date: String,
    pub amount: Decimal,
    pub currency: String,
    pub exchange_rate: Decimal,
    pub base_amount: Decimal,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub description: String,
    pub description_ar: Option<String>,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: String,
    pub lines: Vec<JournalEntryLineDto>,
    pub is_reversed: bool,
    pub reversed_by: Option<String>,
    pub reversal_reason: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
}

pub fn journal_entry_to_dto(j: JournalEntry) -> JournalEntryDto {
    JournalEntryDto {
        id: j.id.to_string(),
        org_id: j.org_id.to_string(),
        reference_number: j.reference_number,
        accounting_date: j.accounting_date.to_string(),
        amount: j.amount,
        currency: j.currency_code,
        exchange_rate: j.exchange_rate,
        base_amount: j.base_amount,
        debit_account_id: j.debit_account_id.to_string(),
        credit_account_id: j.credit_account_id.to_string(),
        description: j.description,
        description_ar: j.description_ar,
        source_type: j.source_type,
        source_id: j.source_id.map(|s| s.to_string()),
        status: j.status.to_string(),
        lines: j.lines.into_iter().map(|l| JournalEntryLineDto {
            id: l.id.to_string(),
            account_id: l.account_id.to_string(),
            debit_amount: l.debit_amount,
            credit_amount: l.credit_amount,
            description: l.description,
            entity_type: l.entity_type,
            entity_id: l.entity_id.map(|e| e.to_string()),
        }).collect(),
        is_reversed: j.is_reversed,
        reversed_by: j.reversed_by.map(|r| r.to_string()),
        reversal_reason: j.reversal_reason,
        created_by: j.created_by.map(|c| c.to_string()),
        created_at: j.created_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscountTemplateDto {
    pub id: String,
    pub org_id: String,
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
    pub created_at: String,
    pub updated_at: String,
}

pub fn discount_template_to_dto(d: DiscountTemplate) -> DiscountTemplateDto {
    DiscountTemplateDto {
        id: d.id.to_string(),
        org_id: d.org_id.to_string(),
        code: d.code,
        name: d.name,
        name_ar: d.name_ar,
        discount_type: match d.discount_type {
            DiscountType::Percentage => "percentage".into(),
            DiscountType::FixedAmount => "fixed_amount".into(),
        },
        value: d.value,
        eligibility_rules: d.eligibility_rules,
        is_approval_required: d.is_approval_required,
        is_auto_apply: d.is_auto_apply,
        is_stackable: d.is_stackable,
        priority: d.priority,
        is_active: d.is_active,
        created_at: d.created_at.to_rfc3339(),
        updated_at: d.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurrencyDto {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub symbol: String,
    pub symbol_position: String,
    pub decimal_places: u32,
    pub is_base: bool,
    pub is_active: bool,
    pub exchange_rate: f64,
}

// ── Request Inputs ───────────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct CreateAccountInput {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub account_type: String,
    pub parent_id: Option<String>,
    pub currency: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateFeeTemplateInput {
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: Option<String>,
    pub default_amount: Decimal,
    pub currency: Option<String>,
    pub revenue_account_id: Option<String>,
    pub receivable_account_id: Option<String>,
    pub applicable_levels: Option<serde_json::Value>,
    pub frequency: Option<String>,
    pub is_mandatory: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateFeeTemplateInput {
    pub id: String,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: Option<String>,
    pub default_amount: Decimal,
    pub currency: Option<String>,
    pub revenue_account_id: Option<String>,
    pub receivable_account_id: Option<String>,
    pub applicable_levels: Option<serde_json::Value>,
    pub frequency: Option<String>,
    pub is_mandatory: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct GenerateFeeInput {
    pub period_id: String,
    pub student_id: String,
    pub enrollment_id: Option<String>,
    pub fee_template_id: String,
    pub discount_amount: Option<Decimal>,
    pub due_date: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct FeeFilterInput {
    pub period_id: Option<String>,
    pub student_id: Option<String>,
    pub enrollment_id: Option<String>,
    pub fee_template_id: Option<String>,
    pub status: Option<String>,
    pub is_overdue: Option<bool>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Default, Debug)]
pub struct PaymentFilterInput {
    pub fee_id: Option<String>,
    pub student_id: Option<String>,
    pub enrollment_id: Option<String>,
    pub method: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateDiscountTemplateInput {
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub discount_type: String,
    pub value: Decimal,
    pub eligibility_rules: Option<serde_json::Value>,
    #[serde(default)]
    pub is_approval_required: bool,
    #[serde(default)]
    pub is_auto_apply: bool,
    #[serde(default)]
    pub is_stackable: bool,
    #[serde(default)]
    pub priority: u32,
}

// ── Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn finance_create_account(
    request: CreateAccountInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AccountDto>, String> {
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

    Ok(ok(account_to_dto(a)))
}

#[tauri::command]
pub async fn finance_list_accounts(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AccountDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.finance_service.list_accounts(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(account_to_dto).collect()))
}

fn parse_levels_from_json(val: Option<serde_json::Value>) -> Option<Vec<String>> {
    let val = val?;
    match val {
        serde_json::Value::Array(arr) => Some(arr.into_iter().filter_map(|v| v.as_str().map(String::from)).collect()),
        serde_json::Value::Object(obj) => {
            obj.get("levels").and_then(|l| l.as_array()).map(|arr| {
                arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()
            })
        }
        serde_json::Value::String(s) => {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&s) {
                match parsed {
                    serde_json::Value::Array(arr) => Some(arr.into_iter().filter_map(|v| v.as_str().map(String::from)).collect()),
                    serde_json::Value::Object(obj) => {
                        obj.get("levels").and_then(|l| l.as_array()).map(|arr| {
                            arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()
                        })
                    }
                    _ => None,
                }
            } else if !s.trim().is_empty() {
                Some(vec![s])
            } else {
                None
            }
        }
        _ => None,
    }
}

#[tauri::command]
pub async fn finance_create_fee_template(
    request: CreateFeeTemplateInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FeeTemplateDto>, String> {
    let ctx = make_ctx(&state);
    let accounts = state.finance_service.list_accounts(ctx.tenant_id).await.map_err(|e| e.to_string())?;

    let rec_id = if let Some(ref s) = request.receivable_account_id {
        Uuid::parse_str(s).map_err(|_| "Invalid receivable_account_id")?
    } else {
        accounts.iter().find(|a| a.code == "110301" || a.code == "1103" || a.code.starts_with("1103"))
            .map(|a| a.id)
            .unwrap_or_else(Uuid::new_v4)
    };

    let rev_id = if let Some(ref s) = request.revenue_account_id {
        Uuid::parse_str(s).map_err(|_| "Invalid revenue_account_id")?
    } else {
        accounts.iter().find(|a| a.code == "410301" || a.code == "410101" || a.code.starts_with("41"))
            .map(|a| a.id)
            .unwrap_or_else(Uuid::new_v4)
    };

    let currency = request.currency.unwrap_or_else(|| "YER".to_string());
    let freq = match request.frequency.as_deref().unwrap_or("yearly").to_lowercase().as_str() {
        "once" => FeeFrequency::Once,
        "monthly" => FeeFrequency::Monthly,
        "termly" | "per_term" => FeeFrequency::PerTerm,
        _ => FeeFrequency::Annual,
    };

    let tmpl_id = Uuid::new_v4();
    let code = request.code.unwrap_or_else(|| format!("FEE-{}", tmpl_id.to_string().chars().take(6).collect::<String>()));
    let levels = parse_levels_from_json(request.applicable_levels);
    let now = chrono::Utc::now();

    let tmpl = FeeTemplate {
        id: tmpl_id,
        org_id: ctx.tenant_id,
        code,
        name: request.name,
        name_ar: request.name_ar,
        category: request.category.unwrap_or_else(|| "tuition".to_string()),
        default_amount: request.default_amount,
        currency_code: currency,
        revenue_account_id: rev_id,
        receivable_account_id: rec_id,
        applicable_levels: levels,
        applicable_study_modes: None,
        frequency: freq,
        is_mandatory: request.is_mandatory.unwrap_or(true),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    state.finance_service.fee_template_repo.save(&tmpl).await.map_err(|e| e.to_string())?;
    Ok(ok(fee_template_to_dto(tmpl)))
}

#[tauri::command]
pub async fn finance_update_fee_template(
    request: UpdateFeeTemplateInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FeeTemplateDto>, String> {
    let ctx = make_ctx(&state);
    let id = Uuid::parse_str(&request.id).map_err(|_| "Invalid fee template id")?;

    let mut tmpl = state.finance_service.fee_template_repo.find_by_id(id).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Fee template not found".to_string())?;

    if tmpl.org_id != ctx.tenant_id {
        return Err("Unauthorized".to_string());
    }

    tmpl.name = request.name;
    tmpl.name_ar = request.name_ar;
    if let Some(c) = request.code {
        if !c.trim().is_empty() {
            tmpl.code = c;
        }
    }
    if let Some(cat) = request.category {
        tmpl.category = cat;
    }
    tmpl.default_amount = request.default_amount;
    if let Some(cur) = request.currency {
        tmpl.currency_code = cur;
    }
    if let Some(rev) = request.revenue_account_id.and_then(|s| Uuid::parse_str(&s).ok()) {
        tmpl.revenue_account_id = rev;
    }
    if let Some(rec) = request.receivable_account_id.and_then(|s| Uuid::parse_str(&s).ok()) {
        tmpl.receivable_account_id = rec;
    }
    if let Some(freq) = request.frequency {
        tmpl.frequency = match freq.to_lowercase().as_str() {
            "once" => FeeFrequency::Once,
            "monthly" => FeeFrequency::Monthly,
            "termly" | "per_term" => FeeFrequency::PerTerm,
            _ => FeeFrequency::Annual,
        };
    }
    if let Some(mand) = request.is_mandatory {
        tmpl.is_mandatory = mand;
    }

    if let Some(levels_val) = request.applicable_levels {
        tmpl.applicable_levels = parse_levels_from_json(Some(levels_val));
    } else {
        tmpl.applicable_levels = None;
    }

    tmpl.updated_at = chrono::Utc::now();
    state.finance_service.fee_template_repo.save(&tmpl).await.map_err(|e| e.to_string())?;

    Ok(ok(fee_template_to_dto(tmpl)))
}

#[tauri::command]
pub async fn finance_delete_fee_template(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let template_id = Uuid::parse_str(&id).map_err(|_| "Invalid fee template id")?;
    state.finance_service.delete_fee_template(&ctx, ctx.tenant_id, template_id).await.map_err(|e| e.to_string())?;
    Ok(ok(true))
}

#[tauri::command]
pub async fn finance_list_fee_templates(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<FeeTemplateDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.finance_service.list_fee_templates(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(fee_template_to_dto).collect()))
}

#[tauri::command]
pub async fn finance_generate_fee(
    request: GenerateFeeInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<StudentFeeDto>, String> {
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

    Ok(ok(student_fee_to_dto(f)))
}

#[tauri::command]
pub async fn finance_list_student_fees(
    filter: Option<FeeFilterInput>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<StudentFeeDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let query_filter = StudentFeeFilter {
        student_id: f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        enrollment_id: f.enrollment_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        fee_template_id: f.fee_template_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        status: f.status,
        is_overdue: f.is_overdue,
        pagination: Some(PaginationParams {
            page: f.page.or(Some(1)),
            page_size: f.page_size.or(Some(1000)),
            sort_by: f.sort_by,
            sort_dir: f.sort_dir,
        }),
    };

    let paginated = state.finance_service
        .list_student_fees(ctx.tenant_id, query_filter)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(paginated.items.into_iter().map(student_fee_to_dto).collect()))
}

#[tauri::command]
pub async fn finance_process_payment(
    request: ProcessPaymentInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<PaymentDto>, String> {
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

    Ok(ok(payment_to_dto(pay)))
}

#[tauri::command]
pub async fn finance_list_payments(
    filter: Option<PaymentFilterInput>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<PaymentDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let from_date = f.from_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .and_then(|nd| nd.and_hms_opt(0, 0, 0))
        .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc));
    let to_date = f.to_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .and_then(|nd| nd.and_hms_opt(23, 59, 59))
        .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc));
    let method = match f.method.as_deref() {
        Some("cash") => Some(PaymentMethod::Cash),
        Some("bank_transfer") | Some("bank") => Some(PaymentMethod::BankTransfer),
        Some("check") | Some("cheque") => Some(PaymentMethod::Check),
        Some("credit_card") | Some("card") => Some(PaymentMethod::CreditCard),
        Some("mada") => Some(PaymentMethod::Mada),
        Some("apple_pay") => Some(PaymentMethod::ApplePay),
        _ => None,
    };

    let query_filter = PaymentFilter {
        student_id: f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        enrollment_id: f.enrollment_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        fee_id: f.fee_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()),
        method,
        from_date,
        to_date,
        pagination: Some(PaginationParams {
            page: f.page.or(Some(1)),
            page_size: f.page_size.or(Some(1000)),
            sort_by: f.sort_by,
            sort_dir: f.sort_dir,
        }),
    };

    let paginated = state.finance_service
        .list_payments(ctx.tenant_id, query_filter)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(paginated.items.into_iter().map(payment_to_dto).collect()))
}

#[tauri::command]
pub async fn finance_list_currencies(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<CurrencyDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.currency_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|c| CurrencyDto {
        code: c.code,
        name: c.name,
        name_ar: c.name_ar,
        symbol: c.symbol,
        symbol_position: c.symbol_position,
        decimal_places: c.decimal_places,
        is_base: c.is_base,
        is_active: c.is_active,
        exchange_rate: if c.is_base { 1.0 } else { 1.0 },
    }).collect()))
}

#[tauri::command]
pub async fn finance_list_journal_entries(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<JournalEntryDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.journal_entry_repo
        .find_by_org(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(journal_entry_to_dto).collect()))
}

#[tauri::command]
pub async fn finance_get_journal_entry(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<JournalEntryDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let entry = state.journal_entry_repo
        .find_by_id(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Journal entry not found".to_string())?;

    Ok(ok(journal_entry_to_dto(entry)))
}

#[tauri::command]
pub async fn finance_list_discount_templates(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DiscountTemplateDto>>, String> {
    let ctx = make_ctx(&state);
    let mut items = state.discount_template_repo
        .find_by_org(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    if items.is_empty() {
        let now = chrono::Utc::now().to_rfc3339();
        let _ = orbis_infra::adapters::bootstrap_adapter::seed_discount_templates(&state.db, ctx.tenant_id, &now).await;
        if let Ok(reloaded) = state.discount_template_repo.find_by_org(ctx.tenant_id).await {
            items = reloaded;
        }
    }

    Ok(ok(items.into_iter().map(discount_template_to_dto).collect()))
}

#[tauri::command]
pub async fn finance_create_discount_template(
    request: CreateDiscountTemplateInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DiscountTemplateDto>, String> {
    let ctx = make_ctx(&state);
    let now = chrono::Utc::now();
    let tmpl_id = Uuid::new_v4();

    let discount_type = match request.discount_type.to_lowercase().as_str() {
        "percentage" | "percent" => DiscountType::Percentage,
        _ => DiscountType::FixedAmount,
    };

    let tmpl = DiscountTemplate {
        id: tmpl_id,
        org_id: ctx.tenant_id,
        code: request.code.unwrap_or_else(|| format!("DISC-{}", tmpl_id.to_string().chars().take(6).collect::<String>())),
        name: request.name,
        name_ar: request.name_ar,
        discount_type,
        value: request.value,
        eligibility_rules: request.eligibility_rules,
        is_approval_required: request.is_approval_required,
        is_auto_apply: request.is_auto_apply,
        is_stackable: request.is_stackable,
        priority: request.priority,
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    state.discount_template_repo.save(&tmpl).await.map_err(|e| e.to_string())?;
    Ok(ok(discount_template_to_dto(tmpl)))
}
