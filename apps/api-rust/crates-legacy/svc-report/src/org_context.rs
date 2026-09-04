//! Organization Context — enriched org metadata for report headers.
//!
//! `OrgContext` is resolved once per report generation from the `organizations`
//! table + `OrgSettings` + template assets. It is injected into every report
//! dataset and used by:
//! - PDF report headers (logo, name, address)
//! - XLSX headers
//! - HTML headers
//! - DocForge template `$org` variable

use serde::{Deserialize, Serialize};

// ── Organization Context ────────────────────────────────────────

/// Full organization context for report generation.
///
/// This structure carries everything a report header needs to render
/// the organization's identity. It is resolved once per report
/// generation and shared across all output formats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgContext {
    /// Organization ID.
    pub id: String,

    /// Organization name (English).
    pub name: String,

    /// Organization name (Arabic).
    pub name_ar: String,

    /// Organization code (e.g., "SCH-001").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Logo URL (for HTML/browser rendering).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    /// Logo image bytes (pre-loaded for PDF/XLSX rendering).
    /// Skipped during JSON serialization to avoid bloat.
    #[serde(skip)]
    pub logo_data: Option<Vec<u8>>,

    // ── Contact Information ─────────────────────────────────────

    /// Physical address (English).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Physical address (Arabic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ar: Option<String>,

    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    // ── Legal Information ───────────────────────────────────────

    /// Tax identification number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// Registration number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,

    // ── Locale & Formatting ─────────────────────────────────────

    /// Default currency code (e.g., "SAR").
    pub currency: String,

    /// Currency symbol (e.g., "ر.س").
    pub currency_symbol: String,

    /// Locale code (e.g., "ar").
    pub locale: String,

    /// Timezone (e.g., "Asia/Riyadh").
    pub timezone: String,

    /// Whether Hijri calendar is enabled.
    #[serde(default)]
    pub hijri_enabled: bool,

    /// Primary brand color (CSS hex, e.g., "#2c5f2d").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,

    // ── Stamps & Signatures ─────────────────────────────────────

    /// Official stamp image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_url: Option<String>,

    /// Authorized signature image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_url: Option<String>,

    /// Stamp image bytes (pre-loaded for PDF).
    #[serde(skip)]
    pub stamp_data: Option<Vec<u8>>,

    /// Signature image bytes (pre-loaded for PDF).
    #[serde(skip)]
    pub signature_data: Option<Vec<u8>>,

    // ── Branch Information ───────────────────────────────────────

    /// List of branches/org units (for multi-branch reports).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub branches: Vec<BranchInfo>,

    // ── Academic Context ────────────────────────────────────────

    /// Current academic period name (e.g., "2025-2026").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_name: Option<String>,

    /// Current academic period name (Arabic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_name_ar: Option<String>,
}

// ── Branch Info ──────────────────────────────────────────────────

/// Summary information about an organizational branch/unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchInfo {
    /// Branch/unit ID.
    pub id: String,
    /// Name (English).
    pub name: String,
    /// Name (Arabic).
    pub name_ar: String,
    /// Address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Phone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

// ── Reporting Settings ──────────────────────────────────────────

/// Organization-level settings for report output behavior.
///
/// Stored in `organizations.settings` JSON as part of `OrgSettings`.
/// Read by `SettingsProvider` and used when resolving `OrgContext`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingSettings {
    /// Header style: "standard" (logo + name + address), "minimal" (name only),
    /// "custom" (user-defined template component).
    #[serde(default = "default_header_style")]
    pub header_style: String,

    /// Whether to show the organization logo in report headers.
    #[serde(default = "default_true")]
    pub show_logo: bool,

    /// Whether to show the address in report headers.
    #[serde(default)]
    pub show_address: bool,

    /// Whether to show contact info (phone/email) in report headers.
    #[serde(default)]
    pub show_contact: bool,

    /// Organization address (English) — overrides any address stored elsewhere.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Organization address (Arabic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_ar: Option<String>,

    /// Phone number for report headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Email for report headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Website for report headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// Tax identification number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// Registration / license number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,

    /// Asset ID of the official stamp image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_asset_id: Option<String>,

    /// Asset ID of the authorized signature image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_asset_id: Option<String>,

    /// Default report locale: "ar", "en", "ar-en", "en-ar".
    #[serde(default = "default_locale")]
    pub default_locale: String,

    /// Number format: "arabic-indic" (٠١٢٣) or "western" (0123).
    #[serde(default = "default_number_format")]
    pub number_format: String,

    /// Date display format.
    #[serde(default = "default_date_format")]
    pub date_format: String,
}

impl Default for ReportingSettings {
    fn default() -> Self {
        Self {
            header_style: default_header_style(),
            show_logo: true,
            show_address: false,
            show_contact: false,
            address: None,
            address_ar: None,
            phone: None,
            email: None,
            website: None,
            tax_id: None,
            registration_number: None,
            stamp_asset_id: None,
            signature_asset_id: None,
            default_locale: default_locale(),
            number_format: default_number_format(),
            date_format: default_date_format(),
        }
    }
}

// ── Serde Defaults ──────────────────────────────────────────────

fn default_true() -> bool {
    true
}

fn default_header_style() -> String {
    "standard".to_string()
}

fn default_locale() -> String {
    "ar".to_string()
}

fn default_number_format() -> String {
    "western".to_string()
}

fn default_date_format() -> String {
    "YYYY/MM/DD".to_string()
}
