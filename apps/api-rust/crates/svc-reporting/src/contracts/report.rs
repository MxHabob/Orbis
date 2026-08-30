use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<ReportParameter>,
    pub columns: Vec<ReportColumn>,
    pub default_master_template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportParameter {
    pub id: String,
    pub label: String,
    pub param_type: ParameterType,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    String,
    Number,
    Date,
    DateRange,
    Boolean,
    Select { options: Vec<SelectOption> },
    EntityRef { entity_type: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportColumn {
    pub id: String,
    pub label: String,
    pub data_type: DataType,
    pub width_weight: u32,
    pub is_summarized: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DataType {
    String,
    Number,
    Currency,
    Date,
    Boolean,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataset {
    pub definition_id: String,
    pub metadata: ReportMetadata,
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportMetadata {
    pub generated_at: String,
    pub generated_by: Option<String>,
    pub applied_filters: HashMap<String, serde_json::Value>,
}
