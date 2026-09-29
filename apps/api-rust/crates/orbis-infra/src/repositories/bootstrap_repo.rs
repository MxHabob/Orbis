//! Bootstrap Repository — bulk organization bootstrap persistence engine.

use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, TransactionTrait};
use orbis_domain::errors::DomainError;
use orbis_app::ports::bootstrap::{BulkBootstrapPort, BootstrapPersistenceResult, ValidatedBootstrapData, ValidatedUserGroup};
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
    groups: &[ValidatedUserGroup],
    now: &str,
) -> Result<(), DomainError> {
    for grp in groups {
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO groups (id, org_id, name, name_ar, description, is_system, status, created_by, updated_by, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, 1, 'active', $6, $7, $8, $9 \
             WHERE NOT EXISTS (SELECT 1 FROM groups WHERE org_id = $2 AND name = $3)",
            vec![
                grp.id.into(), org_id.into(),
                grp.name.clone().into(), grp.name_ar.clone().into(), grp.description.clone().into(),
                admin_id.into(), admin_id.into(),
                now.to_string().into(), now.to_string().into(),
            ]
        )).await.ok();

        let actual_group_row = db.query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT id FROM groups WHERE org_id = $1 AND name = $2 LIMIT 1",
            vec![org_id.into(), grp.name.clone().into()]
        )).await.ok().flatten();

        if let Some(row) = actual_group_row {
            if let Ok(gid) = row.try_get::<Uuid>("", "id") {
                for perm in &grp.permissions {
                    let perm_id = Uuid::new_v4();
                    db.execute(Statement::from_sql_and_values(
                        db.get_database_backend(),
                        "INSERT INTO group_permissions (id, org_id, group_id, permission, created_at) \
                         SELECT $1, $2, $3, $4, $5 \
                         WHERE NOT EXISTS (SELECT 1 FROM group_permissions WHERE org_id = $2 AND group_id = $3 AND permission = $4)",
                        vec![
                            perm_id.into(), org_id.into(), gid.into(),
                            perm.clone().into(), now.to_string().into(),
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
    admin_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    info!("Seeding default reports catalog (56 templates) for org_id={}", org_id);

    let templates = vec![
        // ── 1. Administration & Official Documents (12 templates) ───────────
        ("administration.master-template", "Master Institutional Template", "الترويسة والتذييل الرسمي للمؤسسة", "system", "organization", "MASTER", "A4", "portrait"),
        ("administration.student-id-card", "Smart Student ID Card", "بطاقة الهوية الطلابية الذكية (مع باركود / QR)", "administration", "student", "ID_CARD", "ID_CARD", "landscape"),
        ("administration.staff-id-card", "Employee Work ID Badge", "بطاقة العمل والتعريف للكادر الوظيفي", "administration", "employee", "ID_CARD", "ID_CARD", "landscape"),
        ("administration.enrollment-certificate", "Certificate of Regular Enrollment", "إفادة وقيد طالب منتظم ومستمر", "administration", "student", "CERTIFICATE", "A4", "portrait"),
        ("administration.to-whom-it-may-concern", "Official Attestation Letter", "شهادة لمن يهمه الأمر موثقة", "administration", "student", "CERTIFICATE", "A4", "portrait"),
        ("administration.student-transfer-release", "Student Transfer Clearance Form", "استمارة إخلاء طرف ونقل طالب", "administration", "student", "REPORT", "A4", "portrait"),
        ("administration.guardian-authorization", "Authorized Guardian Pickup Card", "بطاقة وتفويض استلام ولي الأمر المعتمد", "administration", "student", "ID_CARD", "ID_CARD", "landscape"),
        ("administration.disciplinary-notice", "Disciplinary Undertaking Notice", "إشعار وتعهد بالمخالفة السلوكية", "administration", "student", "REPORT", "A4", "portrait"),
        ("administration.honor-appreciation-certificate", "Certificate of Appreciation & Honor", "شهادة شكر وتقدير وتميز أكاديمي", "administration", "student", "CERTIFICATE", "A4", "landscape"),
        ("administration.graduation-diploma", "Official Graduation Diploma", "وثيقة التخرج والدبلوم الرسمي المعتمد", "administration", "student", "CERTIFICATE", "A4", "landscape"),
        ("administration.bus-transport-pass", "School Bus Transport Pass", "تصريح وبطاقة النقل المدرسي", "administration", "student", "ID_CARD", "ID_CARD", "landscape"),
        ("administration.medical-emergency-card", "Medical Emergency & Health Dossier", "بطاقة الملف الصحي وحالات الطوارئ", "administration", "student", "REPORT", "A4", "portrait"),

        // ── 2. Academics & Scheduling (11 templates) ────────────────────────
        ("academic.student-term-report-card", "Student Term Report Card", "كشف درجات الطالب للفصل الدراسي", "academic", "student", "REPORT", "A4", "portrait"),
        ("academic.official-transcript", "Official Multi-Year Academic Transcript", "السجل الأكاديمي الشامل وكشف العلامات التراكمي", "academic", "student", "TRANSCRIPT", "A4", "portrait"),
        ("academic.class-grade-roster", "Class Section Comprehensive Grade Sheet", "مسير درجات الصف والشعبة الموحد", "academic", "class", "REPORT", "A4", "landscape"),
        ("academic.subject-performance-analysis", "Subject Performance Analysis", "تقرير التحليل الإحصائي لأداء المادة", "academic", "subject", "REPORT", "A4", "portrait"),
        ("academic.honor-roll-roster", "Academic Honor Roll & Top Achievers", "لوحة الشرف وقائمة أوائل الطلاب والمتفوقين", "academic", "class", "REPORT", "A4", "portrait"),
        ("academic.class-daily-timetable", "Weekly Class Schedule", "الجدول الأسبوعي للحصص والدروس للشعبة", "academic", "class", "REPORT", "A4", "landscape"),
        ("academic.teacher-weekly-schedule", "Teacher Weekly Timetable & Slots", "جدول الحصص الأسبوعي للمعلم وتوزيع الأعباء", "academic", "employee", "REPORT", "A4", "landscape"),
        ("academic.room-occupancy-schedule", "Room & Lab Utilization Timetable", "جدول إشغال القاعات والمعامل التدريسية", "academic", "room", "REPORT", "A4", "landscape"),
        ("academic.student-curriculum-progress", "Curriculum Progress Tracking Sheet", "بطاقة متابعة الخطة الدراسية والمقررات", "academic", "student", "REPORT", "A4", "portrait"),
        ("academic.credit-hours-audit", "Credit Hours Degree Audit Report", "تقرير تدقيق الساعات المعتمدة والتخرج", "academic", "student", "REPORT", "A4", "portrait"),
        ("academic.remedial-support-plan", "Remedial & Academic Intervention Plan", "خطة المتابعة والدعم الأكاديمي للطلاب المتعثرين", "academic", "student", "REPORT", "A4", "portrait"),

        // ── 3. Attendance, Truancy & Conduct (7 templates) ──────────────────
        ("attendance.student-monthly-sheet", "Student Monthly Attendance Record", "كشف متابعة الحضور والغياب الشهري للطالب", "attendance", "student", "REPORT", "A4", "portrait"),
        ("attendance.class-daily-roster", "Class Daily Absence Register", "مسير تفقد ورصد الغياب اليومي للشعبة", "attendance", "class", "REPORT", "A4", "landscape"),
        ("attendance.chronic-absence-warning", "Truancy Absence Warning Notice", "إشعار إنذار رسمي بتجاوز نسبة الغياب المقررة", "attendance", "student", "REPORT", "A4", "portrait"),
        ("attendance.monthly-school-analytics", "School-Wide Attendance Analytics", "التقرير الإحصائي العام لنسب الحضور والغياب للمنظمة", "attendance", "organization", "REPORT", "A4", "portrait"),
        ("attendance.teacher-attendance-log", "Faculty Attendance & Punctuality Log", "سجل دوام وحضور الكادر التعليمي والإداري", "attendance", "employee", "REPORT", "A4", "landscape"),
        ("behavior.incident-case-report", "Disciplinary Incident Case Report", "تقرير واقعة ومخالفة سلوكية موثق", "behavior", "student", "REPORT", "A4", "portrait"),
        ("behavior.merit-positive-points", "Positive Merit & Good Conduct Log", "سجل الإشادة والنقاط الإيجابية والتعزيز السلوكي", "behavior", "student", "REPORT", "A4", "portrait"),

        // ── 4. Examinations & Control Operations (9 templates) ───────────────
        ("examinations.student-exam-card", "Student Exam Admission & Seating Card", "بطاقة دخول الاختبار ورقم الجلوس المعتمد", "examinations", "student", "EXAM_CARD", "A5", "portrait"),
        ("examinations.seating-hall-roster", "Exam Hall Seating & Attendance Sheet", "كشف توقيع حضور الطلاب بلجنة الاختبار ومواقع المقاعد", "examinations", "exam_hall", "REPORT", "A4", "landscape"),
        ("examinations.secret-barcode-stickers", "Blind Grading Barcode & Secret Labels", "ملصقات الأرقام السرية والباركود للدفاتر الامتحانية", "examinations", "exam_session", "REPORT", "A4", "portrait"),
        ("examinations.control-master-matrix", "Central Control Secret Mapping Matrix", "كشف المطابقة السري للكنترول المركزي", "examinations", "exam_session", "REPORT", "A4", "landscape"),
        ("examinations.hall-door-roster", "Exam Hall Door Notice Roster", "كشف أسماء الطلاب المعلق على باب اللجنة", "examinations", "exam_hall", "REPORT", "A4", "portrait"),
        ("examinations.proctor-duty-roster", "Exam Invigilator & Proctor Schedule", "جدول ومواعيد توزيع المراقبين والمشرفين على اللجان", "examinations", "exam_session", "REPORT", "A4", "landscape"),
        ("examinations.exam-timetable-broadcast", "Master Examination Timetable Notice", "الجدول العام المعتمد لمواعيد وفترات الامتحانات", "examinations", "exam_session", "REPORT", "A4", "landscape"),
        ("examinations.unattended-student-list", "Exam Absence & Zero Mark Log", "كشف حصر الطلاب المتغيبين عن جلسة الاختبار", "examinations", "exam_session", "REPORT", "A4", "portrait"),
        ("examinations.grading-committee-envelope", "Answer Sheets Grading Envelope Slip", "غلاف ظرف أوراق الإجابة ولجنة التصحيح والمراجعة", "examinations", "exam_session", "REPORT", "A4", "portrait"),

        // ── 5. Finance, Billing & Accounting Ledgers (13 templates) ─────────
        ("finance.payment-receipt", "Official Electronic Payment Receipt", "سند قبض مالي إلكتروني رسمي مزود بـ QR", "finance", "finance", "RECEIPT", "A4", "portrait"),
        ("finance.tax-invoice", "Tuition Fee Tax Invoice", "فاتورة ضريبية معتمدة للرسوم الدراسية", "finance", "finance", "REPORT", "A4", "portrait"),
        ("finance.student-statement-of-account", "Student Financial Statement of Account", "كشف حساب ورصيد الرسوم والمدفوعات للطالب", "finance", "student", "REPORT", "A4", "portrait"),
        ("finance.family-consolidated-statement", "Consolidated Family Financial Ledger", "كشف الحساب المالي المجمع لولي الأمر لجميع الأبناء", "finance", "guardian", "REPORT", "A4", "portrait"),
        ("finance.aged-receivables-aging", "Aged Receivables & Overdue Analysis", "تقرير أعمار الديون والمطالبات المالية غير المسددة", "finance", "finance", "REPORT", "A4", "landscape"),
        ("finance.fee-collection-daily-summary", "Daily Cashier & Collection Summary", "التقرير اليومي لحركة الصندوق والمقبوضات المالية", "finance", "finance", "REPORT", "A4", "portrait"),
        ("finance.scholarship-discount-audit", "Scholarship & Fee Concession Audit", "كشف تدقيق الخصومات والمنح الدراسية الممنوحة", "finance", "finance", "REPORT", "A4", "landscape"),
        ("finance.trial-balance-report", "General Ledger Trial Balance Report", "ميزان المراجعة المحاسبي العام للأرصدة والمجاميع", "finance", "finance", "REPORT", "A4", "landscape"),
        ("finance.income-statement", "Income Statement / Profit & Loss", "تقرير قائمة الدخل والأرباح والخسائر التشغيلية", "finance", "finance", "REPORT", "A4", "portrait"),
        ("finance.balance-sheet", "Balance Sheet & Financial Position", "الميزانية العمومية وقائمة المركز المالي", "finance", "finance", "REPORT", "A4", "portrait"),
        ("finance.journal-voucher", "General Journal Entry Voucher", "سند قيد اليومية العامة المعتمد", "finance", "finance", "REPORT", "A4", "portrait"),
        ("finance.fee-structure-catalog", "Approved Tuition Fee Catalog", "دليل هيكل الرسوم والأقساط السنوية المعتمدة للمنظمة", "finance", "finance", "REPORT", "A4", "portrait"),
        ("finance.refund-credit-note", "Credit Note & Fee Refund Slip", "إشعار دائن وسند استرداد مالي للرسوم", "finance", "finance", "REPORT", "A4", "portrait"),

        // ── 6. HR & Employee Affairs (4 templates) ──────────────────────────
        ("hr.employee-contract-summary", "Employment Contract Summary", "ملخص عقد العمل والبيانات الوظيفية للكادر", "hr", "employee", "REPORT", "A4", "portrait"),
        ("hr.salary-pay-slip", "Monthly Salary Pay Slip", "مسير وقسيمة الراتب والمستحقات الشهرية", "hr", "employee", "REPORT", "A4", "portrait"),
        ("hr.teaching-load-allocation", "Faculty Teaching Quota Allocation", "كشف توزيع النصاب التعليمي والحصص للكادر التدريسي", "hr", "organization", "REPORT", "A4", "landscape"),
        ("hr.staff-directory-roster", "Institutional Staff & Faculty Directory", "دليل بيانات وهواتف الكادر الوظيفي والتعليمي", "hr", "organization", "REPORT", "A4", "portrait"),
    ];

    for (code, name_en, name_ar, _cat, entity, r_type, page_sz, page_or) in templates {
        let tid = Uuid::new_v4();
        let default_doc = serde_json::json!({
            "version": "2.0",
            "meta": {
                "id": tid.to_string(),
                "name": name_en,
                "nameAr": name_ar,
                "documentType": r_type,
                "entityType": entity,
                "locale": "ar",
                "pageSetup": {
                    "size": page_sz,
                    "orientation": page_or,
                    "margins": { "top": 10, "bottom": 10, "left": 10, "right": 10 },
                    "unit": "mm"
                }
            },
            "variables": {},
            "assets": {},
            "pages": [{ "type": "Page", "id": "p1", "children": [] }]
        }).to_string();

        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_templates (id, org_id, code, name, name_ar, description, description_ar, document_type, entity_type, master_template_id, status, version, published_version, published_at, published_by, page_size, orientation, margins_json, document_json, locale, tags, is_system, is_default, created_by, deleted_at, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $4, $5, $6, $7, NULL, 'published', 1, 1, $8, $9, $10, $11, '{\"top\":10,\"bottom\":10,\"left\":10,\"right\":10}', $12, 'ar', '[]', 1, 1, $9, NULL, $8, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM report_templates WHERE org_id = $2 AND code = $3)",
            vec![
                tid.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(),
                r_type.into(), entity.into(),
                now.to_string().into(), admin_id.into(),
                page_sz.into(), page_or.into(),
                default_doc.into(),
            ]
        )).await.ok();
    }

    Ok(())
}

pub async fn seed_default_report_definitions<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    info!("Seeding default business report definitions (40 reports) for org_id={}", org_id);

    let report_defs = vec![
        // ── 1. Academic & Grading Reports (8 reports) ────────────────────────
        ("rep.academic.student-grades-summary", "Student Grades Comprehensive Summary", "كشف درجات الطلاب الإجمالي والمعدلات", "academic", "grades", "Comprehensive grade breakdown by section", "كشف تفصيلي للدرجات والمعدلات التراكمية", "academic.grades_summary"),
        ("rep.academic.official-transcript", "Official Multi-Year Academic Transcript", "السجل الأكاديمي الشامل وكشف العلامات", "academic", "transcripts", "Cumulative transcript of all completed terms", "سجل تراكمي لجميع الفصول والمواد المجتازة", "academic.transcript"),
        ("rep.academic.subject-performance", "Subject Performance & Grade Distribution", "التحليل الإحصائي لأداء ومخرجات المادة", "academic", "analytics", "Statistical distribution of pass/fail rates", "توزيع التقديرات والنسب المئوية للنجاح والرسوب", "academic.subject_performance"),
        ("rep.academic.class-roster", "Class Section Student Roster", "القائمة الاسمية لطلاب الصف والشعبة", "academic", "classes", "Official student enrollment roster by classroom", "قائمة الطلاب المسجلين بالصف والشعبة", "academic.class_roster"),
        ("rep.academic.timetable", "Master Weekly Timetable Matrix", "الجدول الدراسي الأسبوعي العام", "academic", "scheduling", "Complete class and teacher period allocation", "توزيع الحصص والأنصبة الأسبوعية", "academic.timetable"),
        ("rep.academic.honor-roll", "Academic Honor Roll & Top Achievers", "لوحة الشرف وقائمة أوائل الطلاب", "academic", "awards", "Highest GPA students per cohort", "الطلاب الحاصلون على أعلى المعدلات التراكمية", "academic.honor_roll"),
        ("rep.academic.remedial-list", "Academic Remedial & Underperforming Students", "كشف الطلاب المتعثرين أكاديمياً", "academic", "analytics", "Students requiring academic support intervention", "الطلاب الحاصلون على درجات دون حد النجاح", "academic.remedial"),
        ("rep.academic.credit-audit", "Degree Audit & Graduation Eligibility", "تقرير تدقيق الساعات المعتمدة والتخرج", "academic", "transcripts", "Validation of graduation degree requirements", "تدقيق استيفاء متطلبات التخرج والساعات", "academic.credit_audit"),

        // ── 2. Attendance & Behavior Reports (6 reports) ─────────────────────
        ("rep.attendance.daily-absence", "Daily Class Absence Register", "تقرير الغياب والتفقد اليومي للشعب", "attendance", "daily", "Daily absenteeism log across all classrooms", "حصر حالات الغياب اليومي بعذر وبدون عذر", "attendance.daily_absence"),
        ("rep.attendance.monthly-summary", "Monthly Cumulative Attendance Analytics", "التقرير التراكمي الشهري للغياب والمواظبة", "attendance", "monthly", "Monthly attendance percentages and trends", "نسب ونسب الغياب والمواظبة الشهرية", "attendance.monthly_summary"),
        ("rep.attendance.chronic-truancy", "Chronic Absence & Truancy Warning Log", "تقرير الطلاب المتجاوزين لنسبة الغياب المسموحة", "attendance", "alerts", "Students exceeding institutional absence thresholds", "الطلاب المستحقين لإنذارات الحرمان والغياب", "attendance.chronic_truancy"),
        ("rep.attendance.teacher-log", "Faculty Attendance & Punctuality Register", "سجل دوام وحضور الكادر التعليمي والإداري", "attendance", "staff", "Teacher check-in/out and lesson attendance", "بيان حضور الحصص وتوقيع الكادر", "attendance.teacher_log"),
        ("rep.behavior.incident-log", "Disciplinary Incidents & Violations Register", "سجل الوقائع والمخالفات السلوكية الموثقة", "behavior", "incidents", "Logged behavioural violations and actions taken", "المخالفات المرصودة والإجراءات المعتمدة", "behavior.incident_log"),
        ("rep.behavior.positive-merits", "Positive Merits & Student Excellence Log", "سجل النقاط الإيجابية والتعزيز السلوكي", "behavior", "merits", "Recorded merits and commendable student conduct", "نقاط التميز والمشاركات الإيجابية", "behavior.positive_merits"),

        // ── 3. Examination & Control Reports (6 reports) ─────────────────────
        ("rep.exam.seating-distribution", "Exam Hall Seating Distribution Matrix", "جدول توزيع المقاعد وأرقام الجلوس باللجان", "examinations", "seating", "Assigned desk and hall seat layout for exams", "توزيع الطلاب على القاعات والمقاعد", "exam.seating_distribution"),
        ("rep.exam.proctor-roster", "Exam Proctors & Invigilators Duty Roster", "جدول مواعيد وتوزيع المراقبين والمشرفين", "examinations", "proctors", "Staff supervisory duties during exam sessions", "نوبات مراقبة الاختبارات وتوزيع اللجان", "exam.proctor_roster"),
        ("rep.exam.secret-mapping", "Control Secret Numbers Mapping Matrix", "كشف مطابقة الأرقام السرية للكنترول المركزي", "examinations", "control", "Confidential mapping between secret codes and students", "كشف الربط السري لأعمال التصحيح والكنترول", "exam.secret_mapping"),
        ("rep.exam.unattended-log", "Exam Absence & Zero-Grade Log", "كشف حصر المتغيبين عن جلسات الاختبارات", "examinations", "control", "Record of missed exam sessions and excused absences", "الطلاب المتغيبين عن أداء الامتحانات", "exam.unattended_log"),
        ("rep.exam.hall-attendance", "Exam Hall Student Attendance Sheet", "كشف توقيع حضور الطلاب بلجنة الاختبار", "examinations", "halls", "Physical attendance sign-in sheet for examination halls", "استمارة توقيع الطلاب داخل لجان الامتحانات", "exam.hall_attendance"),
        ("rep.exam.result-master-sheet", "Master Examination Results Broad Sheet", "مسير النتائج العامة المعتمدة للامتحانات", "examinations", "control", "Final approved examination scores before publication", "المسير العام لدرجات الكنترول النهائي", "exam.result_master"),

        // ── 4. Financial & Accounting Reports (11 reports) ───────────────────
        ("rep.finance.trial-balance", "General Ledger Trial Balance Report", "ميزان المراجعة المحاسبي العام للأرصدة والمجاميع", "finance", "accounting", "Summary of all ledger accounts debit and credit balances", "ميزان مراجعة الحسابات بالأرصدة والمجاميع", "finance.trial_balance"),
        ("rep.finance.income-statement", "Income Statement / Profit & Loss Report", "تقرير قائمة الدخل والأرباح والخسائر التشغيلية", "finance", "accounting", "Institutional operational revenues and expenses", "الإيرادات والمصروفات وصافي الفائض/العجز", "finance.income_statement"),
        ("rep.finance.balance-sheet", "Balance Sheet & Financial Position Statement", "الميزانية العمومية وقائمة المركز المالي", "finance", "accounting", "Institutional assets, liabilities, and equity", "الأصول والخصوم وحقوق الملكية", "finance.balance_sheet"),
        ("rep.finance.general-ledger", "General Ledger Account Detail Statement", "كشف حساب الأستاذ العام وتفاصيل الحركات", "finance", "accounting", "Detailed transaction entries for specified accounts", "حركات المدين والدائن والرصيد التراكمي", "finance.general_ledger"),
        ("rep.finance.journal-entries", "General Journal Entries Audit Register", "دفتر اليومية العامة وسجل قيود اليومية", "finance", "accounting", "Chronological list of all posted journal vouchers", "سجل القيود المحاسبية وتفاصيل بنودها", "finance.journal_entries"),
        ("rep.finance.student-statement", "Student Financial Statement of Account", "كشف حساب الرسوم والمدفوعات للطالب", "finance", "billing", "Billed tuition fees, payments received, and net balance", "الرسوم المقيدة والدفعات المسددة والرصيد المتبقي", "finance.student_statement"),
        ("rep.finance.family-statement", "Consolidated Family Financial Ledger", "كشف الحساب المالي المجمع لولي الأمر", "finance", "billing", "Consolidated account statement for all registered siblings", "حساب الرسوم والدفعات لجميع الأبناء", "finance.family_statement"),
        ("rep.finance.aged-receivables", "Aged Receivables & Overdue Tuition Analysis", "تقرير أعمار الديون والمطالبات المالية غير المسددة", "finance", "billing", "Aging buckets (30, 60, 90+ days) for overdue fees", "تحليل المتأخرات والمديونيات حسب فترات الاستحقاق", "finance.aged_receivables"),
        ("rep.finance.daily-collection", "Daily Cashier & Collection Summary", "التقرير اليومي لحركة الصندوق والمقبوضات", "finance", "cash", "Daily cash, bank transfer, and card receipts", "المقبوضات النقدية والبنكية وسندات القبض", "finance.daily_collection"),
        ("rep.finance.scholarships-audit", "Scholarships & Fee Concessions Audit", "كشف تدقيق المنح والخصومات الدراسية", "finance", "discounts", "Applied discounts, sibling concessions, and scholarships", "الخصومات المعتمدة والمنح والإعفاءات", "finance.scholarships_audit"),
        ("rep.finance.tax-summary", "Value Added Tax (VAT) Summary Report", "إقرار وملخص ضريبة القيمة المضافة", "finance", "tax", "Calculated tax on fees, invoices, and refunds", "ضريبة المخرجات والمدخلات وضريبة القيمة المضافة", "finance.tax_summary"),

        // ── 5. Admissions, Students & Capacity (5 reports) ───────────────────
        ("rep.admission.enrolled-students", "Enrolled Students Comprehensive Roster", "كشف الطلاب المقيدين والمستجدين بالمؤسسة", "students", "admissions", "List of all active and newly registered students", "قائمة الطلاب الفعليين والمسجلين الجدد", "students.enrolled_roster"),
        ("rep.admission.capacity-utilization", "Classroom Capacity Utilization Analysis", "تقرير الطاقة الاستيعابية للشعب والفصول", "students", "capacity", "Classroom student density and available seats", "نسب إشغال الفصول والمقاعد المتاحة", "students.capacity_analysis"),
        ("rep.admission.transfers-log", "Student Section Transfers & Mobility Log", "سجل التحويلات والنقل بين الفصول والشعب", "students", "transfers", "Audit of section transfer approvals and dates", "حركات النقل والتحويل بين الشعب الدراسية", "students.transfers_log"),
        ("rep.admission.documents-compliance", "Student Documents & Archival Compliance", "تقرير استيفاء ونواقص الوثائق للطلاب", "students", "documents", "Missing required admissions documents checklist", "حالة تسليم الوثائق الرسمية والمستندات الناقصة", "students.documents_compliance"),
        ("rep.admission.guardians-directory", "Guardians Contact & Emergency Directory", "دليل بيانات وهواتف أولياء الأمور وجهات الطوارئ", "students", "guardians", "Parent contact directory and authorized pickups", "بيانات التواصل مع أولياء الأمور والطوارئ", "students.guardians_directory"),

        // ── 6. HR & Employee Affairs (4 reports) ─────────────────────────────
        ("rep.hr.faculty-directory", "Institutional Faculty & Staff Directory", "الدليل الشامل للكادر التعليمي والإداري", "employees", "staff", "Comprehensive employee roster and departments", "بيانات الكادر، التخصصات، والأقسام", "hr.faculty_directory"),
        ("rep.hr.teaching-load", "Faculty Teaching Quota & Quota Allocation", "تقرير توزيع الأنصبة والحصص التدريسية", "employees", "workload", "Weekly teaching load and credit hours by instructor", "أنصبة المعلمين وساعات التدريس الأسبوعية", "hr.teaching_load"),
        ("rep.hr.payroll-summary", "Monthly Payroll & Salary Disbursement Sheet", "مسير الرواتب والمستحقات الشهرية للكادر", "employees", "payroll", "Monthly salary payroll register and allowances", "مسير الرواتب والبدلات والاستقطاعات", "hr.payroll_summary"),
        ("rep.hr.contracts-expiration", "Employee Contracts Expiration & Renewals", "تقرير مواعيد تجديد العقود الوظيفية", "employees", "contracts", "Upcoming contract renewals and probation periods", "مواعيد انتهاء العقود وفترات التجربة", "hr.contracts_expiration"),
    ];

    for (code, name_en, name_ar, domain, category, desc_en, desc_ar, provider) in report_defs {
        let rid = Uuid::new_v4();
        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_definitions (id, org_id, code, name, name_ar, domain, category, description, description_ar, parameters_json, required_permissions, supported_formats, data_provider_key, is_system, is_active, sort_order, version, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, '[]', '[]', '[\"PDF\",\"CSV\",\"XLSX\",\"HTML\"]', $10, 1, 1, 0, 1, $11, $12 \
             WHERE NOT EXISTS (SELECT 1 FROM report_definitions WHERE org_id = $2 AND code = $3)",
            vec![
                rid.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(), domain.into(), category.into(),
                desc_en.into(), desc_ar.into(), provider.into(),
                now.to_string().into(), now.to_string().into()
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
        seed_default_user_groups(&txn, org_id, admin_user_id, &data.user_groups, &now).await.ok();

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

        // 13. Default Visual Templates (DocForge AST Layouts in report_templates)
        seed_default_reports(&txn, org_id, admin_user_id, &now).await.ok();

        // 14. Default Business Report Definitions (Executable queries in report_definitions)
        seed_default_report_definitions(&txn, org_id, &now).await.ok();

        // 15. Initial Audit Log
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
