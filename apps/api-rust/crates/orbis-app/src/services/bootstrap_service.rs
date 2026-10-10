//! Bootstrap Service — Orchestrates first-time organization initialization.
//!
//! This is NOT a domain service. It is an application-level orchestrator that
//! coordinates existing domain services (Organization, IAM, Identity, Settings)
//! to create a valid, usable Orbis institution from a fresh database.
//!
//! Responsibilities:
//! - Validate bootstrap request
//! - Check idempotency (prevent duplicate bootstrap)
//! - Create organization via OrganizationService / Repository
//! - Create initial administrator (Person → User → Identity → Role)
//! - Initialize required organization settings
//! - Initialize required system roles and permissions
//! - Verify initialization completeness
//! - Return strongly-typed bootstrap result

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use orbis_domain::entities::academic_policy::{RankingScope, RoundingMode, TiePolicy};
use orbis_domain::entities::admission::{
    AdmissionPolicy, FormFieldConfig, NumberingSchema, YearFormat,
};
use orbis_domain::entities::group::Group;
use orbis_domain::entities::identity::{IdentityProvider, UserIdentity};
use orbis_domain::entities::organization::{InstitutionType, Organization};
use orbis_domain::entities::person::Person;
use orbis_domain::entities::role::Role;
use orbis_domain::entities::role_permission::RolePermission;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::entities::user::User;
use orbis_domain::entities::user_role_assignment::UserRoleAssignment;
use orbis_domain::errors::DomainError;
use rust_decimal::Decimal;

use super::coa_loader::CoaBlueprintLoader;
use crate::ports::event_bus::EventBus;
use crate::ports::repositories::{
    AccountRepository, CurrencyRepository, GroupRepository, OrganizationRepository,
    PersonRepository, RoleRepository, SettingsRepository, SystemRepository, UserIdentityRepository,
    UserRepository,
};
use crate::ports::security::PasswordHasher;
use orbis_domain::entities::currency::Currency;

fn all_admin_permission_pairs() -> Vec<(&'static str, &'static str)> {
    [
        ("organization", &["read", "write"][..]),
        (
            "user",
            &["read", "write", "delete", "role_write", "password_write"][..],
        ),
        (
            "period",
            &["read", "write", "activate", "lock", "unlock"][..],
        ),
        ("org_unit", &["read", "write", "delete"][..]),
        ("class", &["read", "write", "delete"][..]),
        ("subject", &["read", "write", "delete"][..]),
        ("schedule", &["read", "write", "delete"][..]),
        ("assessment", &["read", "write", "delete"][..]),
        (
            "grade",
            &["read", "write", "entry_manage", "analytics", "report"][..],
        ),
        (
            "student",
            &["read", "write", "create", "update", "delete"][..],
        ),
        ("guardian", &["read", "write", "delete"][..]),
        ("enrollment", &["read", "write", "promote", "transfer"][..]),
        ("workflow", &["read", "write", "approve", "execute"][..]),
        ("attendance", &["read", "write", "analytics", "report"][..]),
        ("behavior", &["read", "write", "delete", "ai"][..]),
        ("document", &["read", "write", "delete"][..]),
        (
            "template",
            &[
                "read",
                "write",
                "publish",
                "delete",
                "import",
                "export",
                "generate_single",
                "generate_batch",
                "generate_download",
                "asset_upload",
                "asset_delete",
                "component_write",
                "component_delete",
            ][..],
        ),
        ("exam_session", &["read", "write", "activate", "delete"][..]),
        ("seating_number", &["generate", "read"][..]),
        ("secret_number", &["generate", "read", "reveal"][..]),
        ("exam_hall", &["read", "write", "allocate"][..]),
        ("proctor", &["read", "assign"][..]),
        ("student_order", &["read", "rebuild", "freeze"][..]),
        ("exam_schedule", &["read", "write"][..]),
        ("notification", &["read", "write"][..]),
        ("dashboard", &["read"][..]),
        ("audit", &["read"][..]),
        ("data", &["import", "export"][..]),
        (
            "finance",
            &["read", "write", "payment", "backdate", "future_date"][..],
        ),
        ("promotion", &["read", "write", "execute"][..]),
        (
            "employee",
            &["read", "write", "create", "update", "delete"][..],
        ),
        ("group", &["read", "write", "delete", "assign"][..]),
        ("system", &["autopilot"][..]),
    ]
    .into_iter()
    .flat_map(|(resource, actions)| actions.iter().map(move |action| (resource, *action)))
    .collect()
}

fn all_admin_permission_strings() -> Vec<String> {
    all_admin_permission_pairs()
        .into_iter()
        .map(|(resource, action)| format!("{resource}:{action}"))
        .collect()
}

fn default_org_admin_group_permissions() -> Vec<String> {
    all_admin_permission_strings()
        .into_iter()
        .filter(|permission| permission != "system:autopilot")
        .collect()
}

// ═══════════════════════════════════════════════════════════════
// Bootstrap Request
// ═══════════════════════════════════════════════════════════════

/// Complete request to bootstrap a new Orbis institution.
/// Contains all information required to create a valid organization + administrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapRequest {
    // ── Institution ───────────────────────────────────────────
    pub institution_type: String,
    pub organization_name: String,
    pub organization_name_ar: Option<String>,
    pub organization_slug: Option<String>,
    pub organization_code: Option<String>,
    pub country_name: Option<String>,
    pub ministry_name: Option<String>,
    pub region_name: Option<String>,

    // ── Enriched Organization Profile ─────────────────────────
    pub educational_track: Option<String>,
    pub organization_size: Option<String>,
    pub country_code: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub tax_id: Option<String>,
    pub license_number: Option<String>,

    // ── Mulverse Ecosystem ─────────────────────────────────────
    pub mulverse_node_id: Option<String>,

    // ── Localization ──────────────────────────────────────────
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub format_locale: Option<String>,
    pub time_format: Option<String>,
    pub is_hijri_enabled: Option<bool>,
    pub base_currency_code: Option<String>,

    // ── Branding ──────────────────────────────────────────────
    pub primary_color: Option<String>,

    // ── Administrator ─────────────────────────────────────────
    pub admin_first_name: String,
    pub admin_last_name: String,
    pub admin_first_name_ar: Option<String>,
    pub admin_last_name_ar: Option<String>,
    pub admin_email: String,
    pub admin_password: String,
    pub admin_phone: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
// Bootstrap Result
// ═══════════════════════════════════════════════════════════════

/// Strongly-typed result of a successful bootstrap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapResult {
    pub success: bool,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub institution_type: String,
    pub admin_user_id: Uuid,
    pub admin_email: String,
    pub setup_required: bool,
    pub warnings: Vec<String>,
}

/// Status of the bootstrap system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapStatus {
    /// No organization exists — bootstrap is required.
    Required,
    /// Bootstrap is currently in progress (set via in-flight flag).
    InProgress,
    /// Bootstrap completed successfully — organization exists with admin.
    Completed,
}

/// Bootstrap status response for the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapStatusResponse {
    pub status: BootstrapStatus,
    pub organization_id: Option<Uuid>,
    pub organization_name: Option<String>,
    pub institution_type: Option<String>,
    pub has_admin: bool,
}

// ═══════════════════════════════════════════════════════════════
// Bootstrap Service
// ═══════════════════════════════════════════════════════════════

pub struct BootstrapService {
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub user_identity_repo: Arc<dyn UserIdentityRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub role_repo: Arc<dyn RoleRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
    pub system_repo: Arc<dyn SystemRepository>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
    pub account_repo: Arc<dyn AccountRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub event_bus: Arc<dyn EventBus>,
}

impl BootstrapService {
    pub fn new(
        org_repo: Arc<dyn OrganizationRepository>,
        user_repo: Arc<dyn UserRepository>,
        person_repo: Arc<dyn PersonRepository>,
        user_identity_repo: Arc<dyn UserIdentityRepository>,
        group_repo: Arc<dyn GroupRepository>,
        role_repo: Arc<dyn RoleRepository>,
        settings_repo: Arc<dyn SettingsRepository>,
        system_repo: Arc<dyn SystemRepository>,
        currency_repo: Arc<dyn CurrencyRepository>,
        account_repo: Arc<dyn AccountRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            org_repo,
            user_repo,
            person_repo,
            user_identity_repo,
            group_repo,
            role_repo,
            settings_repo,
            system_repo,
            currency_repo,
            account_repo,
            password_hasher,
            event_bus,
        }
    }

    // ─────────────────────────────────────────────────────────
    // Bootstrap Status
    // ─────────────────────────────────────────────────────────

    /// Determine the current bootstrap status by inspecting actual database state.
    pub async fn get_status(&self) -> Result<BootstrapStatusResponse, DomainError> {
        let has_org = self.system_repo.has_organization().await?;

        if !has_org {
            return Ok(BootstrapStatusResponse {
                status: BootstrapStatus::Required,
                organization_id: None,
                organization_name: None,
                institution_type: None,
                has_admin: false,
            });
        }

        let org_id = self.system_repo.get_first_org_id().await?;
        let org_name = self.system_repo.get_org_name().await?;
        let has_admin = self.system_repo.has_admin().await?;

        // Determine institution type from organization metadata
        let institution_type = if let Some(oid) = org_id {
            self.org_repo.find_by_id(oid).await?.and_then(|o| {
                o.metadata
                    .as_ref()
                    .and_then(|m| m.get("institution_type"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
        } else {
            None
        };

        let status = if has_org && has_admin {
            BootstrapStatus::Completed
        } else {
            // Org exists but no admin — partial bootstrap (failure recovery scenario)
            BootstrapStatus::Required
        };

        Ok(BootstrapStatusResponse {
            status,
            organization_id: org_id,
            organization_name: org_name,
            institution_type,
            has_admin,
        })
    }

    // ─────────────────────────────────────────────────────────
    // Execute Bootstrap
    // ─────────────────────────────────────────────────────────

    /// Execute the complete bootstrap sequence.
    ///
    /// This is idempotent: if an organization + admin already exist,
    /// it returns `DomainError::AlreadyInitialized`.
    ///
    /// If only an organization exists (no admin), it will complete the
    /// bootstrap by creating the admin — this handles crash recovery.
    pub async fn execute(&self, req: BootstrapRequest) -> Result<BootstrapResult, DomainError> {
        // ── Step 1: Validate request ──────────────────────────
        self.validate_request(&req)?;

        // ── Step 2: Check idempotency ─────────────────────────
        let has_org = self.system_repo.has_organization().await?;
        let has_admin = self.system_repo.has_admin().await?;

        if has_org && has_admin {
            return Err(DomainError::AlreadyInitialized);
        }

        let mut warnings = Vec::new();

        // ── Step 3: Create or recover organization ────────────
        let org = if has_org {
            // Recovery: organization exists but admin doesn't.
            // Reuse the existing organization.
            warnings.push("Organization already existed — resuming bootstrap".to_string());
            let org_id = self.system_repo.get_first_org_id().await?.ok_or_else(|| {
                DomainError::Internal("Organization flag set but no org found".into())
            })?;
            self.org_repo.find_by_id(org_id).await?.ok_or_else(|| {
                DomainError::Internal("Organization ID exists but record not found".into())
            })?
        } else {
            self.create_organization(&req).await?
        };

        let org_id = org.id;
        let institution_type = InstitutionType::from_str_loose(&req.institution_type);

        // ── Step 4: Initialize organization settings ──────────
        self.initialize_settings(org_id, &institution_type, &req)
            .await?;

        // ── Step 5: Seed system reference currencies ──────────
        self.seed_system_currencies(org_id, &req).await?;

        // ── Step 6: Initialize Chart of Accounts scaffold ─────
        self.initialize_chart_of_accounts(org_id, &req).await?;

        // ── Step 7: Create system roles ───────────────────────
        self.create_system_roles(org_id).await?;

        // ── Step 8: Create system user groups ─────────────────
        self.create_default_user_groups(org_id).await?;

        // ── Step 9: Create initial administrator ──────────────
        let (admin_user, _admin_person) = self.create_administrator(org_id, &req).await?;

        // ── Step 10: Assign admin role ────────────────────────
        self.assign_admin_role(org_id, admin_user.id).await?;

        // ── Step 11: Verify ───────────────────────────────────
        let verified = self.verify_bootstrap(org_id).await?;
        if !verified {
            warnings.push("Bootstrap verification detected incomplete state".to_string());
        }

        Ok(BootstrapResult {
            success: true,
            organization_id: org_id,
            organization_name: org.name.clone(),
            organization_slug: org.slug.clone(),
            institution_type: institution_type.as_str().to_string(),
            admin_user_id: admin_user.id,
            admin_email: req.admin_email.clone(),
            setup_required: true, // Onboarding always required after bootstrap
            warnings,
        })
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Validate
    // ─────────────────────────────────────────────────────────

    fn validate_request(&self, req: &BootstrapRequest) -> Result<(), DomainError> {
        if req.organization_name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Organization name is required".into(),
            ));
        }
        if req.admin_first_name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Administrator first name is required".into(),
            ));
        }
        if req.admin_last_name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Administrator last name is required".into(),
            ));
        }
        if req.admin_email.trim().is_empty() {
            return Err(DomainError::Validation(
                "Administrator email is required".into(),
            ));
        }
        if req.admin_password.len() < 8 {
            return Err(DomainError::Validation(
                "Administrator password must be at least 8 characters".into(),
            ));
        }
        if req.timezone.trim().is_empty() {
            return Err(DomainError::Validation("Timezone is required".into()));
        }
        if req.locale.trim().is_empty() {
            return Err(DomainError::Validation("Locale is required".into()));
        }

        // Validate institution type
        let _inst = InstitutionType::from_str_loose(&req.institution_type);

        // Validate email format (basic)
        if !req.admin_email.contains('@') || !req.admin_email.contains('.') {
            return Err(DomainError::Validation(
                "Invalid administrator email format".into(),
            ));
        }

        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Create Organization
    // ─────────────────────────────────────────────────────────

    async fn create_organization(
        &self,
        req: &BootstrapRequest,
    ) -> Result<Organization, DomainError> {
        // Automatically determine base slug from request, English name, or Arabic name
        let mut target_slug = req
            .organization_slug
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| slugify_string(s))
            .unwrap_or_else(|| {
                generate_smart_slug(&req.organization_name, req.organization_name_ar.as_deref())
            });

        if target_slug.is_empty() {
            target_slug = "institution".to_string();
        }

        // Auto-resolve collisions by appending counter if needed
        let mut final_slug = target_slug.clone();
        let mut counter = 1;
        while self.org_repo.find_by_slug(&final_slug).await?.is_some() {
            counter += 1;
            final_slug = format!("{}-{}", target_slug, counter);
        }

        let institution_type = InstitutionType::from_str_loose(&req.institution_type);
        let now = Utc::now();

        let mut org = Organization::new(
            req.organization_name.clone(),
            final_slug.clone(),
            req.timezone.clone(),
            req.locale.clone(),
        )?;

        // Apply optional fields from bootstrap request
        org.name_ar = req.organization_name_ar.clone();
        let auto_code = req
            .organization_code
            .as_deref()
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .map(|c| c.to_uppercase())
            .unwrap_or_else(|| {
                let prefix: String = final_slug
                    .split('-')
                    .filter_map(|w| w.chars().next())
                    .take(4)
                    .collect::<String>()
                    .to_uppercase();
                if prefix.len() >= 2 {
                    format!("{}-01", prefix)
                } else {
                    format!(
                        "{}-01",
                        final_slug
                            .chars()
                            .take(3)
                            .collect::<String>()
                            .to_uppercase()
                    )
                }
            });
        org.code = Some(auto_code);
        org.date_format = req.date_format.clone().or(Some("YYYY-MM-DD".to_string()));
        org.format_locale = req.format_locale.clone();
        org.time_format = req.time_format.clone();
        org.is_hijri_enabled = req.is_hijri_enabled.unwrap_or(false);
        org.base_currency_code = req.base_currency_code.clone();
        org.primary_color = req.primary_color.clone().or(Some("#4f46e5".to_string()));
        org.is_verified = true; // Bootstrap-created org is verified
        org.metadata = Some(serde_json::json!({
            "institution_type": institution_type.as_str(),
            "educational_track": req.educational_track,
            "organization_size": req.organization_size,
            "country_code": req.country_code,
            "country_name": req.country_name,
            "ministry_name": req.ministry_name,
            "region_name": req.region_name,
            "city": req.city,
            "phone": req.phone,
            "website": req.website,
            "tax_id": req.tax_id,
            "license_number": req.license_number,
            "mulverse_node_id": req.mulverse_node_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string()),
            "ecosystem": "Mulverse",
            "bootstrapped_at": now.to_rfc3339(),
            "bootstrap_version": "2.0",
        }));
        org.created_at = now;
        org.updated_at = now;

        self.org_repo.save(&org).await?;

        Ok(org)
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Initialize Settings
    // ─────────────────────────────────────────────────────────

    async fn initialize_settings(
        &self,
        org_id: Uuid,
        institution_type: &InstitutionType,
        req: &BootstrapRequest,
    ) -> Result<(), DomainError> {
        let mut settings = OrganizationSettings::default();

        // 1. Academic Policies & Evaluation Defaults
        match institution_type {
            InstitutionType::School => {
                settings.academic.default_grading_system = "percentage".to_string();
                settings.academic.default_passing_score = Decimal::from(50);
                settings.academic.default_max_score = Decimal::from(100);
                settings.academic.passing_threshold_percentage = Decimal::from(50);
                settings.academic.honors_threshold_percentage = Decimal::from(85);
                settings.academic.minimum_final_exam_score = Some(Decimal::from(10));
                settings.academic.max_failed_subjects_for_conditional_pass = 2;
                settings.academic.min_attendance_rate = Decimal::from(75);
                settings.academic.ranking_enabled = true;
                settings.academic.max_displayed_rank = None;
                settings.academic.ranking_scope = RankingScope::Class;
                settings.academic.tie_policy = TiePolicy::StandardCompetition;
                settings.academic.rounding_mode = RoundingMode::RoundHalfUp;
                settings.academic.decimal_places = 2;
                settings.enrollment.max_class_size_default = 30;
                settings.enrollment.allow_late_enrollment = false;
            }
            InstitutionType::University => {
                settings.academic.default_grading_system = "gpa".to_string();
                settings.academic.default_passing_score = Decimal::from(60);
                settings.academic.default_max_score = Decimal::from(100);
                settings.academic.passing_threshold_percentage = Decimal::from(60);
                settings.academic.honors_threshold_percentage = Decimal::from(90);
                settings.academic.minimum_final_exam_score = Some(Decimal::from(15));
                settings.academic.max_failed_subjects_for_conditional_pass = 1;
                settings.academic.min_attendance_rate = Decimal::from(75);
                settings.academic.ranking_enabled = true;
                settings.academic.max_displayed_rank = None;
                settings.academic.ranking_scope = RankingScope::Cohort;
                settings.academic.tie_policy = TiePolicy::StandardCompetition;
                settings.academic.rounding_mode = RoundingMode::RoundHalfUp;
                settings.academic.decimal_places = 2;
                settings.enrollment.max_class_size_default = 50;
                settings.enrollment.allow_late_enrollment = true;
            }
            InstitutionType::Institute => {
                settings.academic.default_grading_system = "percentage".to_string();
                settings.academic.default_passing_score = Decimal::from(50);
                settings.academic.default_max_score = Decimal::from(100);
                settings.academic.passing_threshold_percentage = Decimal::from(50);
                settings.academic.honors_threshold_percentage = Decimal::from(85);
                settings.academic.minimum_final_exam_score = Some(Decimal::from(10));
                settings.academic.max_failed_subjects_for_conditional_pass = 2;
                settings.academic.min_attendance_rate = Decimal::from(80);
                settings.academic.ranking_enabled = true;
                settings.academic.max_displayed_rank = None;
                settings.academic.ranking_scope = RankingScope::Class;
                settings.academic.tie_policy = TiePolicy::StandardCompetition;
                settings.academic.rounding_mode = RoundingMode::RoundHalfUp;
                settings.academic.decimal_places = 2;
                settings.enrollment.max_class_size_default = 25;
                settings.enrollment.allow_late_enrollment = false;
            }
        }
        settings.academic.enrollment_auto_close = true;
        settings.academic.late_enrollment_approval = true;
        settings.academic.grade_entry_auto_close = true;
        settings.academic.allow_grade_edit_after_close = false;
        settings.academic.grade_edit_approval = true;
        settings.academic.auto_graduate_final_level = false;
        settings.academic.exclude_withdrawn = true;

        // 2. Enrollment Policies
        settings.enrollment.registration_enabled = true;
        settings.enrollment.require_admin_approval = true;
        settings.enrollment.allow_waitlist = true;
        settings.enrollment.sibling_priority_enabled = true;

        // 3. Admission Configuration (Numbering Schema, Admission Policy, and Standard Form Fields)
        let prefix = match institution_type {
            InstitutionType::University => "UNI",
            InstitutionType::Institute => "INS",
            _ => "STD",
        };
        settings.admission.numbering = NumberingSchema {
            prefix: prefix.to_string(),
            year_format: YearFormat::Full,
            sequence_digits: 4,
            separator: "-".to_string(),
        };
        settings.admission.policy = AdmissionPolicy {
            requires_approval: true,
            allow_no_guardian: false,
            allow_duplicate_national_id: false,
            auto_assign_fees: true,
            max_class_capacity: settings.enrollment.max_class_size_default,
            allow_late_enrollment: settings.enrollment.allow_late_enrollment,
            allow_waitlist: true,
            sibling_priority_enabled: true,
        };
        settings.admission.fields = vec![
            FormFieldConfig {
                key: "national_id".into(),
                name_ar: "رقم الهوية / الإقامة".into(),
                name_en: "National ID / Iqama".into(),
                is_active: true,
                is_required: true,
                prevent_duplicate: true,
                supports_duplicate: true,
            },
            FormFieldConfig {
                key: "phone".into(),
                name_ar: "رقم الهاتف".into(),
                name_en: "Phone Number".into(),
                is_active: true,
                is_required: true,
                prevent_duplicate: false,
                supports_duplicate: true,
            },
            FormFieldConfig {
                key: "email".into(),
                name_ar: "البريد الإلكتروني".into(),
                name_en: "Email Address".into(),
                is_active: true,
                is_required: false,
                prevent_duplicate: false,
                supports_duplicate: true,
            },
            FormFieldConfig {
                key: "birth_date".into(),
                name_ar: "تاريخ الميلاد".into(),
                name_en: "Date of Birth".into(),
                is_active: true,
                is_required: true,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "sex".into(),
                name_ar: "الجنس".into(),
                name_en: "Gender".into(),
                is_active: true,
                is_required: true,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "nationality".into(),
                name_ar: "الجنسية".into(),
                name_en: "Nationality".into(),
                is_active: true,
                is_required: true,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "address".into(),
                name_ar: "العنوان السكني".into(),
                name_en: "Residential Address".into(),
                is_active: true,
                is_required: false,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "place_of_birth".into(),
                name_ar: "مكان الميلاد".into(),
                name_en: "Place of Birth".into(),
                is_active: true,
                is_required: false,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "blood_type".into(),
                name_ar: "فصيلة الدم".into(),
                name_en: "Blood Type".into(),
                is_active: true,
                is_required: false,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "emergency_contact".into(),
                name_ar: "رقم الطوارئ".into(),
                name_en: "Emergency Contact".into(),
                is_active: true,
                is_required: true,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "medical_notes".into(),
                name_ar: "ملاحظات طبية".into(),
                name_en: "Medical Notes".into(),
                is_active: true,
                is_required: false,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
            FormFieldConfig {
                key: "previous_school".into(),
                name_ar: "المدرسة / الجهة السابقة".into(),
                name_en: "Previous School / Institution".into(),
                is_active: true,
                is_required: false,
                prevent_duplicate: false,
                supports_duplicate: false,
            },
        ];

        // 4. Financial Policies
        settings.finance.currency = req
            .base_currency_code
            .as_deref()
            .unwrap_or("SAR")
            .to_uppercase();
        settings.finance.tax_rate = 0.15;
        settings.finance.invoice_due_days = 14;
        settings.finance.late_fee_type = "fixed".to_string();
        settings.finance.late_fee_value = 0.0;
        settings.finance.allow_partial_payments = true;

        // 5. Notifications
        settings.notifications.notify_parents_on_absence = true;
        settings.notifications.notify_parents_on_grade_publish = true;
        settings.notifications.finance_reminder_days = vec![7];

        // 6. Autopilot Defaults
        settings.autopilot.enabled = false;
        settings.autopilot.mode = "assisted".to_string();
        settings.autopilot.grace_days = 3;
        settings.autopilot.notify_before_deadline_hours = 48;
        settings.autopilot.max_retries = 3;

        self.settings_repo.save_settings(org_id, &settings).await?;
        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Seed System Currencies
    // ─────────────────────────────────────────────────────────

    /// Seed standard reference currencies (SAR, USD, AED, YER, EUR, GBP, KWD, BHD, QAR, OMR).
    async fn seed_system_currencies(
        &self,
        org_id: Uuid,
        req: &BootstrapRequest,
    ) -> Result<(), DomainError> {
        let now = Utc::now();
        let base_code = req
            .base_currency_code
            .as_deref()
            .unwrap_or("SAR")
            .to_uppercase();

        let standard_currencies = [
            ("SAR", "Saudi Riyal", Some("ريال سعودي"), "ر.س", "after", 2),
            ("USD", "US Dollar", Some("دولار أمريكي"), "$", "before", 2),
            ("AED", "UAE Dirham", Some("درهم إماراتي"), "د.إ", "after", 2),
            ("YER", "Yemeni Rial", Some("ريال يمني"), "ر.ي", "after", 2),
            ("EUR", "Euro", Some("يورو"), "€", "after", 2),
            (
                "GBP",
                "British Pound",
                Some("جنيه إسترليني"),
                "£",
                "before",
                2,
            ),
            (
                "KWD",
                "Kuwaiti Dinar",
                Some("دينار كويتي"),
                "د.ك",
                "after",
                3,
            ),
            (
                "BHD",
                "Bahraini Dinar",
                Some("دينار بحريني"),
                "د.ب",
                "after",
                3,
            ),
            ("QAR", "Qatari Riyal", Some("ريال قطري"), "ر.ق", "after", 2),
            ("OMR", "Omani Rial", Some("ريال عماني"), "ر.ع", "after", 3),
        ];

        for (code, name, name_ar, symbol, pos, dec) in standard_currencies {
            let is_base = code == base_code;
            let currency = Currency {
                code: code.to_string(),
                org_id,
                name: name.to_string(),
                name_ar: name_ar.map(|s| s.to_string()),
                symbol: symbol.to_string(),
                symbol_position: pos.to_string(),
                decimal_places: dec,
                is_base,
                is_active: true,
                updated_at: now,
            };
            self.currency_repo.save(&currency).await?;
        }

        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Initialize Chart of Accounts
    // ─────────────────────────────────────────────────────────

    /// Initialize standard double-entry Chart of Accounts scaffold from institution blueprint.
    async fn initialize_chart_of_accounts(
        &self,
        org_id: Uuid,
        req: &BootstrapRequest,
    ) -> Result<(), DomainError> {
        let currency_code = req
            .base_currency_code
            .clone()
            .unwrap_or_else(|| "SAR".to_string());

        let accounts =
            match CoaBlueprintLoader::parse_accounts(org_id, &currency_code, &req.institution_type)
            {
                Ok(accs) => accs,
                Err(e) => {
                    tracing::error!(
                        "Failed to parse COA blueprint for {}: {}",
                        req.institution_type,
                        e
                    );
                    return Err(DomainError::Internal(format!(
                        "Failed to parse COA blueprint: {}",
                        e
                    )));
                }
            };

        for account in accounts {
            if self
                .account_repo
                .find_by_code(org_id, &account.code)
                .await?
                .is_none()
            {
                self.account_repo.save(&account).await?;
            }
        }

        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Create System Roles
    // ─────────────────────────────────────────────────────────

    /// Create the required system roles for the organization.
    /// Idempotent — skips roles that already exist.
    async fn create_system_roles(&self, org_id: Uuid) -> Result<(), DomainError> {
        let now = Utc::now();

        let system_roles = vec![
            ("super_admin", "Super Administrator", Some("مدير النظام")),
            (
                "org_admin",
                "Organization Administrator",
                Some("مدير المؤسسة"),
            ),
            ("principal", "Principal", Some("المدير")),
            ("teacher", "Teacher", Some("معلم")),
            ("guardian", "Guardian", Some("ولي أمر")),
            ("student", "Student", Some("طالب")),
            ("accountant", "Accountant", Some("محاسب")),
            ("registrar", "Registrar", Some("أمين القبول والتسجيل")),
        ];

        for (role_id, name, name_ar) in system_roles {
            let role = Role {
                id: role_id.to_string(),
                org_id,
                name: name.to_string(),
                name_ar: name_ar.map(|s| s.to_string()),
                is_system: true,
                created_at: now,
                updated_at: now,
            };

            self.role_repo.save_role(&role).await?;
        }

        // Create admin permissions for super_admin and org_admin roles
        self.create_admin_permissions(org_id, "super_admin").await?;
        self.create_admin_permissions(org_id, "org_admin").await?;
        Ok(())
    }

    async fn create_default_user_groups(&self, org_id: Uuid) -> Result<(), DomainError> {
        let now = Utc::now();
        let groups = [
            (
                "Organization Administrators",
                "مديرو المؤسسة",
                "org_admin",
                vec![],
            ),
            (
                "Principals",
                "المديرون",
                "principal",
                vec![
                    "organization:read",
                    "period:read",
                    "period:write",
                    "period:activate",
                    "org_unit:read",
                    "class:read",
                    "class:write",
                    "subject:read",
                    "subject:write",
                    "schedule:read",
                    "schedule:write",
                    "assessment:read",
                    "assessment:write",
                    "grade:read",
                    "grade:write",
                    "grade:entry_manage",
                    "grade:analytics",
                    "grade:report",
                    "student:read",
                    "student:create",
                    "student:update",
                    "student:write",
                    "guardian:read",
                    "guardian:write",
                    "enrollment:read",
                    "enrollment:write",
                    "enrollment:promote",
                    "enrollment:transfer",
                    "workflow:read",
                    "workflow:write",
                    "workflow:approve",
                    "attendance:read",
                    "attendance:write",
                    "attendance:analytics",
                    "attendance:report",
                    "behavior:read",
                    "behavior:write",
                    "document:read",
                    "document:write",
                    "template:read",
                    "template:write",
                    "template:import",
                    "template:export",
                    "exam_session:read",
                    "exam_session:write",
                    "seating_number:read",
                    "secret_number:read",
                    "exam_hall:read",
                    "exam_hall:write",
                    "proctor:read",
                    "proctor:assign",
                    "student_order:read",
                    "exam_schedule:read",
                    "exam_schedule:write",
                    "notification:read",
                    "notification:write",
                    "dashboard:read",
                    "audit:read",
                    "data:import",
                    "data:export",
                    "promotion:read",
                    "promotion:write",
                    "promotion:execute",
                    "employee:read",
                    "employee:write",
                    "employee:create",
                    "employee:update",
                    "group:read",
                ],
            ),
            (
                "Teachers",
                "المعلمون",
                "teacher",
                vec![
                    "class:read",
                    "subject:read",
                    "schedule:read",
                    "assessment:read",
                    "grade:read",
                    "grade:write",
                    "grade:entry_manage",
                    "student:read",
                    "enrollment:read",
                    "attendance:read",
                    "attendance:write",
                    "behavior:read",
                    "behavior:write",
                    "document:read",
                    "exam_session:read",
                    "seating_number:read",
                    "secret_number:read",
                    "exam_hall:read",
                    "proctor:read",
                    "exam_schedule:read",
                    "notification:read",
                    "dashboard:read",
                ],
            ),
            (
                "Accountants",
                "المحاسبون",
                "accountant",
                vec![
                    "finance:read",
                    "finance:write",
                    "finance:payment",
                    "student:read",
                    "enrollment:read",
                    "dashboard:read",
                    "notification:read",
                ],
            ),
            (
                "Staff",
                "الموظفون",
                "staff",
                vec![
                    "class:read",
                    "subject:read",
                    "schedule:read",
                    "attendance:read",
                    "document:read",
                    "notification:read",
                    "dashboard:read",
                ],
            ),
            (
                "Registrars",
                "موظفو القبول والتسجيل",
                "registrar",
                vec![
                    "period:read",
                    "class:read",
                    "subject:read",
                    "student:read",
                    "student:create",
                    "student:update",
                    "guardian:read",
                    "guardian:write",
                    "enrollment:read",
                    "enrollment:write",
                    "enrollment:promote",
                    "enrollment:transfer",
                    "document:read",
                    "document:write",
                    "template:read",
                    "template:import",
                    "template:export",
                    "data:import",
                    "data:export",
                    "notification:read",
                    "dashboard:read",
                ],
            ),
            (
                "Guardians",
                "أولياء الأمور",
                "guardian",
                vec!["dashboard:read", "notification:read"],
            ),
            (
                "Students",
                "الطلاب",
                "student",
                vec!["dashboard:read", "notification:read"],
            ),
        ];

        for (name, name_ar, role_id, configured_permissions) in groups {
            let existing = self.group_repo.find_by_name(org_id, name).await?;
            let mut group = if let Some(group) = existing {
                if !group.is_system {
                    return Err(DomainError::Conflict(format!(
                        "A non-system group already uses the reserved default name '{}'",
                        name
                    )));
                }
                group
            } else {
                Group {
                    id: Uuid::new_v4(),
                    org_id,
                    name: name.to_string(),
                    name_ar: Some(name_ar.to_string()),
                    description: Some(format!("Default permissions for the {} role", role_id)),
                    is_system: true,
                    status: "active".to_string(),
                    created_by: None,
                    updated_by: None,
                    created_at: now,
                    updated_at: now,
                }
            };

            group.name_ar = Some(name_ar.to_string());
            group.description = Some(format!(
                "System-managed default permission group for {}",
                role_id
            ));
            group.status = "active".to_string();
            group.updated_at = now;
            self.group_repo.save(&group).await?;
            let permissions: Vec<String> = if role_id == "org_admin" {
                default_org_admin_group_permissions()
            } else {
                configured_permissions
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            };
            self.group_repo
                .set_group_permissions(org_id, group.id, &permissions)
                .await?;
        }

        Ok(())
    }

    /// Assign all permissions to an admin role.
    /// Idempotent — replaces existing permissions.
    async fn create_admin_permissions(
        &self,
        org_id: Uuid,
        role_id: &str,
    ) -> Result<(), DomainError> {
        let now = Utc::now();

        let mut perms = Vec::new();
        for (resource, action) in all_admin_permission_pairs() {
            perms.push(RolePermission {
                id: Uuid::new_v4(),
                role_id: role_id.to_string(),
                org_id,
                resource: resource.to_string(),
                action: action.to_string(),
                condition: None,
                created_at: now,
            });
        }

        self.role_repo
            .save_role_permissions(org_id, role_id, &perms)
            .await?;

        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Create Administrator
    // ─────────────────────────────────────────────────────────

    async fn create_administrator(
        &self,
        org_id: Uuid,
        req: &BootstrapRequest,
    ) -> Result<(User, Person), DomainError> {
        // Check if admin email already exists (idempotency)
        if let Some(existing_user) = self.user_repo.find_by_email(&req.admin_email).await? {
            // Admin already exists — return existing (recovery scenario)
            let person = self
                .person_repo
                .find_by_id(existing_user.person_id)
                .await?
                .ok_or_else(|| {
                    DomainError::Internal("Admin user exists but person record missing".into())
                })?;
            return Ok((existing_user, person));
        }

        let now = Utc::now();
        let person_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Create Person record
        let person = Person {
            id: person_id,
            org_id: Some(org_id),
            first_name: req.admin_first_name.clone(),
            middle_name: None,
            last_name: req.admin_last_name.clone(),
            first_name_ar: req.admin_first_name_ar.clone(),
            middle_name_ar: None,
            last_name_ar: req.admin_last_name_ar.clone(),
            display_name: None,
            national_id: None,
            passport_number: None,
            gender: None,
            birth_date: None,
            place_of_birth: None,
            nationality: None,
            blood_type: None,
            marital_status: None,
            medical_notes: None,
            special_needs_notes: None,
            phone: req.admin_phone.clone(),
            email: Some(req.admin_email.clone()),
            address: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        };
        self.person_repo.save(&person).await?;

        // Hash password
        let password_hash = self.password_hasher.hash(&req.admin_password).await?;

        // Create User record
        let user = User {
            id: user_id,
            person_id,
            org_id,
            email: Some(req.admin_email.clone()),
            is_email_verified: true, // Bootstrap admin is pre-verified
            phone: req.admin_phone.clone(),
            is_phone_verified: false,
            password_hash: Some(password_hash),
            is_mfa_enabled: false,
            status: "active".to_string(),
            login_count: 0,
            failed_login_attempts: 0,
            locked_until: None,
            is_deleted: false,
            created_by: None, // System-created
            updated_by: None,
            created_at: now,
            updated_at: now,
        };
        self.user_repo.save(&user).await?;

        // Create UserIdentity for password authentication
        let identity = UserIdentity {
            id: Uuid::new_v4(),
            user_id,
            provider: IdentityProvider::Password,
            provider_user_id: req.admin_email.clone(),
            email: Some(req.admin_email.clone()),
            is_primary: true,
            profile_data: None,
            connected_at: now,
            last_used_at: None,
        };
        self.user_identity_repo.save(&identity).await?;

        Ok((user, person))
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Assign Admin Role
    // ─────────────────────────────────────────────────────────

    async fn assign_admin_role(&self, org_id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        let now = Utc::now();

        // Assign super_admin role
        let assignment = UserRoleAssignment {
            id: Uuid::new_v4(),
            user_id,
            role_id: "super_admin".to_string(),
            org_id,
            scope_type: "organization".to_string(),
            scope_id: None,
            assigned_by: Some("system:bootstrap".to_string()),
            created_at: now,
        };
        self.role_repo.assign_role(&assignment).await?;

        // Also assign org_admin role
        let org_admin = UserRoleAssignment {
            id: Uuid::new_v4(),
            user_id,
            role_id: "org_admin".to_string(),
            org_id,
            scope_type: "organization".to_string(),
            scope_id: None,
            assigned_by: Some("system:bootstrap".to_string()),
            created_at: now,
        };
        self.role_repo.assign_role(&org_admin).await?;

        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    // Internal: Verify
    // ─────────────────────────────────────────────────────────

    /// Verify that bootstrap created everything required.
    async fn verify_bootstrap(&self, org_id: Uuid) -> Result<bool, DomainError> {
        let has_org = self.org_repo.find_by_id(org_id).await?.is_some();
        let has_admin = self.system_repo.has_admin().await?;

        // Verify settings exist
        let settings = self.settings_repo.get_settings(org_id).await;
        let has_settings = settings.is_ok();

        // Verify at least one role exists
        let role_count = self.role_repo.count_by_org(org_id).await?;

        Ok(has_org && has_admin && has_settings && role_count > 0)
    }
}

// ═══════════════════════════════════════════════════════════════
// Smart Transliteration & Slugification Helpers
// ═══════════════════════════════════════════════════════════════

/// Transliterates Arabic and English text to a clean URL-safe slug.
pub fn generate_smart_slug(input_en: &str, input_ar: Option<&str>) -> String {
    let base_text = if !input_en.trim().is_empty() {
        input_en.trim().to_string()
    } else if let Some(ar) = input_ar {
        transliterate_arabic(ar)
    } else {
        String::new()
    };

    if base_text.is_empty() {
        return "organization".to_string();
    }

    slugify_string(&base_text)
}

/// Slugify a string into lowercase kebab-case.
pub fn slugify_string(text: &str) -> String {
    let mut result = String::new();
    let mut last_was_dash = false;

    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            last_was_dash = false;
        } else if c == ' ' || c == '-' || c == '_' || c == '.' || c == '/' {
            if !last_was_dash && !result.is_empty() {
                result.push('-');
                last_was_dash = true;
            }
        }
    }

    while result.ends_with('-') {
        result.pop();
    }

    if result.is_empty() {
        "institution".to_string()
    } else {
        result
    }
}

/// Convert Arabic text into clean English phonetic words.
pub fn transliterate_arabic(text: &str) -> String {
    let mut cleaned = text.to_string();

    let replacements = [
        ("مدارس", "madaris"),
        ("مدرسة", "school"),
        ("أكاديمية", "academy"),
        ("اكاديمية", "academy"),
        ("جامعة", "university"),
        ("كلية", "college"),
        ("معهد", "institute"),
        ("مركز", "center"),
        ("الأهلية", "al-ahliya"),
        ("الاهلية", "al-ahliya"),
        ("العالمية", "international"),
        ("الدولية", "international"),
        ("النموذجية", "model"),
        ("الخاصة", "private"),
        ("المستقبل", "future"),
        ("النور", "al-noor"),
        ("الرواد", "al-ruwwad"),
        ("البيان", "al-bayan"),
        ("المجد", "al-majd"),
        ("الإبداع", "al-ibda"),
        ("التميز", "al-tamayuz"),
        ("الأندلس", "al-andalus"),
        ("النهضة", "al-nahda"),
        ("الأمل", "al-amal"),
        ("الريادة", "al-riyadah"),
        ("الفضيلة", "al-fadeela"),
        ("الصفوة", "al-safwa"),
        ("الحكمة", "al-hikma"),
        ("الفرقان", "al-furqan"),
        ("الرياض", "riyadh"),
        ("جدة", "jeddah"),
        ("عدن", "aden"),
        ("صنعاء", "sanaa"),
        ("دبي", "dubai"),
        ("القاهرة", "cairo"),
    ];

    for (ar_word, en_word) in replacements {
        cleaned = cleaned.replace(ar_word, &format!(" {} ", en_word));
    }

    let mut out = String::new();
    for c in cleaned.chars() {
        match c {
            'ا' | 'أ' | 'إ' | 'آ' | 'ء' | 'ئ' | 'ؤ' => out.push('a'),
            'ب' => out.push('b'),
            'ت' | 'ة' => out.push('t'),
            'ث' => out.push_str("th"),
            'ج' => out.push('j'),
            'ح' => out.push('h'),
            'خ' => out.push_str("kh"),
            'د' => out.push('d'),
            'ذ' => out.push_str("dh"),
            'ر' => out.push('r'),
            'ز' => out.push('z'),
            'س' => out.push('s'),
            'ش' => out.push_str("sh"),
            'ص' => out.push('s'),
            'ض' => out.push('d'),
            'ط' => out.push('t'),
            'ظ' => out.push('z'),
            'ع' => out.push('a'),
            'غ' => out.push_str("gh"),
            'ف' => out.push('f'),
            'ق' => out.push('q'),
            'ك' => out.push('k'),
            'ل' => out.push('l'),
            'م' => out.push('m'),
            'ن' => out.push('n'),
            'ه' => out.push('h'),
            'و' => out.push('w'),
            'ي' | 'ى' => out.push('y'),
            '٠' => out.push('0'),
            '١' => out.push('1'),
            '٢' => out.push('2'),
            '٣' => out.push('3'),
            '٤' => out.push('4'),
            '٥' => out.push('5'),
            '٦' => out.push('6'),
            '٧' => out.push('7'),
            '٨' => out.push('8'),
            '٩' => out.push('9'),
            'َ' | 'ً' | 'ُ' | 'ٌ' | 'ِ' | 'ٍ' | 'ْ' | 'ّ' => {}
            other => out.push(other),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::default_org_admin_group_permissions;

    #[test]
    fn default_admin_group_excludes_system_bypass() {
        let permissions = default_org_admin_group_permissions();

        assert!(permissions.contains(&"group:assign".to_string()));
        assert!(permissions.contains(&"finance:payment".to_string()));
        assert!(!permissions.contains(&"system:autopilot".to_string()));
    }
}
