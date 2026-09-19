//! Organization Blueprint Composition — unified initialization engine.
//!
//! This is the **single source of truth** for all organization setup data.
//! It replaces the scattered initialization across:
//! - `infra-persistence/bootstrap.rs` (academic_policies + period_templates)
//! - `cli/seeder.rs` (currencies + permissions + period_templates + COA)
//! - `desktop/completeSetup()` (INSTITUTION_PROFILES)
//!
//! Every blueprint produces the complete seed data for one institution type:
//! grade_scales, calculation_policies, period_templates, default_permissions,
//! currencies, and chart_of_accounts.

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
    let org_id_uuid = Uuid::new_v4();
    let admin_id_uuid = Uuid::new_v4();
    let org_id = org_id_uuid.to_string();
    let admin_id = admin_id_uuid.to_string();

    info!("Bootstrapping full organization '{}' (type={})", payload.org_name, payload.org_type);

    // ── 1. Organization ────────────────────────────────────────
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO organizations (id, name, name_ar, slug, timezone, locale, base_currency_code, is_hijri_enabled, is_active, is_verified, is_deleted, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 1, 1, 1, 0, $8, $9)",
        vec![
            org_id_uuid.into(),
            payload.org_name.clone().into(),
            payload.org_name_ar.clone().unwrap_or_default().into(),
            slug(&payload.org_name).into(),
            payload.timezone.clone().unwrap_or_else(|| "Asia/Riyadh".into()).into(),
            payload.locale.clone().unwrap_or_else(|| "ar".into()).into(),
            payload.currency.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create organization: {e}")))?;

    // ── 1.5 Admin Party & Person ───────────────────────────────
    let person_id_uuid = Uuid::new_v4();
    let name_parts: Vec<&str> = payload.admin_name.split_whitespace().collect();
    let first_name = name_parts.first().unwrap_or(&"").to_string();
    let last_name = if name_parts.len() > 1 { name_parts[1..].join(" ") } else { first_name.clone() };

    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO persons (id, org_id, first_name, last_name, display_name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        vec![
            person_id_uuid.into(),
            org_id_uuid.into(),
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
        "INSERT INTO users (id, person_id, org_id, email, password_hash, is_email_verified, is_phone_verified, is_mfa_enabled, status, login_count, failed_login_attempts, is_deleted, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 1, 0, 0, 'active', 0, 0, 0, $6, $7)",
        vec![
            admin_id_uuid.into(),
            person_id_uuid.into(),
            org_id_uuid.into(),
            payload.admin_username.clone().into(),
            payload.admin_password_hash.clone().into(),
            now.clone().into(),
            now.clone().into(),
        ]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to create admin user: {e}")))?;

    // ── 2.5 Blueprint Selection ─────────────────────────────────
    let blueprint = match payload.org_type.as_str() {
        "university" => university_blueprint(),
        "institute" | "training_center" => institute_blueprint(),
        _ => school_blueprint(),
    };

    // ── 3. OrgUnits Hierarchy ──────────────────────────────────
    if let Err(e) = seed_org_units(db, org_id_uuid, &payload.org_type, &now).await {
        info!("Note: OrgUnits seeding warning: {}", e);
    }

    // ── 4. Rooms & Educational Facilities ──────────────────────
    if let Err(e) = seed_rooms(db, org_id_uuid, &now).await {
        info!("Note: Rooms seeding warning: {}", e);
    }

    // ── 5. Period Templates ────────────────────────────────────
    let tmpl_id = seed_period_templates(db, org_id_uuid, &payload.org_type, &blueprint, &now).await?;

    // ── 6. Default Assessment Scheme ───────────────────────────
    if let Err(e) = seed_assessment_schemes(db, org_id_uuid, &blueprint, &now).await {
        info!("Note: Assessment scheme seeding warning: {}", e);
    }

    // ── 7. Default Academic Year & Period ──────────────────────
    let year_id_uuid = Uuid::new_v4();
    let next_year_date = "2027-06-30T23:59:59Z";
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_years (id, org_id, name, name_ar, start_date, end_date, status, is_locked, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, 'active', 0, $7, $8)",
        vec![
            year_id_uuid.into(), org_id_uuid.into(),
            "Academic Year 2026/2027".into(), "العام الدراسي 2026/2027".into(),
            now.clone().into(), next_year_date.into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.ok();

    let period_id_uuid = Uuid::new_v4();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO academic_periods (id, org_id, year_id, name, name_ar, period_type, start_date, end_date, status, is_locked, is_enrollment_open, is_grade_entry_open, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'semester', $6, $7, 'active', 0, 1, 1, $8, $9)",
        vec![
            period_id_uuid.into(), org_id_uuid.into(), year_id_uuid.into(),
            "First Semester".into(), "الفصل الدراسي الأول".into(),
            now.clone().into(), next_year_date.into(),
            now.clone().into(), now.clone().into(),
        ]
    )).await.ok();

    // ── 8. Chart of Accounts (COA) ─────────────────────────────
    if let Err(e) = seed_chart_of_accounts(db, org_id_uuid, &payload.currency, &now).await {
        info!("Note: COA seeding warning: {}", e);
    }

    // ── 9. Fee Templates ───────────────────────────────────────
    if let Err(e) = seed_fee_templates(db, org_id_uuid, &payload.currency, &now).await {
        info!("Note: Fee templates seeding warning: {}", e);
    }

    // ── 10. Discount Templates ─────────────────────────────────
    if let Err(e) = seed_discount_templates(db, org_id_uuid, &now).await {
        info!("Note: Discount templates seeding warning: {}", e);
    }

    // ── 11. Currencies ─────────────────────────────────────────
    if let Err(e) = seed_currencies(db, &now).await {
        info!("Note: Currencies seeding warning: {}", e);
    }

    // ── 12. Roles & Permissions Matrix ─────────────────────────
    if let Err(e) = seed_roles_and_permissions(db, org_id_uuid, admin_id_uuid, &now).await {
        info!("Note: Roles seeding warning: {}", e);
    }

    // ── 13. Default Reports & Document Templates ───────────────
    if let Err(e) = seed_default_reports(db, org_id_uuid, &now).await {
        info!("Note: Report seeding warning: {}", e);
    }

    // ── 14. Organization Settings ──────────────────────────────
    seed_org_settings(db, org_id_uuid, &payload.currency, &blueprint).await.ok();

    info!("Bootstrap completed for org_id={} type={}", org_id, payload.org_type);

    Ok(BootstrapResult {
        org_id,
        admin_id,
        period_template_id: tmpl_id,
    })
}

// ── Subsystem Seeders ───────────────────────────────────────────

async fn seed_org_units<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    org_type: &str,
    now: &str,
) -> Result<Uuid, DomainError> {
    let root_id = Uuid::new_v4();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO org_units (id, org_id, parent_id, path, depth, name, name_ar, code, unit_type, data_isolation, children_count, descendant_count, sort_order, is_active, created_at, updated_at) VALUES ($1, $2, NULL, 'root', 0, 'Main Campus', 'المقر الرئيسي', 'HQ', 'campus', 'none', 0, 0, 1, 1, $3, $4)",
        vec![root_id.into(), org_id.into(), now.to_string().into(), now.to_string().into()]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to seed root org_unit: {e}")))?;

    let sub_units = match org_type {
        "university" => vec![
            ("Faculty of Engineering & Computing", "كلية الهندسة والحاسوب", "ENG", "faculty", 1),
            ("Faculty of Business & Administrative Sciences", "كلية العلوم الإدارية والمالية", "BUS", "faculty", 2),
            ("Faculty of Humanities & Languages", "كلية العلوم الإنسانية واللغات", "HUM", "faculty", 3),
            ("Deanship of Admissions & Registration", "عمادة القبول والتسجيل", "ADM", "deanship", 4),
            ("Deanship of Student Affairs", "عمادة شؤون الطلاب", "STU", "deanship", 5),
            ("Administrative & Financial Affairs", "الشؤون الإدارية والمالية", "FIN", "department", 6),
        ],
        "institute" | "training_center" => vec![
            ("Information Technology & Software Dept", "قسم تقنية المعلومات والبرمجيات", "IT", "department", 1),
            ("Languages & Communication Skills Dept", "قسم اللغات والمهارات العامة", "LANG", "department", 2),
            ("Professional Training & Certifications", "قسم التدريب المهني والشهادات الاحترافية", "TRAIN", "department", 3),
            ("Trainee Affairs & Admissions", "إدارة شؤون المتدربين والقبول", "REG", "department", 4),
            ("Administrative & Financial Services", "الشؤون الإدارية والمالية", "ADM", "department", 5),
        ],
        _ => vec![
            ("Kindergarten Stage", "مرحلة رياض الأطفال", "KG", "division", 1),
            ("Primary Stage", "المرحلة الابتدائية", "PRI", "division", 2),
            ("Middle School Stage", "المرحلة المتوسطة", "MID", "division", 3),
            ("High School Stage", "المرحلة الثانوية", "SEC", "division", 4),
            ("Administrative & Financial Affairs", "الشؤون الإدارية والمالية", "ADM", "department", 5),
            ("Student Affairs & Counseling", "شؤون الطلاب والإرشاد الطلابي", "STU", "department", 6),
        ],
    };

    for (name, name_ar, code, unit_type, sort_order) in sub_units {
        let uid = Uuid::new_v4();
        let path = format!("root.{}", uid);
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO org_units (id, org_id, parent_id, path, depth, name, name_ar, code, unit_type, data_isolation, children_count, descendant_count, sort_order, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, 1, $5, $6, $7, $8, 'none', 0, 0, $9, 1, $10, $11)",
            vec![
                uid.into(), org_id.into(), root_id.into(), path.into(),
                name.into(), name_ar.into(), code.into(), unit_type.into(),
                sort_order.into(), now.to_string().into(), now.to_string().into()
            ]
        )).await.ok();
    }

    Ok(root_id)
}

async fn seed_rooms<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    let rooms = vec![
        ("Main Auditorium", 250, "auditorium"),
        ("Computer & AI Lab", 35, "lab"),
        ("Science & Innovation Lab", 30, "lab"),
        ("Central Library & Learning Hub", 60, "library"),
        ("Smart Lecture Hall A1", 50, "lecture_hall"),
        ("Classroom 101", 30, "classroom"),
        ("Classroom 102", 30, "classroom"),
        ("Classroom 103", 30, "classroom"),
    ];

    for (name, cap, r_type) in rooms {
        let rid = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO rooms (id, org_id, name, capacity, room_type, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            vec![
                rid.into(), org_id.into(), name.into(),
                cap.into(), r_type.into(), now.to_string().into(), now.to_string().into()
            ]
        )).await.ok();
    }
    Ok(())
}

async fn seed_period_templates<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    org_type: &str,
    blueprint: &InstitutionBlueprint,
    now: &str,
) -> Result<String, DomainError> {
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
        (
            Uuid::new_v4().to_string(),
            "Modular & Intensive Training Cycles".to_string(),
            "دورات ووحدات تدريبية مكثفة".to_string(),
            serde_json::json!([
                { "name": "Module 1", "name_ar": "الوحدة التدريبية الأولى", "type": "module", "weight": 50 },
                { "name": "Module 2", "name_ar": "الوحدة التدريبية الثانية", "type": "module", "weight": 50 }
            ]),
            0i32,
        ),
    ];

    let mut primary_tmpl_id = String::new();
    for (idx, (t_id_str, t_name, t_name_ar, t_struct, is_def)) in standard_templates.into_iter().enumerate() {
        let t_id_uuid = Uuid::parse_str(&t_id_str).unwrap_or_else(|_| Uuid::new_v4());
        if idx == 0 {
            primary_tmpl_id = t_id_uuid.to_string();
        }
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            vec![
                t_id_uuid.into(), org_id.into(),
                t_name.into(),
                t_name_ar.into(),
                org_type.into(),
                serde_json::to_string(&t_struct).unwrap().into(),
                is_def.into(),
                now.to_string().into(), now.to_string().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to create period template: {e}")))?;
    }

    Ok(primary_tmpl_id)
}

async fn seed_assessment_schemes<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    blueprint: &InstitutionBlueprint,
    now: &str,
) -> Result<(), DomainError> {
    let scheme_id_uuid = Uuid::new_v4();
    let formula_json = serde_json::json!({
        "type": "weighted_sum",
        "items": [
            { "node": { "type": "variable", "name": "coursework" }, "weight": blueprint.coursework_weight / 100.0 },
            { "node": { "type": "variable", "name": "final_exam" }, "weight": blueprint.exam_weight / 100.0 }
        ]
    });
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO assessment_schemes (id, org_id, name, description, max_score, formula, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        vec![
            scheme_id_uuid.into(), org_id.into(),
            blueprint.assessment_scheme_name.clone().into(),
            "Standard Assessment Scheme".into(),
            100.0f64.into(),
            formula_json.to_string().into(),
            now.to_string().into(), now.to_string().into(),
        ]
    )).await.ok();
    Ok(())
}

async fn seed_fee_templates<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    currency: &str,
    now: &str,
) -> Result<(), DomainError> {
    let rec_row = db.query_one(Statement::from_sql_and_values(
        db.get_database_backend(),
        "SELECT id FROM accounts WHERE org_id = $1 AND code = '110401' LIMIT 1",
        vec![org_id.into()]
    )).await.ok().flatten();

    let rec_id: Uuid = match rec_row {
        Some(r) => r.try_get("", "id").unwrap_or_else(|_| Uuid::new_v4()),
        None => Uuid::new_v4(),
    };

    let fee_defs = vec![
        ("FEE_TUITION", "Tuition Fee", "الرسوم الدراسية الأساسية", "tuition", "410101", 5000.0, true, "semester"),
        ("FEE_REG", "Registration & Admission Fee", "رسوم القبول والتسجيل", "registration", "410201", 500.0, true, "one_time"),
        ("FEE_BUS", "School Bus Transportation Fee", "رسوم النقل والمواصلات", "transport", "410301", 1200.0, false, "semester"),
        ("FEE_BOOKS", "Books & Educational Resources", "رسوم الكتب والمصادر التعليمية", "materials", "410401", 600.0, true, "annual"),
        ("FEE_TECH", "Digital Platform & Technology Fee", "رسوم المنظومة الرقمية والتقنية", "technology", "410501", 300.0, true, "semester"),
        ("FEE_ACT", "Student Activities & Laboratory Fee", "رسوم الأنشطة الطلابية والمختبرات", "activities", "410601", 250.0, false, "semester"),
    ];

    for (code, name_en, name_ar, cat, acc_code, amt, is_mand, freq) in fee_defs {
        let rev_row = db.query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT id FROM accounts WHERE org_id = $1 AND code = $2 LIMIT 1",
            vec![org_id.into(), acc_code.into()]
        )).await.ok().flatten();

        let rev_id: Uuid = match rev_row {
            Some(r) => r.try_get("", "id").unwrap_or(rec_id),
            None => rec_id,
        };

        let fid = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO fee_templates (id, org_id, code, name, name_ar, category, default_amount, currency_code, revenue_account_id, receivable_account_id, frequency, is_mandatory, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 1, $13, $14)",
            vec![
                fid.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(), cat.into(),
                amt.into(), currency.into(),
                rev_id.into(), rec_id.into(),
                freq.into(),
                (if is_mand { 1i32 } else { 0i32 }).into(),
                now.to_string().into(), now.to_string().into()
            ]
        )).await.ok();
    }

    Ok(())
}

async fn seed_discount_templates<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    let discounts = vec![
        ("DISC_SIBLING", "Sibling Discount", "خصم الأخوة والأشقاء", "percentage", 10.0, false, true, false, 1),
        ("DISC_MERIT", "Academic Excellence Scholarship", "منحة التفوق والتميز الأكاديمي", "percentage", 25.0, true, false, false, 2),
        ("DISC_STAFF", "Staff Dependent Discount", "خصم أبناء الكادر الوظيفي والتعليمي", "percentage", 50.0, true, false, false, 1),
        ("DISC_EARLY", "Early Payment Incentive", "خصم السداد المبكر", "percentage", 5.0, false, true, true, 3),
        ("DISC_AID", "Financial Aid & Social Hardship", "إعانة وتخفيض التكافل الاجتماعي", "percentage", 30.0, true, false, false, 2),
    ];

    for (code, name_en, name_ar, d_type, val, req_app, auto_app, stack, prio) in discounts {
        let did = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO discount_templates (id, org_id, code, name, name_ar, discount_type, value, is_approval_required, is_auto_apply, is_stackable, priority, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 1, $12, $13) ON CONFLICT DO NOTHING",
            vec![
                did.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(), d_type.into(),
                val.into(),
                (if req_app { 1i32 } else { 0i32 }).into(),
                (if auto_app { 1i32 } else { 0i32 }).into(),
                (if stack { 1i32 } else { 0i32 }).into(),
                prio.into(),
                now.to_string().into(), now.to_string().into()
            ]
        )).await.ok();
    }

    Ok(())
}

async fn seed_currencies<C: ConnectionTrait>(
    db: &C,
    now: &str,
) -> Result<(), DomainError> {
    let all_currencies = vec![
        ("SAR", "Saudi Riyal", "ريال سعودي", "ر.س", 2),
        ("YER", "Yemeni Rial", "ريال يمني", "ر.ي", 2),
        ("AED", "UAE Dirham", "درهم إماراتي", "د.إ", 2),
        ("KWD", "Kuwaiti Dinar", "دينار كويتي", "د.ك", 3),
        ("QAR", "Qatari Rial", "ريال قطري", "ر.ق", 2),
        ("BHD", "Bahraini Dinar", "دينار بحريني", "د.ب", 3),
        ("OMR", "Omani Rial", "ريال عماني", "ر.ع", 3),
        ("EGP", "Egyptian Pound", "جنيه مصري", "ج.م", 2),
        ("JOD", "Jordanian Dinar", "دينار أردني", "د.أ", 3),
        ("IQD", "Iraqi Dinar", "دينار عراقي", "د.ع", 0),
        ("LYD", "Libyan Dinar", "دينار ليبي", "د.ل", 3),
        ("DZD", "Algerian Dinar", "دينار جزائري", "د.ج", 2),
        ("MAD", "Moroccan Dirham", "درهم مغربي", "د.م", 2),
        ("TND", "Tunisian Dinar", "دينار تونسي", "د.ت", 3),
        ("SDG", "Sudanese Pound", "جنيه سوداني", "ج.س", 2),
        ("TRY", "Turkish Lira", "ليرة تركية", "₺", 2),
        ("USD", "US Dollar", "دولار أمريكي", "$", 2),
        ("EUR", "Euro", "يورو", "€", 2),
        ("GBP", "British Pound", "جنيه إسترليني", "£", 2),
    ];

    for (code, name_en, name_ar, symbol, decimals) in all_currencies {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO currencies (id, name, name_ar, symbol, decimal_places, is_active, created_at) VALUES ($1, $2, $3, $4, $5, 1, $6) ON CONFLICT DO NOTHING",
            vec![
                code.into(), name_en.into(), name_ar.into(),
                symbol.into(), decimals.into(), now.to_string().into()
            ]
        )).await.ok();
    }
    Ok(())
}

async fn seed_roles_and_permissions<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    let default_roles = vec![
        ("super_admin", "Super Administrator", "مدير نظام عام"),
        ("admin", "Operations Administrator", "مدير النظام والعمليات"),
        ("principal", "Principal / Dean", "مدير المؤسسة / عميد الكلية"),
        ("academic_advisor", "Academic Advisor", "المرشد الأكاديمي والطلابي"),
        ("department_head", "Department Head", "رئيس القسم الأكاديمي"),
        ("teacher", "Teacher / Instructor", "معلم / أستاذ / محاضر"),
        ("accountant", "Financial Accountant", "المحاسب المالي"),
        ("cashier", "Cashier / Collector", "أمين الصندوق / المحصل"),
        ("registrar", "Registrar & Admissions", "مسجل القبول وشؤون الطلاب"),
        ("student", "Student", "طالب / متدرب"),
        ("parent", "Parent / Guardian", "ولي أمر / وكيل الطالب"),
        ("staff", "Administrative Staff", "موظف إداري"),
        ("auditor", "Compliance Auditor", "مدقق ومراجع أكاديمي ومالي"),
        ("ai_agent", "Autonomous AI Agent", "وكيل الذكاء الاصطناعي المستقل"),
    ];

    for (role_id, name_en, name_ar) in default_roles {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO roles (id, org_id, name, name_ar, is_system, created_at, updated_at) VALUES ($1, $2, $3, $4, 1, $5, $6) ON CONFLICT DO NOTHING",
            vec![
                role_id.into(), org_id.into(),
                name_en.into(), name_ar.into(),
                now.to_string().into(), now.to_string().into(),
            ]
        )).await.ok();
    }

    let assignment_id_uuid = Uuid::new_v4();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO user_role_assignments (id, user_id, role_id, org_id, scope_type, scope_id, assigned_by, created_at) VALUES ($1, $2, 'super_admin', $3, 'global', NULL, NULL, $4)",
        vec![
            assignment_id_uuid.into(), admin_id.into(),
            org_id.into(), now.to_string().into(),
        ]
    )).await.ok();

    for perm in comprehensive_role_permissions() {
        for action in perm.actions {
            let perm_id_uuid = Uuid::new_v4();
            db.execute(Statement::from_sql_and_values(
                db.get_database_backend(),
                "INSERT INTO role_permissions (id, role_id, org_id, resource, action, effect, created_at) VALUES ($1, $2, $3, $4, $5, 'allow', $6)",
                vec![
                    perm_id_uuid.into(),
                    perm.role.clone().into(),
                    org_id.into(),
                    perm.document_type.clone().into(),
                    action.clone().into(),
                    now.to_string().into(),
                ]
            )).await.ok();
        }
    }

    Ok(())
}

async fn seed_org_settings<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    currency: &str,
    blueprint: &InstitutionBlueprint,
) -> Result<(), DomainError> {
    let org_settings = serde_json::json!({
        "academic": {
            "default_grading_system": blueprint.grading_system,
            "passing_grade": blueprint.passing_grade,
            "requires_all_subjects": blueprint.requires_all_subjects,
            "max_failed_subjects": blueprint.max_failed_subjects,
            "supplementary_eligible": blueprint.supplementary_eligible,
            "attendance_minimum_percentage": 75.0,
            "auto_lock_grades": true,
            "grade_lock_grace_period_days": 7,
            "ranking_enabled": blueprint.ranking_enabled == 1,
            "ranking_scope": blueprint.ranking_scope
        },
        "enrollment": {
            "registration_enabled": true,
            "require_admin_approval": true,
            "allow_waitlist": true,
            "sibling_priority_enabled": true,
            "max_class_size_default": 25,
            "allow_late_enrollment": false,
            "auto_generate_student_id": true,
            "student_id_prefix": "STD"
        },
        "finance": {
            "currency": currency,
            "tax_rate": 0.15,
            "invoice_due_days": 14,
            "late_fee_type": "fixed",
            "late_fee_value": 0.0,
            "allow_partial_payments": true,
            "zatca_einvoicing_enabled": true,
            "primary_payment_gateway_id": null,
            "invoicing_integration_id": null
        },
        "notifications": {
            "email_integration_id": null,
            "sms_integration_id": null,
            "whatsapp_integration_id": null,
            "notify_parents_on_absence": true,
            "notify_parents_on_grade_publish": true,
            "notify_on_fee_due": true,
            "finance_reminder_days": [7, 3, 1]
        },
        "autopilot": {
            "enabled": true,
            "mode": "automatic",
            "max_retries": 3,
            "auto_advance_periods": false,
            "auto_lock_grades": true,
            "grade_entry_auto_close_days": 7,
            "allow_grade_edit_after_close": false,
            "daily_data_integrity_check": true
        },
        "ai_features": {
            "enabled": true,
            "copilot_enabled": true,
            "predictive_risk_detection": true,
            "smart_timetable_optimizer": true,
            "automated_grading_assistance": true
        },
        "security": {
            "enforce_mfa_for_admins": false,
            "session_timeout_minutes": 120,
            "max_login_attempts": 5
        }
    });

    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "UPDATE organizations SET settings = $1 WHERE id = $2",
        vec![org_settings.to_string().into(), org_id.into()]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to save organization settings: {e}")))?;

    Ok(())
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
    org_id: Uuid,
    parent_id: Option<Uuid>,
    parent_path: &str,
    depth: i32,
    currency: &str,
    now: &str,
    out: &mut Vec<(Uuid, Uuid, String, String, String, String, Option<Uuid>, String, i32, String, String)>,
) {
    for node in nodes {
        let acc_id = Uuid::new_v4();
        let path = if parent_path.is_empty() {
            node.code.clone()
        } else {
            format!("{}.{}", parent_path, node.code)
        };
        let db_type = node.acc_type.to_lowercase();

        out.push((
            acc_id,
            org_id,
            node.code.clone(),
            node.name_en.clone(),
            node.name_ar.clone(),
            db_type,
            parent_id,
            path.clone(),
            depth,
            currency.to_string(),
            now.to_string(),
        ));

        if let Some(ref children) = node.children {
            flatten_acc_nodes(children, org_id, Some(acc_id), &path, depth + 1, currency, now, out);
        }
    }
}

async fn seed_chart_of_accounts<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    currency: &str,
    now: &str,
) -> Result<(), DomainError> {
    let coa: CoaData = serde_json::from_str(COA_JSON)
        .map_err(|e| DomainError::Internal(format!("Failed to parse COA JSON: {e}")))?;

    let mut rows = Vec::new();
    flatten_acc_nodes(&coa.accounts, org_id, None, "", 1, currency, now, &mut rows);

    for (id, oid, code, _name, name_ar, db_type, parent_id, _path, _depth, curr, timestamp) in rows {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO accounts (id, org_id, code, name, account_type, parent_id, currency_code, is_active, balances, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 1, '{}', $8, $9)",
            vec![
                id.into(), oid.into(), code.clone().into(), name_ar.into(),
                db_type.into(),
                parent_id.into(), curr.into(),
                timestamp.clone().into(), timestamp.into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to seed account {code}: {e}")))?;
    }

    Ok(())
}

// ── Internal Blueprint Definitions ──────────────────────────────

#[allow(dead_code)]
struct GradeItem {
    label: String,
    min_score: f64,
    max_score: f64,
    gpa_points: f64,
    is_passing: bool,
}

#[allow(dead_code)]
struct RolePermission {
    role: String,
    document_type: String,
    actions: Vec<String>,
}

#[allow(dead_code)]
struct AssessmentComponent {
    name: String,
    component_type: String,
    weight: f64,
    max_score: f64,
    passing_score: f64,
}

#[allow(dead_code)]
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
        // ── Grading Architecture ────────────────────────────
        assessment_scheme_name: "المخطط التقييمي الشهري".into(),
        assessment_components: vec![
            AssessmentComponent { name: "الواجبات".into(), component_type: "assignment".into(), weight: 20.0, max_score: 20.0, passing_score: 10.0 },
            AssessmentComponent { name: "الشفهي".into(), component_type: "oral".into(), weight: 20.0, max_score: 20.0, passing_score: 10.0 },
            AssessmentComponent { name: "الحضور والمشاركة".into(), component_type: "attendance".into(), weight: 20.0, max_score: 20.0, passing_score: 10.0 },
            AssessmentComponent { name: "الاختبار التحريري".into(), component_type: "written_exam".into(), weight: 40.0, max_score: 40.0, passing_score: 20.0 },
        ],
        coursework_weight: 20.0,
        exam_weight: 30.0,
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
        assessment_scheme_name: "University Assessment".into(),
        assessment_components: vec![],
        coursework_weight: 40.0,
        exam_weight: 60.0,
        ranking_enabled: 0,
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

// ── Shared Defaults & Permissions Matrix ─────────────────────────

fn comprehensive_role_permissions() -> Vec<RolePermission> {
    vec![
        // Super Admin & Admin
        RolePermission { role: "super_admin".into(), document_type: "*".into(), actions: vec!["*".into(), "read".into(), "write".into(), "delete".into(), "admin".into()] },
        RolePermission { role: "admin".into(), document_type: "*".into(), actions: vec!["*".into(), "read".into(), "write".into(), "delete".into(), "admin".into()] },

        // Principal / Dean
        RolePermission { role: "principal".into(), document_type: "Organization".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "OrgUnit".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "User".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Period".into(), actions: vec!["read".into(), "write".into(), "activate".into(), "lock".into()] },
        RolePermission { role: "principal".into(), document_type: "Class".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Subject".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Schedule".into(), actions: vec!["read".into(), "write".into(), "optimize".into()] },
        RolePermission { role: "principal".into(), document_type: "Grade".into(), actions: vec!["read".into(), "write".into(), "approve".into(), "lock".into(), "analytics".into()] },
        RolePermission { role: "principal".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "write".into(), "analytics".into()] },
        RolePermission { role: "principal".into(), document_type: "Assessment".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Enrollment".into(), actions: vec!["read".into(), "write".into(), "promote".into(), "transfer".into()] },
        RolePermission { role: "principal".into(), document_type: "Exam".into(), actions: vec!["read_schedule".into(), "write_schedule".into(), "read_hall".into(), "write_hall".into(), "allocate_hall".into(), "read_proctor".into(), "assign_proctor".into()] },
        RolePermission { role: "principal".into(), document_type: "Report".into(), actions: vec!["read".into(), "write".into(), "generate".into()] },
        RolePermission { role: "principal".into(), document_type: "AI".into(), actions: vec!["use_copilot".into(), "analytics".into(), "risk_prediction".into()] },

        // Academic Advisor
        RolePermission { role: "academic_advisor".into(), document_type: "Student".into(), actions: vec!["read".into(), "counsel".into()] },
        RolePermission { role: "academic_advisor".into(), document_type: "Grade".into(), actions: vec!["read".into(), "analytics".into()] },
        RolePermission { role: "academic_advisor".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "analytics".into()] },
        RolePermission { role: "academic_advisor".into(), document_type: "Behavior".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "academic_advisor".into(), document_type: "Enrollment".into(), actions: vec!["read".into()] },
        RolePermission { role: "academic_advisor".into(), document_type: "Schedule".into(), actions: vec!["read".into()] },
        RolePermission { role: "academic_advisor".into(), document_type: "AI".into(), actions: vec!["use_copilot".into(), "risk_prediction".into()] },

        // Department Head
        RolePermission { role: "department_head".into(), document_type: "Subject".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "department_head".into(), document_type: "Class".into(), actions: vec!["read".into()] },
        RolePermission { role: "department_head".into(), document_type: "Grade".into(), actions: vec!["read".into(), "review".into(), "approve".into()] },
        RolePermission { role: "department_head".into(), document_type: "Schedule".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "department_head".into(), document_type: "Exam".into(), actions: vec!["read_schedule".into(), "read_proctor".into()] },
        RolePermission { role: "department_head".into(), document_type: "Teacher".into(), actions: vec!["read".into(), "evaluate".into()] },

        // Teacher
        RolePermission { role: "teacher".into(), document_type: "Grade".into(), actions: vec!["read".into(), "write".into(), "submit".into()] },
        RolePermission { role: "teacher".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "teacher".into(), document_type: "Assessment".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "teacher".into(), document_type: "Class".into(), actions: vec!["read".into()] },
        RolePermission { role: "teacher".into(), document_type: "Subject".into(), actions: vec!["read".into()] },
        RolePermission { role: "teacher".into(), document_type: "Schedule".into(), actions: vec!["read".into()] },
        RolePermission { role: "teacher".into(), document_type: "Behavior".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "teacher".into(), document_type: "Exam".into(), actions: vec!["read_schedule".into(), "read_hall".into(), "read_proctor".into()] },
        RolePermission { role: "teacher".into(), document_type: "Document".into(), actions: vec!["read".into()] },
        RolePermission { role: "teacher".into(), document_type: "AI".into(), actions: vec!["use_copilot".into(), "grading_assist".into()] },

        // Accountant
        RolePermission { role: "accountant".into(), document_type: "Account".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "accountant".into(), document_type: "JournalEntry".into(), actions: vec!["read".into(), "write".into(), "post".into()] },
        RolePermission { role: "accountant".into(), document_type: "FeeTemplate".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "accountant".into(), document_type: "DiscountTemplate".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "accountant".into(), document_type: "StudentFee".into(), actions: vec!["read".into(), "write".into(), "adjust".into()] },
        RolePermission { role: "accountant".into(), document_type: "Payment".into(), actions: vec!["read".into(), "write".into(), "refund".into()] },
        RolePermission { role: "accountant".into(), document_type: "Report".into(), actions: vec!["read".into(), "generate_financial".into()] },

        // Cashier
        RolePermission { role: "cashier".into(), document_type: "StudentFee".into(), actions: vec!["read".into()] },
        RolePermission { role: "cashier".into(), document_type: "Payment".into(), actions: vec!["read".into(), "write".into(), "issue_receipt".into()] },
        RolePermission { role: "cashier".into(), document_type: "Student".into(), actions: vec!["read".into()] },

        // Registrar
        RolePermission { role: "registrar".into(), document_type: "Student".into(), actions: vec!["read".into(), "write".into(), "enroll".into()] },
        RolePermission { role: "registrar".into(), document_type: "Guardian".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "registrar".into(), document_type: "Enrollment".into(), actions: vec!["read".into(), "write".into(), "promote".into()] },
        RolePermission { role: "registrar".into(), document_type: "Class".into(), actions: vec!["read".into()] },
        RolePermission { role: "registrar".into(), document_type: "Document".into(), actions: vec!["read".into(), "issue_certificate".into()] },

        // Student
        RolePermission { role: "student".into(), document_type: "Grade".into(), actions: vec!["read_self".into()] },
        RolePermission { role: "student".into(), document_type: "Attendance".into(), actions: vec!["read_self".into()] },
        RolePermission { role: "student".into(), document_type: "Schedule".into(), actions: vec!["read_self".into()] },
        RolePermission { role: "student".into(), document_type: "Enrollment".into(), actions: vec!["read_self".into()] },
        RolePermission { role: "student".into(), document_type: "Exam".into(), actions: vec!["read_schedule".into()] },
        RolePermission { role: "student".into(), document_type: "StudentFee".into(), actions: vec!["read_self".into()] },
        RolePermission { role: "student".into(), document_type: "AI".into(), actions: vec!["learning_assistant".into()] },

        // Parent
        RolePermission { role: "parent".into(), document_type: "Grade".into(), actions: vec!["read_child".into()] },
        RolePermission { role: "parent".into(), document_type: "Attendance".into(), actions: vec!["read_child".into()] },
        RolePermission { role: "parent".into(), document_type: "Schedule".into(), actions: vec!["read_child".into()] },
        RolePermission { role: "parent".into(), document_type: "StudentFee".into(), actions: vec!["read_child".into(), "pay".into()] },
        RolePermission { role: "parent".into(), document_type: "Document".into(), actions: vec!["read_child".into()] },

        // Staff
        RolePermission { role: "staff".into(), document_type: "User".into(), actions: vec!["read".into()] },
        RolePermission { role: "staff".into(), document_type: "Enrollment".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "staff".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "staff".into(), document_type: "Document".into(), actions: vec!["read".into(), "write".into()] },

        // Auditor
        RolePermission { role: "auditor".into(), document_type: "*".into(), actions: vec!["read".into(), "audit".into(), "export".into()] },

        // AI Agent
        RolePermission { role: "ai_agent".into(), document_type: "Autopilot".into(), actions: vec!["read".into(), "execute".into(), "analyze".into()] },
        RolePermission { role: "ai_agent".into(), document_type: "Grade".into(), actions: vec!["read".into(), "predict".into()] },
        RolePermission { role: "ai_agent".into(), document_type: "Attendance".into(), actions: vec!["read".into(), "predict".into()] },
        RolePermission { role: "ai_agent".into(), document_type: "Schedule".into(), actions: vec!["read".into(), "optimize".into()] },
    ]
}

fn slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

// ── Default Reports Seeding Implementation ──────────────────────

async fn seed_default_reports<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    info!("Seeding default reports for org_id={}", org_id);

    let templates = vec![
        (
            "administration.master-template",
            "Master Template",
            "الترويسة والتذييل الرسمي للمؤسسة",
            "system",
            "MASTER",
            crate::adapters::report_defaults::MASTER_TEMPLATE_HTML,
        ),
        (
            "academic.student-term-report-card",
            "Student Term Report Card",
            "كشف درجات الطالب للفصل الدراسي",
            "academic",
            "REPORT",
            crate::adapters::report_defaults::TABULAR_REPORT_HTML,
        ),
        (
            "academic.official-transcript",
            "Official Academic Transcript",
            "السجل الأكاديمي الشامل",
            "academic",
            "TRANSCRIPT",
            crate::adapters::report_defaults::TRANSCRIPT_REPORT_HTML,
        ),
        (
            "finance.payment-receipt",
            "Payment Receipt",
            "سند قبض مالي إلكتروني",
            "finance",
            "RECEIPT",
            crate::adapters::report_defaults::PAYMENT_RECEIPT_HTML,
        ),
        (
            "finance.student-invoice",
            "Student Fee Invoice & Statement",
            "فاتورة الرسوم وكشف حساب الطالب",
            "finance",
            "INVOICE",
            crate::adapters::report_defaults::STUDENT_INVOICE_HTML,
        ),
        (
            "identification.student-id-card",
            "Student ID Card",
            "بطاقة هوية الطالب",
            "identification",
            "ID_CARD",
            crate::adapters::report_defaults::STUDENT_ID_CARD_HTML,
        ),
        (
            "identification.staff-id-card",
            "Faculty & Staff ID Card",
            "بطاقة هوية الكادر التعليمي والإداري",
            "identification",
            "STAFF_CARD",
            crate::adapters::report_defaults::STAFF_ID_CARD_HTML,
        ),
        (
            "scheduling.class-timetable",
            "Class Timetable",
            "الجدول الدراسي الأسبوعي للفصل",
            "academic",
            "SCHEDULE",
            crate::adapters::report_defaults::CLASS_TIMETABLE_HTML,
        ),
        (
            "admission.enrollment-certificate",
            "Certificate of Enrollment",
            "شهادة قيد دراسي وتأكيد انتظام",
            "academic",
            "CERTIFICATE",
            crate::adapters::report_defaults::ENROLLMENT_CERTIFICATE_HTML,
        ),
    ];

    for (code, name_en, name_ar, domain, doc_type, html_content) in templates {
        // 1. Definition
        let def_id = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_definitions (id, org_id, code, name, name_ar, domain, category, data_provider_key, is_system, is_active, parameters_json, required_permissions, supported_formats, sort_order, version, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1, 1, '[]', '[]', '[\"pdf\", \"html\"]', 0, 1, $9, $10) ON CONFLICT(org_id, code) DO NOTHING",
            vec![
                def_id.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(), domain.into(), doc_type.into(), code.into(),
                now.to_string().into(), now.to_string().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to seed report_definitions for {code}: {e}")))?;

        // Need the def_id (it might already exist)
        let def_row = db.query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT id FROM report_definitions WHERE org_id = $1 AND code = $2 LIMIT 1",
            vec![org_id.into(), code.into()]
        )).await.map_err(|e| DomainError::Internal(e.to_string()))?.ok_or_else(|| DomainError::NotFound("Report definition not found".into()))?;
        let real_def_id: Uuid = def_row.try_get("", "id").map_err(|e| DomainError::Internal(e.to_string()))?;

        // 2. Template
        let tmpl_id = Uuid::new_v4();
        let nil_user_id = Uuid::nil();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_templates (id, org_id, code, name, name_ar, document_type, entity_type, status, version, published_version, page_size, orientation, margins_json, document_json, locale, is_system, is_default, created_by, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, 'system', 'published', 1, 1, 'A4', 'portrait', '{\"top\":15,\"bottom\":15,\"left\":15,\"right\":15}', $7, 'ar', 1, 1, $8, $9, $10) ON CONFLICT(org_id, code) DO NOTHING",
            vec![
                tmpl_id.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(), doc_type.into(),
                html_content.into(),
                nil_user_id.into(),
                now.to_string().into(), now.to_string().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to seed report_templates for {code}: {e}")))?;

        // Get actual template id
        let tmpl_row = db.query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT id FROM report_templates WHERE org_id = $1 AND code = $2 LIMIT 1",
            vec![org_id.into(), code.into()]
        )).await.map_err(|e| DomainError::Internal(e.to_string()))?.ok_or_else(|| DomainError::NotFound("Template not found".into()))?;
        let real_tmpl_id: Uuid = tmpl_row.try_get("", "id").map_err(|e| DomainError::Internal(e.to_string()))?;

        // 3. Binding
        let binding_id = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_template_bindings (id, report_definition_id, template_id, locale, format, is_default, created_at) SELECT $1, $2, $3, 'ar', 'pdf', 1, $4 WHERE NOT EXISTS (SELECT 1 FROM report_template_bindings WHERE report_definition_id = $2 AND format = 'pdf')",
            vec![
                binding_id.into(), real_def_id.into(), real_tmpl_id.into(),
                now.to_string().into(),
            ]
        )).await.ok();

        let binding_id_html = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_template_bindings (id, report_definition_id, template_id, locale, format, is_default, created_at) SELECT $1, $2, $3, 'ar', 'html', 1, $4 WHERE NOT EXISTS (SELECT 1 FROM report_template_bindings WHERE report_definition_id = $2 AND format = 'html')",
            vec![
                binding_id_html.into(), real_def_id.into(), real_tmpl_id.into(),
                now.to_string().into(),
            ]
        )).await.ok();
    }

    Ok(())
}

// ── BulkBootstrapPort Implementation ──────────────────────────────────────
// Adapter that implements the Application-layer port using efficient SQL.

use async_trait::async_trait;
use orbis_app::ports::bootstrap::{
    BulkBootstrapPort, ValidatedBootstrapData, ValidatedInitialSetupData,
    BootstrapPersistenceResult,
};

pub struct BootstrapAdapterImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl BulkBootstrapPort for BootstrapAdapterImpl {
    async fn persist_bootstrap(
        &self,
        data: ValidatedBootstrapData,
    ) -> Result<BootstrapPersistenceResult, DomainError> {
        // Delegate to the existing efficient bootstrap_full() which uses raw SQL
        // inside a single transaction for maximum performance.
        let payload = BootstrapPayload {
            org_name: data.organization.name.clone(),
            org_name_ar: if data.organization.name_ar.is_empty() { None } else { Some(data.organization.name_ar.clone()) },
            org_type: data.organization.org_type.clone(),
            currency: data.organization.base_currency_code.clone(),
            timezone: Some(data.organization.timezone.clone()),
            locale: Some(data.organization.locale.clone()),
            admin_name: data.admin_person.display_name.clone(),
            admin_username: data.admin_user.email.clone(),
            admin_password_hash: data.admin_user.password_hash.clone(),
        };

        let result = bootstrap_full(&self.db, payload).await?;

        Ok(BootstrapPersistenceResult {
            org_id: result.org_id,
            admin_user_id: result.admin_id,
            rows_inserted: 0, // Not tracked in the current impl
        })
    }

    async fn persist_initial_setup(
        &self,
        data: ValidatedInitialSetupData,
    ) -> Result<(), DomainError> {
        let txn = self.db.begin().await
            .map_err(|e| DomainError::Internal(format!("Failed to begin initial setup transaction: {e}")))?;

        let now = Utc::now().to_rfc3339();
        let org_id = data.org_id;

        let normalize_start = |s: &str| -> String {
            if s.len() == 10 && !s.contains('T') {
                format!("{}T00:00:00Z", s)
            } else {
                s.to_string()
            }
        };

        let normalize_end = |s: &str| -> String {
            if s.len() == 10 && !s.contains('T') {
                format!("{}T23:59:59Z", s)
            } else {
                s.to_string()
            }
        };

        let year_start = normalize_start(&data.academic_year.start_date);
        let year_end = normalize_end(&data.academic_year.end_date);

        // Academic Year
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO academic_years (id, org_id, name, name_ar, start_date, end_date, status, is_locked, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, 'active', 0, $7, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM academic_years WHERE org_id = $2 AND name = $3)",
            vec![
                data.academic_year.id.into(),
                org_id.into(),
                data.academic_year.name.clone().into(),
                data.academic_year.name_ar.clone().into(),
                year_start.into(),
                year_end.into(),
                now.clone().into(),
                now.clone().into(),
            ],
        )).await.map_err(|e| DomainError::Internal(format!("Failed to insert academic year: {e}")))?;

        // Periods
        for p in &data.periods {
            let p_start = normalize_start(&p.start_date);
            let p_end = normalize_end(&p.end_date);
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO academic_periods (id, org_id, year_id, name, name_ar, period_type, start_date, end_date, status, is_locked, is_enrollment_open, is_grade_entry_open, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, 'active', 0, 1, 1, $9, $10 \
                 WHERE NOT EXISTS (SELECT 1 FROM academic_periods WHERE org_id = $2 AND year_id = $3 AND name = $4)",
                vec![
                    p.id.into(),
                    org_id.into(),
                    p.year_id.into(),
                    p.name.clone().into(),
                    p.name_ar.clone().into(),
                    p.period_type.clone().into(),
                    p_start.into(),
                    p_end.into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert period: {e}")))?;
        }

        // First period ID for class references
        let period_id = data.periods.first()
            .map(|p| p.id)
            .unwrap_or(data.academic_year.id);

        // Query root org_unit for valid foreign key linking
        let root_unit_row = txn.query_one(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "SELECT id FROM org_units WHERE org_id = $1 ORDER BY depth ASC LIMIT 1",
            vec![org_id.into()]
        )).await.ok().flatten();

        let default_unit_id: Uuid = match root_unit_row {
            Some(r) => r.try_get("", "id").unwrap_or(Uuid::nil()),
            None => Uuid::nil(),
        };

        // Classes
        for c in &data.classes {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO classes (id, org_id, unit_id, period_id, name, name_ar, level, division, is_active, is_deleted, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, 1, 0, $9, $10 \
                 WHERE NOT EXISTS (SELECT 1 FROM classes WHERE org_id = $2 AND period_id = $4 AND name = $5)",
                vec![
                    c.id.into(),
                    org_id.into(),
                    default_unit_id.into(),
                    period_id.into(),
                    c.name.clone().into(),
                    c.name_ar.clone().into(),
                    c.level.clone().into(),
                    c.division.clone().unwrap_or_default().into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert class: {e}")))?;
        }

        // Subjects
        for s in &data.subjects {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO subjects (id, org_id, code, name, name_ar, is_core, is_elective, is_lab_required, is_active, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, 0, 0, 1, $7, $8 \
                 WHERE NOT EXISTS (SELECT 1 FROM subjects WHERE org_id = $2 AND code = $3)",
                vec![
                    s.id.into(),
                    org_id.into(),
                    s.code.clone().into(),
                    s.name.clone().into(),
                    s.name_ar.clone().into(),
                    (if s.is_core { 1i32 } else { 0i32 }).into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert subject: {e}")))?;
        }

        txn.commit().await
            .map_err(|e| DomainError::Internal(format!("Failed to commit initial setup: {e}")))?;

        Ok(())
    }
}
