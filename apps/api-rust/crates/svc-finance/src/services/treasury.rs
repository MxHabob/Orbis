//! Treasury service — cash/bank registers, opening and current balances.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use shared_kernel::errors::AppError;
use crate::entities::treasury;
use crate::params::TreasuryParams;
use crate::repository::{FinanceCommandRepo, FinanceQueryRepo};
use crate::services::ledger::LedgerService;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTreasuryRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String, // bank, cash
    pub bank_name: Option<String>,
    pub account_number: Option<String>,
    pub currency: String,
}

pub struct TreasuryService {
    pub cmd: Arc<FinanceCommandRepo>,
    pub query: Arc<FinanceQueryRepo>,
    pub ledger_service: Arc<LedgerService>,
}

impl TreasuryService {
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

    /// Retrieve treasuries.
    pub async fn get_treasuries(&self, org_id: &str) -> Result<Vec<treasury::Model>, AppError> {
        let params = TreasuryParams { item_type: None, is_active: Some(1) };
        Ok(self.query.get_treasuries(org_id, params).await?)
    }

    /// Create cash or bank treasury and automatically register sub-ledger account.
    pub async fn create_treasury(&self, org_id: &str, req: CreateTreasuryRequest) -> Result<treasury::Model, AppError> {
        let treasury_id = Uuid::new_v4().to_string();
        let entity_type = if req.item_type.to_lowercase() == "bank" {
            "treasury_bank"
        } else {
            "treasury_cash"
        };

        let sub_account = self.ledger_service.get_or_create_sub_account(
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

        Ok(self.cmd.create_treasury(t).await?)
    }
}
