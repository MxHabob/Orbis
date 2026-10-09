//! Report Domain — Rich domain model for Orbis Reporting & Template Engine.
//!
//! Five bounded concepts, cleanly separated:
//!   1. **Report Definition** — what to report (metadata, parameters, permissions)
//!   2. **Report Template**   — how it looks (document layout, lifecycle, versioning)
//!   3. **Report Dataset**    — format-agnostic data contract between providers and renderers
//!   4. **Report Job**        — async execution tracking with audit trail
//!   5. **Org Report Context**— organization branding for headers/footers
//!
//! ZERO infrastructure, rendering, or database dependencies.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
// 1. REPORT DEFINITION
// ═══════════════════════════════════════════════════════════════

/// Complete metadata for a single report type.
///
/// Report definitions describe *what* a report is — its identity,
/// parameter schema, required permissions, and supported outputs.
/// They do NOT contain rendering logic or data access code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDefinition {
    pub id: Uuid,
    pub org_id: Uuid,
    /// Unique report code: `{domain}.{snake_case_name}`.
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub domain: ReportDomain,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_ar: Option<String>,
    /// Declarative parameter schema for UI generation + backend validation.
    pub parameters: Vec<ParameterDef>,
    /// Permissions required to execute (user must have ALL).
    pub required_permissions: Vec<String>,
    /// Output formats this report supports.
    pub supported_formats: Vec<OutputFormat>,
    /// The data provider key used to resolve which provider fetches data.
    pub data_provider_key: String,
    pub is_system: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// High-level domain grouping for reports.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportDomain {
    Finance,
    Academic,
    Exam,
    Attendance,
    Staff,
    Student,
    Admission,
    Document,
    Organization,
    Hr,
    Administration,
    System,
}

impl std::fmt::Display for ReportDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Finance => write!(f, "finance"),
            Self::Academic => write!(f, "academic"),
            Self::Exam => write!(f, "exam"),
            Self::Attendance => write!(f, "attendance"),
            Self::Staff => write!(f, "staff"),
            Self::Student => write!(f, "student"),
            Self::Admission => write!(f, "admission"),
            Self::Document => write!(f, "document"),
            Self::Organization => write!(f, "organization"),
            Self::Hr => write!(f, "hr"),
            Self::Administration => write!(f, "administration"),
            Self::System => write!(f, "system"),
        }
    }
}

impl ReportDomain {
    pub fn from_str(s: &str) -> Self {
        match s {
            "finance" => Self::Finance,
            "academic" => Self::Academic,
            "exam" => Self::Exam,
            "attendance" => Self::Attendance,
            "staff" => Self::Staff,
            "student" => Self::Student,
            "admission" => Self::Admission,
            "document" => Self::Document,
            "organization" => Self::Organization,
            "hr" => Self::Hr,
            "administration" => Self::Administration,
            _ => Self::System,
        }
    }
}

// ── Parameter Schema ────────────────────────────────────────────

/// Declarative definition of a single report parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDef {
    pub key: String,
    pub label: String,
    pub label_ar: String,
    pub param_type: ParamType,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ParamOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_ar: Option<String>,
    #[serde(default)]
    pub sort_order: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
    Text,
    Integer,
    Decimal,
    Boolean,
    Date,
    DateRange,
    Id,
    IdList,
    Enum,
    Period,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamOption {
    pub value: String,
    pub label: String,
    pub label_ar: String,
}

// ── Parameter Values (submitted by user) ────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ParamValue {
    Text(String),
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
    Date(String),
    DateRange { from: String, to: String },
    Id(String),
    IdList(Vec<String>),
    Enum(String),
    Period(String),
}

impl ParamValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(s) | Self::Id(s) | Self::Enum(s) | Self::Date(s) | Self::Period(s) => {
                Some(s.as_str())
            }
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let Self::Integer(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Decimal(v) => Some(*v),
            Self::Integer(v) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Boolean(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_date_range(&self) -> Option<(&str, &str)> {
        if let Self::DateRange { from, to } = self {
            Some((from.as_str(), to.as_str()))
        } else {
            None
        }
    }

    pub fn as_id_list(&self) -> Option<&[String]> {
        if let Self::IdList(ids) = self {
            Some(ids.as_slice())
        } else {
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 2. REPORT TEMPLATE
// ═══════════════════════════════════════════════════════════════

/// A document template with lifecycle management and versioning.
///
/// Templates define *how* a report looks. They contain the document
/// structure (JSON) that renderers consume. Templates go through
/// a lifecycle: Draft → Published → Archived.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportTemplate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_ar: Option<String>,
    /// Template category (e.g., "report", "certificate", "id_card").
    pub document_type: String,
    /// The entity type this template is designed for.
    pub entity_type: String,
    /// Optional reference to a master/layout template for inheritance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_template_id: Option<Uuid>,
    pub status: TemplateStatus,
    pub version: i32,
    pub published_version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_by: Option<Uuid>,
    /// Page setup (size, orientation, margins) as JSON.
    pub page_size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_width_mm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_height_mm: Option<f64>,
    pub orientation: String,
    pub margins_json: String,
    /// The document body — a JSON structure consumed by renderers.
    pub document_json: String,
    pub locale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    pub is_system: bool,
    pub is_default: bool,
    pub created_by: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateStatus {
    Draft,
    Published,
    Archived,
}

impl std::fmt::Display for TemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Published => write!(f, "published"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

impl TemplateStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "published" => Self::Published,
            "archived" => Self::Archived,
            _ => Self::Draft,
        }
    }
}

/// Immutable snapshot of a template at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportTemplateVersion {
    pub id: Uuid,
    pub template_id: Uuid,
    pub version: i32,
    pub document_json: String,
    pub page_size: Option<String>,
    pub page_width_mm: Option<f64>,
    pub page_height_mm: Option<f64>,
    pub orientation: Option<String>,
    pub margins_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_summary: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Links a report definition to a template for a specific locale and output format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportTemplateBinding {
    pub id: Uuid,
    pub report_definition_id: Uuid,
    pub template_id: Uuid,
    pub locale: String,
    pub format: OutputFormat,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════
// 3. REPORT DATASET (universal data contract)
// ═══════════════════════════════════════════════════════════════

/// Format-agnostic intermediate representation.
///
/// This is the single contract between data providers and renderers.
/// Every output format (PDF, HTML, CSV, XLSX, JSON) consumes
/// the same dataset. Data providers never know about presentation;
/// renderers never know about domain logic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataset {
    pub report_code: String,
    pub report_name: String,
    pub report_name_ar: String,
    pub org: OrgReportContext,
    pub parameter_summary: Vec<ParameterSummaryItem>,
    pub columns: Vec<ColumnDef>,
    pub rows: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupDef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<Aggregations>,
    pub total_count: u64,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDef {
    pub key: String,
    pub header: String,
    pub header_ar: String,
    pub data_type: ColumnDataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<ColumnAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_type: Option<AggregationType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnDataType {
    Text,
    Number,
    Decimal,
    Currency,
    Percentage,
    Date,
    DateTime,
    Boolean,
    Status,
    Sequence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnAlign {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSummaryItem {
    pub label: String,
    pub label_ar: String,
    pub value: String,
    pub value_ar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupDef {
    pub field: String,
    pub label: String,
    pub label_ar: String,
    #[serde(default)]
    pub show_subtotals: bool,
    #[serde(default)]
    pub page_break: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregations {
    pub totals: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotals: Option<Vec<SubtotalRow>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtotalRow {
    pub group_value: String,
    pub values: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregationType {
    Sum,
    Count,
    Avg,
    Min,
    Max,
}

// ═══════════════════════════════════════════════════════════════
// 4. REPORT JOB (async execution)
// ═══════════════════════════════════════════════════════════════

/// Tracks the execution of a report generation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportJob {
    pub id: Uuid,
    pub org_id: Uuid,
    pub report_definition_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<Uuid>,
    pub requested_by: Uuid,
    pub report_code: String,
    pub parameters: serde_json::Value,
    pub output_format: OutputFormat,
    pub locale: ReportLocale,
    pub status: JobStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_size_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl JobStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "processing" => Self::Processing,
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            "cancelled" => Self::Cancelled,
            _ => Self::Pending,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 5. OUTPUT FORMAT & LOCALE
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    DataOnly,
    Pdf,
    Xlsx,
    Csv,
    Json,
    Html,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::DataOnly
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataOnly => write!(f, "data_only"),
            Self::Pdf => write!(f, "pdf"),
            Self::Xlsx => write!(f, "xlsx"),
            Self::Csv => write!(f, "csv"),
            Self::Json => write!(f, "json"),
            Self::Html => write!(f, "html"),
        }
    }
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "pdf" => Self::Pdf,
            "xlsx" => Self::Xlsx,
            "csv" => Self::Csv,
            "json" => Self::Json,
            "html" => Self::Html,
            _ => Self::DataOnly,
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::DataOnly | Self::Json => "application/json",
            Self::Pdf => "application/pdf",
            Self::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Self::Csv => "text/csv",
            Self::Html => "text/html",
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Self::DataOnly | Self::Json => "json",
            Self::Pdf => "pdf",
            Self::Xlsx => "xlsx",
            Self::Csv => "csv",
            Self::Html => "html",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportLocale {
    Ar,
    En,
    ArEn,
    EnAr,
}

impl Default for ReportLocale {
    fn default() -> Self {
        Self::Ar
    }
}

impl ReportLocale {
    pub fn is_rtl(&self) -> bool {
        matches!(self, Self::Ar | Self::ArEn)
    }

    pub fn primary_lang(&self) -> &str {
        match self {
            Self::Ar | Self::ArEn => "ar",
            Self::En | Self::EnAr => "en",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "en" => Self::En,
            "ar_en" | "ar-en" => Self::ArEn,
            "en_ar" | "en-ar" => Self::EnAr,
            _ => Self::Ar,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// 6. ORG REPORT CONTEXT (branding)
// ═══════════════════════════════════════════════════════════════

/// Organization branding context injected into report headers/footers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgReportContext {
    pub id: Uuid,
    pub name: String,
    pub name_ar: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Logo bytes for PDF/image rendering. Skipped in JSON.
    #[serde(skip)]
    pub logo_data: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
// 7. EXPORT RESULT
// ═══════════════════════════════════════════════════════════════

/// Result of exporting a report to a file format.
#[derive(Debug, Clone)]
pub struct ExportResult {
    pub format: OutputFormat,
    pub mime_type: String,
    pub filename: String,
    pub data: Vec<u8>,
    pub size: u64,
}

// ═══════════════════════════════════════════════════════════════
// 8. AUDIT
// ═══════════════════════════════════════════════════════════════

/// Audit log entry for report/template operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportAuditEntry {
    pub id: Uuid,
    pub org_id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

// ── Serde Helpers ───────────────────────────────────────────────

fn default_true() -> bool {
    true
}

// ═══════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_round_trip() {
        assert_eq!(OutputFormat::from_str("pdf"), OutputFormat::Pdf);
        assert_eq!(OutputFormat::from_str("csv"), OutputFormat::Csv);
        assert_eq!(OutputFormat::from_str("unknown"), OutputFormat::DataOnly);
        assert_eq!(OutputFormat::Pdf.mime_type(), "application/pdf");
        assert_eq!(OutputFormat::Csv.extension(), "csv");
    }

    #[test]
    fn test_report_locale() {
        assert!(ReportLocale::Ar.is_rtl());
        assert!(ReportLocale::ArEn.is_rtl());
        assert!(!ReportLocale::En.is_rtl());
        assert_eq!(ReportLocale::Ar.primary_lang(), "ar");
        assert_eq!(ReportLocale::EnAr.primary_lang(), "en");
    }

    #[test]
    fn test_template_status_transitions() {
        assert_eq!(TemplateStatus::from_str("draft"), TemplateStatus::Draft);
        assert_eq!(
            TemplateStatus::from_str("published"),
            TemplateStatus::Published
        );
        assert_eq!(
            TemplateStatus::from_str("archived"),
            TemplateStatus::Archived
        );
        assert_eq!(TemplateStatus::from_str("invalid"), TemplateStatus::Draft);
    }

    #[test]
    fn test_job_status() {
        assert_eq!(JobStatus::from_str("pending"), JobStatus::Pending);
        assert_eq!(JobStatus::from_str("processing"), JobStatus::Processing);
        assert_eq!(JobStatus::from_str("completed"), JobStatus::Completed);
        assert_eq!(JobStatus::from_str("failed"), JobStatus::Failed);
    }

    #[test]
    fn test_param_value_accessors() {
        let text = ParamValue::Text("hello".into());
        assert_eq!(text.as_str(), Some("hello"));
        assert_eq!(text.as_i64(), None);

        let int = ParamValue::Integer(42);
        assert_eq!(int.as_i64(), Some(42));
        assert_eq!(int.as_f64(), Some(42.0));

        let range = ParamValue::DateRange {
            from: "2026-01-01".into(),
            to: "2026-12-31".into(),
        };
        assert_eq!(range.as_date_range(), Some(("2026-01-01", "2026-12-31")));

        let ids = ParamValue::IdList(vec!["a".into(), "b".into()]);
        assert_eq!(ids.as_id_list().unwrap().len(), 2);
    }

    #[test]
    fn test_report_definition_serialize() {
        let def = ReportDefinition {
            id: Uuid::new_v4(),
            org_id: Uuid::new_v4(),
            code: "finance.fee_collection".into(),
            name: "Fee Collection Report".into(),
            name_ar: "تقرير تحصيل الرسوم".into(),
            domain: ReportDomain::Finance,
            category: "fees".into(),
            description: Some("Summary of collected fees".into()),
            description_ar: None,
            parameters: vec![ParameterDef {
                key: "period_id".into(),
                label: "Period".into(),
                label_ar: "الفترة".into(),
                param_type: ParamType::Period,
                required: true,
                default_value: None,
                options: None,
                entity_type: Some("academic_period".into()),
                placeholder: None,
                placeholder_ar: None,
                sort_order: 0,
                group: None,
            }],
            required_permissions: vec!["report.finance.view".into()],
            supported_formats: vec![OutputFormat::DataOnly, OutputFormat::Pdf, OutputFormat::Csv],
            data_provider_key: "finance.fee_collection".into(),
            is_system: true,
            is_active: true,
            sort_order: 1,
            version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&def).unwrap();
        assert!(json.contains("finance.fee_collection"));
        assert!(json.contains("تقرير تحصيل الرسوم"));
    }
}

// ═══════════════════════════════════════════════════════════════
// BATCH PRINTING / MULTI-UP LAYOUT
// ═══════════════════════════════════════════════════════════════

/// Configuration for batch printing — placing multiple items on one physical sheet.
/// Supports ID cards, seating cards, certificates, labels, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchPrintConfig {
    /// Physical sheet size
    pub sheet_size: SheetSize,
    /// Sheet orientation
    #[serde(default)]
    pub sheet_orientation: String,
    /// Sheet margins in mm
    #[serde(default)]
    pub margins: PrintMargins,
    /// Template/card width in mm
    pub item_width_mm: f64,
    /// Template/card height in mm
    pub item_height_mm: f64,
    /// Gap between items in mm
    #[serde(default = "default_gap")]
    pub gap_mm: f64,
    /// Fill order: "ltr" (left-to-right) or "rtl" (right-to-left)
    #[serde(default = "default_fill_order")]
    pub fill_order: FillOrder,
    /// Output mode
    #[serde(default)]
    pub output_mode: BatchOutputMode,
    /// Whether to show cut lines between items
    #[serde(default)]
    pub show_cut_lines: bool,
    /// Forced grid dimensions (overrides auto-compute)
    pub forced_columns: Option<u32>,
    pub forced_rows: Option<u32>,
}

/// Physical sheet size for printing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SheetSize {
    A4,
    A3,
    A5,
    Letter,
    Legal,
    Custom { width_mm: f64, height_mm: f64 },
}

impl SheetSize {
    pub fn dimensions_mm(&self) -> (f64, f64) {
        match self {
            SheetSize::A4 => (210.0, 297.0),
            SheetSize::A3 => (297.0, 420.0),
            SheetSize::A5 => (148.0, 210.0),
            SheetSize::Letter => (216.0, 279.0),
            SheetSize::Legal => (216.0, 356.0),
            SheetSize::Custom {
                width_mm,
                height_mm,
            } => (*width_mm, *height_mm),
        }
    }
}

impl Default for SheetSize {
    fn default() -> Self {
        SheetSize::A4
    }
}

/// Margins for printing (mm).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintMargins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Default for PrintMargins {
    fn default() -> Self {
        Self {
            top: 10.0,
            bottom: 10.0,
            left: 10.0,
            right: 10.0,
        }
    }
}

/// Fill order for multi-up layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FillOrder {
    /// Left→Right, Top→Bottom (LTR)
    Ltr,
    /// Right→Left, Top→Bottom (RTL, default for Arabic)
    Rtl,
}

impl Default for FillOrder {
    fn default() -> Self {
        FillOrder::Rtl
    }
}

/// Output mode for batch printing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BatchOutputMode {
    /// All items merged into a single multi-page document (default)
    Merged,
    /// Each item produces a separate file
    Individual,
    /// Individual files bundled into a ZIP archive
    Zip,
}

impl Default for BatchOutputMode {
    fn default() -> Self {
        BatchOutputMode::Merged
    }
}

fn default_gap() -> f64 {
    3.0
}
fn default_fill_order() -> FillOrder {
    FillOrder::Rtl
}

/// Computed multi-up layout slot position (in mm).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotPosition {
    pub x_mm: f64,
    pub y_mm: f64,
    pub width_mm: f64,
    pub height_mm: f64,
    pub column: u32,
    pub row: u32,
}

/// Computed multi-up layout result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiUpLayout {
    pub columns: u32,
    pub rows: u32,
    pub items_per_sheet: u32,
    pub slots: Vec<SlotPosition>,
    pub sheet_width_mm: f64,
    pub sheet_height_mm: f64,
}

impl BatchPrintConfig {
    /// Compute optimal multi-up layout for this configuration.
    pub fn compute_layout(&self) -> MultiUpLayout {
        let (mut sheet_w, mut sheet_h) = self.sheet_size.dimensions_mm();
        if self.sheet_orientation == "landscape" {
            std::mem::swap(&mut sheet_w, &mut sheet_h);
        }

        let printable_w = sheet_w - self.margins.left - self.margins.right;
        let printable_h = sheet_h - self.margins.top - self.margins.bottom;

        // Compute grid
        let columns = self
            .forced_columns
            .unwrap_or_else(|| {
                ((printable_w + self.gap_mm) / (self.item_width_mm + self.gap_mm)).floor() as u32
            })
            .max(1);

        let rows = self
            .forced_rows
            .unwrap_or_else(|| {
                ((printable_h + self.gap_mm) / (self.item_height_mm + self.gap_mm)).floor() as u32
            })
            .max(1);

        let items_per_sheet = columns * rows;

        // Center items on sheet
        let total_items_w =
            columns as f64 * self.item_width_mm + (columns as f64 - 1.0) * self.gap_mm;
        let total_items_h = rows as f64 * self.item_height_mm + (rows as f64 - 1.0) * self.gap_mm;
        let offset_x = self.margins.left + (printable_w - total_items_w) / 2.0;
        let offset_y = self.margins.top + (printable_h - total_items_h) / 2.0;

        // Generate slot positions
        let mut slots = Vec::with_capacity(items_per_sheet as usize);
        for row_idx in 0..rows {
            for col_idx in 0..columns {
                let actual_col = match self.fill_order {
                    FillOrder::Ltr => col_idx,
                    FillOrder::Rtl => columns - 1 - col_idx,
                };
                let x = offset_x + actual_col as f64 * (self.item_width_mm + self.gap_mm);
                let y = offset_y + row_idx as f64 * (self.item_height_mm + self.gap_mm);
                slots.push(SlotPosition {
                    x_mm: x,
                    y_mm: y,
                    width_mm: self.item_width_mm,
                    height_mm: self.item_height_mm,
                    column: actual_col,
                    row: row_idx,
                });
            }
        }

        MultiUpLayout {
            columns,
            rows,
            items_per_sheet,
            slots,
            sheet_width_mm: sheet_w,
            sheet_height_mm: sheet_h,
        }
    }

    /// Compute how many physical sheets are needed for a given number of records.
    pub fn total_sheets(&self, total_records: usize) -> u32 {
        let layout = self.compute_layout();
        if layout.items_per_sheet == 0 {
            return 0;
        }
        ((total_records as f64) / (layout.items_per_sheet as f64)).ceil() as u32
    }

    /// Paginate a list of items into sheets.
    pub fn paginate<T: Clone>(&self, items: &[T]) -> Vec<Vec<T>> {
        let layout = self.compute_layout();
        if layout.items_per_sheet == 0 {
            return vec![];
        }
        items
            .chunks(layout.items_per_sheet as usize)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}
