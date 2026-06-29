// ═══════════════════════════════════════════════════════════════
// docforge-engine — Node Types (Rust)
// Tagged union (serde) matching the TypeScript TemplateNode type
// ═══════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use super::document::Position;

/// Discriminated union of all node types.
/// Deserialized from JSON using the "type" field as the tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateNode {
    Page(PageNode),
    Frame(FrameNode),
    Grid(GridNode),
    Text(TextNode),
    Image(ImageNode),
    Shape(ShapeNode),
    Table(TableNode),
    Barcode(BarcodeNode),
    QRCode(QRCodeNode),
    Separator(SeparatorNode),
    Spacer(SpacerNode),
    Repeater(RepeaterNode),
    Conditional(ConditionalNode),
    ComponentRef(ComponentRefNode),
    PageBreak(PageBreakNode),
    Header(HeaderNode),
    Footer(FooterNode),
    Watermark(WatermarkNode),
}

impl TemplateNode {
    pub fn id(&self) -> &str {
        match self {
            Self::Page(n) => &n.id,
            Self::Frame(n) => &n.id,
            Self::Grid(n) => &n.id,
            Self::Text(n) => &n.id,
            Self::Image(n) => &n.id,
            Self::Shape(n) => &n.id,
            Self::Table(n) => &n.id,
            Self::Barcode(n) => &n.id,
            Self::QRCode(n) => &n.id,
            Self::Separator(n) => &n.id,
            Self::Spacer(n) => &n.id,
            Self::Repeater(n) => &n.id,
            Self::Conditional(n) => &n.id,
            Self::ComponentRef(n) => &n.id,
            Self::PageBreak(n) => &n.id,
            Self::Header(n) => &n.id,
            Self::Footer(n) => &n.id,
            Self::Watermark(n) => &n.id,
        }
    }

    pub fn children(&self) -> Option<&Vec<TemplateNode>> {
        match self {
            Self::Page(n) => Some(&n.children),
            Self::Frame(n) => Some(&n.children),
            Self::Grid(n) => Some(&n.children),
            Self::Repeater(n) => Some(&n.children),
            Self::Conditional(n) => Some(&n.children),
            Self::Header(n) => Some(&n.children),
            Self::Footer(n) => Some(&n.children),
            _ => None,
        }
    }
}

// ── Container Nodes ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: FrameProps,
    #[serde(default)]
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: GridProps,
    #[serde(default)]
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProps {
    pub columns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
}

// ── Content Nodes ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: TextProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextProps {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub font_size: f64,
    pub font_family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: ImageProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: ShapeProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeProps {
    pub shape_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: serde_json::Value, // Complex props handled as dynamic JSON
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: BarcodeProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarcodeProps {
    pub data: String,
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRCodeNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: QRCodeProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QRCodeProps {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub size: f64,
    pub error_correction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparatorNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: SeparatorProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparatorProps {
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacerNode {
    pub id: String,
    pub props: SpacerProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

// ── Logic Nodes ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeaterNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: RepeaterProps,
    #[serde(default)]
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepeaterProps {
    pub data_source: String,
    pub item_variable: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_variable: Option<String>,
    pub layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: ConditionalProps,
    #[serde(default)]
    pub children: Vec<TemplateNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub else_children: Option<Vec<TemplateNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalProps {
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRefNode {
    pub id: String,
    pub props: ComponentRefProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRefProps {
    pub component_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<serde_json::Value>,
}

// ── Control Nodes ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBreakNode {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: HeaderProps,
    #[serde(default)]
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderProps {
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FooterNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub props: FooterProps,
    #[serde(default)]
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FooterProps {
    pub height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_page_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatermarkNode {
    pub id: String,
    pub props: WatermarkProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatermarkProps {
    pub watermark_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
