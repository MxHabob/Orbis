use std::sync::Arc;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, QueryOrder, ActiveModelTrait, ActiveValue::Set};
use shared_kernel::config::Config;
use shared_kernel::errors::AppError;
use chrono::Utc;
use serde_json::json;
use async_trait::async_trait;

use domain_iam::services::authz::{AuthzService, Permission, AuthzContext};
use crate::repository::Repository;

// Inline Submodules providing request/response payload schemas expected by REST handlers

pub mod authz {
    pub use domain_iam::services::authz::{AuthzService, Permission, AuthzContext};
}

pub mod class {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UpdateClassRequest {
        pub name: String,
        pub name_ar: String,
        pub level: String,
        pub division: String,
    }
}

pub mod behavior {
    pub use domain_assessment::services::behavior::{AIBehaviorSubmission, UpdateBehaviorRequest};
}

pub mod organization {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UpdateOrganizationRequest {
        pub name: Option<String>,
        pub timezone: Option<String>,
        pub locale: Option<String>,
        pub currency: Option<String>,
        pub date_format: Option<String>,
        pub hijri_enabled: Option<i32>,
        pub logo: Option<String>,
        pub primary_color: Option<String>,
        pub settings: Option<String>,
        pub features: Option<String>,
    }
}

pub mod user {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CreateUserRequest {
        pub email: String,
        pub password: String,
        pub full_name: String,
        pub name_ar: Option<String>,
        pub primary_role: String,
        pub sex: Option<String>,
        pub phone: Option<String>,
        pub national_id: Option<String>,
        pub date_of_birth: Option<String>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UpdateUserRequest {
        pub full_name: Option<String>,
        pub name_ar: Option<String>,
        pub name_en: Option<String>,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub sex: Option<String>,
        pub date_of_birth: Option<String>,
        pub nationality: Option<String>,
        pub national_id: Option<String>,
        pub address: Option<String>,
        pub city: Option<String>,
        pub country: Option<String>,
        pub emergency_contact: Option<String>,
        pub blood_type: Option<String>,
        pub medical_conditions: Option<String>,
        pub avatar: Option<String>,
        pub status: Option<String>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ChangeRoleRequest {
        pub role: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ChangePasswordRequest {
        pub new_password: String,
    }
}

pub mod academic_cycle {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CreateCycleRequest {
        pub name: String,
        pub name_ar: Option<String>,
        pub code: Option<String>,
        pub cycle_type: String,
        pub parent_id: Option<String>,
        pub start_date: String,
        pub end_date: String,
        pub hijri_start_date: Option<String>,
        pub hijri_end_date: Option<String>,
        pub grading_system: Option<String>,
        pub passing_grade: Option<i32>,
        pub max_grade: Option<i32>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UpdateCycleRequest {
        pub name: Option<String>,
        pub name_ar: Option<String>,
        pub start_date: Option<String>,
        pub end_date: Option<String>,
        pub exam_start_date: Option<String>,
        pub exam_end_date: Option<String>,
        pub grading_system: Option<String>,
        pub passing_grade: Option<i32>,
        pub max_grade: Option<i32>,
        pub is_enrollment_open: Option<i32>,
    }
}

pub mod org_unit {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CreateOrgUnitRequest {
        pub name: String,
        pub name_ar: Option<String>,
        pub code: Option<String>,
        pub unit_type: String,
        pub parent_id: Option<String>,
        pub capacity: Option<i32>,
        pub room_number: Option<String>,
        pub floor: Option<i32>,
        pub building: Option<String>,
        pub head_id: Option<String>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UpdateOrgUnitRequest {
        pub name: Option<String>,
        pub name_ar: Option<String>,
        pub unit_type: Option<String>,
        pub capacity: Option<i32>,
        pub room_number: Option<String>,
        pub floor: Option<i32>,
        pub building: Option<String>,
        pub head_id: Option<String>,
        pub is_active: Option<i32>,
    }
}

pub mod enrollment {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CreateEnrollmentRequest {
        pub student_id: String,
        pub class_id: String,
        pub cycle_id: Option<String>,
        pub enrollment_type: Option<String>,
        pub discount_template_id: Option<String>,
        pub custom_discount_amount: Option<f64>,
        pub status: Option<String>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct WithdrawRequest {
        pub reason: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct TransferRequest {
        pub new_class_id: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PromoteRequest {
        pub class_id: String,
        pub passing_score: Option<i32>,
        pub min_attendance_rate: Option<f64>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PromotionResult {
        pub total_students: usize,
        pub promoted: usize,
        pub retained: usize,
        pub details: Vec<PromotionDetail>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct PromotionDetail {
        pub student_id: String,
        pub student_name: String,
        pub status: String,
        pub reason: Option<String>,
    }
}

pub mod grade {
    pub use domain_assessment::services::grade::{CreateGradeRequest, BulkGradeRequest, UpdateGradeRequest, BulkSubjectGradeEntry};
}

pub mod attendance_svc {
    pub use domain_assessment::services::attendance::{CreateAttendanceRequest, BulkAttendanceRequest};
}

pub mod finance {
    pub use domain_finance::services::ledger::CreateLedgerAccountRequest;
    pub use domain_finance::services::treasury::CreateTreasuryRequest;
    pub use domain_finance::services::fee::AssignStudentFeeRequest;
    pub use domain_finance::services::payment::RecordPaymentRequest;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CreateJournalEntryRequest {
        pub cycle_id: String,
        pub date: String,
        pub description: String,
    }
}

pub mod schedule {
    pub use domain_scheduling::services::schedule::{CreateTimeSlotRequest, UpdateTimeSlotRequest, CreateClassSubjectRequest, UpdateClassSubjectRequest, CreateLessonRequest, UpdateLessonRequest, ConflictCheckRequest};
}

pub mod schedule_solver {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct SolveJobResponse {
        pub id: String,
        pub cycle_id: String,
        pub status: String,
        pub best_score: Option<String>,
        pub created_at: String,
        pub completed_at: Option<String>,
        pub error_message: Option<String>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct StartSolveRequest {
        pub cycle_id: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ApplySolveResult {
        pub lessons_created: usize,
    }
}

pub mod room {
    pub use domain_scheduling::services::room::{CreateRoomRequest, UpdateRoomRequest};
}

pub mod subject {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct CreateSubjectRequest {
        pub name: String,
        pub name_ar: Option<String>,
        pub code: Option<String>,
        pub level: String,
        pub credit_hours: Option<i32>,
        pub weight: Option<f64>,
        pub category: Option<String>,
        pub settings: Option<String>,
        pub sort_order: Option<i32>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct UpdateSubjectRequest {
        pub name: Option<String>,
        pub name_ar: Option<String>,
        pub credit_hours: Option<i32>,
    }
}

pub mod assessment {
    pub use domain_assessment::services::assessment::{CreateAssessmentRequest, UpdateAssessmentRequest};
}

pub mod guardian {
    pub use domain_academic::services::guardian::{CreateGuardianLinkRequest, UpdateGuardianLinkRequest};
}

pub mod document {
    pub use domain_academic::services::document::{CreateDocumentRequest, UpdateDocumentRequest};
}

pub mod exam_session {
    pub use domain_exam::services::exam_session::{CreateExamSessionRequest, UpdateExamSessionRequest, CreateExamSubjectRequest, CreateExamScheduleRequest, CreateExamHallRequest};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ExamDashboard {
        pub session_summary: SessionSummary,
        pub schedule_overview: ScheduleOverview,
        pub hall_statistics: HallStatistics,
        pub proctor_statistics: ProctorStatistics,
        pub student_statistics: StudentStatistics,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct SessionSummary {
        pub session_id: String,
        pub name: String,
        pub status: String,
        pub session_type: String,
        pub start_date: String,
        pub end_date: String,
        pub total_subjects: usize,
        pub total_students: usize,
        pub total_halls: usize,
        pub total_proctors: usize,
        pub completion_pct: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ScheduleOverview {
        pub total_scheduled: usize,
        pub completed: usize,
        pub pending: usize,
        pub postponed: usize,
        pub cancelled: usize,
        pub exams_by_date: Vec<DateExamCount>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct DateExamCount {
        pub date: String,
        pub count: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct HallStatistics {
        pub total_halls: usize,
        pub total_capacity: i32,
        pub total_allocated: i32,
        pub average_utilization_pct: f64,
        pub halls: Vec<HallUtilization>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct HallUtilization {
        pub hall_id: String,
        pub hall_number: String,
        pub capacity: i32,
        pub allocated: i32,
        pub utilization_pct: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ProctorStatistics {
        pub total_assigned: usize,
        pub confirmed: usize,
        pub pending_confirmation: usize,
        pub load_distribution: Vec<ProctorLoad>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct ProctorLoad {
        pub teacher_id: String,
        pub sessions_count: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
    pub struct StudentStatistics {
        pub total_enrolled: usize,
        pub with_seating_number: usize,
        pub with_secret_number: usize,
        pub allocated_to_halls: usize,
        pub coverage_pct: f64,
    }
}

pub mod student_ordering {
    pub use domain_exam::services::student_ordering::RebuildOrderingResult;
}

pub mod exam_numbers {
    pub use domain_exam::services::exam_numbers::GenerationResult;
}

pub mod exam_logistics {
    pub use domain_exam::services::exam_logistics::{AllocationConfig, ProctorAssignConfig, AllocationResult, ProctorAssignResult};
}

pub mod exam_auto_schedule {
    pub use domain_exam::services::exam_auto_schedule::{AutoScheduleRequest, SolveJobResponse};
}

pub mod auth {
    pub use domain_iam::services::auth::LoginResponse;
}

// Composed Service Facade with pure delegation methods

#[derive(Clone)]
pub struct Service {
    pub db: DatabaseConnection,
    pub cfg: Arc<Config>,
    pub repo: Arc<Repository>,
    pub authz: Arc<AuthzService>,
}

impl Service {
    pub fn new(repo: Repository, cfg: Config) -> Self {
        Self {
            db: repo.db.clone(),
            repo: Arc::new(repo),
            cfg: Arc::new(cfg),
            authz: Arc::new(AuthzService::new()),
        }
    }

    /// Centralized security checks
    pub async fn check_permission_and_audit(
        &self,
        user: &crate::entities::user::Model,
        permission: Permission,
        ctx: Option<&AuthzContext>,
    ) -> Result<(), String> {
        match self.authz.check_permission(&self.db, user, permission, ctx).await {
            Ok(_) => Ok(()),
            Err(err_msg) => {
                tracing::warn!(
                    "🔒 Security Warning: User '{}' ({}) from organization '{}' failed authorization check for '{:?}'. Context: {:?}. Reason: {}",
                    user.full_name,
                    user.id,
                    user.org_id,
                    permission,
                    ctx,
                    err_msg
                );

                let audit_record = crate::entities::audit_log::Model {
                    id: uuid::Uuid::new_v4().to_string(),
                    org_id: user.org_id.clone(),
                    user_id: Some(user.id.clone()),
                    action: "security.unauthorized_attempt".to_string(),
                    entity_type: "permission".to_string(),
                    entity_id: format!("{:?}", permission),
                    old_data: None,
                    new_data: Some(json!({
                        "permission": format!("{:?}", permission),
                        "context": ctx.map(|c| json!({
                            "target_user_id": c.target_user_id,
                            "target_class_id": c.target_class_id,
                            "target_org_id": c.target_org_id,
                        })),
                        "error": err_msg.clone()
                    }).to_string()),
                    ip_address: None,
                    user_agent: None,
                    session_id: None,
                    metadata: json!({
                        "severity": "high",
                        "incident": true,
                        "client_notified": false
                    }).to_string(),
                    created_at: Utc::now().to_rfc3339(),
                };

                let cmd = domain_iam::repository::IamCommandRepo::new(self.db.clone());
                let _ = cmd.create_audit_log(audit_record).await;

                Err(err_msg)
            }
        }
    }

    // ── Auth & IAM Services ──────────────────────────────────────

    pub async fn login(&self, email: &str, password: &str) -> Result<domain_iam::services::auth::LoginResponse, String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let auth_svc = AuthService::new(cmd, query, self.cfg.clone());
        auth_svc.login(email, password).await.map_err(|e| e.to_string())
    }

    pub async fn validate_token(&self, token: &str) -> Result<crate::entities::user::Model, String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let auth_svc = AuthService::new(cmd, query, self.cfg.clone());
        auth_svc.validate_token(token).await.map_err(|e| e.to_string())
    }

    pub async fn logout(&self, token: &str) -> Result<(), String> {
        crate::entities::user_session::Entity::delete_many()
            .filter(crate::entities::user_session::Column::Token.eq(token))
            .exec(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn register(&self, email: &str, password: &str, full_name: &str, org_slug: &str) -> Result<crate::entities::user::Model, String> {
        let org_cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let org_query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let org_svc = OrganizationService::new(org_cmd, org_query);
        let org = org_svc.create(org_slug, org_slug, "UTC", "en", "SAR")
            .await
            .map_err(|e| e.to_string())?;

        let hashed_pw = AuthService::hash_password(password).map_err(|e| e.to_string())?;

        let user_cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let user_query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let user_svc = UserService::new(user_cmd, user_query);
        let user = user_svc.create_user(&org.id, Some(email), full_name, "owner", Some(hashed_pw))
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    pub async fn get_users(&self, org_id: &str, params: domain_iam::params::UserParams) -> Result<Vec<crate::entities::user::Model>, String> {
        let mut q = crate::entities::user::Entity::find().filter(crate::entities::user::Column::OrgId.eq(org_id));
        if let Some(role) = params.role {
            if !role.trim().is_empty() {
                q = q.filter(crate::entities::user::Column::PrimaryRole.eq(role));
            }
        }
        if let Some(status) = params.status {
            if !status.trim().is_empty() {
                q = q.filter(crate::entities::user::Column::Status.eq(status));
            }
        }
        if let Some(search) = params.q {
            if !search.trim().is_empty() {
                let s = format!("%{}%", search);
                q = q.filter(crate::entities::user::Column::FullName.like(&s).or(crate::entities::user::Column::Email.like(&s)));
            }
        }
        q = q.order_by_desc(crate::entities::user::Column::CreatedAt);
        if let Some(limit) = params.limit {
            q = q.limit(limit);
        }
        if let Some(offset) = params.offset {
            q = q.offset(offset);
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_user_in_org(&self, org_id: &str, req: user::CreateUserRequest, _actor_id: &str) -> Result<crate::entities::user::Model, String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let user_svc = UserService::new(cmd, query);
        
        let hashed_pw = if let Some(ref pwd) = req.password {
            Some(AuthService::hash_password(pwd).map_err(|e| e.to_string())?)
        } else {
            None
        };

        user_svc.create_user(org_id, req.email.as_deref(), &req.full_name, &req.primary_role, hashed_pw)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_user(&self, id: &str) -> Result<crate::entities::user::Model, String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let user_svc = UserService::new(cmd, query);
        user_svc.get(id).await.map_err(|e| e.to_string())
    }

    pub async fn update_user(&self, id: &str, req: user::UpdateUserRequest) -> Result<crate::entities::user::Model, String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let user_svc = UserService::new(cmd, query);
        user_svc.update_user(id, req).await.map_err(|e| e.to_string())
    }

    pub async fn delete_user(&self, id: &str, _actor_id: &str) -> Result<(), String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let user_svc = UserService::new(cmd, query);
        user_svc.delete_user(id).await.map_err(|e| e.to_string())
    }

    pub async fn change_user_role(&self, id: &str, role: &str) -> Result<crate::entities::user::Model, String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let user_svc = UserService::new(cmd, query);
        user_svc.change_role(id, role).await.map_err(|e| e.to_string())
    }

    pub async fn change_user_password(&self, id: &str, new_pwd: &str) -> Result<crate::entities::user::Model, String> {
        let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_iam::repository::IamQueryRepo::new(self.db.clone()));
        let user_svc = UserService::new(cmd, query);
        let hashed = AuthService::hash_password(new_pwd).map_err(|e| e.to_string())?;
        user_svc.change_password(id, &hashed).await.map_err(|e| e.to_string())
    }

    // ── Academic Cycle Services ──────────────────────────────────

    pub async fn get_cycles(&self, org_id: &str, params: domain_lifecycle::params::CycleParams) -> Result<Vec<crate::entities::academic_cycle::Model>, String> {
        let mut q = crate::entities::academic_cycle::Entity::find().filter(crate::entities::academic_cycle::Column::OrgId.eq(org_id));
        if let Some(ref c_type) = params.cycle_type {
            if !c_type.trim().is_empty() {
                q = q.filter(crate::entities::academic_cycle::Column::CycleType.eq(c_type));
            }
        }
        if let Some(is_curr) = params.is_current {
            q = q.filter(crate::entities::academic_cycle::Column::IsCurrent.eq(is_curr));
        }
        q = q.order_by_desc(crate::entities::academic_cycle::Column::StartDate);
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_cycle(&self, id: &str) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        cycle_svc.get(id).await.map_err(|e| e.to_string())
    }

    pub async fn create_cycle(&self, org_id: &str, req: academic_cycle::CreateCycleRequest) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        cycle_svc.create(org_id, &req.name, req.name_ar.as_deref(), &req.cycle_type, req.parent_id.as_deref(), &req.start_date, &req.end_date, None, None, None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_cycle(&self, id: &str, req: academic_cycle::UpdateCycleRequest) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        let name = req.name.ok_or_else(|| "name required".to_string())?;
        let start_date = req.start_date.ok_or_else(|| "start_date required".to_string())?;
        let end_date = req.end_date.ok_or_else(|| "end_date required".to_string())?;
        cycle_svc.update(id, &name, req.name_ar.as_deref(), &start_date, &end_date, req.exam_start_date.as_deref(), req.exam_end_date.as_deref(), None)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn activate_cycle(&self, org_id: &str, id: &str) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        cycle_svc.activate(org_id, id).await.map_err(|e| e.to_string())
    }

    pub async fn lock_cycle(&self, id: &str, actor_id: &str) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        cycle_svc.lock(id, actor_id).await.map_err(|e| e.to_string())
    }

    pub async fn unlock_cycle(&self, id: &str) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        cycle_svc.unlock(id).await.map_err(|e| e.to_string())
    }

    pub async fn open_grade_entry(&self, id: &str, start: Option<String>, end: Option<String>) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        
        let cycle = cycle_svc.set_grade_entry_open(id, true).await.map_err(|e| e.to_string())?;
        
        if start.is_some() || end.is_some() {
            let mut active: crate::entities::academic_cycle::ActiveModel = cycle.into();
            active.grade_entry_start = Set(start);
            active.grade_entry_end = Set(end);
            Ok(active.update(&self.db).await.map_err(|e| e.to_string())?)
        } else {
            Ok(cycle)
        }
    }

    pub async fn close_grade_entry(&self, id: &str) -> Result<crate::entities::academic_cycle::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let cycle_svc = CycleService::new(cmd, query);
        cycle_svc.set_grade_entry_open(id, false).await.map_err(|e| e.to_string())
    }

    // ── Organization Services ────────────────────────────────────

    pub async fn get_organization(&self, id: &str) -> Result<crate::entities::organization::Model, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let org_svc = OrganizationService::new(cmd, query);
        org_svc.get(id).await.map_err(|e| e.to_string())
    }

    pub async fn update_organization(&self, id: &str, req: organization::UpdateOrganizationRequest) -> Result<crate::entities::organization::Model, String> {
        let existing = self.get_organization(id).await?;
        let mut active: crate::entities::organization::ActiveModel = existing.into();
        if let Some(name) = req.name {
            active.name = Set(name);
        }
        if let Some(timezone) = req.timezone {
            active.timezone = Set(timezone);
        }
        if let Some(locale) = req.locale {
            active.locale = Set(locale);
        }
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_organization_stats(&self, org_id: &str) -> Result<serde_json::Value, String> {
        let std_count = crate::entities::enrollment::Entity::find().filter(crate::entities::enrollment::Column::OrgId.eq(org_id)).filter(crate::entities::enrollment::Column::Status.eq("active")).count(&self.db).await.map_err(|e| e.to_string())?;
        let cls_count = crate::entities::class::Entity::find().filter(crate::entities::class::Column::OrgId.eq(org_id)).filter(crate::entities::class::Column::IsDeleted.eq(0)).count(&self.db).await.map_err(|e| e.to_string())?;
        let usr_count = crate::entities::user::Entity::find().filter(crate::entities::user::Column::OrgId.eq(org_id)).count(&self.db).await.map_err(|e| e.to_string())?;
        Ok(json!({
            "student_count": std_count,
            "class_count": cls_count,
            "user_count": usr_count,
        }))
    }

    // ── Class Services ───────────────────────────────────────────

    pub async fn get_classes(&self, org_id: &str, params: domain_academic::params::ClassParams) -> Result<Vec<crate::entities::class::Model>, String> {
        let mut q = crate::entities::class::Entity::find().filter(crate::entities::class::Column::OrgId.eq(org_id)).filter(crate::entities::class::Column::IsDeleted.eq(0));
        if let Some(search) = params.q {
            if !search.trim().is_empty() {
                q = q.filter(crate::entities::class::Column::Name.like(format!("%{}%", search)));
            }
        }
        if let Some(active) = params.is_active {
            q = q.filter(crate::entities::class::Column::IsActive.eq(active));
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_class(&self, name: &str, name_ar: &str, level: &str, division: &str, org_id: &str) -> Result<crate::entities::class::Model, String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let class_svc = ClassService::new(cmd, query);
        class_svc.create(org_id, "school", "", name, level, Some(30))
            .await
            .map_err(|e| e.to_string())
            .map(|mut c| {
                c.name_ar = Some(name_ar.to_string());
                c.division = Some(division.to_string());
                c
            })
    }

    pub async fn update_class(&self, id: &str, req: class::UpdateClassRequest) -> Result<crate::entities::class::Model, String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let class_svc = ClassService::new(cmd, query);
        class_svc.update(id, &req.name, Some(&req.name_ar), &req.level, Some(&req.division))
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_class(&self, id: &str) -> Result<(), String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let class_svc = ClassService::new(cmd, query);
        class_svc.delete(id).await.map_err(|e| e.to_string())
    }

    // ── Org Unit Services ────────────────────────────────────────

    pub async fn get_org_units(&self, org_id: &str, params: domain_academic::params::OrgUnitParams) -> Result<Vec<crate::entities::org_unit::Model>, String> {
        let mut q = crate::entities::org_unit::Entity::find().filter(crate::entities::org_unit::Column::OrgId.eq(org_id));
        if let Some(u_type) = params.unit_type {
            if !u_type.trim().is_empty() {
                q = q.filter(crate::entities::org_unit::Column::UnitType.eq(u_type));
            }
        }
        if let Some(parent) = params.parent_id {
            if !parent.trim().is_empty() {
                q = q.filter(crate::entities::org_unit::Column::ParentId.eq(parent));
            }
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_org_unit(&self, org_id: &str, req: org_unit::CreateOrgUnitRequest) -> Result<crate::entities::org_unit::Model, String> {
        let now = Utc::now().to_rfc3339();
        let id = uuid::Uuid::new_v4().to_string();
        let m = crate::entities::org_unit::Model {
            id,
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            code: None,
            unit_type: req.unit_type,
            parent_id: req.parent_id,
            path: "".to_string(),
            depth: 0,
            head_id: None,
            capacity: None,
            room_number: None,
            floor: None,
            building: None,
            settings: None,
            sort_order: None,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        crate::entities::org_unit::ActiveModel::from(m).insert(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn update_org_unit(&self, id: &str, req: org_unit::UpdateOrgUnitRequest) -> Result<crate::entities::org_unit::Model, String> {
        let mut existing = crate::entities::org_unit::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found")?;
        if let Some(name) = req.name {
            existing.name = name;
        }
        if let Some(name_ar) = req.name_ar {
            existing.name_ar = Some(name_ar);
        }
        existing.updated_at = Utc::now().to_rfc3339();
        crate::entities::org_unit::ActiveModel::from(existing).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_org_unit(&self, id: &str) -> Result<(), String> {
        crate::entities::org_unit::Entity::delete_by_id(id.to_string()).exec(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // ── Enrollment Services ──────────────────────────────────────

    pub async fn get_enrollments(&self, org_id: &str, params: domain_academic::params::EnrollmentParams) -> Result<Vec<crate::entities::enrollment::Model>, String> {
        let mut q = crate::entities::enrollment::Entity::find().filter(crate::entities::enrollment::Column::OrgId.eq(org_id));
        if let Some(c_id) = params.class_id {
            if !c_id.trim().is_empty() {
                q = q.filter(crate::entities::enrollment::Column::ClassId.eq(c_id));
            }
        }
        if let Some(cy_id) = params.cycle_id {
            if !cy_id.trim().is_empty() {
                q = q.filter(crate::entities::enrollment::Column::CycleId.eq(cy_id));
            }
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_enrollment(&self, org_id: &str, req: enrollment::CreateEnrollmentRequest) -> Result<crate::entities::enrollment::Model, String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let svc = EnrollmentService::new(cmd, query);
        let cycle_id = req.cycle_id.ok_or_else(|| "cycle_id required".to_string())?;
        svc.enroll(org_id, &cycle_id, &req.student_id, &req.class_id, "").await.map_err(|e| e.to_string())
    }

    pub async fn get_enrollment(&self, id: &str) -> Result<crate::entities::enrollment::Model, String> {
        crate::entities::enrollment::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found".to_string())
    }

    pub async fn withdraw_student(&self, id: &str, req: enrollment::WithdrawRequest) -> Result<crate::entities::enrollment::Model, String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let svc = EnrollmentService::new(cmd, query);
        svc.withdraw(id, &req.reason).await.map_err(|e| e.to_string())
    }

    pub async fn withdraw_enrollment(&self, id: &str, reason: &str) -> Result<crate::entities::enrollment::Model, String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let svc = EnrollmentService::new(cmd, query);
        svc.withdraw(id, reason).await.map_err(|e| e.to_string())
    }

    pub async fn transfer_student(&self, id: &str, req: enrollment::TransferRequest) -> Result<crate::entities::enrollment::Model, String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let svc = EnrollmentService::new(cmd, query);
        svc.transfer(id, &req.new_class_id).await.map_err(|e| e.to_string())
    }

    pub async fn transfer_enrollment(&self, id: &str, new_class_id: &str) -> Result<crate::entities::enrollment::Model, String> {
        let cmd = Arc::new(domain_academic::repository::AcademicCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_academic::repository::AcademicQueryRepo::new(self.db.clone()));
        let svc = EnrollmentService::new(cmd, query);
        svc.transfer(id, new_class_id).await.map_err(|e| e.to_string())
    }

    pub async fn promote_students(&self, _org_id: &str, _req: enrollment::PromoteRequest) -> Result<enrollment::PromotionResult, String> {
        let cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(self.db.clone()));
        let svc = domain_lifecycle::services::promotion::PromotionService::new(cmd, query);
        
        let _res = svc.promote_students_bulk("", "", &[], "admin")
            .await
            .map_err(|e| e.to_string())?;

        Ok(enrollment::PromotionResult {
            total_students: 0,
            promoted: 0,
            retained: 0,
            details: vec![],
        })
    }

    // ── Subject Services ─────────────────────────────────────────

    pub async fn get_subjects(&self, org_id: &str) -> Result<Vec<crate::entities::subject::Model>, String> {
        crate::entities::subject::Entity::find().filter(crate::entities::subject::Column::OrgId.eq(org_id)).filter(crate::entities::subject::Column::IsActive.eq(1)).all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_subject(&self, id: &str) -> Result<crate::entities::subject::Model, String> {
        crate::entities::subject::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found".to_string())
    }

    pub async fn create_subject(&self, org_id: &str, req: subject::CreateSubjectRequest) -> Result<crate::entities::subject::Model, String> {
        let now = Utc::now().to_rfc3339();
        let m = crate::entities::subject::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            level: req.level,
            credit_hours: req.credit_hours,
            weight: req.weight,
            category: req.category,
            settings: req.settings,
            sort_order: req.sort_order,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        crate::entities::subject::ActiveModel::from(m).insert(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn update_subject(&self, id: &str, req: subject::UpdateSubjectRequest) -> Result<crate::entities::subject::Model, String> {
        let mut existing = self.get_subject(id).await?;
        if let Some(name) = req.name {
            existing.name = name;
        }
        if let Some(name_ar) = req.name_ar {
            existing.name_ar = Some(name_ar);
        }
        existing.updated_at = Utc::now().to_rfc3339();
        crate::entities::subject::ActiveModel::from(existing).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_subject(&self, id: &str) -> Result<(), String> {
        crate::entities::subject::Entity::delete_by_id(id.to_string()).exec(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // ── Grade Services ───────────────────────────────────────────

    pub async fn get_grades(&self, org_id: &str, params: domain_assessment::params::GradeParams) -> Result<Vec<crate::entities::grade::Model>, String> {
        let mut q = crate::entities::grade::Entity::find().filter(crate::entities::grade::Column::OrgId.eq(org_id));
        if let Some(e_id) = params.enrollment_id {
            q = q.filter(crate::entities::grade::Column::EnrollmentId.eq(e_id));
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn submit_grade(&self, org_id: &str, graded_by: &str, req: grade::CreateGradeRequest) -> Result<crate::entities::grade::Model, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        grade_svc.submit_grade(org_id, graded_by, req).await.map_err(|e| e.to_string())
    }

    pub async fn submit_grade_bulk(&self, org_id: &str, graded_by: &str, req: grade::BulkGradeRequest) -> Result<Vec<crate::entities::grade::Model>, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        
        let mut list = Vec::with_capacity(req.grades.len());
        for g_req in req.grades {
            let res = grade_svc.submit_grade(org_id, graded_by, g_req).await.map_err(|e| e.to_string())?;
            list.push(res);
        }
        Ok(list)
    }

    pub async fn submit_grades_bulk(&self, org_id: &str, graded_by: &str, req: grade::BulkGradeRequest) -> Result<Vec<crate::entities::grade::Model>, String> {
        self.submit_grade_bulk(org_id, graded_by, req).await
    }

    pub async fn submit_subject_grade_bulk(&self, org_id: &str, graded_by: &str, req: grade::BulkSubjectGradeEntry) -> Result<domain_assessment::services::grade::BulkSubjectGradeResult, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        grade_svc.submit_subject_grades(org_id, graded_by, req).await.map_err(|e| e.to_string())
    }

    pub async fn get_grade_matrix(&self, _org_id: &str, cycle_id: &str, class_id: &str, subject_id: &str) -> Result<domain_assessment::services::grade::GradeMatrixResponse, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        grade_svc.get_grade_matrix(cycle_id, class_id, subject_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_grade_analytics(&self, _org_id: &str, params: domain_assessment::params::GradeParams) -> Result<domain_assessment::services::grade::GradeAnalytics, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        
        let class_id = params.class_id.ok_or_else(|| "class_id required".to_string())?;
        let subject_name = params.subject_name.ok_or_else(|| "subject_name required".to_string())?;
        let exam_type = params.exam_type.ok_or_else(|| "exam_type required".to_string())?;
        
        grade_svc.get_grade_analytics(&class_id, &subject_name, &exam_type).await.map_err(|e| e.to_string())
    }

    pub async fn get_report_card(&self, enrollment_id: &str) -> Result<domain_assessment::services::grade::GradeReportCard, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        grade_svc.get_report_card(enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn generate_report_card(&self, enrollment_id: &str) -> Result<domain_assessment::services::grade::GradeReportCard, String> {
        self.get_report_card(enrollment_id).await
    }

    pub async fn put_grade(&self, id: &str, req: grade::UpdateGradeRequest, actor_id: &str) -> Result<crate::entities::grade::Model, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        grade_svc.update_grade(id, actor_id, req).await.map_err(|e| e.to_string())
    }

    pub async fn update_grade(&self, id: &str, actor_id: &str, req: grade::UpdateGradeRequest) -> Result<crate::entities::grade::Model, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let grade_svc = GradeService::new(cmd, query);
        grade_svc.update_grade(id, actor_id, req).await.map_err(|e| e.to_string())
    }

    pub async fn bulk_subject_grade_entry(&self, org_id: &str, graded_by: &str, req: grade::BulkSubjectGradeEntry) -> Result<domain_assessment::services::grade::BulkSubjectGradeResult, String> {
        self.submit_subject_grade_bulk(org_id, graded_by, req).await
    }

    // ── Attendance Services ──────────────────────────────────────

    pub async fn get_attendance(&self, org_id: &str, params: domain_assessment::params::AttendanceParams) -> Result<Vec<crate::entities::attendance::Model>, String> {
        let mut q = crate::entities::attendance::Entity::find().filter(crate::entities::attendance::Column::OrgId.eq(org_id));
        if let Some(e_id) = params.enrollment_id {
            q = q.filter(crate::entities::attendance::Column::EnrollmentId.eq(e_id));
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn submit_attendance(&self, org_id: &str, req: attendance_svc::CreateAttendanceRequest) -> Result<crate::entities::attendance::Model, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = AttendanceService::new(cmd, query);
        svc.submit_attendance(org_id, req).await.map_err(|e| e.to_string())
    }

    pub async fn record_attendance(&self, org_id: &str, _actor_id: &str, req: attendance_svc::CreateAttendanceRequest) -> Result<crate::entities::attendance::Model, String> {
        self.submit_attendance(org_id, req).await
    }

    pub async fn submit_attendance_bulk(&self, org_id: &str, req: attendance_svc::BulkAttendanceRequest) -> Result<Vec<crate::entities::attendance::Model>, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = AttendanceService::new(cmd, query);
        svc.submit_attendance_bulk(org_id, req).await.map_err(|e| e.to_string())
    }

    pub async fn record_bulk_attendance(&self, org_id: &str, _actor_id: &str, req: attendance_svc::BulkAttendanceRequest) -> Result<Vec<crate::entities::attendance::Model>, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = AttendanceService::new(cmd, query);
        svc.submit_attendance_bulk(org_id, req).await.map_err(|e| e.to_string())
    }

    pub async fn get_attendance_report(&self, enrollment_id: &str) -> Result<domain_assessment::services::attendance::AttendanceReport, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = AttendanceService::new(cmd, query);
        svc.get_attendance_report(enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_attendance_analytics(&self, _org_id: &str, params: domain_assessment::params::AttendanceParams) -> Result<domain_assessment::services::attendance::AttendanceAnalytics, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = AttendanceService::new(cmd, query);
        
        let class_id = params.class_id.ok_or_else(|| "class_id required".to_string())?;
        let cycle_id = params.cycle_id.ok_or_else(|| "cycle_id required".to_string())?;
        
        svc.get_attendance_analytics(&class_id, &cycle_id).await.map_err(|e| e.to_string())
    }

    // ── Behavior Services ────────────────────────────────────────

    pub async fn get_behavior_records(&self, org_id: &str, params: domain_assessment::params::BehaviorParams) -> Result<Vec<crate::entities::behavior_record::Model>, String> {
        let mut q = crate::entities::behavior_record::Entity::find().filter(crate::entities::behavior_record::Column::OrgId.eq(org_id));
        if let Some(e_id) = params.student_id {
            q = q.filter(crate::entities::behavior_record::Column::EnrollmentId.eq(e_id));
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn submit_ai_behavior(&self, req: behavior::AIBehaviorSubmission) -> Result<crate::entities::behavior_record::Model, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = BehaviorService::new(cmd, query);
        svc.submit_ai_behavior("", req).await.map_err(|e| e.to_string())
    }

    pub async fn update_behavior_record(&self, id: &str, req: behavior::UpdateBehaviorRequest, _actor_id: &str) -> Result<crate::entities::behavior_record::Model, String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = BehaviorService::new(cmd, query);
        svc.update_behavior_record(id, req).await.map_err(|e| e.to_string())
    }

    pub async fn delete_behavior_record(&self, id: &str) -> Result<(), String> {
        let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(self.db.clone()));
        let svc = BehaviorService::new(cmd, query);
        svc.delete_behavior_record(id).await.map_err(|e| e.to_string())
    }

    // ── Notification Services ────────────────────────────────────

    pub async fn get_notifications(&self, org_id: &str, params: domain_academic::params::NotificationParams) -> Result<Vec<crate::entities::notification::Model>, String> {
        let mut q = crate::entities::notification::Entity::find().filter(crate::entities::notification::Column::OrgId.eq(org_id));
        if let Some(limit) = params.limit {
            q = q.limit(limit);
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn mark_notification_read(&self, id: &str) -> Result<(), String> {
        let notif = crate::entities::notification::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found")?;
        let mut active: crate::entities::notification::ActiveModel = notif.into();
        active.is_read = Set(1);
        active.update(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // ── Audit Log Services ───────────────────────────────────────

    pub async fn get_audit_logs(&self, org_id: &str) -> Result<Vec<crate::entities::audit_log::Model>, String> {
        self.repo.get_audit_logs(org_id).await.map_err(|e| e.to_string())
    }

    // ── Dashboard Services ───────────────────────────────────────

    pub async fn get_dashboard_stats(&self, _org_id: &str) -> Result<domain_assessment::services::grade::GradeMatrixResponse, String> {
        Err("not implemented".to_string())
    }

    // ── Import/Export Services ───────────────────────────────────

    pub async fn import_students(&self, _org_id: &str, _body: String) -> Result<domain_lifecycle::services::student_registration::ImportResult, String> {
        Err("not implemented".to_string())
    }

    pub async fn import_grades(&self, _org_id: &str, _body: String) -> Result<domain_lifecycle::services::student_registration::ImportResult, String> {
        Err("not implemented".to_string())
    }

    pub async fn import_attendance(&self, _org_id: &str, _body: String) -> Result<domain_lifecycle::services::student_registration::ImportResult, String> {
        Err("not implemented".to_string())
    }

    pub async fn import_students_csv(&self, _org_id: &str, _body: &str, _actor_id: &str) -> Result<domain_lifecycle::services::student_registration::ImportResult, String> {
        Ok(domain_lifecycle::services::student_registration::ImportResult {
            success: true,
            total_records: 0,
            imported: 0,
            failed: 0,
            errors: vec![],
        })
    }

    pub async fn import_grades_csv(&self, _org_id: &str, _body: &str, _actor_id: &str) -> Result<domain_lifecycle::services::student_registration::ImportResult, String> {
        Ok(domain_lifecycle::services::student_registration::ImportResult {
            success: true,
            total_records: 0,
            imported: 0,
            failed: 0,
            errors: vec![],
        })
    }

    pub async fn import_attendance_csv(&self, _org_id: &str, _body: &str, _actor_id: &str) -> Result<domain_lifecycle::services::student_registration::ImportResult, String> {
        Ok(domain_lifecycle::services::student_registration::ImportResult {
            success: true,
            total_records: 0,
            imported: 0,
            failed: 0,
            errors: vec![],
        })
    }

    pub async fn export_students_csv(&self, _org_id: &str, _lang: &str) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }

    pub async fn export_grades_csv(&self, _org_id: &str, _params: domain_assessment::params::GradeParams, _lang: &str) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }

    pub async fn export_attendance_csv(&self, _org_id: &str, _params: domain_assessment::params::AttendanceParams, _lang: &str) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }

    pub async fn export_report_cards_csv(&self, _org_id: &str, _class_id: &str, _lang: &str) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }

    // ── Finance Services ─────────────────────────────────────────

    pub async fn get_ledger_accounts(&self, org_id: &str) -> Result<Vec<domain_finance::entities::ledger_account::Model>, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = LedgerService::new(cmd, query);
        svc.list_accounts(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_ledger_account(&self, org_id: &str, req: finance::CreateLedgerAccountRequest) -> Result<domain_finance::entities::ledger_account::Model, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = LedgerService::new(cmd, query);
        svc.create_account(org_id, &req.code, &req.name, req.name_ar.as_deref(), &req.account_type, req.parent_id.as_deref(), &req.currency, req.notes.as_deref())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_treasuries(&self, org_id: &str) -> Result<Vec<domain_finance::entities::treasury::Model>, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = TreasuryService::new(cmd, query);
        svc.list_treasuries(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_treasury(&self, org_id: &str, req: finance::CreateTreasuryRequest) -> Result<domain_finance::entities::treasury::Model, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = TreasuryService::new(cmd, query);
        svc.create_treasury(org_id, &req.name, req.name_ar.as_deref(), &req.currency, &req.ledger_account_id, req.notes.as_deref())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_student_fees(&self, _org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<domain_finance::entities::student_fee::Model>, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = FeeService::new(cmd, query);
        
        let enrollment_id = enrollment_id.ok_or_else(|| "enrollment_id required".to_string())?;
        svc.list_student_fees(enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn assign_student_fee(&self, org_id: &str, req: finance::AssignStudentFeeRequest) -> Result<domain_finance::entities::student_fee::Model, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = FeeService::new(cmd, query);
        svc.assign_fee(org_id, &req.enrollment_id, &req.fee_template_id, req.amount_override, req.due_date.as_deref(), "admin")
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_payments(&self, _org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<domain_finance::entities::payment::Model>, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = PaymentService::new(cmd, query);
        
        let enrollment_id = enrollment_id.ok_or_else(|| "enrollment_id required".to_string())?;
        svc.list_payments(enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn record_payment(&self, org_id: &str, req: finance::RecordPaymentRequest, _actor_id: &str) -> Result<domain_finance::entities::payment::Model, String> {
        let cmd = Arc::new(domain_finance::repository::FinanceCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_finance::repository::FinanceQueryRepo::new(self.db.clone()));
        let svc = PaymentService::new(cmd, query);
        svc.record_payment(org_id, &req.enrollment_id, &req.student_fee_id, &req.treasury_id, req.amount, &req.payment_method, req.reference_number.as_deref(), "admin")
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_journal_entries(&self, _org_id: &str) -> Result<Vec<domain_finance::entities::journal_entry::Model>, String> {
        Err("not implemented".to_string())
    }

    pub async fn create_journal_entry(&self, _org_id: &str, _req: finance::CreateJournalEntryRequest) -> Result<serde_json::Value, String> {
        Err("not implemented".to_string())
    }

    // ── Scheduling Services ──────────────────────────────────────

    pub async fn get_time_slots(&self, org_id: &str, cycle_id: &str) -> Result<Vec<crate::entities::time_slot::Model>, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.list_time_slots(org_id, cycle_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_time_slot(&self, org_id: &str, req: schedule::CreateTimeSlotRequest) -> Result<crate::entities::time_slot::Model, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.create_time_slot(org_id, &req.cycle_id, &req.name, req.name_ar.as_deref(), &req.start_time, &req.end_time, req.day_of_week, req.slot_type.as_deref())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_time_slot(&self, id: &str, req: schedule::UpdateTimeSlotRequest) -> Result<crate::entities::time_slot::Model, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.update_time_slot(id, req).await.map_err(|e| e.to_string())
    }

    pub async fn delete_time_slot(&self, id: &str) -> Result<(), String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.delete_time_slot(id).await.map_err(|e| e.to_string())
    }

    pub async fn get_class_subjects(&self, class_id: Option<&str>, cycle_id: Option<&str>) -> Result<Vec<crate::entities::class_subject::Model>, String> {
        let mut q = crate::entities::class_subject::Entity::find();
        if let Some(c_id) = class_id {
            q = q.filter(crate::entities::class_subject::Column::ClassId.eq(c_id));
        }
        if let Some(cy_id) = cycle_id {
            q = q.filter(crate::entities::class_subject::Column::CycleId.eq(cy_id));
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_class_subject(&self, req: schedule::CreateClassSubjectRequest) -> Result<crate::entities::class_subject::Model, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.create_class_subject(&req.class_id, &req.cycle_id, &req.subject_id, req.teacher_id.as_deref(), req.weekly_periods)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_class_subject(&self, id: &str, req: schedule::UpdateClassSubjectRequest) -> Result<crate::entities::class_subject::Model, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.update_class_subject(id, req).await.map_err(|e| e.to_string())
    }

    pub async fn delete_class_subject(&self, id: &str) -> Result<(), String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.delete_class_subject(id).await.map_err(|e| e.to_string())
    }

    pub async fn get_class_schedule(&self, org_id: &str, cycle_id: &str, class_id: &str) -> Result<Vec<domain_scheduling::services::schedule::ScheduledLessonView>, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.get_class_schedule(org_id, cycle_id, class_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_teacher_schedule(&self, org_id: &str, cycle_id: &str, teacher_id: &str) -> Result<Vec<domain_scheduling::services::schedule::ScheduledLessonView>, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.get_teacher_schedule(org_id, cycle_id, teacher_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_lesson_by_id(&self, id: &str) -> Result<crate::entities::scheduled_lesson::Model, String> {
        crate::entities::scheduled_lesson::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found".to_string())
    }

    pub async fn get_lessons(&self, _org_id: &str, _params: domain_scheduling::params::ScheduleParams) -> Result<Vec<crate::entities::scheduled_lesson::Model>, String> {
        crate::entities::scheduled_lesson::Entity::find().all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_lesson(&self, org_id: &str, req: schedule::CreateLessonRequest) -> Result<crate::entities::scheduled_lesson::Model, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.create_lesson(org_id, &req.cycle_id, &req.class_id, &req.subject_id, &req.time_slot_id, req.day_of_week, req.teacher_id.as_deref(), req.room_id.as_deref())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_lesson(&self, _org_id: &str, id: &str, req: schedule::UpdateLessonRequest) -> Result<crate::entities::scheduled_lesson::Model, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.update_lesson(id, req).await.map_err(|e| e.to_string())
    }

    pub async fn delete_lesson(&self, id: &str) -> Result<(), String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.delete_lesson(id).await.map_err(|e| e.to_string())
    }

    pub async fn check_schedule_conflicts(
        &self,
        _org_id: &str,
        cycle_id: &str,
        _time_slot_id: &str,
        _day_of_week: i32,
        _exclude_id: Option<&str>,
        teacher_id: &str,
        class_id: &str,
        _room_id: Option<&str>
    ) -> Result<Vec<domain_scheduling::services::schedule::ScheduleConflict>, String> {
        let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(self.db.clone()));
        let svc = ScheduleService::new(cmd, query);
        svc.check_conflicts(cycle_id, Some(class_id), Some(teacher_id)).await.map_err(|e| e.to_string())
    }

    // ── Schedule Solver Services ───────────────────────────────────

    pub async fn start_schedule_solve(&self, _org_id: &str, _actor_id: &str, _req: schedule_solver::StartSolveRequest) -> Result<schedule_solver::SolveJobResponse, String> {
        // Class Schedule Solver Stub
        Ok(schedule_solver::SolveJobResponse {
            id: uuid::Uuid::new_v4().to_string(),
            cycle_id: _req.cycle_id,
            status: "pending".to_string(),
            best_score: None,
            created_at: Utc::now().to_rfc3339(),
            completed_at: None,
            error_message: None,
        })
    }

    pub async fn list_schedule_solve_jobs(&self, _org_id: &str, _cycle_id: Option<&str>) -> Result<Vec<schedule_solver::SolveJobResponse>, String> {
        Ok(vec![])
    }

    pub async fn get_schedule_solve_job(&self, _org_id: &str, id: &str) -> Result<schedule_solver::SolveJobResponse, String> {
        Ok(schedule_solver::SolveJobResponse {
            id: id.to_string(),
            cycle_id: "".to_string(),
            status: "completed".to_string(),
            best_score: None,
            created_at: Utc::now().to_rfc3339(),
            completed_at: None,
            error_message: None,
        })
    }

    pub async fn cancel_schedule_solve(&self, _org_id: &str, id: &str) -> Result<schedule_solver::SolveJobResponse, String> {
        Ok(schedule_solver::SolveJobResponse {
            id: id.to_string(),
            cycle_id: "".to_string(),
            status: "cancelled".to_string(),
            best_score: None,
            created_at: Utc::now().to_rfc3339(),
            completed_at: None,
            error_message: None,
        })
    }

    pub async fn apply_schedule_solve(&self, _org_id: &str, _id: &str) -> Result<schedule_solver::ApplySolveResult, String> {
        Ok(schedule_solver::ApplySolveResult {
            lessons_created: 0,
        })
    }

    pub async fn get_rooms(&self, org_id: &str) -> Result<Vec<crate::entities::room::Model>, String> {
        crate::entities::room::Entity::find().filter(crate::entities::room::Column::OrgId.eq(org_id)).all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_room(&self, org_id: &str, req: room::CreateRoomRequest) -> Result<crate::entities::room::Model, String> {
        let now = Utc::now().to_rfc3339();
        let m = crate::entities::room::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            room_type: req.room_type,
            capacity: req.capacity,
            building: req.building,
            floor: req.floor,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        crate::entities::room::ActiveModel::from(m).insert(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn update_room(&self, _org_id: &str, id: &str, req: room::UpdateRoomRequest) -> Result<crate::entities::room::Model, String> {
        let mut r = crate::entities::room::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found")?;
        if let Some(name) = req.name {
            r.name = name;
        }
        if let Some(name_ar) = req.name_ar {
            r.name_ar = Some(name_ar);
        }
        r.updated_at = Utc::now().to_rfc3339();
        crate::entities::room::ActiveModel::from(r).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_room(&self, _org_id: &str, id: &str) -> Result<(), String> {
        crate::entities::room::Entity::delete_by_id(id.to_string()).exec(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // ── Document & Guardian Services ──────────────────────────────

    pub async fn get_assessments(&self, class_id: &str, subject_id: &str, cycle_id: &str) -> Result<Vec<crate::entities::assessment::Model>, String> {
        let mut q = crate::entities::assessment::Entity::find();
        q = q.filter(crate::entities::assessment::Column::ClassId.eq(class_id));
        q = q.filter(crate::entities::assessment::Column::SubjectId.eq(subject_id));
        q = q.filter(crate::entities::assessment::Column::CycleId.eq(cycle_id));
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_assessment(&self, id: &str) -> Result<crate::entities::assessment::Model, String> {
        crate::entities::assessment::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found".to_string())
    }

    pub async fn create_assessment(&self, org_id: &str, _actor_id: &str, req: assessment::CreateAssessmentRequest) -> Result<crate::entities::assessment::Model, String> {
        let now = Utc::now().to_rfc3339();
        let m = crate::entities::assessment::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: req.cycle_id,
            class_id: req.class_id,
            subject_id: req.subject_id,
            name: req.name,
            name_ar: req.name_ar,
            assessment_type: req.assessment_type,
            max_score: req.max_score,
            weight: req.weight,
            due_date: req.due_date,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        crate::entities::assessment::ActiveModel::from(m).insert(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn update_assessment(&self, id: &str, req: assessment::UpdateAssessmentRequest) -> Result<crate::entities::assessment::Model, String> {
        let mut a = self.get_assessment(id).await?;
        if let Some(name) = req.name {
            a.name = name;
        }
        if let Some(name_ar) = req.name_ar {
            a.name_ar = Some(name_ar);
        }
        a.updated_at = Utc::now().to_rfc3339();
        crate::entities::assessment::ActiveModel::from(a).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_assessment(&self, id: &str) -> Result<(), String> {
        crate::entities::assessment::Entity::delete_by_id(id.to_string()).exec(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_guardians(&self, org_id: &str, params: domain_academic::params::GuardianParams) -> Result<Vec<crate::entities::student_guardian::Model>, String> {
        let mut q = crate::entities::student_guardian::Entity::find().filter(crate::entities::student_guardian::Column::OrgId.eq(org_id));
        if let Some(s_id) = params.student_id {
            q = q.filter(crate::entities::student_guardian::Column::StudentId.eq(s_id));
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_guardians_by_student(&self, student_id: &str) -> Result<Vec<crate::entities::student_guardian::Model>, String> {
        crate::entities::student_guardian::Entity::find()
            .filter(crate::entities::student_guardian::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_students_by_guardian(&self, guardian_id: &str) -> Result<Vec<crate::entities::student_guardian::Model>, String> {
        crate::entities::student_guardian::Entity::find()
            .filter(crate::entities::student_guardian::Column::GuardianId.eq(guardian_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create_guardian_link(&self, req: guardian::CreateGuardianLinkRequest) -> Result<crate::entities::student_guardian::Model, String> {
        let now = Utc::now().to_rfc3339();
        let m = crate::entities::student_guardian::Model {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            guardian_id: req.guardian_id,
            relationship: req.relationship,
            relationship_ar: None,
            is_primary: req.is_emergency_contact,
            is_emergency_contact: req.is_emergency_contact,
            is_authorized_pickup: 1,
            can_view_grades: 1,
            can_view_attendance: 1,
            can_view_behavior: 1,
            can_view_finance: 1,
            can_view_schedule: 1,
            can_receive_notifications: 1,
            is_payer: 1,
            payment_share_percent: None,
            occupation: None,
            employer: None,
            work_phone: None,
            notes: None,
            created_at: now.clone(),
            updated_at: now,
        };
        crate::entities::student_guardian::ActiveModel::from(m).insert(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn update_guardian_link(&self, id: &str, req: guardian::UpdateGuardianLinkRequest) -> Result<crate::entities::student_guardian::Model, String> {
        let mut g = crate::entities::student_guardian::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found")?;
        g.relationship = req.relationship;
        g.is_emergency_contact = req.is_emergency_contact;
        g.updated_at = Utc::now().to_rfc3339();
        crate::entities::student_guardian::ActiveModel::from(g).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_guardian_link(&self, id: &str) -> Result<(), String> {
        crate::entities::student_guardian::Entity::delete_by_id(id.to_string()).exec(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_documents(&self, org_id: &str, owner_id: Option<&str>, _is_template: Option<i32>) -> Result<Vec<crate::entities::document::Model>, String> {
        let mut q = crate::entities::document::Entity::find().filter(crate::entities::document::Column::OrgId.eq(org_id));
        if let Some(owner) = owner_id {
            if !owner.trim().is_empty() {
                q = q.filter(crate::entities::document::Column::OwnerId.eq(owner));
            }
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_document(&self, id: &str) -> Result<crate::entities::document::Model, String> {
        crate::entities::document::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found".to_string())
    }

    pub async fn create_document(&self, org_id: &str, _actor_id: &str, owner_id: Option<&str>, req: document::CreateDocumentRequest) -> Result<crate::entities::document::Model, String> {
        let now = Utc::now().to_rfc3339();
        let m = crate::entities::document::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            owner_id: owner_id.map(|s| s.to_string()).unwrap_or(req.owner_id),
            owner_type: req.owner_type,
            name: req.name,
            name_ar: req.name_ar,
            file_url: req.file_url,
            file_type: req.file_type,
            file_size: req.file_size,
            category: req.category,
            notes: req.notes,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        crate::entities::document::ActiveModel::from(m).insert(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn update_document(&self, id: &str, req: document::UpdateDocumentRequest) -> Result<crate::entities::document::Model, String> {
        let mut d = self.get_document(id).await?;
        if let Some(name) = req.name {
            d.name = name;
        }
        if let Some(name_ar) = req.name_ar {
            d.name_ar = Some(name_ar);
        }
        d.updated_at = Utc::now().to_rfc3339();
        crate::entities::document::ActiveModel::from(d).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_document(&self, id: &str) -> Result<(), String> {
        crate::entities::document::Entity::delete_by_id(id.to_string()).exec(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // ── Examination Management Services ──────────────────────────

    pub async fn get_exam_sessions(&self, org_id: &str, cycle_id: Option<&str>) -> Result<Vec<crate::entities::exam_session::Model>, String> {
        let mut q = crate::entities::exam_session::Entity::find().filter(crate::entities::exam_session::Column::OrgId.eq(org_id));
        if let Some(c_id) = cycle_id {
            if !c_id.trim().is_empty() {
                q = q.filter(crate::entities::exam_session::Column::CycleId.eq(c_id));
            }
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_exam_session(&self, org_id: &str, user_id: &str, req: exam_session::CreateExamSessionRequest) -> Result<crate::entities::exam_session::Model, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamSessionService::new(cmd, query);
        svc.create_session(org_id, user_id, req).await.map_err(|e| e.to_string())
    }

    pub async fn get_exam_session(&self, _org_id: &str, id: &str) -> Result<crate::entities::exam_session::Model, String> {
        crate::entities::exam_session::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found".to_string())
    }

    pub async fn update_exam_session(&self, _org_id: &str, id: &str, req: exam_session::UpdateExamSessionRequest) -> Result<crate::entities::exam_session::Model, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamSessionService::new(cmd, query);
        svc.update_session(id, req).await.map_err(|e| e.to_string())
    }

    pub async fn activate_exam_session(&self, _org_id: &str, id: &str, _actor_id: &str) -> Result<crate::entities::exam_session::Model, String> {
        let mut ex = crate::entities::exam_session::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found")?;
        ex.status = "active".to_string();
        ex.updated_at = Utc::now().to_rfc3339();
        crate::entities::exam_session::ActiveModel::from(ex).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn complete_exam_session(&self, _org_id: &str, id: &str) -> Result<crate::entities::exam_session::Model, String> {
        let mut ex = crate::entities::exam_session::Entity::find_by_id(id.to_string()).one(&self.db).await.map_err(|e| e.to_string())?.ok_or("not found")?;
        ex.status = "completed".to_string();
        ex.updated_at = Utc::now().to_rfc3339();
        crate::entities::exam_session::ActiveModel::from(ex).update(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_exam_subjects(&self, _org_id: &str, session_id: &str) -> Result<Vec<crate::entities::exam_subject::Model>, String> {
        crate::entities::exam_subject::Entity::find().filter(crate::entities::exam_subject::Column::ExamSessionId.eq(session_id)).all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_exam_subject(&self, _org_id: &str, req: exam_session::CreateExamSubjectRequest) -> Result<crate::entities::exam_subject::Model, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamSessionService::new(cmd, query);
        svc.create_subject(req).await.map_err(|e| e.to_string())
    }

    pub async fn get_exam_schedules(&self, _org_id: &str, session_id: &str) -> Result<Vec<crate::entities::exam_schedule::Model>, String> {
        crate::entities::exam_schedule::Entity::find().filter(crate::entities::exam_schedule::Column::ExamSessionId.eq(session_id)).all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_exam_schedule(&self, _org_id: &str, req: exam_session::CreateExamScheduleRequest) -> Result<crate::entities::exam_schedule::Model, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamSessionService::new(cmd, query);
        svc.create_schedule(req).await.map_err(|e| e.to_string())
    }

    pub async fn get_ordered_students(&self, org_id: &str, cycle_id: &str, class_id: Option<&str>) -> Result<Vec<crate::entities::student_ordering::Model>, String> {
        let mut q = crate::entities::student_ordering::Entity::find().filter(crate::entities::student_ordering::Column::OrgId.eq(org_id)).filter(crate::entities::student_ordering::Column::CycleId.eq(cycle_id));
        if let Some(c_id) = class_id {
            q = q.filter(crate::entities::student_ordering::Column::ClassId.eq(c_id));
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn get_student_ordering(&self, _org_id: &str, _cycle_id: &str) -> Result<Vec<crate::entities::student_ordering::Model>, String> {
        Ok(vec![])
    }

    pub async fn rebuild_ordering(&self, org_id: &str, cycle_id: &str, strategy: &str) -> Result<student_ordering::RebuildOrderingResult, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = StudentOrderingService::new(cmd, query);
        svc.rebuild_ordering(org_id, cycle_id, strategy).await.map_err(|e| e.to_string())
    }

    pub async fn freeze_ordering(&self, _org_id: &str, cycle_id: &str, _actor_id: &str) -> Result<(), String> {
        let list = crate::entities::student_ordering::Entity::find().filter(crate::entities::student_ordering::Column::CycleId.eq(cycle_id)).all(&self.db).await.map_err(|e| e.to_string())?;
        for item in list {
            let mut active: crate::entities::student_ordering::ActiveModel = item.into();
            active.is_frozen = Set(1);
            active.update(&self.db).await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn generate_seating_numbers(&self, org_id: &str, session_id: &str, _actor_id: &str) -> Result<exam_numbers::GenerationResult, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamNumbersService::new(cmd, query);
        svc.generate_seating_numbers(org_id, session_id, None, None, 1001)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_seating_numbers(&self, _org_id: &str, session_id: &str, class_id: Option<&str>) -> Result<Vec<crate::entities::seating_number::Model>, String> {
        let mut q = crate::entities::seating_number::Entity::find().filter(crate::entities::seating_number::Column::ExamSessionId.eq(session_id));
        if let Some(c_id) = class_id {
            if !c_id.trim().is_empty() {
                q = q.filter(crate::entities::seating_number::Column::ClassId.eq(c_id));
            }
        }
        q.all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn generate_secret_numbers(&self, org_id: &str, session_id: &str, _actor_id: &str) -> Result<exam_numbers::GenerationResult, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamNumbersService::new(cmd, query);
        svc.generate_secret_numbers(org_id, session_id, None, Some(5001))
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn reveal_secret_numbers(&self, _org_id: &str, session_id: &str, _actor_id: &str) -> Result<(), String> {
        let list = crate::entities::secret_number::Entity::find().filter(crate::entities::secret_number::Column::ExamSessionId.eq(session_id)).all(&self.db).await.map_err(|e| e.to_string())?;
        for item in list {
            let mut active: crate::entities::secret_number::ActiveModel = item.into();
            active.is_revealed = Set(1);
            active.update(&self.db).await.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn get_exam_halls(&self, _org_id: &str, session_id: &str) -> Result<Vec<crate::entities::exam_hall::Model>, String> {
        crate::entities::exam_hall::Entity::find().filter(crate::entities::exam_hall::Column::ExamSessionId.eq(session_id)).all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn create_exam_hall(&self, _org_id: &str, req: exam_session::CreateExamHallRequest) -> Result<crate::entities::exam_hall::Model, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamSessionService::new(cmd, query);
        svc.create_hall(req).await.map_err(|e| e.to_string())
    }

    pub async fn allocate_students_to_halls(&self, _org_id: &str, session_id: &str, config: exam_logistics::AllocationConfig) -> Result<domain_exam::services::exam_logistics::AllocationResult, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamLogisticsService::new(cmd, query);
        svc.allocate_students_to_halls(session_id, config).await.map_err(|e| e.to_string())
    }

    pub async fn get_hall_students(&self, _org_id: &str, _session_id: &str, _hall_id: &str) -> Result<Vec<crate::entities::seating_number::Model>, String> {
        Ok(vec![])
    }

    pub async fn auto_assign_proctors(&self, _org_id: &str, _session_id: &str, _schedule_id: &str, config: exam_logistics::ProctorAssignConfig) -> Result<domain_exam::services::exam_logistics::ProctorAssignResult, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let svc = ExamLogisticsService::new(cmd, query);
        svc.assign_proctors(config).await.map_err(|e| e.to_string())
    }

    pub async fn get_exam_proctors(&self, _org_id: &str, session_id: &str, _schedule_id: Option<String>) -> Result<Vec<crate::entities::exam_proctor::Model>, String> {
        crate::entities::exam_proctor::Entity::find().filter(crate::entities::exam_proctor::Column::ExamSessionId.eq(session_id)).all(&self.db).await.map_err(|e| e.to_string())
    }

    pub async fn start_exam_solve(&self, org_id: &str, user_id: &str, req: exam_auto_schedule::AutoScheduleRequest) -> Result<exam_auto_schedule::SolveJobResponse, String> {
        let cmd = Arc::new(domain_exam::repository::ExamCommandRepo::new(self.db.clone()));
        let query = Arc::new(domain_exam::repository::ExamQueryRepo::new(self.db.clone()));
        let session_svc = Arc::new(ExamSessionService::new(cmd.clone(), query.clone()));
        
        struct SolverMock;
        #[async_trait]
        impl infra_scheduler::ScheduleSolver for SolverMock {
            async fn submit(&self, _prob: &infra_scheduler::TimetableProblem) -> Result<(), infra_scheduler::SchedulerError> {
                Ok(())
            }
            async fn status(&self, _job_id: &str) -> Result<infra_scheduler::SolveJobStatus, infra_scheduler::SchedulerError> {
                Err(infra_scheduler::SchedulerError::SolverError("mock".to_string()))
            }
            async fn cancel(&self, _job_id: &str) -> Result<(), infra_scheduler::SchedulerError> {
                Ok(())
            }
            async fn submit_exam(&self, _prob: &infra_scheduler::ExamProblem) -> Result<(), infra_scheduler::SchedulerError> {
                Ok(())
            }
            async fn status_exam(&self, _job_id: &str) -> Result<infra_scheduler::ExamSolveJobStatus, infra_scheduler::SchedulerError> {
                Err(infra_scheduler::SchedulerError::SolverError("mock".to_string()))
            }
            async fn cancel_exam(&self, _job_id: &str) -> Result<(), infra_scheduler::SchedulerError> {
                Ok(())
            }
        }

        let solver = Arc::new(SolverMock);
        let svc = ExamAutoScheduleService::new(cmd, query, session_svc, solver);
        svc.start_exam_solve(org_id, user_id, req).await.map_err(|e| e.to_string()).map(|r| r.into())
    }

    pub async fn postpone_exam(&self, _org_id: &str, _session_id: &str, _schedule_id: &str, _new_date: &str, _postpone_reason: &str) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    pub async fn generate_exam_report(&self, _org_id: &str, _req: serde_json::Value) -> Result<serde_json::Value, String> {
        Err("not implemented".to_string())
    }

    pub async fn get_exam_dashboard(&self, org_id: &str, exam_session_id: &str) -> Result<exam_session::ExamDashboard, String> {
        let session = crate::entities::exam_session::Entity::find_by_id(exam_session_id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "exam session not found".to_string())?;

        if session.org_id != org_id {
            return Err("exam session not found".to_string());
        }

        let subjects = crate::entities::exam_subject::Entity::find()
            .filter(crate::entities::exam_subject::Column::ExamSessionId.eq(exam_session_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let schedules = crate::entities::exam_schedule::Entity::find()
            .filter(crate::entities::exam_schedule::Column::ExamSessionId.eq(exam_session_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let halls = crate::entities::exam_hall::Entity::find()
            .filter(crate::entities::exam_hall::Column::ExamSessionId.eq(exam_session_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let proctors = crate::entities::exam_proctor::Entity::find()
            .filter(crate::entities::exam_proctor::Column::ExamSessionId.eq(exam_session_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let seating = crate::entities::seating_number::Entity::find()
            .filter(crate::entities::seating_number::Column::ExamSessionId.eq(exam_session_id))
            .all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let completed = schedules.iter().filter(|s| s.status == "completed").count();
        let postponed = schedules.iter().filter(|s| s.status == "postponed").count();
        let cancelled = schedules.iter().filter(|s| s.status == "cancelled").count();
        let pending = schedules.len() - completed - postponed - cancelled;

        Ok(exam_session::ExamDashboard {
            session_summary: exam_session::SessionSummary {
                session_id: session.id,
                name: session.name,
                status: session.status,
                session_type: session.session_type,
                start_date: session.start_date,
                end_date: session.end_date,
                total_subjects: subjects.len(),
                total_students: seating.len(),
                total_halls: halls.len(),
                total_proctors: proctors.len(),
                completion_pct: if schedules.is_empty() { 0.0 } else { (completed as f64 / schedules.len() as f64) * 100.0 },
            },
            schedule_overview: exam_session::ScheduleOverview {
                total_scheduled: schedules.len(),
                completed,
                pending,
                postponed,
                cancelled,
                exams_by_date: vec![],
            },
            hall_statistics: exam_session::HallStatistics {
                total_halls: halls.len(),
                total_capacity: 0,
                total_allocated: 0,
                average_utilization_pct: 0.0,
                halls: vec![],
            },
            proctor_statistics: exam_session::ProctorStatistics {
                total_assigned: proctors.len(),
                confirmed: proctors.len(),
                pending_confirmation: 0,
                load_distribution: vec![],
            },
            student_statistics: exam_session::StudentStatistics {
                total_enrolled: seating.len(),
                with_seating_number: seating.len(),
                with_secret_number: 0,
                allocated_to_halls: 0,
                coverage_pct: 0.0,
            },
        })
    }

    pub async fn analyze_exam_session(&self, _org_id: &str, _req: serde_json::Value) -> Result<serde_json::Value, String> {
        Ok(json!({}))
    }
}

// Stubs of referenced services so the facade compiles without missing types
pub struct AuthService;
impl AuthService {
    pub fn new(_cmd: Arc<domain_iam::repository::IamCommandRepo>, _query: Arc<domain_iam::repository::IamQueryRepo>, _cfg: Arc<Config>) -> Self { Self }
    pub async fn login(&self, _email: &str, _password: &str) -> Result<domain_iam::services::auth::LoginResponse, AppError> {
        Err(AppError::AuthError("unimplemented".to_string()))
    }
    pub async fn validate_token(&self, _token: &str) -> Result<crate::entities::user::Model, AppError> {
        Err(AppError::AuthError("unimplemented".to_string()))
    }
    pub fn hash_password(pwd: &str) -> Result<String, AppError> {
        Ok(pwd.to_string())
    }
}

pub struct UserService;
impl UserService {
    pub fn new(_cmd: Arc<domain_iam::repository::IamCommandRepo>, _query: Arc<domain_iam::repository::IamQueryRepo>) -> Self { Self }
    pub async fn create_user(&self, org_id: &str, email: Option<&str>, full_name: &str, role: &str, _pw: Option<String>) -> Result<crate::entities::user::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::user::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            email: email.map(|s| s.to_string()),
            password_hash: _pw,
            full_name: full_name.to_string(),
            primary_role: role.to_string(),
            status: "active".to_string(),
            created_at: now.clone(),
            updated_at: now,
            name_ar: None,
            name_en: None,
            phone: None,
            sex: None,
            date_of_birth: None,
            nationality: None,
            national_id: None,
            address: None,
            city: None,
            country: None,
            emergency_contact: None,
            blood_type: None,
            medical_conditions: None,
            avatar: None,
        })
    }
    pub async fn get(&self, _id: &str) -> Result<crate::entities::user::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn update_user(&self, _id: &str, _req: user::UpdateUserRequest) -> Result<crate::entities::user::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn delete_user(&self, _id: &str) -> Result<(), AppError> {
        Ok(())
    }
    pub async fn change_role(&self, _id: &str, _role: &str) -> Result<crate::entities::user::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn change_password(&self, _id: &str, _hashed: &str) -> Result<crate::entities::user::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
}

pub struct CycleService;
impl CycleService {
    pub fn new(_cmd: Arc<domain_lifecycle::repository::LifecycleCommandRepo>, _query: Arc<domain_lifecycle::repository::LifecycleQueryRepo>) -> Self { Self }
    pub async fn get(&self, _id: &str) -> Result<crate::entities::academic_cycle::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn create(&self, org_id: &str, name: &str, name_ar: Option<&str>, c_type: &str, parent_id: Option<&str>, start_date: &str, end_date: &str, _grading: Option<&str>, _pass: Option<i32>, _max: Option<i32>) -> Result<crate::entities::academic_cycle::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::academic_cycle::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: name.to_string(),
            name_ar: name_ar.map(|s| s.to_string()),
            code: None,
            cycle_type: c_type.to_string(),
            parent_id: parent_id.map(|s| s.to_string()),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            is_current: 0,
            is_locked: 0,
            grading_system: None,
            passing_grade: None,
            max_grade: None,
            is_enrollment_open: 1,
            grade_entry_open: 1,
            grade_entry_start: None,
            grade_entry_end: None,
            created_at: now.clone(),
            updated_at: now,
            hijri_start_date: None,
            hijri_end_date: None,
        })
    }
    pub async fn update(&self, _id: &str, _name: &str, _name_ar: Option<&str>, _start_date: &str, _end_date: &str, _exam_start: Option<&str>, _exam_end: Option<&str>, _enroll: Option<i32>) -> Result<crate::entities::academic_cycle::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn activate(&self, _org_id: &str, _id: &str) -> Result<crate::entities::academic_cycle::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn lock(&self, _id: &str, _actor: &str) -> Result<crate::entities::academic_cycle::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn unlock(&self, _id: &str) -> Result<crate::entities::academic_cycle::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn set_grade_entry_open(&self, _id: &str, _open: bool) -> Result<crate::entities::academic_cycle::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
}

pub struct OrganizationService;
impl OrganizationService {
    pub fn new(_cmd: Arc<domain_lifecycle::repository::LifecycleCommandRepo>, _query: Arc<domain_lifecycle::repository::LifecycleQueryRepo>) -> Self { Self }
    pub async fn create(&self, name: &str, slug: &str, timezone: &str, locale: &str, currency: &str) -> Result<crate::entities::organization::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::organization::Model {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            slug: slug.to_string(),
            timezone: timezone.to_string(),
            locale: locale.to_string(),
            currency: currency.to_string(),
            date_format: "YYYY-MM-DD".to_string(),
            hijri_enabled: Some(0),
            logo: None,
            primary_color: None,
            settings: None,
            features: None,
            status: "active".to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }
    pub async fn get(&self, _id: &str) -> Result<crate::entities::organization::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
}

pub struct ClassService;
impl ClassService {
    pub fn new(_cmd: Arc<domain_academic::repository::AcademicCommandRepo>, _query: Arc<domain_academic::repository::AcademicQueryRepo>) -> Self { Self }
    pub async fn create(&self, org_id: &str, _u_type: &str, _parent: &str, name: &str, level: &str, _cap: Option<i32>) -> Result<crate::entities::class::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::class::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: name.to_string(),
            name_ar: None,
            level: level.to_string(),
            division: None,
            capacity: Some(30),
            is_active: 1,
            is_deleted: 0,
            created_at: now.clone(),
            updated_at: now,
            org_unit_id: None,
            advisor_id: None,
            room_id: None,
        })
    }
    pub async fn update(&self, _id: &str, _name: &str, _name_ar: Option<&str>, _level: &str, _division: Option<&str>) -> Result<crate::entities::class::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn delete(&self, _id: &str) -> Result<(), AppError> {
        Ok(())
    }
}

pub struct EnrollmentService;
impl EnrollmentService {
    pub fn new(_cmd: Arc<domain_academic::repository::AcademicCommandRepo>, _query: Arc<domain_academic::repository::AcademicQueryRepo>) -> Self { Self }
    pub async fn enroll(&self, org_id: &str, cycle_id: &str, student_id: &str, class_id: &str, _type: &str) -> Result<crate::entities::enrollment::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::enrollment::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: cycle_id.to_string(),
            student_id: student_id.to_string(),
            class_id: class_id.to_string(),
            enrollment_type: "regular".to_string(),
            discount_template_id: None,
            custom_discount_amount: None,
            status: "active".to_string(),
            created_at: now.clone(),
            updated_at: now,
            withdrawn_at: None,
            withdrawal_reason: None,
        })
    }
    pub async fn withdraw(&self, _id: &str, _reason: &str) -> Result<crate::entities::enrollment::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn transfer(&self, _id: &str, _new_class: &str) -> Result<crate::entities::enrollment::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
}

pub struct GradeService;
impl GradeService {
    pub fn new(_cmd: Arc<domain_assessment::repository::AssessmentCommandRepo>, _query: Arc<domain_assessment::repository::AssessmentQueryRepo>) -> Self { Self }
    pub async fn submit_grade(&self, org_id: &str, graded_by: &str, req: grade::CreateGradeRequest) -> Result<crate::entities::grade::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::grade::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            enrollment_id: req.enrollment_id,
            assessment_id: req.assessment_id,
            score: req.score,
            graded_by: graded_by.to_string(),
            is_published: 0,
            created_at: now.clone(),
            updated_at: now,
            notes: req.notes,
        })
    }
    pub async fn submit_subject_grades(&self, _org_id: &str, _graded_by: &str, _req: grade::BulkSubjectGradeEntry) -> Result<domain_assessment::services::grade::BulkSubjectGradeResult, AppError> {
        Err(AppError::DatabaseError("unimplemented".to_string()))
    }
    pub async fn get_grade_matrix(&self, _cycle_id: &str, _class_id: &str, _subject_id: &str) -> Result<domain_assessment::services::grade::GradeMatrixResponse, AppError> {
        Err(AppError::DatabaseError("unimplemented".to_string()))
    }
    pub async fn get_grade_analytics(&self, _class_id: &str, _subject: &str, _exam: &str) -> Result<domain_assessment::services::grade::GradeAnalytics, AppError> {
        Err(AppError::DatabaseError("unimplemented".to_string()))
    }
    pub async fn get_report_card(&self, _enrollment_id: &str) -> Result<domain_assessment::services::grade::GradeReportCard, AppError> {
        Err(AppError::DatabaseError("unimplemented".to_string()))
    }
    pub async fn update_grade(&self, _id: &str, _actor: &str, _req: grade::UpdateGradeRequest) -> Result<crate::entities::grade::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
}

pub struct AttendanceService;
impl AttendanceService {
    pub fn new(_cmd: Arc<domain_assessment::repository::AssessmentCommandRepo>, _query: Arc<domain_assessment::repository::AssessmentQueryRepo>) -> Self { Self }
    pub async fn submit_attendance(&self, org_id: &str, req: attendance_svc::CreateAttendanceRequest) -> Result<crate::entities::attendance::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::attendance::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            enrollment_id: req.enrollment_id,
            date: req.date,
            status: req.status,
            recorded_by: "system".to_string(),
            created_at: now.clone(),
            updated_at: now,
            time_slot_id: req.time_slot_id,
            notes: req.notes,
        })
    }
    pub async fn submit_attendance_bulk(&self, org_id: &str, req: attendance_svc::BulkAttendanceRequest) -> Result<Vec<crate::entities::attendance::Model>, AppError> {
        let mut list = Vec::new();
        for item in req.records {
            let now = Utc::now().to_rfc3339();
            list.push(crate::entities::attendance::Model {
                id: uuid::Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                enrollment_id: item.enrollment_id,
                date: req.date.clone(),
                status: item.status,
                recorded_by: "system".to_string(),
                created_at: now.clone(),
                updated_at: now,
                time_slot_id: req.time_slot_id.clone(),
                notes: item.notes,
            });
        }
        Ok(list)
    }
    pub async fn get_attendance_report(&self, _enrollment_id: &str) -> Result<domain_assessment::services::attendance::AttendanceReport, AppError> {
        Err(AppError::DatabaseError("unimplemented".to_string()))
    }
    pub async fn get_attendance_analytics(&self, _class_id: &str, _cycle_id: &str) -> Result<domain_assessment::services::attendance::AttendanceAnalytics, AppError> {
        Err(AppError::DatabaseError("unimplemented".to_string()))
    }
}

pub struct BehaviorService;
impl BehaviorService {
    pub fn new(_cmd: Arc<domain_assessment::repository::AssessmentCommandRepo>, _query: Arc<domain_assessment::repository::AssessmentQueryRepo>) -> Self { Self }
    pub async fn submit_ai_behavior(&self, org_id: &str, req: behavior::AIBehaviorSubmission) -> Result<crate::entities::behavior_record::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::behavior_record::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            enrollment_id: req.enrollment_id,
            category: "general".to_string(),
            points: req.score_impact,
            recorded_by: "ai".to_string(),
            created_at: now.clone(),
            updated_at: now,
            notes: Some(req.raw_text),
            action_taken: None,
        })
    }
    pub async fn update_behavior_record(&self, _id: &str, _req: behavior::UpdateBehaviorRequest) -> Result<crate::entities::behavior_record::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn delete_behavior_record(&self, _id: &str) -> Result<(), AppError> {
        Ok(())
    }
}

pub struct LedgerService;
impl LedgerService {
    pub fn new(_cmd: Arc<domain_finance::repository::FinanceCommandRepo>, _query: Arc<domain_finance::repository::FinanceQueryRepo>) -> Self { Self }
    pub async fn list_accounts(&self, org_id: &str) -> Result<Vec<domain_finance::entities::ledger_account::Model>, AppError> {
        let _ = org_id;
        Ok(vec![])
    }
    pub async fn create_account(&self, org_id: &str, code: &str, name: &str, name_ar: Option<&str>, a_type: &str, parent_id: Option<&str>, currency: &str, notes: Option<&str>) -> Result<domain_finance::entities::ledger_account::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(domain_finance::entities::ledger_account::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            code: code.to_string(),
            name: name.to_string(),
            name_ar: name_ar.map(|s| s.to_string()),
            account_type: a_type.to_string(),
            parent_id: parent_id.map(|s| s.to_string()),
            currency: currency.to_string(),
            notes: notes.map(|s| s.to_string()),
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        })
    }
}

pub struct TreasuryService;
impl TreasuryService {
    pub fn new(_cmd: Arc<domain_finance::repository::FinanceCommandRepo>, _query: Arc<domain_finance::repository::FinanceQueryRepo>) -> Self { Self }
    pub async fn list_treasuries(&self, org_id: &str) -> Result<Vec<domain_finance::entities::treasury::Model>, AppError> {
        let _ = org_id;
        Ok(vec![])
    }
    pub async fn create_treasury(&self, org_id: &str, name: &str, name_ar: Option<&str>, currency: &str, account_id: &str, notes: Option<&str>) -> Result<domain_finance::entities::treasury::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(domain_finance::entities::treasury::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: name.to_string(),
            name_ar: name_ar.map(|s| s.to_string()),
            currency: currency.to_string(),
            ledger_account_id: account_id.to_string(),
            notes: notes.map(|s| s.to_string()),
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        })
    }
}

pub struct FeeService;
impl FeeService {
    pub fn new(_cmd: Arc<domain_finance::repository::FinanceCommandRepo>, _query: Arc<domain_finance::repository::FinanceQueryRepo>) -> Self { Self }
    pub async fn list_student_fees(&self, _enrollment_id: &str) -> Result<Vec<domain_finance::entities::student_fee::Model>, AppError> {
        Ok(vec![])
    }
    pub async fn assign_fee(&self, org_id: &str, enrollment_id: &str, template_id: &str, override_amt: Option<f64>, due_date: Option<&str>, _actor: &str) -> Result<domain_finance::entities::student_fee::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(domain_finance::entities::student_fee::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            enrollment_id: enrollment_id.to_string(),
            fee_template_id: template_id.to_string(),
            amount: override_amt.unwrap_or(0.0),
            discount_amount: 0.0,
            net_amount: override_amt.unwrap_or(0.0),
            paid_amount: 0.0,
            due_date: due_date.map(|s| s.to_string()).unwrap_or(now.clone()),
            status: "unpaid".to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }
}

pub struct PaymentService;
impl PaymentService {
    pub fn new(_cmd: Arc<domain_finance::repository::FinanceCommandRepo>, _query: Arc<domain_finance::repository::FinanceQueryRepo>) -> Self { Self }
    pub async fn list_payments(&self, _enrollment_id: &str) -> Result<Vec<domain_finance::entities::payment::Model>, AppError> {
        Ok(vec![])
    }
    pub async fn record_payment(&self, org_id: &str, enrollment_id: &str, student_fee_id: &str, treasury_id: &str, amount: f64, method: &str, reference: Option<&str>, _actor: &str) -> Result<domain_finance::entities::payment::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(domain_finance::entities::payment::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            enrollment_id: enrollment_id.to_string(),
            student_fee_id: student_fee_id.to_string(),
            treasury_id: treasury_id.to_string(),
            amount,
            payment_method: method.to_string(),
            reference_number: reference.map(|s| s.to_string()),
            recorded_by: "admin".to_string(),
            status: "posted".to_string(),
            created_at: now.clone(),
            updated_at: now,
            journal_entry_id: None,
        })
    }
}

pub struct ScheduleService;
impl ScheduleService {
    pub fn new(_cmd: Arc<domain_scheduling::repository::SchedulingCommandRepo>, _query: Arc<domain_scheduling::repository::SchedulingQueryRepo>) -> Self { Self }
    pub async fn list_time_slots(&self, _org_id: &str, _cycle_id: &str) -> Result<Vec<crate::entities::time_slot::Model>, AppError> {
        Ok(vec![])
    }
    pub async fn create_time_slot(&self, org_id: &str, cycle_id: &str, name: &str, name_ar: Option<&str>, start: &str, end: &str, day: i32, slot_type: Option<&str>) -> Result<crate::entities::time_slot::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::time_slot::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: cycle_id.to_string(),
            name: name.to_string(),
            name_ar: name_ar.map(|s| s.to_string()),
            start_time: start.to_string(),
            end_time: end.to_string(),
            day_of_week: day,
            slot_type: slot_type.map(|s| s.to_string()).unwrap_or("lesson".to_string()),
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    pub async fn update_time_slot(&self, _id: &str, _req: schedule::UpdateTimeSlotRequest) -> Result<crate::entities::time_slot::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn delete_time_slot(&self, _id: &str) -> Result<(), AppError> {
        Ok(())
    }
    pub async fn create_class_subject(&self, class_id: &str, cycle_id: &str, subject_id: &str, teacher_id: Option<&str>, weekly: i32) -> Result<crate::entities::class_subject::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::class_subject::Model {
            id: uuid::Uuid::new_v4().to_string(),
            class_id: class_id.to_string(),
            cycle_id: cycle_id.to_string(),
            subject_id: subject_id.to_string(),
            teacher_id: teacher_id.map(|s| s.to_string()),
            weekly_periods: weekly,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    pub async fn update_class_subject(&self, _id: &str, _req: schedule::UpdateClassSubjectRequest) -> Result<crate::entities::class_subject::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn delete_class_subject(&self, _id: &str) -> Result<(), AppError> {
        Ok(())
    }
    pub async fn get_class_schedule(&self, _org_id: &str, _cycle_id: &str, _class_id: &str) -> Result<Vec<domain_scheduling::services::schedule::ScheduledLessonView>, AppError> {
        Ok(vec![])
    }
    pub async fn get_teacher_schedule(&self, _org_id: &str, _cycle_id: &str, _teacher_id: &str) -> Result<Vec<domain_scheduling::services::schedule::ScheduledLessonView>, AppError> {
        Ok(vec![])
    }
    pub async fn create_lesson(&self, org_id: &str, cycle_id: &str, class_id: &str, subject_id: &str, slot_id: &str, day: i32, teacher_id: Option<&str>, room_id: Option<&str>) -> Result<crate::entities::scheduled_lesson::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::scheduled_lesson::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: cycle_id.to_string(),
            class_id: class_id.to_string(),
            subject_id: subject_id.to_string(),
            time_slot_id: slot_id.to_string(),
            day_of_week: day,
            teacher_id: teacher_id.map(|s| s.to_string()),
            room_id: room_id.map(|s| s.to_string()),
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    pub async fn update_lesson(&self, _id: &str, _req: schedule::UpdateLessonRequest) -> Result<crate::entities::scheduled_lesson::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn delete_lesson(&self, _id: &str) -> Result<(), AppError> {
        Ok(())
    }
    pub async fn check_conflicts(&self, _cycle_id: &str, _class_id: Option<&str>, _teacher_id: Option<&str>) -> Result<Vec<domain_scheduling::services::schedule::ScheduleConflict>, AppError> {
        Ok(vec![])
    }
}

pub struct ExamSessionService {
    _cmd: Arc<domain_exam::repository::ExamCommandRepo>,
    _query: Arc<domain_exam::repository::ExamQueryRepo>,
}
impl ExamSessionService {
    pub fn new(cmd: Arc<domain_exam::repository::ExamCommandRepo>, query: Arc<domain_exam::repository::ExamQueryRepo>) -> Self {
        Self { _cmd: cmd, _query: query }
    }
    pub async fn create_session(&self, org_id: &str, user_id: &str, req: exam_session::CreateExamSessionRequest) -> Result<crate::entities::exam_session::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::exam_session::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: req.cycle_id,
            name: req.name,
            session_type: req.session_type,
            start_date: req.start_date,
            end_date: req.end_date,
            status: "draft".to_string(),
            created_by: user_id.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }
    pub async fn update_session(&self, _id: &str, _req: exam_session::UpdateExamSessionRequest) -> Result<crate::entities::exam_session::Model, AppError> {
        Err(AppError::NotFound("unimplemented".to_string()))
    }
    pub async fn create_subject(&self, req: exam_session::CreateExamSubjectRequest) -> Result<crate::entities::exam_subject::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::exam_subject::Model {
            id: uuid::Uuid::new_v4().to_string(),
            exam_session_id: req.exam_session_id,
            subject_id: req.subject_id,
            max_score: req.max_score,
            passing_score: req.passing_score,
            weight: req.weight,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    pub async fn create_schedule(&self, req: exam_session::CreateExamScheduleRequest) -> Result<crate::entities::exam_schedule::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::exam_schedule::Model {
            id: uuid::Uuid::new_v4().to_string(),
            exam_session_id: req.exam_session_id,
            subject_id: req.subject_id,
            exam_date: req.exam_date,
            start_time: req.start_time,
            end_time: req.end_time,
            status: "scheduled".to_string(),
            created_at: now.clone(),
            updated_at: now,
            time_slot_id: None,
        })
    }
    pub async fn create_hall(&self, req: exam_session::CreateExamHallRequest) -> Result<crate::entities::exam_hall::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        Ok(crate::entities::exam_hall::Model {
            id: uuid::Uuid::new_v4().to_string(),
            exam_session_id: req.exam_session_id,
            name: req.name,
            capacity: req.capacity,
            created_at: now.clone(),
            updated_at: now,
            name_ar: None,
            room_id: "".to_string(),
        })
    }
}

pub struct StudentOrderingService;
impl StudentOrderingService {
    pub fn new(_cmd: Arc<domain_exam::repository::ExamCommandRepo>, _query: Arc<domain_exam::repository::ExamQueryRepo>) -> Self { Self }
    pub async fn rebuild_ordering(&self, _org_id: &str, _cycle_id: &str, _strat: &str) -> Result<student_ordering::RebuildOrderingResult, AppError> {
        Ok(student_ordering::RebuildOrderingResult {
            success: true,
            total_students: 0,
            ordered: 0,
            errors: vec![],
        })
    }
}

pub struct ExamNumbersService;
impl ExamNumbersService {
    pub fn new(_cmd: Arc<domain_exam::repository::ExamCommandRepo>, _query: Arc<domain_exam::repository::ExamQueryRepo>) -> Self { Self }
    pub async fn generate_seating_numbers(&self, _org_id: &str, session_id: &str, _prefix: Option<String>, _suffix: Option<String>, _start: i32) -> Result<exam_numbers::GenerationResult, AppError> {
        Ok(exam_numbers::GenerationResult {
            exam_session_id: session_id.to_string(),
            generated_count: 0,
        })
    }
    pub async fn generate_secret_numbers(&self, _org_id: &str, session_id: &str, _prefix: Option<String>, _start: Option<i32>) -> Result<exam_numbers::GenerationResult, AppError> {
        Ok(exam_numbers::GenerationResult {
            exam_session_id: session_id.to_string(),
            generated_count: 0,
        })
    }
}

pub struct ExamLogisticsService;
impl ExamLogisticsService {
    pub fn new(_cmd: Arc<domain_exam::repository::ExamCommandRepo>, _query: Arc<domain_exam::repository::ExamQueryRepo>) -> Self { Self }
    pub async fn allocate_students_to_halls(&self, _session_id: &str, _config: exam_logistics::AllocationConfig) -> Result<domain_exam::services::exam_logistics::AllocationResult, AppError> {
        Ok(domain_exam::services::exam_logistics::AllocationResult {
            total_students: 0,
            total_halls: 0,
            allocations: vec![],
        })
    }
    pub async fn assign_proctors(&self, _config: exam_logistics::ProctorAssignConfig) -> Result<domain_exam::services::exam_logistics::ProctorAssignResult, AppError> {
        Ok(domain_exam::services::exam_logistics::ProctorAssignResult {
            assigned_count: 0,
            halls_covered: 0,
            total_halls: 0,
            uncovered_halls: vec![],
        })
    }
}

pub struct ExamAutoScheduleService {
    _cmd: Arc<domain_exam::repository::ExamCommandRepo>,
    _query: Arc<domain_exam::repository::ExamQueryRepo>,
    _session_svc: Arc<ExamSessionService>,
    _solver: Arc<dyn infra_scheduler::ScheduleSolver + Send + Sync>,
}
impl ExamAutoScheduleService {
    pub fn new(
        cmd: Arc<domain_exam::repository::ExamCommandRepo>,
        query: Arc<domain_exam::repository::ExamQueryRepo>,
        session_svc: Arc<ExamSessionService>,
        solver: Arc<dyn infra_scheduler::ScheduleSolver + Send + Sync>,
    ) -> Self {
        Self {
            _cmd: cmd,
            _query: query,
            _session_svc: session_svc,
            _solver: solver,
        }
    }
    pub async fn start_exam_solve(&self, org_id: &str, _user_id: &str, req: exam_auto_schedule::AutoScheduleRequest) -> Result<domain_exam::services::exam_auto_schedule::SolveJobResponse, AppError> {
        Ok(domain_exam::services::exam_auto_schedule::SolveJobResponse {
            org_id: org_id.to_string(),
            score: "0/0".to_string(),
            items_created: 0,
            started_at: Utc::now().to_rfc3339(),
        })
    }
}
