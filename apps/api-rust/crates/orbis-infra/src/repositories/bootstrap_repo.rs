//! Bootstrap Repository — bulk organization bootstrap persistence engine.

use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, TransactionTrait};
use orbis_domain::errors::DomainError;
use orbis_app::ports::bootstrap::{BulkBootstrapPort, BootstrapPersistenceResult, ValidatedBootstrapData};
use uuid::Uuid;
use chrono::Utc;
use serde::Deserialize;
use async_trait::async_trait;
use tracing::info;

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

pub async fn seed_currencies<C: ConnectionTrait>(
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

pub async fn seed_chart_of_accounts<C: ConnectionTrait>(
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
            "A4",
            "portrait",
        ),
        (
            "academic.student-term-report-card",
            "Student Term Report Card",
            "كشف درجات الطالب للفصل الدراسي",
            "academic",
            "student",
            "REPORT",
            "A4",
            "portrait",
        ),
        (
            "academic.official-transcript",
            "Official Academic Transcript",
            "السجل الأكاديمي الشامل",
            "academic",
            "student",
            "TRANSCRIPT",
            "A4",
            "portrait",
        ),
        (
            "finance.payment-receipt",
            "Payment Receipt",
            "سند قبض مالي إلكتروني",
            "finance",
            "finance",
            "RECEIPT",
            "A4",
            "portrait",
        ),
    ];

    for (code, name_en, name_ar, cat, entity, r_type, page_sz, page_or) in templates {
        let tid = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO template_catalog (id, org_id, code, name, name_ar, category, entity_type, report_type, page_size, page_orientation, current_version, status, is_system, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 1, 'published', 1, $11, $12 \
             WHERE NOT EXISTS (SELECT 1 FROM template_catalog WHERE org_id = $2 AND code = $3)",
            vec![
                tid.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(), cat.into(),
                entity.into(), r_type.into(), page_sz.into(),
                page_or.into(), now.to_string().into(), now.to_string().into()
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
    let log_id = Uuid::new_v4();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        "INSERT INTO report_audit_log (id, org_id, actor_id, action, entity_type, entity_id, details, created_at) \
         VALUES ($1, $2, $3, 'system.bootstrap', 'Organization', $2, '{\"status\": \"initialized\"}', $4)",
        vec![log_id.into(), org_id.into(), admin_id.into(), now.to_string().into()]
    )).await.ok();
    Ok(())
}

pub struct BootstrapRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl BulkBootstrapPort for BootstrapRepoImpl {
    async fn persist_bootstrap(
        &self,
        data: ValidatedBootstrapData,
    ) -> Result<BootstrapPersistenceResult, DomainError> {
        let txn = self.db.begin().await
            .map_err(|e| DomainError::Internal(format!("Failed to begin bootstrap transaction: {e}")))?;

        let now = Utc::now().to_rfc3339();
        let org_id = data.organization.id;
        let admin_user_id = data.admin_user.id;
        let admin_person_id = data.admin_person.id;

        // 1. Organization
        let mut settings_json = data.org_settings.clone();
        if let Some(obj) = settings_json.as_object_mut() {
            obj.insert("org_type".to_string(), serde_json::Value::String(data.organization.org_type.clone()));
            obj.insert("institution_type".to_string(), serde_json::Value::String(data.organization.org_type.clone()));
        }

        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO organizations (id, name, name_ar, slug, timezone, locale, base_currency_code, is_active, is_verified, is_deleted, is_hijri_enabled, version, settings, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, 1, 1, 0, 0, 1, $8, $9, $10)",
            vec![
                org_id.into(),
                data.organization.name.clone().into(),
                data.organization.name_ar.clone().into(),
                data.organization.slug.clone().into(),
                data.organization.timezone.clone().into(),
                data.organization.locale.clone().into(),
                data.organization.base_currency_code.clone().into(),
                settings_json.to_string().into(),
                now.clone().into(),
                now.clone().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to insert organization: {e}")))?;

        // 2. Admin Person
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO persons (id, org_id, first_name, last_name, display_name, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            vec![
                admin_person_id.into(),
                org_id.into(),
                data.admin_person.first_name.clone().into(),
                data.admin_person.last_name.clone().into(),
                data.admin_person.display_name.clone().into(),
                now.clone().into(),
                now.clone().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to insert admin person: {e}")))?;

        // 3. Admin User
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO users (id, org_id, person_id, email, password_hash, status, is_email_verified, is_phone_verified, is_mfa_enabled, login_count, failed_login_attempts, is_deleted, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, 'active', 1, 0, 0, 0, 0, 0, $6, $7)",
            vec![
                admin_user_id.into(),
                org_id.into(),
                admin_person_id.into(),
                data.admin_user.email.clone().into(),
                data.admin_user.password_hash.clone().into(),
                now.clone().into(),
                now.clone().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to insert admin user: {e}")))?;

        // 4. System Roles
        for role in &data.roles {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO roles (id, org_id, name, name_ar, is_system, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7 \
                 WHERE NOT EXISTS (SELECT 1 FROM roles WHERE id = $1 AND org_id = $2)",
                vec![
                    role.id.clone().into(),
                    org_id.into(),
                    role.name.clone().into(),
                    role.name_ar.clone().into(),
                    (if role.is_system { 1i32 } else { 0i32 }).into(),
                    now.clone().into(),
                    now.clone().into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert role '{}': {e}", role.id)))?;
        }

        // 5. Admin Role Assignment
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO user_role_assignments (id, user_id, role_id, org_id, scope_type, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6)",
            vec![
                data.admin_role_assignment.id.into(),
                admin_user_id.into(),
                data.admin_role_assignment.role_id.clone().into(),
                org_id.into(),
                data.admin_role_assignment.scope_type.clone().into(),
                now.clone().into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to assign super_admin role: {e}")))?;

        // 6. Role Permissions
        for perm in &data.role_permissions {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO role_permissions (id, role_id, org_id, resource, action, condition, created_at) \
                 VALUES ($1, $2, $3, $4, $5, NULL, $6)",
                vec![
                    perm.id.into(),
                    perm.role_id.clone().into(),
                    org_id.into(),
                    perm.resource.clone().into(),
                    perm.action.clone().into(),
                    now.clone().into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert permission: {e}")))?;
        }

        // 7. Currencies
        seed_currencies(&txn, &now).await.ok();

        // 8. Default User Groups
        seed_default_user_groups(&txn, org_id, admin_user_id, &now).await.ok();

        // 9. Chart of Accounts (COA)
        seed_chart_of_accounts(&txn, org_id, admin_user_id, &data.currency.id, &now).await.ok();

        // 10. Discount Templates
        seed_discount_templates(&txn, org_id, &now).await.ok();

        // 11. Period Templates Catalog
        for tpl in &data.period_templates {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO period_templates (id, org_id, name, name_ar, institution_type, structure, is_default, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9 \
                 WHERE NOT EXISTS (SELECT 1 FROM period_templates WHERE org_id = $2 AND name = $3)",
                vec![
                    tpl.id.into(),
                    org_id.into(),
                    tpl.name.clone().into(),
                    tpl.name_ar.clone().into(),
                    tpl.institution_type.clone().into(),
                    tpl.structure.to_string().into(),
                    (if tpl.is_default { 1i32 } else { 0i32 }).into(),
                    now.clone().into(),
                    now.clone().into(),
                ]
            )).await.ok();
        }

        // 12. Assessment Scheme
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO assessment_schemes (id, org_id, name, description, max_score, formula, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM assessment_schemes WHERE org_id = $2)",
            vec![
                data.assessment_scheme.id.into(),
                data.assessment_scheme.org_id.into(),
                data.assessment_scheme.name.clone().into(),
                data.assessment_scheme.description.clone().into(),
                data.assessment_scheme.max_score.into(),
                data.assessment_scheme.formula.to_string().into(),
                now.clone().into(),
                now.clone().into(),
            ]
        )).await.ok();

        // 13. Default Reports AST
        seed_default_reports(&txn, org_id, &now).await.ok();

        // 14. Initial Audit Log
        seed_initial_audit_logs(&txn, org_id, admin_user_id, &now).await.ok();

        txn.commit().await.map_err(|e| DomainError::Internal(format!("Failed to commit bootstrap transaction: {e}")))?;

        info!("Bootstrap persistence committed successfully for org_id={}", org_id);

        Ok(BootstrapPersistenceResult {
            org_id: org_id.to_string(),
            admin_user_id: admin_user_id.to_string(),
            rows_inserted: 0,
        })
    }
}
