use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct PeriodParams {
    pub period_type: Option<String>,
    pub is_current: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct ExportParams {
    pub class_id: Option<String>,
    pub period_id: Option<String>,
    pub lang: Option<String>,
}
