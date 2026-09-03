//! Report Definition types — the metadata layer that describes every available report.
//!
//! A `ReportDefinition` is the single source of truth for:
//! - What a report is called (bilingual)
//! - What domain it belongs to
//! - What parameters it accepts (declarative schema)
//! - What permissions are needed to execute it
//! - What output formats it supports
//! - Which DocForge template(s) to use for PDF output

use serde::{Deserialize, Serialize};

// ── Report Definition ───────────────────────────────────────────

/// Complete metadata for a single report type.
///
/// Report definitions are registered at startup (core reports in code)
/// and/or loaded from the `report_definitions` database table
/// (CLI-installed or custom reports).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportDefinition {
    /// Unique report code (e.g., "finance.fee_collection", "academic.student_list").
    /// Convention: `{domain}.{snake_case_name}`.
    pub code: String,

    /// Display name (English).
    pub name: String,

    /// Display name (Arabic) — primary label in Arabic-first environments.
    pub name_ar: String,

    /// Domain grouping: "finance", "academic", "exam", "staff", "system".
    pub domain: ReportDomain,

    /// Sub-category within the domain (e.g., "fees", "ledger", "grades").
    pub category: String,

    /// English description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Arabic description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_ar: Option<String>,

    /// Declarative parameter schema — drives UI form generation and backend validation.
    pub parameters: Vec<ParameterDef>,

    /// Permissions required to execute this report.
    /// The user must have ALL listed permissions.
    pub required_permissions: Vec<String>,

    /// Output formats this report supports.
    pub supported_formats: Vec<OutputFormatKind>,

    /// Default DocForge template ID for PDF output (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_template_id: Option<String>,

    /// Whether this is a system-provided (non-deletable) report.
    #[serde(default = "default_true")]
    pub is_system: bool,

    /// Whether this report is currently active (visible to users).
    #[serde(default = "default_true")]
    pub is_active: bool,

    /// Sort order for UI display.
    #[serde(default)]
    pub sort_order: i32,

    /// Schema version — incremented when the parameter schema changes.
    #[serde(default = "default_version")]
    pub version: i32,
}

// ── Report Domain ───────────────────────────────────────────────

/// High-level domain grouping for reports.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportDomain {
    Finance,
    Academic,
    Exam,
    Staff,
    System,
}

impl ReportDomain {
    /// Human-readable English label.
    pub fn label(&self) -> &str {
        match self {
            Self::Finance => "Finance",
            Self::Academic => "Academic",
            Self::Exam => "Examinations",
            Self::Staff => "Staff & HR",
            Self::System => "System",
        }
    }

    /// Arabic label.
    pub fn label_ar(&self) -> &str {
        match self {
            Self::Finance => "المالية",
            Self::Academic => "الأكاديمية",
            Self::Exam => "الاختبارات",
            Self::Staff => "الموظفين",
            Self::System => "النظام",
        }
    }
}

impl std::fmt::Display for ReportDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Finance => write!(f, "finance"),
            Self::Academic => write!(f, "academic"),
            Self::Exam => write!(f, "exam"),
            Self::Staff => write!(f, "staff"),
            Self::System => write!(f, "system"),
        }
    }
}

// ── Output Format ───────────────────────────────────────────────

/// Output formats a report can be rendered to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormatKind {
    /// Raw data only (for DataTable display).
    DataOnly,
    /// PDF document (canvas or flow mode).
    Pdf,
    /// Excel spreadsheet (.xlsx).
    Xlsx,
    /// Comma-separated values.
    Csv,
    /// JSON data export.
    Json,
    /// Standalone HTML page (print-optimized).
    Html,
}

impl std::fmt::Display for OutputFormatKind {
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

// ── Parameter Definition ────────────────────────────────────────

/// Declarative definition of a single report parameter.
///
/// This schema serves three purposes:
/// 1. **Backend validation**: The parameter validator checks submitted values
///    against this schema before passing to the data provider.
/// 2. **UI generation**: The frontend renders a form control for each parameter
///    based on the type and constraints.
/// 3. **Documentation**: Self-describing parameters for API consumers and AI agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDef {
    /// Parameter key — matches keys in `ReportRequest.parameters`.
    pub key: String,

    /// Display label (English).
    pub label: String,

    /// Display label (Arabic).
    pub label_ar: String,

    /// Parameter data type — determines validation and UI control.
    pub param_type: ParamType,

    /// Whether this parameter must be provided.
    #[serde(default)]
    pub required: bool,

    /// Default value (serialized as JSON).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,

    /// For `Enum` type: the allowed options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ParamOption>>,

    /// For `Id` type: the entity type to look up (e.g., "class", "org_unit").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,

    /// Placeholder text (English) for the UI control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// Placeholder text (Arabic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_ar: Option<String>,

    /// Help text / tooltip (English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,

    /// Help text / tooltip (Arabic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text_ar: Option<String>,

    /// Display order in the form.
    #[serde(default)]
    pub sort_order: i32,

    /// UI grouping label (parameters with the same group are visually grouped).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// Supported parameter data types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
    /// Free-text string.
    Text,
    /// Integer number.
    Integer,
    /// Decimal number.
    Decimal,
    /// True/false toggle.
    Boolean,
    /// Single date (ISO-8601).
    Date,
    /// Date range (from + to).
    DateRange,
    /// Single entity ID (rendered as a lookup/select).
    Id,
    /// Multiple entity IDs (rendered as multi-select).
    IdList,
    /// Enumerated choice (rendered as dropdown).
    Enum,
    /// Numeric threshold with operator (>, >=, <, <=, =, !=).
    Threshold,
    /// Academic period picker (special entity lookup).
    Period,
}

/// A selectable option for `ParamType::Enum`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamOption {
    /// Internal value.
    pub value: String,
    /// Display label (English).
    pub label: String,
    /// Display label (Arabic).
    pub label_ar: String,
}

// ── Report Template Binding ─────────────────────────────────────

/// Links a report definition to a DocForge template for a specific
/// locale and output format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportTemplateBinding {
    /// Binding ID.
    pub id: String,
    /// Report code (FK to ReportDefinition).
    pub report_code: String,
    /// DocForge template ID (FK to templates table).
    pub template_id: String,
    /// Locale this binding applies to ("ar", "en", "ar-en").
    pub locale: String,
    /// Output format this binding applies to.
    pub format: OutputFormatKind,
    /// Whether this is the default binding for the locale+format combination.
    pub is_default: bool,
}

// ── Report Metadata (lightweight summary) ───────────────────────

/// Lightweight summary of a report for listing/discovery purposes.
/// Excludes the full parameter schema to reduce payload size.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportSummary {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub domain: ReportDomain,
    pub category: String,
    pub description: Option<String>,
    pub description_ar: Option<String>,
    pub supported_formats: Vec<OutputFormatKind>,
    pub is_system: bool,
    pub sort_order: i32,
}

impl From<&ReportDefinition> for ReportSummary {
    fn from(def: &ReportDefinition) -> Self {
        Self {
            code: def.code.clone(),
            name: def.name.clone(),
            name_ar: def.name_ar.clone(),
            domain: def.domain.clone(),
            category: def.category.clone(),
            description: def.description.clone(),
            description_ar: def.description_ar.clone(),
            supported_formats: def.supported_formats.clone(),
            is_system: def.is_system,
            sort_order: def.sort_order,
        }
    }
}

// ── Serde Defaults ──────────────────────────────────────────────

fn default_true() -> bool {
    true
}

fn default_version() -> i32 {
    1
}
