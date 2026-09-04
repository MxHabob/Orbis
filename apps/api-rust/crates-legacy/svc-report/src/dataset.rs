//! Report Dataset — the universal, format-agnostic output contract.
//!
//! `ReportDataset` is the single intermediate representation between domain
//! data providers and output renderers/exporters. Every output target
//! (DataTable, PDF, XLSX, CSV, JSON, HTML) consumes the same dataset.
//!
//! This separation ensures:
//! - Data providers never know about presentation concerns
//! - Exporters never know about domain logic
//! - New output formats can be added without touching data providers
//! - New reports can be added without touching exporters

use serde::{Deserialize, Serialize};
use crate::org_context::OrgContext;

// ── Report Dataset ──────────────────────────────────────────────

/// Complete report output, carrying metadata + data + formatting hints.
///
/// This is what a data provider returns and what every exporter consumes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataset {
    // ── Report Identity ─────────────────────────────────────────

    /// Report code that produced this dataset.
    pub report_code: String,

    /// Report display name (English).
    pub report_name: String,

    /// Report display name (Arabic).
    pub report_name_ar: String,

    // ── Organization Context ────────────────────────────────────

    /// Organization context for headers/footers.
    pub org: OrgContext,

    // ── Resolved Parameters ─────────────────────────────────────

    /// Human-readable summary of the parameters used to generate this report.
    /// Displayed in the report header (e.g., "Period: 2025-09-01 — 2026-06-30").
    pub parameter_summary: Vec<ParameterSummaryItem>,

    // ── Column Definitions ──────────────────────────────────────

    /// Ordered list of columns in the result set.
    pub columns: Vec<ColumnDef>,

    // ── Data Rows ───────────────────────────────────────────────

    /// The actual data rows. Each row is a JSON object whose keys
    /// correspond to `ColumnDef.key`.
    pub rows: Vec<serde_json::Value>,

    // ── Grouping ────────────────────────────────────────────────

    /// Group definitions, if the report is grouped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupDef>>,

    // ── Aggregations ────────────────────────────────────────────

    /// Grand total row and/or summary aggregations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<Aggregations>,

    // ── Pagination Metadata ─────────────────────────────────────

    /// Total number of rows matching the query (before pagination).
    pub total_count: u64,

    /// Current page number (1-based). 0 means unpaginated (all rows).
    #[serde(default)]
    pub page: u32,

    /// Page size used.
    #[serde(default)]
    pub page_size: u32,

    // ── Generation Metadata ─────────────────────────────────────

    /// ISO-8601 timestamp when this dataset was generated.
    pub generated_at: String,

    /// User ID of the person who generated this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_by: Option<String>,
}

// ── Column Definition ───────────────────────────────────────────

/// Definition of a single column in the report dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDef {
    /// Column key — matches a field in each data row.
    pub key: String,

    /// Display header (English).
    pub header: String,

    /// Display header (Arabic).
    pub header_ar: String,

    /// Data type for formatting and alignment.
    pub data_type: ColumnDataType,

    /// Column width hint (percentage or fixed mm). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,

    /// Text alignment override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<ColumnAlign>,

    /// Formatter string (e.g., "number:2" for 2 decimal places,
    /// "currency", "date:short", "percentage").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Whether this column is visible by default.
    #[serde(default = "default_true")]
    pub visible: bool,

    /// Aggregation type for totals row (sum, count, avg, min, max).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_type: Option<AggregationType>,

    /// Whether this column's values may be sensitive (PII).
    /// Exporters can redact based on permissions.
    #[serde(default)]
    pub sensitive: bool,
}

/// Supported column data types for formatting.
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
    /// A status badge (rendered with color mapping).
    Status,
    /// Sequence/row number (auto-incremented).
    Sequence,
}

/// Column alignment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnAlign {
    Start,
    Center,
    End,
}

// ── Parameter Summary ───────────────────────────────────────────

/// A single resolved parameter displayed in the report header.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSummaryItem {
    /// English label.
    pub label: String,
    /// Arabic label.
    pub label_ar: String,
    /// Formatted display value (English).
    pub value: String,
    /// Formatted display value (Arabic).
    pub value_ar: String,
}

// ── Group Definition ────────────────────────────────────────────

/// Definition of a grouping level in the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupDef {
    /// Field key to group by.
    pub field: String,
    /// Group header label (English).
    pub label: String,
    /// Group header label (Arabic).
    pub label_ar: String,
    /// Whether to show subtotals for this group.
    #[serde(default)]
    pub show_subtotals: bool,
    /// Whether to insert a page break between groups.
    #[serde(default)]
    pub page_break: bool,
}

// ── Aggregations ────────────────────────────────────────────────

/// Aggregation results (totals row, summary stats).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregations {
    /// Grand totals — a JSON object with the same keys as data rows,
    /// containing aggregated values for columns that have `total_type` set.
    pub totals: serde_json::Value,

    /// Group-level subtotals, keyed by group value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotals: Option<Vec<SubtotalRow>>,
}

/// A subtotal row for a specific group value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtotalRow {
    /// The group field value this subtotal belongs to.
    pub group_value: String,
    /// Aggregated values.
    pub values: serde_json::Value,
}

/// Type of aggregation to compute for a column.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregationType {
    Sum,
    Count,
    Avg,
    Min,
    Max,
}

// ── Export Result ────────────────────────────────────────────────

/// Result of exporting a report to a file format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    /// The output format that was generated.
    pub format: String,
    /// MIME type of the output.
    pub mime_type: String,
    /// Suggested filename.
    pub filename: String,
    /// Raw bytes of the generated file (base64-encoded for JSON transport).
    #[serde(skip)]
    pub data: Vec<u8>,
    /// File size in bytes.
    pub size: u64,
}

// ── Serde Defaults ──────────────────────────────────────────────

fn default_true() -> bool {
    true
}
