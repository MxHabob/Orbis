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
use orbis_app::ports::onboarding::ValidatedOnboardingClass;
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
        "INSERT INTO organizations (id, name, name_ar, slug, timezone, locale, base_currency_code, is_hijri_enabled, is_active, is_verified, is_deleted, created_by, updated_by, version, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 1, 1, 1, 0, $8, $9, 1, $10, $11)",
        vec![
            org_id_uuid.into(),
            payload.org_name.clone().into(),
            payload.org_name_ar.clone().unwrap_or_default().into(),
            slug(&payload.org_name).into(),
            payload.timezone.clone().unwrap_or_else(|| "Asia/Riyadh".into()).into(),
            payload.locale.clone().unwrap_or_else(|| "ar".into()).into(),
            payload.currency.clone().into(),
            admin_id_uuid.into(),
            admin_id_uuid.into(),
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
        "INSERT INTO users (id, person_id, org_id, email, password_hash, is_email_verified, is_phone_verified, is_mfa_enabled, status, login_count, failed_login_attempts, is_deleted, created_by, updated_by, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 1, 0, 0, 'active', 0, 0, 0, $6, $7, $8, $9)",
        vec![
            admin_id_uuid.into(),
            person_id_uuid.into(),
            org_id_uuid.into(),
            payload.admin_username.clone().into(),
            payload.admin_password_hash.clone().into(),
            admin_id_uuid.into(),
            admin_id_uuid.into(),
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

    // ── 3. Period Templates Catalog (System templates for onboarding options) ────
    let tmpl_id = seed_period_templates(db, org_id_uuid, &payload.org_type, &blueprint, &now).await?;

    // ── 4. Currencies Catalog (System standard currencies) ────────────────────────
    if let Err(e) = seed_currencies(db, &now).await {
        info!("Note: Currencies seeding warning: {}", e);
    }

    // ── 5. Roles & Permissions Matrix (Core IAM RBAC) ────────────────────────────
    if let Err(e) = seed_roles_and_permissions(db, org_id_uuid, admin_id_uuid, &now).await {
        info!("Note: Roles seeding warning: {}", e);
    }

    // ── 5.1 System Default User Groups ──────────────────────────────────────────
    if let Err(e) = seed_default_user_groups(db, org_id_uuid, admin_id_uuid, &now).await {
        info!("Note: User groups seeding warning: {}", e);
    }

    // ── 5.5 Chart of Accounts (COA) ──────────────────────────────────────────────
    if let Err(e) = seed_chart_of_accounts(db, org_id_uuid, admin_id_uuid, &payload.currency, &now).await {
        info!("Note: COA seeding warning: {}", e);
    }

    // ── 5.7 Standard Discount Templates ──────────────────────────────────────────
    if let Err(e) = seed_discount_templates(db, org_id_uuid, &now).await {
        info!("Note: Discount templates seeding warning: {}", e);
    }

    // ── 5.8 Default Organization Settings ────────────────────────────────────────
    if let Err(e) = seed_org_settings(db, org_id_uuid, &payload.currency, &payload.org_type, &blueprint).await {
        info!("Note: Org settings seeding warning: {}", e);
    }

    // ── 6. Default Document & Report Templates (System templates) ─────────────────
    if let Err(e) = seed_default_reports(db, org_id_uuid, &now).await {
        info!("Note: Report seeding warning: {}", e);
    }

    // ── 7. Default Assessment Schemes (Evaluation architecture) ───────────────────
    if let Err(e) = seed_assessment_schemes(db, org_id_uuid, &blueprint, &now).await {
        info!("Note: Assessment scheme seeding warning: {}", e);
    }

    // ── 8. Initial Audit Log Entries ──────────────────────────────────────────────
    if let Err(e) = seed_initial_audit_logs(db, org_id_uuid, admin_id_uuid, &now).await {
        info!("Note: Audit log seeding warning: {}", e);
    }

    info!("Full technical bootstrap completed for org_id={} type={}", org_id, payload.org_type);

    Ok(BootstrapResult {
        org_id,
        admin_id,
        period_template_id: tmpl_id,
    })
}

// ── Subsystem Seeders ───────────────────────────────────────────

#[allow(dead_code)]
async fn seed_org_units<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Option<Uuid>,
    _org_type: &str,
    now: &str,
) -> Result<Uuid, DomainError> {
    let root_id = Uuid::new_v4();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO org_units (id, org_id, parent_id, path, depth, name, name_ar, code, unit_type, data_isolation, children_count, descendant_count, sort_order, is_active, created_by, updated_by, created_at, updated_at) VALUES ($1, $2, NULL, 'root', 0, 'Main Campus', 'المقر الرئيسي', 'HQ', 'campus', 'none', 0, 0, 1, 1, $3, $4, $5, $6)",
        vec![root_id.into(), org_id.into(), admin_id.into(), admin_id.into(), now.to_string().into(), now.to_string().into()]
    )).await.map_err(|e| DomainError::Internal(format!("Failed to seed root org_unit: {e}")))?;

    // Note: Child organizational units (Branches, Buildings, Stages, Departments)
    // are created and structured dynamically by the Onboarding wizard according to the institution's actual configuration.
    Ok(root_id)
}

async fn seed_period_templates<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    org_type: &str,
    _blueprint: &InstitutionBlueprint,
    now: &str,
) -> Result<String, DomainError> {
    let standard_templates = vec![
        (
            Uuid::new_v4().to_string(),
            "Two Semesters with Monthly Assessments".to_string(),
            "نظام الفصلين الدراسيين مع تقييمات شهرية".to_string(),
            serde_json::json!([
                {
                    "name": "First Semester",
                    "name_ar": "الفصل الدراسي الأول",
                    "type": "semester",
                    "weight": 50,
                    "children": [
                        { "name": "Month 1 Assessment", "name_ar": "تقييم الشهر الأول", "type": "monthly", "weight": 10 },
                        { "name": "Month 2 Assessment", "name_ar": "تقييم الشهر الثاني", "type": "monthly", "weight": 10 },
                        { "name": "Month 3 Assessment", "name_ar": "تقييم الشهر الثالث", "type": "monthly", "weight": 10 },
                        { "name": "Midterm Exam", "name_ar": "اختبار منتصف الفصل الأول", "type": "exam", "weight": 20 },
                        { "name": "First Semester Final Exam", "name_ar": "اختبار نهاية الفصل الأول", "type": "final_exam", "weight": 50 }
                    ]
                },
                {
                    "name": "Second Semester",
                    "name_ar": "الفصل الدراسي الثاني",
                    "type": "semester",
                    "weight": 50,
                    "children": [
                        { "name": "Month 4 Assessment", "name_ar": "تقييم الشهر الرابع", "type": "monthly", "weight": 10 },
                        { "name": "Month 5 Assessment", "name_ar": "تقييم الشهر الخامس", "type": "monthly", "weight": 10 },
                        { "name": "Month 6 Assessment", "name_ar": "تقييم الشهر السادس", "type": "monthly", "weight": 10 },
                        { "name": "Midterm Exam", "name_ar": "اختبار منتصف الفصل الثاني", "type": "exam", "weight": 20 },
                        { "name": "Second Semester Final Exam", "name_ar": "اختبار نهاية الفصل الثاني", "type": "final_exam", "weight": 50 }
                    ]
                }
            ]),
            1i32,
        ),
        (
            Uuid::new_v4().to_string(),
            "Two Standard Semesters (Fall & Spring)".to_string(),
            "نظام الفصلين الدراسيين القياسي (ترم أول وترم ثاني)".to_string(),
            serde_json::json!([
                { "name": "First Semester", "name_ar": "الفصل الدراسي الأول", "type": "semester", "weight": 50 },
                { "name": "Second Semester", "name_ar": "الفصل الدراسي الثاني", "type": "semester", "weight": 50 }
            ]),
            0i32,
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
    let components: Vec<serde_json::Value> = blueprint.assessment_components.iter().enumerate().map(|(idx, c)| {
        serde_json::json!({
            "id": format!("comp_{}", idx + 1),
            "name": c.name,
            "component_type": c.component_type,
            "weight": c.weight,
            "max_score": c.max_score,
            "passing_score": c.passing_score,
        })
    }).collect();

    let formula_json = serde_json::json!({
        "type": "weighted_sum",
        "components": components,
    });

    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO assessment_schemes (id, org_id, name, description, max_score, formula, created_at, updated_at) \
         SELECT $1, $2, $3, $4, $5, $6, $7, $8 \
         WHERE NOT EXISTS (SELECT 1 FROM assessment_schemes WHERE org_id = $2)",
        vec![
            scheme_id_uuid.into(), org_id.into(),
            blueprint.assessment_scheme_name.clone().into(),
            "هيكل التقييم القياسي المعتمد للمؤسسة".into(),
            100.0f64.into(),
            formula_json.to_string().into(),
            now.to_string().into(), now.to_string().into(),
        ]
    )).await.ok();
    Ok(())
}

pub async fn seed_discount_templates<C: ConnectionTrait>(
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
            "INSERT INTO discount_templates (id, org_id, code, name, name_ar, discount_type, value, is_approval_required, is_auto_apply, is_stackable, priority, is_active, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 1, $12, $13 \
             WHERE NOT EXISTS (SELECT 1 FROM discount_templates WHERE org_id = $2 AND code = $3)",
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

fn detect_grade_level(name: &str, name_ar: &str, level_str: &str) -> Option<u32> {
    if let Ok(n) = level_str.trim().parse::<u32>() {
        return Some(n);
    }
    let lower_name = name.to_lowercase();
    let lower_level = level_str.to_lowercase();
    let combined = format!("{lower_name} {name_ar} {lower_level}");

    if combined.contains("kg1") || combined.contains("روضة 1") || combined.contains("روضه 1") || combined.contains("تمهيدي 1") || combined.contains("kg 1") {
        return Some(0);
    }
    if combined.contains("kg2") || combined.contains("روضة 2") || combined.contains("روضه 2") || combined.contains("تمهيدي 2") || combined.contains("kg 2") || combined.contains("روضة") || combined.contains("روضه") || combined.contains("تمهيدي") {
        return Some(0);
    }

    if combined.contains("12") || combined.contains("ثالث ثانوي") || combined.contains("الثالث الثانوي") || combined.contains("grade 12") || combined.contains("g12") {
        return Some(12);
    }
    if combined.contains("11") || combined.contains("ثاني ثانوي") || combined.contains("الثاني الثانوي") || combined.contains("grade 11") || combined.contains("g11") {
        return Some(11);
    }
    if combined.contains("10") || combined.contains("أول ثانوي") || combined.contains("اول ثانوي") || combined.contains("الأول الثانوي") || combined.contains("grade 10") || combined.contains("g10") {
        return Some(10);
    }
    if combined.contains("9") || combined.contains("تاسع") || combined.contains("التاسع") || combined.contains("ثالث إعدادي") || combined.contains("ثالث متوسط") || combined.contains("grade 9") || combined.contains("g9") {
        return Some(9);
    }
    if combined.contains("8") || combined.contains("ثامن") || combined.contains("الثامن") || combined.contains("ثاني إعدادي") || combined.contains("ثاني متوسط") || combined.contains("grade 8") || combined.contains("g8") {
        return Some(8);
    }
    if combined.contains("7") || combined.contains("سابع") || combined.contains("السابع") || combined.contains("أول إعدادي") || combined.contains("اول اعدادي") || combined.contains("أول متوسط") || combined.contains("grade 7") || combined.contains("g7") {
        return Some(7);
    }
    if combined.contains("6") || combined.contains("سادس") || combined.contains("السادس") || combined.contains("grade 6") || combined.contains("g6") {
        return Some(6);
    }
    if combined.contains("5") || combined.contains("خامس") || combined.contains("الخامس") || combined.contains("grade 5") || combined.contains("g5") {
        return Some(5);
    }
    if combined.contains("4") || combined.contains("رابع") || combined.contains("الرابع") || combined.contains("grade 4") || combined.contains("g4") {
        return Some(4);
    }
    if combined.contains("3") || combined.contains("ثالث") || combined.contains("الثالث") || combined.contains("grade 3") || combined.contains("g3") {
        return Some(3);
    }
    if combined.contains("2") || combined.contains("ثاني") || combined.contains("الثاني") || combined.contains("grade 2") || combined.contains("g2") {
        return Some(2);
    }
    if combined.contains("1") || combined.contains("أول") || combined.contains("اول") || combined.contains("الأول") || combined.contains("grade 1") || combined.contains("g1") {
        return Some(1);
    }

    None
}

pub async fn seed_document_requirements<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    org_type: &str,
    classes: &[ValidatedOnboardingClass],
    now: &str,
) -> Result<(), DomainError> {
    // 1. Organization-wide Global Document Requirements
    let mut doc_entries: Vec<(Option<Uuid>, &str, &str, &str, &str, bool, bool, &str)> = Vec::new();

    match org_type {
        "university" => {
            doc_entries.push((None, "global", "DOC_UNI_HIGH_SCHOOL_CERT", "Original High School Certificate", "أصل شهادة الثانوية العامة / كشف الدرجات المعتمد", true, true, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_STUDENT_ID", "Student National ID Card / Passport", "أصل البطاقة الشخصية / جواز السفر للطالب", true, true, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_PHOTOS", "Personal Photos (6 Photos 4x6)", "عدد 6 صور شخصية حديثة بخلفية بيضاء", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_CONDUCT", "Good Conduct Certificate", "شهادة حسن السيرة والسلوك", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_MILITARY", "Military Service Form", "استمارة الخدمة العسكرية / وثيقة الموقف التجنيدي (للذكور)", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_TRANSFER", "University Transfer & Equivalency Form", "استمارة ونموذج المقاصة والتحويل الأكاديمي", true, true, "transfer_students_only"));
            doc_entries.push((None, "global", "DOC_UNI_INT_PASSPORT", "Valid Passport & Student Visa/Iqama", "جواز السفر وتأشيرة الإقامة الدراسية للوافدين", true, true, "non_national_only"));

            // Dynamic per-academic level transcripts for universities
            for c in classes {
                let name_ar = c.name_ar.as_deref().unwrap_or(&c.name);
                let level_num = detect_grade_level(&c.name, name_ar, &c.level).unwrap_or(1);
                if level_num >= 2 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y1",
                        "Year 1 Official Academic Transcript",
                        "كشف درجات السنة الأولى الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
                if level_num >= 3 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y2",
                        "Year 2 Official Academic Transcript",
                        "كشف درجات السنة الثانية الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
                if level_num >= 4 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y3",
                        "Year 3 Official Academic Transcript",
                        "كشف درجات السنة الثالثة الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
                if level_num >= 5 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y4",
                        "Year 4 Official Academic Transcript",
                        "كشف درجات السنة الرابعة الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
            }
        },
        "institute" | "training_center" => {
            doc_entries.push((None, "global", "DOC_INST_NATIONAL_ID", "National ID Card / Valid Passport", "البطاقة الشخصية / الهوية الوطنية / جواز السفر", true, true, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_QUALIFICATION", "Last Academic Qualification Certificate", "أصل أو صورة مصدقة لآخر مؤهل دراسي / شهادة دراسية", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_PHOTOS", "Personal Photos (4x6)", "عدد 4 صور شخصية حديثة", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_GUARDIAN_CONSENT", "Guardian Approval & Consent Letter", "موافقة خطية من ولي الأمر (للمتدربين دون سن الرشد)", false, false, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_NON_NAT", "Valid Residency / Iqama Copy", "صورة الإقامة السارية للوافدين والمقيمين", true, true, "non_national_only"));
        },
        _ => {
            // General school global requirements
            doc_entries.push((None, "global", "DOC_SCH_PHOTO", "Personal Photos (4x6)", "صور شخصية حديثة (4×6)", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_SCH_TRANSFER", "Official School Transfer Certificate", "استمارة التحويل المدرسي المعتمدة من الإدارة التعليمية", true, true, "transfer_students_only"));
            doc_entries.push((None, "global", "DOC_NON_NAT_PASSPORT", "Valid Passport & Iqama / Residence Permit", "جواز السفر والإقامة السارية للطالب وولي الأمر", true, true, "non_national_only"));

            // Exact cumulative grade certificates per class
            for c in classes {
                let name_ar = c.name_ar.as_deref().unwrap_or(&c.name);
                let grade = detect_grade_level(&c.name, name_ar, &c.level).unwrap_or(1);

                match grade {
                    0 => {
                        // Kindergarten (روضة / تمهيدي)
                        doc_entries.push((Some(c.id), "grade_level", "DOC_KG_BIRTH_CERT", "Child Official Birth Certificate", "شهادة ميلاد الطفل الرسمية المميكنة", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_KG_FATHER_ID", "Father / Guardian National ID Card", "بطاقة الرقم القومي / الهوية الوطنية لولي الأمر", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_KG_VACCINE", "Child Health Record & Vaccination Card", "كرت التطعيمات والسجل الصحي للطفل", true, false, "all_students"));
                    },
                    1 => {
                        // Grade 1 (الصف الأول الابتدائي)
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G1_BIRTH_CERT", "Original Birth Certificate", "أصل شهادة ميلاد الطفل المعتمدة", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G1_FATHER_ID", "Guardian National ID Copy", "صورة بطاقة الهوية الوطنية / الإقامة لولي الأمر", true, false, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G1_MEDICAL", "School Health Examination", "استمارة الفحص الطبي والتطعيمات", true, false, "all_students"));
                    },
                    2 => {
                        // Grade 2: Birth Cert + Grade 1
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G2_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G2_PREV_G1", "Grade 1 Pass Certificate / Report Card", "شهادة اجتياز الصف الأول الابتدائي / بيان الدرجات", true, true, "all_students"));
                    },
                    3 => {
                        // Grade 3: Birth Cert + Grade 1 + Grade 2
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G3_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G3_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G3_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                    },
                    4 => {
                        // Grade 4: Birth Cert + Grade 1 + Grade 2 + Grade 3
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_PREV_G3", "Grade 3 Pass Certificate", "شهادة اجتياز الصف الثالث الابتدائي", true, true, "all_students"));
                    },
                    5 => {
                        // Grade 5: Birth Cert + Grades 1-4
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G3", "Grade 3 Pass Certificate", "شهادة اجتياز الصف الثالث الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G4", "Grade 4 Pass Certificate", "شهادة اجتياز الصف الرابع الابتدائي", true, true, "all_students"));
                    },
                    6 => {
                        // Grade 6: Birth Cert + Grades 1-5
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G3", "Grade 3 Pass Certificate", "شهادة اجتياز الصف الثالث الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G4", "Grade 4 Pass Certificate", "شهادة اجتياز الصف الرابع الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G5", "Grade 5 Pass Certificate", "شهادة اجتياز الصف الخامس الابتدائي", true, true, "all_students"));
                    },
                    7 => {
                        // Grade 7 / 1st Prep: Primary Completion Cert + Birth Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G7_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G7_PRI_COMPLETION", "Primary Stage Completion Certificate", "أصل شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                    },
                    8 => {
                        // Grade 8 / 2nd Prep: Primary Completion + Grade 7 Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G8_PRI_COMPLETION", "Primary Stage Completion Certificate", "أصل شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G8_PREV_G7", "Grade 7 Pass Certificate", "شهادة اجتياز الصف الأول الإعدادي (السابع)", true, true, "all_students"));
                    },
                    9 => {
                        // Grade 9 / 3rd Prep: Primary Completion + Grade 7 + Grade 8
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G9_PRI_COMPLETION", "Primary Stage Completion Certificate", "أصل شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G9_PREV_G7", "Grade 7 Pass Certificate", "شهادة اجتياز الصف الأول الإعدادي (السابع)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G9_PREV_G8", "Grade 8 Pass Certificate", "شهادة اجتياز الصف الثاني الإعدادي (الثامن)", true, true, "all_students"));
                    },
                    10 => {
                        // Grade 10 / 1st Sec: Prep Completion + Student ID / Birth Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G10_ID_OR_BIRTH", "National ID Card or Birth Certificate", "أصل بطاقة الهوية الوطنية أو شهادة الميلاد", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G10_MID_COMPLETION", "Preparatory Stage Completion Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
                    },
                    11 => {
                        // Grade 11 / 2nd Sec: Prep Completion + Grade 10 Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G11_MID_COMPLETION", "Preparatory Stage Completion Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G11_PREV_G10", "Grade 10 Pass Certificate", "شهادة اجتياز الصف الأول الثانوي (العاشر)", true, true, "all_students"));
                    },
                    12 => {
                        // Grade 12 / 3rd Sec: Prep Completion + Grade 10 + Grade 11
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G12_MID_COMPLETION", "Preparatory Stage Completion Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G12_PREV_G10", "Grade 10 Pass Certificate", "شهادة اجتياز الصف الأول الثانوي (العاشر)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G12_PREV_G11", "Grade 11 Pass Certificate", "شهادة اجتياز الصف الثاني الثانوي (الحادي عشر)", true, true, "all_students"));
                    },
                    _ => {
                        // General grade fallback
                        doc_entries.push((Some(c.id), "grade_level", "DOC_GEN_PREV_CERT", "Previous Grade Pass Certificate", "شهادة اجتياز الصف السابق المعتمدة", true, true, "all_students"));
                    }
                }
            }

            // Fallback stage templates if classes list was empty during minimal seed
            if classes.is_empty() {
                doc_entries.push((None, "stage", "DOC_PRI_PREV_CERT", "Previous Grade Certificate", "شهادة الصف السابق / بيان الدرجات المعتمد", true, true, "all_students"));
                doc_entries.push((None, "stage", "DOC_MID_PRIMARY_CERT", "Primary Stage Completion Certificate", "شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                doc_entries.push((None, "stage", "DOC_SEC_MIDDLE_CERT", "Preparatory / Middle School Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
            }
        }
    }

    for (target_id, target_type, base_code, name_en, name_ar, is_mandatory, blocks_enrollment, condition) in doc_entries {
        let did = Uuid::new_v4();
        let code = match target_id {
            Some(tid) => format!("{}_{}", base_code, &tid.to_string()[..8]),
            None => base_code.to_string(),
        };

        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO document_requirements (id, org_id, target_type, target_id, code, name_ar, name_en, is_mandatory, blocks_enrollment, condition, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 \
             WHERE NOT EXISTS (SELECT 1 FROM document_requirements WHERE org_id = $2 AND code = $5)",
            vec![
                did.into(),
                org_id.into(),
                target_type.into(),
                target_id.into(),
                code.into(),
                name_ar.into(),
                name_en.into(),
                (if is_mandatory { 1i32 } else { 0i32 }).into(),
                (if blocks_enrollment { 1i32 } else { 0i32 }).into(),
                condition.into(),
                now.into(),
                now.into(),
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

pub async fn seed_roles_and_permissions<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    let default_roles = vec![
        ("super_admin", "Super Administrator", "مدير نظام عام"),
        ("admin", "Operations Administrator", "مدير النظام والعمليات"),
        ("org_admin", "Organization Administrator", "مدير المؤسسة"),
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
            "INSERT INTO roles (id, org_id, name, name_ar, is_system, created_at, updated_at) \
             SELECT $1, $2, $3, $4, 1, $5, $6 \
             WHERE NOT EXISTS (SELECT 1 FROM roles WHERE id = $1 AND org_id = $2)",
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
        "INSERT INTO user_role_assignments (id, user_id, role_id, org_id, scope_type, scope_id, assigned_by, created_at) \
         SELECT $1, $2, 'super_admin', $3, 'global', NULL, NULL, $4 \
         WHERE NOT EXISTS (SELECT 1 FROM user_role_assignments WHERE user_id = $2 AND org_id = $3)",
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
                "INSERT INTO role_permissions (id, role_id, org_id, resource, action, created_at) \
                 SELECT $1, $2, $3, $4, $5, $6 \
                 WHERE NOT EXISTS (SELECT 1 FROM role_permissions WHERE role_id = $2 AND org_id = $3 AND resource = $4 AND action = $5)",
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

pub async fn seed_default_user_groups<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    let default_groups = vec![
        (
            "School Leadership",
            "القيادة المدرسية والإدارة",
            "صلاحيات الإشراف الإداري والأكاديمي الشامل، إدارة الوحدات والسياسات العامة.",
            vec![
                "organization:read", "org_unit:read", "user:read", "employee:read", "group:read",
                "student:read", "period:read", "class:read", "grade:analytics", "grade:report",
                "dashboard:read", "audit:read", "attendance:analytics", "attendance:report",
            ],
        ),
        (
            "Teaching Faculty",
            "هيئة التدريس والمعلمين",
            "صلاحيات إدخال ورصد الدرجات والغياب اليومي، متابعة الجداول والحصص الدراسية.",
            vec![
                "class:read", "subject:read", "timetable:read",
                "attendance:read", "attendance:write",
                "grade:read", "grade:write", "grade:submit",
                "assessment:read", "student:read",
            ],
        ),
        (
            "Admissions & Student Affairs",
            "شؤون الطلاب والقبول",
            "إدارة ملفات الطلاب وأولياء الأمور، التسجيل، السجلات الأكاديمية والوثائق الرسمية.",
            vec![
                "student:read", "student:write", "student:create", "student:update",
                "enrollment:read", "enrollment:write", "enrollment:approve",
                "guardian:read", "guardian:write",
                "document:read", "document:write", "attendance:read",
            ],
        ),
        (
            "HR & Employee Affairs",
            "الموارد البشرية وشؤون الموظفين",
            "إدارة السجلات الوظيفية للكادر التعليمي والإداري، العقود، والصلاحيات والمجموعات.",
            vec![
                "employee:read", "employee:write", "employee:create", "employee:update",
                "group:read", "group:assign", "org_unit:read", "audit:read",
            ],
        ),
        (
            "Finance & Accounting",
            "الشؤون المالية والحسابات",
            "إدارة الرسوم الدراسية، الفواتير، سندات القبض، الخصومات والتقارير المالية.",
            vec![
                "finance:read", "finance:write", "finance:manage",
                "invoice:read", "invoice:write", "payment:read", "payment:write",
                "fee:read", "fee:write", "discount:read", "discount:write",
                "report:read", "report:generate",
            ],
        ),
        (
            "Examination Board",
            "لجنة الامتحانات والكنترول",
            "إدارة الاختبارات والتقييمات الفصلية، مراجعة واعتماد وتثبيت النتائج النهائية.",
            vec![
                "assessment:read", "assessment:write", "assessment:publish",
                "grade:read", "grade:write", "grade:approve", "grade:lock",
                "period:read", "class:read", "student:read", "report:generate",
            ],
        ),
    ];

    for (name_en, name_ar, desc, perms) in default_groups {
        let group_id = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO groups (id, org_id, name, name_ar, description, is_system, status, created_by, updated_by, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, 1, 'active', $6, $7, $8, $9 \
             WHERE NOT EXISTS (SELECT 1 FROM groups WHERE org_id = $2 AND name = $3)",
            vec![
                group_id.into(), org_id.into(),
                name_en.into(), name_ar.into(), desc.into(),
                admin_id.into(), admin_id.into(),
                now.to_string().into(), now.to_string().into(),
            ]
        )).await.ok();

        let actual_group_row = db.query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT id FROM groups WHERE org_id = $1 AND name = $2 LIMIT 1",
            vec![org_id.into(), name_en.into()]
        )).await.ok().flatten();

        if let Some(row) = actual_group_row {
            if let Ok(gid) = row.try_get::<Uuid>("", "id") {
                for perm in perms {
                    let perm_id = Uuid::new_v4();
                    db.execute(Statement::from_sql_and_values(
                        db.get_database_backend(),
                        "INSERT INTO group_permissions (id, org_id, group_id, permission, created_at) \
                         SELECT $1, $2, $3, $4, $5 \
                         WHERE NOT EXISTS (SELECT 1 FROM group_permissions WHERE org_id = $2 AND group_id = $3 AND permission = $4)",
                        vec![
                            perm_id.into(), org_id.into(), gid.into(),
                            perm.into(), now.to_string().into(),
                        ]
                    )).await.ok();
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
async fn seed_org_settings<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    currency: &str,
    org_type: &str,
    blueprint: &InstitutionBlueprint,
) -> Result<(), DomainError> {
    let (inst_type, id_prefix, id_digits, req_app, no_guardian, max_cap) = match org_type {
        "university" => ("university", "UNI", 5, true, true, 60),
        "institute" | "training_center" => ("institute", "TRN", 4, false, true, 25),
        "quran_center" => ("quran_center", "QRN", 4, false, false, 20),
        _ => ("school", "STU", 4, false, false, 30),
    };

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
            "require_admin_approval": req_app,
            "allow_waitlist": true,
            "sibling_priority_enabled": true,
            "max_class_size_default": max_cap,
            "allow_late_enrollment": false,
            "auto_generate_student_id": true,
            "student_id_prefix": id_prefix
        },
        "registration": {
            "institution_type": inst_type,
            "student_number_pattern": format!("{}-{{YEAR}}-{{SEQ:{}}}", id_prefix, id_digits),
            "student_number_prefix": id_prefix,
            "student_number_digits": id_digits,
            "student_number_year_format": "YEAR",
            "policy_settings": {
                "requires_approval": req_app,
                "allow_no_guardian": no_guardian,
                "allow_duplicate_national_id": false,
                "auto_assign_fees": true,
                "max_class_capacity": max_cap
            }
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

async fn seed_account_tree<C: ConnectionTrait>(
    db: &C,
    nodes: &[AccNode],
    org_id: Uuid,
    parent_id: Option<Uuid>,
    admin_id: Uuid,
    currency: &str,
    now: &str,
) -> Result<(), DomainError> {
    for node in nodes {
        let existing_row = db.query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT id FROM accounts WHERE org_id = $1 AND code = $2 LIMIT 1",
            vec![org_id.into(), node.code.clone().into()]
        )).await.map_err(|e| DomainError::Internal(e.to_string()))?;

        let current_acc_id = if let Some(row) = existing_row {
            row.try_get("", "id").unwrap_or_else(|_| Uuid::new_v4())
        } else {
            let new_id = Uuid::new_v4();
            let db_type = node.acc_type.to_lowercase();
            db.execute(Statement::from_sql_and_values(
                db.get_database_backend(),
                "INSERT INTO accounts (id, org_id, code, name, account_type, parent_id, currency_code, is_active, balances, created_by, updated_by, version, created_at, updated_at) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, 1, '{}', $8, $9, 1, $10, $11)",
                vec![
                    new_id.into(),
                    org_id.into(),
                    node.code.clone().into(),
                    node.name_ar.clone().into(),
                    db_type.into(),
                    parent_id.into(),
                    currency.into(),
                    admin_id.into(),
                    admin_id.into(),
                    now.into(),
                    now.into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to seed account {}: {e}", node.code)))?;
            new_id
        };

        if let Some(ref children) = node.children {
            Box::pin(seed_account_tree(db, children, org_id, Some(current_acc_id), admin_id, currency, now)).await?;
        }
    }
    Ok(())
}

async fn seed_chart_of_accounts<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    currency: &str,
    now: &str,
) -> Result<(), DomainError> {
    let coa: CoaData = serde_json::from_str(COA_JSON)
        .map_err(|e| DomainError::Internal(format!("Failed to parse COA JSON: {e}")))?;

    seed_account_tree(db, &coa.accounts, org_id, None, admin_id, currency, now).await
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
        // Super Admin & Admin & Org Admin
        RolePermission { role: "super_admin".into(), document_type: "*".into(), actions: vec!["*".into(), "read".into(), "write".into(), "delete".into(), "admin".into()] },
        RolePermission { role: "admin".into(), document_type: "*".into(), actions: vec!["*".into(), "read".into(), "write".into(), "delete".into(), "admin".into()] },
        RolePermission { role: "org_admin".into(), document_type: "*".into(), actions: vec!["*".into(), "read".into(), "write".into(), "delete".into(), "admin".into()] },

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
        RolePermission { role: "principal".into(), document_type: "Document".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Finance".into(), actions: vec!["read".into(), "write".into(), "payment".into()] },
        RolePermission { role: "principal".into(), document_type: "Integration".into(), actions: vec!["read".into(), "write".into()] },
        RolePermission { role: "principal".into(), document_type: "Dashboard".into(), actions: vec!["read".into()] },
        RolePermission { role: "principal".into(), document_type: "Audit".into(), actions: vec!["read".into()] },
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

pub async fn seed_default_reports<C: ConnectionTrait>(
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
            "organization",
            "MASTER",
            crate::adapters::report_defaults::MASTER_TEMPLATE_HTML,
        ),
        (
            "academic.student-term-report-card",
            "Student Term Report Card",
            "كشف درجات الطالب للفصل الدراسي",
            "academic",
            "student",
            "REPORT",
            crate::adapters::report_defaults::TABULAR_REPORT_HTML,
        ),
        (
            "academic.official-transcript",
            "Official Academic Transcript",
            "السجل الأكاديمي الشامل",
            "academic",
            "student",
            "TRANSCRIPT",
            crate::adapters::report_defaults::TRANSCRIPT_REPORT_HTML,
        ),
        (
            "finance.payment-receipt",
            "Payment Receipt",
            "سند قبض مالي إلكتروني",
            "finance",
            "finance",
            "RECEIPT",
            crate::adapters::report_defaults::PAYMENT_RECEIPT_HTML,
        ),
        (
            "finance.student-invoice",
            "Student Fee Invoice & Statement",
            "فاتورة الرسوم وكشف حساب الطالب",
            "finance",
            "finance",
            "INVOICE",
            crate::adapters::report_defaults::STUDENT_INVOICE_HTML,
        ),
        (
            "identification.student-id-card",
            "Student ID Card",
            "بطاقة هوية الطالب",
            "identification",
            "student",
            "ID_CARD",
            crate::adapters::report_defaults::STUDENT_ID_CARD_HTML,
        ),
        (
            "identification.staff-id-card",
            "Faculty & Staff ID Card",
            "بطاقة هوية الكادر التعليمي والإداري",
            "identification",
            "staff",
            "STAFF_CARD",
            crate::adapters::report_defaults::STAFF_ID_CARD_HTML,
        ),
        (
            "scheduling.class-timetable",
            "Class Timetable",
            "الجدول الدراسي الأسبوعي للفصل",
            "academic",
            "class",
            "SCHEDULE",
            crate::adapters::report_defaults::CLASS_TIMETABLE_HTML,
        ),
        (
            "admission.enrollment-certificate",
            "Certificate of Enrollment",
            "شهادة قيد دراسي وتأكيد انتظام",
            "academic",
            "enrollment",
            "CERTIFICATE",
            crate::adapters::report_defaults::ENROLLMENT_CERTIFICATE_HTML,
        ),
    ];

    for (code, name_en, name_ar, domain, entity_type, doc_type, html_content) in templates {
        // 1. Definition
        let def_id = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_definitions (id, org_id, code, name, name_ar, domain, category, data_provider_key, is_system, is_active, parameters_json, required_permissions, supported_formats, sort_order, version, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8, 1, 1, '[]', '[]', '[\"pdf\", \"html\"]', 0, 1, $9, $10 \
             WHERE NOT EXISTS (SELECT 1 FROM report_definitions WHERE org_id = $2 AND code = $3)",
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
            "INSERT INTO report_templates (id, org_id, code, name, name_ar, document_type, entity_type, status, version, published_version, page_size, orientation, margins_json, document_json, locale, is_system, is_default, created_by, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, 'published', 1, 1, 'A4', 'portrait', '{\"top\":15,\"bottom\":15,\"left\":15,\"right\":15}', $8, 'ar', 1, 1, $9, $10, $11 \
             WHERE NOT EXISTS (SELECT 1 FROM report_templates WHERE org_id = $2 AND code = $3)",
            vec![
                tmpl_id.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(), doc_type.into(), entity_type.into(),
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
            "INSERT INTO report_template_bindings (id, report_definition_id, template_id, locale, format, is_default, created_at) \
             SELECT $1, $2, $3, 'ar', 'pdf', 1, $4 \
             WHERE NOT EXISTS (SELECT 1 FROM report_template_bindings WHERE report_definition_id = $2 AND format = 'pdf')",
            vec![
                binding_id.into(), real_def_id.into(), real_tmpl_id.into(),
                now.to_string().into(),
            ]
        )).await.ok();

        let binding_id_html = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_template_bindings (id, report_definition_id, template_id, locale, format, is_default, created_at) \
             SELECT $1, $2, $3, 'ar', 'html', 1, $4 \
             WHERE NOT EXISTS (SELECT 1 FROM report_template_bindings WHERE report_definition_id = $2 AND format = 'html')",
            vec![
                binding_id_html.into(), real_def_id.into(), real_tmpl_id.into(),
                now.to_string().into(),
            ]
        )).await.ok();
    }

    Ok(())
}

pub async fn seed_initial_audit_logs<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    let initial_events = vec![
        ("organization.bootstrap", "Organization", org_id, serde_json::json!({"action": "Organization created and initialized with core blueprint"})),
        ("iam.admin_initialized", "User", admin_id, serde_json::json!({"role": "super_admin", "status": "active"})),
        ("iam.roles_permissions_seeded", "Role", org_id, serde_json::json!({"roles_count": 15, "scope": "comprehensive"})),
        ("finance.chart_of_accounts_seeded", "Account", org_id, serde_json::json!({"standard": "Educational COA", "status": "active"})),
        ("finance.discount_templates_seeded", "DiscountTemplate", org_id, serde_json::json!({"templates": ["DISC_SIBLING", "DISC_MERIT", "DISC_STAFF", "DISC_EARLY", "DISC_AID"]})),
        ("finance.fee_templates_seeded", "FeeTemplate", org_id, serde_json::json!({"templates": ["FEE_TUITION", "FEE_REG", "FEE_BUS", "FEE_BOOKS", "FEE_TECH", "FEE_ACT"]})),
        ("reporting.default_templates_seeded", "ReportTemplate", org_id, serde_json::json!({"templates_count": 9, "format": "AST/DocForge/Vector"}))
    ];

    for (action, entity_type, entity_id, details) in initial_events {
        let log_id = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_audit_log (id, org_id, actor_id, action, entity_type, entity_id, details, created_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM report_audit_log WHERE org_id = $2 AND action = $4 AND entity_type = $5 AND entity_id = $6)",
            vec![
                log_id.into(), org_id.into(), admin_id.into(),
                action.into(), entity_type.into(), entity_id.into(),
                details.to_string().into(), now.to_string().into(),
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

// ── OnboardingPort Implementation ─────────────────────────────────────────

use orbis_app::ports::onboarding::{OnboardingPort, ValidatedOnboardingData};

pub struct OnboardingAdapterImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl OnboardingPort for OnboardingAdapterImpl {
    async fn persist_onboarding(
        &self,
        data: ValidatedOnboardingData,
    ) -> Result<(), DomainError> {
        let txn = self.db.begin().await
            .map_err(|e| DomainError::Internal(format!("Failed to begin onboarding transaction: {e}")))?;

        let now = Utc::now().to_rfc3339();
        let org_id = data.org_id;

        // Lookup admin user id for audit metadata
        let admin_user_row = txn.query_one(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "SELECT id FROM users WHERE org_id = $1 LIMIT 1",
            vec![org_id.into()]
        )).await.ok().flatten();
        let admin_user_id = admin_user_row.and_then(|r| r.try_get::<Uuid>("", "id").ok()).unwrap_or_else(Uuid::new_v4);

        // 1. Update Organization Profile & Settings
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "UPDATE organizations SET base_currency_code = $1, timezone = $2, locale = $3, settings = $4, updated_by = $5, version = version + 1, updated_at = $6 WHERE id = $7",
            vec![
                data.base_currency_code.clone().into(),
                data.timezone.into(),
                data.locale.into(),
                data.settings.to_string().into(),
                admin_user_id.into(),
                now.clone().into(),
                org_id.into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to update organization settings: {e}")))?;

        // 2. OrgUnits Hierarchy (Campus + Buildings)
        let campus_id = Uuid::new_v4();
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO org_units (id, org_id, parent_id, path, depth, name, name_ar, code, unit_type, data_isolation, children_count, descendant_count, sort_order, is_active, created_by, updated_by, created_at, updated_at) \
             SELECT $1, $2, NULL, 'root', 0, 'Main Campus', 'المقر الرئيسي', 'HQ', 'campus', 'none', 0, 0, 1, 1, $3, $3, $4, $5 \
             WHERE NOT EXISTS (SELECT 1 FROM org_units WHERE org_id = $2 AND depth = 0)",
            vec![campus_id.into(), org_id.into(), admin_user_id.into(), now.clone().into(), now.clone().into()]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to seed root campus: {e}")))?;

        let root_unit_row = txn.query_one(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "SELECT id FROM org_units WHERE org_id = $1 AND depth = 0 LIMIT 1",
            vec![org_id.into()]
        )).await.map_err(|e| DomainError::Internal(e.to_string()))?.ok_or_else(|| DomainError::NotFound("Root campus unit not found".into()))?;
        let real_campus_id: Uuid = root_unit_row.try_get("", "id").unwrap_or(campus_id);

        for b in &data.buildings {
            let parent_id = b.parent_unit_id.unwrap_or(real_campus_id);
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO org_units (id, org_id, parent_id, path, depth, name, name_ar, code, unit_type, data_isolation, children_count, descendant_count, sort_order, is_active, created_by, updated_by, created_at, updated_at) \
                 SELECT $1, $2, $3, 'root.' || $4, 1, $5, $6, $4, 'building', 'none', 0, 0, 1, 1, $7, $7, $8, $9 \
                 WHERE NOT EXISTS (SELECT 1 FROM org_units WHERE org_id = $2 AND code = $4)",
                vec![
                    b.id.into(), org_id.into(), parent_id.into(),
                    b.code.clone().into(),
                    b.name.clone().into(),
                    b.name_ar.clone().unwrap_or_else(|| b.name.clone()).into(),
                    admin_user_id.into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to create building unit: {e}")))?;
        }

        // 3. Rooms
        for rm in &data.rooms {
            let room_unit_id = rm.building_unit_id.unwrap_or(real_campus_id);
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO rooms (id, org_id, name, room_type, capacity, org_unit_id, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8 \
                 WHERE NOT EXISTS (SELECT 1 FROM rooms WHERE org_id = $2 AND name = $3)",
                vec![
                    rm.id.into(), org_id.into(), rm.name.clone().into(),
                    rm.room_type.clone().into(), rm.capacity.into(),
                    room_unit_id.into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert room: {e}")))?;
        }

        // 4. Academic Year
        let normalize_start = |s: &str| -> String {
            if s.len() == 10 && !s.contains('T') { format!("{}T00:00:00Z", s) } else { s.to_string() }
        };
        let normalize_end = |s: &str| -> String {
            if s.len() == 10 && !s.contains('T') { format!("{}T23:59:59Z", s) } else { s.to_string() }
        };

        let year_start = normalize_start(&data.academic_year.start_date);
        let year_end = normalize_end(&data.academic_year.end_date);

        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO academic_years (id, org_id, name, name_ar, start_date, end_date, status, is_locked, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, 'active', 0, $7, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM academic_years WHERE org_id = $2 AND name = $3)",
            vec![
                data.academic_year.id.into(),
                org_id.into(),
                data.academic_year.name.clone().into(),
                data.academic_year.name_ar.clone().unwrap_or_else(|| data.academic_year.name.clone()).into(),
                year_start.into(),
                year_end.into(),
                now.clone().into(),
                now.clone().into(),
            ],
        )).await.map_err(|e| DomainError::Internal(format!("Failed to insert academic year: {e}")))?;

        // 5. Academic Periods
        for p in &data.periods {
            let p_start = normalize_start(&p.start_date);
            let p_end = normalize_end(&p.end_date);
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO academic_periods (id, org_id, year_id, parent_id, name, name_ar, period_type, start_date, end_date, status, is_locked, is_enrollment_open, is_grade_entry_open, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, 'active', 0, 1, 1, $10, $11 \
                 WHERE NOT EXISTS (SELECT 1 FROM academic_periods WHERE org_id = $2 AND year_id = $3 AND name = $5)",
                vec![
                    p.id.into(),
                    org_id.into(),
                    p.year_id.into(),
                    p.parent_id.into(),
                    p.name.clone().into(),
                    p.name_ar.clone().unwrap_or_else(|| p.name.clone()).into(),
                    p.period_type.clone().into(),
                    p_start.into(),
                    p_end.into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert period: {e}")))?;
        }

        let first_period_id = data.periods.first().map(|p| p.id).unwrap_or(data.academic_year.id);

        // 6. Classes
        for c in &data.classes {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO classes (id, org_id, unit_id, period_id, name, name_ar, level, division, is_active, is_deleted, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, 1, 0, $9, $10 \
                 WHERE NOT EXISTS (SELECT 1 FROM classes WHERE org_id = $2 AND period_id = $4 AND name = $5)",
                vec![
                    c.id.into(),
                    org_id.into(),
                    real_campus_id.into(),
                    first_period_id.into(),
                    c.name.clone().into(),
                    c.name_ar.clone().unwrap_or_else(|| c.name.clone()).into(),
                    c.level.clone().into(),
                    c.division.clone().unwrap_or_default().into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert class: {e}")))?;
        }

        // 7. Subjects
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
                    s.name_ar.clone().unwrap_or_else(|| s.name.clone()).into(),
                    (if s.is_core { 1i32 } else { 0i32 }).into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert subject: {e}")))?;
        }

        // 8. Time Slots
        for (slot_idx, ts) in data.time_slots.iter().enumerate() {
            let start_time_str = if ts.start_time.len() == 5 { format!("{}:00", ts.start_time) } else { ts.start_time.clone() };
            let end_time_str = if ts.end_time.len() == 5 { format!("{}:00", ts.end_time) } else { ts.end_time.clone() };

            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO time_slots (id, period_id, day_of_week, slot_index, name, name_ar, start_time, end_time, is_break, break_type, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 \
                 WHERE NOT EXISTS (SELECT 1 FROM time_slots WHERE period_id = $2 AND start_time = $7 AND end_time = $8 AND day_of_week = $3)",
                vec![
                    ts.id.into(),
                    first_period_id.into(),
                    (ts.day_of_week as i16).into(),
                    (slot_idx as i32 + 1).into(),
                    ts.name.clone().into(),
                    ts.name_ar.clone().into(),
                    start_time_str.into(),
                    end_time_str.into(),
                    (if ts.is_break { 1i32 } else { 0i32 }).into(),
                    ts.break_type.clone().into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.ok();
        }

        // 9. Chart of Accounts (COA) initialization if requested
        if data.initialize_chart_of_accounts {
            if let Err(e) = seed_chart_of_accounts(&txn, org_id, admin_user_id, &data.base_currency_code, &now).await {
                info!("COA onboarding initialization note: {}", e);
            }
        }

        // 10. Fee Templates (Only persist if explicitly provided by user in onboarding)
        if !data.fee_templates.is_empty() {
            let rec_row = txn.query_one(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "SELECT id FROM accounts WHERE org_id = $1 AND (code = '110301' OR code = '110401' OR account_type = 'asset') ORDER BY CASE WHEN code = '110301' THEN 0 WHEN code = '110401' THEN 1 ELSE 2 END LIMIT 1",
                vec![org_id.into()]
            )).await.ok().flatten();
            let rec_id: Uuid = match rec_row {
                Some(r) => r.try_get("", "id").unwrap_or_else(|_| Uuid::new_v4()),
                None => Uuid::new_v4(),
            };
            let rev_row = txn.query_one(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "SELECT id FROM accounts WHERE org_id = $1 AND (code = '410301' OR code = '410101' OR account_type = 'revenue') ORDER BY CASE WHEN code IN ('410301', '410101') THEN 0 ELSE 1 END LIMIT 1",
                vec![org_id.into()]
            )).await.ok().flatten();
            let rev_id: Uuid = match rev_row {
                Some(r) => r.try_get("", "id").unwrap_or(rec_id),
                None => rec_id,
            };

            for f in &data.fee_templates {
                let applicable_levels_json = f.applicable_levels.as_ref().map(|l| serde_json::to_string(&serde_json::json!({ "levels": l })).unwrap_or_default());

                txn.execute(Statement::from_sql_and_values(
                    txn.get_database_backend(),
                    "INSERT INTO fee_templates (id, org_id, code, name, name_ar, category, default_amount, currency_code, revenue_account_id, receivable_account_id, applicable_levels, frequency, is_mandatory, is_active, created_at, updated_at) \
                     SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 1, $14, $15 \
                     WHERE NOT EXISTS (SELECT 1 FROM fee_templates WHERE org_id = $2 AND code = $3)",
                    vec![
                        f.id.into(), org_id.into(), f.code.clone().into(),
                        f.name.clone().into(), f.name_ar.clone().into(),
                        f.category.clone().into(), f.default_amount.into(),
                        f.currency_code.clone().into(),
                        rev_id.into(), rec_id.into(),
                        applicable_levels_json.into(),
                        f.frequency.clone().into(),
                        (if f.is_mandatory { 1i32 } else { 0i32 }).into(),
                        now.clone().into(), now.clone().into(),
                    ]
                )).await.ok();
            }
        }

        // 11. Discount Templates
        if data.discount_templates.is_empty() {
            if let Err(e) = seed_discount_templates(&txn, org_id, &now).await {
                info!("Default discount templates onboarding seeding note: {}", e);
            }
        } else {
            for d in &data.discount_templates {
                txn.execute(Statement::from_sql_and_values(
                    txn.get_database_backend(),
                    "INSERT INTO discount_templates (id, org_id, code, name, name_ar, discount_type, value, is_approval_required, is_auto_apply, is_stackable, priority, is_active, created_at, updated_at) \
                     SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 1, $12, $13 \
                     WHERE NOT EXISTS (SELECT 1 FROM discount_templates WHERE org_id = $2 AND code = $3)",
                    vec![
                        d.id.into(), org_id.into(), d.code.clone().into(),
                        d.name.clone().into(), d.name_ar.clone().into(),
                        d.discount_type.clone().into(), d.value.into(),
                        (if d.is_approval_required { 1i32 } else { 0i32 }).into(),
                        (if d.is_auto_apply { 1i32 } else { 0i32 }).into(),
                        (if d.is_stackable { 1i32 } else { 0i32 }).into(),
                        d.priority.into(),
                        now.clone().into(), now.clone().into(),
                    ]
                )).await.ok();
            }
        }

        // 12. Assessment Schemes
        let blueprint = match data.org_type.as_str() {
            "university" => university_blueprint(),
            "institute" | "training_center" => institute_blueprint(),
            _ => school_blueprint(),
        };
        if let Err(e) = seed_assessment_schemes(&txn, org_id, &blueprint, &now).await {
            info!("Assessment schemes seeding note: {}", e);
        }

        // 13. Roles & Granular Permissions Seeding
        if let Err(e) = seed_roles_and_permissions(&txn, org_id, admin_user_id, &now).await {
            info!("Roles & permissions onboarding seeding note: {}", e);
        }

        // 13.5 System Default User Groups Seeding
        if let Err(e) = seed_default_user_groups(&txn, org_id, admin_user_id, &now).await {
            info!("User groups onboarding seeding note: {}", e);
        }

        // 14. Default Reports & Templates Seeding
        if let Err(e) = seed_default_reports(&txn, org_id, &now).await {
            info!("Default reports onboarding seeding note: {}", e);
        }

        // 14.5 Document Requirements Seeding (Per Profile, Stage, & Grade)
        if let Err(e) = seed_document_requirements(&txn, org_id, &data.org_type, &data.classes, &now).await {
            info!("Document requirements onboarding seeding note: {}", e);
        }

        // 15. Record Onboarding Audit Log
        let ob_log_id = Uuid::new_v4();
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO report_audit_log (id, org_id, actor_id, action, entity_type, entity_id, details, created_at) \
             VALUES ($1, $2, $3, 'onboarding.completed', 'Organization', $2, '{\"status\": \"operational\"}', $4)",
            vec![ob_log_id.into(), org_id.into(), admin_user_id.into(), now.clone().into()]
        )).await.ok();

        txn.commit().await
            .map_err(|e| DomainError::Internal(format!("Failed to commit onboarding transaction: {e}")))?;

        Ok(())
    }
}
