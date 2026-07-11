use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default, utoipa::ToSchema)]
pub struct BehaviorParams {
    pub student_id: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, utoipa::ToSchema)]
pub struct GradeParams {
    pub enrollment_id: Option<String>,
    pub class_id: Option<String>,
    pub cycle_id: Option<String>,
    pub subject_name: Option<String>,
    pub exam_type: Option<String>,
    pub semester: Option<String>,
    pub is_final: Option<i32>,
    pub assessment_id: Option<String>,
    pub class_subject_id: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, utoipa::ToSchema)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Default, utoipa::ToSchema)]
pub struct GradeMatrixParams {
    pub cycle_id: String,
    pub class_id: String,
    pub subject_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, utoipa::ToSchema)]
pub struct AssessmentParams {
    pub class_id: String,
    pub subject_id: String,
    pub cycle_id: String,
}

