//! Report Request types — what a caller sends to generate a report.
//!
//! `ReportRequest` is the primary input to the report pipeline. It carries:
//! - The report code (which report to run)
//! - User-submitted parameter values
//! - Desired output format
//! - Locale preference
//! - Optional pagination

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Report Request ──────────────────────────────────────────────

/// A request to generate a specific report.
///
/// This is the top-level input to the report pipeline. The registry
/// uses `report_code` to look up the `ReportDefinition`, validates
/// `parameters` against the definition's schema, then dispatches
/// to the appropriate data provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportRequest {
    /// Report code (e.g., "finance.fee_collection").
    pub report_code: String,

    /// User-submitted parameter values, keyed by parameter key.
    #[serde(default)]
    pub parameters: HashMap<String, ParamValue>,

    /// Desired output format.
    #[serde(default)]
    pub output_format: OutputFormat,

    /// Locale preference for labels, dates, numbers.
    #[serde(default)]
    pub locale: ReportLocale,

    /// Pagination settings (for `DataOnly` format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<ReportPagination>,
}

// ── Parameter Values ────────────────────────────────────────────

/// A typed parameter value submitted by the user.
///
/// Each variant corresponds to a `ParamType` in the parameter schema.
/// The report validator ensures that submitted values match the expected type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ParamValue {
    /// Free-text string.
    Text(String),

    /// Integer number.
    Integer(i64),

    /// Decimal number.
    Decimal(f64),

    /// Boolean flag.
    Boolean(bool),

    /// Single date (ISO-8601 string, e.g., "2026-01-15").
    Date(String),

    /// Date range with inclusive start and end.
    DateRange {
        from: String,
        to: String,
    },

    /// Single entity ID.
    Id(String),

    /// List of entity IDs.
    IdList(Vec<String>),

    /// Enumerated choice value.
    Enum(String),

    /// Numeric threshold with comparison operator.
    Threshold {
        operator: ThresholdOp,
        value: f64,
    },

    /// Academic period ID (semantic alias for Id).
    Period(String),
}

impl ParamValue {
    /// Extract as a string, if the value is a Text, Id, Enum, Date, or Period.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(s) | Self::Id(s) | Self::Enum(s) | Self::Date(s) | Self::Period(s) => {
                Some(s.as_str())
            }
            _ => None,
        }
    }

    /// Extract as an i64, if the value is an Integer.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(v) => Some(*v),
            _ => None,
        }
    }

    /// Extract as an f64, if the value is a Decimal.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Decimal(v) => Some(*v),
            Self::Integer(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// Extract as a bool, if the value is a Boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(v) => Some(*v),
            _ => None,
        }
    }

    /// Extract the date range, if the value is a DateRange.
    pub fn as_date_range(&self) -> Option<(&str, &str)> {
        match self {
            Self::DateRange { from, to } => Some((from.as_str(), to.as_str())),
            _ => None,
        }
    }

    /// Extract as a list of IDs, if the value is an IdList.
    pub fn as_id_list(&self) -> Option<&[String]> {
        match self {
            Self::IdList(ids) => Some(ids.as_slice()),
            _ => None,
        }
    }
}

// ── Threshold Operator ──────────────────────────────────────────

/// Comparison operator for threshold parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdOp {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl std::fmt::Display for ThresholdOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "="),
            Self::Neq => write!(f, "!="),
            Self::Gt => write!(f, ">"),
            Self::Gte => write!(f, ">="),
            Self::Lt => write!(f, "<"),
            Self::Lte => write!(f, "<="),
        }
    }
}

// ── Output Format ───────────────────────────────────────────────

/// Requested output format for the report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    /// Return raw data only (for DataTable display in the frontend).
    DataOnly,
    /// Generate a PDF document.
    Pdf,
    /// Generate an Excel spreadsheet (.xlsx).
    Xlsx,
    /// Generate a CSV file.
    Csv,
    /// Generate a JSON export.
    Json,
    /// Generate a standalone HTML page.
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

// ── Report Locale ───────────────────────────────────────────────

/// Locale preference for the report output.
///
/// Controls column headers, date/number formatting, text direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportLocale {
    /// Arabic only.
    Ar,
    /// English only.
    En,
    /// Bilingual — Arabic primary, English secondary.
    ArEn,
    /// Bilingual — English primary, Arabic secondary.
    EnAr,
}

impl Default for ReportLocale {
    fn default() -> Self {
        Self::Ar
    }
}

impl ReportLocale {
    /// Whether the primary language is RTL.
    pub fn is_rtl(&self) -> bool {
        matches!(self, Self::Ar | Self::ArEn)
    }

    /// The primary language code.
    pub fn primary_lang(&self) -> &str {
        match self {
            Self::Ar | Self::ArEn => "ar",
            Self::En | Self::EnAr => "en",
        }
    }
}

// ── Pagination ──────────────────────────────────────────────────

/// Pagination settings for `DataOnly` format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportPagination {
    /// Page number (1-based).
    #[serde(default = "default_page")]
    pub page: u32,

    /// Number of rows per page.
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

impl Default for ReportPagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 500,
        }
    }
}

impl ReportPagination {
    /// Maximum allowed page size.
    pub const MAX_PAGE_SIZE: u32 = 5000;

    /// Calculate the SQL OFFSET.
    pub fn offset(&self) -> u64 {
        ((self.page.saturating_sub(1)) as u64) * (self.page_size as u64)
    }

    /// Effective limit, capped at MAX_PAGE_SIZE.
    pub fn limit(&self) -> u64 {
        self.page_size.min(Self::MAX_PAGE_SIZE) as u64
    }
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    500
}
