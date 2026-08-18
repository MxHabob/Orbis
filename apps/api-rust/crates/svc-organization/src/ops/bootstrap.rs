//! Organization Blueprint Composition — unified initialization engine.
//!
//! This is the **single source of truth** for all organization setup data.
//! It replaces the scattered initialization across:
//! - `infra-persistence/bootstrap.rs` (academic_policies + period_templates)
//! - `cli/seeder.rs` (currencies + permissions + period_templates + COA)
//! - `desktop-svelte/completeSetup()` (INSTITUTION_PROFILES)
//!
//! Every blueprint produces the complete seed data for one institution type:
//! academic_policies, grade_scales, calculation_policies, pass_fail_policies,
//! period_templates, default_permissions, currencies, and chart_of_accounts.

use sea_orm::{DatabaseConnection, Statement, ConnectionTrait};
use shared_kernel::errors::AppError;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::info;

// ── Public API ──────────────────────────────────────────────────

/// Request payload for bootstrapping an organization.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BootstrapPayload {
    pub org_name: String,
    pub org_name_ar: Option<String>,
    pub org_type: String,       // "school" | "university" | "institute"
    pub currency: String,       // ISO 4217 code
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub admin_name: String,
    pub admin_username: String,
    pub admin_password_hash: String,
}

/// Result of bootstrapping.
#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapResult {
    pub org_id: String,
    pub admin_id: String,
    pub grade_scale_id: String,
    pub calculation_policy_id: String,
    pub period_template_id: String,
}

/// Main entry point — creates everything from a single call.
pub async fn bootstrap_full(
    db: &DatabaseConnection,
    payload: BootstrapPayload,
) -> Result<BootstrapResult, AppError> {
    let now = Utc::now().to_rfc3339();
    let org_id = Uuid::new_v4().to_string();
    let admin_id = Uuid::new_v4().to_string();

    info!("Bootstrapping full organization '{}' (type={})", payload.org_name, payload.org_type);

    // ── 1. Organization ────────────────────────────────────────
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO organizations (id, name, name_ar, slug, currency, timezone, locale, is_active, subscription_plan, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 1, 'basic', $8, $9)",
        vec![
            org_id.clone().into(),
            payload.org_name.clone().into(),
            payload.org_name_ar.clone().unwrap_or_default().into(),
            slug(&payload.org_name).into(),
            payload.currency.clone().into(),
            payload.timezone.clone().unwrap_or_else(|| "Asia/Riyadh".into()).into(),
            payload.locale.clone().unwrap_or_else(|| "ar".into()).into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create organization: {e}")))?;

    // ── 2. Admin User ──────────────────────────────────────────
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO users (id, org_id, email, full_name, password_hash, status, primary_role, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'active', 'super_admin', $6, $7)",
        vec![
            admin_id.clone().into(),
            org_id.clone().into(),
            payload.admin_username.clone().into(),
            payload.admin_name.clone().into(),
            payload.admin_password_hash.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create admin user: {e}")))?;

    let user_role_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO user_roles (id, user_id, role, org_id, scope_type, is_primary, assigned_by, assigned_at) VALUES ($1, $2, 'super_admin', $3, 'global', 1, 'system', $4)",
        vec![user_role_id.into(), admin_id.clone().into(), org_id.clone().into(), now.clone().into()]
    )).await.map_err(|e| AppError::internal(format!("Failed to assign admin role: {e}")))?;

    // ── 3. Blueprint-specific setup ────────────────────────────
    let blueprint = match payload.org_type.as_str() {
        "university" => university_blueprint(),
        "institute" | "training_center" => institute_blueprint(),
        _ => school_blueprint(),
    };

    // 3a. Academic Policies
    let policy_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_policies (id, org_id, default_grading_system, default_passing_grade, default_max_grade, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        vec![
            policy_id.into(), org_id.clone().into(),
            blueprint.grading_system.clone().into(),
            blueprint.passing_grade.into(),
            100.0f64.into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create academic policies: {e}")))?;

    // 3b. Grade Scale + Items
    let scale_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO grade_scales (id, org_id, name, type, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)",
        vec![
            scale_id.clone().into(), org_id.clone().into(),
            blueprint.scale_name.clone().into(),
            blueprint.grading_system.clone().into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create grade scale: {e}")))?;

    for item in &blueprint.grade_items {
        let item_id = Uuid::new_v4().to_string();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO grade_scale_items (id, scale_id, grade_label, min_score, max_score, gpa_points, is_passing, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            vec![
                item_id.into(), scale_id.clone().into(),
                item.label.clone().into(),
                item.min_score.into(), item.max_score.into(),
                item.gpa_points.into(),
                (if item.is_passing { 1i32 } else { 0i32 }).into(),
                now.clone().into(), now.clone().into(),
            ]
        )).await.map_err(|e| AppError::internal(format!("Failed to create grade scale item: {e}")))?;
    }

    // 3c. Default Calculation Policy (simple weighted sum)
    let calc_policy_id = Uuid::new_v4().to_string();
    let formula = serde_json::json!({
        "type": "weighted_sum",
        "items": [
            { "node": { "type": "variable", "name": "midterm" }, "weight": 0.3 },
            { "node": { "type": "variable", "name": "final" }, "weight": 0.5 },
            { "node": { "type": "variable", "name": "classwork" }, "weight": 0.2 }
        ]
    });
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO calculation_policies (id, org_id, target_period_id, name, version, rule_type, formula_expression, rounding_policy, created_at, updated_at) VALUES ($1, $2, '', $3, 1, 'weighted_sum', $4, 'round_half_up', $5, $6)",
        vec![
            calc_policy_id.clone().into(), org_id.clone().into(),
            format!("Default {} Policy", blueprint.scale_name).into(),
            formula.to_string().into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create calculation policy: {e}")))?;

    // 3d. Pass/Fail Policy
    let pf_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO pass_fail_policies (id, org_id, period_id, name, min_passing_score, requires_all_subjects, max_failed_subjects, supplementary_exam_eligible, created_at, updated_at) VALUES ($1, $2, '', $3, $4, $5, $6, $7, $8, $9)",
        vec![
            pf_id.into(), org_id.clone().into(),
            format!("Default Pass/Fail - {}", blueprint.scale_name).into(),
            blueprint.passing_grade.into(),
            (if blueprint.requires_all_subjects { 1i32 } else { 0 }).into(),
            blueprint.max_failed_subjects.into(),
            (if blueprint.supplementary_eligible { 1i32 } else { 0 }).into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create pass/fail policy: {e}")))?;

    // 3e. Period Template
    let tmpl_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $8)",
        vec![
            tmpl_id.clone().into(), org_id.clone().into(),
            blueprint.period_template_name.clone().into(),
            blueprint.period_template_name_ar.clone().into(),
            payload.org_type.clone().into(),
            serde_json::to_string(&blueprint.period_structure).unwrap().into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| AppError::internal(format!("Failed to create period template: {e}")))?;

    // 3f. Default Role Permissions
    for perm in &blueprint.default_permissions {
        let perm_id = Uuid::new_v4().to_string();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO role_permissions (id, org_id, role, permission, is_granted, created_at) VALUES ($1, $2, $3, $4, 1, $5)",
            vec![
                perm_id.into(), org_id.clone().into(),
                perm.role.clone().into(),
                perm.permission.clone().into(),
                now.clone().into(),
            ]
        )).await.map_err(|e| AppError::internal(format!("Failed to insert permission: {e}")))?;
    }

    // 3g. Currency
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO currencies (code, org_id, name, name_ar, symbol, symbol_position, decimal_places, exchange_rate, is_base, is_active, updated_at) VALUES ($1, $2, $3, $4, $5, 'after', $6, 1.0, 1, 1, $7)",
        vec![
            payload.currency.clone().into(),
            org_id.clone().into(),
            currency_name(&payload.currency).into(),
            currency_name_ar(&payload.currency).into(),
            currency_symbol(&payload.currency).into(),
            2i32.into(),
            now.clone().into(),
        ]
    )).await.ok(); // Ignore duplicate currency errors

    // 3h. Chart of Accounts (COA)
    if let Err(e) = seed_chart_of_accounts(db, &org_id, &payload.currency, &now).await {
        info!("Note: COA seeding warning: {}", e);
    }

    // 3i. OrgSettings Initialization
    let org_settings = serde_json::json!({
        "registration": {
            "require_approval": false,
            "allow_no_guardian": false,
            "auto_assign_fees": true,
            "allow_duplicate_national_id": false
        },
        "finance": {
            "default_currency": payload.currency,
            "tax_rate": 0.0,
            "auto_invoice_on_enrollment": true
        },
        "grading": {
            "default_grading_system": blueprint.grading_system,
            "default_passing_grade": blueprint.passing_grade,
            "default_max_grade": 100.0
        }
    });

    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "UPDATE organizations SET settings = $1 WHERE id = $2",
        vec![org_settings.to_string().into(), org_id.clone().into()]
    )).await.ok();

    info!("Bootstrap completed for org_id={} type={}", org_id, payload.org_type);

    Ok(BootstrapResult {
        org_id,
        admin_id,
        grade_scale_id: scale_id,
        calculation_policy_id: calc_policy_id,
        period_template_id: tmpl_id,
    })
}

// ── COA Seeding Implementation ──────────────────────────────────

const COA_JSON: &str = include_str!("../../../../../../apps/cli/data/educational_chart_of_accounts.json");

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct AccNode {
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    #[serde(rename = "type")]
    pub acc_type: String,
    pub nature: String,
    pub level: i32,
    pub children: Option<Vec<AccNode>>,
}

#[derive(Deserialize, Debug)]
struct CoaData {
    pub accounts: Vec<AccNode>,
}

fn flatten_acc_nodes(
    nodes: &[AccNode],
    org_id: &str,
    parent_id: Option<&str>,
    parent_path: &str,
    depth: i32,
    currency: &str,
    now: &str,
    out: &mut Vec<(String, String, String, String, String, String, Option<String>, String, i32, String, String)>,
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
            now.to_string(),
        ));

        if let Some(ref children) = node.children {
            flatten_acc_nodes(children, org_id, Some(&acc_id), &path, depth + 1, currency, now, out);
        }
    }
}

async fn seed_chart_of_accounts(
    db: &DatabaseConnection,
    org_id: &str,
    currency: &str,
    now: &str,
) -> Result<(), AppError> {
    let coa: CoaData = serde_json::from_str(COA_JSON)
        .map_err(|e| AppError::internal(format!("Failed to parse COA JSON: {e}")))?;

    let mut rows = Vec::new();
    flatten_acc_nodes(&coa.accounts, org_id, None, "", 1, currency, now, &mut rows);

    for (id, oid, code, name, name_ar, db_type, parent_id, path, depth, curr, timestamp) in rows {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO accounts (id, org_id, code, name, name_ar, type, parent_id, path, depth, currency, balances, is_control_account, is_system_account, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, '{}', 0, 0, 1, $11, $12)",
            vec![
                id.into(), oid.into(), code.clone().into(), name.into(), name_ar.into(),
                db_type.into(), parent_id.into(), path.into(), depth.into(),
                curr.into(), timestamp.clone().into(), timestamp.into(),
            ]
        )).await.map_err(|e| AppError::internal(format!("Failed to seed account {code}: {e}")))?;
    }

    Ok(())
}

// ── Internal Blueprint Definitions ──────────────────────────────

struct GradeItem {
    label: String,
    min_score: f64,
    max_score: f64,
    gpa_points: f64,
    is_passing: bool,
}

struct RolePermission {
    role: String,
    permission: String,
}

struct InstitutionBlueprint {
    grading_system: String,
    passing_grade: f64,
    scale_name: String,
    grade_items: Vec<GradeItem>,
    requires_all_subjects: bool,
    max_failed_subjects: i32,
    supplementary_eligible: bool,
    period_template_name: String,
    period_template_name_ar: String,
    period_structure: serde_json::Value,
    default_permissions: Vec<RolePermission>,
}

fn school_blueprint() -> InstitutionBlueprint {
    InstitutionBlueprint {
        grading_system: "percentage".into(),
        passing_grade: 50.0,
        scale_name: "المقياس المدرسي".into(),
        grade_items: vec![
            GradeItem { label: "ممتاز".into(),   min_score: 90.0, max_score: 100.0, gpa_points: 4.0, is_passing: true },
            GradeItem { label: "جيد جداً".into(), min_score: 80.0, max_score: 89.99, gpa_points: 3.5, is_passing: true },
            GradeItem { label: "جيد".into(),      min_score: 70.0, max_score: 79.99, gpa_points: 3.0, is_passing: true },
            GradeItem { label: "مقبول".into(),    min_score: 60.0, max_score: 69.99, gpa_points: 2.5, is_passing: true },
            GradeItem { label: "ضعيف".into(),    min_score: 50.0, max_score: 59.99, gpa_points: 2.0, is_passing: false },
            GradeItem { label: "راسب".into(),    min_score: 0.0,  max_score: 49.99, gpa_points: 0.0, is_passing: false },
        ],
        requires_all_subjects: false,
        max_failed_subjects: 2,
        supplementary_eligible: true,
        period_template_name: "العام الدراسي".into(),
        period_template_name_ar: "العام الدراسي".into(),
        period_structure: serde_json::json!([
            {
                "name": "الفصل الأول", "name_ar": "الفصل الأول", "type": "semester", "weight": 50,
                "children": [
                    { "name": "الفترة الأولى", "type": "quarter", "weight": 25 },
                    { "name": "الفترة الثانية", "type": "quarter", "weight": 25 }
                ]
            },
            {
                "name": "الفصل الثاني", "name_ar": "الفصل الثاني", "type": "semester", "weight": 50,
                "children": [
                    { "name": "الفترة الثالثة", "type": "quarter", "weight": 25 },
                    { "name": "الفترة الرابعة", "type": "quarter", "weight": 25 }
                ]
            }
        ]),
        default_permissions: default_role_permissions(),
    }
}

fn university_blueprint() -> InstitutionBlueprint {
    InstitutionBlueprint {
        grading_system: "gpa_4_0".into(),
        passing_grade: 60.0,
        scale_name: "GPA 4.0 Scale".into(),
        grade_items: vec![
            GradeItem { label: "A+".into(), min_score: 95.0, max_score: 100.0, gpa_points: 4.0, is_passing: true },
            GradeItem { label: "A".into(),  min_score: 90.0, max_score: 94.99, gpa_points: 3.75, is_passing: true },
            GradeItem { label: "B+".into(), min_score: 85.0, max_score: 89.99, gpa_points: 3.5, is_passing: true },
            GradeItem { label: "B".into(),  min_score: 80.0, max_score: 84.99, gpa_points: 3.0, is_passing: true },
            GradeItem { label: "C+".into(), min_score: 75.0, max_score: 79.99, gpa_points: 2.5, is_passing: true },
            GradeItem { label: "C".into(),  min_score: 70.0, max_score: 74.99, gpa_points: 2.0, is_passing: true },
            GradeItem { label: "D+".into(), min_score: 65.0, max_score: 69.99, gpa_points: 1.5, is_passing: true },
            GradeItem { label: "D".into(),  min_score: 60.0, max_score: 64.99, gpa_points: 1.0, is_passing: true },
            GradeItem { label: "F".into(),  min_score: 0.0,  max_score: 59.99, gpa_points: 0.0, is_passing: false },
        ],
        requires_all_subjects: true,
        max_failed_subjects: 0,
        supplementary_eligible: false,
        period_template_name: "University Year".into(),
        period_template_name_ar: "العام الجامعي".into(),
        period_structure: serde_json::json!([
            { "name": "Fall Semester", "name_ar": "الفصل الخريفي", "type": "semester", "weight": 40 },
            { "name": "Spring Semester", "name_ar": "الفصل الربيعي", "type": "semester", "weight": 40 },
            { "name": "Summer Term", "name_ar": "الفصل الصيفي", "type": "term", "weight": 20 }
        ]),
        default_permissions: default_role_permissions(),
    }
}

fn institute_blueprint() -> InstitutionBlueprint {
    InstitutionBlueprint {
        grading_system: "pass_fail".into(),
        passing_grade: 70.0,
        scale_name: "Pass/Fail Scale".into(),
        grade_items: vec![
            GradeItem { label: "ناجح".into(), min_score: 70.0, max_score: 100.0, gpa_points: 1.0, is_passing: true },
            GradeItem { label: "راسب".into(), min_score: 0.0,  max_score: 69.99, gpa_points: 0.0, is_passing: false },
        ],
        requires_all_subjects: true,
        max_failed_subjects: 0,
        supplementary_eligible: false,
        period_template_name: "Training Year".into(),
        period_template_name_ar: "سنة التدريب".into(),
        period_structure: serde_json::json!([
            { "name": "Q1", "name_ar": "الربع الأول", "type": "quarter", "weight": 25 },
            { "name": "Q2", "name_ar": "الربع الثاني", "type": "quarter", "weight": 25 },
            { "name": "Q3", "name_ar": "الربع الثالث", "type": "quarter", "weight": 25 },
            { "name": "Q4", "name_ar": "الربع الرابع", "type": "quarter", "weight": 25 }
        ]),
        default_permissions: default_role_permissions(),
    }
}

// ── Shared Defaults ─────────────────────────────────────────────

fn default_role_permissions() -> Vec<RolePermission> {
    let mut perms = Vec::new();

    // Teacher
    for p in &["grade.read", "grade.write", "attendance.read", "attendance.write", "student.read", "class.read", "assessment.read", "assessment.write"] {
        perms.push(RolePermission { role: "teacher".into(), permission: p.to_string() });
    }

    // Student
    for p in &["grade.read_own", "attendance.read_own", "enrollment.read_own", "schedule.read"] {
        perms.push(RolePermission { role: "student".into(), permission: p.to_string() });
    }

    // Parent
    for p in &["grade.read_children", "attendance.read_children", "enrollment.read_children", "payment.read_own"] {
        perms.push(RolePermission { role: "parent".into(), permission: p.to_string() });
    }

    // Principal
    for p in &["grade.read", "grade.write", "grade.analytics", "grade.report", "attendance.read", "attendance.write", "student.read", "student.write", "class.read", "class.write", "staff.read", "settings.read"] {
        perms.push(RolePermission { role: "principal".into(), permission: p.to_string() });
    }

    // Staff
    for p in &["student.read", "student.write", "enrollment.read", "enrollment.write", "payment.read", "payment.write", "class.read"] {
        perms.push(RolePermission { role: "staff".into(), permission: p.to_string() });
    }

    perms
}

// ── Utility Helpers ─────────────────────────────────────────────

fn slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn currency_name(code: &str) -> String {
    match code {
        "SAR" => "Saudi Riyal".into(),
        "AED" => "UAE Dirham".into(),
        "KWD" => "Kuwaiti Dinar".into(),
        "QAR" => "Qatari Riyal".into(),
        "BHD" => "Bahraini Dinar".into(),
        "OMR" => "Omani Rial".into(),
        "EGP" => "Egyptian Pound".into(),
        "JOD" => "Jordanian Dinar".into(),
        "USD" => "US Dollar".into(),
        "EUR" => "Euro".into(),
        "GBP" => "British Pound".into(),
        _ => code.to_string(),
    }
}

fn currency_name_ar(code: &str) -> String {
    match code {
        "SAR" => "ريال سعودي".into(),
        "AED" => "درهم إماراتي".into(),
        "KWD" => "دينار كويتي".into(),
        "QAR" => "ريال قطري".into(),
        "BHD" => "دينار بحريني".into(),
        "OMR" => "ريال عماني".into(),
        "EGP" => "جنيه مصري".into(),
        "JOD" => "دينار أردني".into(),
        "USD" => "دولار أمريكي".into(),
        "EUR" => "يورو".into(),
        "GBP" => "جنيه إسترليني".into(),
        _ => code.to_string(),
    }
}

fn currency_symbol(code: &str) -> String {
    match code {
        "SAR" => "ر.س".into(),
        "AED" => "د.إ".into(),
        "KWD" => "د.ك".into(),
        "QAR" => "ر.ق".into(),
        "BHD" => "د.ب".into(),
        "OMR" => "ر.ع".into(),
        "EGP" => "ج.م".into(),
        "JOD" => "د.أ".into(),
        "USD" => "$".into(),
        "EUR" => "€".into(),
        "GBP" => "£".into(),
        _ => code.to_string(),
    }
}
