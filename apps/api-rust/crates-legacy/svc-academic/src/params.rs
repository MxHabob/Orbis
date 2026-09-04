use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ClassParams {
    pub q: Option<String>,
    pub is_active: Option<i32>,
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
    pub period_id: Option<String>,
    pub student_id: Option<String>,
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct DocumentParams {
    pub owner_id: Option<String>,
    pub is_template: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct GuardianParams {
    pub student_id: Option<String>,
    pub guardian_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ClassSubjectParams {
    pub class_id: Option<String>,
    pub period_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct NotificationParams {
    pub is_read: Option<i32>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
