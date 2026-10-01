//! Onboarding Service — Orchestrates post-bootstrap institution configuration.
//!
//! This is an application-level orchestration service that evaluates the real
//! backend database state against the active institution blueprint to determine:
//! - Complete DAG of setup tasks
//! - Completed, in-progress, blocked, and optional tasks
//! - Task dependencies and blocking reasons
//! - Overall and category progress percentages
//! - Recommended next administrative action
//! - Optional idempotent default scaffolding provisioning
//!
//! Invariants:
//! - Idempotent, resumable, crash-safe
//! - 100% derived from real database state (never fakes completion)
//! - Never directly bypasses domain rules
//! - Blueprint-aware (School vs University vs Institute)

use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

use orbis_domain::entities::organization::InstitutionType;
use orbis_domain::entities::academic_year::AcademicYear;
use orbis_domain::entities::academic_period::{AcademicPeriod, PeriodType, PeriodKind, AcademicPeriodState};
use orbis_domain::entities::subject::Subject;
use orbis_domain::entities::class::Class;
use orbis_domain::entities::assessment_scheme::{AssessmentScheme, AssessmentComponent, AssessmentLevel, InstitutionScope};
use orbis_domain::entities::fee_template::{FeeTemplate, FeeFrequency};
use orbis_domain::entities::discount_template::{DiscountTemplate, DiscountType};
use orbis_domain::entities::org_unit::OrgUnit;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::{SubjectFilter, ClassFilter, ClassSubjectFilter};
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::{
    OrganizationRepository, OrgUnitRepository, AcademicYearRepository,
    AcademicPeriodRepository, SubjectRepository, ClassRepository,
    ClassSubjectRepository, AssessmentSchemeRepository, SettingsRepository,
    AccountRepository, FeeTemplateRepository, DiscountTemplateRepository,
    RoomRepository, DocumentRequirementRepository, IntegrationRepository,
};
use crate::ports::reporting::ReportTemplateRepository;
use crate::ports::iam::PermissionCheckerPort;

// ═══════════════════════════════════════════════════════════════
// Models & Data Transfer Objects (DTOs)
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingTaskCategory {
    Organization,
    Calendar,
    Academics,
    Assessment,
    Finance,
    Students,
    Templates,
    Integrations,
}

impl OnboardingTaskCategory {
    pub fn label_en(&self) -> &'static str {
        match self {
            Self::Organization => "Organization & Identity",
            Self::Calendar => "Academic Calendar",
            Self::Academics => "Academic Structure",
            Self::Assessment => "Grading & Policy",
            Self::Finance => "Finance & Fees",
            Self::Students => "Student Readiness",
            Self::Templates => "Reports & Documents",
            Self::Integrations => "Integrations & Alerts",
        }
    }

    pub fn label_ar(&self) -> &'static str {
        match self {
            Self::Organization => "الهوية والمنشأة",
            Self::Calendar => "التقويم الدراسي",
            Self::Academics => "الهيكل الأكاديمي",
            Self::Assessment => "التقييم والسياسات",
            Self::Finance => "المالية والرسوم",
            Self::Students => "جاهزية الطلاب",
            Self::Templates => "النماذج والشهادات",
            Self::Integrations => "الربط والإشعارات",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingTaskStatus {
    NotStarted,
    InProgress,
    Completed,
    Blocked,
    Optional,
    NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupTask {
    pub id: String,
    pub category: OnboardingTaskCategory,
    pub title: String,
    pub title_ar: String,
    pub description: String,
    pub description_ar: String,
    pub is_required: bool,
    pub status: OnboardingTaskStatus,
    pub dependencies: Vec<String>,
    pub progress: u8,
    pub route: String,
    pub action_type: String,
    pub institution_types: Vec<String>,
    pub required_permissions: Vec<String>,
    pub completion_condition: String,
    pub blocking_reason: Option<String>,
    pub blocking_reason_ar: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryProgress {
    pub category: OnboardingTaskCategory,
    pub label: String,
    pub label_ar: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub progress_percentage: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingStateResponse {
    pub org_id: Uuid,
    pub org_name: String,
    pub institution_type: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub required_tasks: usize,
    pub required_completed: usize,
    pub optional_tasks: usize,
    pub optional_completed: usize,
    pub blocked_tasks: usize,
    pub progress_percentage: u8,
    pub required_progress_percentage: u8,
    pub is_institution_ready: bool,
    pub tasks: Vec<SetupTask>,
    pub categories: Vec<CategoryProgress>,
    pub recommended_next_task: Option<SetupTask>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionDefaultsRequest {
    pub academic_year_name: Option<String>,
    pub academic_year_name_ar: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub provision_stages_and_grades: Option<bool>,
    pub provision_subjects: Option<bool>,
    pub provision_assessment_scheme: Option<bool>,
    pub provision_tuition_fee_template: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionDefaultsResult {
    pub success: bool,
    pub created_items: Vec<String>,
    pub state: OnboardingStateResponse,
}

// ═══════════════════════════════════════════════════════════════
// Onboarding Service
// ═══════════════════════════════════════════════════════════════

pub struct OnboardingService {
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub year_repo: Arc<dyn AcademicYearRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub class_repo: Arc<dyn ClassRepository>,
    pub class_subject_repo: Arc<dyn ClassSubjectRepository>,
    pub assessment_scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
    pub account_repo: Arc<dyn AccountRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub discount_template_repo: Arc<dyn DiscountTemplateRepository>,
    pub room_repo: Arc<dyn RoomRepository>,
    pub doc_req_repo: Arc<dyn DocumentRequirementRepository>,
    pub template_repo: Arc<dyn ReportTemplateRepository>,
    pub integration_repo: Arc<dyn IntegrationRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl OnboardingService {
    pub fn new(
        org_repo: Arc<dyn OrganizationRepository>,
        org_unit_repo: Arc<dyn OrgUnitRepository>,
        year_repo: Arc<dyn AcademicYearRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
        subject_repo: Arc<dyn SubjectRepository>,
        class_repo: Arc<dyn ClassRepository>,
        class_subject_repo: Arc<dyn ClassSubjectRepository>,
        assessment_scheme_repo: Arc<dyn AssessmentSchemeRepository>,
        settings_repo: Arc<dyn SettingsRepository>,
        account_repo: Arc<dyn AccountRepository>,
        fee_template_repo: Arc<dyn FeeTemplateRepository>,
        discount_template_repo: Arc<dyn DiscountTemplateRepository>,
        room_repo: Arc<dyn RoomRepository>,
        doc_req_repo: Arc<dyn DocumentRequirementRepository>,
        template_repo: Arc<dyn ReportTemplateRepository>,
        integration_repo: Arc<dyn IntegrationRepository>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            org_repo,
            org_unit_repo,
            year_repo,
            period_repo,
            subject_repo,
            class_repo,
            class_subject_repo,
            assessment_scheme_repo,
            settings_repo,
            account_repo,
            fee_template_repo,
            discount_template_repo,
            room_repo,
            doc_req_repo,
            template_repo,
            integration_repo,
            permission_checker,
        }
    }

    // ─────────────────────────────────────────────────────────
    // Get Complete Onboarding State
    // ─────────────────────────────────────────────────────────

    /// Dynamically evaluates real database state to produce the full DAG onboarding state.
    pub async fn get_state(&self, org_id: Uuid) -> Result<OnboardingStateResponse, DomainError> {
        let org = self.org_repo.find_by_id(org_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Organization {} not found", org_id)))?;

        let institution_type_str = org.metadata.as_ref()
            .and_then(|m| m.get("institution_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("school");
        let inst_type = InstitutionType::from_str_loose(institution_type_str);

        // ── 1. Gather Real Backend Metrics ─────────────────────
        let units = self.org_unit_repo.find_by_org(org_id).await?;
        let rooms = self.room_repo.find_by_org(org_id).await?;
        let years = self.year_repo.find_by_org(org_id).await?;
        let active_year = years.iter().find(|y| y.status == "active" || !y.is_locked);
        let periods = if let Some(ay) = active_year {
            self.period_repo.find_by_year(ay.id).await?
        } else {
            self.period_repo.find_by_org(org_id).await?
        };

        let subjects_res = self.subject_repo.list(org_id, SubjectFilter::default()).await?;
        let classes_res = self.class_repo.list(org_id, ClassFilter::default()).await?;
        let class_subjects_res = self.class_subject_repo.list(org_id, ClassSubjectFilter::default()).await?;
        let schemes = self.assessment_scheme_repo.find_by_org(org_id).await?;
        let settings = self.settings_repo.get_settings(org_id).await.unwrap_or_default();
        let accounts = self.account_repo.find_by_org(org_id).await?;
        let fee_templates = self.fee_template_repo.find_by_org(org_id).await?;
        let discount_templates = self.discount_template_repo.find_by_org(org_id).await?;
        let doc_reqs = self.doc_req_repo.find_by_org(org_id).await?;
        let templates = self.template_repo.list_by_org(org_id, None, None, None).await.unwrap_or_default();
        let integrations = self.integration_repo.find_configs_by_org(org_id).await?;

        // ── 2. Raw Task Completion & Progress Checks ───────────
        let mut raw_tasks = Vec::new();

        // Task 1: Organization Profile
        let has_org_profile = !org.name.trim().is_empty()
            && !org.timezone.trim().is_empty()
            && !org.locale.trim().is_empty()
            && (org.name_ar.is_some() || org.primary_color.is_some() || org.code.is_some());
        raw_tasks.push((
            "org_profile",
            OnboardingTaskCategory::Organization,
            "Organization Profile & Branding",
            "الملف التعريفي والهوية المؤسسية",
            "Configure legal names, logo, timezone, locale, and primary contact information.",
            "إعداد بيانات المنشأة الرسمية، الاسم بالعربية والإنجليزية، الشعار، والمنطقة الزمنية.",
            true,
            vec![],
            if has_org_profile { 100 } else { 50 },
            has_org_profile,
            "/settings",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "Organization name, timezone, locale, and branding verified.",
            None,
        ));

        // Task 2: Campuses & Branches (Org Units)
        let has_org_units = !units.is_empty();
        raw_tasks.push((
            "org_units",
            OnboardingTaskCategory::Organization,
            "Campuses & Branch Structure",
            "الفروع والمقرات الإدارية",
            "Create main campus and institutional administrative units or branches.",
            "تعريف المقر الرئيسي والفروع التعليمية والإدارية التابعة للمؤسسة.",
            true,
            vec!["org_profile".into()],
            if has_org_units { 100 } else { 0 },
            has_org_units,
            "/settings/branches",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["org_unit:write".into()],
            "At least one active branch or organizational unit created.",
            Some(("Requires organization profile completion.", "يتطلب إكمال الملف التعريفي أولاً.")),
        ));

        // Task 3: Facilities & Rooms (Optional)
        let has_rooms = !rooms.is_empty();
        raw_tasks.push((
            "facilities_rooms",
            OnboardingTaskCategory::Organization,
            "Rooms & Educational Facilities",
            "القاعات والمرافق التعليمية",
            "Configure classrooms, laboratories, auditoriums, and capacity bounds.",
            "إضافة الفصول، المعامل، القاعات الدراسية، والطاقة الاستيعابية لكل مرفق.",
            false,
            vec!["org_units".into()],
            if has_rooms { 100 } else { 0 },
            has_rooms,
            "/settings/rooms",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["org_unit:write".into()],
            "Educational rooms or facilities defined.",
            Some(("Requires at least one branch or campus.", "يتطلب وجود فرع أو مقر واحد على الأقل.")),
        ));

        // Task 4: Academic Year
        let has_academic_year = active_year.is_some() || !years.is_empty();
        raw_tasks.push((
            "academic_year",
            OnboardingTaskCategory::Calendar,
            "Academic Year & Dates",
            "العام الدراسي والتواريخ",
            "Define the active academic year, opening and closing dates, and official calendar status.",
            "تحديد العام الدراسي الفعال، تواريخ البداية والنهاية، وحالة التقويم الرسمي.",
            true,
            vec!["org_units".into()],
            if has_academic_year { 100 } else { 0 },
            has_academic_year,
            "/settings/periods",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["period:write".into()],
            "An active academic year exists with valid date range.",
            Some(("Requires branch setup before opening academic year.", "يتطلب إعداد الفروع أولاً.")),
        ));

        // Task 5: Academic Periods / Terms & Monthly Periods
        let min_periods_required = match inst_type {
            InstitutionType::School => 2,
            InstitutionType::University => 2,
            InstitutionType::Institute => 1,
        };
        let has_periods = periods.len() >= min_periods_required;
        let period_progress = if has_periods {
            100
        } else if !periods.is_empty() {
            50
        } else {
            0
        };
        let period_desc_en = match inst_type {
            InstitutionType::School => "Configure Term 1 & Term 2, with monthly evaluation periods and exam sessions.",
            InstitutionType::University => "Configure Semester 1 (Fall), Semester 2 (Spring), and enrollment windows.",
            InstitutionType::Institute => "Configure modular training cycles and cohort evaluation periods.",
        };
        let period_desc_ar = match inst_type {
            InstitutionType::School => "تهيئة الفصول الدراسية (الأول والثاني) مع الفترات التقويمية الشهرية وجلسات الاختبارات.",
            InstitutionType::University => "تهيئة الفصول الجامعية (الخريف والربيع) وفترات التسجيل والحذف والإضافة.",
            InstitutionType::Institute => "تهيئة الدورات التدريبية والدفعات وفترات التقييم المهني.",
        };
        raw_tasks.push((
            "academic_periods",
            OnboardingTaskCategory::Calendar,
            "Terms & Evaluation Periods",
            "الفصول والفترات التقويمية",
            period_desc_en,
            period_desc_ar,
            true,
            vec!["academic_year".into()],
            period_progress,
            has_periods,
            "/settings/periods",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["period:write".into()],
            "Configured terms and periods under the active academic year.",
            Some(("Requires an active Academic Year.", "يتطلب إنشاء وتفعيل العام الدراسي أولاً.")),
        ));

        // Task 6: Academic Subjects / Courses
        let has_subjects = subjects_res.total_count > 0;
        raw_tasks.push((
            "academic_subjects",
            OnboardingTaskCategory::Academics,
            "Curriculum Subjects & Courses",
            "المواد والمساقات الدراسية",
            "Add curriculum subjects with code, credit/weekly hours, and classification.",
            "إضافة المواد الدراسية مع الرمز، عدد الحصص/الساعات الأسبوعية، ونوع المادة.",
            true,
            vec!["org_units".into()],
            if has_subjects { 100 } else { 0 },
            has_subjects,
            "/settings/subjects",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["subject:write".into()],
            "At least one subject or course defined.",
            Some(("Requires branch configuration.", "يتطلب وجود فرع تعليمي واحد على الأقل.")),
        ));

        // Task 7: Classes / Sections
        let has_classes = classes_res.total_count > 0;
        raw_tasks.push((
            "academic_classes",
            OnboardingTaskCategory::Academics,
            "Classes, Sections & Cohorts",
            "الفصول والشعب الدراسية",
            "Configure classroom sections, homeroom divisions, grade level associations, and capacity limits.",
            "إنشاء الشعب الدراسية والفصول، وتحديد المرحلة والصف والطاقة الاستيعابية.",
            true,
            vec!["academic_periods".into(), "academic_subjects".into()],
            if has_classes { 100 } else { 0 },
            has_classes,
            "/academics/classes",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["class:write".into()],
            "At least one class or section created for the active period.",
            Some(("Requires academic periods and subjects to be created first.", "يتطلب إعداد الفصول الدراسية والمواد أولاً.")),
        ));

        // Task 8: Teacher Subject Assignments (Optional prerequisite for timetable)
        let has_teaching = class_subjects_res.total_count > 0;
        raw_tasks.push((
            "teaching_assignments",
            OnboardingTaskCategory::Academics,
            "Teaching Assignments",
            "إسناد المواد للمعلمين",
            "Link teachers to classes and subjects for gradebook and scheduling.",
            "تعيين المعلمين للمواد والشعب الدراسية لتمكين رصد الدرجات والجدول المدرسي.",
            false,
            vec!["academic_classes".into(), "academic_subjects".into()],
            if has_teaching { 100 } else { 0 },
            has_teaching,
            "/academics/classes",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["class:write".into()],
            "Class-subject teacher assignments configured.",
            Some(("Requires classes and subjects to be configured.", "يتطلب إنشاء الفصول والمواد أولاً.")),
        ));

        // Task 9: Assessment Scheme & Grading Policy
        let has_schemes = !schemes.is_empty();
        let scheme_desc_en = match inst_type {
            InstitutionType::School => "Configure 100-point monthly scheme (Homework, Oral, Attendance, Written Exam) and term weighting.",
            InstitutionType::University => "Configure course assessment breakdown (Midterm, Projects, Final Exam) and GPA grading scale.",
            InstitutionType::Institute => "Configure competency-based practical assessment and pass/fail thresholds.",
        };
        let scheme_desc_ar = match inst_type {
            InstitutionType::School => "تهيئة كشف المحصلة الشهرية من 100 (واجبات، شفوي، مواظبة، اختبار تحريري) ووزن الفصل.",
            InstitutionType::University => "تهيئة توزيع درجات المقرر (نصفي، مشاريع، نهائي) وسلم المعدل التراكمي GPA.",
            InstitutionType::Institute => "تهيئة كفايات التقييم العملي والمهني ومعايير الاجتياز.",
        };
        raw_tasks.push((
            "assessment_schemes",
            OnboardingTaskCategory::Assessment,
            "Assessment & Grading Schemes",
            "خطط التقييم وتوزيع الدرجات",
            scheme_desc_en,
            scheme_desc_ar,
            true,
            vec!["academic_subjects".into()],
            if has_schemes { 100 } else { 0 },
            has_schemes,
            "/settings/assessment-schemes",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["assessment:write".into()],
            "At least one assessment scheme created with component weights.",
            Some(("Requires subjects to be created first.", "يتطلب إنشاء المواد الدراسية أولاً.")),
        ));

        // Task 10: Academic Policy & Pass/Fail Criteria
        let has_academic_policy = !settings.academic.default_grading_system.trim().is_empty();
        raw_tasks.push((
            "academic_policy",
            OnboardingTaskCategory::Assessment,
            "Pass/Fail & Academic Policies",
            "سياسات النجاح والرسوب والتفوق",
            "Set minimum passing percentage, honors thresholds, ranking scope, and rounding precision.",
            "تحديد نسبة النجاح الدنيا، شروط مرتبة الشرف، نطاق الترتيب، ودقة التقريب الرياضي.",
            true,
            vec!["assessment_schemes".into()],
            if has_academic_policy { 100 } else { 0 },
            has_academic_policy,
            "/settings/academic-policy",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "Academic policy thresholds configured.",
            Some(("Requires assessment schemes to be configured.", "يتطلب إعداد خطط التقييم أولاً.")),
        ));

        // Task 11: Chart of Accounts & Ledger
        let has_coa = accounts.iter().any(|a| a.code == "110101" || a.code == "410101" || a.code == "1" || a.code == "1101" || a.code == "110301" || a.code == "1000");
        raw_tasks.push((
            "chart_of_accounts",
            OnboardingTaskCategory::Finance,
            "Chart of Accounts & Ledger",
            "دليل الحسابات ودفتر الأستاذ",
            "Review asset, liability, revenue, discount, and cashbox accounts for double-entry postings.",
            "مراجعة حسابات الأصول، الإيرادات، الخصومات، والصناديق لضمان صحة القيود المحاسبية.",
            true,
            vec!["org_units".into()],
            if has_coa { 100 } else { 0 },
            has_coa,
            "/finance/accounts",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["finance:write".into()],
            "Standard double-entry chart of accounts verified.",
            Some(("Requires branch setup.", "يتطلب إعداد الفروع أولاً.")),
        ));

        // Task 12: Fee Templates & Tuition Rates
        let has_fees = !fee_templates.is_empty();
        raw_tasks.push((
            "fee_templates",
            OnboardingTaskCategory::Finance,
            "Fee Templates & Tuition Rates",
            "قوالب الرسوم الدراسية والخدمات",
            "Configure tuition, transportation, and service fee templates with grade-level targeting and installment rules.",
            "إنشاء قوالب الرسوم (تعليم، نقل، خدمات) مع تحديد الصفوف المستهدفة وسياسات الأقساط.",
            true,
            vec!["chart_of_accounts".into(), "academic_year".into()],
            if has_fees { 100 } else { 0 },
            has_fees,
            "/settings/fee-templates",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["finance:write".into()],
            "At least one active fee template configured with revenue account link.",
            Some(("Requires chart of accounts and academic year.", "يتطلب دليل الحسابات والعام الدراسي.")),
        ));

        // Task 13: Discount Templates & Scholarships (Optional)
        let has_discounts = !discount_templates.is_empty();
        raw_tasks.push((
            "discount_templates",
            OnboardingTaskCategory::Finance,
            "Discounts & Scholarships",
            "الخصومات والمنح الدراسية",
            "Configure sibling discounts, staff concessions, and academic scholarship rules.",
            "تهيئة خصومات الأخوة، أبناء العاملين، ومنح التفوق الأكاديمي.",
            false,
            vec!["fee_templates".into()],
            if has_discounts { 100 } else { 0 },
            has_discounts,
            "/settings/discount-templates",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["finance:write".into()],
            "Discount templates configured.",
            Some(("Requires fee templates to be created first.", "يتطلب إنشاء قوالب الرسوم أولاً.")),
        ));

        // Task 14: Student Registration & Admission Policy
        let has_registration_policy = settings.enrollment.max_class_size_default > 0;
        raw_tasks.push((
            "student_registration_policy",
            OnboardingTaskCategory::Students,
            "Admission & Enrollment Rules",
            "شروط القبول وسياسات التسجيل",
            "Set default class capacity, waitlist rules, late enrollment permissions, and numbering sequence.",
            "تحديد الحد الأقصى للطلاب، سياسة قائمة الانتظار، وشروط القبول المتأخر وتوليد الأرقام الأكاديمية.",
            true,
            vec!["academic_classes".into(), "fee_templates".into()],
            if has_registration_policy { 100 } else { 0 },
            has_registration_policy,
            "/settings/registration",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "Student registration and enrollment rules configured.",
            Some(("Requires classes and fee templates to be configured.", "يتطلب إعداد الفصول والرسوم أولاً.")),
        ));

        // Task 15: Document Requirements (Optional)
        let has_doc_reqs = !doc_reqs.is_empty();
        raw_tasks.push((
            "document_requirements",
            OnboardingTaskCategory::Students,
            "Required Student Documents",
            "المستندات والوثائق المطلوبة",
            "Define mandatory student admission documents (birth certificate, national ID, vaccine card, transfer files).",
            "تحديد الوثائق الإلزامية لملف الطالب (شهادة الميلاد، الهوية، بطاقة التطعيم، ملف النقل).",
            false,
            vec!["student_registration_policy".into()],
            if has_doc_reqs { 100 } else { 0 },
            has_doc_reqs,
            "/settings/archive",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["document:write".into()],
            "Document requirements or archive folders configured.",
            Some(("Requires registration policy configuration.", "يتطلب إعداد سياسة التسجيل أولاً.")),
        ));

        // Task 16: Certificate & Report Templates (Optional)
        let has_templates = !templates.is_empty();
        raw_tasks.push((
            "system_templates",
            OnboardingTaskCategory::Templates,
            "Certificates & Report Templates",
            "نماذج الشهادات والبطاقات الذكية",
            "Review and customize official student report card, ID card, and graduation certificate visual templates.",
            "مراجعة وتخصيص نماذج الشهادات الرسمية، بطاقات الهوية الذكية، وكشوف الدرجات المعتمدة.",
            false,
            vec!["org_profile".into()],
            if has_templates { 100 } else { 0 },
            has_templates,
            "/templates",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["template:write".into()],
            "Visual templates available and verified.",
            Some(("Requires organization profile.", "يتطلب الملف التعريفي للمنشأة.")),
        ));

        // Task 17: Integrations & Notifications (Optional)
        let has_integrations = !integrations.is_empty();
        raw_tasks.push((
            "integrations_setup",
            OnboardingTaskCategory::Integrations,
            "Integrations & Gateways",
            "بوابات الربط والإشعارات",
            "Configure Email SMTP, SMS, WhatsApp, Electronic Payment Gateway, and DocForge connections.",
            "ربط مزودي البريد الإلكتروني، الرسائل النصية، الواتساب، وبوابات الدفع الإلكتروني ومحرك المستندات.",
            false,
            vec!["org_profile".into()],
            if has_integrations { 100 } else { 0 },
            has_integrations,
            "/settings/notifications",
            "navigate",
            vec!["school".into(), "university".into(), "institute".into()],
            vec!["organization:write".into()],
            "External communication channels or integrations configured.",
            Some(("Requires organization profile.", "يتطلب الملف التعريفي للمنشأة.")),
        ));

        // ── 3. Resolve Dependencies & Directed Statuses ────────
        let mut completed_task_ids = std::collections::HashSet::new();
        for (id, _, _, _, _, _, _, _, _, is_completed, _, _, _, _, _, _) in &raw_tasks {
            if *is_completed {
                completed_task_ids.insert(id.to_string());
            }
        }

        let mut tasks = Vec::new();
        let mut blocked_count = 0;
        let mut required_count = 0;
        let mut required_completed = 0;
        let mut optional_count = 0;
        let mut optional_completed = 0;
        let mut total_progress_sum: u32 = 0;
        let mut required_progress_sum: u32 = 0;

        for (
            id,
            category,
            title,
            title_ar,
            desc,
            desc_ar,
            is_required,
            deps,
            raw_progress,
            is_completed,
            route,
            action_type,
            inst_types,
            perms,
            condition,
            blocking_hint,
        ) in raw_tasks {
            let id_str = id.to_string();

            // Check if all prerequisites are fulfilled
            let mut is_blocked = false;
            let mut blocking_reason = None;
            let mut blocking_reason_ar = None;

            for dep in &deps {
                if !completed_task_ids.contains(dep) {
                    is_blocked = true;
                    if let Some((hint_en, hint_ar)) = blocking_hint {
                        blocking_reason = Some(hint_en.to_string());
                        blocking_reason_ar = Some(hint_ar.to_string());
                    } else {
                        blocking_reason = Some(format!("Requires prerequisite task '{}'", dep));
                        blocking_reason_ar = Some(format!("يتطلب إنجاز المهمة السابقة '{}'", dep));
                    }
                    break;
                }
            }

            let status = if is_completed {
                OnboardingTaskStatus::Completed
            } else if is_blocked {
                blocked_count += 1;
                OnboardingTaskStatus::Blocked
            } else if raw_progress > 0 {
                OnboardingTaskStatus::InProgress
            } else if is_required {
                OnboardingTaskStatus::NotStarted
            } else {
                OnboardingTaskStatus::Optional
            };

            let effective_progress = if is_completed { 100 } else { raw_progress };

            if is_required {
                required_count += 1;
                if is_completed {
                    required_completed += 1;
                }
                required_progress_sum += effective_progress as u32;
            } else {
                optional_count += 1;
                if is_completed {
                    optional_completed += 1;
                }
            }

            total_progress_sum += effective_progress as u32;

            tasks.push(SetupTask {
                id: id_str,
                category,
                title: title.to_string(),
                title_ar: title_ar.to_string(),
                description: desc.to_string(),
                description_ar: desc_ar.to_string(),
                is_required,
                status,
                dependencies: deps,
                progress: effective_progress,
                route: route.to_string(),
                action_type: action_type.to_string(),
                institution_types: inst_types,
                required_permissions: perms,
                completion_condition: condition.to_string(),
                blocking_reason,
                blocking_reason_ar,
                metadata: None,
            });
        }

        // ── 4. Calculate Category Progress Summaries ────────────
        let all_categories = [
            OnboardingTaskCategory::Organization,
            OnboardingTaskCategory::Calendar,
            OnboardingTaskCategory::Academics,
            OnboardingTaskCategory::Assessment,
            OnboardingTaskCategory::Finance,
            OnboardingTaskCategory::Students,
            OnboardingTaskCategory::Templates,
            OnboardingTaskCategory::Integrations,
        ];

        let mut categories = Vec::new();
        for cat in all_categories {
            let cat_tasks: Vec<_> = tasks.iter().filter(|t| t.category == cat).collect();
            let total = cat_tasks.len();
            let completed = cat_tasks.iter().filter(|t| t.status == OnboardingTaskStatus::Completed).count();
            let prog = if total == 0 {
                100
            } else {
                let sum: u32 = cat_tasks.iter().map(|t| t.progress as u32).sum();
                (sum / total as u32).min(100) as u8
            };

            categories.push(CategoryProgress {
                category: cat,
                label: cat.label_en().to_string(),
                label_ar: cat.label_ar().to_string(),
                total_tasks: total,
                completed_tasks: completed,
                progress_percentage: prog,
            });
        }

        // ── 5. Global Aggregations & Next Recommended Action ───
        let total_tasks = tasks.len();
        let completed_tasks = required_completed + optional_completed;
        let progress_percentage = if total_tasks == 0 {
            100
        } else {
            (total_progress_sum / total_tasks as u32).min(100) as u8
        };
        let required_progress_percentage = if required_count == 0 {
            100
        } else {
            (required_progress_sum / required_count as u32).min(100) as u8
        };
        let is_institution_ready = required_completed >= required_count;

        // Find the first unblocked, uncompleted required task (or first optional)
        let recommended_next_task = tasks.iter()
            .find(|t| t.is_required && (t.status == OnboardingTaskStatus::NotStarted || t.status == OnboardingTaskStatus::InProgress))
            .cloned()
            .or_else(|| {
                tasks.iter()
                    .find(|t| t.status == OnboardingTaskStatus::Optional || t.status == OnboardingTaskStatus::InProgress)
                    .cloned()
            });

        let mut warnings = Vec::new();
        if !is_institution_ready {
            warnings.push(format!(
                "Institution is in setup mode: {} of {} required tasks remaining.",
                required_count - required_completed,
                required_count
            ));
        }

        Ok(OnboardingStateResponse {
            org_id,
            org_name: org.name,
            institution_type: institution_type_str.to_string(),
            total_tasks,
            completed_tasks,
            required_tasks: required_count,
            required_completed,
            optional_tasks: optional_count,
            optional_completed,
            blocked_tasks: blocked_count,
            progress_percentage,
            required_progress_percentage,
            is_institution_ready,
            tasks,
            categories,
            recommended_next_task,
            warnings,
        })
    }

    // ─────────────────────────────────────────────────────────
    // Optional Helper: Provision Blueprint Defaults Scaffolding
    // ─────────────────────────────────────────────────────────

    /// Idempotently provisions standard blueprint defaults (Year, Terms, Stages, Classes, Schemes, Fees)
    /// for administrators who request accelerated setup.
    pub async fn provision_blueprint_defaults(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        req: ProvisionDefaultsRequest,
    ) -> Result<ProvisionDefaultsResult, DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;

        let org = self.org_repo.find_by_id(org_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Organization {} not found", org_id)))?;
        let institution_type_str = org.metadata.as_ref()
            .and_then(|m| m.get("institution_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("school");
        let inst_type = InstitutionType::from_str_loose(institution_type_str);

        let now = Utc::now();
        let mut created_items = Vec::new();

        // 1. Ensure at least one OrgUnit exists
        let units = self.org_unit_repo.find_by_org(org_id).await?;
        let main_unit_id = if units.is_empty() {
            let unit_id = Uuid::new_v4();
            let unit = OrgUnit {
                id: unit_id,
                org_id,
                parent_id: None,
                path: "/MAIN/".to_string(),
                depth: 1,
                name: "Main Campus".to_string(),
                name_ar: Some("المقر الرئيسي".to_string()),
                code: Some("MAIN".to_string()),
                unit_type: "campus".to_string(),
                head_id: None,
                capacity: Some(500),
                room_number: None,
                floor: None,
                building: None,
                settings: None,
                local_policies: None,
                data_isolation: "shared".to_string(),
                children_count: 0,
                descendant_count: 0,
                sort_order: Some(1),
                is_active: true,
                created_by: None,
                updated_by: None,
                created_at: now,
                updated_at: now,
            };
            self.org_unit_repo.save(&unit).await?;
            created_items.push("Main Campus (OrgUnit)".to_string());
            unit_id
        } else {
            units[0].id
        };

        // 2. Provision Academic Year & Blueprint Periods if missing
        let existing_years = self.year_repo.find_by_org(org_id).await?;
        let active_year_id = if existing_years.is_empty() {
            let year_id = Uuid::new_v4();
            let current_year_num = chrono::Datelike::year(&now);
            let next_year_num = current_year_num + 1;
            let year_name = req.academic_year_name.unwrap_or_else(|| format!("{}-{}", current_year_num, next_year_num));
            let year_name_ar = req.academic_year_name_ar.unwrap_or_else(|| format!("{}-{}", current_year_num, next_year_num));

            let start_date = req.start_date.unwrap_or_else(|| {
                chrono::NaiveDate::from_ymd_opt(current_year_num, 9, 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc()
            });
            let end_date = req.end_date.unwrap_or_else(|| {
                chrono::NaiveDate::from_ymd_opt(next_year_num, 6, 30).unwrap().and_hms_opt(23, 59, 59).unwrap().and_utc()
            });

            let year = AcademicYear {
                id: year_id,
                org_id,
                name: year_name,
                name_ar: Some(year_name_ar),
                start_date,
                end_date,
                hijri_start: None,
                hijri_end: None,
                status: "active".to_string(),
                is_locked: false,
                locked_at: None,
                locked_by: None,
                lock_reason: None,
                current_period_id: None,
                archived_at: None,
                archived_by: None,
                notes: None,
                created_at: now,
                updated_at: now,
            };
            self.year_repo.save(&year).await?;
            created_items.push("Active Academic Year".to_string());

            // Create Terms and Monthly Periods for School
            match inst_type {
                InstitutionType::School => {
                    // Term 1 (Sep - Jan)
                    let t1_id = Uuid::new_v4();
                    let t1_start = start_date;
                    let t1_end = chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 1, 31).unwrap().and_hms_opt(23, 59, 59).unwrap().and_utc();
                    let t1 = AcademicPeriod {
                        id: t1_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "First Term".to_string(),
                        name_ar: Some("الفصل الدراسي الأول".to_string()),
                        code: Some("T1".to_string()),
                        period_type: PeriodType::Semester,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::ActiveTeaching,
                        start_date: t1_start,
                        end_date: t1_end,
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: true,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: true,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(1),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&t1).await?;

                    // Term 2 (Feb - Jun)
                    let t2_id = Uuid::new_v4();
                    let t2_start = chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 2, 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc();
                    let t2_end = end_date;
                    let t2 = AcademicPeriod {
                        id: t2_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "Second Term".to_string(),
                        name_ar: Some("الفصل الدراسي الثاني".to_string()),
                        code: Some("T2".to_string()),
                        period_type: PeriodType::Semester,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::Planning,
                        start_date: t2_start,
                        end_date: t2_end,
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: false,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: false,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(2),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&t2).await?;
                    created_items.push("Terms (Term 1 & Term 2)".to_string());
                }
                InstitutionType::University => {
                    // Semester 1 (Fall)
                    let sem1_id = Uuid::new_v4();
                    let sem1 = AcademicPeriod {
                        id: sem1_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "Fall Semester".to_string(),
                        name_ar: Some("فصل الخريف".to_string()),
                        code: Some("FALL".to_string()),
                        period_type: PeriodType::Semester,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::ActiveTeaching,
                        start_date,
                        end_date: chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 1, 31).unwrap().and_hms_opt(23, 59, 59).unwrap().and_utc(),
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: true,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: true,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(1),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&sem1).await?;

                    // Semester 2 (Spring)
                    let sem2_id = Uuid::new_v4();
                    let sem2 = AcademicPeriod {
                        id: sem2_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "Spring Semester".to_string(),
                        name_ar: Some("فصل الربيع".to_string()),
                        code: Some("SPRING".to_string()),
                        period_type: PeriodType::Semester,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::Planning,
                        start_date: chrono::NaiveDate::from_ymd_opt(current_year_num + 1, 2, 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc(),
                        end_date,
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: false,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: false,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(2),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&sem2).await?;
                    created_items.push("Semesters (Fall & Spring)".to_string());
                }
                InstitutionType::Institute => {
                    let cohort_id = Uuid::new_v4();
                    let cohort = AcademicPeriod {
                        id: cohort_id,
                        org_id,
                        year_id,
                        parent_id: None,
                        name: "Cohort 1".to_string(),
                        name_ar: Some("الدفعة التدريبية الأولى".to_string()),
                        code: Some("C1".to_string()),
                        period_type: PeriodType::Module,
                        period_kind: PeriodKind::Teaching,
                        state: AcademicPeriodState::ActiveTeaching,
                        start_date,
                        end_date,
                        hijri_start: None,
                        hijri_end: None,
                        enrollment_starts_at: None,
                        enrollment_ends_at: None,
                        is_enrollment_open: true,
                        grade_entry_starts_at: None,
                        grade_entry_ends_at: None,
                        is_grade_entry_open: true,
                        exam_starts_at: None,
                        exam_ends_at: None,
                        is_locked: false,
                        locked_at: None,
                        locked_by: None,
                        sort_order: Some(1),
                        metadata: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.period_repo.save(&cohort).await?;
                    created_items.push("Training Cohort Cycle".to_string());
                }
            }
            year_id
        } else {
            existing_years[0].id
        };

        // 3. Provision Core Subjects if requested & empty
        let existing_subjects = self.subject_repo.list(org_id, SubjectFilter::default()).await?;
        if existing_subjects.total_count == 0 && req.provision_subjects.unwrap_or(true) {
            let sample_subjects = match inst_type {
                InstitutionType::School => vec![
                    ("MATH", "Mathematics", Some("الرياضيات"), true, 5),
                    ("ARAB", "Arabic Language", Some("اللغة العربية"), true, 5),
                    ("ENG", "English Language", Some("اللغة الإنجليزية"), true, 4),
                    ("SCI", "General Science", Some("العلوم العامة"), true, 4),
                    ("ISLAM", "Islamic Studies", Some("التربية الإسلامية"), true, 3),
                    ("SOC", "Social Studies", Some("الدراسات الاجتماعية"), false, 2),
                    ("CS", "Computer & Digital Skills", Some("الحاسب والمهارات الرقمية"), false, 2),
                ],
                InstitutionType::University => vec![
                    ("CS101", "Introduction to Computer Science", Some("مقدمة في علوم الحاسب"), true, 3),
                    ("ENG101", "Academic Writing & Research", Some("الكتابة الأكاديمية والبحث"), true, 3),
                    ("MATH101", "Calculus I", Some("حساب التفاضل والتكامل 1"), true, 4),
                    ("PHYS101", "General Physics", Some("الفيزياء العامة"), true, 4),
                ],
                InstitutionType::Institute => vec![
                    ("TECH101", "Applied Technical Foundations", Some("الأسس التقنية التطبيقية"), true, 4),
                    ("COMM101", "Professional Communication", Some("مهارات الاتصال المهني"), true, 2),
                ],
            };

            for (code, name, name_ar, is_core, hours) in sample_subjects {
                let sub = Subject {
                    id: Uuid::new_v4(),
                    org_id,
                    unit_id: Some(main_unit_id),
                    code: code.to_string(),
                    name: name.to_string(),
                    name_ar: name_ar.map(|s| s.to_string()),
                    description: None,
                    is_core,
                    is_elective: !is_core,
                    credit_hours: Some(Decimal::from(hours)),
                    theory_hours: Some(Decimal::from(hours)),
                    practical_hours: None,
                    is_lab_required: false,
                    max_students_per_class: Some(30),
                    color: None,
                    icon: None,
                    sort_order: None,
                    is_active: true,
                    created_at: now,
                    updated_at: now,
                };
                self.subject_repo.save(&sub).await?;
            }
            created_items.push("Core Curriculum Subjects".to_string());
        }

        // 4. Provision Starter Classes if requested & empty
        let existing_classes = self.class_repo.list(org_id, ClassFilter::default()).await?;
        if existing_classes.total_count == 0 && req.provision_stages_and_grades.unwrap_or(true) {
            let active_period = self.period_repo.find_by_year(active_year_id).await?.into_iter().next();
            if let Some(period) = active_period {
                let sample_classes = match inst_type {
                    InstitutionType::School => vec![
                        ("Grade 1 - Section A", Some("الصف الأول - شعبة أ"), "Grade 1", "A"),
                        ("Grade 1 - Section B", Some("الصف الأول - شعبة ب"), "Grade 1", "B"),
                        ("Grade 2 - Section A", Some("الصف الثاني - شعبة أ"), "Grade 2", "A"),
                        ("Grade 3 - Section A", Some("الصف الثالث - شعبة أ"), "Grade 3", "A"),
                    ],
                    InstitutionType::University => vec![
                        ("CS101 - Section 01", Some("مقدمة علوم الحاسب - شعبة 01"), "Year 1", "01"),
                        ("MATH101 - Section 01", Some("تفاضل 1 - شعبة 01"), "Year 1", "01"),
                    ],
                    InstitutionType::Institute => vec![
                        ("Cohort 1 - Morning Track", Some("الدفعة 1 - المسار الصباحي"), "Level 1", "A"),
                    ],
                };

                for (name, name_ar, level, div) in sample_classes {
                    let cls = Class {
                        id: Uuid::new_v4(),
                        org_id,
                        unit_id: main_unit_id,
                        period_id: period.id,
                        name: name.to_string(),
                        name_ar: name_ar.map(|s| s.to_string()),
                        level: level.to_string(),
                        division: Some(div.to_string()),
                        homeroom_teacher_id: None,
                        capacity: Some(30),
                        current_students: Some(0),
                        room_id: None,
                        color: None,
                        is_active: true,
                        is_deleted: false,
                        deleted_at: None,
                        created_at: now,
                        updated_at: now,
                    };
                    self.class_repo.save(&cls).await?;
                }
                created_items.push("Starter Classrooms & Sections".to_string());
            }
        }

        // 5. Provision Standard 100-pt Assessment Scheme if requested & empty
        let existing_schemes = self.assessment_scheme_repo.find_by_org(org_id).await?;
        if existing_schemes.is_empty() && req.provision_assessment_scheme.unwrap_or(true) {
            let scheme = match inst_type {
                InstitutionType::School => {
                    AssessmentScheme {
                        id: Uuid::new_v4(),
                        org_id,
                        name: "K-12 Monthly 100-Point Evaluation Scheme".to_string(),
                        name_ar: Some("لائحة التقييم الشهري المدرسي من 100 درجة".to_string()),
                        description: Some("Standard school scheme: 20 homework, 20 oral, 20 attendance, 40 written test".to_string()),
                        max_score: Decimal::from(100),
                        formula: None,
                        level: AssessmentLevel::MonthlyPeriod,
                        institution_scope: InstitutionScope::School,
                        components: vec![
                            AssessmentComponent {
                                code: "HW".to_string(),
                                name: "Homework & Assignments".to_string(),
                                name_ar: Some("الواجبات والتكليفات".to_string()),
                                max_score: Decimal::from(20),
                                weight: Decimal::from(20),
                                sort_order: 1,
                                is_required: true,
                            },
                            AssessmentComponent {
                                code: "ORAL".to_string(),
                                name: "Oral & Interaction".to_string(),
                                name_ar: Some("الشفهي والمشاركة".to_string()),
                                max_score: Decimal::from(20),
                                weight: Decimal::from(20),
                                sort_order: 2,
                                is_required: true,
                            },
                            AssessmentComponent {
                                code: "ATT".to_string(),
                                name: "Attendance & Conduct".to_string(),
                                name_ar: Some("المواظبة والسلوك".to_string()),
                                max_score: Decimal::from(20),
                                weight: Decimal::from(20),
                                sort_order: 3,
                                is_required: true,
                            },
                            AssessmentComponent {
                                code: "EXAM_M".to_string(),
                                name: "Written Assessment".to_string(),
                                name_ar: Some("الاختبار التحريري الشهري".to_string()),
                                max_score: Decimal::from(40),
                                weight: Decimal::from(40),
                                sort_order: 4,
                                is_required: true,
                            },
                        ],
                        is_default: true,
                        is_system: true,
                        created_at: now,
                        updated_at: now,
                    }
                }
                _ => {
                    AssessmentScheme {
                        id: Uuid::new_v4(),
                        org_id,
                        name: "Standard Course Evaluation Scheme".to_string(),
                        name_ar: Some("لائحة تقييم المقرر القياسية".to_string()),
                        description: Some("50 coursework / midterm + 50 final examination".to_string()),
                        max_score: Decimal::from(100),
                        formula: None,
                        level: AssessmentLevel::Coursework,
                        institution_scope: InstitutionScope::All,
                        components: vec![
                            AssessmentComponent {
                                code: "COURSEWORK".to_string(),
                                name: "Coursework & Midterm".to_string(),
                                name_ar: Some("الأعمال الفصلية والاختبار النصفي".to_string()),
                                max_score: Decimal::from(50),
                                weight: Decimal::from(50),
                                sort_order: 1,
                                is_required: true,
                            },
                            AssessmentComponent {
                                code: "FINAL".to_string(),
                                name: "Final Examination".to_string(),
                                name_ar: Some("الاختبار النهائي".to_string()),
                                max_score: Decimal::from(50),
                                weight: Decimal::from(50),
                                sort_order: 2,
                                is_required: true,
                            },
                        ],
                        is_default: true,
                        is_system: true,
                        created_at: now,
                        updated_at: now,
                    }
                }
            };
            self.assessment_scheme_repo.save(&scheme).await?;
            created_items.push("Standard Assessment Scheme (100-Point Formula)".to_string());
        }

        // 6. Provision Tuition Fee Template if requested & empty
        let existing_fees = self.fee_template_repo.find_by_org(org_id).await?;
        if existing_fees.is_empty() && req.provision_tuition_fee_template.unwrap_or(true) {
            let accounts = self.account_repo.find_by_org(org_id).await?;
            let rev_acct = accounts.iter().find(|a| a.code == "410101" || a.code == "4101" || a.code == "4000" || a.account_type == orbis_domain::entities::account::AccountType::Revenue).map(|a| a.id).unwrap_or_else(Uuid::new_v4);
            let rec_acct = accounts.iter().find(|a| a.code == "110301" || a.code == "1103" || a.code == "1120" || (a.account_type == orbis_domain::entities::account::AccountType::Asset && a.code.starts_with("1103"))).map(|a| a.id).unwrap_or_else(Uuid::new_v4);

            let fee_tmpl = FeeTemplate {
                id: Uuid::new_v4(),
                org_id,
                code: "TUITION-ANNUAL".to_string(),
                name: "Annual Academic Tuition".to_string(),
                name_ar: Some("الرسوم الدراسية السنوية".to_string()),
                category: "tuition".to_string(),
                default_amount: Decimal::from(5000),
                currency_code: org.base_currency_code.clone().unwrap_or_else(|| "SAR".to_string()),
                revenue_account_id: rev_acct,
                receivable_account_id: rec_acct,
                applicable_levels: None, // Applies to all unless filtered
                applicable_study_modes: None,
                frequency: FeeFrequency::Annual,
                is_mandatory: true,
                is_active: true,
                created_at: now,
                updated_at: now,
            };
            self.fee_template_repo.save(&fee_tmpl).await?;
            created_items.push("Tuition Fee Template".to_string());
        }

        // 7. Provision Standard Discount Templates if empty
        let existing_discounts = self.discount_template_repo.find_by_org(org_id).await?;
        if existing_discounts.is_empty() {
            let sibling_disc = DiscountTemplate {
                id: Uuid::new_v4(),
                org_id,
                code: "DISC-SIBLING".to_string(),
                name: "Sibling Discount (10%)".to_string(),
                name_ar: Some("خصم الأخوة (10%)".to_string()),
                discount_type: DiscountType::Percentage,
                value: Decimal::from(10),
                eligibility_rules: Some(serde_json::json!({ "type": "sibling", "account_code": "410301" })),
                is_approval_required: false,
                is_auto_apply: true,
                is_stackable: false,
                priority: 1,
                is_active: true,
                created_at: now,
                updated_at: now,
            };
            self.discount_template_repo.save(&sibling_disc).await?;

            let merit_disc = DiscountTemplate {
                id: Uuid::new_v4(),
                org_id,
                code: "DISC-MERIT".to_string(),
                name: "Academic Excellence Scholarship (20%)".to_string(),
                name_ar: Some("منحة التفوق الأكاديمي (20%)".to_string()),
                discount_type: DiscountType::Percentage,
                value: Decimal::from(20),
                eligibility_rules: Some(serde_json::json!({ "type": "merit", "account_code": "410302" })),
                is_approval_required: true,
                is_auto_apply: false,
                is_stackable: false,
                priority: 2,
                is_active: true,
                created_at: now,
                updated_at: now,
            };
            self.discount_template_repo.save(&merit_disc).await?;
            created_items.push("Standard Discount Templates (Sibling, Merit)".to_string());
        }

        let updated_state = self.get_state(org_id).await?;

        Ok(ProvisionDefaultsResult {
            success: true,
            created_items,
            state: updated_state,
        })
    }
}
