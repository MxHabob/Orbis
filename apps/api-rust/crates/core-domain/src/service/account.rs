use serde::{Deserialize, Serialize};

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

impl super::Service {
    pub async fn get_account_statement(
        &self,
        org_id: &str,
        account_id: &str,
        start_date: &str, // YYYY-MM-DD
        end_date: &str,   // YYYY-MM-DD
    ) -> Result<AccountStatement, String> {
        let account = self.repo.get_ledger_account_by_id(account_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Account not found".to_string())?;

        let is_debit_increasing = match account.item_type.to_lowercase().as_str() {
            "asset" | "expense" => true,
            "liability" | "equity" | "revenue" => false,
            _ => true,
        };

        // Fetch all journal entries for this account (sorted asc by date)
        let entries = self.repo.get_journal_entries_by_account(org_id, account_id).await
            .map_err(|e| e.to_string())?;

        let mut opening_balance = account.opening_balance.unwrap_or(0.0);
        let mut running_balance = opening_balance;
        let mut items = Vec::new();

        for entry in entries {
            let is_debit = entry.debit_account_id == account_id;
            
            // Determine the amount in the account's currency
            let amt = if entry.currency == account.currency {
                entry.amount
            } else {
                entry.base_currency_amount.unwrap_or(entry.amount)
            };

            // Calculate change in balance
            let change = if is_debit {
                if is_debit_increasing { amt } else { -amt }
            } else {
                if is_debit_increasing { -amt } else { amt }
            };

            // If entry is before the statement start date, it contributes to the opening balance
            if entry.accounting_date.as_str() < start_date {
                opening_balance += change;
                running_balance = opening_balance;
            } else if entry.accounting_date.as_str() <= end_date {
                // If it falls within the statement date range
                running_balance += change;
                items.push(StatementItem {
                    id: entry.id.clone(),
                    date: entry.accounting_date.clone(),
                    reference: entry.transaction_ref.clone(),
                    description: entry.description.clone(),
                    description_ar: entry.description_ar.clone(),
                    debit: if is_debit { Some(amt) } else { None },
                    credit: if !is_debit { Some(amt) } else { None },
                    running_balance,
                });
            }
        }

        let closing_balance = running_balance;

        Ok(AccountStatement {
            account_id: account.id,
            account_name: account.name,
            account_code: account.code,
            currency: account.currency,
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            opening_balance,
            closing_balance,
            items,
        })
    }
}
