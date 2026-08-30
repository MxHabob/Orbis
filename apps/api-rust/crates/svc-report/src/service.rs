//! Report Service — the top-level orchestrator.
//!
//! `ReportService` is the main entry point for all report operations.
//! It ties together:
//! - The registry (report discovery and validation)
//! - Data providers (domain-specific query execution)
//! - Exporters (format conversion)
//! - Organization context resolution

use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::dataset::{ExportResult, ReportDataset};
use crate::definition::{ReportDefinition, ReportSummary};
use crate::error::ReportError;
use crate::org_context::OrgContext;
use crate::registry::ReportRegistry;
use crate::request::{OutputFormat, ReportRequest};

/// The main report service — call this to generate reports.
///
/// # Usage
///
/// ```rust,ignore
/// let service = ReportService::new(registry, db);
///
/// // List available reports
/// let reports = service.list_reports(None);
///
/// // Generate a report
/// let dataset = service.generate("finance.fee_collection", request, org).await?;
///
/// // Export to a file format
/// let file = service.export(request, org).await?;
/// ```
pub struct ReportService {
    registry: Arc<ReportRegistry>,
    db: Arc<DatabaseConnection>,
}

impl ReportService {
    /// Create a new report service.
    pub fn new(registry: Arc<ReportRegistry>, db: Arc<DatabaseConnection>) -> Self {
        Self { registry, db }
    }

    /// Register all built-in report definitions from providers.
    pub fn register_builtin_reports(&self) {
        use crate::providers;
        self.registry.register_all(providers::finance::definitions());
        self.registry.register_all(providers::academic::definitions());
        self.registry.register_all(providers::exam::definitions());
        self.registry.register_all(providers::attendance::definitions());
        tracing::info!(count = self.registry.count(), "Registered built-in report definitions");
    }

    /// Get the registry (for direct access to definitions).
    pub fn registry(&self) -> &ReportRegistry {
        &self.registry
    }

    /// List all available report summaries, optionally filtered by domain.
    pub fn list_reports(&self, domain: Option<&crate::definition::ReportDomain>) -> Vec<ReportSummary> {
        match domain {
            Some(d) => self.registry.list_by_domain(d),
            None => self.registry.list_all(),
        }
    }

    /// Get a full report definition by code.
    pub fn get_definition(&self, code: &str) -> Result<ReportDefinition, ReportError> {
        self.registry.get(code)
    }

    /// Generate a report dataset.
    ///
    /// This is the main entry point for data generation:
    /// 1. Validates the request against the report definition
    /// 2. Dispatches to the appropriate data provider
    /// 3. Returns the populated `ReportDataset`
    pub async fn generate(
        &self,
        request: &ReportRequest,
        org: &OrgContext,
    ) -> Result<ReportDataset, ReportError> {
        // Validate request (checks code exists, format supported, params valid)
        let _definition = self.registry.validate(request)?;

        // Dispatch to the appropriate data provider
        let dataset = self.dispatch_provider(&request.report_code, org, request).await?;

        Ok(dataset)
    }

    /// Export a report to a specific file format.
    ///
    /// This generates the dataset and then converts it to the requested format.
    /// For `DataOnly` format, returns the dataset as JSON.
    pub async fn export(
        &self,
        request: &ReportRequest,
        org: &OrgContext,
    ) -> Result<ExportResult, ReportError> {
        let dataset = self.generate(request, org).await?;

        match request.output_format {
            OutputFormat::DataOnly | OutputFormat::Json => {
                crate::exporters::json::export(&dataset)
            }
            OutputFormat::Csv => {
                crate::exporters::csv::export(&dataset, &request.locale)
            }
            OutputFormat::Xlsx => {
                crate::exporters::xlsx::export(&dataset, &request.locale)
            }
            OutputFormat::Pdf => {
                use crate::flow::flow_layout::generate_flow_layout;
                use docforge_engine::render::font_cache::FontCache;
                use docforge_engine::render::pdf_renderer::{render_to_pdf, PdfRenderConfig};

                let layout = generate_flow_layout(&dataset, &request.locale);
                let fonts = FontCache::new("Noto Sans Arabic");
                let config = PdfRenderConfig {
                    title: Some(dataset.report_name.clone()),
                    author: Some(dataset.org.name.clone()),
                    ..Default::default()
                };

                let pdf_output = render_to_pdf(&layout, &fonts, &config)
                    .map_err(|e| ReportError::Internal(format!("PDF rendering failed: {}", e)))?;

                let size = pdf_output.bytes.len() as u64;
                let filename = format!("{}.pdf", dataset.report_code.replace('.', "_"));

                Ok(ExportResult {
                    format: "pdf".into(),
                    mime_type: "application/pdf".into(),
                    filename,
                    data: pdf_output.bytes,
                    size,
                })
            }
            OutputFormat::Html => {
                // HTML exporter is future work
                Err(ReportError::UnsupportedFormat {
                    report_code: request.report_code.clone(),
                    format: "html".into(),
                })
            }
        }
    }

    /// Dispatch to the correct data provider based on report code.
    async fn dispatch_provider(
        &self,
        report_code: &str,
        org: &OrgContext,
        request: &ReportRequest,
    ) -> Result<ReportDataset, ReportError> {
        use crate::providers::{finance, academic, exam, attendance};

        match report_code {
            // Finance reports
            "finance.fee_collection" => {
                finance::fee_collection(&self.db, &org.id, org, request).await
            }
            "finance.payment_register" => {
                finance::payment_register(&self.db, &org.id, org, request).await
            }
            "finance.unpaid_fees" => {
                finance::unpaid_fees(&self.db, &org.id, org, request).await
            }

            // Academic reports
            "academic.student_list" => {
                academic::student_list(&self.db, &org.id, org, request).await
            }

            // Exam reports
            "exam.seating_plan" => {
                exam::seating_plan(&self.db, &org.id, org, request).await
            }

            // Attendance reports
            "attendance.summary" => {
                attendance::attendance_summary(&self.db, &org.id, org, request).await
            }

            // Unknown report code (should not happen if registry validation passed)
            _ => Err(ReportError::NotFound(report_code.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::ReportRegistry;

    #[test]
    fn test_register_builtin_definitions() {
        let reg = ReportRegistry::new();
        use crate::providers;

        reg.register_all(providers::finance::definitions());
        reg.register_all(providers::academic::definitions());
        reg.register_all(providers::exam::definitions());
        reg.register_all(providers::attendance::definitions());

        // 3 finance + 1 academic + 2 exam + 1 attendance = 7
        assert_eq!(reg.count(), 7);

        // Verify finance
        assert!(reg.contains("finance.fee_collection"));
        assert!(reg.contains("finance.unpaid_fees"));
        assert!(reg.contains("finance.payment_register"));
        // Verify academic
        assert!(reg.contains("academic.student_list"));
        // Verify exam
        assert!(reg.contains("exam.seating_plan"));
        assert!(reg.contains("exam.schedule"));
        // Verify attendance
        assert!(reg.contains("attendance.summary"));
    }

    #[test]
    fn test_list_by_domain() {
        let reg = ReportRegistry::new();
        use crate::providers;
        reg.register_all(providers::finance::definitions());
        reg.register_all(providers::academic::definitions());
        reg.register_all(providers::exam::definitions());
        reg.register_all(providers::attendance::definitions());

        let finance = reg.list_by_domain(&crate::definition::ReportDomain::Finance);
        assert_eq!(finance.len(), 3);

        // academic domain: student_list + attendance.summary
        let academic = reg.list_by_domain(&crate::definition::ReportDomain::Academic);
        assert_eq!(academic.len(), 2);

        let exam = reg.list_by_domain(&crate::definition::ReportDomain::Exam);
        assert_eq!(exam.len(), 2);
    }
}
