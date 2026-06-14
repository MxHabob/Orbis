use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ClassParams {
    pub q: Option<String>,
    pub is_active: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BehaviorParams {
    pub student_id: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub q: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NotificationParams {
    pub is_read: Option<i32>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
