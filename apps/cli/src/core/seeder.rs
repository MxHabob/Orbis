use anyhow::Result;
use colored::*;
use sea_orm::{Statement, DatabaseBackend, ConnectionTrait, DatabaseConnection};
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

pub async fn seed_essential_data(db: &DatabaseConnection, org_id: &str) -> Result<()> {
    println!("{}", "[*] Seeding essential educational data...".cyan());
    let now = chrono::Local::now().to_rfc3339();

    // Currencies
    let currencies = vec![
        ("SAR", "Saudi Riyal", "ريال سعودي", "ر.س"),
        ("YER", "Yemeni Rial", "ريال يمني", "ر.ي"),
        ("USD", "US Dollar", "دولار أمريكي", "$"),
        ("CNY", "Chinese Yuan", "يوان صيني", "¥"),
        ("QAR", "Qatari Riyal", "ريال قطري", "ر.ق"),
        ("AED", "UAE Dirham", "درهم إماراتي", "د.إ"),
    ];
    for (code, name, name_ar, symbol) in currencies {
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "INSERT INTO currencies (code, org_id, name, name_ar, symbol, symbol_position, decimal_places, exchange_rate, is_base, is_active, updated_at) VALUES (?, ?, ?, ?, ?, 'right', 2, 1.0, ?, 1, ?) ON CONFLICT DO NOTHING",
            vec![code.into(), org_id.to_string().into(), name.into(), name_ar.into(), symbol.into(), (if code == "SAR" { 1 } else { 0 }).into(), now.clone().into()]
        )).await?;
    }

    // Period Templates
    let tpl1_id = format!("tpl-2-semesters-{}", org_id);
    let tpl1_structure = r#"[{"name":"Semester 1","name_ar":"الفصل الدراسي الأول","type":"term","weight":50,"children":[{"name":"Month 1","name_ar":"الشهر الأول","type":"month","weight":25},{"name":"Month 2","name_ar":"الشهر الثاني","type":"month","weight":25},{"name":"Month 3","name_ar":"الشهر الثالث","type":"month","weight":25},{"name":"Month 4 (Final)","name_ar":"الشهر الرابع (النهائي)","type":"month","weight":25}]},{"name":"Semester 2","name_ar":"الفصل الدراسي الثاني","type":"term","weight":50,"children":[{"name":"Month 1","name_ar":"الشهر الأول","type":"month","weight":25},{"name":"Month 2","name_ar":"الشهر الثاني","type":"month","weight":25},{"name":"Month 3","name_ar":"الشهر الثالث","type":"month","weight":25},{"name":"Month 4 (Final)","name_ar":"الشهر الرابع (النهائي)","type":"month","weight":25}]}]"#;
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) VALUES (?, ?, 'Two Semesters (4 Months each)', 'نظام فصلين دراسيين', 'school', ?, 1, ?, ?) ON CONFLICT DO NOTHING",
        vec![tpl1_id.into(), org_id.to_string().into(), tpl1_structure.into(), now.clone().into(), now.clone().into()]
    )).await?;

    let tpl2_id = format!("tpl-3-trimesters-{}", org_id);
    let tpl2_structure = r#"[{"name":"Trimester 1","name_ar":"الفصل الدراسي الأول","type":"term","weight":33.33,"children":[{"name":"Mid-Term","name_ar":"منتصف الفصل","type":"quarter","weight":50},{"name":"Final","name_ar":"النهائي","type":"quarter","weight":50}]},{"name":"Trimester 2","name_ar":"الفصل الدراسي الثاني","type":"term","weight":33.33,"children":[{"name":"Mid-Term","name_ar":"منتصف الفصل","type":"quarter","weight":50},{"name":"Final","name_ar":"النهائي","type":"quarter","weight":50}]},{"name":"Trimester 3","name_ar":"الفصل الدراسي الثالث","type":"term","weight":33.34,"children":[{"name":"Mid-Term","name_ar":"منتصف الفصل","type":"quarter","weight":50},{"name":"Final","name_ar":"النهائي","type":"quarter","weight":50}]}]"#;
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) VALUES (?, ?, 'Three Trimesters', 'نظام 3 فصول دراسية', 'school', ?, 0, ?, ?) ON CONFLICT DO NOTHING",
        vec![tpl2_id.into(), org_id.to_string().into(), tpl2_structure.into(), now.clone().into(), now.clone().into()]
    )).await?;

    let tpl3_id = format!("tpl-4-quarters-{}", org_id);
    let tpl3_structure = r#"[{"name":"Quarter 1","name_ar":"الربع الأول","type":"quarter","weight":25},{"name":"Quarter 2","name_ar":"الربع الثاني","type":"quarter","weight":25},{"name":"Quarter 3","name_ar":"الربع الثالث","type":"quarter","weight":25},{"name":"Quarter 4","name_ar":"الربع الرابع","type":"quarter","weight":25}]"#;
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) VALUES (?, ?, 'Four Quarters', 'نظام 4 أرباع', 'school', ?, 0, ?, ?) ON CONFLICT DO NOTHING",
        vec![tpl3_id.into(), org_id.to_string().into(), tpl3_structure.into(), now.clone().into(), now.clone().into()]
    )).await?;

    // Chart of Accounts
    let coa_json_str = include_str!("../../data/educational_chart_of_accounts.json");
    if let Ok(chart) = serde_json::from_str::<ChartOfAccounts>(coa_json_str) {
        let mut flat_accounts = Vec::new();
        flatten_accounts(&chart.accounts, org_id, None, "", 0, &chart.metadata.currency, &mut flat_accounts);
        
        for acc in flat_accounts {
            db.execute(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "INSERT INTO accounts (id, org_id, code, name, name_ar, type, parent_id, path, depth, currency, balances, is_control_account, is_system_account, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, '{}', 0, 0, 1, ?, ?) ON CONFLICT DO NOTHING",
                vec![
                    acc.0.into(), acc.1.into(), acc.2.into(), acc.3.into(), acc.4.into(), acc.5.into(),
                    acc.6.into(), 
                    acc.7.into(), acc.8.into(), acc.9.into(), now.clone().into(), now.clone().into()
                ]
            )).await?;
        }
        println!("{}", "[+] Chart of Accounts seeded successfully.".green());
    } else {
        println!("{}", "[!] Warning: Could not parse educational_chart_of_accounts.json".yellow());
    }

    // Default Role Permissions
    let default_perms = vec![
        // Teacher
        ("teacher", "Class", "[\"read\"]"),
        ("teacher", "Subject", "[\"read\"]"),
        ("teacher", "Student", "[\"read\"]"),
        ("teacher", "Attendance", "[\"read\", \"write\"]"),
        ("teacher", "Behavior", "[\"read\", \"write\"]"),
        ("teacher", "Grade", "[\"read\", \"write\"]"),
        ("teacher", "Assessment Result", "[\"read\", \"write\"]"),
        ("teacher", "Document", "[\"read\", \"write\", \"delete\"]"),
        ("teacher", "Schedule", "[\"read\"]"),
        ("teacher", "Exam", "[\"read\"]"),
        
        // Student
        ("student", "Class", "[\"read\"]"),
        ("student", "Subject", "[\"read\"]"),
        ("student", "Attendance", "[\"read\"]"),
        ("student", "Behavior", "[\"read\"]"),
        ("student", "Grade", "[\"read\"]"),
        ("student", "Assessment Result", "[\"read\"]"),
        ("student", "Document", "[\"read\"]"),
        ("student", "Schedule", "[\"read\"]"),
        ("student", "Exam", "[\"read\"]"),

        // Parent
        ("parent", "Class", "[\"read\"]"),
        ("parent", "Subject", "[\"read\"]"),
        ("parent", "Attendance", "[\"read\"]"),
        ("parent", "Behavior", "[\"read\"]"),
        ("parent", "Grade", "[\"read\"]"),
        ("parent", "Assessment Result", "[\"read\"]"),
        ("parent", "Document", "[\"read\"]"),
        ("parent", "Schedule", "[\"read\"]"),
        ("parent", "Exam", "[\"read\"]"),
        ("parent", "Finance", "[\"read\"]"),

        // Principal
        ("principal", "Class", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Subject", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Student", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Teacher", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Attendance", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Behavior", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Grade", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Assessment Result", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Document", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Schedule", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Exam", "[\"read\", \"write\", \"delete\"]"),
        ("principal", "Dashboard", "[\"read\"]"),
        ("principal", "Department", "[\"read\", \"write\"]"),

        // Staff
        ("staff", "Class", "[\"read\"]"),
        ("staff", "Subject", "[\"read\"]"),
        ("staff", "Student", "[\"read\"]"),
        ("staff", "Document", "[\"read\", \"write\"]"),
    ];

    for (role, doc_type, actions) in default_perms {
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "INSERT INTO role_permissions (role, document_type, level, only_if_creator, actions, created_at) VALUES (?, ?, 0, 0, ?, ?) ON CONFLICT DO NOTHING",
            vec![role.into(), doc_type.into(), actions.into(), now.clone().into()]
        )).await?;
    }
    
    println!("{}", "[+] Default Role Permissions seeded successfully.".green());

    Ok(())
}
