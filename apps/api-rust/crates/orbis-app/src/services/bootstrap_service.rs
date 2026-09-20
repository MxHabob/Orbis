//! Bootstrap Service — organization bootstrap orchestration and initial setup.
//!
//! This is the **single Application-layer owner** of organization initialization.
//! It replaces the scattered bootstrap logic that was previously split between
//! CLI (`commands/init.rs`) and Infrastructure (`bootstrap_adapter.rs`).
//!
//! Responsibilities:
//! - Input validation (org name, admin credentials, institution type)
//! - Business rule enforcement (idempotency, no duplicate orgs)
//! - Blueprint resolution (institution type → roles, permissions, COA, settings)
//! - Delegation to Infrastructure for efficient bulk persistence
//! - Auto-login token generation after bootstrap
//!
//! Callable from: CLI, Desktop (Local mode), and Server (Remote mode).

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::info;

use orbis_domain::entities::organization::InstitutionType;
use orbis_domain::errors::DomainError;

use crate::ports::bootstrap::{
    BulkBootstrapPort, ValidatedBootstrapData, ValidatedOrg, ValidatedPerson,
    ValidatedUser, ValidatedRoleAssignment, ValidatedRole, ValidatedRolePermission,
    ValidatedPeriodTemplate, ValidatedAssessmentScheme, ValidatedCurrency,
    ValidatedInitialSetupData, ValidatedAcademicYear, ValidatedPeriod,
    ValidatedClass, ValidatedSubject,
};
use crate::ports::entitlement::EntitlementProvider;
use crate::ports::repositories::SystemRepository;
use crate::ports::security::{PasswordHasher, TokenService};
use crate::services::system_service::{OrganizationReadiness, OrgLifecycleState, SetupItem};

// ── Public Request/Response Types ───────────────────────────────────────────

/// Request to bootstrap a new organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapRequest {
    pub org_name: String,
    pub org_name_ar: Option<String>,
    pub org_type: String,
    pub timezone: String,
    pub locale: String,
    pub currency: String,
    pub admin_full_name: String,
    pub admin_email: String,
    pub admin_password: String,
}

/// Result of a successful bootstrap.
#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapResult {
    pub org_id: String,
    pub admin_user_id: String,
    pub token: String,
    pub readiness: OrganizationReadiness,
}

/// Request for completing initial academic setup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialSetupRequest {
    pub academic_year: AcademicYearInput,
    pub periods: Vec<PeriodInput>,
    pub classes: Vec<ClassInput>,
    pub subjects: Vec<SubjectInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicYearInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub period_type: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectInput {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub is_core: bool,
}

// ── Service ─────────────────────────────────────────────────────────────────

pub struct BootstrapService {
    pub bootstrap_port: Arc<dyn BulkBootstrapPort>,
    pub system_repo: Arc<dyn SystemRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub token_service: Arc<dyn TokenService>,
    pub entitlement_provider: Arc<dyn EntitlementProvider>,
}

impl BootstrapService {
    pub fn new(
        bootstrap_port: Arc<dyn BulkBootstrapPort>,
        system_repo: Arc<dyn SystemRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<dyn TokenService>,
        entitlement_provider: Arc<dyn EntitlementProvider>,
    ) -> Self {
        Self {
            bootstrap_port,
            system_repo,
            password_hasher,
            token_service,
            entitlement_provider,
        }
    }

    /// Bootstrap a new organization with admin — single transactional operation.
    ///
    /// This is the **only** way to create an organization. Callable from CLI, Desktop, and Server.
    ///
    /// Guarantees:
    /// - Input validation before any persistence
    /// - Idempotency — returns error if org already exists
    /// - Single atomic transaction (all-or-nothing)
    /// - Auto-login token for the created admin
    pub async fn bootstrap_organization(
        &self,
        request: BootstrapRequest,
    ) -> Result<BootstrapResult, DomainError> {
        // 1. Validate inputs
        self.validate_bootstrap_request(&request)?;

        // 2. Check idempotency — no org should exist yet
        if self.system_repo.has_organization().await? {
            return Err(DomainError::AlreadyInitialized);
        }

        // 3. Hash admin password
        let password_hash = self.password_hasher.hash(&request.admin_password).await?;

        // 4. Resolve institution type and blueprint
        let institution_type = InstitutionType::from_str_loose(&request.org_type);

        // 5. Build validated data
        let org_id = Uuid::new_v4();
        let admin_person_id = Uuid::new_v4();
        let admin_user_id = Uuid::new_v4();
        let slug = Self::slugify(&request.org_name);

        let validated = self.build_bootstrap_data(
            org_id,
            admin_person_id,
            admin_user_id,
            &slug,
            &password_hash,
            &institution_type,
            &request,
        );

        info!(
            "Bootstrapping organization '{}' (type={}, org_id={})",
            request.org_name, institution_type, org_id
        );

        // 6. Delegate to Infrastructure for efficient bulk persistence
        let persistence_result = self.bootstrap_port.persist_bootstrap(validated).await
            .map_err(|e| DomainError::BootstrapFailed(format!("Persistence failed: {e}")))?;

        // 7. Generate auto-login token
        let token = self.token_service
            .generate_access_token(
                &admin_user_id.to_string(),
                &org_id.to_string(),
                "super_admin",
            )
            .await?;

        // 8. Get readiness (should be CoreReady after bootstrap)
        let readiness = self.get_readiness(org_id).await.unwrap_or(OrganizationReadiness {
            state: OrgLifecycleState::CoreReady,
            org_id: Some(org_id.to_string()),
            org_name: Some(request.org_name.clone()),
            has_organization: true,
            has_admin: true,
            has_academic_year: false,
            has_classes: false,
            has_subjects: false,
            pending_setup_items: vec![],
        });

        info!(
            "Bootstrap complete: org_id={}, admin_id={}",
            persistence_result.org_id, persistence_result.admin_user_id
        );

        Ok(BootstrapResult {
            org_id: org_id.to_string(),
            admin_user_id: admin_user_id.to_string(),
            token,
            readiness,
        })
    }

    /// Complete initial academic setup — create year, periods, classes, subjects.
    ///
    /// Single transactional operation. Idempotent — safe to retry.
    pub async fn complete_initial_setup(
        &self,
        org_id: Uuid,
        request: InitialSetupRequest,
    ) -> Result<OrganizationReadiness, DomainError> {
        // 1. Validate org exists
        if !self.system_repo.has_organization().await? {
            return Err(DomainError::NotFound("No organization found".into()));
        }

        // 2. Validate inputs
        self.validate_initial_setup(&request)?;

        // 3. Build validated data
        let year_id = Uuid::new_v4();
        let validated = ValidatedInitialSetupData {
            org_id,
            academic_year: ValidatedAcademicYear {
                id: year_id,
                org_id,
                name: request.academic_year.name.clone(),
                name_ar: request.academic_year.name_ar.clone().unwrap_or_default(),
                start_date: request.academic_year.start_date.clone(),
                end_date: request.academic_year.end_date.clone(),
            },
            periods: request.periods.iter().map(|p| ValidatedPeriod {
                id: Uuid::new_v4(),
                org_id,
                year_id,
                name: p.name.clone(),
                name_ar: p.name_ar.clone().unwrap_or_default(),
                period_type: p.period_type.clone(),
                start_date: p.start_date.clone(),
                end_date: p.end_date.clone(),
            }).collect(),
            classes: request.classes.iter().map(|c| ValidatedClass {
                id: Uuid::new_v4(),
                org_id,
                name: c.name.clone(),
                name_ar: c.name_ar.clone().unwrap_or_default(),
                level: c.level.clone(),
                division: c.division.clone(),
            }).collect(),
            subjects: request.subjects.iter().map(|s| ValidatedSubject {
                id: Uuid::new_v4(),
                org_id,
                code: s.code.clone(),
                name: s.name.clone(),
                name_ar: s.name_ar.clone().unwrap_or_default(),
                is_core: s.is_core,
            }).collect(),
        };

        // 4. Persist in single transaction
        self.bootstrap_port.persist_initial_setup(validated).await?;

        // 5. Return updated readiness
        self.get_readiness(org_id).await
    }

    /// Get organization readiness for a specific org.
    pub async fn get_readiness(&self, org_id: Uuid) -> Result<OrganizationReadiness, DomainError> {
        let has_org = self.system_repo.has_organization().await?;
        let org_name = self.system_repo.get_org_name().await?;
        let has_admin = self.system_repo.has_admin().await?;
        let has_year = self.system_repo.has_academic_year(org_id).await?;
        let has_classes = self.system_repo.has_classes(org_id).await?;
        let has_subjects = self.system_repo.has_subjects(org_id).await?;

        let state = if has_year && has_classes && has_subjects {
            OrgLifecycleState::Operational
        } else if has_org && has_admin {
            OrgLifecycleState::CoreReady
        } else {
            OrgLifecycleState::Empty
        };

        let items = vec![
            SetupItem {
                key: "academic_year".into(),
                label: "Academic Year".into(),
                label_ar: "العام الدراسي".into(),
                is_required: true,
                is_completed: has_year,
            },
            SetupItem {
                key: "classes".into(),
                label: "Classes".into(),
                label_ar: "الفصول".into(),
                is_required: true,
                is_completed: has_classes,
            },
            SetupItem {
                key: "subjects".into(),
                label: "Subjects".into(),
                label_ar: "المواد".into(),
                is_required: true,
                is_completed: has_subjects,
            },
        ];

        Ok(OrganizationReadiness {
            state,
            org_id: Some(org_id.to_string()),
            org_name,
            has_organization: has_org,
            has_admin,
            has_academic_year: has_year,
            has_classes,
            has_subjects,
            pending_setup_items: items,
        })
    }

    // ── Private Helpers ─────────────────────────────────────────────────────

    fn validate_bootstrap_request(&self, req: &BootstrapRequest) -> Result<(), DomainError> {
        if req.org_name.trim().is_empty() {
            return Err(DomainError::Validation("Organization name cannot be empty".into()));
        }
        if req.org_name.trim().len() < 2 {
            return Err(DomainError::Validation("Organization name must be at least 2 characters".into()));
        }
        if req.admin_email.trim().is_empty() || !req.admin_email.contains('@') {
            return Err(DomainError::Validation("Invalid admin email address".into()));
        }
        if req.admin_full_name.trim().is_empty() {
            return Err(DomainError::Validation("Admin full name cannot be empty".into()));
        }
        if req.admin_password.len() < 8 {
            return Err(DomainError::Validation("Admin password must be at least 8 characters".into()));
        }
        if req.timezone.trim().is_empty() {
            return Err(DomainError::Validation("Timezone cannot be empty".into()));
        }
        if req.currency.trim().is_empty() {
            return Err(DomainError::Validation("Currency code cannot be empty".into()));
        }
        Ok(())
    }

    fn validate_initial_setup(&self, req: &InitialSetupRequest) -> Result<(), DomainError> {
        if req.academic_year.name.trim().is_empty() {
            return Err(DomainError::Validation("Academic year name cannot be empty".into()));
        }
        if req.academic_year.start_date.trim().is_empty() {
            return Err(DomainError::Validation("Academic year start date is required".into()));
        }
        if req.academic_year.end_date.trim().is_empty() {
            return Err(DomainError::Validation("Academic year end date is required".into()));
        }
        if req.classes.is_empty() {
            return Err(DomainError::Validation("At least one class is required".into()));
        }
        if req.subjects.is_empty() {
            return Err(DomainError::Validation("At least one subject is required".into()));
        }
        Ok(())
    }

    fn slugify(name: &str) -> String {
        name.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_string()
    }

    fn build_bootstrap_data(
        &self,
        org_id: Uuid,
        admin_person_id: Uuid,
        admin_user_id: Uuid,
        slug: &str,
        password_hash: &str,
        institution_type: &InstitutionType,
        request: &BootstrapRequest,
    ) -> ValidatedBootstrapData {
        let assignment_id = Uuid::new_v4();

        // Parse admin name into first/last
        let name_parts: Vec<&str> = request.admin_full_name.split_whitespace().collect();
        let first_name = name_parts.first().unwrap_or(&"").to_string();
        let last_name = if name_parts.len() > 1 { name_parts[1..].join(" ") } else { first_name.clone() };

        // System roles
        let roles = vec![
            ValidatedRole { id: "super_admin".into(), org_id, name: "Super Administrator".into(), name_ar: "مدير نظام عام".into(), is_system: true },
            ValidatedRole { id: "admin".into(), org_id, name: "Administrator".into(), name_ar: "مدير النظام".into(), is_system: true },
            ValidatedRole { id: "principal".into(), org_id, name: "Principal".into(), name_ar: "مدير المدرسة/الجامعة".into(), is_system: true },
            ValidatedRole { id: "teacher".into(), org_id, name: "Teacher".into(), name_ar: "معلم/محاضر".into(), is_system: true },
            ValidatedRole { id: "student".into(), org_id, name: "Student".into(), name_ar: "طالب".into(), is_system: true },
            ValidatedRole { id: "parent".into(), org_id, name: "Parent/Guardian".into(), name_ar: "ولي أمر".into(), is_system: true },
            ValidatedRole { id: "staff".into(), org_id, name: "Staff".into(), name_ar: "موظف".into(), is_system: true },
            ValidatedRole { id: "auditor".into(), org_id, name: "Financial Auditor".into(), name_ar: "مراجع مالي".into(), is_system: true },
        ];

        // Role permissions — resolved from institution blueprint
        let role_permissions = self.build_default_permissions(org_id);

        // Period templates
        let period_templates = self.build_period_templates(org_id, institution_type);

        // Assessment scheme
        let (formula, max_score, scheme_name) = match institution_type {
            InstitutionType::School => (
                serde_json::json!({
                    "type": "add",
                    "operands": [
                        { "type": "variable", "name": "assignments" },
                        { "type": "variable", "name": "oral" },
                        { "type": "variable", "name": "attendance" },
                        { "type": "variable", "name": "written_test" }
                    ],
                    "weights": {
                        "assignments": 20,
                        "oral": 20,
                        "attendance": 20,
                        "written_test": 40,
                        "monthly_total": 100,
                        "term_monthly_contribution": 20,
                        "term_final_exam": 30,
                        "term_total": 50,
                        "annual_total": 100
                    }
                }),
                100.0,
                "مخطط التقييم المدرسي القياسي (واجبات 20 + شفهي 20 + مواظبة 20 + تحريري 40 = 100)"
            ),
            InstitutionType::University => (
                serde_json::json!({
                    "type": "weighted_sum",
                    "items": [
                        { "node": { "type": "variable", "name": "coursework" }, "weight": 0.4 },
                        { "node": { "type": "variable", "name": "final_exam" }, "weight": 0.6 }
                    ]
                }),
                100.0,
                "University Assessment Scheme (Coursework 40% + Final Exam 60%)"
            ),
            InstitutionType::Institute => (
                serde_json::json!({
                    "type": "weighted_sum",
                    "items": [
                        { "node": { "type": "variable", "name": "practical" }, "weight": 0.5 },
                        { "node": { "type": "variable", "name": "theory" }, "weight": 0.5 }
                    ]
                }),
                100.0,
                "Training Assessment Scheme (Practical 50% + Theory 50%)"
            ),
        };

        let assessment_scheme = ValidatedAssessmentScheme {
            id: Uuid::new_v4(),
            org_id,
            name: scheme_name.to_string(),
            description: "Standard Institution Assessment Scheme".to_string(),
            max_score,
            formula,
        };

        // Currency
        let currency = ValidatedCurrency {
            id: request.currency.clone(),
            name: Self::currency_name(&request.currency),
            name_ar: Self::currency_name_ar(&request.currency),
            symbol: Self::currency_symbol(&request.currency),
            decimal_places: 2,
        };

        // Org settings
        let org_settings = self.build_org_settings(institution_type, &request.currency);

        ValidatedBootstrapData {
            organization: ValidatedOrg {
                id: org_id,
                name: request.org_name.clone(),
                name_ar: request.org_name_ar.clone().unwrap_or_default(),
                slug: slug.to_string(),
                org_type: institution_type.as_str().to_string(),
                timezone: request.timezone.clone(),
                locale: request.locale.clone(),
                base_currency_code: request.currency.clone(),
            },
            admin_person: ValidatedPerson {
                id: admin_person_id,
                org_id,
                first_name,
                last_name,
                display_name: request.admin_full_name.clone(),
            },
            admin_user: ValidatedUser {
                id: admin_user_id,
                person_id: admin_person_id,
                org_id,
                email: request.admin_email.clone(),
                password_hash: password_hash.to_string(),
            },
            admin_role_assignment: ValidatedRoleAssignment {
                id: assignment_id,
                user_id: admin_user_id,
                role_id: "super_admin".to_string(),
                org_id,
                scope_type: "global".to_string(),
            },
            roles,
            role_permissions,
            period_templates,
            assessment_scheme,
            accounts: vec![], // COA loaded from JSON by Infrastructure
            currency,
            org_settings,
            report_definitions: vec![],
        }
    }

    fn build_default_permissions(&self, org_id: Uuid) -> Vec<ValidatedRolePermission> {
        let mut perms = vec![];
        let rules: Vec<(&str, &str, Vec<&str>)> = vec![
            // Administrative roles — universal access
            ("super_admin", "*", vec!["*"]),
            ("admin", "*", vec!["*"]),
            // Domain-specific roles
            ("teacher", "Grade", vec!["read", "write"]),
            ("teacher", "Attendance", vec!["read", "write"]),
            ("teacher", "Assessment", vec!["read", "write"]),
            ("teacher", "Class", vec!["read"]),
            ("teacher", "Subject", vec!["read"]),
            ("teacher", "Schedule", vec!["read"]),
            ("student", "Grade", vec!["read"]),
            ("student", "Attendance", vec!["read"]),
            ("student", "Schedule", vec!["read"]),
            ("student", "Enrollment", vec!["read"]),
            ("parent", "Grade", vec!["read"]),
            ("parent", "Attendance", vec!["read"]),
            ("parent", "Schedule", vec!["read"]),
            ("parent", "Enrollment", vec!["read"]),
            ("principal", "Organization", vec!["read"]),
            ("principal", "User", vec!["read", "write"]),
            ("principal", "Period", vec!["read", "write", "activate", "lock"]),
            ("principal", "Class", vec!["read", "write"]),
            ("principal", "Subject", vec!["read", "write"]),
            ("principal", "Grade", vec!["read", "write", "analytics", "report"]),
            ("principal", "Attendance", vec!["read", "write", "analytics"]),
            ("principal", "Assessment", vec!["read", "write"]),
            ("principal", "Enrollment", vec!["read", "write", "promote", "transfer"]),
            ("staff", "User", vec!["read"]),
            ("staff", "Enrollment", vec!["read", "write"]),
            ("staff", "Attendance", vec!["read", "write"]),
        ];

        for (role, resource, actions) in rules {
            for action in actions {
                perms.push(ValidatedRolePermission {
                    id: Uuid::new_v4(),
                    role_id: role.to_string(),
                    org_id,
                    resource: resource.to_string(),
                    action: action.to_string(),
                    effect: "allow".to_string(),
                });
            }
        }
        perms
    }

    fn build_period_templates(&self, org_id: Uuid, _inst_type: &InstitutionType) -> Vec<ValidatedPeriodTemplate> {
        vec![
            ValidatedPeriodTemplate {
                id: Uuid::new_v4(), org_id,
                name: "Standard School System (2 Terms, 4 Assessment Periods Each)".into(),
                name_ar: "نظام الفصلين الدراسيين (فصلان، 4 فترات تقييم لكل فصل)".into(),
                institution_type: "school".into(),
                structure: serde_json::json!([
                    {
                        "name": "Term 1",
                        "name_ar": "الفصل الدراسي الأول",
                        "type": "term",
                        "weight": 50,
                        "periods": [
                            { "name": "Month 1", "name_ar": "الشهر الأول", "type": "month", "weight": 100 },
                            { "name": "Month 2", "name_ar": "الشهر الثاني", "type": "month", "weight": 100 },
                            { "name": "Month 3", "name_ar": "الشهر الثالث", "type": "month", "weight": 100 },
                            { "name": "Term 1 Final Exam", "name_ar": "اختبار نهاية الفصل الأول", "type": "exam", "weight": 30 }
                        ]
                    },
                    {
                        "name": "Term 2",
                        "name_ar": "الفصل الدراسي الثاني",
                        "type": "term",
                        "weight": 50,
                        "periods": [
                            { "name": "Month 1", "name_ar": "الشهر الأول", "type": "month", "weight": 100 },
                            { "name": "Month 2", "name_ar": "الشهر الثاني", "type": "month", "weight": 100 },
                            { "name": "Month 3", "name_ar": "الشهر الثالث", "type": "month", "weight": 100 },
                            { "name": "Term 2 Final Exam", "name_ar": "اختبار نهاية الفصل الثاني", "type": "exam", "weight": 30 }
                        ]
                    }
                ]),
                is_default: true,
            },
            ValidatedPeriodTemplate {
                id: Uuid::new_v4(), org_id,
                name: "Three Terms / Trimesters".into(),
                name_ar: "نظام الفصول الثلاثية (3 فصول دراسية)".into(),
                institution_type: "school".into(),
                structure: serde_json::json!([
                    { "name": "First Term", "name_ar": "الفصل الأول", "type": "term", "weight": 33.33 },
                    { "name": "Second Term", "name_ar": "الفصل الثاني", "type": "term", "weight": 33.33 },
                    { "name": "Third Term", "name_ar": "الفصل الثالث", "type": "term", "weight": 33.34 }
                ]),
                is_default: false,
            },
            ValidatedPeriodTemplate {
                id: Uuid::new_v4(), org_id,
                name: "Four Quarters".into(),
                name_ar: "نظام الأرباع السنوية (4 أرباع)".into(),
                institution_type: "school".into(),
                structure: serde_json::json!([
                    { "name": "Quarter 1", "name_ar": "الربع الأول", "type": "quarter", "weight": 25 },
                    { "name": "Quarter 2", "name_ar": "الربع الثاني", "type": "quarter", "weight": 25 },
                    { "name": "Quarter 3", "name_ar": "الربع الثالث", "type": "quarter", "weight": 25 },
                    { "name": "Quarter 4", "name_ar": "الربع الرابع", "type": "quarter", "weight": 25 }
                ]),
                is_default: false,
            },
        ]
    }

    fn build_org_settings(&self, inst_type: &InstitutionType, currency: &str) -> serde_json::Value {
        let grading_system = match inst_type {
            InstitutionType::School => "percentage",
            InstitutionType::University => "gpa_4_0",
            InstitutionType::Institute => "pass_fail",
        };

        serde_json::json!({
            "academic": {
                "default_grading_system": grading_system
            },
            "enrollment": {
                "registration_enabled": true,
                "require_admin_approval": true,
                "allow_waitlist": true,
                "max_class_size_default": 25
            },
            "finance": {
                "currency": currency,
                "tax_rate": 0.15,
                "invoice_due_days": 14
            },
            "notifications": {
                "notify_parents_on_absence": true,
                "notify_parents_on_grade_publish": true
            },
            "autopilot": {
                "enabled": true,
                "mode": "automatic",
                "max_retries": 3
            }
        })
    }

    fn currency_name(code: &str) -> String {
        match code {
            "SAR" => "Saudi Riyal", "YER" => "Yemeni Rial", "AED" => "UAE Dirham",
            "KWD" => "Kuwaiti Dinar", "QAR" => "Qatari Rial", "EGP" => "Egyptian Pound",
            "JOD" => "Jordanian Dinar", "USD" => "US Dollar", "EUR" => "Euro",
            "GBP" => "British Pound",
            _ => "Currency",
        }.to_string()
    }

    fn currency_name_ar(code: &str) -> String {
        match code {
            "SAR" => "ريال سعودي", "YER" => "ريال يمني", "AED" => "درهم إماراتي",
            "KWD" => "دينار كويتي", "QAR" => "ريال قطري", "EGP" => "جنيه مصري",
            "JOD" => "دينار أردني", "USD" => "دولار أمريكي", "EUR" => "يورو",
            "GBP" => "جنيه إسترليني",
            _ => "عملة",
        }.to_string()
    }

    fn currency_symbol(code: &str) -> String {
        match code {
            "SAR" => "﷼", "YER" => "﷼", "AED" => "د.إ", "KWD" => "د.ك",
            "QAR" => "﷼", "EGP" => "ج.م", "JOD" => "د.أ", "USD" => "$",
            "EUR" => "€", "GBP" => "£",
            _ => code,
        }.to_string()
    }
}
