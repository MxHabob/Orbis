// ═══════════════════════════════════════════════════════════════
// docforge-engine — Document Root & Page Types
// ═══════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root type representing a complete template document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDocument {
    pub version: String,
    pub meta: DocumentMeta,
    #[serde(default)]
    pub variables: HashMap<String, VariableDefinition>,
    #[serde(default)]
    pub assets: HashMap<String, AssetReference>,
    pub root: TemplateNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_ar: Option<String>,
    pub document_type: String,
    pub entity_type: String,
    #[serde(default = "default_locale")]
    pub locale: String,
    pub page_setup: PageSetup,
}

fn default_locale() -> String {
    "ar".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageSetup {
    pub size: PageSize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default = "default_orientation")]
    pub orientation: Orientation,
    pub margins: Spacing,
    #[serde(default = "default_unit")]
    pub unit: String,
}

fn default_orientation() -> Orientation {
    Orientation::Portrait
}

fn default_unit() -> String {
    "mm".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    A3,
    A5,
    Letter,
    Legal,
    #[serde(rename = "ID_CARD")]
    IdCard,
    #[serde(rename = "BADGE")]
    Badge,
    Custom,
}

impl PageSize {
    /// Get the dimensions in mm (width, height) for portrait orientation.
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (210.0, 297.0),
            PageSize::A3 => (297.0, 420.0),
            PageSize::A5 => (148.0, 210.0),
            PageSize::Letter => (216.0, 279.0),
            PageSize::Legal => (216.0, 356.0),
            PageSize::IdCard => (85.6, 53.98),
            PageSize::Badge => (86.0, 54.0),
            PageSize::Custom => (210.0, 297.0), // default to A4
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableDefinition {
    pub source: String,
    #[serde(rename = "type")]
    pub var_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_ar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReference {
    pub asset_id: String,
    #[serde(rename = "type")]
    pub asset_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub width: serde_json::Value, // number or string (percentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<serde_json::Value>,
}

/// Use TemplateNode from the nodes module
pub use super::nodes::TemplateNode;
