//! Organization Blueprint Composition — unified initialization engine.
//!
//! This is the **single source of truth** for all organization setup data.
//! It replaces the scattered initialization across:
//! - `infra-persistence/bootstrap.rs` (academic_policies + period_templates)
//! - `cli/seeder.rs` (currencies + permissions + period_templates + COA)
//! - `desktop/completeSetup()` (INSTITUTION_PROFILES)
//!
//! Every blueprint produces the complete seed data for one institution type:
//! academic_policies, grade_scales, calculation_policies, pass_fail_policies,
//! period_templates, default_permissions, currencies, and chart_of_accounts.

use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, TransactionTrait};
use orbis_domain::errors::DomainError;
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
) -> Result<BootstrapResult, DomainError> {
    let txn = db
        .begin()
        .await
        .map_err(|e| DomainError::Internal(format!("Failed to begin bootstrap transaction: {e}")))?;

    match bootstrap_full_inner(&txn, payload).await {
        Ok(result) => {
            txn.commit()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to commit bootstrap transaction: {e}")))?;
            Ok(result)
        }
        Err(error) => Err(error),
    }
}

async fn bootstrap_full_inner<C: ConnectionTrait>(
    db: &C,
    payload: BootstrapPayload,
) -> Result<BootstrapResult, DomainError> {
    let now = Utc::now().to_rfc3339();
    let org_id = Uuid::new_v4().to_string();
    let admin_id = Uuid::new_v4().to_string();

    info!("Bootstrapping full organization '{}' (type={})", payload.org_name, payload.org_type);

    // ── 1. Organization ────────────────────────────────────────
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO organizations (id, name, name_ar, slug, timezone, locale, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $8)",
        vec![
            org_id.clone().into(),
            payload.org_name.clone().into(),
            payload.org_name_ar.clone().unwrap_or_default().into(),
            slug(&payload.org_name).into(),
            payload.timezone.clone().unwrap_or_else(|| "Asia/Riyadh".into()).into(),
            payload.locale.clone().unwrap_or_else(|| "ar".into()).into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create organization: {e}")))?;

    // ── 1.5 Admin Party & Person ───────────────────────────────
    let party_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO parties (id, party_type, created_at, updated_at) VALUES ($1, 'person', $2, $3)",
        vec![
            party_id.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create admin party: {e}")))?;

    let person_id = Uuid::new_v4().to_string();
    let name_parts: Vec<&str> = payload.admin_name.split_whitespace().collect();
    let first_name = name_parts.first().unwrap_or(&"").to_string();
    let last_name = if name_parts.len() > 1 { name_parts[1..].join(" ") } else { first_name.clone() };

    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO persons (id, party_id, org_id, first_name, last_name, full_name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        vec![
            person_id.clone().into(),
            party_id.clone().into(),
            org_id.clone().into(),
            first_name.into(),
            last_name.into(),
            payload.admin_name.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create admin person: {e}")))?;

    // ── 2. Admin User ──────────────────────────────────────────
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO users (id, person_id, org_id, email, full_name, password_hash, status, primary_role, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, 'active', 'super_admin', $7, $8)",
        vec![
            admin_id.clone().into(),
            person_id.clone().into(),
            org_id.clone().into(),
            payload.admin_username.clone().into(),
            payload.admin_name.clone().into(),
            payload.admin_password_hash.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create admin user: {e}")))?;

    let user_role_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO user_roles (id, user_id, role, org_id, scope_type, is_primary, assigned_by, assigned_at) VALUES ($1, $2, 'super_admin', $3, 'global', 1, 'system', $4)",
        vec![user_role_id.into(), admin_id.clone().into(), org_id.clone().into(), now.clone().into()]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to assign admin role: {e}")))?;

    // ── 3. Blueprint-specific setup ────────────────────────────
    let blueprint = match payload.org_type.as_str() {
        "university" => university_blueprint(),
        "institute" | "training_center" => institute_blueprint(),
        _ => school_blueprint(),
    };

    // 3a. Grade Scale + Items
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
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create grade scale: {e}")))?;

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
        )).await.map_err(|e| DomainError::Internal(format!("Failed to create grade scale item: {e}")))?;
    }

    // 3b. Pass/Fail Policy
    let pf_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO pass_fail_policies (id, org_id, period_id, name, min_passing_score, requires_all_subjects, max_failed_subjects, supplementary_exam_eligible, created_at, updated_at) VALUES ($1, $2, NULL, $3, $4, $5, $6, $7, $8, $9)",
        vec![
            pf_id.clone().into(), org_id.clone().into(),
            format!("Default Pass/Fail - {}", blueprint.scale_name).into(),
            blueprint.passing_grade.into(),
            (if blueprint.requires_all_subjects { 1i32 } else { 0 }).into(),
            blueprint.max_failed_subjects.into(),
            (if blueprint.supplementary_eligible { 1i32 } else { 0 }).into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create pass/fail policy: {e}")))?;

    // 3c. Academic Policies (SSOT reference hub + Full Autopilot Baseline)
    let policy_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_policies (
            id, org_id, default_scale_id, default_pass_fail_policy_id,
            default_grading_system, default_passing_score, default_max_score,
            enrollment_auto_close, allow_late_enrollment, late_enrollment_approval,
            grade_entry_auto_close, allow_grade_edit_after_close, grade_edit_approval,
            min_attendance_rate, auto_graduate_final_level, exclude_withdrawn,
            autopilot_mode, autopilot_enabled, max_auto_retries, pause_on_error,
            auto_open_enrollment, auto_close_enrollment, auto_open_grade_entry, auto_close_grade_entry,
            auto_lock_periods, auto_advance_periods, auto_archive_years,
            grace_days_after_deadline, notify_before_deadline_days,
            created_at, updated_at
        ) VALUES (
            $1, $2, $3, $4,
            $5, $6, $7,
            1, 1, 0,
            1, 0, 1,
            75.0, 0, 1,
            'automatic', 1, 3, 0,
            1, 1, 1, 1,
            1, 0, 0,
            7, 3,
            $8, $9
        )",
        vec![
            policy_id.clone().into(), org_id.clone().into(),
            scale_id.clone().into(),
            pf_id.into(),
            blueprint.grading_system.clone().into(),
            blueprint.passing_grade.into(),
            100.0f64.into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create academic policies: {e}")))?;

    // 3d. Default Calculation Policy — period-level (from scheme components when present)
    let calc_policy_id = Uuid::new_v4().to_string();
    let formula = if blueprint.assessment_components.is_empty() {
        serde_json::json!({
            "type": "weighted_sum",
            "items": [
                { "node": { "type": "variable", "name": "midterm" }, "weight": 0.3 },
                { "node": { "type": "variable", "name": "final" }, "weight": 0.5 },
                { "node": { "type": "variable", "name": "classwork" }, "weight": 0.2 }
            ]
        })
    } else {
        let weight_total: f64 = blueprint.assessment_components.iter().map(|c| c.weight).sum();
        let denom = if weight_total > 0.0 { weight_total } else { 1.0 };
        let items: Vec<serde_json::Value> = blueprint.assessment_components.iter().map(|c| {
            serde_json::json!({
                "node": { "type": "variable", "name": c.component_type },
                "weight": c.weight / denom
            })
        }).collect();
        serde_json::json!({ "type": "weighted_sum", "items": items })
    };
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO calculation_policies (id, org_id, target_period_id, name, version, rule_type, formula_expression, rounding_policy, is_archived, created_at, updated_at) VALUES ($1, $2, NULL, $3, 1, 'weighted_sum', $4, 'round_half_up', 0, $5, $6)",
        vec![
            calc_policy_id.clone().into(), org_id.clone().into(),
            format!("Default {} Policy", blueprint.scale_name).into(),
            formula.to_string().into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create calculation policy: {e}")))?;

    // 3d-ii. Term Calculation Policy (coursework + exam → term total)
    let term_calc_id = Uuid::new_v4().to_string();
    let term_formula = serde_json::json!({
        "type": "add",
        "operands": [
            {
                "type": "multiply",
                "operands": [
                    { "type": "variable", "name": "coursework_avg" },
                    { "type": "literal", "value": blueprint.coursework_weight / 100.0 }
                ]
            },
            { "type": "variable", "name": "exam" }
        ]
    });
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO calculation_policies (id, org_id, target_period_id, name, version, rule_type, formula_expression, rounding_policy, is_archived, created_at, updated_at) VALUES ($1, $2, NULL, $3, 1, 'term', $4, 'round_half_up', 0, $5, $6)",
        vec![
            term_calc_id.into(), org_id.clone().into(),
            format!("Default Term Policy - {}", blueprint.scale_name).into(),
            term_formula.to_string().into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create term calculation policy: {e}")))?;

    // 3d-iii. Annual Calculation Policy (sum of terms)
    let annual_calc_id = Uuid::new_v4().to_string();
    let annual_formula = serde_json::json!({
        "type": "variable",
        "name": "annual_total"
    });
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO calculation_policies (id, org_id, target_period_id, name, version, rule_type, formula_expression, rounding_policy, is_archived, created_at, updated_at) VALUES ($1, $2, NULL, $3, 1, 'annual', $4, 'round_half_up', 0, $5, $6)",
        vec![
            annual_calc_id.into(), org_id.clone().into(),
            format!("Default Annual Policy - {}", blueprint.scale_name).into(),
            annual_formula.to_string().into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create annual calculation policy: {e}")))?;

    // 3d-iv. Default Assessment Scheme + Components
    if !blueprint.assessment_components.is_empty() {
        let scheme_id = Uuid::new_v4().to_string();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO assessment_schemes (id, org_id, period_id, name, is_archived, created_at, updated_at) VALUES ($1, $2, NULL, $3, 0, $4, $5)",
            vec![
                scheme_id.clone().into(), org_id.clone().into(),
                blueprint.assessment_scheme_name.clone().into(),
                now.clone().into(), now.clone().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to create assessment scheme: {e}")))?;

        for comp in &blueprint.assessment_components {
            let comp_id = Uuid::new_v4().to_string();
            db.execute(Statement::from_sql_and_values(
                db.get_database_backend(),
                "INSERT INTO assessment_scheme_components (id, scheme_id, name, component_type, weight, max_score, passing_score, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                vec![
                    comp_id.into(), scheme_id.clone().into(),
                    comp.name.clone().into(), comp.component_type.clone().into(),
                    comp.weight.into(), comp.max_score.into(),
                    comp.passing_score.into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to create scheme component: {e}")))?;
        }
    }

    // 3d-v. Default Ranking Policy
    let ranking_id = Uuid::new_v4().to_string();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO ranking_policies (id, org_id, period_id, name, is_enabled, ranking_scope, ranking_method, tie_handling, include_failed_students, ranking_metric, created_at, updated_at) VALUES ($1, $2, NULL, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        vec![
            ranking_id.into(), org_id.clone().into(),
            format!("Default Ranking - {}", blueprint.scale_name).into(),
            blueprint.ranking_enabled.into(),
            blueprint.ranking_scope.clone().into(),
            blueprint.ranking_method.clone().into(),
            "same_rank".into(),
            (if blueprint.include_failed_in_ranking { 1i32 } else { 0 }).into(),
            "total_score".into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create ranking policy: {e}")))?;

    // 3e. Period Templates (Seeding default blueprint and all standard structures)
    let standard_templates = vec![
        (
            Uuid::new_v4().to_string(),
            "Two Semesters (Fall & Spring)".to_string(),
            "نظام الفصلين الدراسيين (الترم الأول والثاني)".to_string(),
            serde_json::json!([
                { "name": "First Semester", "name_ar": "الفصل الدراسي الأول", "type": "semester", "weight": 50 },
                { "name": "Second Semester", "name_ar": "الفصل الدراسي الثاني", "type": "semester", "weight": 50 }
            ]),
            1i32,
        ),
        (
            Uuid::new_v4().to_string(),
            "Three Terms / Trimesters".to_string(),
            "نظام الفصول الثلاثية (3 فصول دراسية)".to_string(),
            serde_json::json!([
                { "name": "First Term", "name_ar": "الفصل الدراسي الأول", "type": "term", "weight": 33.33 },
                { "name": "Second Term", "name_ar": "الفصل الدراسي الثاني", "type": "term", "weight": 33.33 },
                { "name": "Third Term", "name_ar": "الفصل الدراسي الثالث", "type": "term", "weight": 33.34 }
            ]),
            0i32,
        ),
        (
            Uuid::new_v4().to_string(),
            "Four Quarters".to_string(),
            "نظام الأرباع السنوية (4 أرباع)".to_string(),
            serde_json::json!([
                { "name": "Quarter 1", "name_ar": "الربع الأول", "type": "quarter", "weight": 25 },
                { "name": "Quarter 2", "name_ar": "الربع الثاني", "type": "quarter", "weight": 25 },
                { "name": "Quarter 3", "name_ar": "الربع الثالث", "type": "quarter", "weight": 25 },
                { "name": "Quarter 4", "name_ar": "الربع الرابع", "type": "quarter", "weight": 25 }
            ]),
            0i32,
        ),
        (
            Uuid::new_v4().to_string(),
            "Two Semesters with Monthly Assessment".to_string(),
            "فصلين دراسيين مع تقييمات شهرية".to_string(),
            blueprint.period_structure.clone(),
            0i32,
        ),
        (
            Uuid::new_v4().to_string(),
            "Full Academic Year (Single Period)".to_string(),
            "عام دراسي سنوي متصل (بدون تقسيم فصول)".to_string(),
            serde_json::json!([
                { "name": "Full Year", "name_ar": "العام الدراسي الكامل", "type": "year", "weight": 100 }
            ]),
            0i32,
        ),
    ];

    let mut primary_tmpl_id = String::new();
    for (idx, (t_id, t_name, t_name_ar, t_struct, is_def)) in standard_templates.into_iter().enumerate() {
        if idx == 0 {
            primary_tmpl_id = t_id.clone();
        }
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            vec![
                t_id.into(), org_id.clone().into(),
                t_name.into(),
                t_name_ar.into(),
                payload.org_type.clone().into(),
                serde_json::to_string(&t_struct).unwrap().into(),
                is_def.into(),
                now.clone().into(), now.clone().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to create period template: {e}")))?;
    }
    let tmpl_id = primary_tmpl_id;

    // Link default template to academic policies
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "UPDATE academic_policies SET default_template_id = $1, updated_at = $2 WHERE org_id = $3",
        vec![
            tmpl_id.clone().into(),
            now.clone().into(),
            org_id.clone().into(),
        ]
    )).await.ok();

    // 3f. Default Role Permissions
    for perm in &blueprint.default_permissions {
        let actions_json = serde_json::to_string(&perm.actions).unwrap_or_else(|_| "[]".to_string());
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO role_permissions (role, document_type, level, only_if_creator, actions, created_at) VALUES ($1, $2, 0, 0, $3, $4) ON CONFLICT (role, document_type) DO UPDATE SET level = EXCLUDED.level, only_if_creator = EXCLUDED.only_if_creator, actions = EXCLUDED.actions, created_at = EXCLUDED.created_at",
            vec![
                perm.role.clone().into(),
                perm.document_type.clone().into(),
                actions_json.into(),
                now.clone().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to insert permission: {e}")))?;
    }

    // 3g. Currencies
    let default_currencies = vec![
        "SAR", "YER", "USD", "AED", "KWD", "QAR", "BHD", "OMR", "EGP", "JOD", "IQD", "LYD", "DZD", "MAD", "TND", "SDG", "TRY", "EUR", "GBP"
    ];
    let mut currencies_to_insert: Vec<String> = default_currencies.into_iter().map(|s| s.to_string()).collect();
    
    if !currencies_to_insert.contains(&payload.currency) {
        currencies_to_insert.push(payload.currency.clone());
    }

    for curr_code in currencies_to_insert {
        let is_base = if curr_code == payload.currency { 1i32 } else { 0i32 };
        let exchange_rate = if is_base == 1 { 1.0f64 } else { 0.0f64 };

        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO currencies (code, org_id, name, name_ar, symbol, symbol_position, decimal_places, exchange_rate, is_base, is_active, updated_at) VALUES ($1, $2, $3, $4, $5, 'after', $6, $7, $8, 1, $9)",
            vec![
                curr_code.clone().into(),
                org_id.clone().into(),
                currency_name(&curr_code).into(),
                currency_name_ar(&curr_code).into(),
                currency_symbol(&curr_code).into(),
                2i32.into(),
                exchange_rate.into(),
                is_base.into(),
                now.clone().into(),
            ]
        )).await.ok(); 
    }

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
            "default_passing_score": blueprint.passing_grade,
            "default_max_score": 100.0
        }
    });

    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "UPDATE organizations SET settings = $1 WHERE id = $2",
        vec![org_settings.to_string().into(), org_id.clone().into()]
    )).await.ok();

    // 3j. Default Report Templates
    let report_templates = vec![
        (
            "44444444-4444-4444-4444-444444444444",
            "Master Layout",
            "layout",
            include_str!("../../../../../../apps/cli/templates/reports/master_layout.html")
        ),
        (
            "11111111-1111-1111-1111-111111111111",
            "Payment Receipt",
            "payment_receipt",
            include_str!("../../../../../../apps/cli/templates/reports/payment_receipt.html")
        ),
        (
            "22222222-2222-2222-2222-222222222222",
            "Bulk Payments Receipt",
            "payments_bulk",
            include_str!("../../../../../../apps/cli/templates/reports/payments_bulk.html")
        ),
        (
            "33333333-3333-3333-3333-333333333333",
            "Class Subject Gradebook",
            "class_subject_gradebook",
            include_str!("../../../../../../apps/cli/templates/reports/class_subject_gradebook.html")
        ),
    ];

    for (t_id, name, t_type, t_body) in report_templates {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_templates (id, org_id, name, template_type, template_body, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 1, $6, $7)",
            vec![
                t_id.into(),
                org_id.clone().into(),
                name.into(),
                t_type.into(),
                t_body.into(),
                now.clone().into(),
                now.clone().into(),
            ]
        )).await.ok();
    }


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

async fn seed_chart_of_accounts<C: ConnectionTrait>(
    db: &C,
    org_id: &str,
    currency: &str,
    now: &str,
) -> Result<(), DomainError> {
    let coa: CoaData = serde_json::from_str(COA_JSON)
        .map_err(|e| DomainError::Internal(format!("Failed to parse COA JSON: {e}")))?;

    let mut rows = Vec::new();
    flatten_acc_nodes(&coa.accounts, org_id, None, "", 1, currency, now, &mut rows);

    for (id, oid, code, name, name_ar, db_type, parent_id, path, depth, curr, timestamp) in rows {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO accounts (id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, balances, is_control_account, is_system_account, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, '{}', CASE WHEN $3 = '1101' THEN 1 ELSE 0 END, 0, 1, $12, $13)",
            vec![
                id.into(), oid.into(), code.clone().into(), name.into(), name_ar.into(),
                db_type.into(),
                match code.as_str() {
                    "110101" => Some("cash"),
                    "110102" | "110103" => Some("bank"),
                    _ => None,
                }.into(),
                parent_id.into(), path.into(), depth.into(), curr.into(),
                timestamp.clone().into(), timestamp.into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to seed account {code}: {e}")))?;
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
    document_type: String,
    actions: Vec<String>,
}

struct AssessmentComponent {
    name: String,
    component_type: String,
    weight: f64,
    max_score: f64,
    passing_score: f64,
}

struct InstitutionBlueprint {
    grading_system: String,
    passing_grade: f64,
    scale_name: String,
    grade_items: Vec<GradeItem>,
    requires_all_subjects: bool,
    max_failed_subjects: i32,
    supplementary_eligible: bool,
    #[allow(dead_code)]
    period_template_name: String,
    #[allow(dead_code)]
    period_template_name_ar: String,
    period_structure: serde_json::Value,
    default_permissions: Vec<RolePermission>,
    // ── Grading Architecture Presets ─────────────────────────
    assessment_scheme_name: String,
    assessment_components: Vec<AssessmentComponent>,
    coursework_weight: f64,  // e.g. 20.0 (out of 100)
    #[allow(dead_code)]
    exam_weight: f64,        // e.g. 30.0
    ranking_enabled: i32,
    ranking_scope: String,
    ranking_method: String,
    include_failed_in_ranking: bool,
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
            GradeItem { label: "مقبول".into(),    min_score: 50.0, max_score: 69.99, gpa_points: 2.5, is_passing: true },
            GradeItem { label: "راسب".into(),    min_score: 0.0,  max_score: 49.99, gpa_points: 0.0, is_passing: false },
        ],
        requires_all_subjects: false,
        max_failed_subjects: 2,
        supplementary_eligible: true,
        period_template_name: "العام الدراسي".into(),
        period_template_name_ar: "العام الدراسي".into(),
        period_structure: serde_json::json!([
            {
                "name": "الفصل الأول", "name_ar": "الفصل الأول", "type": "term", "weight": 50,
                "children": [
                    { "name": "الشهر الأول", "name_ar": "الشهر الأول", "type": "monthly", "weight": 0 },
                    { "name": "الشهر الثاني", "name_ar": "الشهر الثاني", "type": "monthly", "weight": 0 },
                    { "name": "الشهر الثالث", "name_ar": "الشهر الثالث", "type": "monthly", "weight": 0 },
                    { "name": "اختبار الفصل الأول", "name_ar": "اختبار الفصل الأول", "type": "exam", "weight": 0 }
                ]
            },
            {
                "name": "الفصل الثاني", "name_ar": "الفصل الثاني", "type": "term", "weight": 50,
                "children": [
                    { "name": "الشهر الرابع", "name_ar": "الشهر الرابع", "type": "monthly", "weight": 0 },
                    { "name": "الشهر الخامس", "name_ar": "الشهر الخامس", "type": "monthly", "weight": 0 },
                    { "name": "الشهر السادس", "name_ar": "الشهر السادس", "type": "monthly", "weight": 0 },
                    { "name": "اختبار الفصل الثاني", "name_ar": "اختبار الفصل الثاني", "type": "exam", "weight": 0 }
                ]
            }
        ]),
        default_permissions: default_role_permissions(),
        // ── Grading Architecture ────────────────────────────
        assessment_scheme_name: "المخطط التقييمي الشهري".into(),
        assessment_components: vec![
            AssessmentComponent { name: "الواجبات".into(), component_type: "assignment".into(), weight: 20.0, max_score: 20.0, passing_score: 10.0 },
            AssessmentComponent { name: "الشفهي".into(), component_type: "oral".into(), weight: 20.0, max_score: 20.0, passing_score: 10.0 },
            AssessmentComponent { name: "الحضور والمشاركة".into(), component_type: "attendance".into(), weight: 20.0, max_score: 20.0, passing_score: 10.0 },
            AssessmentComponent { name: "الاختبار التحريري".into(), component_type: "written_exam".into(), weight: 40.0, max_score: 40.0, passing_score: 20.0 },
        ],
        coursework_weight: 20.0,  // 20 out of 50 term points
        exam_weight: 30.0,        // 30 out of 50 term points
        ranking_enabled: 1,
        ranking_scope: "class".into(),
        ranking_method: "standard".into(),
        include_failed_in_ranking: false,
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
        assessment_scheme_name: "University Assessment".into(),
        assessment_components: vec![],  // University uses credit-hour GPA, no fixed components
        coursework_weight: 40.0,
        exam_weight: 60.0,
        ranking_enabled: 0,  // Universities typically don't rank
        ranking_scope: "program".into(),
        ranking_method: "standard".into(),
        include_failed_in_ranking: false,
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
        assessment_scheme_name: "Training Assessment".into(),
        assessment_components: vec![],
        coursework_weight: 50.0,
        exam_weight: 50.0,
        ranking_enabled: 0,
        ranking_scope: "class".into(),
        ranking_method: "standard".into(),
        include_failed_in_ranking: false,
    }
}

// ── Shared Defaults ─────────────────────────────────────────────

fn default_role_permissions() -> Vec<RolePermission> {
    vec![
        // Teacher
        RolePermission { role: "teacher".into(), document_type: "Grade".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "teacher".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "teacher".into(), document_type: "Assessment".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "teacher".into(), document_type: "Class".into(), actions: vec!["read".into()] },
        RolePermission { role: "teacher".into(), document_type: "Subject".into(), actions: vec!["read".into()] },
        RolePermission { role: "teacher".into(), document_type: "Schedule".into(), actions: vec!["read".into()] },
        RolePermission { role: "teacher".into(), document_type: "Exam".into(), actions: vec!["read_schedule".into(), "read_hall".into(), "read_proctor".into()] },
        RolePermission { role: "teacher".into(), document_type: "Document".into(), actions: vec!["read".into()] },

        // Student
        RolePermission { role: "student".into(), document_type: "Grade".into(), actions: vec!["read".into()] },
        RolePermission { role: "student".into(), document_type: "Attendance".into(), actions: vec!["read".into()] },
        RolePermission { role: "student".into(), document_type: "Schedule".into(), actions: vec!["read".into()] },
        RolePermission { role: "student".into(), document_type: "Enrollment".into(), actions: vec!["read".into()] },
        RolePermission { role: "student".into(), document_type: "Exam".into(), actions: vec!["read_schedule".into()] },

        // Parent
        RolePermission { role: "parent".into(), document_type: "Grade".into(), actions: vec!["read".into()] },
        RolePermission { role: "parent".into(), document_type: "Attendance".into(), actions: vec!["read".into()] },
        RolePermission { role: "parent".into(), document_type: "Schedule".into(), actions: vec!["read".into()] },
        RolePermission { role: "parent".into(), document_type: "Enrollment".into(), actions: vec!["read".into()] },
        RolePermission { role: "parent".into(), document_type: "Document".into(), actions: vec!["read".into()] },

        // Principal
        RolePermission { role: "principal".into(), document_type: "Organization".into(), actions: vec!["read".into()] },
        RolePermission { role: "principal".into(), document_type: "User".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Period".into(), actions: vec!["read".into(), "write".into(), "activate".into(), "lock".into()] },
        RolePermission { role: "principal".into(), document_type: "Class".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Subject".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Schedule".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Grade".into(), actions: vec!["read".into(), "write".into(), "analytics".into(), "report".into()] },
        RolePermission { role: "principal".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "write".into(), "analytics".into()] },
        RolePermission { role: "principal".into(), document_type: "Assessment".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Enrollment".into(), actions: vec!["read".into(), "write".into(), "promote".into(), "transfer".into()] },
        RolePermission { role: "principal".into(), document_type: "Exam".into(), actions: vec!["read_schedule".into(), "write_schedule".into(), "read_hall".into(), "write_hall".into(), "allocate_hall".into(), "read_proctor".into(), "assign_proctor".into(), "read_order".into(), "rebuild_order".into(), "freeze_order".into()] },
        RolePermission { role: "principal".into(), document_type: "Document".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Template".into(), actions: vec!["read".into(), "write".into(), "generate_single".into(), "generate_batch".into()] },
        RolePermission { role: "principal".into(), document_type: "Promotion".into(), actions: vec!["read".into(), "write".into(), "execute".into()] },

        // Staff
        RolePermission { role: "staff".into(), document_type: "User".into(), actions: vec!["read".into()] },
        RolePermission { role: "staff".into(), document_type: "Enrollment".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "staff".into(), document_type: "Class".into(), actions: vec!["read".into()] },
        RolePermission { role: "staff".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "staff".into(), document_type: "Document".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "staff".into(), document_type: "Template".into(), actions: vec!["read".into(), "generate_single".into()] },
    ]
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
        "YER" => "Yemeni Rial".into(),
        "AED" => "UAE Dirham".into(),
        "KWD" => "Kuwaiti Dinar".into(),
        "QAR" => "Qatari Rial".into(),
        "BHD" => "Bahraini Dinar".into(),
        "OMR" => "Omani Rial".into(),
        "EGP" => "Egyptian Pound".into(),
        "JOD" => "Jordanian Dinar".into(),
        "IQD" => "Iraqi Dinar".into(),
        "LYD" => "Libyan Dinar".into(),
        "DZD" => "Algerian Dinar".into(),
        "MAD" => "Moroccan Dirham".into(),
        "TND" => "Tunisian Dinar".into(),
        "SDG" => "Sudanese Pound".into(),
        "TRY" => "Turkish Lira".into(),
        "USD" => "US Dollar".into(),
        "EUR" => "Euro".into(),
        "GBP" => "British Pound".into(),
        _ => code.to_string(),
    }
}

fn currency_name_ar(code: &str) -> String {
    match code {
        "SAR" => "ريال سعودي".into(),
        "YER" => "ريال يمني".into(),
        "AED" => "درهم إماراتي".into(),
        "KWD" => "دينار كويتي".into(),
        "QAR" => "ريال قطري".into(),
        "BHD" => "دينار بحريني".into(),
        "OMR" => "ريال عماني".into(),
        "EGP" => "جنيه مصري".into(),
        "JOD" => "دينار أردني".into(),
        "IQD" => "دينار عراقي".into(),
        "LYD" => "دينار ليبي".into(),
        "DZD" => "دينار جزائري".into(),
        "MAD" => "درهم مغربي".into(),
        "TND" => "دينار تونسي".into(),
        "SDG" => "جنيه سوداني".into(),
        "TRY" => "ليرة تركية".into(),
        "USD" => "دولار أمريكي".into(),
        "EUR" => "يورو".into(),
        "GBP" => "جنيه إسترليني".into(),
        _ => code.to_string(),
    }
}

fn currency_symbol(code: &str) -> String {
    match code {
        "SAR" => "ر.س".into(),
        "YER" => "ر.ي".into(),
        "AED" => "د.إ".into(),
        "KWD" => "د.ك".into(),
        "QAR" => "ر.ق".into(),
        "BHD" => "د.ب".into(),
        "OMR" => "ر.ع".into(),
        "EGP" => "ج.م".into(),
        "JOD" => "د.أ".into(),
        "IQD" => "د.ع".into(),
        "LYD" => "د.ل".into(),
        "DZD" => "د.ج".into(),
        "MAD" => "د.م".into(),
        "TND" => "د.ت".into(),
        "SDG" => "ج.س".into(),
        "TRY" => "₺".into(),
        "USD" => "$".into(),
        "EUR" => "€".into(),
        "GBP" => "£".into(),
        _ => code.to_string(),
    }
}
