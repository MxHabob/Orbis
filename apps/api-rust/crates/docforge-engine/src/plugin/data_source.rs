// ═══════════════════════════════════════════════════════════════
// docforge-engine — DataSource Trait
// Consumers implement this to provide data for template rendering
// ═══════════════════════════════════════════════════════════════

use crate::error::DocforgeError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// The data context is a nested JSON-like object that provides
/// variable values for expression evaluation.
pub type DataContext = serde_json::Value;

/// Consumers implement this trait to provide data for templates.
/// Each data source handles one entity type (e.g., "enrollment").
#[async_trait]
pub trait DataSource: Send + Sync {
    /// Unique entity type identifier (e.g., "enrollment", "student")
    fn entity_type(&self) -> &str;

    /// JSON Schema describing available variables
    fn schema(&self) -> &DataSchema;

    /// Resolve a single record by ID
    async fn resolve_one(&self, id: &str) -> Result<DataContext, DocforgeError>;

    /// Resolve multiple records (for batch processing)
    async fn resolve_many(&self, filter: &DataFilter) -> Result<Vec<DataContext>, DocforgeError>;

    /// Return count of matching records (for progress estimation)
    async fn count(&self, filter: &DataFilter) -> Result<u64, DocforgeError>;

    /// Return sample data for editor preview
    fn sample_data(&self) -> DataContext;
}

/// Describes available data fields for a data source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSchema {
    pub entity_type: String,
    pub label: String,
    pub label_ar: Option<String>,
    pub fields: Vec<DataFieldSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFieldSchema {
    pub path: String,
    pub field_type: String,
    pub label: String,
    pub label_ar: Option<String>,
    pub children: Option<Vec<DataFieldSchema>>,
}

/// Filter specification for batch queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFilter {
    pub conditions: Vec<FilterCondition>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<SortDirection>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl DataFilter {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            sort_by: None,
            sort_direction: None,
            offset: None,
            limit: None,
        }
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Default for DataFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    Contains,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}
