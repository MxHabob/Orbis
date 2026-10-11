//! Chart of Accounts Blueprint Defaults & Loader.
//!
//! Provides embedded, institution-specific COA blueprints for Orbis ERP/SIS.
//! Embedded directly into the binary via `include_str!` to guarantee self-contained
//! runtime execution across Local (Tauri) and Remote (Cloud) deployments.

use chrono::Utc;
use orbis_domain::entities::account::{Account, AccountType};
use serde::Deserialize;
use uuid::Uuid;

const COA_SCHOOL: &str = include_str!("../defaults/coa/orbis_coa_school_v2.json");
const COA_UNIVERSITY: &str = include_str!("../defaults/coa/orbis_coa_university_v2.json");
const COA_INSTITUTE: &str = include_str!("../defaults/coa/orbis_coa_institute_v2.json");
const COA_VOCATIONAL: &str = include_str!("../defaults/coa/orbis_coa_vocational_training_v2.json");
const COA_NONPROFIT: &str = include_str!("../defaults/coa/orbis_coa_nonprofit_education_v2.json");

#[derive(Debug, Clone, Deserialize)]
pub struct CoaMetadata {
    pub name: String,
    pub name_en: String,
    pub description: Option<String>,
    pub blueprint_id: String,
    pub version: Option<String>,
    pub levels: Option<u32>,
    pub account_count: Option<usize>,
    pub leaf_account_count: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoaNode {
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub nature: Option<String>,
    pub level: u32,
    pub children: Option<Vec<CoaNode>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoaFile {
    pub metadata: CoaMetadata,
    pub accounts: Vec<CoaNode>,
}

pub struct CoaBlueprintLoader;

impl CoaBlueprintLoader {
    /// Selects the raw JSON string corresponding to the given institution blueprint type.
    pub fn get_blueprint_json(institution_type: &str) -> &'static str {
        match institution_type.trim().to_lowercase().as_str() {
            "university" | "higher_education" => COA_UNIVERSITY,
            "institute" | "language_center" | "training_center" => COA_INSTITUTE,
            "vocational" | "vocational_institute" | "vocational_training" => COA_VOCATIONAL,
            "nonprofit" | "nonprofit_education" | "charity" => COA_NONPROFIT,
            _ => COA_SCHOOL, // Default: K12 School
        }
    }

    /// Parses the blueprint metadata without inflating all accounts.
    pub fn get_metadata(institution_type: &str) -> Result<CoaMetadata, serde_json::Error> {
        let json_str = Self::get_blueprint_json(institution_type);
        let coa_file: CoaFile = serde_json::from_str(json_str)?;
        Ok(coa_file.metadata)
    }

    /// Recursively inflates the hierarchical chart of accounts for an organization.
    ///
    /// - Computes deterministic paths (e.g. `/1/11/1101/`)
    /// - Assigns fresh `Uuid`s and parent relationships
    /// - Binds the organization's designated base currency
    /// - Sets `is_control_account = true` for branch nodes and `false` for leaf nodes
    pub fn parse_accounts(
        org_id: Uuid,
        currency_code: &str,
        institution_type: &str,
    ) -> Result<Vec<Account>, serde_json::Error> {
        let json_str = Self::get_blueprint_json(institution_type);
        let coa_file: CoaFile = serde_json::from_str(json_str)?;
        let mut flat_accounts = Vec::new();
        let now = Utc::now();

        fn traverse(
            node: &CoaNode,
            org_id: Uuid,
            currency_code: &str,
            parent_id: Option<Uuid>,
            parent_path: &str,
            now: chrono::DateTime<Utc>,
            list: &mut Vec<Account>,
        ) {
            let account_id = Uuid::new_v4();
            let current_path = format!("{}{}/", parent_path, node.code);
            let has_children = node.children.as_ref().map_or(false, |c| !c.is_empty());

            let acc_type = match node.account_type.trim().to_uppercase().as_str() {
                "ASSET" => AccountType::Asset,
                "LIABILITY" => AccountType::Liability,
                "EQUITY" => AccountType::Equity,
                "REVENUE" => AccountType::Revenue,
                "EXPENSE" => AccountType::Expense,
                _ => AccountType::Asset,
            };

            list.push(Account {
                id: account_id,
                org_id,
                code: node.code.clone(),
                name: node.name_en.clone(),
                name_ar: Some(node.name_ar.clone()),
                account_type: acc_type,
                sub_type: None,
                parent_id,
                path: current_path.clone(),
                depth: node.level,
                currency_code: currency_code.to_string(),
                entity_type: None,
                entity_id: None,
                is_control_account: has_children,
                is_system_account: true,
                is_active: true,
                balances: serde_json::json!({ "debit": 0, "credit": 0, "net": 0 }),
                created_by: None,
                updated_by: None,
                version: 1,
                created_at: now,
                updated_at: now,
            });

            if let Some(children) = &node.children {
                for child in children {
                    traverse(
                        child,
                        org_id,
                        currency_code,
                        Some(account_id),
                        &current_path,
                        now,
                        list,
                    );
                }
            }
        }

        for root_node in &coa_file.accounts {
            traverse(
                root_node,
                org_id,
                currency_code,
                None,
                "/",
                now,
                &mut flat_accounts,
            );
        }

        Ok(flat_accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_school_coa_parses_successfully() {
        let org_id = Uuid::new_v4();
        let accounts = CoaBlueprintLoader::parse_accounts(org_id, "SAR", "k12_school")
            .expect("School COA JSON should parse cleanly");

        assert!(!accounts.is_empty());
        assert!(
            accounts.len() > 400,
            "School COA should contain over 400 accounts, got {}",
            accounts.len()
        );

        let roots: Vec<_> = accounts.iter().filter(|a| a.parent_id.is_none()).collect();
        assert_eq!(
            roots.len(),
            5,
            "Expected 5 standard root accounts (Assets, Liabilities, Equity, Revenue, Expense)"
        );
        for r in &roots {
            assert_eq!(r.depth, 1);
            assert_eq!(r.org_id, org_id);
            assert_eq!(r.currency_code, "SAR");
            assert!(r.is_control_account);
        }

        let children: Vec<_> = accounts.iter().filter(|a| a.parent_id.is_some()).collect();
        assert!(!children.is_empty());
        for c in &children {
            assert!(c.path.starts_with('/'));
            assert!(c.path.ends_with('/'));
            assert!(c.depth > 1);
        }
    }

    #[test]
    fn test_default_coa_blueprints_are_institution_neutral() {
        for blueprint in [COA_SCHOOL, COA_UNIVERSITY, COA_INSTITUTE, COA_VOCATIONAL] {
            let lower = blueprint.to_ascii_lowercase();
            assert!(!lower.contains("middle school"));
            assert!(!lower.contains("high school"));
            assert!(!lower.contains("kindergarten"));
            assert!(!lower.contains("primary"));
        }
    }

    #[test]
    fn test_university_coa_parses_successfully() {
        let org_id = Uuid::new_v4();
        let accounts = CoaBlueprintLoader::parse_accounts(org_id, "USD", "university")
            .expect("University COA JSON should parse cleanly");

        assert!(accounts.len() > 400);
        assert_eq!(accounts[0].currency_code, "USD");
    }

    #[test]
    fn test_institute_coa_parses_successfully() {
        let org_id = Uuid::new_v4();
        let accounts = CoaBlueprintLoader::parse_accounts(org_id, "YER", "language_center")
            .expect("Institute COA JSON should parse cleanly");

        assert!(accounts.len() > 400);
        assert_eq!(accounts[0].currency_code, "YER");
    }

    #[test]
    fn test_vocational_coa_parses_successfully() {
        let org_id = Uuid::new_v4();
        let accounts = CoaBlueprintLoader::parse_accounts(org_id, "AED", "vocational_institute")
            .expect("Vocational COA JSON should parse cleanly");

        assert!(accounts.len() > 400);
        assert_eq!(accounts[0].currency_code, "AED");
    }

    #[test]
    fn test_nonprofit_coa_parses_successfully() {
        let org_id = Uuid::new_v4();
        let accounts = CoaBlueprintLoader::parse_accounts(org_id, "EUR", "nonprofit_education")
            .expect("Nonprofit COA JSON should parse cleanly");

        assert!(accounts.len() > 400);
        assert_eq!(accounts[0].currency_code, "EUR");
    }
}
