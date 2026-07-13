//! Ledger account operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait};

use uuid::Uuid;
use serde::{Deserialize, Serialize};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};
use svc_iam::entities::user;

use crate::contracts::commands::{CreateLedgerAccountCmd, CreateJournalEntryCmd};
use crate::contracts::events::event_types;
use crate::entities::{account, journal_entry};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LedgerAccountResponse {
    pub id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String,
    pub sub_type: Option<String>,
    pub parent_id: Option<String>,
    pub path: String,
    pub depth: i32,
    pub currency: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub is_control_account: i32,
    pub is_system_account: i32,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
    pub balances: shared_kernel::currency::CurrencyBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AccountStatement {
    pub account_id: String,
    pub account_name: String,
    pub account_code: String,
    pub currency: String,
    pub start_date: String,
    pub end_date: String,
    pub opening_balance: f64,
    pub closing_balance: f64,
    pub items: Vec<StatementItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct StatementItem {
    pub id: String,
    pub date: String,
    pub reference: String,
    pub description: String,
    pub description_ar: Option<String>,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
    pub running_balance: f64,
}

pub async fn list_accounts(ctx: &ServiceContext) -> Result<Vec<LedgerAccountResponse>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;

    let accounts = account::Entity::find()
        .filter(account::Column::OrgId.eq(&ctx.org_id))
        .filter(account::Column::IsActive.eq(1))
        .order_by_asc(account::Column::Code)
        .all(&ctx.db).await?;

    let mut result: Vec<LedgerAccountResponse> = accounts.into_iter().filter(|acc| acc.depth < 4).map(|acc| {
        let balances: shared_kernel::currency::CurrencyBalance = serde_json::from_value(acc.balances.clone()).unwrap_or_default();
        LedgerAccountResponse {
            id: acc.id, org_id: acc.org_id, code: acc.code,
            name: acc.name, name_ar: acc.name_ar, item_type: acc.item_type,
            sub_type: acc.sub_type, parent_id: acc.parent_id, path: acc.path,
            depth: acc.depth, currency: acc.currency, entity_type: acc.entity_type, entity_id: acc.entity_id,
            is_control_account: acc.is_control_account, is_system_account: acc.is_system_account,
            is_active: acc.is_active, created_at: acc.created_at, updated_at: acc.updated_at,
            balances,
        }
    }).collect();

    result.sort_by(|a, b| a.code.cmp(&b.code));
    Ok(result)
}



pub async fn list_transaction_accounts(ctx: &ServiceContext) -> Result<Vec<LedgerAccountResponse>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;

    let accounts = account::Entity::find()
        .filter(account::Column::OrgId.eq(&ctx.org_id))
        .filter(account::Column::IsActive.eq(1))
        .filter(account::Column::IsControlAccount.eq(0))
        .order_by_asc(account::Column::Code)
        .all(&ctx.db).await?;

    let mut result: Vec<LedgerAccountResponse> = accounts.into_iter().filter(|acc| acc.depth < 4).map(|acc| {
        let balances: shared_kernel::currency::CurrencyBalance = serde_json::from_value(acc.balances.clone()).unwrap_or_default();
        LedgerAccountResponse {
            id: acc.id, org_id: acc.org_id, code: acc.code,
            name: acc.name, name_ar: acc.name_ar, item_type: acc.item_type,
            sub_type: acc.sub_type, parent_id: acc.parent_id, path: acc.path,
            depth: acc.depth, currency: acc.currency, entity_type: acc.entity_type, entity_id: acc.entity_id,
            is_control_account: acc.is_control_account, is_system_account: acc.is_system_account,
            is_active: acc.is_active, created_at: acc.created_at, updated_at: acc.updated_at,
            balances,
        }
    }).collect();

    result.sort_by(|a, b| a.code.cmp(&b.code));
    Ok(result)
}

pub async fn create_account(
    ctx: &ServiceContext,
    cmd: CreateLedgerAccountCmd,
) -> Result<account::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let parent = if let Some(ref pid) = cmd.parent_id {
        if pid.is_empty() {
            None
        } else {
            let p = account::Entity::find_by_id(pid.clone())
                .one(&ctx.db).await?
                .ok_or_else(|| AppError::not_found("account", pid))?;
            Some(p)
        }
    } else if let Some(ref sub_type) = cmd.sub_type {
        let parent_code = match sub_type.to_lowercase().as_str() {
            "vendor" => "210106",
            "partner" => "310101",
            "employee" => "210401",
            "student" => "110301",
            "guardian" => "110308",
            _ => return Err(AppError::validation(format!("Unsupported sub_type for auto parent: {}", sub_type))),
        };
        let parent_id = format!("{}_{}", ctx.org_id, parent_code);
        let p = account::Entity::find_by_id(parent_id.clone())
            .one(&ctx.db).await?
            .ok_or_else(|| AppError::validation(format!(
                "Parent control account '{}' not found. Did you seed default financials?", parent_code
            )))?;
        Some(p)
    } else {
        None
    };

    let final_code = if cmd.code.is_empty() {
        if let Some(ref p) = parent {
            let children = account::Entity::find()
                .filter(account::Column::OrgId.eq(&ctx.org_id))
                .filter(account::Column::ParentId.eq(Some(p.id.clone())))
                .all(&ctx.db).await?;

            let next_suffix = if children.is_empty() {
                1
            } else {
                let mut max_suf = 0;
                for child in children {
                    if child.code.starts_with(&p.code) && child.code.len() > p.code.len() {
                        let suffix_str = &child.code[p.code.len()..];
                        if let Ok(num) = suffix_str.parse::<u32>() {
                            if num > max_suf {
                                max_suf = num;
                            }
                        }
                    }
                }
                max_suf + 1
            };
            format!("{}{:05}", p.code, next_suffix)
        } else {
            return Err(AppError::validation("Account code cannot be empty if there is no parent account"));
        }
    } else {
        cmd.code.clone()
    };

    let (parent_id, path, depth, currency, item_type) = if let Some(ref p) = parent {
        (Some(p.id.clone()), format!("{}/{}", p.path, final_code), p.depth + 1, p.currency.clone(), p.item_type.clone())
    } else {
        (None, format!("{}_{}", ctx.org_id, final_code), 0, cmd.currency.clone(), cmd.item_type.clone())
    };

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let model = account::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), code: final_code,
        name: cmd.name, name_ar: cmd.name_ar, item_type, sub_type: cmd.sub_type,
        parent_id, path, depth, currency,
        balances: serde_json::to_value(shared_kernel::currency::CurrencyBalance::new()).unwrap(),
        entity_type: None, entity_id: None,
        is_control_account: 0, is_system_account: 0, is_active: 1,
        created_at: now.clone(), updated_at: now,
    };

    let active: account::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ACCOUNT_CREATED.into(),
        entity_type: "account".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "account.create".into(), entity_type: "account".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn get_or_create_sub_account(
    ctx: &ServiceContext,
    entity_type: &str,
    entity_id: &str,
    entity_name: Option<&str>,
    entity_name_ar: Option<&str>,
) -> Result<account::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let parent_code = match entity_type {
        "student" => "110301",
        "employee" => "210401",
        "treasury_cash" => "110101",
        "treasury_bank" => "110102",
        "vendor" => "210106",
        "partner" => "310101",
        "guardian" => "110308",
        "branch" => "110604",
        _ => return Err(AppError::validation(format!("Unsupported sub-ledger entity type: {}", entity_type))),
    };

    if let Some(existing) = account::Entity::find()
        .filter(account::Column::OrgId.eq(&ctx.org_id))
        .filter(account::Column::EntityType.eq(Some(entity_type.to_string())))
        .filter(account::Column::EntityId.eq(Some(entity_id.to_string())))
        .one(&ctx.db)
        .await? 
    {
        return Ok(existing);
    }

    let parent_id = format!("{}_{}", ctx.org_id, parent_code);
    let parent = account::Entity::find_by_id(parent_id.clone())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::validation(format!(
            "Parent control account '{}' not found in chart of accounts. Did you run seeding?", parent_code
        )))?;

    let (resolved_name, resolved_name_ar) = if entity_type == "student" || entity_type == "employee" || entity_type == "guardian" {
        let usr = user::Entity::find_by_id(entity_id.to_string())
            .one(&ctx.db)
            .await?
            .ok_or_else(|| AppError::not_found("user", entity_id))?;

        let ar_name = usr.name_ar.clone().or_else(|| Some(usr.full_name.clone()));
        (
            format!("{} - {}", parent.name, usr.full_name),
            ar_name.map(|ar| format!("{} - {}", parent.name_ar.as_deref().unwrap_or(&parent.name), ar))
        )
    } else {
        let name_val = entity_name.unwrap_or(entity_type);
        let name_ar_val = entity_name_ar.or(entity_name).unwrap_or(entity_type);
        (
            format!("{} - {}", parent.name, name_val),
            Some(format!("{} - {}", parent.name_ar.as_deref().unwrap_or(&parent.name), name_ar_val))
        )
    };

    let children = account::Entity::find()
        .filter(account::Column::OrgId.eq(&ctx.org_id))
        .filter(account::Column::ParentId.eq(Some(parent.id.clone())))
        .all(&ctx.db)
        .await?;

    let next_suffix = if children.is_empty() {
        1
    } else {
        let mut max_suf = 0;
        for child in children {
            if child.code.starts_with(parent_code) && child.code.len() > parent_code.len() {
                let suffix_str = &child.code[parent_code.len()..];
                if let Ok(num) = suffix_str.parse::<u32>() {
                    if num > max_suf {
                        max_suf = num;
                    }
                }
            }
        }
        max_suf + 1
    };

    let new_code = format!("{}{:05}", parent_code, next_suffix);
    let acc_id = format!("{}_{}_{}", ctx.org_id, entity_type, entity_id);
    let now = Utc::now().to_rfc3339();

    let new_acc = account::Model {
        id: acc_id.clone(),
        org_id: ctx.org_id.to_string(),
        code: new_code,
        name: resolved_name,
        name_ar: resolved_name_ar,
        item_type: parent.item_type,
        sub_type: Some(entity_type.to_string()),
        parent_id: Some(parent.id),
        path: format!("{}/{}", parent.path, acc_id),
        depth: parent.depth + 1,
        currency: parent.currency,
        balances: serde_json::to_value(shared_kernel::currency::CurrencyBalance::new()).unwrap(),
        entity_type: Some(entity_type.to_string()),
        entity_id: Some(entity_id.to_string()),
        is_control_account: 0,
        is_system_account: 0,
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: account::ActiveModel = new_acc.into();
    let created = active.insert(&ctx.db).await?;
    
    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ACCOUNT_CREATED.into(),
        entity_type: "account".into(), entity_id: acc_id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "account.create_sub_account".into(), entity_type: "account".into(),
        entity_id: acc_id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn create_journal_entry(
    ctx: &ServiceContext,
    cmd: CreateJournalEntryCmd,
) -> Result<journal_entry::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let child_count_debit = account::Entity::find()
        .filter(account::Column::ParentId.eq(Some(cmd.debit_account_id.clone())))
        .count(&ctx.db).await?;
    if child_count_debit > 0 {
        return Err(AppError::validation("Cannot post journal entries to a control account (debit account has child sub-accounts)"));
    }

    let child_count_credit = account::Entity::find()
        .filter(account::Column::ParentId.eq(Some(cmd.credit_account_id.clone())))
        .count(&ctx.db).await?;
    if child_count_credit > 0 {
        return Err(AppError::validation("Cannot post journal entries to a control account (credit account has child sub-accounts)"));
    }

    let debit_account = account::Entity::find_by_id(cmd.debit_account_id.clone())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("account", &cmd.debit_account_id))?;

    let credit_account = account::Entity::find_by_id(cmd.credit_account_id.clone())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("account", &cmd.credit_account_id))?;

    let je_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let je = journal_entry::Model {
        id: je_id.clone(),
        org_id: ctx.org_id.clone(),
        transaction_ref: cmd.transaction_ref,
        accounting_date: cmd.accounting_date,
        fiscal_period_id: cmd.fiscal_period_id,
        amount: cmd.amount,
        debit_account_id: cmd.debit_account_id.clone(),
        credit_account_id: cmd.credit_account_id.clone(),
        currency: cmd.currency.clone(),
        exchange_rate: cmd.exchange_rate.or(Some(1.0)),
        base_currency_amount: Some(cmd.amount * cmd.exchange_rate.unwrap_or(1.0)),
        source_type: cmd.source_type,
        source_id: cmd.source_id,
        source_module: cmd.source_module,
        description: cmd.description,
        description_ar: cmd.description_ar,
        metadata: None,
        is_reversed: 0,
        reversed_by: None,
        reversal_reason: None,
        is_reconciled: 0,
        reconciled_at: None,
        reconciliation_ref: None,
        status: "posted".to_string(),
        posted_by: Some(ctx.actor.as_str().to_string()),
        posted_at: Some(now.clone()),
        approved_by: Some(ctx.actor.as_str().to_string()),
        approved_at: Some(now.clone()),
        created_by: Some(ctx.actor.as_str().to_string()),
        created_at: now.clone(),
    };

    let active: journal_entry::ActiveModel = je.into();
    let created_je = active.insert(&ctx.db).await?;

    let debit_change = match debit_account.item_type.to_lowercase().as_str() {
        "asset" | "expense" => cmd.amount,
        _ => -cmd.amount,
    };
    
    let repo = crate::repository::FinanceCommandRepo::new(ctx.db.clone());
    repo.update_account_balance(&cmd.debit_account_id, &cmd.currency, debit_change).await?;

    let credit_change = match credit_account.item_type.to_lowercase().as_str() {
        "asset" | "expense" => -cmd.amount,
        _ => cmd.amount,
    };
    
    repo.update_account_balance(&cmd.credit_account_id, &cmd.currency, credit_change).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::JOURNAL_ENTRY_POSTED.into(),
        entity_type: "journal_entry".into(), entity_id: je_id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created_je).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "journal_entry.post".into(), entity_type: "journal_entry".into(),
        entity_id: je_id, old_data: None,
        new_data: Some(serde_json::to_value(&created_je).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created_je)
}

pub async fn get_journal_entries(ctx: &ServiceContext) -> Result<Vec<journal_entry::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    
    Ok(journal_entry::Entity::find()
        .filter(journal_entry::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(journal_entry::Column::AccountingDate)
        .all(&ctx.db).await?)
}

pub async fn get_account_statement(
    ctx: &ServiceContext,
    account_id: &str,
    start_date: &str,
    end_date: &str,
) -> Result<AccountStatement, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;

    let all_accounts = account::Entity::find()
        .filter(account::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db).await?;

    let account = all_accounts.iter().find(|a| a.id == account_id)
        .ok_or_else(|| AppError::not_found("account", account_id))?;

    let is_debit_increasing = matches!(account.item_type.to_lowercase().as_str(), "asset" | "expense");

    let parent_ids: std::collections::HashSet<String> = all_accounts
        .iter()
        .filter_map(|a| a.parent_id.clone())
        .collect();

    fn get_descendant_ids(parent_id: &str, accounts: &[account::Model]) -> Vec<String> {
        let mut ids = Vec::new();
        for acc in accounts {
            if acc.parent_id.as_deref() == Some(parent_id) {
                ids.push(acc.id.clone());
                ids.extend(get_descendant_ids(&acc.id, accounts));
            }
        }
        ids
    }

    let target_ids = if parent_ids.contains(account_id) {
        let descendants = get_descendant_ids(account_id, &all_accounts);
        let leaf_descendants: Vec<String> = descendants
            .into_iter()
            .filter(|id| !parent_ids.contains(id))
            .collect();
        if leaf_descendants.is_empty() {
            vec![account_id.to_string()]
        } else {
            leaf_descendants
        }
    } else {
        vec![account_id.to_string()]
    };


    let mut opening_balance = 0.0;

    let entries = journal_entry::Entity::find()
        .filter(journal_entry::Column::OrgId.eq(&ctx.org_id))
        .filter(
            journal_entry::Column::DebitAccountId.is_in(target_ids.clone())
                .or(journal_entry::Column::CreditAccountId.is_in(target_ids.clone()))
        )
        .order_by_asc(journal_entry::Column::AccountingDate)
        .all(&ctx.db).await?;

    let mut running_balance = opening_balance;
    let mut items = Vec::new();

    for entry in entries {
        let is_debit = target_ids.contains(&entry.debit_account_id);
        let is_credit = target_ids.contains(&entry.credit_account_id);

        let amt = if entry.currency == account.currency {
            entry.amount
        } else {
            entry.base_currency_amount.unwrap_or(entry.amount)
        };

        let change = if is_debit && !is_credit {
            if is_debit_increasing { amt } else { -amt }
        } else if is_credit && !is_debit {
            if is_debit_increasing { -amt } else { amt }
        } else {
            0.0
        };

        if entry.accounting_date.as_str() < start_date {
            opening_balance += change;
            running_balance = opening_balance;
        } else if entry.accounting_date.as_str() <= end_date {
            running_balance += change;
            items.push(StatementItem {
                id: entry.id.clone(),
                date: entry.accounting_date.clone(),
                reference: entry.transaction_ref.clone(),
                description: entry.description.clone(),
                description_ar: entry.description_ar.clone(),
                debit: if is_debit { Some(amt) } else { None },
                credit: if is_credit { Some(amt) } else { None },
                running_balance,
            });
        }
    }

    let closing_balance = running_balance;

    Ok(AccountStatement {
        account_id: account.id.clone(),
        account_name: account.name.clone(),
        account_code: account.code.clone(),
        currency: account.currency.clone(),
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
        opening_balance,
        closing_balance,
        items,
    })
}
