// ═══════════════════════════════════════════════════════════════
// docforge-engine — Node Types (Rust)
// Tagged union (serde) matching the TypeScript TemplateNode type
// ═══════════════════════════════════════════════════════════════

use super::document::Position;
use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// Table props — accepts either the new typed `TableProps` or legacy
    /// opaque JSON. Old templates continue to work via `serde(untagged)`.
    pub props: TablePropsWrapper,
}

/// Backward-compatible wrapper: tries typed `TableProps` first,
/// falls back to raw `serde_json::Value` for legacy templates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TablePropsWrapper {
    Typed(TableProps),
    Legacy(serde_json::Value),
}

impl TablePropsWrapper {
    pub fn into_typed(self) -> TableProps {
        match self {
            TablePropsWrapper::Typed(props) => props,
            TablePropsWrapper::Legacy(val) => serde_json::from_value(val).unwrap_or_else(|_| TableProps {
                columns: Vec::new(),
                data_source: None,
                rows: None,
                show_header: true,
                show_totals: false,
                show_subtotals: false,
                group_by: None,
                sort_by: None,
                repeating_header: false,
                position: None,
                show_grid: true,
                show_border: true,
                direction: None,
                style: None,
                conditional_formatting: None,
            }),
        }
    }
}

/// Typed table properties for structured table rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableProps {
    /// Column definitions.
    pub columns: Vec<TableColumnDef>,
    /// Data source identifier (e.g. "term_certificate.subjects").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    /// Static row data (alternative to data_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<serde_json::Value>>,
    /// Whether to show the header row. Default true.
    #[serde(default = "default_true")]
    pub show_header: bool,
    /// Whether to show a totals row.
    #[serde(default)]
    pub show_totals: bool,
    /// Whether to show subtotal rows when grouping.
    #[serde(default)]
    pub show_subtotals: bool,
    /// Field name to group rows by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    /// Field name to sort rows by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Whether the header repeats on every page.
    #[serde(default)]
    pub repeating_header: bool,
    /// Position if configured inside props.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// Whether to draw grid divider lines. Default true.
    #[serde(default = "default_true")]
    pub show_grid: bool,
    /// Whether to draw outer border. Default true.
    #[serde(default = "default_true")]
    pub show_border: bool,
    /// Text direction ("rtl" or "ltr").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Table-level styling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<TableStyle>,
    /// Conditional formatting rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting: Option<Vec<ConditionalFormat>>,
}

/// Column definition for a typed table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableColumnDef {
    /// Unique column identifier.
    #[serde(default)]
    pub id: String,
    /// Display header text.
    #[serde(default)]
    pub header: String,
    /// Arabic header text (for bilingual templates).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_ar: Option<String>,
    /// Data field path to bind (e.g. "totalScore", "student.name").
    #[serde(default, alias = "key")]
    pub field: String,
    /// Column width (e.g. "120px", "20%", "auto").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    /// Text alignment: "left", "center", "right".
    #[serde(skip_serializing_if = "Option::is_none", alias = "align")]
    pub alignment: Option<String>,
    /// Default fallback value if data is null/missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Display format (e.g. "number:2" for 2 decimal places).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Whether the column is visible.
    #[serde(default = "default_true")]
    pub is_visible: bool,
    /// Totals row aggregation: "sum", "average", "count", "min", "max".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_type: Option<String>,
}

/// Table-level style properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_row_bg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// Conditional formatting rule for table cells.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalFormat {
    /// Field to evaluate.
    pub field: String,
    /// Operator: "eq", "neq", "gt", "gte", "lt", "lte", "contains".
    pub operator: String,
    /// Value to compare against.
    pub value: serde_json::Value,
    /// Style to apply when condition is met.
    pub style: serde_json::Value,
}

fn default_true() -> bool {
    true
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
    /// Group items by this field path (e.g., "category", "subject.department").
    /// When set, the repeater emits a group header before each group of items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    /// Variable name for the group key value (defaults to "group_key").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_variable: Option<String>,
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
