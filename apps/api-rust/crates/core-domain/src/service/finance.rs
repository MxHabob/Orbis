use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};
use crate::entities::{ledger_account, treasury, student_fee, payment, fee_template, discount_template, journal_entry};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateDiscountTemplateRequest {
    pub name: String,
    pub code: Option<String>,
    pub item_type: String, // mapped to 'type' in DB (e.g. system, manual, etc.)
    pub calculation_method: String, // percentage, fixed
    pub value: i32,
    pub eligibility_rules: Option<String>,
    pub limits: Option<String>,
    pub requires_approval: i32,
    pub approver_roles: Option<String>,
    pub is_auto_apply: i32,
    pub is_stackable: i32,
    pub priority: Option<i32>,
    pub is_active: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateLedgerAccountRequest {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String, // e.g., asset, liability, equity, revenue, expense
    pub sub_type: Option<String>,
    pub parent_id: Option<String>,
    pub currency: String,
    pub opening_balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTreasuryRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String, // bank, cash
    pub ledger_account_id: Option<String>, // Deprecated: Now auto-created
    pub bank_name: Option<String>,
    pub account_number: Option<String>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateFeeTemplateRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub category: String,
    pub default_amount: f64,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub applicable_levels: Option<String>,
    pub frequency: String,
    pub installment_config: Option<String>,
    pub late_fee_rules: Option<String>,
    pub is_mandatory: i32,
    pub is_active: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AssignStudentFeeRequest {
    pub enrollment_id: String,
    pub fee_template_id: String,
    pub amount: f64,
    pub due_date: String, // YYYY-MM-DD
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RecordPaymentRequest {
    pub fee_id: String,
    pub amount: f64,
    pub method: String, // cash, bank_transfer, card
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub treasury_id: Option<String>,
    pub currency: Option<String>,
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

impl super::Service {
    // ── Ledger Accounts ──
    pub async fn get_ledger_accounts(&self, org_id: &str) -> Result<Vec<LedgerAccountResponse>, String> {
        let accounts = self.repo.get_ledger_accounts(org_id).await.map_err(|e| e.to_string())?;
        let entries = self.repo.get_journal_entries(org_id).await.map_err(|e| e.to_string())?;

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
                let is_debit_type = match acc.item_type.to_lowercase().as_str() {
                    "asset" | "expense" => true,
                    _ => false,
                };
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

        // Only return control accounts (depth < 4)
        let mut result: Vec<LedgerAccountResponse> = response_map.into_values()
            .filter(|acc| acc.depth < 4)
            .collect();
        result.sort_by(|a, b| a.code.cmp(&b.code));
        Ok(result)
    }

    pub async fn get_transaction_accounts(&self, org_id: &str) -> Result<Vec<LedgerAccountResponse>, String> {
        let accounts = self.repo.get_ledger_accounts(org_id).await.map_err(|e| e.to_string())?;
        let entries = self.repo.get_journal_entries(org_id).await.map_err(|e| e.to_string())?;

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
                let is_debit_type = match acc.item_type.to_lowercase().as_str() {
                    "asset" | "expense" => true,
                    _ => false,
                };
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

        // Only return transaction accounts (depth >= 4)
        let mut result: Vec<LedgerAccountResponse> = response_map.into_values()
            .filter(|acc| acc.depth >= 4)
            .collect();
        result.sort_by(|a, b| a.code.cmp(&b.code));
        Ok(result)
    }

    pub async fn get_or_create_sub_account(
        &self,
        org_id: &str,
        entity_type: &str,
        entity_id: &str,
        entity_name: Option<&str>,
        entity_name_ar: Option<&str>,
    ) -> Result<ledger_account::Model, String> {
        let parent_code = match entity_type {
            "student" => "110301",
            "employee" => "210401",
            "treasury_cash" => "110101",
            "treasury_bank" => "110102",
            "vendor" => "210106",
            "partner" => "310101",
            "guardian" => "110308",
            "branch" => "110604",
            _ => return Err(format!("Unsupported sub-ledger entity type: {}", entity_type)),
        };

        if let Some(existing) = self.repo.get_ledger_account_by_entity(org_id, entity_type, entity_id).await.map_err(|e| e.to_string())? {
            return Ok(existing);
        }

        let parent_id = format!("{}_{}", org_id, parent_code);
        let parent = self.repo.get_ledger_account_by_id(&parent_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Parent control account '{}' not found in chart of accounts. Did you run seeding?", parent_code))?;

        let (resolved_name, resolved_name_ar) = if entity_type == "student" || entity_type == "employee" || entity_type == "guardian" {
            let usr = self.repo.get_user_by_id(entity_id).await
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("User entity '{}' not found", entity_id))?;

            let ar_name = usr.name_ar.clone().or_else(|| Some(usr.full_name.clone()));
            (format!("{} - {}", parent.name, usr.full_name), ar_name.map(|ar| format!("{} - {}", parent.name_ar.as_deref().unwrap_or(&parent.name), ar)))
        } else {
            let name_val = entity_name.unwrap_or(entity_type);
            let name_ar_val = entity_name_ar.or(entity_name).unwrap_or(entity_type);
            (format!("{} - {}", parent.name, name_val), Some(format!("{} - {}", parent.name_ar.as_deref().unwrap_or(&parent.name), name_ar_val)))
        };

        let children = ledger_account::Entity::find()
            .filter(ledger_account::Column::OrgId.eq(org_id))
            .filter(ledger_account::Column::ParentId.eq(Some(parent.id.clone())))
            .all(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?;

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
            item_type: parent.item_type.clone(),
            sub_type: Some(entity_type.to_string()),
            parent_id: Some(parent.id.clone()),
            path: format!("{}/{}", parent.path, acc_id),
            depth: parent.depth + 1,
            currency: parent.currency.clone(),
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

        let created = self.repo.create_ledger_account(new_acc).await.map_err(|e| e.to_string())?;
        Ok(created)
    }

    pub async fn create_ledger_account(&self, org_id: &str, req: CreateLedgerAccountRequest) -> Result<ledger_account::Model, String> {
        // 1. Resolve parent account
        let parent = if let Some(ref pid) = req.parent_id {
            if pid.is_empty() {
                None
            } else {
                let p = self.repo.get_ledger_account_by_id(pid).await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| "Parent account not found".to_string())?;
                Some(p)
            }
        } else if let Some(ref sub_type) = req.sub_type {
            let parent_code = match sub_type.to_lowercase().as_str() {
                "vendor" => "210106",
                "partner" => "310101",
                "employee" => "210401",
                "student" => "110301",
                "guardian" => "110308",
                _ => return Err(format!("Unsupported sub_type for auto parent: {}", sub_type)),
            };
            let parent_id = format!("{}_{}", org_id, parent_code);
            let p = self.repo.get_ledger_account_by_id(&parent_id).await
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("Parent control account '{}' not found. Did you seed default financials?", parent_code))?;
            Some(p)
        } else {
            None
        };

        // 2. Resolve code (auto-generate if empty)
        let final_code = if req.code.is_empty() {
            if let Some(ref p) = parent {
                let children = ledger_account::Entity::find()
                    .filter(ledger_account::Column::OrgId.eq(org_id))
                    .filter(ledger_account::Column::ParentId.eq(Some(p.id.clone())))
                    .all(&self.repo.db)
                    .await
                    .map_err(|e| e.to_string())?;

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
                return Err("Account code cannot be empty if there is no parent account".to_string());
            }
        } else {
            req.code.clone()
        };

        // 3. Resolve path, depth, parent_id, currency, and type
        let (parent_id, path, depth, currency, item_type) = if let Some(ref p) = parent {
            let pid = Some(p.id.clone());
            let path = format!("{}/{}", p.path, final_code);
            let depth = p.depth + 1;
            let currency = p.currency.clone();
            let item_type = p.item_type.clone();
            (pid, path, depth, currency, item_type)
        } else {
            let pid = None;
            let path = format!("{}_{}", org_id, final_code);
            let depth = 0;
            let currency = req.currency.clone();
            let item_type = req.item_type.clone();
            (pid, path, depth, currency, item_type)
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
        self.repo.create_ledger_account(la).await.map_err(|e| e.to_string())
    }

    // ── Treasuries ──
    pub async fn get_treasuries(&self, org_id: &str) -> Result<Vec<treasury::Model>, String> {
        self.repo.get_treasuries(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_treasury(&self, org_id: &str, req: CreateTreasuryRequest) -> Result<treasury::Model, String> {
        let treasury_id = Uuid::new_v4().to_string();
        let entity_type = if req.item_type.to_lowercase() == "bank" {
            "treasury_bank"
        } else {
            "treasury_cash"
        };

        let sub_account = self.get_or_create_sub_account(
            org_id,
            entity_type,
            &treasury_id,
            Some(&req.name),
            req.name_ar.as_deref(),
        ).await?;

        let t = treasury::Model {
            id: treasury_id,
            org_id: org_id.to_string(),
            unit_id: None,
            name: req.name,
            name_ar: req.name_ar,
            code: None,
            item_type: req.item_type,
            ledger_account_id: sub_account.id,
            bank_name: req.bank_name,
            bank_branch: None,
            account_number: req.account_number,
            account_name: None,
            iban: None,
            swift_code: None,
            opening_balance: Some(0.0),
            current_balance: Some(0.0),
            currency: req.currency,
            authorized_users: None,
            is_default: 0,
            is_active: 1,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_treasury(t).await.map_err(|e| e.to_string())
    }

    pub async fn get_active_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, String> {
        self.repo.get_active_fee_templates(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, String> {
        self.repo.get_fee_templates(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_fee_template(&self, org_id: &str, req: CreateFeeTemplateRequest) -> Result<fee_template::Model, String> {
        // Validate that chosen accounts are leaf accounts
        let rev_leaf = self.repo.is_leaf_account(&req.revenue_account_id).await.map_err(|e| e.to_string())?;
        if !rev_leaf {
            return Err("Revenue account must be a leaf account".to_string());
        }
        let rec_leaf = self.repo.is_leaf_account(&req.receivable_account_id).await.map_err(|e| e.to_string())?;
        if !rec_leaf {
            return Err("Receivable account must be a leaf account".to_string());
        }

        let tpl = fee_template::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            category: req.category,
            default_amount: req.default_amount,
            revenue_account_id: req.revenue_account_id,
            receivable_account_id: req.receivable_account_id,
            applicable_levels: req.applicable_levels,
            frequency: req.frequency,
            installment_config: req.installment_config,
            late_fee_rules: req.late_fee_rules,
            is_mandatory: req.is_mandatory,
            is_active: req.is_active,
            sort_order: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_fee_template(tpl).await.map_err(|e| e.to_string())
    }

    pub async fn update_fee_template(&self, org_id: &str, id: &str, req: CreateFeeTemplateRequest) -> Result<fee_template::Model, String> {
        let tpl = self.repo.get_fee_template_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Fee template not found".to_string())?;

        if tpl.org_id != org_id {
            return Err("Unauthorized".to_string());
        }

        // Validate leaf accounts
        let rev_leaf = self.repo.is_leaf_account(&req.revenue_account_id).await.map_err(|e| e.to_string())?;
        if !rev_leaf {
            return Err("Revenue account must be a leaf account".to_string());
        }
        let rec_leaf = self.repo.is_leaf_account(&req.receivable_account_id).await.map_err(|e| e.to_string())?;
        if !rec_leaf {
            return Err("Receivable account must be a leaf account".to_string());
        }

        let mut active: fee_template::ActiveModel = tpl.into();
        active.name = sea_orm::ActiveValue::Set(req.name);
        active.name_ar = sea_orm::ActiveValue::Set(req.name_ar);
        active.code = sea_orm::ActiveValue::Set(req.code);
        active.category = sea_orm::ActiveValue::Set(req.category);
        active.default_amount = sea_orm::ActiveValue::Set(req.default_amount);
        active.revenue_account_id = sea_orm::ActiveValue::Set(req.revenue_account_id);
        active.receivable_account_id = sea_orm::ActiveValue::Set(req.receivable_account_id);
        active.applicable_levels = sea_orm::ActiveValue::Set(req.applicable_levels);
        active.frequency = sea_orm::ActiveValue::Set(req.frequency);
        active.installment_config = sea_orm::ActiveValue::Set(req.installment_config);
        active.late_fee_rules = sea_orm::ActiveValue::Set(req.late_fee_rules);
        active.is_mandatory = sea_orm::ActiveValue::Set(req.is_mandatory);
        active.is_active = sea_orm::ActiveValue::Set(req.is_active);
        active.updated_at = sea_orm::ActiveValue::Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_fee_template(&self, org_id: &str, id: &str) -> Result<u64, String> {
        self.repo.delete_fee_template(org_id, id).await.map_err(|e| e.to_string())
    }

    // ── Student Fees ──
    pub async fn get_student_fees(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<student_fee::Model>, String> {
        self.repo.get_student_fees(org_id, enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn assign_student_fee(&self, org_id: &str, req: AssignStudentFeeRequest) -> Result<student_fee::Model, String> {
        let enrollment = self.repo.get_enrollment_by_id(&req.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Enrollment not found".to_string())?;

        let student_account = match self.repo.get_ledger_account_by_entity(org_id, "student", &enrollment.student_id).await.map_err(|e| e.to_string())? {
            Some(acc) => acc,
            None => {
                self.get_or_create_sub_account(org_id, "student", &enrollment.student_id, None, None).await?
            }
        };

        let template = self.repo.get_fee_template_by_id(&req.fee_template_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Fee template not found".to_string())?;

        let fee_id = Uuid::new_v4().to_string();
        let sf = student_fee::Model {
            id: fee_id.clone(),
            org_id: org_id.to_string(),
            cycle_id: enrollment.cycle_id.clone(),
            enrollment_id: req.enrollment_id.clone(),
            fee_template_id: req.fee_template_id.clone(),
            original_amount: req.amount,
            discount_amount: 0.0,
            net_amount: req.amount,
            status: "unpaid".to_string(),
            due_date: req.due_date,
            discount_details: None,
            parent_fee_id: None,
            installment_number: None,
            notes: req.notes.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let created_fee = self.repo.create_student_fee(sf).await.map_err(|e| e.to_string())?;

        let je_id = Uuid::new_v4().to_string();
        let description = format!("Tuition fee charged: {}", template.name);
        let description_ar = Some(format!("قيد رسوم دراسية: {}", template.name_ar.clone().unwrap_or(template.name.clone())));

        let je = crate::entities::journal_entry::Model {
            id: je_id,
            org_id: org_id.to_string(),
            transaction_ref: format!("INV-{}", &fee_id[..8].to_uppercase()),
            accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
            fiscal_period_id: enrollment.cycle_id,
            amount: req.amount,
            debit_account_id: student_account.id.clone(),
            credit_account_id: template.revenue_account_id.clone(),
            currency: student_account.currency.clone(),
            exchange_rate: Some(1.0),
            base_currency_amount: Some(req.amount),
            source_type: "student_fee".to_string(),
            source_id: Some(fee_id),
            source_module: Some("finance".to_string()),
            description,
            description_ar,
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

        self.repo.create_journal_entry(je).await.map_err(|e| e.to_string())?;

        self.repo.update_ledger_account_balance(&student_account.id, req.amount).await.map_err(|e| e.to_string())?;
        self.repo.update_ledger_account_balance(&template.revenue_account_id, req.amount).await.map_err(|e| e.to_string())?;

        Ok(created_fee)
    }

    // ── Payments ──
    pub async fn get_payments(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<payment::Model>, String> {
        self.repo.get_payments(org_id, enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn record_payment(&self, org_id: &str, req: RecordPaymentRequest, created_by: &str) -> Result<payment::Model, String> {
        let fee = self.repo.get_student_fee_by_id(&req.fee_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Student fee not found".to_string())?;

        let enrollment = self.repo.get_enrollment_by_id(&fee.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Enrollment not found".to_string())?;

        let student_account = match self.repo.get_ledger_account_by_entity(org_id, "student", &enrollment.student_id).await.map_err(|e| e.to_string())? {
            Some(acc) => acc,
            None => {
                self.get_or_create_sub_account(org_id, "student", &enrollment.student_id, None, None).await?
            }
        };

        let treasury_id = req.treasury_id.clone()
            .ok_or_else(|| "Treasury ID is required".to_string())?;
        let treasury = self.repo.get_treasury_by_id(&treasury_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Treasury not found".to_string())?;

        if req.amount <= 0.0 {
            return Err("Payment amount must be greater than zero".to_string());
        }

        let payment_currency = req.currency.clone().unwrap_or_else(|| student_account.currency.clone());
        let exchange_rate = if payment_currency == student_account.currency {
            1.0
        } else {
            let cur = self.repo.get_currency(org_id, &payment_currency).await
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("Currency {} not supported", payment_currency))?;
            cur.exchange_rate
        };

        let base_amount = req.amount * exchange_rate;
        let payment_id = Uuid::new_v4().to_string();
        let receipt_number = format!("REC-{}", Utc::now().timestamp_millis());
        let journal_entry_id = Uuid::new_v4().to_string();

        let p = payment::Model {
            id: payment_id.clone(),
            org_id: org_id.to_string(),
            fee_id: Some(req.fee_id.clone()),
            enrollment_id: Some(fee.enrollment_id.clone()),
            account_id: student_account.id.clone(),
            amount: req.amount,
            currency: payment_currency.clone(),
            exchange_rate,
            base_amount,
            method: req.method.clone(),
            reference: req.reference.clone(),
            receipt_number: Some(receipt_number.clone()),
            journal_entry_id: Some(journal_entry_id.clone()),
            treasury_id: Some(treasury_id.clone()),
            payer_name: req.payer_name.clone(),
            payer_phone: req.payer_phone.clone(),
            payment_type: "receipt".to_string(),
            related_payment_id: None,
            notes: req.notes.clone(),
            paid_at: Utc::now().to_rfc3339(),
            created_at: Utc::now().to_rfc3339(),
            created_by: Some(created_by.to_string()),
        };

        let payment_record = self.repo.create_payment(p).await.map_err(|e| e.to_string())?;

        let description = format!("Fee payment received via {}: {}", req.method, receipt_number);
        let description_ar = Some(format!("سداد رسوم عبر {}: {}", req.method, receipt_number));

        let je = crate::entities::journal_entry::Model {
            id: journal_entry_id.clone(),
            org_id: org_id.to_string(),
            transaction_ref: receipt_number,
            accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
            fiscal_period_id: fee.cycle_id.clone(),
            amount: req.amount,
            debit_account_id: treasury.ledger_account_id.clone(),
            credit_account_id: student_account.id.clone(),
            currency: payment_currency.clone(),
            exchange_rate: Some(exchange_rate),
            base_currency_amount: Some(base_amount),
            source_type: "payment".to_string(),
            source_id: Some(payment_id.clone()),
            source_module: Some("finance".to_string()),
            description,
            description_ar,
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
            created_by: Some(created_by.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        self.repo.create_journal_entry(je).await.map_err(|e| e.to_string())?;

        let tx = crate::entities::treasury_transaction::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            treasury_id: treasury_id.clone(),
            journal_entry_id: journal_entry_id.clone(),
            item_type: "inflow".to_string(),
            category: "tuition".to_string(),
            amount: req.amount,
            currency: payment_currency.clone(),
            exchange_rate: Some(exchange_rate),
            reference_type: Some("payment".to_string()),
            reference_id: Some(payment_id),
            description: format!("Tuition payment for student fee"),
            notes: req.notes.clone(),
            attachments: None,
            created_by: Some(created_by.to_string()),
            approved_by: Some(created_by.to_string()),
            approved_at: Some(Utc::now().to_rfc3339()),
            transaction_date: Utc::now().format("%Y-%m-%d").to_string(),
            created_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_treasury_transaction(tx).await.map_err(|e| e.to_string())?;

        self.repo.update_treasury_balance(&treasury_id, req.amount).await.map_err(|e| e.to_string())?;
        self.repo.update_ledger_account_balance(&treasury.ledger_account_id, base_amount).await.map_err(|e| e.to_string())?;
        self.repo.update_ledger_account_balance(&student_account.id, -base_amount).await.map_err(|e| e.to_string())?;

        let payments = self.repo.get_payments(org_id, Some(&fee.enrollment_id)).await.map_err(|e| e.to_string())?;
        let total_paid: f64 = payments.iter()
            .filter(|p| p.fee_id.as_ref() == Some(&fee.id))
            .map(|p| p.base_amount)
            .sum::<f64>() + base_amount;

        let new_status = if total_paid >= fee.net_amount {
            "paid".to_string()
        } else if total_paid > 0.0 {
            "partially_paid".to_string()
        } else {
            "unpaid".to_string()
        };

        self.repo.update_student_fee_status(&fee.id, &new_status).await.map_err(|e| e.to_string())?;
        Ok(payment_record)
    }

    pub async fn get_journal_entries(&self, org_id: &str) -> Result<Vec<crate::entities::journal_entry::Model>, String> {
        self.repo.get_journal_entries(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_journal_entry(&self, org_id: &str, req: CreateJournalEntryRequest) -> Result<journal_entry::Model, String> {
        let debit_is_leaf = self.repo.is_leaf_account(&req.debit_account_id).await.map_err(|e| e.to_string())?;
        if !debit_is_leaf {
            return Err("Cannot post journal entries to a control account (debit account has child sub-accounts)".to_string());
        }
        let credit_is_leaf = self.repo.is_leaf_account(&req.credit_account_id).await.map_err(|e| e.to_string())?;
        if !credit_is_leaf {
            return Err("Cannot post journal entries to a control account (credit account has child sub-accounts)".to_string());
        }

        let debit_account = self.repo.get_ledger_account_by_id(&req.debit_account_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Debit account not found".to_string())?;

        let credit_account = self.repo.get_ledger_account_by_id(&req.credit_account_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Credit account not found".to_string())?;

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

        let created_je = self.repo.create_journal_entry(je).await.map_err(|e| e.to_string())?;

        let debit_change = match debit_account.item_type.to_lowercase().as_str() {
            "asset" | "expense" => req.amount,
            _ => -req.amount,
        };
        self.repo.update_ledger_account_balance(&req.debit_account_id, debit_change).await.map_err(|e| e.to_string())?;

        let credit_change = match credit_account.item_type.to_lowercase().as_str() {
            "asset" | "expense" => -req.amount,
            _ => req.amount,
        };
        self.repo.update_ledger_account_balance(&req.credit_account_id, credit_change).await.map_err(|e| e.to_string())?;

        Ok(created_je)
    }

    // ── Discount Templates ──
    pub async fn get_active_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, String> {
        self.repo.get_active_discount_templates(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, String> {
        self.repo.get_discount_templates(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_discount_template(&self, org_id: &str, req: CreateDiscountTemplateRequest) -> Result<discount_template::Model, String> {
        let tpl = discount_template::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            code: req.code,
            item_type: req.item_type,
            calculation_method: req.calculation_method,
            value: req.value,
            eligibility_rules: req.eligibility_rules,
            limits: req.limits,
            requires_approval: req.requires_approval,
            approver_roles: req.approver_roles,
            is_auto_apply: req.is_auto_apply,
            is_stackable: req.is_stackable,
            priority: req.priority,
            is_active: req.is_active,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_discount_template(tpl).await.map_err(|e| e.to_string())
    }

    pub async fn update_discount_template(&self, org_id: &str, id: &str, req: CreateDiscountTemplateRequest) -> Result<discount_template::Model, String> {
        let tpl = self.repo.get_discount_template_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Discount template not found".to_string())?;

        if tpl.org_id != org_id {
            return Err("Unauthorized".to_string());
        }

        let mut active: discount_template::ActiveModel = tpl.into();
        active.name = sea_orm::ActiveValue::Set(req.name);
        active.code = sea_orm::ActiveValue::Set(req.code);
        active.item_type = sea_orm::ActiveValue::Set(req.item_type);
        active.calculation_method = sea_orm::ActiveValue::Set(req.calculation_method);
        active.value = sea_orm::ActiveValue::Set(req.value);
        active.eligibility_rules = sea_orm::ActiveValue::Set(req.eligibility_rules);
        active.limits = sea_orm::ActiveValue::Set(req.limits);
        active.requires_approval = sea_orm::ActiveValue::Set(req.requires_approval);
        active.approver_roles = sea_orm::ActiveValue::Set(req.approver_roles);
        active.is_auto_apply = sea_orm::ActiveValue::Set(req.is_auto_apply);
        active.is_stackable = sea_orm::ActiveValue::Set(req.is_stackable);
        active.priority = sea_orm::ActiveValue::Set(req.priority);
        active.is_active = sea_orm::ActiveValue::Set(req.is_active);
        active.updated_at = sea_orm::ActiveValue::Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_discount_template(&self, org_id: &str, id: &str) -> Result<u64, String> {
        self.repo.delete_discount_template(org_id, id).await.map_err(|e| e.to_string())
    }
}
