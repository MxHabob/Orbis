//! Report Registry — in-memory catalog of all available reports.
//!
//! The registry is the central lookup point for report definitions.
//! It is populated at startup from:
//! 1. **Core reports** — registered programmatically from domain crates
//! 2. **DB reports** — loaded from the `report_definitions` table
//!    (CLI-installed or user-created)
//!
//! # Thread Safety
//!
//! The registry uses `Arc<RwLock<...>>` for concurrent read access
//! (report lookups) with rare write access (adding new reports).

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::definition::{OutputFormatKind, ReportDefinition, ReportDomain, ReportSummary};
use crate::error::ReportError;
use crate::request::ReportRequest;
use crate::validation::validate_parameters;

/// Central registry of all available report definitions.
///
/// # Usage
///
/// ```rust,ignore
/// let registry = ReportRegistry::new();
///
/// // Register core reports at startup
/// registry.register(fee_collection_definition());
/// registry.register(student_list_definition());
///
/// // Look up a report by code
/// let def = registry.get("finance.fee_collection")?;
///
/// // Validate a request
/// registry.validate(&request)?;
///
/// // List all reports for a domain
/// let finance_reports = registry.list_by_domain(&ReportDomain::Finance);
/// ```
pub struct ReportRegistry {
    definitions: Arc<RwLock<HashMap<String, ReportDefinition>>>,
}

impl ReportRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            definitions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a report definition. Overwrites if the code already exists.
    pub fn register(&self, definition: ReportDefinition) {
        let code = definition.code.clone();
        let mut defs = self.definitions.write().expect("registry write lock");
        tracing::info!(
            report_code = %code,
            domain = %definition.domain,
            "Registered report definition"
        );
        defs.insert(code, definition);
    }

    /// Register multiple definitions at once.
    pub fn register_all(&self, definitions: Vec<ReportDefinition>) {
        let mut defs = self.definitions.write().expect("registry write lock");
        for def in definitions {
            tracing::debug!(report_code = %def.code, "Registered report definition");
            defs.insert(def.code.clone(), def);
        }
    }

    /// Look up a report definition by code.
    pub fn get(&self, code: &str) -> Result<ReportDefinition, ReportError> {
        let defs = self.definitions.read().expect("registry read lock");
        defs.get(code)
            .cloned()
            .ok_or_else(|| ReportError::NotFound(code.to_string()))
    }

    /// Check if a report with the given code exists.
    pub fn contains(&self, code: &str) -> bool {
        let defs = self.definitions.read().expect("registry read lock");
        defs.contains_key(code)
    }

    /// List all registered report summaries (lightweight, no full schemas).
    pub fn list_all(&self) -> Vec<ReportSummary> {
        let defs = self.definitions.read().expect("registry read lock");
        let mut summaries: Vec<ReportSummary> = defs
            .values()
            .filter(|d| d.is_active)
            .map(ReportSummary::from)
            .collect();
        summaries.sort_by(|a, b| {
            a.domain
                .to_string()
                .cmp(&b.domain.to_string())
                .then(a.sort_order.cmp(&b.sort_order))
                .then(a.code.cmp(&b.code))
        });
        summaries
    }

    /// List report summaries for a specific domain.
    pub fn list_by_domain(&self, domain: &ReportDomain) -> Vec<ReportSummary> {
        let defs = self.definitions.read().expect("registry read lock");
        let mut summaries: Vec<ReportSummary> = defs
            .values()
            .filter(|d| d.is_active && d.domain == *domain)
            .map(ReportSummary::from)
            .collect();
        summaries.sort_by(|a, b| a.sort_order.cmp(&b.sort_order).then(a.code.cmp(&b.code)));
        summaries
    }

    /// List report summaries for a specific domain and category.
    pub fn list_by_category(&self, domain: &ReportDomain, category: &str) -> Vec<ReportSummary> {
        let defs = self.definitions.read().expect("registry read lock");
        let mut summaries: Vec<ReportSummary> = defs
            .values()
            .filter(|d| d.is_active && d.domain == *domain && d.category == category)
            .map(ReportSummary::from)
            .collect();
        summaries.sort_by(|a, b| a.sort_order.cmp(&b.sort_order).then(a.code.cmp(&b.code)));
        summaries
    }

    /// Validate a `ReportRequest` against its definition.
    ///
    /// This performs:
    /// 1. Report code lookup
    /// 2. Output format support check
    /// 3. Parameter validation against the schema
    pub fn validate(&self, request: &ReportRequest) -> Result<ReportDefinition, ReportError> {
        let definition = self.get(&request.report_code)?;

        // Check output format is supported
        let format_kind = output_format_to_kind(&request.output_format);
        if !definition.supported_formats.contains(&format_kind) {
            return Err(ReportError::UnsupportedFormat {
                report_code: request.report_code.clone(),
                format: request.output_format.to_string(),
            });
        }

        // Validate parameters
        validate_parameters(request, &definition)?;

        Ok(definition)
    }

    /// Total number of registered reports.
    pub fn count(&self) -> usize {
        let defs = self.definitions.read().expect("registry read lock");
        defs.len()
    }

    /// Remove a report definition by code. Returns true if it was present.
    pub fn remove(&self, code: &str) -> bool {
        let mut defs = self.definitions.write().expect("registry write lock");
        defs.remove(code).is_some()
    }
}

impl Default for ReportRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Map a request `OutputFormat` to a definition `OutputFormatKind`.
fn output_format_to_kind(format: &crate::request::OutputFormat) -> OutputFormatKind {
    match format {
        crate::request::OutputFormat::DataOnly => OutputFormatKind::DataOnly,
        crate::request::OutputFormat::Pdf => OutputFormatKind::Pdf,
        crate::request::OutputFormat::Xlsx => OutputFormatKind::Xlsx,
        crate::request::OutputFormat::Csv => OutputFormatKind::Csv,
        crate::request::OutputFormat::Json => OutputFormatKind::Json,
        crate::request::OutputFormat::Html => OutputFormatKind::Html,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::*;

    fn sample_def(code: &str, domain: ReportDomain) -> ReportDefinition {
        ReportDefinition {
            code: code.into(),
            name: format!("Report {}", code),
            name_ar: format!("تقرير {}", code),
            domain,
            category: "general".into(),
            description: None,
            description_ar: None,
            parameters: vec![],
            required_permissions: vec![],
            supported_formats: vec![OutputFormatKind::DataOnly, OutputFormatKind::Xlsx],
            default_template_id: None,
            is_system: true,
            is_active: true,
            sort_order: 0,
            version: 1,
        }
    }

    #[test]
    fn test_register_and_get() {
        let registry = ReportRegistry::new();
        registry.register(sample_def("finance.fees", ReportDomain::Finance));

        assert!(registry.contains("finance.fees"));
        assert!(!registry.contains("nonexistent"));

        let def = registry.get("finance.fees").unwrap();
        assert_eq!(def.code, "finance.fees");
    }

    #[test]
    fn test_list_by_domain() {
        let registry = ReportRegistry::new();
        registry.register(sample_def("finance.fees", ReportDomain::Finance));
        registry.register(sample_def("finance.ledger", ReportDomain::Finance));
        registry.register(sample_def("academic.students", ReportDomain::Academic));

        let finance = registry.list_by_domain(&ReportDomain::Finance);
        assert_eq!(finance.len(), 2);

        let academic = registry.list_by_domain(&ReportDomain::Academic);
        assert_eq!(academic.len(), 1);

        let all = registry.list_all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_validate_unsupported_format() {
        let registry = ReportRegistry::new();
        registry.register(sample_def("test.report", ReportDomain::Finance));

        let request = ReportRequest {
            report_code: "test.report".into(),
            parameters: std::collections::HashMap::new(),
            output_format: crate::request::OutputFormat::Pdf, // Not in supported_formats
            locale: crate::request::ReportLocale::Ar,
            pagination: None,
        };

        let result = registry.validate(&request);
        assert!(matches!(
            result.unwrap_err(),
            ReportError::UnsupportedFormat { .. }
        ));
    }

    #[test]
    fn test_validate_valid_request() {
        let registry = ReportRegistry::new();
        registry.register(sample_def("test.report", ReportDomain::Finance));

        let request = ReportRequest {
            report_code: "test.report".into(),
            parameters: std::collections::HashMap::new(),
            output_format: crate::request::OutputFormat::DataOnly,
            locale: crate::request::ReportLocale::Ar,
            pagination: None,
        };

        let result = registry.validate(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove() {
        let registry = ReportRegistry::new();
        registry.register(sample_def("test.report", ReportDomain::Finance));
        assert_eq!(registry.count(), 1);

        assert!(registry.remove("test.report"));
        assert_eq!(registry.count(), 0);
        assert!(!registry.remove("test.report"));
    }
}
