// ═══════════════════════════════════════════════════════════════
// docforge-engine — Error Types
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum DocforgeError {
    #[error("Document validation failed: {0}")]
    Validation(String),

    #[error("Expression error in node '{node_id}': {message}")]
    Expression { node_id: String, message: String },

    #[error("Asset not found: {0}")]
    AssetNotFound(String),

    #[error("Font not found: {family} {weight}")]
    FontNotFound { family: String, weight: String },

    #[error("Typst compilation error: {0}")]
    Compilation(String),

    #[error("Rendering error: {0}")]
    Render(String),

    #[error("Batch processing: {processed}/{total} processed, {failed} failed")]
    BatchPartial {
        processed: u64,
        total: u64,
        failed: u64,
    },

    #[error("Operation cancelled")]
    Cancelled,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Data source error: {0}")]
    DataSourceError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl DocforgeError {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn expression(node_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Expression {
            node_id: node_id.into(),
            message: message.into(),
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
