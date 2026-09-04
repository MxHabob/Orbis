use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ExamSessionParams {
    pub period_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ExamScheduleParams {
    pub exam_session_id: Option<String>,
    pub exam_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ExamHallParams {
    pub exam_session_id: Option<String>,
    pub room_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct SeatingNumberParams {
    pub exam_session_id: Option<String>,
    pub class_id: Option<String>,
    pub is_active: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct SecretNumberParams {
    pub exam_session_id: Option<String>,
    pub is_active: Option<i32>,
    pub is_revealed: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ExamProctorParams {
    pub exam_session_id: Option<String>,
    pub exam_schedule_id: Option<String>,
    pub teacher_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct StudentOrderingParams {
    pub period_id: Option<String>,
    pub class_id: Option<String>,
    pub strategy: Option<String>,
}


#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct TeacherExamAvailabilityParams {
    pub exam_session_id: Option<String>,
    pub teacher_id: Option<String>,
    pub date: Option<String>,
}
