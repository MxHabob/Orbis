//! Ledger service — chart of accounts, sub-accounts, and double-entry journal entries.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use svc_iam::entities::user;

use shared_kernel::errors::AppError;
use crate::entities::{ledger_account, journal_entry};
use crate::params::LedgerAccountParams;
use crate::repository::{FinanceCommandRepo, FinanceQueryRepo};


#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateLedgerAccountRequest {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String, // asset, liability, equity, revenue, expense
    pub sub_type: Option<String>,
    pub parent_id: Option<String>,
    pub currency: String,
    pub opening_balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateJournalEntryRequest {
    pub transaction_ref: String,
    pub accounting_date: String,
    pub fiscal_period_id: String,
    pub amount: f64,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub currency: String,
    pub exchange_rate: Option<f64>,
    pub description: String,
    pub description_ar: Option<String>,
    pub source_type: String,
    pub source_id: Option<String>,
    pub source_module: Option<String>,
}

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
    pub opening_balance: Option<f64>,
    pub current_balance: Option<f64>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub is_control_account: i32,
    pub is_system_account: i32,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
    pub debit_yer: f64,
    pub credit_yer: f64,
    pub debit_sar: f64,
    pub credit_sar: f64,
    pub debit_usd: f64,
    pub credit_usd: f64,
    pub debit_aed: f64,
    pub credit_aed: f64,
}

pub struct LedgerService {
    pub cmd: Arc<FinanceCommandRepo>,
    pub query: Arc<FinanceQueryRepo>,
}

impl LedgerService {
    pub fn new(cmd: Arc<FinanceCommandRepo>, query: Arc<FinanceQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Retrieve all control accounts (depth < 4) with aggregated balances.
    pub async fn get_ledger_accounts(&self, org_id: &str) -> Result<Vec<LedgerAccountResponse>, AppError> {
        let accounts = self.query.get_ledger_accounts(org_id, LedgerAccountParams { item_type: None, is_active: None }).await?;
        let entries = self.query.get_journal_entries(org_id).await?;

        use std::collections::HashMap;
        let mut response_map: HashMap<String, LedgerAccountResponse> = accounts.iter().map(|acc| {
            let mut debit_yer = 0.0;
            let mut credit_yer = 0.0;
            let mut debit_sar = 0.0;
            let mut credit_sar = 0.0;
            let mut debit_usd = 0.0;
            let mut credit_usd = 0.0;
            let mut debit_aed = 0.0;
            let mut credit_aed = 0.0;

            let ob = acc.opening_balance.unwrap_or(0.0);
            if ob > 0.0 {
                let is_debit_type = matches!(acc.item_type.to_lowercase().as_str(), "asset" | "expense");
                match acc.currency.to_uppercase().as_str() {
                    "YER" => { if is_debit_type { debit_yer = ob; } else { credit_yer = ob; } }
                    "SAR" => { if is_debit_type { debit_sar = ob; } else { credit_sar = ob; } }
                    "USD" => { if is_debit_type { debit_usd = ob; } else { credit_usd = ob; } }
                    "AED" => { if is_debit_type { debit_aed = ob; } else { credit_aed = ob; } }
                    _ => {}
                }
            }

            (acc.id.clone(), LedgerAccountResponse {
                id: acc.id.clone(),
                org_id: acc.org_id.clone(),
                code: acc.code.clone(),
                name: acc.name.clone(),
                name_ar: acc.name_ar.clone(),
                item_type: acc.item_type.clone(),
                sub_type: acc.sub_type.clone(),
                parent_id: acc.parent_id.clone(),
                path: acc.path.clone(),
                depth: acc.depth,
                currency: acc.currency.clone(),
                opening_balance: acc.opening_balance,
                current_balance: acc.current_balance,
                entity_type: acc.entity_type.clone(),
                entity_id: acc.entity_id.clone(),
                is_control_account: acc.is_control_account,
                is_system_account: acc.is_system_account,
                is_active: acc.is_active,
                created_at: acc.created_at.clone(),
                updated_at: acc.updated_at.clone(),
                debit_yer,
                credit_yer,
                debit_sar,
                credit_sar,
                debit_usd,
                credit_usd,
                debit_aed,
                credit_aed,
            })
        }).collect();

        for je in entries {
            let currency = je.currency.to_uppercase();
            let amount = je.amount;

            if let Some(res) = response_map.get_mut(&je.debit_account_id) {
                match currency.as_str() {
                    "YER" => res.debit_yer += amount,
                    "SAR" => res.debit_sar += amount,
                    "USD" => res.debit_usd += amount,
                    "AED" => res.debit_aed += amount,
                    _ => {}
                }
            }

            if let Some(res) = response_map.get_mut(&je.credit_account_id) {
                match currency.as_str() {
                    "YER" => res.credit_yer += amount,
                    "SAR" => res.credit_sar += amount,
                    "USD" => res.credit_usd += amount,
                    "AED" => res.credit_aed += amount,
                    _ => {}
                }
            }
        }

        let mut sorted_ids: Vec<String> = accounts.iter().map(|a| a.id.clone()).collect();
        sorted_ids.sort_by(|id_a, id_b| {
            let depth_a = response_map.get(id_a).map(|a| a.depth).unwrap_or(0);
            let depth_b = response_map.get(id_b).map(|b| b.depth).unwrap_or(0);
            depth_b.cmp(&depth_a)
        });

        for id in sorted_ids {
            if let Some(acc) = response_map.get(&id).cloned() {
                if let Some(parent_id) = &acc.parent_id {
                    if let Some(parent) = response_map.get_mut(parent_id) {
                        parent.debit_yer += acc.debit_yer;
                        parent.credit_yer += acc.credit_yer;
                        parent.debit_sar += acc.debit_sar;
                        parent.credit_sar += acc.credit_sar;
                        parent.debit_usd += acc.debit_usd;
                        parent.credit_usd += acc.credit_usd;
                        parent.debit_aed += acc.debit_aed;
                        parent.credit_aed += acc.credit_aed;
                    }
                }
            }
        }

        let mut result: Vec<LedgerAccountResponse> = response_map.into_values()
            .filter(|acc| acc.depth < 4)
            .collect();
        result.sort_by(|a, b| a.code.cmp(&b.code));
        Ok(result)
    }

    /// Retrieve only leaf transaction accounts (depth >= 4) with aggregated balances.
    pub async fn get_transaction_accounts(&self, org_id: &str) -> Result<Vec<LedgerAccountResponse>, AppError> {
        let accounts = self.query.get_ledger_accounts(org_id, LedgerAccountParams { item_type: None, is_active: None }).await?;
        let entries = self.query.get_journal_entries(org_id).await?;

        use std::collections::HashMap;
        let mut response_map: HashMap<String, LedgerAccountResponse> = accounts.iter().map(|acc| {
            let mut debit_yer = 0.0;
            let mut credit_yer = 0.0;
            let mut debit_sar = 0.0;
            let mut credit_sar = 0.0;
            let mut debit_usd = 0.0;
            let mut credit_usd = 0.0;
            let mut debit_aed = 0.0;
            let mut credit_aed = 0.0;

            let ob = acc.opening_balance.unwrap_or(0.0);
            if ob > 0.0 {
                let is_debit_type = matches!(acc.item_type.to_lowercase().as_str(), "asset" | "expense");
                match acc.currency.to_uppercase().as_str() {
                    "YER" => { if is_debit_type { debit_yer = ob; } else { credit_yer = ob; } }
                    "SAR" => { if is_debit_type { debit_sar = ob; } else { credit_sar = ob; } }
                    "USD" => { if is_debit_type { debit_usd = ob; } else { credit_usd = ob; } }
                    "AED" => { if is_debit_type { debit_aed = ob; } else { credit_aed = ob; } }
                    _ => {}
                }
            }

            (acc.id.clone(), LedgerAccountResponse {
                id: acc.id.clone(),
                org_id: acc.org_id.clone(),
                code: acc.code.clone(),
                name: acc.name.clone(),
                name_ar: acc.name_ar.clone(),
                item_type: acc.item_type.clone(),
                sub_type: acc.sub_type.clone(),
                parent_id: acc.parent_id.clone(),
                path: acc.path.clone(),
                depth: acc.depth,
                currency: acc.currency.clone(),
                opening_balance: acc.opening_balance,
                current_balance: acc.current_balance,
                entity_type: acc.entity_type.clone(),
                entity_id: acc.entity_id.clone(),
                is_control_account: acc.is_control_account,
                is_system_account: acc.is_system_account,
                is_active: acc.is_active,
                created_at: acc.created_at.clone(),
                updated_at: acc.updated_at.clone(),
                debit_yer,
                credit_yer,
                debit_sar,
                credit_sar,
                debit_usd,
                credit_usd,
                debit_aed,
                credit_aed,
            })
        }).collect();

        for je in entries {
            let currency = je.currency.to_uppercase();
            let amount = je.amount;

            if let Some(res) = response_map.get_mut(&je.debit_account_id) {
                match currency.as_str() {
                    "YER" => res.debit_yer += amount,
                    "SAR" => res.debit_sar += amount,
                    "USD" => res.debit_usd += amount,
                    "AED" => res.debit_aed += amount,
                    _ => {}
                }
            }

            if let Some(res) = response_map.get_mut(&je.credit_account_id) {
                match currency.as_str() {
                    "YER" => res.credit_yer += amount,
                    "SAR" => res.credit_sar += amount,
                    "USD" => res.credit_usd += amount,
                    "AED" => res.credit_aed += amount,
                    _ => {}
                }
            }
        }

        let mut result: Vec<LedgerAccountResponse> = response_map.into_values()
            .filter(|acc| acc.depth >= 4)
            .collect();
        result.sort_by(|a, b| a.code.cmp(&b.code));
        Ok(result)
    }

    /// Resolve or auto-generate a sub-ledger account under a control account parent (e.g. Students control account 110301).
    pub async fn get_or_create_sub_account(
        &self,
        org_id: &str,
        entity_type: &str,
        entity_id: &str,
        entity_name: Option<&str>,
        entity_name_ar: Option<&str>,
    ) -> Result<ledger_account::Model, AppError> {
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

        if let Some(existing) = self.query.get_ledger_account_by_entity(org_id, entity_type, entity_id).await? {
            return Ok(existing);
        }

        let parent_id = format!("{}_{}", org_id, parent_code);
        let parent = self.query.get_ledger_account_by_id(&parent_id).await?
            .ok_or_else(|| AppError::validation(format!(
                "Parent control account '{}' not found in chart of accounts. Did you run seeding?", parent_code
            )))?;

        let (resolved_name, resolved_name_ar) = if entity_type == "student" || entity_type == "employee" || entity_type == "guardian" {
            let usr = user::Entity::find_by_id(entity_id.to_string())
                .one(&self.query.db)
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

        let children = ledger_account::Entity::find()
            .filter(ledger_account::Column::OrgId.eq(org_id))
            .filter(ledger_account::Column::ParentId.eq(Some(parent.id.clone())))
            .all(&self.query.db)
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
        let acc_id = format!("{}_{}_{}", org_id, entity_type, entity_id);

        let new_acc = ledger_account::Model {
            id: acc_id.clone(),
            org_id: org_id.to_string(),
            code: new_code,
            name: resolved_name,
            name_ar: resolved_name_ar,
            item_type: parent.item_type,
            sub_type: Some(entity_type.to_string()),
            parent_id: Some(parent.id),
            path: format!("{}/{}", parent.path, acc_id),
            depth: parent.depth + 1,
            currency: parent.currency,
            opening_balance: Some(0.0),
            current_balance: Some(0.0),
            entity_type: Some(entity_type.to_string()),
            entity_id: Some(entity_id.to_string()),
            is_control_account: 0,
            is_system_account: 0,
            is_active: 1,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let created = self.cmd.create_ledger_account(new_acc).await?;
        Ok(created)
    }

    /// Create a new custom ledger account.
    pub async fn create_ledger_account(&self, org_id: &str, req: CreateLedgerAccountRequest) -> Result<ledger_account::Model, AppError> {
        let parent = if let Some(ref pid) = req.parent_id {
            if pid.is_empty() {
                None
            } else {
                let p = self.query.get_ledger_account_by_id(pid).await?
                    .ok_or_else(|| AppError::not_found("ledger_account", pid))?;
                Some(p)
            }
        } else if let Some(ref sub_type) = req.sub_type {
            let parent_code = match sub_type.to_lowercase().as_str() {
                "vendor" => "210106",
                "partner" => "310101",
                "employee" => "210401",
                "student" => "110301",
                "guardian" => "110308",
                _ => return Err(AppError::validation(format!("Unsupported sub_type for auto parent: {}", sub_type))),
            };
            let parent_id = format!("{}_{}", org_id, parent_code);
            let p = self.query.get_ledger_account_by_id(&parent_id).await?
                .ok_or_else(|| AppError::validation(format!(
                    "Parent control account '{}' not found. Did you seed default financials?", parent_code
                )))?;
            Some(p)
        } else {
            None
        };

        let final_code = if req.code.is_empty() {
            if let Some(ref p) = parent {
                let children = ledger_account::Entity::find()
                    .filter(ledger_account::Column::OrgId.eq(org_id))
                    .filter(ledger_account::Column::ParentId.eq(Some(p.id.clone())))
                    .all(&self.query.db)
                    .await?;

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
            req.code.clone()
        };

        let (parent_id, path, depth, currency, item_type) = if let Some(ref p) = parent {
            (Some(p.id.clone()), format!("{}/{}", p.path, final_code), p.depth + 1, p.currency.clone(), p.item_type.clone())
        } else {
            (None, format!("{}_{}", org_id, final_code), 0, req.currency.clone(), req.item_type.clone())
        };

        let la = ledger_account::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            code: final_code,
            name: req.name,
            name_ar: req.name_ar,
            item_type,
            sub_type: req.sub_type,
            parent_id,
            path,
            depth,
            currency,
            opening_balance: req.opening_balance,
            current_balance: req.opening_balance.or(Some(0.0)),
            entity_type: None,
            entity_id: None,
            is_control_account: 0,
            is_system_account: 0,
            is_active: 1,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        Ok(self.cmd.create_ledger_account(la).await?)
    }

    /// Create and post a new manual journal entry. Validates double-entry constraint and control account invariants.
    pub async fn create_journal_entry(&self, org_id: &str, req: CreateJournalEntryRequest) -> Result<journal_entry::Model, AppError> {
        let debit_is_leaf = self.query.is_leaf_account(&req.debit_account_id).await?;
        if !debit_is_leaf {
            return Err(AppError::validation("Cannot post journal entries to a control account (debit account has child sub-accounts)"));
        }
        let credit_is_leaf = self.query.is_leaf_account(&req.credit_account_id).await?;
        if !credit_is_leaf {
            return Err(AppError::validation("Cannot post journal entries to a control account (credit account has child sub-accounts)"));
        }

        let debit_account = self.query.get_ledger_account_by_id(&req.debit_account_id).await?
            .ok_or_else(|| AppError::not_found("ledger_account", &req.debit_account_id))?;

        let credit_account = self.query.get_ledger_account_by_id(&req.credit_account_id).await?
            .ok_or_else(|| AppError::not_found("ledger_account", &req.credit_account_id))?;

        let je_id = Uuid::new_v4().to_string();
        let je = journal_entry::Model {
            id: je_id,
            org_id: org_id.to_string(),
            transaction_ref: req.transaction_ref,
            accounting_date: req.accounting_date,
            fiscal_period_id: req.fiscal_period_id,
            amount: req.amount,
            debit_account_id: req.debit_account_id.clone(),
            credit_account_id: req.credit_account_id.clone(),
            currency: req.currency.clone(),
            exchange_rate: req.exchange_rate.or(Some(1.0)),
            base_currency_amount: Some(req.amount * req.exchange_rate.unwrap_or(1.0)),
            source_type: req.source_type,
            source_id: req.source_id,
            source_module: req.source_module,
            description: req.description,
            description_ar: req.description_ar,
            metadata: None,
            is_reversed: 0,
            reversed_by: None,
            reversal_reason: None,
            is_reconciled: 0,
            reconciled_at: None,
            reconciliation_ref: None,
            status: "posted".to_string(),
            posted_by: None,
            posted_at: Some(Utc::now().to_rfc3339()),
            approved_by: None,
            approved_at: Some(Utc::now().to_rfc3339()),
            created_by: None,
            created_at: Utc::now().to_rfc3339(),
        };

        let created_je = self.cmd.create_journal_entry(je).await?;

        let debit_change = match debit_account.item_type.to_lowercase().as_str() {
            "asset" | "expense" => req.amount,
            _ => -req.amount,
        };
        self.cmd.update_ledger_account_balance(&req.debit_account_id, debit_change).await?;

        let credit_change = match credit_account.item_type.to_lowercase().as_str() {
            "asset" | "expense" => -req.amount,
            _ => req.amount,
        };
        self.cmd.update_ledger_account_balance(&req.credit_account_id, credit_change).await?;

        Ok(created_je)
    }

    /// Retrieve manual/system journal entries.
    pub async fn get_journal_entries(&self, org_id: &str) -> Result<Vec<journal_entry::Model>, AppError> {
        Ok(self.query.get_journal_entries(org_id).await?)
    }

    /// Generate an account statement report for a given date range.
    pub async fn get_account_statement(
        &self,
        org_id: &str,
        account_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<AccountStatement, AppError> {
        let all_accounts = self.query.get_ledger_accounts(org_id, LedgerAccountParams { item_type: None, is_active: None }).await?;
        let account = all_accounts.iter().find(|a| a.id == account_id)
            .ok_or_else(|| AppError::not_found("ledger_account", account_id))?;

        let is_debit_increasing = matches!(account.item_type.to_lowercase().as_str(), "asset" | "expense");

        let parent_ids: std::collections::HashSet<String> = all_accounts
            .iter()
            .filter_map(|a| a.parent_id.clone())
            .collect();

        fn get_descendant_ids(parent_id: &str, accounts: &[ledger_account::Model]) -> Vec<String> {
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

        let target_accounts: Vec<&ledger_account::Model> = all_accounts
            .iter()
            .filter(|a| target_ids.contains(&a.id))
            .collect();
        let mut opening_balance = target_accounts.iter().map(|a| a.opening_balance.unwrap_or(0.0)).sum::<f64>();

        let entries = self.query.get_journal_entries_by_accounts(org_id, &target_ids).await?;

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
