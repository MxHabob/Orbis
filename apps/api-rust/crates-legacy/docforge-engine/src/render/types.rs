// ═══════════════════════════════════════════════════════════════
// docforge-engine — Render Types
// Intermediate representation between layout and PDF rendering
// ═══════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Complete layout result for an entire document, spanning one or more pages.
#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub pages: Vec<PageLayout>,
}

/// Layout for a single page — all elements already positioned in points (1pt = 1/72 inch).
#[derive(Debug, Clone)]
pub struct PageLayout {
    pub width_pt: f32,
    pub height_pt: f32,
    pub elements: Vec<PositionedElement>,
}

/// A fully positioned primitive ready for PDF rendering.
/// All coordinates are in PDF points (1pt = 1/72 inch), origin at top-left.
#[derive(Debug, Clone)]
pub enum PositionedElement {
    Text(TextElement),
    Image(ImageElement),
    Rect(RectElement),
    Line(LineElement),
    BarcodeSvg(SvgElement),
    QrCodeSvg(SvgElement),
}

/// Text element with pre-resolved content (expressions already evaluated).
#[derive(Debug, Clone)]
pub struct TextElement {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub content: String,
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub color: Color,
    pub text_align: TextAlign,
    pub direction: TextDirection,
    pub line_height: f32,
}

#[derive(Debug, Clone)]
pub struct ImageElement {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    /// Raw image bytes (PNG/JPEG), loaded on demand
    pub data: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug, Clone)]
pub struct RectElement {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub border_radius: f32,
}

#[derive(Debug, Clone)]
pub struct LineElement {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: Color,
    pub width: f32,
}

/// SVG content element (used for barcodes and QR codes)
#[derive(Debug, Clone)]
pub struct SvgElement {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub svg_content: String,
}

// ── Value Types ──────────────────────────────────────────────

/// RGBA color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Parse CSS hex color (e.g., "#ff0000", "#f00", "#ff000080")
    pub fn from_hex(hex: &str) -> Option<Color> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(Color::new(r, g, b))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color::new(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Color::with_alpha(r, g, b, a))
            }
            _ => None,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FontWeight {
    Light,
    Normal,
    Bold,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl FontWeight {
    pub fn from_str_opt(s: Option<&str>) -> Self {
        match s {
            Some("bold") => FontWeight::Bold,
            Some("light") => FontWeight::Light,
            _ => FontWeight::Normal,
        }
    }

    /// Map to CSS numeric weight for cosmic-text
    pub fn to_css_weight(&self) -> u16 {
        match self {
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Bold => 700,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    Normal,
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextAlign {
    Start,
    Center,
    End,
    Justify,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Start
    }
}

impl TextAlign {
    pub fn from_str_opt(s: Option<&str>) -> Self {
        match s {
            Some("center") => TextAlign::Center,
            Some("end") | Some("right") => TextAlign::End,
            Some("justify") => TextAlign::Justify,
            _ => TextAlign::Start,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextDirection {
    Ltr,
    Rtl,
    Auto,
}

impl Default for TextDirection {
    fn default() -> Self {
        TextDirection::Auto
    }
}

impl TextDirection {
    pub fn from_str_opt(s: Option<&str>) -> Self {
        match s {
            Some("rtl") => TextDirection::Rtl,
            Some("ltr") => TextDirection::Ltr,
            _ => TextDirection::Auto,
        }
    }
}

// ── Unit Conversion ──────────────────────────────────────────

/// 1 mm = 2.834645669 PDF points
pub const MM_TO_PT: f32 = 2.834_645_7;
/// 1 CSS px = 0.75 PDF points  
pub const PX_TO_PT: f32 = 0.75;

/// Convert millimeters to PDF points
pub fn mm_to_pt(mm: f64) -> f32 {
    (mm as f32) * MM_TO_PT
}

/// Convert PDF points to millimeters
pub fn pt_to_mm(pt: f32) -> f64 {
    (pt / MM_TO_PT) as f64
}
