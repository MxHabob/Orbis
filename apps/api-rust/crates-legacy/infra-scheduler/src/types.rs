use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SolveMode {
    #[default]
    Full,
    Incremental,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableProblem {
    pub job_id: String,
    pub org_id: String,
    pub period_id: String,
    pub mode: SolveMode,
    pub time_slots: Vec<TimeSlotDto>,
    pub rooms: Vec<RoomDto>,
    pub assignments: Vec<LessonAssignmentDto>,
    pub pinned_lessons: Vec<PinnedLessonDto>,
    #[serde(default)]
    pub unavailable_slots: Vec<TeacherUnavailableDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherUnavailableDto {
    pub teacher_id: String,
    pub day_of_week: i32,
    pub time_slot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlotDto {
    pub id: String,
    pub slot_number: i32,
    pub start_time: String,
    pub end_time: String,
    pub is_break: bool,
    pub applicable_days: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomDto {
    pub id: String,
    pub name: String,
    pub capacity: Option<i32>,
    pub room_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonAssignmentDto {
    pub id: String,
    pub class_subject_id: String,
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub hour_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_slot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedLessonDto {
    pub id: String,
    pub class_id: String,
    pub teacher_id: String,
    pub subject_id: String,
    pub class_subject_id: String,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableSolution {
    pub job_id: String,
    pub assignments: Vec<LessonAssignmentDto>,
    pub score: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveJobStatus {
    pub job_id: String,
    pub status: JobStatus,
    pub score: Option<String>,
    pub best_solution: Option<TimetableSolution>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub job_id: String,
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamTimeSlotDto {
    pub id: String,
    pub date: String,
    pub period: String, // "morning" or "afternoon"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamAssignmentDto {
    pub id: String,
    pub exam_subject_id: String,
    pub subject_id: String,
    pub class_id: String,
    pub conflict_group_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamProblem {
    pub job_id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub slots: Vec<ExamTimeSlotDto>,
    pub assignments: Vec<ExamAssignmentDto>,
    pub max_exams_per_day: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSolution {
    pub job_id: String,
    pub assignments: Vec<ExamAssignmentDto>,
    pub score: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSolveJobStatus {
    pub job_id: String,
    pub status: JobStatus,
    pub score: Option<String>,
    pub best_solution: Option<ExamSolution>,
    pub error: Option<String>,
}
