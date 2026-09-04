// ═══════════════════════════════════════════════════════════════
// docforge-engine — ExpressionFormatter Trait
// Allows registering custom string formatters for expression engine
// ═══════════════════════════════════════════════════════════════

use crate::error::DocforgeError;

pub trait ExpressionFormatter: Send + Sync {
    /// Unique name of the formatter (used in pipes like `{{var | name}}`)
    fn name(&self) -> &str;

    /// Formats the JSON value with the provided arguments
    fn format(
        &self,
        value: &serde_json::Value,
        args: &[String],
    ) -> Result<String, DocforgeError>;
}
