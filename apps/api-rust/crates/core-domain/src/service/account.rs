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
        let all_accounts = self.repo.get_ledger_accounts(org_id).await
            .map_err(|e| e.to_string())?;

        let account = all_accounts.iter().find(|a| a.id == account_id)
            .ok_or_else(|| "Account not found".to_string())?;

        let is_debit_increasing = match account.item_type.to_lowercase().as_str() {
            "asset" | "expense" => true,
            "liability" | "equity" | "revenue" => false,
            _ => true,
        };

        // Determine if account is parent by checking if other accounts list it as parent_id
        let parent_ids: std::collections::HashSet<String> = all_accounts
            .iter()
            .filter_map(|a| a.parent_id.clone())
            .collect();

        // Helper to recursively get all descendant accounts
        fn get_descendant_ids(parent_id: &str, accounts: &[crate::entities::ledger_account::Model]) -> Vec<String> {
            let mut ids = Vec::new();
            for acc in accounts {
                if acc.parent_id.as_deref() == Some(parent_id) {
                    ids.push(acc.id.clone());
                    ids.extend(get_descendant_ids(&acc.id, accounts));
                }
            }
            ids
        }

        // Determine target leaf account IDs
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

        // Sum opening balance of target leaf accounts
        let target_accounts: Vec<&crate::entities::ledger_account::Model> = all_accounts
            .iter()
            .filter(|a| target_ids.contains(&a.id))
            .collect();
        let mut opening_balance = target_accounts.iter().map(|a| a.opening_balance.unwrap_or(0.0)).sum::<f64>();

        // Fetch journal entries for these accounts (sorted asc by date)
        let entries = self.repo.get_journal_entries_by_accounts(org_id, &target_ids).await
            .map_err(|e| e.to_string())?;

        let mut running_balance = opening_balance;
        let mut items = Vec::new();

        for entry in entries {
            let is_debit = target_ids.contains(&entry.debit_account_id);
            let is_credit = target_ids.contains(&entry.credit_account_id);

            // Determine the amount in the account's currency
            let amt = if entry.currency == account.currency {
                entry.amount
            } else {
                entry.base_currency_amount.unwrap_or(entry.amount)
            };

            // Calculate change in balance
            let change = if is_debit && !is_credit {
                if is_debit_increasing { amt } else { -amt }
            } else if is_credit && !is_debit {
                if is_debit_increasing { -amt } else { amt }
            } else {
                0.0
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
                    credit: if is_credit { Some(amt) } else { None },
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
