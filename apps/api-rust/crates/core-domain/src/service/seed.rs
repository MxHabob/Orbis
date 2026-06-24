use uuid::Uuid;
use chrono::Utc;
use crate::entities::{currency, ledger_account};

impl super::Service {
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
                exchange_rate: 250.0, // Default rate (or customized per school)
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
                exchange_rate: 66.0,
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

        // 2. Seed Chart of Accounts
        // Format: (id, parent_id, code, name, name_ar, type, sub_type, is_control, is_system)
        let coa_structure = vec![
            // الأصول
            ("assets", None, "1000", "Assets", "الأصول", "asset", None, 1, 1),
            ("current_assets", Some("assets"), "1100", "Current Assets", "الأصول المتداولة", "asset", None, 1, 1),
            ("cash_treasury_control", Some("current_assets"), "1110", "Cash & Treasury Control", "حساب مراقبة النقدية والخزائن", "asset", Some("treasury_control"), 1, 1),
            ("student_receivable_control", Some("current_assets"), "1120", "Accounts Receivable - Students", "ذمم الطلاب المدينين", "asset", Some("student_receivable"), 1, 1),
            
            // الالتزامات
            ("liabilities", None, "2000", "Liabilities", "الالتزامات", "liability", None, 1, 1),
            ("current_liabilities", Some("liabilities"), "2100", "Current Liabilities", "الالتزامات المتداولة", "liability", None, 1, 1),
            ("employee_payable_control", Some("current_liabilities"), "2110", "Accounts Payable - Employees", "ذمم الموظفين الدائنة", "liability", Some("employee_payable"), 1, 1),
            
            // حقوق الملكية
            ("equity", None, "3000", "Equity", "حقوق الملكية", "equity", None, 1, 1),
            ("retained_earnings", Some("equity"), "3100", "Retained Earnings", "الأرباح المحتجزة", "equity", Some("retained_earnings"), 0, 1),
            
            // الإيرادات
            ("revenue", None, "4000", "Revenue", "الإيرادات", "revenue", None, 1, 1),
            ("tuition_revenue", Some("revenue"), "4100", "Tuition Revenue", "إيرادات الرسوم الدراسية", "revenue", Some("tuition_revenue"), 0, 1),
            ("other_revenue", Some("revenue"), "4200", "Other School Revenue", "إيرادات مدرسية أخرى", "revenue", None, 0, 1),
            
            // المصروفات
            ("expenses", None, "5000", "Expenses", "المصروفات", "expense", None, 1, 1),
            ("salaries_expense", Some("expenses"), "5100", "Academic Salaries Expense", "مصروفات رواتب الكادر التعليمي", "expense", Some("salaries_expense"), 0, 1),
            ("operational_expense", Some("expenses"), "5200", "Operational Expenses", "المصروفات التشغيلية", "expense", None, 0, 1),
        ];

        for (id_suffix, parent_suffix, code, name, name_ar, item_type, sub_type, is_control, is_system) in coa_structure {
            let full_id = format!("{}_{}", org_id, id_suffix);
            let full_parent_id = parent_suffix.map(|s| format!("{}_{}", org_id, s));

            if self.repo.get_ledger_account_by_id(&full_id).await.map_err(|e| e.to_string())?.is_none() {
                // Determine path and depth
                let (path, depth) = if let Some(ref pid) = full_parent_id {
                    let parent = self.repo.get_ledger_account_by_id(pid).await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| "Parent account not found".to_string())?;
                    (format!("{}/{}", parent.path, full_id), parent.depth + 1)
                } else {
                    (full_id.clone(), 0)
                };

                let la = ledger_account::Model {
                    id: full_id,
                    org_id: org_id.to_string(),
                    code: code.to_string(),
                    name: name.to_string(),
                    name_ar: Some(name_ar.to_string()),
                    item_type: item_type.to_string(),
                    sub_type: sub_type.map(|s| s.to_string()),
                    parent_id: full_parent_id,
                    path,
                    depth,
                    currency: "YER".to_string(), // Default base currency
                    opening_balance: Some(0.0),
                    current_balance: Some(0.0),
                    entity_type: None,
                    entity_id: None,
                    is_control_account: is_control,
                    is_system_account: is_system,
                    is_active: 1,
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                };

                self.repo.create_ledger_account(la).await.map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}
