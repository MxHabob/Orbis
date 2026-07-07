//! Fee service — fee templates and student fee assignment with invoice posting.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveValue, EntityTrait};

use shared_kernel::errors::AppError;
use crate::entities::{fee_template, student_fee, ledger_account, journal_entry};
use crate::params::{FeeTemplateParams, StudentFeeParams};
use crate::repository::{FinanceCommandRepo, FinanceQueryRepo};
use crate::services::ledger::LedgerService;

use domain_academic::entities::enrollment;

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

pub struct FeeService {
    pub cmd: Arc<FinanceCommandRepo>,
    pub query: Arc<FinanceQueryRepo>,
    pub ledger_service: Arc<LedgerService>,
}

impl FeeService {
    pub fn new(
        cmd: Arc<FinanceCommandRepo>,
        query: Arc<FinanceQueryRepo>,
        ledger_service: Arc<LedgerService>,
    ) -> Self {
        Self {
            cmd,
            query,
            ledger_service,
        }
    }

    pub async fn get_active_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, AppError> {
        let params = FeeTemplateParams { category: None, is_active: Some(1) };
        Ok(self.query.get_fee_templates(org_id, params).await?)
    }

    pub async fn get_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, AppError> {
        let params = FeeTemplateParams { category: None, is_active: None };
        Ok(self.query.get_fee_templates(org_id, params).await?)
    }

    pub async fn create_fee_template(&self, org_id: &str, req: CreateFeeTemplateRequest) -> Result<fee_template::Model, AppError> {
        let ledgers = self.query.get_ledger_accounts(org_id, crate::params::LedgerAccountParams { item_type: None, is_active: None }).await?;
        let (resolved_revenue_id, resolved_receivable_id) = resolve_fee_template_accounts(
            &req.category,
            &req.applicable_levels,
            &ledgers,
            org_id,
        );

        let rev_leaf = self.query.is_leaf_account(&resolved_revenue_id).await?;
        if !rev_leaf {
            return Err(AppError::validation("Resolved Revenue account must be a leaf account"));
        }
        let rec_leaf = self.query.is_leaf_account(&resolved_receivable_id).await?;
        if !rec_leaf {
            return Err(AppError::validation("Resolved Receivable account must be a leaf account"));
        }

        let tpl = fee_template::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            category: req.category,
            default_amount: req.default_amount,
            revenue_account_id: resolved_revenue_id,
            receivable_account_id: resolved_receivable_id,
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

        Ok(self.cmd.create_fee_template(tpl).await?)
    }

    pub async fn update_fee_template(&self, org_id: &str, id: &str, req: CreateFeeTemplateRequest) -> Result<fee_template::Model, AppError> {
        let tpl = self.query.get_fee_template_by_id(id).await?
            .ok_or_else(|| AppError::not_found("fee_template", id))?;

        if tpl.org_id != org_id {
            return Err(AppError::unauthorized("Not authorized to update this fee template"));
        }

        let ledgers = self.query.get_ledger_accounts(org_id, crate::params::LedgerAccountParams { item_type: None, is_active: None }).await?;
        let (resolved_revenue_id, resolved_receivable_id) = resolve_fee_template_accounts(
            &req.category,
            &req.applicable_levels,
            &ledgers,
            org_id,
        );

        let rev_leaf = self.query.is_leaf_account(&resolved_revenue_id).await?;
        if !rev_leaf {
            return Err(AppError::validation("Resolved Revenue account must be a leaf account"));
        }
        let rec_leaf = self.query.is_leaf_account(&resolved_receivable_id).await?;
        if !rec_leaf {
            return Err(AppError::validation("Resolved Receivable account must be a leaf account"));
        }

        let mut model = tpl;
        model.name = req.name;
        model.name_ar = req.name_ar;
        model.code = req.code;
        model.category = req.category;
        model.default_amount = req.default_amount;
        model.revenue_account_id = resolved_revenue_id;
        model.receivable_account_id = resolved_receivable_id;
        model.applicable_levels = req.applicable_levels;
        model.frequency = req.frequency;
        model.installment_config = req.installment_config;
        model.late_fee_rules = req.late_fee_rules;
        model.is_mandatory = req.is_mandatory;
        model.is_active = req.is_active;
        model.updated_at = Utc::now().to_rfc3339();

        Ok(self.cmd.update_fee_template(model).await?)
    }

    pub async fn delete_fee_template(&self, org_id: &str, id: &str) -> Result<u64, AppError> {
        Ok(self.cmd.delete_fee_template(org_id, id).await?)
    }

    // ── Student Fees ──
    pub async fn get_student_fees(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<student_fee::Model>, AppError> {
        let params = StudentFeeParams { enrollment_id: enrollment_id.map(String::from), cycle_id: None, status: None };
        Ok(self.query.get_student_fees(org_id, params).await?)
    }

    /// Assign a fee to a student enrollment and post the receivable journal entry.
    pub async fn assign_student_fee(&self, org_id: &str, req: AssignStudentFeeRequest) -> Result<student_fee::Model, AppError> {
        let enrollment = enrollment::Entity::find_by_id(req.enrollment_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", &req.enrollment_id))?;

        let student_account = match self.query.get_ledger_account_by_entity(org_id, "student", &enrollment.student_id).await? {
            Some(acc) => acc,
            None => {
                self.ledger_service.get_or_create_sub_account(org_id, "student", &enrollment.student_id, None, None).await?
            }
        };

        let template = self.query.get_fee_template_by_id(&req.fee_template_id).await?
            .ok_or_else(|| AppError::not_found("fee_template", &req.fee_template_id))?;

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

        let created_fee = self.cmd.create_student_fee(sf).await?;

        let je_id = Uuid::new_v4().to_string();
        let description = format!("Tuition fee charged: {}", template.name);
        let description_ar = Some(format!("قيد رسوم دراسية: {}", template.name_ar.clone().unwrap_or(template.name.clone())));

        let je = journal_entry::Model {
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

        self.cmd.create_journal_entry(je).await?;

        self.cmd.update_ledger_account_balance(&student_account.id, req.amount).await?;
        self.cmd.update_ledger_account_balance(&template.revenue_account_id, req.amount).await?;

        Ok(created_fee)
    }
}

fn resolve_fee_template_accounts(
    category: &str,
    applicable_levels: &Option<String>,
    ledgers: &[ledger_account::Model],
    org_id: &str,
) -> (String, String) {
    let receivable_code;
    let revenue_code;

    match category.to_lowercase().as_str() {
        "tuition" => {
            receivable_code = "110301";
            let mut temp_rev = "410301";
            if let Some(ref levels_str) = applicable_levels {
                if let Ok(levels_json) = serde_json::from_str::<serde_json::Value>(levels_str) {
                    if let Some(levels) = levels_json.get("levels").and_then(|l| l.as_array()) {
                        let mut all_kg = true;
                        let mut all_primary = true;
                        let mut all_middle = true;
                        let mut all_high = true;
                        let mut has_any = false;

                        for lvl_val in levels {
                            if let Some(lvl) = lvl_val.as_str() {
                                has_any = true;
                                let lvl_lower = lvl.to_lowercase();
                                if !lvl_lower.starts_with("kg") {
                                    all_kg = false;
                                }
                                if !lvl_lower.starts_with("grade") || !["grade1", "grade2", "grade3", "grade4", "grade5", "grade6"].contains(&lvl_lower.as_str()) {
                                    all_primary = false;
                                }
                                if !lvl_lower.starts_with("grade") || !["grade7", "grade8", "grade9"].contains(&lvl_lower.as_str()) {
                                    all_middle = false;
                                }
                                if !lvl_lower.starts_with("grade") || !["grade10", "grade11", "grade12"].contains(&lvl_lower.as_str()) {
                                    all_high = false;
                                }
                            }
                        }

                        if has_any {
                            if all_kg {
                                temp_rev = "410304";
                            } else if all_primary {
                                temp_rev = "410301";
                            } else if all_middle {
                                temp_rev = "410302";
                            } else if all_high {
                                temp_rev = "410303";
                            }
                        }
                    }
                }
            }
            revenue_code = temp_rev;
        }
        "transport" => {
            receivable_code = "110303";
            revenue_code = "420201";
        }
        "books" => {
            receivable_code = "110308";
            revenue_code = "450103";
        }
        "uniform" => {
            receivable_code = "110308";
            revenue_code = "450301";
        }
        "exam" => {
            receivable_code = "110308";
            revenue_code = "450503";
        }
        _ => {
            receivable_code = "110308";
            revenue_code = "450706";
        }
    }

    let revenue_id = ledgers.iter()
        .find(|l| l.code == revenue_code)
        .map(|l| l.id.clone())
        .unwrap_or_else(|| format!("{}_{}", org_id, revenue_code));

    let receivable_id = ledgers.iter()
        .find(|l| l.code == receivable_code)
        .map(|l| l.id.clone())
        .unwrap_or_else(|| format!("{}_{}", org_id, receivable_code));

    (revenue_id, receivable_id)
}
