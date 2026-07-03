use chrono::Utc;
use crate::entities::{currency, ledger_account};
use serde::Deserialize;

#[derive(Deserialize)]
struct CoaAccount {
    code: String,
    name_ar: String,
    name_en: String,
    #[serde(rename = "type")]
    account_type: String,
    level: i32,
    children: Option<Vec<CoaAccount>>,
}

#[derive(Deserialize)]
struct CoaJson {
    accounts: Vec<CoaAccount>,
}

impl super::Service {
    async fn seed_coa_recursive(
        &self,
        org_id: &str,
        accounts: &[CoaAccount],
        parent_id: Option<String>,
        parent_path: &str,
        base_currency: &str,
    ) -> Result<(), String> {
        for acc in accounts {
            let full_id = format!("{}_{}", org_id, acc.code);
            let path = if parent_path.is_empty() {
                full_id.clone()
            } else {
                format!("{}/{}", parent_path, full_id)
            };

            let depth = acc.level - 1;
            let is_control = if acc.children.as_ref().map_or(true, |c| c.is_empty()) {
                0
            } else {
                1
            };

            if self.repo.get_ledger_account_by_id(&full_id).await.map_err(|e| e.to_string())?.is_none() {
                let item_type = acc.account_type.to_lowercase(); // ASSET -> asset

                let la = ledger_account::Model {
                    id: full_id.clone(),
                    org_id: org_id.to_string(),
                    code: acc.code.clone(),
                    name: acc.name_en.clone(),
                    name_ar: Some(acc.name_ar.clone()),
                    item_type,
                    sub_type: None,
                    parent_id: parent_id.clone(),
                    path: path.clone(),
                    depth,
                    currency: base_currency.to_string(),
                    opening_balance: Some(0.0),
                    current_balance: Some(0.0),
                    entity_type: None,
                    entity_id: None,
                    is_control_account: is_control,
                    is_system_account: 1,
                    is_active: 1,
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                };

                self.repo.create_ledger_account(la).await.map_err(|e| e.to_string())?;
            }

            if let Some(ref children) = acc.children {
                if !children.is_empty() {
                    Box::pin(self.seed_coa_recursive(
                        org_id,
                        children,
                        Some(full_id),
                        &path,
                        base_currency,
                    )).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn seed_default_financials(&self, org_id: &str) -> Result<(), String> {
        // 1. Seed Currencies (YER, USD, SAR)
        let default_currencies = vec![
            currency::Model {
                code: "YER".to_string(),
                org_id: org_id.to_string(),
                name: "Yemeni Rial".to_string(),
                name_ar: Some("ريال يمني".to_string()),
                symbol: "﷼".to_string(),
                symbol_position: "after".to_string(),
                decimal_places: 0,
                exchange_rate: 1.0,
                is_base: 1,
                is_active: 1,
                updated_at: Utc::now().to_rfc3339(),
            },
            currency::Model {
                code: "USD".to_string(),
                org_id: org_id.to_string(),
                name: "US Dollar".to_string(),
                name_ar: Some("دولار أمريكي".to_string()),
                symbol: "$".to_string(),
                symbol_position: "before".to_string(),
                decimal_places: 2,
                exchange_rate: 533.0,
                is_base: 0,
                is_active: 1,
                updated_at: Utc::now().to_rfc3339(),
            },
            currency::Model {
                code: "SAR".to_string(),
                org_id: org_id.to_string(),
                name: "Saudi Riyal".to_string(),
                name_ar: Some("ريال سعودي".to_string()),
                symbol: "ر.س".to_string(),
                symbol_position: "after".to_string(),
                decimal_places: 2,
                exchange_rate: 140.0,
                is_base: 0,
                is_active: 1,
                updated_at: Utc::now().to_rfc3339(),
            },
        ];

        for cur in default_currencies {
            if self.repo.get_currency(org_id, &cur.code).await.map_err(|e| e.to_string())?.is_none() {
                self.repo.create_currency(cur).await.map_err(|e| e.to_string())?;
            }
        }

        // 2. Query the actual base currency in the organization's settings
        let base_currency = self.repo.get_currencies(org_id).await
            .map(|list| list.into_iter().find(|c| c.is_base == 1).map(|c| c.code))
            .unwrap_or(None)
            .unwrap_or_else(|| "SAR".to_string()); // Default fallback to SAR if none is marked base

        // 3. Load COA JSON
        let coa_json_str = include_str!("../database/educational_chart_of_accounts.json");
        let coa: CoaJson = serde_json::from_str(coa_json_str)
            .map_err(|e| format!("Failed to parse COA JSON: {}", e))?;

        // 4. Seed the hierarchy recursively
        self.seed_coa_recursive(org_id, &coa.accounts, None, "", &base_currency).await?;

        Ok(())
    }
}
