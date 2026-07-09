use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ScheduleParams {
    pub cycle_id: Option<String>,
    pub class_id: Option<String>,
    pub teacher_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct RoomParams {
    pub building: Option<String>,
    pub room_type: Option<String>,
    pub is_active: Option<i32>,
}
