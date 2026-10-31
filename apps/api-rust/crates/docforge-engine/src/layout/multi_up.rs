// ═══════════════════════════════════════════════════════════════
// docforge-engine — Multi-Up Layout Engine
//
// Computes optimal placement of multiple template instances
// on a single physical sheet (e.g., 8 seating cards on A4).
//
// Pipeline:
// 1. MultiUpConfig describes sheet + template dimensions
// 2. compute_layout() → MultiUpLayout with slot positions
// 3. LayoutEngine::layout_multi_up() uses this to arrange rendered elements
//
// ZERO Orbis dependencies. Pure geometry + unit conversion.
// ═══════════════════════════════════════════════════════════════

use crate::model::document::Orientation;
use crate::render::types::{mm_to_pt, Color, LineElement, PositionedElement};
use serde::{Deserialize, Serialize};

// ── Sheet Size ───────────────────────────────────────────────

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
    /// Get dimensions in mm (width, height) for portrait orientation.
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

// ── Fill Order ───────────────────────────────────────────────

/// Order in which template slots are filled on the sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FillOrder {
    /// Left→Right, Top→Bottom (LTR languages)
    LeftToRightTopToBottom,
    /// Right→Left, Top→Bottom (RTL languages like Arabic)
    RightToLeftTopToBottom,
}

impl Default for FillOrder {
    fn default() -> Self {
        FillOrder::RightToLeftTopToBottom // Default for Arabic-first system
    }
}

// ── Multi-Up Configuration ──────────────────────────────────

/// Complete configuration for multi-up layout computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiUpConfig {
    /// Physical sheet size
    pub sheet_size: SheetSize,
    /// Physical sheet orientation
    #[serde(default = "default_portrait")]
    pub sheet_orientation: Orientation,
    /// Sheet margins in mm
    pub sheet_margins: SheetMargins,
    /// Template width in mm (from TemplateDocument.meta.page_setup)
    pub template_width_mm: f64,
    /// Template height in mm
    pub template_height_mm: f64,
    /// Horizontal gap between templates in mm
    #[serde(default = "default_gap")]
    pub gap_horizontal_mm: f64,
    /// Vertical gap between templates in mm
    #[serde(default = "default_gap")]
    pub gap_vertical_mm: f64,
    /// Fill order (LTR or RTL)
    #[serde(default)]
    pub fill_order: FillOrder,
    /// Cut line width in mm (0 = disabled). Default: 0.5mm
    #[serde(default = "default_cut_line_width")]
    pub cut_line_width_mm: f64,
    /// Cut line color (hex). Default: light gray
    #[serde(default = "default_cut_line_color")]
    pub cut_line_color: String,
    /// Manually forced grid dimensions (overrides auto-compute).
    /// None = compute automatically from dimensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced_grid: Option<ForcedGrid>,
    /// Whether to center templates within the available area
    #[serde(default = "default_true")]
    pub center_on_sheet: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetMargins {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

impl Default for SheetMargins {
    fn default() -> Self {
        SheetMargins {
            top: 10.0,
            bottom: 10.0,
            left: 10.0,
            right: 10.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForcedGrid {
    pub columns: u32,
    pub rows: u32,
}

fn default_portrait() -> Orientation {
    Orientation::Portrait
}
fn default_gap() -> f64 {
    3.0
}
fn default_cut_line_width() -> f64 {
    0.5
}
fn default_cut_line_color() -> String {
    "#cccccc".to_string()
}
fn default_true() -> bool {
    true
}

// ── Computed Layout ─────────────────────────────────────────

/// Result of multi-up layout computation.
/// Contains slot positions and sheet dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiUpLayout {
    /// Number of columns
    pub columns: u32,
    /// Number of rows
    pub rows: u32,
    /// Total items per physical sheet
    pub items_per_sheet: u32,
    /// Positioned slots (in PDF points)
    pub slots: Vec<SlotPosition>,
    /// Physical sheet width in points
    pub sheet_width_pt: f32,
    /// Physical sheet height in points
    pub sheet_height_pt: f32,
    /// Physical sheet width in mm (for TS mirror)
    pub sheet_width_mm: f64,
    /// Physical sheet height in mm
    pub sheet_height_mm: f64,
    /// Template width in points
    pub template_width_pt: f32,
    /// Template height in points
    pub template_height_pt: f32,
}

/// Position of a single template slot on the physical sheet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotPosition {
    /// X position in PDF points
    pub x_pt: f32,
    /// Y position in PDF points
    pub y_pt: f32,
    /// Slot width in PDF points
    pub width_pt: f32,
    /// Slot height in PDF points
    pub height_pt: f32,
    /// Column index (0-based)
    pub col: u32,
    /// Row index (0-based)
    pub row: u32,
    /// Sequential index (0-based, follows fill_order)
    pub index: u32,
}

// ── Core Algorithm ──────────────────────────────────────────

impl MultiUpConfig {
    /// Compute the optimal multi-up layout.
    ///
    /// Algorithm:
    /// 1. Get sheet dimensions (applying orientation)
    /// 2. Compute printable area (sheet - margins)
    /// 3. Compute grid size: cols × rows (or use forced_grid)
    /// 4. Compute slot positions with centering
    /// 5. Apply fill order (LTR or RTL)
    pub fn compute_layout(&self) -> MultiUpLayout {
        // Step 1: Resolve sheet dimensions in mm
        let (raw_w, raw_h) = self.sheet_size.dimensions_mm();
        let (sheet_w_mm, sheet_h_mm) = match self.sheet_orientation {
            Orientation::Portrait => (raw_w, raw_h),
            Orientation::Landscape => (raw_h, raw_w),
        };

        // Step 2: Printable area in mm
        let printable_w = sheet_w_mm - self.sheet_margins.left - self.sheet_margins.right;
        let printable_h = sheet_h_mm - self.sheet_margins.top - self.sheet_margins.bottom;

        // Step 3: Compute grid dimensions
        let (columns, rows) = if let Some(ref forced) = self.forced_grid {
            (forced.columns, forced.rows)
        } else {
            let cols =
                compute_max_items(printable_w, self.template_width_mm, self.gap_horizontal_mm);
            let rows =
                compute_max_items(printable_h, self.template_height_mm, self.gap_vertical_mm);
            (cols.max(1), rows.max(1))
        };

        let items_per_sheet = columns * rows;

        // Step 4: Compute slot positions
        let template_w_pt = mm_to_pt(self.template_width_mm);
        let template_h_pt = mm_to_pt(self.template_height_mm);
        let gap_h_pt = mm_to_pt(self.gap_horizontal_mm);
        let gap_v_pt = mm_to_pt(self.gap_vertical_mm);
        let sheet_w_pt = mm_to_pt(sheet_w_mm);
        let sheet_h_pt = mm_to_pt(sheet_h_mm);
        let margin_left_pt = mm_to_pt(self.sheet_margins.left);
        let margin_top_pt = mm_to_pt(self.sheet_margins.top);

        // Total content block dimensions
        let content_block_w =
            template_w_pt * columns as f32 + gap_h_pt * (columns.saturating_sub(1)) as f32;
        let content_block_h =
            template_h_pt * rows as f32 + gap_v_pt * (rows.saturating_sub(1)) as f32;

        let printable_w_pt = mm_to_pt(printable_w);
        let printable_h_pt = mm_to_pt(printable_h);

        // Centering offsets
        let (offset_x, offset_y) = if self.center_on_sheet {
            let ox = margin_left_pt + (printable_w_pt - content_block_w) / 2.0;
            let oy = margin_top_pt + (printable_h_pt - content_block_h) / 2.0;
            (ox, oy)
        } else {
            (margin_left_pt, margin_top_pt)
        };

        // Build slot positions
        let mut slots = Vec::with_capacity(items_per_sheet as usize);

        for row_idx in 0..rows {
            for col_idx in 0..columns {
                let physical_col = match self.fill_order {
                    FillOrder::LeftToRightTopToBottom => col_idx,
                    FillOrder::RightToLeftTopToBottom => columns - 1 - col_idx,
                };

                let x = offset_x + physical_col as f32 * (template_w_pt + gap_h_pt);
                let y = offset_y + row_idx as f32 * (template_h_pt + gap_v_pt);

                let sequential_index = row_idx * columns + col_idx;

                slots.push(SlotPosition {
                    x_pt: x,
                    y_pt: y,
                    width_pt: template_w_pt,
                    height_pt: template_h_pt,
                    col: physical_col,
                    row: row_idx,
                    index: sequential_index,
                });
            }
        }

        MultiUpLayout {
            columns,
            rows,
            items_per_sheet,
            slots,
            sheet_width_pt: sheet_w_pt,
            sheet_height_pt: sheet_h_pt,
            sheet_width_mm: sheet_w_mm,
            sheet_height_mm: sheet_h_mm,
            template_width_pt: template_w_pt,
            template_height_pt: template_h_pt,
        }
    }
}

/// Compute maximum number of items that fit in `available` space,
/// given item `size` and `gap` between items.
///
/// Formula: n = floor((available + gap) / (size + gap))
fn compute_max_items(available: f64, size: f64, gap: f64) -> u32 {
    if size <= 0.0 || available <= 0.0 {
        return 0;
    }
    let n = ((available + gap) / (size + gap)).floor();
    n.max(0.0) as u32
}

// ── Element Offsetting ──────────────────────────────────────

/// Offset a positioned element by (dx, dy) in points.
/// Used to place a rendered template's elements into a specific slot.
pub fn offset_element(element: &PositionedElement, dx: f32, dy: f32) -> PositionedElement {
    match element {
        PositionedElement::Text(t) => {
            let mut t2 = t.clone();
            t2.x += dx;
            t2.y += dy;
            PositionedElement::Text(t2)
        }
        PositionedElement::Image(img) => {
            let mut img2 = img.clone();
            img2.x += dx;
            img2.y += dy;
            PositionedElement::Image(img2)
        }
        PositionedElement::Rect(r) => {
            let mut r2 = r.clone();
            r2.x += dx;
            r2.y += dy;
            PositionedElement::Rect(r2)
        }
        PositionedElement::Line(l) => {
            let mut l2 = l.clone();
            l2.x1 += dx;
            l2.y1 += dy;
            l2.x2 += dx;
            l2.y2 += dy;
            PositionedElement::Line(l2)
        }
        PositionedElement::BarcodeSvg(svg) => {
            let mut svg2 = svg.clone();
            svg2.x += dx;
            svg2.y += dy;
            PositionedElement::BarcodeSvg(svg2)
        }
        PositionedElement::QrCodeSvg(svg) => {
            let mut svg2 = svg.clone();
            svg2.x += dx;
            svg2.y += dy;
            PositionedElement::QrCodeSvg(svg2)
        }
    }
}

// ── Cut Line Generation ─────────────────────────────────────

/// Generate cut guide lines between template slots on a sheet.
///
/// Produces horizontal and vertical dashed lines at the midpoints
/// between adjacent slots, extending across the full printable area.
pub fn generate_cut_lines(
    layout: &MultiUpLayout,
    config: &MultiUpConfig,
) -> Vec<PositionedElement> {
    if config.cut_line_width_mm <= 0.0 {
        return Vec::new();
    }

    let line_width = mm_to_pt(config.cut_line_width_mm);
    let color = Color::from_hex(&config.cut_line_color).unwrap_or(Color::new(204, 204, 204));
    let _gap_h_pt = mm_to_pt(config.gap_horizontal_mm);
    let _gap_v_pt = mm_to_pt(config.gap_vertical_mm);

    let margin_left_pt = mm_to_pt(config.sheet_margins.left);
    let margin_top_pt = mm_to_pt(config.sheet_margins.top);
    let margin_right_pt = mm_to_pt(config.sheet_margins.right);
    let margin_bottom_pt = mm_to_pt(config.sheet_margins.bottom);

    let mut lines = Vec::new();

    // Vertical cut lines (between columns)
    if layout.columns > 1 {
        for col in 1..layout.columns {
            // Find the x-coordinate at the midpoint of the gap between col-1 and col
            // Use LTR physical coordinates for line placement
            let left_col_right_edge =
                if let Some(slot) = layout.slots.iter().find(|s| s.col == col - 1 && s.row == 0) {
                    slot.x_pt + slot.width_pt
                } else {
                    continue;
                };
            let right_col_left_edge =
                if let Some(slot) = layout.slots.iter().find(|s| s.col == col && s.row == 0) {
                    slot.x_pt
                } else {
                    continue;
                };

            let x = (left_col_right_edge + right_col_left_edge) / 2.0;
            let y1 = margin_top_pt * 0.5; // Extend slightly into margin
            let y2 = layout.sheet_height_pt - margin_bottom_pt * 0.5;

            lines.push(PositionedElement::Line(LineElement {
                x1: x,
                y1: y1,
                x2: x,
                y2: y2,
                color,
                width: line_width,
            }));
        }
    }

    // Horizontal cut lines (between rows)
    if layout.rows > 1 {
        for row in 1..layout.rows {
            // Find the y-coordinate at the midpoint of the gap between row-1 and row
            let top_row_bottom =
                if let Some(slot) = layout.slots.iter().find(|s| s.row == row - 1 && s.col == 0) {
                    slot.y_pt + slot.height_pt
                } else {
                    // Try any slot in the row
                    if let Some(slot) = layout.slots.iter().find(|s| s.row == row - 1) {
                        slot.y_pt + slot.height_pt
                    } else {
                        continue;
                    }
                };
            let bottom_row_top = if let Some(slot) = layout.slots.iter().find(|s| s.row == row) {
                slot.y_pt
            } else {
                continue;
            };

            let y = (top_row_bottom + bottom_row_top) / 2.0;
            let x1 = margin_left_pt * 0.5;
            let x2 = layout.sheet_width_pt - margin_right_pt * 0.5;

            lines.push(PositionedElement::Line(LineElement {
                x1: x1,
                y1: y,
                x2: x2,
                y2: y,
                color,
                width: line_width,
            }));
        }
    }

    lines
}

// ── Paginator ───────────────────────────────────────────────

/// Split a list of data contexts into pages, where each page contains
/// at most `items_per_sheet` items. Returns (page_index, items_in_page).
pub fn paginate_contexts<T>(contexts: &[T], items_per_sheet: u32) -> Vec<&[T]> {
    if items_per_sheet == 0 {
        return Vec::new();
    }
    contexts.chunks(items_per_sheet as usize).collect()
}

/// Compute total number of physical sheets needed.
pub fn compute_total_sheets(total_records: usize, items_per_sheet: u32) -> u32 {
    if items_per_sheet == 0 {
        return 0;
    }
    ((total_records as f64) / (items_per_sheet as f64)).ceil() as u32
}

// ── Suggestion Engine ───────────────────────────────────────

/// Suggest possible multi-up layouts for a given sheet+template combination.
/// Returns layouts sorted by items_per_sheet descending.
pub fn suggest_layouts(
    sheet_size: &SheetSize,
    template_width_mm: f64,
    template_height_mm: f64,
) -> Vec<MultiUpLayout> {
    let mut results = Vec::new();

    let orientations = [Orientation::Portrait, Orientation::Landscape];
    let gaps = [2.0, 3.0, 5.0];

    for orientation in &orientations {
        for gap in &gaps {
            let config = MultiUpConfig {
                sheet_size: sheet_size.clone(),
                sheet_orientation: orientation.clone(),
                sheet_margins: SheetMargins::default(),
                template_width_mm,
                template_height_mm,
                gap_horizontal_mm: *gap,
                gap_vertical_mm: *gap,
                fill_order: FillOrder::default(),
                cut_line_width_mm: 0.5,
                cut_line_color: default_cut_line_color(),
                forced_grid: None,
                center_on_sheet: true,
            };

            let layout = config.compute_layout();
            if layout.items_per_sheet > 0 {
                results.push(layout);
            }
        }
    }

    // Deduplicate by items_per_sheet and sort descending
    results.sort_by(|a, b| b.items_per_sheet.cmp(&a.items_per_sheet));
    results.dedup_by_key(|l| l.items_per_sheet);
    results
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(
        sheet: SheetSize,
        orientation: Orientation,
        tpl_w: f64,
        tpl_h: f64,
    ) -> MultiUpConfig {
        MultiUpConfig {
            sheet_size: sheet,
            sheet_orientation: orientation,
            sheet_margins: SheetMargins {
                top: 10.0,
                bottom: 10.0,
                left: 10.0,
                right: 10.0,
            },
            template_width_mm: tpl_w,
            template_height_mm: tpl_h,
            gap_horizontal_mm: 3.0,
            gap_vertical_mm: 3.0,
            fill_order: FillOrder::LeftToRightTopToBottom,
            cut_line_width_mm: 0.5,
            cut_line_color: "#cccccc".to_string(),
            forced_grid: None,
            center_on_sheet: true,
        }
    }

    #[test]
    fn test_a4_portrait_id_card_85x54() {
        // ID Card: 85.6mm × 53.98mm on A4 Portrait (210×297mm)
        // Printable: 190×277mm
        // Expected: 2 cols × 5 rows = 10 cards
        let config = make_config(SheetSize::A4, Orientation::Portrait, 85.6, 53.98);
        let layout = config.compute_layout();

        assert_eq!(layout.columns, 2, "Should fit 2 columns of 85.6mm cards");
        assert_eq!(layout.rows, 4, "Should fit 4-5 rows of 54mm cards");
        assert!(
            layout.items_per_sheet >= 8,
            "Should fit at least 8 ID cards on A4"
        );
        assert_eq!(layout.slots.len(), layout.items_per_sheet as usize);

        // Verify all slots are within sheet bounds
        for slot in &layout.slots {
            assert!(slot.x_pt >= 0.0, "Slot x should be >= 0");
            assert!(slot.y_pt >= 0.0, "Slot y should be >= 0");
            assert!(
                slot.x_pt + slot.width_pt <= layout.sheet_width_pt + 0.01,
                "Slot right edge should be within sheet"
            );
            assert!(
                slot.y_pt + slot.height_pt <= layout.sheet_height_pt + 0.01,
                "Slot bottom edge should be within sheet"
            );
        }
    }

    #[test]
    fn test_a4_portrait_seating_number_90x50() {
        // Seating number card: 90mm × 50mm on A4 Portrait
        // Printable: 190×277mm
        // Expected: 2 cols × 5 rows = 10
        let config = make_config(SheetSize::A4, Orientation::Portrait, 90.0, 50.0);
        let layout = config.compute_layout();

        assert_eq!(layout.columns, 2);
        assert_eq!(layout.rows, 5);
        assert_eq!(layout.items_per_sheet, 10);
    }

    #[test]
    fn test_a4_landscape_seating_number() {
        // Same card on A4 Landscape (297×210mm)
        // Printable: 277×190mm
        let config = make_config(SheetSize::A4, Orientation::Landscape, 90.0, 50.0);
        let layout = config.compute_layout();

        // A4 Landscape printable width = 277mm → (277+3)/(90+3) = 3.01 → 3 columns
        assert_eq!(layout.columns, 3, "Should fit 3 cols of 90mm in 277mm wide");
        assert!(
            layout.rows >= 3,
            "Should fit at least 3 rows of 50mm in 190mm"
        );
        // landscape might actually fit 3 cols: (277+3)/(90+3) = 3.01 → 3
        assert!(layout.items_per_sheet >= 6);
    }

    #[test]
    fn test_a4_full_page_certificate() {
        // Full A4 certificate: 190mm × 267mm on A4
        // Should fit exactly 1 per page
        let config = make_config(SheetSize::A4, Orientation::Portrait, 190.0, 267.0);
        let layout = config.compute_layout();

        assert_eq!(layout.columns, 1);
        assert_eq!(layout.rows, 1);
        assert_eq!(layout.items_per_sheet, 1);
    }

    #[test]
    fn test_sheet_sized_template_is_centered_without_margin_offset() {
        let config = make_config(SheetSize::A4, Orientation::Portrait, 210.0, 297.0);
        let layout = config.compute_layout();
        let slot = &layout.slots[0];

        assert_eq!(layout.items_per_sheet, 1);
        assert!(
            slot.x_pt.abs() < 0.01,
            "Full-sheet template should start at the sheet's left edge, got {}pt",
            slot.x_pt
        );
        assert!(
            slot.y_pt.abs() < 0.01,
            "Full-sheet template should start at the sheet's top edge, got {}pt",
            slot.y_pt
        );
    }

    #[test]
    fn test_a3_small_badges() {
        // Small badges: 86mm × 54mm on A3 Portrait (297×420mm)
        // Printable: 277×400mm
        // Expected: 3 cols × 7 rows = 21
        let config = make_config(SheetSize::A3, Orientation::Portrait, 86.0, 54.0);
        let layout = config.compute_layout();

        assert!(layout.columns >= 3, "A3 should fit >= 3 cols of 86mm");
        assert!(layout.rows >= 7, "A3 should fit >= 7 rows of 54mm");
        assert!(
            layout.items_per_sheet >= 21,
            "Should fit >= 21 badges on A3"
        );
    }

    #[test]
    fn test_forced_grid() {
        // Force 3×2 layout regardless of available space
        let mut config = make_config(SheetSize::A4, Orientation::Portrait, 50.0, 50.0);
        config.forced_grid = Some(ForcedGrid {
            columns: 3,
            rows: 2,
        });
        let layout = config.compute_layout();

        assert_eq!(layout.columns, 3);
        assert_eq!(layout.rows, 2);
        assert_eq!(layout.items_per_sheet, 6);
    }

    #[test]
    fn test_rtl_fill_order() {
        let mut config = make_config(SheetSize::A4, Orientation::Portrait, 90.0, 50.0);
        config.fill_order = FillOrder::RightToLeftTopToBottom;
        let layout = config.compute_layout();

        // First slot (index=0) should be at the rightmost column
        let first_slot = &layout.slots[0];
        let second_slot = &layout.slots[1];

        assert!(
            first_slot.x_pt > second_slot.x_pt,
            "RTL: first slot should be to the right of second"
        );
        assert_eq!(first_slot.row, 0);
        assert_eq!(second_slot.row, 0);
    }

    #[test]
    fn test_ltr_fill_order() {
        let config = make_config(SheetSize::A4, Orientation::Portrait, 90.0, 50.0);
        let layout = config.compute_layout();

        let first_slot = &layout.slots[0];
        let second_slot = &layout.slots[1];

        assert!(
            first_slot.x_pt < second_slot.x_pt,
            "LTR: first slot should be to the left of second"
        );
    }

    #[test]
    fn test_centering() {
        let config = make_config(SheetSize::A4, Orientation::Portrait, 90.0, 50.0);
        let layout = config.compute_layout();

        // Content should be centered: check symmetry
        let first_slot = layout
            .slots
            .iter()
            .min_by(|a, b| a.x_pt.partial_cmp(&b.x_pt).unwrap())
            .unwrap();
        let last_slot = layout
            .slots
            .iter()
            .max_by(|a, b| {
                (a.x_pt + a.width_pt)
                    .partial_cmp(&(b.x_pt + b.width_pt))
                    .unwrap()
            })
            .unwrap();

        let left_margin = first_slot.x_pt;
        let right_margin = layout.sheet_width_pt - (last_slot.x_pt + last_slot.width_pt);

        // Margins should be roughly equal (within 1pt tolerance)
        assert!(
            (left_margin - right_margin).abs() < 1.0,
            "Content should be horizontally centered: left={:.1}, right={:.1}",
            left_margin,
            right_margin
        );
    }

    #[test]
    fn test_slots_no_overlap() {
        let config = make_config(SheetSize::A4, Orientation::Portrait, 90.0, 50.0);
        let layout = config.compute_layout();

        for i in 0..layout.slots.len() {
            for j in (i + 1)..layout.slots.len() {
                let a = &layout.slots[i];
                let b = &layout.slots[j];

                let overlap_x = a.x_pt < b.x_pt + b.width_pt && b.x_pt < a.x_pt + a.width_pt;
                let overlap_y = a.y_pt < b.y_pt + b.height_pt && b.y_pt < a.y_pt + a.height_pt;

                assert!(
                    !(overlap_x && overlap_y),
                    "Slots {} and {} overlap! a=({:.1},{:.1})+({}×{}), b=({:.1},{:.1})+({}×{})",
                    i,
                    j,
                    a.x_pt,
                    a.y_pt,
                    a.width_pt,
                    a.height_pt,
                    b.x_pt,
                    b.y_pt,
                    b.width_pt,
                    b.height_pt
                );
            }
        }
    }

    #[test]
    fn test_cut_lines_generated() {
        let config = make_config(SheetSize::A4, Orientation::Portrait, 90.0, 50.0);
        let layout = config.compute_layout();
        let cut_lines = generate_cut_lines(&layout, &config);

        // Should have (cols-1) vertical + (rows-1) horizontal lines
        let expected_vertical = (layout.columns - 1) as usize;
        let expected_horizontal = (layout.rows - 1) as usize;
        assert_eq!(
            cut_lines.len(),
            expected_vertical + expected_horizontal,
            "Expected {} vertical + {} horizontal cut lines",
            expected_vertical,
            expected_horizontal
        );
    }

    #[test]
    fn test_cut_lines_disabled() {
        let mut config = make_config(SheetSize::A4, Orientation::Portrait, 90.0, 50.0);
        config.cut_line_width_mm = 0.0;
        let layout = config.compute_layout();
        let cut_lines = generate_cut_lines(&layout, &config);
        assert!(cut_lines.is_empty(), "No cut lines when width is 0");
    }

    #[test]
    fn test_paginate_contexts() {
        let items: Vec<i32> = (0..25).collect();
        let pages = paginate_contexts(&items, 8);
        assert_eq!(pages.len(), 4); // 8+8+8+1
        assert_eq!(pages[0].len(), 8);
        assert_eq!(pages[3].len(), 1);
    }

    #[test]
    fn test_compute_total_sheets() {
        assert_eq!(compute_total_sheets(0, 8), 0);
        assert_eq!(compute_total_sheets(1, 8), 1);
        assert_eq!(compute_total_sheets(8, 8), 1);
        assert_eq!(compute_total_sheets(9, 8), 2);
        assert_eq!(compute_total_sheets(120, 8), 15);
        assert_eq!(compute_total_sheets(50000, 10), 5000);
    }

    #[test]
    fn test_suggest_layouts() {
        let suggestions = suggest_layouts(&SheetSize::A4, 85.6, 53.98);
        assert!(
            !suggestions.is_empty(),
            "Should have at least one suggestion"
        );
        // Should be sorted by items_per_sheet descending
        for i in 1..suggestions.len() {
            assert!(
                suggestions[i - 1].items_per_sheet >= suggestions[i].items_per_sheet,
                "Suggestions should be sorted descending"
            );
        }
    }

    #[test]
    fn test_template_larger_than_sheet() {
        // Template larger than printable area → should fit 1
        let config = make_config(SheetSize::A5, Orientation::Portrait, 200.0, 300.0);
        let layout = config.compute_layout();

        // compute_max_items returns 0 when template > printable, forced to max(1)
        assert_eq!(layout.columns, 1);
        assert_eq!(layout.rows, 1);
        assert_eq!(layout.items_per_sheet, 1);
    }

    #[test]
    fn test_custom_sheet_size() {
        let config = make_config(
            SheetSize::Custom {
                width_mm: 300.0,
                height_mm: 400.0,
            },
            Orientation::Portrait,
            90.0,
            50.0,
        );
        let layout = config.compute_layout();

        // Custom 300×400, printable 280×380
        // Cols: (280+3)/(90+3) = 3.04 → 3
        // Rows: (380+3)/(50+3) = 7.22 → 7
        assert_eq!(layout.columns, 3);
        assert_eq!(layout.rows, 7);
        assert_eq!(layout.items_per_sheet, 21);
    }

    #[test]
    fn test_offset_element_text() {
        use crate::render::types::*;
        let elem = PositionedElement::Text(TextElement {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
            content: "Test".to_string(),
            font_family: "Rubik".to_string(),
            font_size: 12.0,
            font_weight: FontWeight::Normal,
            font_style: FontStyle::Normal,
            color: Color::BLACK,
            text_align: TextAlign::Start,
            direction: TextDirection::Ltr,
            line_height: 15.0,
        });

        let offset = offset_element(&elem, 50.0, 100.0);
        if let PositionedElement::Text(t) = offset {
            assert_eq!(t.x, 60.0);
            assert_eq!(t.y, 120.0);
            assert_eq!(t.content, "Test");
        } else {
            panic!("Expected Text element");
        }
    }

    #[test]
    fn test_offset_element_rect() {
        use crate::render::types::*;
        let elem = PositionedElement::Rect(RectElement {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 100.0,
            fill: Some(Color::WHITE),
            stroke: None,
            stroke_width: 0.0,
            border_radius: 0.0,
        });

        let offset = offset_element(&elem, 30.0, 40.0);
        if let PositionedElement::Rect(r) = offset {
            assert_eq!(r.x, 30.0);
            assert_eq!(r.y, 40.0);
            assert_eq!(r.width, 200.0); // Width unchanged
        } else {
            panic!("Expected Rect element");
        }
    }

    #[test]
    fn test_offset_element_line() {
        use crate::render::types::*;
        let elem = PositionedElement::Line(LineElement {
            x1: 5.0,
            y1: 10.0,
            x2: 100.0,
            y2: 10.0,
            color: Color::BLACK,
            width: 1.0,
        });

        let offset = offset_element(&elem, 20.0, 30.0);
        if let PositionedElement::Line(l) = offset {
            assert_eq!(l.x1, 25.0);
            assert_eq!(l.y1, 40.0);
            assert_eq!(l.x2, 120.0);
            assert_eq!(l.y2, 40.0);
        } else {
            panic!("Expected Line element");
        }
    }
}
