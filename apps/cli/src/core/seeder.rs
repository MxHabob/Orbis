use anyhow::Result;
use colored::*;
use sea_orm::{Statement, ConnectionTrait, DatabaseConnection};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ChartMetadata {
    pub currency: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AccountNode {
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    #[serde(rename = "type")]
    pub acc_type: String,
    pub nature: String,
    pub level: i32,
    pub children: Option<Vec<AccountNode>>,
}

#[derive(Deserialize, Debug)]
struct ChartOfAccounts {
    pub metadata: ChartMetadata,
    pub accounts: Vec<AccountNode>,
}

fn flatten_accounts(
    nodes: &[AccountNode],
    org_id: &str,
    parent_id: Option<&str>,
    parent_path: &str,
    depth: i32,
    currency: &str,
    out: &mut Vec<(String, String, String, String, String, String, Option<String>, String, i32, String)>
) {
    for node in nodes {
        let acc_id = format!("{}-acc-{}", org_id, node.code);
        let path = if parent_path.is_empty() {
            node.code.clone()
        } else {
            format!("{}.{}", parent_path, node.code)
        };
        let db_type = node.acc_type.to_lowercase();
        
        out.push((
            acc_id.clone(),
            org_id.to_string(),
            node.code.clone(),
            node.name_en.clone(),
            node.name_ar.clone(),
            db_type,
            parent_id.map(|s| s.to_string()),
            path.clone(),
            depth,
            currency.to_string(),
        ));
        
        if let Some(children) = &node.children {
            flatten_accounts(children, org_id, Some(&acc_id), &path, depth + 1, currency, out);
        }
    }
}

/// Seed the Chart of Accounts from the embedded JSON file.
///
/// All other essential data (currencies, period templates, grade scales,
/// permissions) is now created by the unified `bootstrap_full()` in
/// `svc-organization::ops::bootstrap`. This function only handles COA
/// which requires the large embedded JSON dataset.
pub async fn seed_chart_of_accounts(db: &DatabaseConnection, org_id: &str) -> Result<()> {
    println!("{}", "[*] Seeding Chart of Accounts...".cyan());
    let now = chrono::Local::now().to_rfc3339();

    // Use the DB backend from the connection (works with both SQLite and PostgreSQL)
    let backend = db.get_database_backend();

    let coa_json_str = include_str!("../../data/educational_chart_of_accounts.json");
    if let Ok(chart) = serde_json::from_str::<ChartOfAccounts>(coa_json_str) {
        let mut flat_accounts = Vec::new();
        flatten_accounts(&chart.accounts, org_id, None, "", 0, &chart.metadata.currency, &mut flat_accounts);
        
        let mut inserted = 0usize;
        for acc in flat_accounts {
            let result = db.execute(Statement::from_sql_and_values(
                backend,
                "INSERT INTO accounts (id, org_id, code, name, name_ar, type, parent_id, path, depth, currency, balances, is_control_account, is_system_account, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, '{}', 0, 0, 1, $11, $12) ON CONFLICT DO NOTHING",
                vec![
                    acc.0.into(), acc.1.into(), acc.2.into(), acc.3.into(), acc.4.into(), acc.5.into(),
                    acc.6.into(), 
                    acc.7.into(), acc.8.into(), acc.9.into(), now.clone().into(), now.clone().into()
                ]
            )).await;
            
            if result.is_ok() {
                inserted += 1;
            }
        }
        println!("{}", format!("[+] Chart of Accounts seeded: {} accounts inserted.", inserted).green());
    } else {
        println!("{}", "[!] Warning: Could not parse educational_chart_of_accounts.json".yellow());
    }

    Ok(())
}

/// Legacy compatibility wrapper — calls seed_chart_of_accounts only.
/// Currencies, period templates, and permissions are now handled by bootstrap_full().
#[deprecated(note = "Use seed_chart_of_accounts() directly. Other data is created by bootstrap_full().")]
pub async fn seed_essential_data(db: &DatabaseConnection, org_id: &str) -> Result<()> {
    seed_chart_of_accounts(db, org_id).await
}
