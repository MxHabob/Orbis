use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
#[derive(utoipa::ToSchema)]
pub struct UniversalFilter {
    pub conditions: Vec<FilterCondition>,
    pub logic: FilterLogic,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<SortDirection>,
}

#[derive(Debug, Clone, Deserialize, Serialize, utoipa::ToSchema)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
#[derive(utoipa::ToSchema)]
pub enum FilterLogic {
    #[default]
    #[serde(alias = "AND")]
    And,
    #[serde(alias = "OR")]
    Or,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[derive(utoipa::ToSchema)]
pub enum SortDirection {
    #[serde(alias = "ASC")]
    Asc,
    #[serde(alias = "DESC")]
    Desc,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[derive(utoipa::ToSchema)]
pub enum FilterOperator {
    #[serde(alias = "Eq")]
    Eq,
    #[serde(alias = "Ne", alias = "Neq")]
    Neq,
    #[serde(alias = "Gt")]
    Gt,
    #[serde(alias = "Gte")]
    Gte,
    #[serde(alias = "Lt")]
    Lt,
    #[serde(alias = "Lte")]
    Lte,
    #[serde(alias = "Like")]
    Contains,
    #[serde(alias = "NotLike")]
    NotContains,
    StartsWith,
    EndsWith,
    #[serde(alias = "In")]
    In,
    #[serde(alias = "NotIn")]
    NotIn,
    #[serde(alias = "IsNull")]
    IsNull,
    #[serde(alias = "IsNotNull")]
    IsNotNull,
    Between,
    DateBefore,
    DateAfter,
    DateBetween,
    DateInLast,
    DateInNext,
    DateIsToday,
    DateIsThisWeek,
    DateIsThisMonth,
    BoolTrue,
    BoolFalse,
}
