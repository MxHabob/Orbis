//! Reporting ports — trait contracts for the reporting system.
//!
//! These are the Application-layer ports (hexagonal architecture).
//! Infrastructure adapters implement these traits.

use async_trait::async_trait;
use orbis_domain::entities::report::*;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════
// REPOSITORY PORTS
// ═══════════════════════════════════════════════════════════════

#[async_trait]
pub trait ReportDefinitionRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ReportDefinition>, DomainError>;
    async fn find_by_code(&self, org_id: Uuid, code: &str) -> Result<Option<ReportDefinition>, DomainError>;
    async fn list_by_org(&self, org_id: Uuid, domain: Option<&str>, category: Option<&str>) -> Result<Vec<ReportDefinition>, DomainError>;
    async fn save(&self, definition: &ReportDefinition) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ReportTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ReportTemplate>, DomainError>;
    async fn list_by_org(&self, org_id: Uuid, status: Option<&str>, document_type: Option<&str>, entity_type: Option<&str>) -> Result<Vec<ReportTemplate>, DomainError>;
    async fn save(&self, template: &ReportTemplate) -> Result<(), DomainError>;
    async fn update_status(&self, id: Uuid, status: &TemplateStatus, published_by: Option<Uuid>) -> Result<(), DomainError>;
    async fn soft_delete(&self, id: Uuid) -> Result<(), DomainError>;

    // Version management
    async fn save_version(&self, version: &ReportTemplateVersion) -> Result<(), DomainError>;
    async fn find_versions(&self, template_id: Uuid) -> Result<Vec<ReportTemplateVersion>, DomainError>;
    async fn find_version(&self, template_id: Uuid, version: i32) -> Result<Option<ReportTemplateVersion>, DomainError>;

    // Binding management
    async fn find_binding(&self, definition_id: Uuid, locale: &str, format: &OutputFormat) -> Result<Option<ReportTemplateBinding>, DomainError>;
    async fn save_binding(&self, binding: &ReportTemplateBinding) -> Result<(), DomainError>;
    async fn find_bindings_by_definition(&self, definition_id: Uuid) -> Result<Vec<ReportTemplateBinding>, DomainError>;
}

#[async_trait]
pub trait ReportJobRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ReportJob>, DomainError>;
    async fn save(&self, job: &ReportJob) -> Result<(), DomainError>;
    async fn update_status(&self, job_id: Uuid, status: &JobStatus, output_path: Option<String>, output_size: Option<i64>, error: Option<String>) -> Result<(), DomainError>;
    async fn list_by_org(&self, org_id: Uuid, limit: u32) -> Result<Vec<ReportJob>, DomainError>;
}

#[async_trait]
pub trait ReportAuditRepository: Send + Sync {
    async fn save(&self, entry: &ReportAuditEntry) -> Result<(), DomainError>;
    async fn list_by_org(&self, org_id: Uuid, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<ReportAuditEntry>, DomainError>;
}

// ═══════════════════════════════════════════════════════════════
// DATA PROVIDER PORT
// ═══════════════════════════════════════════════════════════════

/// Trait for fetching report data. Each domain area (finance, academic, etc.)
/// provides its own implementation. Resolved by `data_provider_key` in the
/// report definition, NOT by a central match statement.
#[async_trait]
pub trait ReportDataProvider: Send + Sync {
    /// The unique key this provider handles (e.g., "finance.fee_collection").
    fn provider_key(&self) -> &str;

    /// Fetch data and return a fully populated ReportDataset.
    async fn fetch_data(
        &self,
        org_id: Uuid,
        org_context: &OrgReportContext,
        parameters: &HashMap<String, ParamValue>,
        locale: &ReportLocale,
    ) -> Result<ReportDataset, DomainError>;
}

// ═══════════════════════════════════════════════════════════════
// DOCUMENT RENDERER PORT
// ═══════════════════════════════════════════════════════════════

/// Trait for rendering a ReportDataset into a specific output format.
/// Each output format has its own adapter. Resolved by OutputFormat,
/// NOT by a central match in the domain/application layer.
#[async_trait]
pub trait DocumentRenderer: Send + Sync {
    /// Which output format this renderer handles.
    fn supported_format(&self) -> OutputFormat;

    /// Render the dataset, optionally using a template document.
    async fn render(
        &self,
        dataset: &ReportDataset,
        template: Option<&ReportTemplate>,
        master_template: Option<&ReportTemplate>,
    ) -> Result<ExportResult, DomainError>;
}

// ═══════════════════════════════════════════════════════════════
// JOB EXECUTOR PORT
// ═══════════════════════════════════════════════════════════════

/// Port for async job execution. The initial adapter uses `tokio::spawn`,
/// but the contract allows swapping in a proper job queue.
#[async_trait]
pub trait ReportJobExecutor: Send + Sync {
    /// Submit a job for async execution. Returns immediately.
    async fn submit(&self, job_id: Uuid) -> Result<(), DomainError>;
}

// ═══════════════════════════════════════════════════════════════
// PROVIDER/RENDERER REGISTRIES
// ═══════════════════════════════════════════════════════════════

/// Registry for resolving data providers by key.
/// Extensible: new providers are registered without modifying existing code.
pub trait DataProviderRegistry: Send + Sync {
    fn get_provider(&self, key: &str) -> Option<&dyn ReportDataProvider>;
    fn register_provider(&mut self, provider: Box<dyn ReportDataProvider>);
    fn list_provider_keys(&self) -> Vec<&str>;
}

/// Registry for resolving document renderers by output format.
/// Extensible: new renderers are registered without modifying existing code.
pub trait RendererRegistry: Send + Sync {
    fn get_renderer(&self, format: &OutputFormat) -> Option<&dyn DocumentRenderer>;
    fn register_renderer(&mut self, renderer: Box<dyn DocumentRenderer>);
}
