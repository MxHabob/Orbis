use chrono::Utc;
use orbis_domain::entities::device::{DeviceType, UserDevice};
use orbis_domain::entities::identity::{IdentityProvider, UserIdentity};
use orbis_domain::entities::mfa::{MfaEnrollmentResult, MfaFactor, MfaRecoveryCode, MfaType};
use orbis_domain::entities::person::Person;
use orbis_domain::entities::session::Session;
use orbis_domain::entities::user::User;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AcademicPeriodRepository, EmployeeRepository, EnrollmentRepository, IntegrationRepository,
    MfaFactorRepository, MfaRecoveryCodeRepository, PersonRepository, SectionRepository,
    SectionSubjectRepository, SessionRepository, StudentGuardianRepository, StudentRepository,
    UserDeviceRepository, UserIdentityRepository, UserRepository,
};
use crate::ports::security::{OidcService, PasswordHasher, TokenService, TotpService};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedDependentDto {
    pub student_id: Uuid,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub relationship: String,
    pub is_primary_contact: bool,
    pub is_emergency_contact: bool,
    pub is_fee_payer: bool,
    pub current_grade_level: Option<String>,
    pub current_section_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveContextDto {
    pub effective_role: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub active_period_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub dependents: Vec<LinkedDependentDto>,
    pub teaching_classes: Vec<TeachingClassDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingClassDto {
    pub class_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub user: User,
    pub person: Option<Person>,
    pub effective_context: Option<EffectiveContextDto>,
    pub mfa_required: bool,
    pub mfa_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResponse {
    pub token: String,
    pub user: User,
    pub person: Option<Person>,
    pub effective_context: EffectiveContextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedSessionResponse {
    pub token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleLoginResult {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub mfa_required: bool,
    pub mfa_token: Option<String>,
}

pub struct AuthService {
    pub user_repo: Arc<dyn UserRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub mfa_factor_repo: Arc<dyn MfaFactorRepository>,
    pub mfa_recovery_code_repo: Arc<dyn MfaRecoveryCodeRepository>,
    pub user_device_repo: Arc<dyn UserDeviceRepository>,
    pub user_identity_repo: Arc<dyn UserIdentityRepository>,
    pub token_service: Arc<dyn TokenService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub totp_service: Arc<dyn TotpService>,
    pub oidc_service: Arc<dyn OidcService>,
    pub integration_repo: Option<Arc<dyn IntegrationRepository>>,
    pub academic_period_repo: Option<Arc<dyn AcademicPeriodRepository>>,
    pub student_guardian_repo: Option<Arc<dyn StudentGuardianRepository>>,
    pub student_repo: Option<Arc<dyn StudentRepository>>,
    pub enrollment_repo: Option<Arc<dyn EnrollmentRepository>>,
    pub employee_repo: Option<Arc<dyn EmployeeRepository>>,
    pub section_subject_repo: Option<Arc<dyn SectionSubjectRepository>>,
    pub section_repo: Option<Arc<dyn SectionRepository>>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        person_repo: Arc<dyn PersonRepository>,
        session_repo: Arc<dyn SessionRepository>,
        mfa_factor_repo: Arc<dyn MfaFactorRepository>,
        mfa_recovery_code_repo: Arc<dyn MfaRecoveryCodeRepository>,
        user_device_repo: Arc<dyn UserDeviceRepository>,
        user_identity_repo: Arc<dyn UserIdentityRepository>,
        token_service: Arc<dyn TokenService>,
        password_hasher: Arc<dyn PasswordHasher>,
        totp_service: Arc<dyn TotpService>,
        oidc_service: Arc<dyn OidcService>,
        integration_repo: Option<Arc<dyn IntegrationRepository>>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            user_repo,
            person_repo,
            session_repo,
            mfa_factor_repo,
            mfa_recovery_code_repo,
            user_device_repo,
            user_identity_repo,
            token_service,
            password_hasher,
            totp_service,
            oidc_service,
            integration_repo,
            academic_period_repo: None,
            student_guardian_repo: None,
            student_repo: None,
            enrollment_repo: None,
            employee_repo: None,
            section_subject_repo: None,
            section_repo: None,
            event_bus,
            permission_checker,
        }
    }

    pub fn with_context_repos(
        mut self,
        academic_period_repo: Option<Arc<dyn AcademicPeriodRepository>>,
        student_guardian_repo: Option<Arc<dyn StudentGuardianRepository>>,
        student_repo: Option<Arc<dyn StudentRepository>>,
        enrollment_repo: Option<Arc<dyn EnrollmentRepository>>,
    ) -> Self {
        self.academic_period_repo = academic_period_repo;
        self.student_guardian_repo = student_guardian_repo;
        self.student_repo = student_repo;
        self.enrollment_repo = enrollment_repo;
        self
    }

    pub fn with_teaching_scope_repos(
        mut self,
        employee_repo: Arc<dyn EmployeeRepository>,
        section_subject_repo: Arc<dyn SectionSubjectRepository>,
        section_repo: Arc<dyn SectionRepository>,
    ) -> Self {
        self.employee_repo = Some(employee_repo);
        self.section_subject_repo = Some(section_subject_repo);
        self.section_repo = Some(section_repo);
        self
    }

    /// Build effective context (roles, permissions, active period, linked dependents)
    pub async fn build_effective_context(
        &self,
        user: &User,
        person: Option<&Person>,
    ) -> Result<EffectiveContextDto, DomainError> {
        let permissions = self
            .permission_checker
            .get_user_permissions(user.id, user.org_id)
            .await?;
        let roles = self
            .permission_checker
            .get_user_roles(user.id, user.org_id)
            .await?;

        let effective_role = if roles.contains(&"super_admin".to_string()) {
            "super_admin".to_string()
        } else if roles.contains(&"org_admin".to_string()) {
            "org_admin".to_string()
        } else if roles.contains(&"principal".to_string()) {
            "principal".to_string()
        } else if roles.contains(&"teacher".to_string()) {
            "teacher".to_string()
        } else if roles.contains(&"guardian".to_string()) {
            "guardian".to_string()
        } else if roles.contains(&"student".to_string()) {
            "student".to_string()
        } else if let Some(first) = roles.first() {
            first.clone()
        } else {
            "guest".to_string()
        };

        let active_period_id = if let Some(ref pr) = self.academic_period_repo {
            pr.find_by_org(user.org_id)
                .await?
                .into_iter()
                .find(|p| {
                    matches!(
                        p.state,
                        orbis_domain::entities::academic_period::AcademicPeriodState::ActiveTeaching
                            | orbis_domain::entities::academic_period::AcademicPeriodState::EnrollmentOpen
                    )
                })
                .map(|p| p.id)
        } else {
            None
        };

        let mut dependents = Vec::new();
        if let (Some(person), Some(gr), Some(sr)) = (
            person,
            self.student_guardian_repo.as_ref(),
            self.student_repo.as_ref(),
        ) {
            for guardianship in gr.find_by_guardian(person.id).await? {
                if let Some(student) = sr.find_by_id(guardianship.student_id).await? {
                    let student_person = self.person_repo.find_by_id(student.person_id).await?;
                    let (full_name, full_name_ar) = student_person
                        .map(|student_person| {
                            (student_person.full_name(), student_person.full_name_ar())
                        })
                        .unwrap_or_else(|| (format!("Student {}", student.student_number), None));

                    dependents.push(LinkedDependentDto {
                        student_id: student.id,
                        full_name,
                        full_name_ar,
                        relationship: guardianship.relationship,
                        is_primary_contact: guardianship.is_primary,
                        is_emergency_contact: guardianship.is_emergency_contact,
                        is_fee_payer: guardianship.is_payer,
                        current_grade_level: None,
                        current_section_name: None,
                    });
                }
            }
        }

        let student_id = if roles.iter().any(|role| role == "student") {
            if let Some(student_repo) = self.student_repo.as_ref() {
                student_repo
                    .find_by_user_id(user.org_id, user.id)
                    .await?
                    .map(|student| student.id)
            } else {
                None
            }
        } else {
            None
        };

        let mut teaching_classes = Vec::new();
        if roles.iter().any(|role| role == "teacher") {
            if let (
                Some(period_id),
                Some(employee_repo),
                Some(section_subject_repo),
                Some(section_repo),
            ) = (
                active_period_id,
                self.employee_repo.as_ref(),
                self.section_subject_repo.as_ref(),
                self.section_repo.as_ref(),
            ) {
                if let Some(employee) = employee_repo.find_by_user_id(user.org_id, user.id).await? {
                    let assignments = section_subject_repo
                        .find_by_teacher(employee.id, period_id)
                        .await?;
                    let mut seen_class_ids = std::collections::HashSet::new();
                    for assignment in assignments.into_iter().filter(|item| item.is_active) {
                        if !seen_class_ids.insert(assignment.section_id) {
                            continue;
                        }
                        if let Some(section) =
                            section_repo.find_by_id(assignment.section_id).await?
                        {
                            if section.org_id == user.org_id
                                && section.is_active
                                && !section.is_deleted
                            {
                                teaching_classes.push(TeachingClassDto {
                                    class_id: section.id,
                                    name: section.name,
                                    name_ar: section.name_ar,
                                    level: section.level,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(EffectiveContextDto {
            effective_role,
            roles,
            permissions,
            active_period_id,
            student_id,
            dependents,
            teaching_classes,
        })
    }

    /// Authenticate user via email and password, creating session and JWT tokens
    pub async fn login(
        &self,
        ctx: &RequestContext,
        email: &str,
        password: &str,
    ) -> Result<LoginResponse, DomainError> {
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid email or password".into()))?;

        let hash = user.password_hash.as_ref().ok_or_else(|| {
            DomainError::Unauthorized("Password authentication not supported for this user".into())
        })?;

        let is_valid = self.password_hasher.verify(password, hash).await?;
        if !is_valid {
            return Err(DomainError::Unauthorized(
                "Invalid email or password".into(),
            ));
        }

        let person = self.person_repo.find_by_id(user.person_id).await?;
        let effective_context = self.build_effective_context(&user, person.as_ref()).await?;

        let user_id_str = user.id.to_string();
        let org_id_str = user.org_id.to_string();
        let role_str = effective_context.effective_role.clone();

        let has_verified_mfa = self
            .mfa_factor_repo
            .find_by_user(user.id)
            .await?
            .iter()
            .any(|factor| factor.is_verified);
        if user.is_mfa_enabled || has_verified_mfa {
            if !has_verified_mfa {
                return Err(DomainError::Internal(
                    "MFA is enabled but no verified factor is available".into(),
                ));
            }
            let mfa_token = self
                .token_service
                .generate_mfa_token(&user_id_str, &org_id_str, &role_str)
                .await?;
            self.save_mfa_challenge_session(user.id, &mfa_token).await?;
            return Ok(LoginResponse {
                token: None,
                refresh_token: None,
                user,
                person,
                effective_context: Some(effective_context),
                mfa_required: true,
                mfa_token: Some(mfa_token),
            });
        }

        let access_token = self
            .token_service
            .generate_access_token(&user_id_str, &org_id_str, &role_str)
            .await?;
        let refresh_token = self
            .token_service
            .generate_refresh_token(&user_id_str, &org_id_str, &role_str)
            .await?;

        let now = Utc::now();
        let session_id = Uuid::new_v4();

        let session = Session {
            id: session_id,
            user_id: user.id,
            token: access_token.clone(),
            refresh_token: Some(refresh_token.clone()),
            device_id: None,
            family_id: None,
            is_revoked: false,
            is_current: true,
            is_trusted: false,
            is_mfa_verified: false,
            expires_at: now + chrono::Duration::hours(2),
            refresh_expires_at: Some(now + chrono::Duration::days(7)),
            last_active_at: Some(now),
            created_at: now,
        };

        self.session_repo.save(&session).await?;

        let event = DomainEvent::UserLoggedIn {
            user_id: user.id,
            org_id: user.org_id,
            session_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(LoginResponse {
            token: Some(access_token),
            refresh_token: Some(refresh_token),
            user,
            person,
            effective_context: Some(effective_context),
            mfa_required: false,
            mfa_token: None,
        })
    }

    /// Register a new user in the organization
    pub async fn register_user(
        &self,
        ctx: &RequestContext,
        full_name: String,
        email: Option<String>,
        password_plain: Option<String>,
        _role: Option<String>,
        org_id: Option<Uuid>,
    ) -> Result<User, DomainError> {
        let actual_org_id = org_id.unwrap_or(ctx.tenant_id);

        if let Some(ref e) = email {
            if self.user_repo.find_by_email(e).await?.is_some() {
                return Err(DomainError::Conflict(format!(
                    "Email '{}' already registered",
                    e
                )));
            }
        }

        let now = Utc::now();
        let person_id = Uuid::new_v4();

        let name_parts: Vec<&str> = full_name.split_whitespace().collect();
        let first_name = name_parts.first().unwrap_or(&"").to_string();
        let last_name = if name_parts.len() > 1 {
            name_parts[1..].join(" ")
        } else {
            first_name.clone()
        };

        let person = Person {
            id: person_id,
            org_id: Some(actual_org_id),
            first_name,
            middle_name: None,
            last_name,
            first_name_ar: None,
            middle_name_ar: None,
            last_name_ar: None,
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
            phone: None,
            email: email.clone(),
            address: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        };
        self.person_repo.save(&person).await?;

        let password_hash = match password_plain {
            Some(p) => Some(self.password_hasher.hash(&p).await?),
            None => None,
        };

        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            person_id,
            org_id: actual_org_id,
            email,
            is_email_verified: false,
            phone: None,
            is_phone_verified: false,
            password_hash,
            is_mfa_enabled: false,
            status: "active".to_string(),
            login_count: 0,
            failed_login_attempts: 0,
            locked_until: None,
            is_deleted: false,
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.user_repo.save(&user).await?;

        let event = DomainEvent::UserCreated {
            user_id,
            person_id,
            org_id: actual_org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(user)
    }

    /// Get user by ID
    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        self.user_repo.find_by_id(id).await
    }

    async fn save_mfa_challenge_session(
        &self,
        user_id: Uuid,
        token: &str,
    ) -> Result<(), DomainError> {
        let now = Utc::now();
        self.session_repo
            .save(&Session {
                id: Uuid::new_v4(),
                user_id,
                token: token.to_string(),
                refresh_token: None,
                device_id: None,
                family_id: None,
                is_revoked: false,
                is_current: false,
                is_trusted: false,
                is_mfa_verified: false,
                expires_at: now + chrono::Duration::minutes(5),
                refresh_expires_at: None,
                last_active_at: Some(now),
                created_at: now,
            })
            .await
    }

    /// List users in an organization
    pub async fn list_users(&self, org_id: Uuid) -> Result<Vec<User>, DomainError> {
        self.user_repo.find_by_org(org_id).await
    }

    /// Update user details
    pub async fn update_user(
        &self,
        ctx: &RequestContext,
        user_id: Uuid,
        full_name: Option<String>,
    ) -> Result<User, DomainError> {
        self.permission_checker
            .check(ctx, Permission::UserWrite)
            .await?;

        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User {} not found", user_id)))?;

        if let Some(name) = full_name {
            if let Ok(Some(mut person)) = self.person_repo.find_by_id(user.person_id).await {
                let parts: Vec<&str> = name.split_whitespace().collect();
                person.first_name = parts.first().unwrap_or(&"").to_string();
                if parts.len() > 1 {
                    person.last_name = parts[1..].join(" ");
                }
                person.updated_at = Utc::now();
                self.person_repo.save(&person).await?;
            }
        }
        user.updated_at = Utc::now();
        self.user_repo.save(&user).await?;

        Ok(user)
    }

    /// Invalidate session / Logout
    pub async fn logout(&self, token: &str) -> Result<(), DomainError> {
        self.session_repo.invalidate(token).await
    }

    /// Enroll MFA factor (TOTP)
    pub async fn enroll_totp(
        &self,
        ctx: &RequestContext,
        factor_name: Option<String>,
    ) -> Result<MfaEnrollmentResult, DomainError> {
        let user_id = ctx
            .actor
            .id()
            .ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))?;

        for factor in self.mfa_factor_repo.find_by_user(user_id).await? {
            if !factor.is_verified {
                self.mfa_factor_repo.delete(factor.id).await?;
            }
        }

        let secret = self.totp_service.generate_secret();
        let account_name = user.email.clone().unwrap_or_else(|| user.id.to_string());
        let qr_code_uri = self
            .totp_service
            .generate_qr_uri("Orbis", &account_name, &secret);

        let factor_id = Uuid::new_v4();
        let now = Utc::now();

        let factor = MfaFactor {
            id: factor_id,
            user_id,
            factor_type: MfaType::Totp,
            factor_name: factor_name.unwrap_or_else(|| "Authenticator App".to_string()),
            secret_encrypted: self.totp_service.encrypt_secret(&secret)?,
            is_verified: false,
            is_default: true,
            created_at: now,
            last_used_at: None,
        };
        self.mfa_factor_repo.save(&factor).await?;

        Ok(MfaEnrollmentResult {
            factor_id,
            secret,
            qr_code_uri,
        })
    }

    /// Verify TOTP enrollment and activate MFA
    pub async fn verify_totp_enrollment(
        &self,
        ctx: &RequestContext,
        factor_id: Uuid,
        code: &str,
    ) -> Result<Vec<String>, DomainError> {
        let user_id = ctx
            .actor
            .id()
            .ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        let factor = self
            .mfa_factor_repo
            .find_by_id(factor_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Factor not found".into()))?;

        if factor.user_id != user_id {
            return Err(DomainError::Forbidden("Invalid factor".into()));
        }
        if factor.is_verified {
            return Err(DomainError::Conflict(
                "MFA factor has already been activated".into(),
            ));
        }

        let secret = self.totp_service.decrypt_secret(&factor.secret_encrypted)?;
        let is_valid = self.totp_service.verify_code(&secret, code);
        if !is_valid {
            return Err(DomainError::Validation("Invalid verification code".into()));
        }

        let now = Utc::now();
        let raw_recovery_codes = self.totp_service.generate_recovery_codes(8);
        let mut recovery_code_entities = Vec::with_capacity(raw_recovery_codes.len());
        for code in &raw_recovery_codes {
            recovery_code_entities.push(MfaRecoveryCode {
                id: Uuid::new_v4(),
                user_id,
                code_hash: self.password_hasher.hash(code).await?,
                is_used: false,
                created_at: now,
                used_at: None,
            });
        }
        self.mfa_recovery_code_repo
            .save_all(&recovery_code_entities)
            .await?;

        let mut updated_factor = factor;
        if !updated_factor.secret_encrypted.starts_with("v1:") {
            let secret = self
                .totp_service
                .decrypt_secret(&updated_factor.secret_encrypted)?;
            updated_factor.secret_encrypted = self.totp_service.encrypt_secret(&secret)?;
        }
        updated_factor.is_verified = true;
        updated_factor.last_used_at = Some(now);
        self.mfa_factor_repo.save(&updated_factor).await?;

        if let Some(mut user) = self.user_repo.find_by_id(user_id).await? {
            user.is_mfa_enabled = true;
            user.updated_at = now;
            self.user_repo.save(&user).await?;
        }

        Ok(raw_recovery_codes)
    }

    /// Verify MFA challenge and issue session tokens
    pub async fn verify_mfa_challenge(
        &self,
        mfa_token: &str,
        code: &str,
        device_id: Option<String>,
        remember_device: bool,
    ) -> Result<VerifiedSessionResponse, DomainError> {
        let claims = self.token_service.validate_mfa_token(mfa_token).await?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| DomainError::Unauthorized("Invalid MFA token subject".into()))?;
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))?;
        if user.org_id.to_string() != claims.org_id {
            return Err(DomainError::Unauthorized("Invalid MFA token scope".into()));
        }
        let challenge_exists = self
            .session_repo
            .find_by_token(mfa_token)
            .await?
            .is_some_and(|session| {
                session.user_id == user_id
                    && !session.is_mfa_verified
                    && session.expires_at > Utc::now()
            });
        if !challenge_exists {
            return Err(DomainError::Unauthorized(
                "MFA challenge has expired or was already used".into(),
            ));
        }

        let factors = self.mfa_factor_repo.find_by_user(user_id).await?;
        let verified_factor = factors.into_iter().find(|f| f.is_verified);

        let mut authenticated = false;

        if let Some(mut factor) = verified_factor {
            let secret = self.totp_service.decrypt_secret(&factor.secret_encrypted)?;
            if self.totp_service.verify_code(&secret, code) {
                if !factor.secret_encrypted.starts_with("v1:") {
                    let secret = self.totp_service.decrypt_secret(&factor.secret_encrypted)?;
                    factor.secret_encrypted = self.totp_service.encrypt_secret(&secret)?;
                }
                factor.last_used_at = Some(Utc::now());
                self.mfa_factor_repo.save(&factor).await?;
                authenticated = true;
            }
        }

        if !authenticated {
            let active_codes = self
                .mfa_recovery_code_repo
                .find_active_by_user(user_id)
                .await?;
            for rc in active_codes {
                if self.password_hasher.verify(code, &rc.code_hash).await? {
                    authenticated = self.mfa_recovery_code_repo.mark_used(rc.id).await?;
                    if authenticated {
                        break;
                    }
                }
            }
        }

        if !authenticated {
            return Err(DomainError::Unauthorized(
                "Invalid authentication code or recovery code".into(),
            ));
        }

        if !self.session_repo.consume_mfa_challenge(mfa_token).await? {
            return Err(DomainError::Unauthorized(
                "MFA challenge has expired or was already used".into(),
            ));
        }

        if let Some(dev_id) = device_id {
            if remember_device {
                if let Ok(Some(mut dev)) = self
                    .user_device_repo
                    .find_by_device_id(user_id, &dev_id)
                    .await
                {
                    dev.is_trusted = true;
                    dev.trusted_until = Some(Utc::now() + chrono::Duration::days(30));
                    dev.last_active_at = Utc::now();
                    let _ = self.user_device_repo.save(&dev).await;
                }
            }
        }

        let user_id_str = user.id.to_string();
        let org_id_str = user.org_id.to_string();
        let role_str = "org_admin".to_string();

        let access_token = self
            .token_service
            .generate_access_token(&user_id_str, &org_id_str, &role_str)
            .await?;
        let refresh_token = self
            .token_service
            .generate_refresh_token(&user_id_str, &org_id_str, &role_str)
            .await?;

        let now = Utc::now();
        let session = Session {
            id: Uuid::new_v4(),
            user_id: user.id,
            device_id: None,
            token: access_token.clone(),
            refresh_token: Some(refresh_token.clone()),
            family_id: Some(Uuid::new_v4().to_string()),
            is_current: true,
            is_trusted: remember_device,
            is_revoked: false,
            is_mfa_verified: true,
            expires_at: now + chrono::Duration::hours(24),
            refresh_expires_at: Some(now + chrono::Duration::days(30)),
            last_active_at: Some(now),
            created_at: now,
        };
        self.session_repo.save(&session).await?;

        Ok(VerifiedSessionResponse {
            token: access_token,
            refresh_token,
            user_id,
        })
    }

    /// Link Google account to existing user
    pub async fn link_google(
        &self,
        ctx: &RequestContext,
        id_token: &str,
    ) -> Result<UserIdentity, DomainError> {
        let user_id = ctx
            .actor
            .id()
            .ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))?;

        let profile = self.oidc_service.verify_google_token(id_token).await?;

        let provider_user_id = profile.email.clone().unwrap_or_default();
        if provider_user_id.is_empty() {
            return Err(DomainError::Validation(
                "Invalid Google account profile".into(),
            ));
        }

        self.verify_org_identity_config(user.org_id, "google", &provider_user_id)
            .await?;

        if let Some(existing) = self
            .user_identity_repo
            .find_by_provider("google", &provider_user_id)
            .await?
        {
            if existing.user_id != user_id {
                return Err(DomainError::Conflict(
                    "This Google account is already linked to another user".into(),
                ));
            }
            return Ok(existing);
        }

        let identity = UserIdentity {
            id: Uuid::new_v4(),
            user_id,
            provider: IdentityProvider::Google,
            provider_user_id,
            email: profile.email.clone(),
            is_primary: false,
            profile_data: Some(serde_json::to_value(&profile).unwrap_or_default()),
            connected_at: Utc::now(),
            last_used_at: Some(Utc::now()),
        };

        self.user_identity_repo.save(&identity).await?;
        Ok(identity)
    }

    /// Login via Google identity
    pub async fn login_google(&self, id_token: &str) -> Result<GoogleLoginResult, DomainError> {
        let profile = self.oidc_service.verify_google_token(id_token).await?;
        let provider_user_id = profile.email.clone().unwrap_or_default();

        if provider_user_id.is_empty() {
            return Err(DomainError::Validation(
                "Invalid Google account profile".into(),
            ));
        }

        let user = if let Some(identity) = self
            .user_identity_repo
            .find_by_provider("google", &provider_user_id)
            .await?
        {
            self.user_repo
                .find_by_id(identity.user_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("User not found".into()))?
        } else if let Some(existing_user) = self.user_repo.find_by_email(&provider_user_id).await? {
            let new_identity = UserIdentity {
                id: Uuid::new_v4(),
                user_id: existing_user.id,
                provider: IdentityProvider::Google,
                provider_user_id: provider_user_id.clone(),
                email: Some(provider_user_id.clone()),
                is_primary: false,
                profile_data: Some(serde_json::to_value(&profile).unwrap_or_default()),
                connected_at: Utc::now(),
                last_used_at: Some(Utc::now()),
            };
            let _ = self.user_identity_repo.save(&new_identity).await;
            existing_user
        } else {
            return Err(DomainError::NotFound(
                "No account linked with this Google identity".into(),
            ));
        };

        self.verify_org_identity_config(user.org_id, "google", &provider_user_id)
            .await?;

        let effective_role = "org_admin";
        let verified_factor_exists = self
            .mfa_factor_repo
            .find_by_user(user.id)
            .await?
            .iter()
            .any(|factor| factor.is_verified);
        if user.is_mfa_enabled || verified_factor_exists {
            if !verified_factor_exists {
                return Err(DomainError::Internal(
                    "MFA is enabled but no verified factor is available".into(),
                ));
            }
            let mfa_token = self
                .token_service
                .generate_mfa_token(
                    &user.id.to_string(),
                    &user.org_id.to_string(),
                    effective_role,
                )
                .await?;
            self.save_mfa_challenge_session(user.id, &mfa_token).await?;
            return Ok(GoogleLoginResult {
                token: None,
                refresh_token: None,
                user_id: user.id,
                org_id: user.org_id,
                mfa_required: true,
                mfa_token: Some(mfa_token),
            });
        }

        let user_id_str = user.id.to_string();
        let org_id_str = user.org_id.to_string();
        let role_str = "org_admin".to_string();

        let access_token = self
            .token_service
            .generate_access_token(&user_id_str, &org_id_str, &role_str)
            .await?;
        let refresh_token = self
            .token_service
            .generate_refresh_token(&user_id_str, &org_id_str, &role_str)
            .await?;

        let now = Utc::now();
        let session = Session {
            id: Uuid::new_v4(),
            user_id: user.id,
            device_id: None,
            token: access_token.clone(),
            refresh_token: Some(refresh_token.clone()),
            family_id: Some(Uuid::new_v4().to_string()),
            is_current: true,
            is_trusted: true,
            is_revoked: false,
            is_mfa_verified: true,
            expires_at: now + chrono::Duration::hours(24),
            refresh_expires_at: Some(now + chrono::Duration::days(30)),
            last_active_at: Some(now),
            created_at: now,
        };
        self.session_repo.save(&session).await?;

        Ok(GoogleLoginResult {
            token: Some(access_token),
            refresh_token: Some(refresh_token),
            user_id: user.id,
            org_id: user.org_id,
            mfa_required: false,
            mfa_token: None,
        })
    }

    /// List user devices
    pub async fn list_devices(&self, ctx: &RequestContext) -> Result<Vec<UserDevice>, DomainError> {
        let user_id = ctx
            .actor
            .id()
            .ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        self.user_device_repo.find_by_user(user_id).await
    }

    /// Revoke a device
    pub async fn revoke_device(
        &self,
        ctx: &RequestContext,
        device_id: Uuid,
    ) -> Result<(), DomainError> {
        let _user_id = ctx
            .actor
            .id()
            .ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        self.user_device_repo.delete(device_id).await
    }

    /// Register or update a user device
    pub async fn register_device(
        &self,
        user_id: Uuid,
        device_id: String,
        device_name: String,
        device_type: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<UserDevice, DomainError> {
        let now = Utc::now();
        let dev_type = DeviceType::from(device_type.as_str());
        let device = match self
            .user_device_repo
            .find_by_device_id(user_id, &device_id)
            .await?
        {
            Some(mut d) => {
                d.device_name = device_name;
                d.client_ip = ip_address;
                d.last_active_at = now;
                self.user_device_repo.save(&d).await?;
                d
            }
            None => {
                let d = UserDevice {
                    id: Uuid::new_v4(),
                    user_id,
                    device_id: device_id.clone(),
                    device_name,
                    device_type: dev_type,
                    os: None,
                    browser: None,
                    client_ip: ip_address,
                    user_agent,
                    fingerprint_hash: device_id,
                    is_trusted: true,
                    trusted_until: None,
                    first_seen_at: now,
                    last_active_at: now,
                };
                self.user_device_repo.save(&d).await?;
                d
            }
        };
        Ok(device)
    }

    async fn verify_org_identity_config(
        &self,
        org_id: Uuid,
        provider_name: &str,
        email: &str,
    ) -> Result<(), DomainError> {
        if let Some(ref repo) = self.integration_repo {
            let org_config = repo
                .find_config_by_provider(org_id, &format!("{}-workspace", provider_name))
                .await?
                .or(repo.find_config_by_provider(org_id, provider_name).await?);

            if let Some(config) = org_config {
                if !config.is_enabled {
                    return Err(DomainError::Forbidden(format!(
                        "{} integration is disabled for this organization",
                        provider_name
                    )));
                }

                if let Some(settings) = &config.settings {
                    if let Some(enable_sso) = settings.get("enable_sso").and_then(|v| v.as_bool()) {
                        if !enable_sso {
                            return Err(DomainError::Forbidden(format!(
                                "{} SSO is disabled by organization administrator",
                                provider_name
                            )));
                        }
                    }

                    if let Some(domain) = settings.get("domain").and_then(|v| v.as_str()) {
                        let domain = domain.trim().trim_start_matches('@').to_lowercase();
                        if !domain.is_empty() {
                            let email_lower = email.to_lowercase();
                            let expected_suffix = format!("@{}", domain);
                            if !email_lower.ends_with(&expected_suffix) {
                                return Err(DomainError::Validation(format!(
                                    "Account email ({}) does not match organization domain ({})",
                                    email, domain
                                )));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
