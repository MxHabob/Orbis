//! Flow Layout Engine — converts `ReportDataset` into `LayoutResult` for PDF rendering.
//!
//! This is the "flow" counterpart to DocForge's "canvas" layout engine.
//! Instead of absolute positioning, flow layout arranges content vertically:
//!
//! 1. Org header (fixed at top of first page, or every page)
//! 2. Report title and parameters
//! 3. Table with automatic page-break handling
//! 4. Grand totals row
//! 5. Footer with page numbers and generation timestamp
//!
//! Uses the same `PositionedElement` types as DocForge, so the existing
//! `pdf_renderer::render_to_pdf()` function works without modification.

use docforge_engine::render::types::*;

use crate::dataset::{ReportDataset, ColumnDef, ColumnDataType, ColumnAlign as DataColumnAlign};
use crate::request::ReportLocale;

// ── Page Configuration ──────────────────────────────────────

/// A4 dimensions in PDF points
const A4_WIDTH: f32 = 595.28;
const A4_HEIGHT: f32 = 841.89;

/// Page margins in PDF points
const MARGIN_TOP: f32 = 40.0;
const MARGIN_BOTTOM: f32 = 50.0;
const MARGIN_LEFT: f32 = 40.0;
const MARGIN_RIGHT: f32 = 40.0;

/// Vertical spacing
const HEADER_HEIGHT: f32 = 60.0;
const TITLE_HEIGHT: f32 = 24.0;
const PARAM_LINE_HEIGHT: f32 = 14.0;
const TABLE_HEADER_HEIGHT: f32 = 22.0;
const ROW_HEIGHT: f32 = 18.0;
#[allow(dead_code)]
const GROUP_HEADER_HEIGHT: f32 = 20.0;
const TOTALS_ROW_HEIGHT: f32 = 22.0;
const FOOTER_HEIGHT: f32 = 16.0;
const SPACING: f32 = 8.0;

/// Font sizes
const FONT_ORG_NAME: f32 = 14.0;
const FONT_TITLE: f32 = 12.0;
const FONT_PARAM: f32 = 8.0;
const FONT_TABLE_HEADER: f32 = 8.5;
const FONT_CELL: f32 = 8.0;
const FONT_TOTAL: f32 = 8.5;
const FONT_FOOTER: f32 = 7.0;

/// Colors
const COLOR_HEADER_BG: Color = Color { r: 44, g: 95, b: 45, a: 255 };     // Dark green
const COLOR_HEADER_TEXT: Color = Color { r: 255, g: 255, b: 255, a: 255 };
const COLOR_ALT_ROW: Color = Color { r: 245, g: 250, b: 245, a: 255 };     // Light green
const COLOR_TOTAL_BG: Color = Color { r: 232, g: 245, b: 233, a: 255 };    // Soft green
#[allow(dead_code)]
const COLOR_GROUP_BG: Color = Color { r: 230, g: 240, b: 250, a: 255 };    // Soft blue
const COLOR_BORDER: Color = Color { r: 200, g: 200, b: 200, a: 255 };
const COLOR_TEXT: Color = Color { r: 30, g: 30, b: 30, a: 255 };
const COLOR_SUBTITLE: Color = Color { r: 100, g: 100, b: 100, a: 255 };

const DEFAULT_FONT: &str = "Noto Sans Arabic";

/// Generate a complete `LayoutResult` from a `ReportDataset`.
///
/// This produces a multi-page PDF layout with:
/// - Organization header on every page
/// - Table header repeated on every page
/// - Automatic page breaks
/// - Alternating row colors
/// - Grand totals at the end
/// - Footer with page numbers
pub fn generate_flow_layout(
    dataset: &ReportDataset,
    locale: &ReportLocale,
) -> LayoutResult {
    let is_rtl = locale.is_rtl();
    let printable_width = A4_WIDTH - MARGIN_LEFT - MARGIN_RIGHT;

    // Determine visible columns and compute widths
    let visible_cols: Vec<&ColumnDef> = dataset.columns.iter()
        .filter(|c| c.visible)
        .collect();

    let col_widths = compute_column_widths(&visible_cols, printable_width);

    // Pre-compute content
    let org_name = match locale {
        ReportLocale::Ar | ReportLocale::ArEn => &dataset.org.name_ar,
        ReportLocale::En | ReportLocale::EnAr => &dataset.org.name,
    };
    let report_title = match locale {
        ReportLocale::Ar | ReportLocale::ArEn => &dataset.report_name_ar,
        ReportLocale::En | ReportLocale::EnAr => &dataset.report_name,
    };

    // Count total pages needed (estimate)
    let content_per_page = A4_HEIGHT - MARGIN_TOP - MARGIN_BOTTOM - HEADER_HEIGHT
        - TITLE_HEIGHT - TABLE_HEADER_HEIGHT - FOOTER_HEIGHT - SPACING * 3.0;
    let rows_per_page = (content_per_page / ROW_HEIGHT).floor() as usize;
    let total_pages = if dataset.rows.is_empty() {
        1
    } else {
        (dataset.rows.len() + rows_per_page - 1) / rows_per_page
    };

    let mut pages = Vec::new();
    let mut row_idx = 0;

    for page_num in 0..total_pages.max(1) {
        let mut elements = Vec::new();
        let mut y = MARGIN_TOP;

        // ── Org Header ──────────────────────────────────────
        // Org name centered
        elements.push(PositionedElement::Text(TextElement {
            x: MARGIN_LEFT,
            y,
            width: printable_width,
            height: FONT_ORG_NAME + 4.0,
            content: org_name.clone(),
            font_family: DEFAULT_FONT.into(),
            font_size: FONT_ORG_NAME,
            font_weight: FontWeight::Bold,
            font_style: FontStyle::Normal,
            color: COLOR_TEXT,
            text_align: TextAlign::Center,
            direction: if is_rtl { TextDirection::Rtl } else { TextDirection::Ltr },
            line_height: 1.3,
        }));
        y += FONT_ORG_NAME + 8.0;

        // Separator line
        elements.push(PositionedElement::Line(LineElement {
            x1: MARGIN_LEFT,
            y1: y,
            x2: A4_WIDTH - MARGIN_RIGHT,
            y2: y,
            color: COLOR_BORDER,
            width: 0.5,
        }));
        y += 4.0;

        // Report title
        elements.push(PositionedElement::Text(TextElement {
            x: MARGIN_LEFT,
            y,
            width: printable_width,
            height: TITLE_HEIGHT,
            content: report_title.clone(),
            font_family: DEFAULT_FONT.into(),
            font_size: FONT_TITLE,
            font_weight: FontWeight::Bold,
            font_style: FontStyle::Normal,
            color: COLOR_TEXT,
            text_align: TextAlign::Center,
            direction: if is_rtl { TextDirection::Rtl } else { TextDirection::Ltr },
            line_height: 1.3,
        }));
        y += TITLE_HEIGHT;

        // Parameters (only on first page)
        if page_num == 0 {
            for param in &dataset.parameter_summary {
                let (label, value) = match locale {
                    ReportLocale::Ar | ReportLocale::ArEn => (&param.label_ar, &param.value_ar),
                    ReportLocale::En | ReportLocale::EnAr => (&param.label, &param.value),
                };
                elements.push(PositionedElement::Text(TextElement {
                    x: MARGIN_LEFT,
                    y,
                    width: printable_width,
                    height: PARAM_LINE_HEIGHT,
                    content: format!("{}: {}", label, value),
                    font_family: DEFAULT_FONT.into(),
                    font_size: FONT_PARAM,
                    font_weight: FontWeight::Normal,
                    font_style: FontStyle::Italic,
                    color: COLOR_SUBTITLE,
                    text_align: if is_rtl { TextAlign::End } else { TextAlign::Start },
                    direction: if is_rtl { TextDirection::Rtl } else { TextDirection::Ltr },
                    line_height: 1.2,
                }));
                y += PARAM_LINE_HEIGHT;
            }
        }

        y += SPACING;

        // ── Table Header ────────────────────────────────────
        // Header background
        elements.push(PositionedElement::Rect(RectElement {
            x: MARGIN_LEFT,
            y,
            width: printable_width,
            height: TABLE_HEADER_HEIGHT,
            fill: Some(COLOR_HEADER_BG),
            stroke: None,
            stroke_width: 0.0,
            border_radius: 0.0,
        }));

        let mut col_x = if is_rtl {
            A4_WIDTH - MARGIN_RIGHT
        } else {
            MARGIN_LEFT
        };

        for (ci, col) in visible_cols.iter().enumerate() {
            let w = col_widths[ci];
            let header_text = match locale {
                ReportLocale::Ar | ReportLocale::ArEn => &col.header_ar,
                ReportLocale::En | ReportLocale::EnAr => &col.header,
            };

            let cell_x = if is_rtl { col_x - w } else { col_x };

            elements.push(PositionedElement::Text(TextElement {
                x: cell_x + 3.0,
                y: y + 3.0,
                width: w - 6.0,
                height: TABLE_HEADER_HEIGHT - 6.0,
                content: header_text.clone(),
                font_family: DEFAULT_FONT.into(),
                font_size: FONT_TABLE_HEADER,
                font_weight: FontWeight::Bold,
                font_style: FontStyle::Normal,
                color: COLOR_HEADER_TEXT,
                text_align: col_text_align(col, is_rtl),
                direction: if is_rtl { TextDirection::Rtl } else { TextDirection::Ltr },
                line_height: 1.2,
            }));

            if is_rtl { col_x -= w; } else { col_x += w; }
        }

        y += TABLE_HEADER_HEIGHT;

        // ── Data Rows ───────────────────────────────────────
        let max_y = A4_HEIGHT - MARGIN_BOTTOM - FOOTER_HEIGHT - TOTALS_ROW_HEIGHT;
        let mut local_row = 0;

        while row_idx < dataset.rows.len() && y + ROW_HEIGHT <= max_y {
            let row = &dataset.rows[row_idx];

            // Alternating row background
            if local_row % 2 == 1 {
                elements.push(PositionedElement::Rect(RectElement {
                    x: MARGIN_LEFT,
                    y,
                    width: printable_width,
                    height: ROW_HEIGHT,
                    fill: Some(COLOR_ALT_ROW),
                    stroke: None,
                    stroke_width: 0.0,
                    border_radius: 0.0,
                }));
            }

            // Bottom border
            elements.push(PositionedElement::Line(LineElement {
                x1: MARGIN_LEFT,
                y1: y + ROW_HEIGHT,
                x2: A4_WIDTH - MARGIN_RIGHT,
                y2: y + ROW_HEIGHT,
                color: Color { r: 230, g: 230, b: 230, a: 255 },
                width: 0.3,
            }));

            col_x = if is_rtl { A4_WIDTH - MARGIN_RIGHT } else { MARGIN_LEFT };

            for (ci, col) in visible_cols.iter().enumerate() {
                let w = col_widths[ci];
                let cell_x = if is_rtl { col_x - w } else { col_x };

                let cell_text = format_cell_value(&row[&col.key], &col.data_type);

                elements.push(PositionedElement::Text(TextElement {
                    x: cell_x + 3.0,
                    y: y + 2.0,
                    width: w - 6.0,
                    height: ROW_HEIGHT - 4.0,
                    content: cell_text,
                    font_family: DEFAULT_FONT.into(),
                    font_size: FONT_CELL,
                    font_weight: FontWeight::Normal,
                    font_style: FontStyle::Normal,
                    color: COLOR_TEXT,
                    text_align: col_text_align(col, is_rtl),
                    direction: TextDirection::Auto,
                    line_height: 1.2,
                }));

                if is_rtl { col_x -= w; } else { col_x += w; }
            }

            y += ROW_HEIGHT;
            row_idx += 1;
            local_row += 1;
        }

        // ── Grand Totals (only on last page, after all rows) ──
        if row_idx >= dataset.rows.len() {
            if let Some(ref agg) = dataset.aggregations {
                // Totals background
                elements.push(PositionedElement::Rect(RectElement {
                    x: MARGIN_LEFT,
                    y,
                    width: printable_width,
                    height: TOTALS_ROW_HEIGHT,
                    fill: Some(COLOR_TOTAL_BG),
                    stroke: Some(COLOR_BORDER),
                    stroke_width: 0.5,
                    border_radius: 0.0,
                }));

                col_x = if is_rtl { A4_WIDTH - MARGIN_RIGHT } else { MARGIN_LEFT };

                for (ci, col) in visible_cols.iter().enumerate() {
                    let w = col_widths[ci];
                    let cell_x = if is_rtl { col_x - w } else { col_x };

                    let content = if ci == 0 {
                        match locale {
                            ReportLocale::Ar | ReportLocale::ArEn => "المجموع".to_string(),
                            ReportLocale::En | ReportLocale::EnAr => "Total".to_string(),
                        }
                    } else if let Some(val) = agg.totals.get(&col.key) {
                        format_cell_value(val, &col.data_type)
                    } else {
                        String::new()
                    };

                    elements.push(PositionedElement::Text(TextElement {
                        x: cell_x + 3.0,
                        y: y + 3.0,
                        width: w - 6.0,
                        height: TOTALS_ROW_HEIGHT - 6.0,
                        content,
                        font_family: DEFAULT_FONT.into(),
                        font_size: FONT_TOTAL,
                        font_weight: FontWeight::Bold,
                        font_style: FontStyle::Normal,
                        color: COLOR_TEXT,
                        text_align: col_text_align(col, is_rtl),
                        direction: if is_rtl { TextDirection::Rtl } else { TextDirection::Ltr },
                        line_height: 1.2,
                    }));

                    if is_rtl { col_x -= w; } else { col_x += w; }
                }

                let _y = y + TOTALS_ROW_HEIGHT;
            }
        }

        // ── Footer ──────────────────────────────────────────
        let footer_y = A4_HEIGHT - MARGIN_BOTTOM;

        // Separator line
        elements.push(PositionedElement::Line(LineElement {
            x1: MARGIN_LEFT,
            y1: footer_y - FOOTER_HEIGHT - 2.0,
            x2: A4_WIDTH - MARGIN_RIGHT,
            y2: footer_y - FOOTER_HEIGHT - 2.0,
            color: COLOR_BORDER,
            width: 0.3,
        }));

        // Page number
        let page_text = format!("{} / {}", page_num + 1, total_pages);
        elements.push(PositionedElement::Text(TextElement {
            x: MARGIN_LEFT,
            y: footer_y - FOOTER_HEIGHT,
            width: printable_width,
            height: FOOTER_HEIGHT,
            content: page_text,
            font_family: DEFAULT_FONT.into(),
            font_size: FONT_FOOTER,
            font_weight: FontWeight::Normal,
            font_style: FontStyle::Normal,
            color: COLOR_SUBTITLE,
            text_align: TextAlign::Center,
            direction: TextDirection::Ltr,
            line_height: 1.2,
        }));

        // Generated timestamp (on last page)
        if row_idx >= dataset.rows.len() || page_num == total_pages - 1 {
            elements.push(PositionedElement::Text(TextElement {
                x: MARGIN_LEFT,
                y: footer_y - FOOTER_HEIGHT,
                width: printable_width,
                height: FOOTER_HEIGHT,
                content: dataset.generated_at.clone(),
                font_family: DEFAULT_FONT.into(),
                font_size: FONT_FOOTER,
                font_weight: FontWeight::Normal,
                font_style: FontStyle::Italic,
                color: Color { r: 160, g: 160, b: 160, a: 255 },
                text_align: if is_rtl { TextAlign::Start } else { TextAlign::End },
                direction: TextDirection::Ltr,
                line_height: 1.2,
            }));
        }

        pages.push(PageLayout {
            width_pt: A4_WIDTH,
            height_pt: A4_HEIGHT,
            elements,
        });
    }

    LayoutResult { pages }
}

// ── Helpers ─────────────────────────────────────────────────

/// Compute column widths based on percentage hints, distributing remaining space evenly.
fn compute_column_widths(cols: &[&ColumnDef], total_width: f32) -> Vec<f32> {
    let mut widths = Vec::with_capacity(cols.len());
    let mut allocated = 0.0f32;
    let mut unspecified = 0usize;

    for col in cols {
        if let Some(ref w) = col.width {
            if let Some(pct) = w.strip_suffix('%') {
                if let Ok(p) = pct.parse::<f32>() {
                    let w = total_width * p / 100.0;
                    widths.push(w);
                    allocated += w;
                    continue;
                }
            }
        }
        widths.push(0.0); // placeholder
        unspecified += 1;
    }

    // Distribute remaining space
    if unspecified > 0 {
        let remaining = (total_width - allocated).max(0.0);
        let per_col = remaining / unspecified as f32;
        for w in &mut widths {
            if *w == 0.0 {
                *w = per_col;
            }
        }
    }

    widths
}

/// Map ColumnDef alignment to TextAlign, respecting RTL.
fn col_text_align(col: &ColumnDef, _is_rtl: bool) -> TextAlign {
    match col.align {
        Some(DataColumnAlign::Center) => TextAlign::Center,
        Some(DataColumnAlign::End) => TextAlign::End,
        Some(DataColumnAlign::Start) => TextAlign::Start,
        None => {
            // Auto-align based on data type
            match col.data_type {
                ColumnDataType::Currency | ColumnDataType::Decimal | ColumnDataType::Percentage => TextAlign::End,
                ColumnDataType::Sequence | ColumnDataType::Number => TextAlign::Center,
                _ => TextAlign::Start,
            }
        }
    }
}

/// Format a JSON value for display in a PDF cell.
fn format_cell_value(val: &serde_json::Value, data_type: &ColumnDataType) -> String {
    match val {
        serde_json::Value::Null => String::new(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => {
            match data_type {
                ColumnDataType::Currency | ColumnDataType::Decimal => {
                    if let Some(f) = n.as_f64() {
                        format!("{:.2}", f)
                    } else {
                        n.to_string()
                    }
                }
                ColumnDataType::Percentage => {
                    if let Some(f) = n.as_f64() {
                        format!("{:.1}%", f)
                    } else {
                        n.to_string()
                    }
                }
                _ => n.to_string(),
            }
        }
        serde_json::Value::Bool(b) => {
            if *b { "✓".to_string() } else { "✗".to_string() }
        }
        _ => val.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::*;
    use crate::org_context::OrgContext;
    use serde_json::json;

    fn sample_dataset(row_count: usize) -> ReportDataset {
        let mut rows = Vec::with_capacity(row_count);
        for i in 0..row_count {
            rows.push(json!({
                "seq": i + 1,
                "name": format!("Student {}", i + 1),
                "amount": 1500.0 + (i as f64 * 100.0),
            }));
        }

        ReportDataset {
            report_code: "test.report".into(),
            report_name: "Test Report".into(),
            report_name_ar: "تقرير اختبار".into(),
            org: OrgContext {
                name: "Al-Noor International School".into(),
                name_ar: "مدرسة النور الدولية".into(),
                ..Default::default()
            },
            parameter_summary: vec![ParameterSummaryItem {
                label: "Period".into(), label_ar: "الفترة".into(),
                value: "2025-2026".into(), value_ar: "2025-2026".into(),
            }],
            columns: vec![
                ColumnDef {
                    key: "seq".into(), header: "#".into(), header_ar: "#".into(),
                    data_type: ColumnDataType::Sequence, width: Some("5%".into()),
                    align: Some(ColumnAlign::Center), format: None, visible: true,
                    total_type: Some(AggregationType::Count), sensitive: false,
                },
                ColumnDef {
                    key: "name".into(), header: "Name".into(), header_ar: "الاسم".into(),
                    data_type: ColumnDataType::Text, width: Some("60%".into()),
                    align: None, format: None, visible: true,
                    total_type: None, sensitive: false,
                },
                ColumnDef {
                    key: "amount".into(), header: "Amount".into(), header_ar: "المبلغ".into(),
                    data_type: ColumnDataType::Currency, width: Some("35%".into()),
                    align: Some(ColumnAlign::End), format: None, visible: true,
                    total_type: Some(AggregationType::Sum), sensitive: false,
                },
            ],
            rows,
            groups: None,
            aggregations: Some(Aggregations {
                totals: json!({"seq": row_count, "amount": row_count as f64 * 1550.0}),
                subtotals: None,
            }),
            total_count: row_count as u64,
            page: 0,
            page_size: 0,
            generated_at: "2026-01-01T00:00:00Z".into(),
            generated_by: None,
        }
    }

    #[test]
    fn test_single_page_layout() {
        let dataset = sample_dataset(5);
        let layout = generate_flow_layout(&dataset, &ReportLocale::En);

        assert_eq!(layout.pages.len(), 1);
        assert_eq!(layout.pages[0].width_pt, A4_WIDTH);
        assert_eq!(layout.pages[0].height_pt, A4_HEIGHT);
        // Should have elements: org name, separator, title, params, table header bg,
        // 3 header texts, data rows (5 × cell elements), totals, footer
        assert!(layout.pages[0].elements.len() > 20);
    }

    #[test]
    fn test_multi_page_layout() {
        let dataset = sample_dataset(100);
        let layout = generate_flow_layout(&dataset, &ReportLocale::Ar);

        // 100 rows should span multiple pages
        assert!(layout.pages.len() >= 2, "Expected >= 2 pages, got {}", layout.pages.len());

        // Every page should have elements
        for (i, page) in layout.pages.iter().enumerate() {
            assert!(!page.elements.is_empty(), "Page {} has no elements", i);
        }
    }

    #[test]
    fn test_rtl_layout() {
        let dataset = sample_dataset(3);
        let layout = generate_flow_layout(&dataset, &ReportLocale::Ar);

        // Check that org name uses RTL direction
        let first_text = layout.pages[0].elements.iter().find_map(|e| match e {
            PositionedElement::Text(t) => Some(t),
            _ => None,
        }).unwrap();

        assert_eq!(first_text.direction, TextDirection::Rtl);
        assert!(first_text.content.contains("النور")); // Arabic org name
    }

    #[test]
    fn test_column_width_computation() {
        let cols = vec![
            ColumnDef {
                key: "a".into(), header: "A".into(), header_ar: "أ".into(),
                data_type: ColumnDataType::Text, width: Some("30%".into()),
                align: None, format: None, visible: true, total_type: None, sensitive: false,
            },
            ColumnDef {
                key: "b".into(), header: "B".into(), header_ar: "ب".into(),
                data_type: ColumnDataType::Text, width: None,
                align: None, format: None, visible: true, total_type: None, sensitive: false,
            },
        ];

        let col_refs: Vec<&ColumnDef> = cols.iter().collect();
        let widths = compute_column_widths(&col_refs, 500.0);

        assert_eq!(widths.len(), 2);
        assert!((widths[0] - 150.0).abs() < 0.1); // 30% of 500
        assert!((widths[1] - 350.0).abs() < 0.1); // remaining
    }

    #[test]
    fn test_empty_dataset() {
        let dataset = sample_dataset(0);
        let layout = generate_flow_layout(&dataset, &ReportLocale::En);

        assert_eq!(layout.pages.len(), 1);
        // Should still have header, title, and footer
        assert!(layout.pages[0].elements.len() > 5);
    }
}
