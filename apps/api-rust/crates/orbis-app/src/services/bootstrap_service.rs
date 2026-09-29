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
    ValidatedUserGroup,
};
use crate::ports::entitlement::EntitlementProvider;
use crate::ports::repositories::SystemRepository;
use crate::ports::security::{PasswordHasher, TokenService};
use crate::services::system_service::{OrganizationReadiness, OrgLifecycleState, SetupItem};

// ── Public Request/Response Types ───────────────────────────────────────────

/// Request to bootstrap a new organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapRequest {
    #[serde(alias = "org_name")]
    pub org_name: String,
    #[serde(default, alias = "org_name_ar")]
    pub org_name_ar: Option<String>,
    #[serde(alias = "org_type")]
    pub org_type: String,
    pub timezone: String,
    pub locale: String,
    pub currency: String,
    #[serde(alias = "admin_full_name")]
    pub admin_full_name: String,
    #[serde(alias = "admin_email")]
    pub admin_email: String,
    #[serde(alias = "admin_password")]
    pub admin_password: String,
}

/// Result of a successful bootstrap.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapResult {
    pub org_id: String,
    pub admin_user_id: String,
    pub token: String,
    pub readiness: OrganizationReadiness,
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

        // System roles (11 standard ERP roles covering all departments)
        let roles = vec![
            ValidatedRole { id: "super_admin".into(), org_id, name: "Super Administrator".into(), name_ar: "مدير نظام عام".into(), is_system: true },
            ValidatedRole { id: "admin".into(), org_id, name: "Administrator".into(), name_ar: "مدير النظام".into(), is_system: true },
            ValidatedRole { id: "principal".into(), org_id, name: "Principal".into(), name_ar: "مدير المدرسة/الجامعة".into(), is_system: true },
            ValidatedRole { id: "teacher".into(), org_id, name: "Teacher".into(), name_ar: "معلم/محاضر".into(), is_system: true },
            ValidatedRole { id: "registrar".into(), org_id, name: "Registrar".into(), name_ar: "مسجل شؤون الطلاب".into(), is_system: true },
            ValidatedRole { id: "accountant".into(), org_id, name: "Financial Accountant".into(), name_ar: "محاسب مالي".into(), is_system: true },
            ValidatedRole { id: "exam_officer".into(), org_id, name: "Exam Officer".into(), name_ar: "مسؤول الكنترول والامتحانات".into(), is_system: true },
            ValidatedRole { id: "student".into(), org_id, name: "Student".into(), name_ar: "طالب".into(), is_system: true },
            ValidatedRole { id: "parent".into(), org_id, name: "Parent/Guardian".into(), name_ar: "ولي أمر".into(), is_system: true },
            ValidatedRole { id: "staff".into(), org_id, name: "Staff".into(), name_ar: "موظف".into(), is_system: true },
            ValidatedRole { id: "auditor".into(), org_id, name: "Financial Auditor".into(), name_ar: "مراجع مالي".into(), is_system: true },
        ];

        // Role permissions — resolved from comprehensive ERP permissions catalog
        let role_permissions = self.build_default_permissions(org_id);

        // Period templates
        let period_templates = self.build_period_templates(org_id, institution_type);

        // Assessment scheme
        let (formula, max_score, scheme_name) = match institution_type {
            InstitutionType::School => (
                serde_json::json!({
                    "type": "weighted_average",
                    "components": [
                        {
                            "id": "comp_assignments",
                            "name": "الواجبات والمهام المدرسية",
                            "component_type": "assignment",
                            "weight": 20,
                            "max_score": 100,
                            "passing_score": 50
                        },
                        {
                            "id": "comp_oral",
                            "name": "التقييم الشفهي والمشاركات",
                            "component_type": "oral",
                            "weight": 20,
                            "max_score": 100,
                            "passing_score": 50
                        },
                        {
                            "id": "comp_attendance",
                            "name": "المواظبة والانضباط السلوكي",
                            "component_type": "attendance",
                            "weight": 20,
                            "max_score": 100,
                            "passing_score": 50
                        },
                        {
                            "id": "comp_exam",
                            "name": "الاختبار التحريري الفصلي",
                            "component_type": "exam",
                            "weight": 40,
                            "max_score": 100,
                            "passing_score": 50
                        }
                    ]
                }),
                100.0,
                "مخطط التقييم المدرسي القياسي (واجبات 20 + شفهي 20 + مواظبة 20 + تحريري 40 = 100)"
            ),
            InstitutionType::University => (
                serde_json::json!({
                    "type": "weighted_average",
                    "components": [
                        {
                            "id": "comp_coursework",
                            "name": "الأعمال الفصلية والبحوث",
                            "component_type": "assignment",
                            "weight": 40,
                            "max_score": 100,
                            "passing_score": 60
                        },
                        {
                            "id": "comp_final_exam",
                            "name": "الاختبار النهائي",
                            "component_type": "exam",
                            "weight": 60,
                            "max_score": 100,
                            "passing_score": 60
                        }
                    ]
                }),
                100.0,
                "مخطط التقييم الجامعي القياسي (أعمال فصلية 40 + اختبار نهائي 60 = 100)"
            ),
            InstitutionType::Institute => (
                serde_json::json!({
                    "type": "weighted_average",
                    "components": [
                        {
                            "id": "comp_practical",
                            "name": "التقييم العملي والتطبيقي",
                            "component_type": "practical",
                            "weight": 50,
                            "max_score": 100,
                            "passing_score": 50
                        },
                        {
                            "id": "comp_theory",
                            "name": "الاختبار النظري",
                            "component_type": "exam",
                            "weight": 50,
                            "max_score": 100,
                            "passing_score": 50
                        }
                    ]
                }),
                100.0,
                "مخطط تقييم المعاهد والتدريب المهني (عملي 50 + نظري 50 = 100)"
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
            user_groups: self.build_default_user_groups(org_id),
            currency,
            org_settings,
            report_definitions: vec![],
        }
    }

    pub fn build_default_user_groups(&self, _org_id: Uuid) -> Vec<ValidatedUserGroup> {
        vec![
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "School Administration".to_string(),
                name_ar: "الإدارة المدرسية".to_string(),
                description: "صلاحيات الإشراف الإداري والأكاديمي الشامل، إدارة الوحدات والسياسات العامة.".to_string(),
                permissions: vec![
                    "organization:read".into(), "organization:write".into(), "org_unit:read".into(), "org_unit:write".into(),
                    "user:read".into(), "user:write".into(), "employee:read".into(), "employee:write".into(),
                    "group:read".into(), "group:assign".into(), "student:read".into(), "student:write".into(),
                    "enrollment:read".into(), "period:read".into(), "class:read".into(), "subject:read".into(),
                    "schedule:read".into(), "timetable:read".into(), "grade:read".into(), "grade:analytics".into(),
                    "grade:report".into(), "attendance:read".into(), "attendance:analytics".into(), "attendance:report".into(),
                    "behavior:read".into(), "finance:read".into(), "dashboard:read".into(), "audit:read".into(),
                    "report:read".into(), "report:generate".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Teachers & Faculty".to_string(),
                name_ar: "هيئة التدريس والمعلمون".to_string(),
                description: "إدخال ورصد الدرجات والتقييمات، تسجيل الحضور والغياب، ومتابعة الحصص والجداول.".to_string(),
                permissions: vec![
                    "class:read".into(), "subject:read".into(), "schedule:read".into(), "timetable:read".into(),
                    "lesson:read".into(), "lesson:write".into(), "student:read".into(), "attendance:read".into(),
                    "attendance:write".into(), "attendance:report".into(), "grade:read".into(), "grade:write".into(),
                    "grade:submit".into(), "grade:report".into(), "assessment:read".into(), "behavior:read".into(),
                    "behavior:write".into(), "document:read".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Student Affairs & Admissions".to_string(),
                name_ar: "شؤون الطلاب والقبول".to_string(),
                description: "إدارة ملفات الطلاب وأولياء الأمور، التسجيل، السجلات الأكاديمية والوثائق الرسمية.".to_string(),
                permissions: vec![
                    "student:read".into(), "student:write".into(), "student:create".into(), "student:update".into(), "student:delete".into(),
                    "enrollment:read".into(), "enrollment:write".into(), "enrollment:approve".into(), "enrollment:transfer".into(), "enrollment:promote".into(),
                    "guardian:read".into(), "guardian:write".into(), "guardian:delete".into(),
                    "document:read".into(), "document:write".into(), "document:verify".into(),
                    "attendance:read".into(), "attendance:write".into(), "behavior:read".into(), "behavior:write".into(),
                    "class:read".into(), "period:read".into(), "report:read".into(), "report:generate".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Human Resources".to_string(),
                name_ar: "الموارد البشرية وشؤون الموظفين".to_string(),
                description: "إدارة السجلات الوظيفية للكادر التعليمي والإداري، العقود، والصلاحيات والمجموعات.".to_string(),
                permissions: vec![
                    "employee:read".into(), "employee:write".into(), "employee:create".into(), "employee:update".into(), "employee:delete".into(),
                    "user:read".into(), "user:password_write".into(), "group:read".into(), "group:assign".into(),
                    "org_unit:read".into(), "document:read".into(), "document:write".into(), "audit:read".into(),
                    "report:read".into(), "report:generate".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Finance & Accounting".to_string(),
                name_ar: "الشؤون المالية والحسابات".to_string(),
                description: "إدارة الرسوم الدراسية، الفواتير، سندات القبض والصرف، الخصومات والتقارير المالية.".to_string(),
                permissions: vec![
                    "finance:read".into(), "finance:write".into(), "finance:manage".into(), "account:read".into(), "account:write".into(),
                    "invoice:read".into(), "invoice:write".into(), "payment:read".into(), "payment:write".into(), "payment:refund".into(),
                    "fee:read".into(), "fee:write".into(), "fee:split".into(), "discount:read".into(), "discount:write".into(),
                    "student:read".into(), "enrollment:read".into(), "report:read".into(), "report:generate".into(), "audit:read".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Exam Control Board".to_string(),
                name_ar: "الكنترول ولجنة الامتحانات".to_string(),
                description: "إدارة الاختبارات والتقييمات الفصلية، توزيع اللجان، مراجعة واعتماد وتثبيت النتائج النهائية.".to_string(),
                permissions: vec![
                    "exam:read".into(), "exam:write".into(), "exam:manage".into(), "exam_session:read".into(), "exam_session:write".into(),
                    "assessment:read".into(), "assessment:write".into(), "assessment:publish".into(),
                    "grade:read".into(), "grade:write".into(), "grade:submit".into(), "grade:approve".into(), "grade:lock".into(), "grade:report".into(),
                    "period:read".into(), "class:read".into(), "subject:read".into(), "student:read".into(), "report:generate".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Students".to_string(),
                name_ar: "مجموعة الطلاب".to_string(),
                description: "بوابة الطالب لاستعراض الجداول الدراسية، الحضور، التقييمات، والنتائج الأكاديمية.".to_string(),
                permissions: vec![
                    "student:read_self".into(), "grade:read_self".into(), "attendance:read_self".into(),
                    "schedule:read_self".into(), "timetable:read_self".into(), "lesson:read_self".into(),
                    "invoice:read_self".into(), "document:read_self".into(), "assessment:read_self".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Parents & Guardians".to_string(),
                name_ar: "مجموعة أولياء الأمور".to_string(),
                description: "بوابة ولي الأمر لمتابعة التحصيل الدراسي للأبناء، الحضور، الرسوم، والتقارير الدورية.".to_string(),
                permissions: vec![
                    "student:read_children".into(), "grade:read_children".into(), "attendance:read_children".into(),
                    "schedule:read_children".into(), "timetable:read_children".into(), "invoice:read_children".into(),
                    "payment:read_children".into(), "document:read_children".into(), "behavior:read_children".into(),
                ],
            },
            ValidatedUserGroup {
                id: Uuid::new_v4(),
                name: "Student Guidance & Counseling".to_string(),
                name_ar: "التوجيه والإرشاد الطلابي".to_string(),
                description: "متابعة السلوك والانضباط المدرسي، الرعاية الطلابية، والتواصل مع أولياء الأمور.".to_string(),
                permissions: vec![
                    "student:read".into(), "attendance:read".into(), "behavior:read".into(), "behavior:write".into(),
                    "behavior:delete".into(), "grade:read".into(), "guardian:read".into(), "document:read".into(),
                    "report:read".into(), "report:generate".into(),
                ],
            },
        ]
    }

    fn build_default_permissions(&self, org_id: Uuid) -> Vec<ValidatedRolePermission> {
        let mut perms = vec![];
        let rules: Vec<(&str, &str, Vec<&str>)> = vec![
            // ── 1. Super Administrator (Universal Root Access) ─────────────────
            ("super_admin", "*", vec!["*"]),

            // ── 2. Organization Administrator (Complete Operations) ────────────
            ("admin", "organization", vec!["read", "write"]),
            ("admin", "org_unit", vec!["read", "write", "delete"]),
            ("admin", "user", vec!["read", "write", "delete", "role_write", "password_write"]),
            ("admin", "group", vec!["read", "write", "delete", "assign"]),
            ("admin", "employee", vec!["read", "write", "create", "update", "delete"]),
            ("admin", "period", vec!["read", "write", "activate", "lock", "unlock"]),
            ("admin", "class", vec!["read", "write", "delete"]),
            ("admin", "subject", vec!["read", "write", "delete"]),
            ("admin", "schedule", vec!["read", "write", "delete"]),
            ("admin", "student", vec!["read", "create", "update", "delete", "write"]),
            ("admin", "guardian", vec!["read", "write", "delete"]),
            ("admin", "enrollment", vec!["read", "write", "promote", "transfer"]),
            ("admin", "promotion", vec!["read", "write", "execute"]),
            ("admin", "assessment", vec!["read", "write", "delete"]),
            ("admin", "grade", vec!["read", "write", "entry_manage", "analytics", "report"]),
            ("admin", "attendance", vec!["read", "write", "analytics", "report"]),
            ("admin", "behavior", vec!["read", "write", "delete", "ai"]),
            ("admin", "exam_session", vec!["read", "write", "activate", "delete"]),
            ("admin", "seating_number", vec!["read", "generate"]),
            ("admin", "secret_number", vec!["read", "generate", "reveal"]),
            ("admin", "exam_hall", vec!["read", "write", "allocate"]),
            ("admin", "proctor", vec!["read", "assign"]),
            ("admin", "student_order", vec!["read", "rebuild", "freeze"]),
            ("admin", "exam_schedule", vec!["read", "write"]),
            ("admin", "finance", vec!["read", "write", "payment"]),
            ("admin", "document", vec!["read", "write", "delete"]),
            ("admin", "template", vec!["read", "write", "publish", "delete", "import", "export", "generate_single", "generate_batch", "generate_download", "asset_upload", "asset_delete", "component_write", "component_delete"]),
            ("admin", "workflow", vec!["read", "write", "approve", "execute"]),
            ("admin", "notification", vec!["read", "write"]),
            ("admin", "dashboard", vec!["read"]),
            ("admin", "audit", vec!["read"]),
            ("admin", "data", vec!["import", "export"]),
            ("admin", "system", vec!["autopilot"]),

            // ── 3. School Principal / Dean (Academic Leadership & Control) ─────
            ("principal", "organization", vec!["read"]),
            ("principal", "org_unit", vec!["read"]),
            ("principal", "user", vec!["read"]),
            ("principal", "employee", vec!["read"]),
            ("principal", "period", vec!["read", "activate", "lock", "unlock"]),
            ("principal", "class", vec!["read", "write"]),
            ("principal", "subject", vec!["read"]),
            ("principal", "schedule", vec!["read", "write"]),
            ("principal", "student", vec!["read", "update"]),
            ("principal", "guardian", vec!["read"]),
            ("principal", "enrollment", vec!["read", "promote", "transfer"]),
            ("principal", "promotion", vec!["read", "execute"]),
            ("principal", "assessment", vec!["read", "write"]),
            ("principal", "grade", vec!["read", "analytics", "report"]),
            ("principal", "attendance", vec!["read", "analytics", "report"]),
            ("principal", "behavior", vec!["read", "write", "ai"]),
            ("principal", "exam_session", vec!["read", "activate"]),
            ("principal", "seating_number", vec!["read", "generate"]),
            ("principal", "secret_number", vec!["read", "reveal"]),
            ("principal", "exam_hall", vec!["read", "allocate"]),
            ("principal", "proctor", vec!["read", "assign"]),
            ("principal", "student_order", vec!["read", "freeze"]),
            ("principal", "exam_schedule", vec!["read", "write"]),
            ("principal", "finance", vec!["read"]),
            ("principal", "document", vec!["read", "write"]),
            ("principal", "template", vec!["read", "generate_single", "generate_batch", "generate_download"]),
            ("principal", "workflow", vec!["read", "approve", "execute"]),
            ("principal", "notification", vec!["read", "write"]),
            ("principal", "dashboard", vec!["read"]),
            ("principal", "audit", vec!["read"]),

            // ── 4. Teacher / Faculty (Classroom, Attendance, Gradebook) ────────
            ("teacher", "class", vec!["read"]),
            ("teacher", "subject", vec!["read"]),
            ("teacher", "schedule", vec!["read"]),
            ("teacher", "student", vec!["read"]),
            ("teacher", "guardian", vec!["read"]),
            ("teacher", "assessment", vec!["read", "write"]),
            ("teacher", "grade", vec!["read", "write", "report"]),
            ("teacher", "attendance", vec!["read", "write"]),
            ("teacher", "behavior", vec!["read", "write"]),
            ("teacher", "exam_schedule", vec!["read"]),
            ("teacher", "document", vec!["read"]),
            ("teacher", "template", vec!["read", "generate_single"]),
            ("teacher", "notification", vec!["read"]),

            // ── 5. Academic Registrar (Admissions, Rosters & Dossiers) ─────────
            ("registrar", "period", vec!["read"]),
            ("registrar", "class", vec!["read", "write"]),
            ("registrar", "subject", vec!["read"]),
            ("registrar", "student", vec!["read", "create", "update", "write"]),
            ("registrar", "guardian", vec!["read", "write"]),
            ("registrar", "enrollment", vec!["read", "write", "promote", "transfer"]),
            ("registrar", "promotion", vec!["read", "write"]),
            ("registrar", "attendance", vec!["read"]),
            ("registrar", "document", vec!["read", "write"]),
            ("registrar", "template", vec!["read", "generate_single", "generate_batch", "generate_download"]),
            ("registrar", "notification", vec!["read", "write"]),

            // ── 6. Financial Accountant (Billing, Payments & Receipts) ─────────
            ("accountant", "student", vec!["read"]),
            ("accountant", "guardian", vec!["read"]),
            ("accountant", "enrollment", vec!["read"]),
            ("accountant", "finance", vec!["read", "write", "payment"]),
            ("accountant", "document", vec!["read", "write"]),
            ("accountant", "template", vec!["read", "generate_single", "generate_batch", "generate_download"]),
            ("accountant", "notification", vec!["read"]),
            ("accountant", "dashboard", vec!["read"]),

            // ── 7. Exam & Control Officer (Blind Grading, Halls & Rosters) ─────
            ("exam_officer", "period", vec!["read"]),
            ("exam_officer", "class", vec!["read"]),
            ("exam_officer", "subject", vec!["read"]),
            ("exam_officer", "student", vec!["read"]),
            ("exam_officer", "assessment", vec!["read"]),
            ("exam_officer", "grade", vec!["read", "write", "entry_manage", "report"]),
            ("exam_officer", "exam_session", vec!["read", "write", "activate", "delete"]),
            ("exam_officer", "seating_number", vec!["read", "generate"]),
            ("exam_officer", "secret_number", vec!["read", "generate", "reveal"]),
            ("exam_officer", "exam_hall", vec!["read", "write", "allocate"]),
            ("exam_officer", "proctor", vec!["read", "assign"]),
            ("exam_officer", "student_order", vec!["read", "rebuild", "freeze"]),
            ("exam_officer", "exam_schedule", vec!["read", "write"]),
            ("exam_officer", "document", vec!["read", "write"]),
            ("exam_officer", "template", vec!["read", "generate_single", "generate_batch", "generate_download"]),
            ("exam_officer", "notification", vec!["read", "write"]),

            // ── 8. Administrative Staff (Support & Inquiries) ──────────────────
            ("staff", "user", vec!["read"]),
            ("staff", "student", vec!["read"]),
            ("staff", "guardian", vec!["read"]),
            ("staff", "enrollment", vec!["read"]),
            ("staff", "attendance", vec!["read", "write"]),
            ("staff", "document", vec!["read", "write"]),
            ("staff", "template", vec!["read", "generate_single"]),
            ("staff", "notification", vec!["read"]),

            // ── 9. Financial Auditor (Compliance & Inspection) ─────────────────
            ("auditor", "finance", vec!["read"]),
            ("auditor", "audit", vec!["read"]),
            ("auditor", "dashboard", vec!["read"]),
            ("auditor", "document", vec!["read"]),
            ("auditor", "workflow", vec!["read"]),
            ("auditor", "template", vec!["read"]),

            // ── 10. Student (Self-Service Portal) ─────────────────────────────
            ("student", "schedule", vec!["read"]),
            ("student", "grade", vec!["read"]),
            ("student", "attendance", vec!["read"]),
            ("student", "document", vec!["read"]),
            ("student", "notification", vec!["read"]),

            // ── 11. Parent / Guardian (Family Portal) ─────────────────────────
            ("parent", "student", vec!["read"]),
            ("parent", "schedule", vec!["read"]),
            ("parent", "grade", vec!["read"]),
            ("parent", "attendance", vec!["read"]),
            ("parent", "behavior", vec!["read"]),
            ("parent", "finance", vec!["read"]),
            ("parent", "document", vec!["read"]),
            ("parent", "notification", vec!["read"]),
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

    fn build_period_templates(&self, org_id: Uuid, inst_type: &InstitutionType) -> Vec<ValidatedPeriodTemplate> {
        let is_school = matches!(inst_type, InstitutionType::School);
        let is_uni = matches!(inst_type, InstitutionType::University);
        let is_inst = matches!(inst_type, InstitutionType::Institute);

        vec![
            // ── K-12 / General School Templates ─────────────────────────────
            ValidatedPeriodTemplate {
                id: Uuid::new_v4(), org_id,
                name: "Standard School System (2 Semesters, 4 Assessment Periods Each)".into(),
                name_ar: "نظام الفصلين الدراسيين (فصلان، 4 فترات تقييم لكل فصل)".into(),
                institution_type: "school".into(),
                structure: serde_json::json!([
                    {
                        "name": "Term 1",
                        "name_ar": "الفصل الدراسي الأول",
                        "type": "term",
                        "weight": 50,
                        "children": [
                            { "name": "Month 1", "name_ar": "الشهر الأول", "type": "month", "weight": 20 },
                            { "name": "Month 2", "name_ar": "الشهر الثاني", "type": "month", "weight": 20 },
                            { "name": "Month 3", "name_ar": "الشهر الثالث", "type": "month", "weight": 20 },
                            { "name": "Term 1 Final Exam", "name_ar": "الاختبار النهائي للفصل الأول", "type": "exam", "weight": 40 }
                        ]
                    },
                    {
                        "name": "Term 2",
                        "name_ar": "الفصل الدراسي الثاني",
                        "type": "term",
                        "weight": 50,
                        "children": [
                            { "name": "Month 4", "name_ar": "الشهر الرابع", "type": "month", "weight": 20 },
                            { "name": "Month 5", "name_ar": "الشهر الخامس", "type": "month", "weight": 20 },
                            { "name": "Month 6", "name_ar": "الشهر السادس", "type": "month", "weight": 20 },
                            { "name": "Term 2 Final Exam", "name_ar": "الاختبار النهائي للفصل الثاني", "type": "exam", "weight": 40 }
                        ]
                    }
                ]),
                is_default: is_school,
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
                name: "Four Quarters System".into(),
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

            // ── University / Higher Education Templates ─────────────────────
            ValidatedPeriodTemplate {
                id: Uuid::new_v4(), org_id,
                name: "University Semesters + Summer Session".into(),
                name_ar: "النظام الجامعي الفصلي (فصلان دراسيان + فصل صيفي)".into(),
                institution_type: "university".into(),
                structure: serde_json::json!([
                    {
                        "name": "Fall Semester",
                        "name_ar": "الفصل الدراسي الخريفي (الأول)",
                        "type": "semester",
                        "weight": 50,
                        "children": [
                            { "name": "Midterm Exam", "name_ar": "اختبار منتصف الفصل الخريفي", "type": "exam", "weight": 30 },
                            { "name": "Coursework & Practical", "name_ar": "الأعمال الفصلية والمختبرات", "type": "month", "weight": 20 },
                            { "name": "Final Semester Exam", "name_ar": "الاختبار النهائي للفصل الخريفي", "type": "exam", "weight": 50 }
                        ]
                    },
                    {
                        "name": "Spring Semester",
                        "name_ar": "الفصل الدراسي الربيعي (الثاني)",
                        "type": "semester",
                        "weight": 50,
                        "children": [
                            { "name": "Midterm Exam", "name_ar": "اختبار منتصف الفصل الربيعي", "type": "exam", "weight": 30 },
                            { "name": "Coursework & Practical", "name_ar": "الأعمال الفصلية والمختبرات", "type": "month", "weight": 20 },
                            { "name": "Final Semester Exam", "name_ar": "الاختبار النهائي للفصل الربيعي", "type": "exam", "weight": 50 }
                        ]
                    },
                    {
                        "name": "Summer Session (Optional)",
                        "name_ar": "الفصل الدراسي الصيفي (اختياري)",
                        "type": "semester",
                        "weight": 0,
                        "children": [
                            { "name": "Summer Intensive Midterm", "name_ar": "اختبار منتصف الصيفي", "type": "exam", "weight": 40 },
                            { "name": "Summer Final Exam", "name_ar": "الاختبار النهائي للصيفي", "type": "exam", "weight": 60 }
                        ]
                    }
                ]),
                is_default: is_uni,
            },

            // ── Technical Institute / Training Center Templates ─────────────
            ValidatedPeriodTemplate {
                id: Uuid::new_v4(), org_id,
                name: "Modular Vocational System (3 Modules + Capstone)".into(),
                name_ar: "نظام الوحدات والمساقات التدريبية (3 وحدات + مشروع التخرج والتقييم النهائي)".into(),
                institution_type: "institute".into(),
                structure: serde_json::json!([
                    {
                        "name": "Module 1: Foundations",
                        "name_ar": "الوحدة التدريبية الأولى: الأساسيات",
                        "type": "module",
                        "weight": 30,
                        "children": [
                            { "name": "Theory Assessment", "name_ar": "التقييم النظري للوحدة الأولى", "type": "exam", "weight": 50 },
                            { "name": "Hands-on Workshop", "name_ar": "التطبيق العملي للورشة", "type": "month", "weight": 50 }
                        ]
                    },
                    {
                        "name": "Module 2: Advanced Practice",
                        "name_ar": "الوحدة التدريبية الثانية: التطبيقات المتقدمة",
                        "type": "module",
                        "weight": 30,
                        "children": [
                            { "name": "Theory Assessment", "name_ar": "التقييم النظري للوحدة الثانية", "type": "exam", "weight": 50 },
                            { "name": "Hands-on Workshop", "name_ar": "التطبيق العملي للورشة", "type": "month", "weight": 50 }
                        ]
                    },
                    {
                        "name": "Module 3: Capstone & Internship",
                        "name_ar": "الوحدة التدريبية الثالثة: مشروع التخرج والتدريب الميداني",
                        "type": "module",
                        "weight": 40,
                        "children": [
                            { "name": "Final Practical Assessment", "name_ar": "التقييم العملي والمهني النهائي", "type": "exam", "weight": 60 },
                            { "name": "Capstone Project Evaluation", "name_ar": "مناقشة وتقييم مشروع التخرج", "type": "exam", "weight": 40 }
                        ]
                    }
                ]),
                is_default: is_inst,
            }
        ]
    }

    fn build_org_settings(&self, inst_type: &InstitutionType, currency: &str) -> serde_json::Value {
        let (grading_system, gpa_scale, max_grade, passing_grade) = match inst_type {
            InstitutionType::School => ("percentage", 4.0, 100.0, 50.0),
            InstitutionType::University => ("gpa_4_0", 4.0, 100.0, 60.0),
            InstitutionType::Institute => ("pass_fail", 4.0, 100.0, 60.0),
        };

        let student_prefix = match inst_type {
            InstitutionType::School => "STU",
            InstitutionType::University => "UNI",
            InstitutionType::Institute => "INST",
        };

        let max_capacity = match inst_type {
            InstitutionType::School => 25,
            InstitutionType::University => 50,
            InstitutionType::Institute => 20,
        };

        serde_json::json!({
            "academic": {
                "default_grading_system": grading_system,
                "passing_grade": passing_grade,
                "max_grade": max_grade,
                "credit_hours_enabled": matches!(inst_type, InstitutionType::University),
                "allow_grade_change_after_lock": false,
                "gpa_scale": gpa_scale,
                "is_hijri_enabled": false,
                "week_start_day": "sunday",
                "working_days": ["sunday", "monday", "tuesday", "wednesday", "thursday"]
            },
            "admission": {
                "numbering": {
                    "prefix": student_prefix,
                    "year_format": "full",
                    "sequence_digits": 4,
                    "separator": "-"
                },
                "policy": {
                    "requires_approval": true,
                    "allow_no_guardian": matches!(inst_type, InstitutionType::University | InstitutionType::Institute),
                    "allow_duplicate_national_id": false,
                    "auto_assign_fees": true,
                    "max_class_capacity": max_capacity,
                    "allow_late_enrollment": false,
                    "allow_waitlist": true,
                    "sibling_priority_enabled": matches!(inst_type, InstitutionType::School)
                },
                "fields": [
                    { "key": "national_id", "name_ar": "الرقم الوطني / رقم الهوية", "name_en": "National ID Number", "is_active": true, "is_required": true, "prevent_duplicate": true, "supports_duplicate": true },
                    { "key": "phone", "name_ar": "رقم هاتف الطالب / التواصل", "name_en": "Student Phone Number", "is_active": true, "is_required": true, "prevent_duplicate": false, "supports_duplicate": true },
                    { "key": "email", "name_ar": "البريد الإلكتروني", "name_en": "Email Address", "is_active": true, "is_required": false, "prevent_duplicate": false, "supports_duplicate": true },
                    { "key": "birth_date", "name_ar": "تاريخ الميلاد", "name_en": "Date of Birth", "is_active": true, "is_required": true, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "sex", "name_ar": "الجنس", "name_en": "Gender", "is_active": true, "is_required": true, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "nationality", "name_ar": "الجنسية الرسمية", "name_en": "Nationality", "is_active": true, "is_required": true, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "address", "name_ar": "عنوان السكن والحي", "name_en": "Residential Address", "is_active": true, "is_required": false, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "place_of_birth", "name_ar": "مكان الميلاد", "name_en": "Place of Birth", "is_active": true, "is_required": false, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "blood_type", "name_ar": "فصيلة الدم", "name_en": "Blood Type", "is_active": true, "is_required": false, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "emergency_contact", "name_ar": "جهة الاتصال للطوارئ", "name_en": "Emergency Contact", "is_active": true, "is_required": true, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "medical_notes", "name_ar": "الملاحظات الطبية والحساسية", "name_en": "Medical Notes & Allergies", "is_active": true, "is_required": false, "prevent_duplicate": false, "supports_duplicate": false },
                    { "key": "previous_school", "name_ar": "المدرسة / المؤسسة السابقة", "name_en": "Previous Institution", "is_active": true, "is_required": false, "prevent_duplicate": false, "supports_duplicate": false }
                ]
            },
            "enrollment": {
                "registration_enabled": true,
                "require_admin_approval": true,
                "allow_waitlist": true,
                "sibling_priority_enabled": matches!(inst_type, InstitutionType::School),
                "max_class_size_default": max_capacity,
                "allow_late_enrollment": false
            },
            "finance": {
                "currency": currency,
                "tax_rate": 0.15,
                "invoice_due_days": 14,
                "late_fee_type": "fixed",
                "late_fee_value": 0.0,
                "allow_partial_payments": true,
                "primary_payment_gateway_id": null,
                "invoicing_integration_id": null
            },
            "attendance": {
                "daily_attendance_mode": "single",
                "absence_threshold_warning": 3,
                "absence_threshold_critical": 7,
                "allow_excuse_submission": true,
                "auto_notify_parents_on_absence": true
            },
            "examinations": {
                "blind_grading_enabled": true,
                "secret_number_digits": 5,
                "seating_number_prefix": "",
                "seating_number_start": 1001,
                "auto_balance_exam_halls": true,
                "proctor_ratio_per_students": 20,
                "allow_proctor_self_swaps": false
            },
            "documents": {
                "default_certificate_header_ar": "المملكة العربية السعودية - وزارة التعليم",
                "qr_verification_enabled": true,
                "signature_verification_required": true,
                "archival_retention_years": 10
            },
            "notifications": {
                "email_integration_id": null,
                "sms_integration_id": null,
                "whatsapp_integration_id": null,
                "notify_parents_on_absence": true,
                "notify_parents_on_grade_publish": true,
                "finance_reminder_days": [7, 3, 1]
            },
            "security": {
                "session_timeout_minutes": 60,
                "max_login_attempts": 5,
                "password_min_length": 8,
                "mfa_policy": "optional",
                "device_tracking_enabled": true
            },
            "autopilot": {
                "enabled": true,
                "mode": "suggested",
                "auto_open_enrollment": false,
                "auto_close_enrollment": true,
                "auto_record_attendance": true,
                "auto_fee_assignment": true,
                "auto_grade_entry_reminder": true,
                "auto_period_lock": false,
                "auto_promotion": false,
                "auto_exam_scheduling": false,
                "auto_report_generation": true,
                "grace_days": 3,
                "notify_before_deadline_hours": 24,
                "max_retries": 3,
                "auto_advance_periods": false,
                "auto_lock_grades": false,
                "grade_entry_auto_close_days": 7,
                "allow_grade_edit_after_close": false
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
