use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ClassParams {
    pub q: Option<String>,
    pub is_active: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct BehaviorParams {
    pub student_id: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct NotificationParams {
    pub is_read: Option<i32>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct UserParams {
    pub q: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct CycleParams {
    pub cycle_type: Option<String>,
    pub is_current: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct OrgUnitParams {
    pub unit_type: Option<String>,
    pub parent_id: Option<String>,
    pub is_active: Option<i32>,
    pub q: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct EnrollmentParams {
    pub class_id: Option<String>,
    pub cycle_id: Option<String>,
    pub student_id: Option<String>,
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct GradeParams {
    pub enrollment_id: Option<String>,
    pub class_id: Option<String>,
    pub cycle_id: Option<String>,
    pub subject_name: Option<String>,
    pub exam_type: Option<String>,
    pub semester: Option<String>,
    pub is_final: Option<i32>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct AttendanceParams {
    pub enrollment_id: Option<String>,
    pub class_id: Option<String>,
    pub cycle_id: Option<String>,
    pub date: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub status: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct AuditLogParams {
    pub action: Option<String>,
    pub entity_type: Option<String>,
    pub user_id: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ExportParams {
    pub class_id: Option<String>,
    pub cycle_id: Option<String>,
    pub lang: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ScheduleParams {
    pub cycle_id: Option<String>,
    pub class_id: Option<String>,
    pub teacher_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ClassSubjectParams {
    pub class_id: Option<String>,
    pub cycle_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct AssessmentParams {
    pub class_id: String,
    pub subject_id: String,
    pub cycle_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct GuardianParams {
    pub student_id: Option<String>,
    pub guardian_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct DocumentParams {
    pub owner_id: Option<String>,
    pub is_template: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ExamSessionParams {
    pub cycle_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct StudentOrderingParams {
    pub cycle_id: Option<String>,
    pub class_id: Option<String>,
    pub strategy: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct SeatingNumberParams {
    pub class_id: Option<String>,
}
