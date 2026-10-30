// ═══════════════════════════════════════════════════════════════
// docforge-engine — Layout Engine
// Computes absolute positions using Taffy and cosmic-text.
// Outputs a fully positioned LayoutResult.
// ═══════════════════════════════════════════════════════════════

use crate::error::DocforgeError;
use crate::layout::expr_eval::{evaluate_node_tree, EvaluatedNode, EvaluatedTable};
use crate::model::document::{Orientation, PageSetup, PageSize, TemplateDocument};
use crate::model::nodes::TextProps;
use crate::render::font_cache::{create_font_system_for_font, FontCache};
use crate::render::types::*;
use serde_json::Value;
use std::collections::HashMap;

/// Main Layout Engine structure.
pub struct LayoutEngine<'a> {
    fonts: &'a FontCache,
}

impl<'a> LayoutEngine<'a> {
    /// Create a new layout engine with a given font cache.
    pub fn new(fonts: &'a FontCache) -> Self {
        Self { fonts }
    }

    /// Layout a TemplateDocument using the provided data context.
    pub fn layout(
        &self,
        doc: &TemplateDocument,
        context: &Value,
    ) -> Result<LayoutResult, DocforgeError> {
        let (page_width, page_height) = get_page_dimensions(&doc.meta.page_setup);
        let margins = &doc.meta.page_setup.margins;

        let left_margin = mm_to_pt(margins.left);
        let right_margin = mm_to_pt(margins.right);
        let top_margin = mm_to_pt(margins.top);
        let bottom_margin = mm_to_pt(margins.bottom);

        let content_width = page_width - left_margin - right_margin;
        let content_height = page_height - top_margin - bottom_margin;

        // Step 1: Pre-evaluate the node tree (resolving conditionals/repeaters/expressions)
        let evaluated_nodes = evaluate_node_tree(&doc.root, context);

        // Step 2: Group nodes into pages based on PageBreak nodes
        let mut pages_nodes = Vec::new();
        let mut current_page_nodes = Vec::new();

        for node in evaluated_nodes {
            match node {
                EvaluatedNode::Page(page) => {
                    // If root is a page, split children into pages at PageBreak
                    for child in page.children {
                        if let EvaluatedNode::PageBreak = child {
                            if !current_page_nodes.is_empty() {
                                pages_nodes.push(std::mem::take(&mut current_page_nodes));
                            }
                        } else {
                            current_page_nodes.push(child);
                        }
                    }
                }
                EvaluatedNode::PageBreak => {
                    if !current_page_nodes.is_empty() {
                        pages_nodes.push(std::mem::take(&mut current_page_nodes));
                    }
                }
                _ => {
                    current_page_nodes.push(node);
                }
            }
        }
        if !current_page_nodes.is_empty() {
            pages_nodes.push(current_page_nodes);
        }
        if pages_nodes.is_empty() {
            // Guarantee at least one empty page
            pages_nodes.push(Vec::new());
        }

        // Step 3: Run layout computation for each page
        let mut pages = Vec::new();

        for page_idx in 0..pages_nodes.len() {
            let nodes = &pages_nodes[page_idx];
            let mut taffy = taffy::TaffyTree::<()>::new();

            // Build the Taffy parent node for the page content area
            let mut page_root_style = taffy::style::Style::default();
            page_root_style.display = taffy::style::Display::Flex;
            page_root_style.flex_direction = taffy::style::FlexDirection::Column;
            page_root_style.size = taffy::geometry::Size {
                width: taffy::style::Dimension::Length(content_width),
                height: taffy::style::Dimension::Length(content_height),
            };

            let mut taffy_children = Vec::new();
            let mut positioned_elements = Vec::new();
            let mut text_measure_registry = HashMap::new();

            for node in nodes {
                // Check if the node has an absolute position. If so, layout absolute.
                if let Some(abs_pos) = get_absolute_position(node) {
                    let elems =
                        layout_absolute_node(node, &abs_pos, 0.0, 0.0, page_width, page_height)?;
                    positioned_elements.extend(elems);
                } else {
                    // Regular in-flow node: add to Taffy tree
                    let taffy_node_id =
                        build_taffy_node(&mut taffy, node, &mut text_measure_registry)?;
                    taffy_children.push((taffy_node_id, node));
                }
            }

            let root_node = taffy
                .new_with_children(
                    page_root_style,
                    &taffy_children.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
                )
                .map_err(|e| {
                    DocforgeError::Render(format!("Failed to create page root taffy node: {:?}", e))
                })?;

            // Compute Taffy layout with measure closure (5 arguments for taffy 0.7.7)
            taffy
                .compute_layout_with_measure(
                    root_node,
                    taffy::geometry::Size {
                        width: taffy::style::AvailableSpace::Definite(content_width),
                        height: taffy::style::AvailableSpace::Definite(content_height),
                    },
                    |known_dimensions, available_space, node_id, _context, _style| {
                        if let Some(text_props) = text_measure_registry.get(&node_id) {
                            let max_w = match known_dimensions.width {
                                Some(w) => w,
                                None => match available_space.width {
                                    taffy::style::AvailableSpace::Definite(w) => w,
                                    _ => content_width, // Default fallback width
                                },
                            };
                            let font_size = text_props.font_size as f32;
                            let line_height =
                                text_props.line_height.unwrap_or(text_props.font_size * 1.2) as f32;

                            let (measured_w, height) = measure_text_size(
                                &text_props.content,
                                &text_props.font_family,
                                font_size,
                                line_height,
                                max_w,
                                self.fonts,
                            );

                            let final_width = match known_dimensions.width {
                                Some(w) => w,
                                None => measured_w,
                            };

                            taffy::geometry::Size {
                                width: final_width,
                                height,
                            }
                        } else {
                            taffy::geometry::Size {
                                width: known_dimensions.width.unwrap_or(0.0),
                                height: known_dimensions.height.unwrap_or(0.0),
                            }
                        }
                    },
                )
                .map_err(|e| {
                    DocforgeError::Render(format!("Taffy layout computation failed: {:?}", e))
                })?;

            // Extract elements from Taffy tree
            for (node_id, evaluated_node) in taffy_children {
                extract_positioned_elements(
                    &taffy,
                    node_id,
                    evaluated_node,
                    left_margin,
                    top_margin,
                    &mut positioned_elements,
                )?;
            }

            pages.push(PageLayout {
                width_pt: page_width,
                height_pt: page_height,
                elements: positioned_elements,
            });
        }

        Ok(LayoutResult { pages })
    }

    /// Layout multiple data records onto physical sheets using multi-up arrangement.
    ///
    /// Each record is rendered using the same template document, then placed into
    /// the appropriate slot position on the physical sheet. When all slots on a
    /// sheet are filled, a new sheet page is created automatically.
    ///
    /// # Arguments
    /// * `doc` - The template document (defines template dimensions and content)
    /// * `contexts` - Data contexts for each record to render
    /// * `multi_up_config` - Multi-up layout configuration (sheet size, gaps, etc.)
    ///
    /// # Returns
    /// A `LayoutResult` where each page represents one physical sheet containing
    /// multiple template instances arranged according to the multi-up config.
    pub fn layout_multi_up(
        &self,
        doc: &TemplateDocument,
        contexts: &[Value],
        multi_up_config: &crate::layout::multi_up::MultiUpConfig,
    ) -> Result<LayoutResult, DocforgeError> {
        use crate::layout::multi_up::{generate_cut_lines, offset_element, paginate_contexts};

        let multi_up = multi_up_config.compute_layout();
        let chunks = paginate_contexts(contexts, multi_up.items_per_sheet);
        let mut pages = Vec::with_capacity(chunks.len());

        for chunk in chunks {
            let mut sheet_elements = Vec::new();

            for (i, ctx) in chunk.iter().enumerate() {
                if i >= multi_up.slots.len() {
                    break; // Safety guard
                }
                let slot = &multi_up.slots[i];

                // Layout the template for this single record
                let single_result = self.layout(doc, ctx)?;

                // Take elements from the first page only (multi-up templates are single-page)
                if let Some(first_page) = single_result.pages.first() {
                    for element in &first_page.elements {
                        sheet_elements.push(offset_element(element, slot.x_pt, slot.y_pt));
                    }
                }
            }

            // Add cut guide lines if enabled
            let cut_lines = generate_cut_lines(&multi_up, multi_up_config);
            sheet_elements.extend(cut_lines);

            pages.push(PageLayout {
                width_pt: multi_up.sheet_width_pt,
                height_pt: multi_up.sheet_height_pt,
                elements: sheet_elements,
            });
        }

        // Ensure at least one empty page if no contexts provided
        if pages.is_empty() {
            pages.push(PageLayout {
                width_pt: multi_up.sheet_width_pt,
                height_pt: multi_up.sheet_height_pt,
                elements: Vec::new(),
            });
        }

        Ok(LayoutResult { pages })
    }
}

// ── Absolute Position Layout ──────────────────────────────────

fn get_absolute_position(node: &EvaluatedNode) -> Option<crate::model::document::Position> {
    match node {
        EvaluatedNode::Frame(n) => n.props.position.clone(),
        EvaluatedNode::Text(n) => n.props.position.clone(),
        EvaluatedNode::Image(n) => n.props.position.clone(),
        EvaluatedNode::Shape(n) => n.props.position.clone(),
        EvaluatedNode::Table(n) => n.position.clone(),
        EvaluatedNode::Barcode(n) => n.props.position.clone(),
        EvaluatedNode::QRCode(n) => n.props.position.clone(),
        EvaluatedNode::Separator(n) => n.props.position.clone(),
        _ => None,
    }
}

fn parse_json_dimension(val: &Value, parent_size: f32) -> f32 {
    match val {
        Value::Number(n) => mm_to_pt(n.as_f64().unwrap_or(0.0)),
        Value::String(s) => {
            let s_trimmed = s.trim();
            if s_trimmed.ends_with('%') {
                if let Ok(pct) = s_trimmed[..s_trimmed.len() - 1].parse::<f32>() {
                    parent_size * (pct / 100.0)
                } else {
                    0.0
                }
            } else if s_trimmed.ends_with("px") {
                if let Ok(px) = s_trimmed[..s_trimmed.len() - 2].parse::<f32>() {
                    px * PX_TO_PT
                } else {
                    0.0
                }
            } else if s_trimmed.ends_with("mm") {
                if let Ok(mm) = s_trimmed[..s_trimmed.len() - 2].parse::<f64>() {
                    mm_to_pt(mm)
                } else {
                    0.0
                }
            } else if let Ok(v) = s_trimmed.parse::<f64>() {
                mm_to_pt(v)
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

fn layout_absolute_node(
    node: &EvaluatedNode,
    pos: &crate::model::document::Position,
    margin_left: f32,
    margin_top: f32,
    parent_w: f32,
    parent_h: f32,
) -> Result<Vec<PositionedElement>, DocforgeError> {
    let x = mm_to_pt(pos.x) + margin_left;
    let y = mm_to_pt(pos.y) + margin_top;
    let w = parse_json_dimension(&pos.width, parent_w);
    let h = pos
        .height
        .as_ref()
        .map(|val| parse_json_dimension(val, parent_h))
        .unwrap_or(0.0);

    match node {
        EvaluatedNode::Text(n) => {
            let weight = FontWeight::from_str_opt(n.props.font_weight.as_deref());
            let align = TextAlign::from_str_opt(n.props.text_align.as_deref());
            let dir = TextDirection::from_str_opt(n.props.direction.as_deref());
            let col = Color::from_hex(n.props.color.as_deref().unwrap_or("#000000"))
                .unwrap_or(Color::BLACK);

            Ok(vec![PositionedElement::Text(TextElement {
                x,
                y,
                width: w,
                height: h,
                content: n.props.content.clone(),
                font_family: n.props.font_family.clone(),
                font_size: n.props.font_size as f32,
                font_weight: weight,
                font_style: FontStyle::Normal,
                color: col,
                text_align: align,
                direction: dir,
                line_height: n.props.line_height.unwrap_or(n.props.font_size * 1.2) as f32,
            })])
        }
        EvaluatedNode::Image(n) => {
            let (mime, data) = match n.props.src.as_deref() {
                Some(src) => crate::render::image_loader::decode_data_uri(src)?
                    .unwrap_or_else(|| ("image/png".to_string(), Vec::new())),
                None => ("image/png".to_string(), Vec::new()),
            };
            Ok(vec![PositionedElement::Image(ImageElement {
                x,
                y,
                width: w,
                height: h,
                data,
                mime_type: mime,
            })])
        }
        EvaluatedNode::Shape(n) => {
            let fill = n.props.fill.as_deref().and_then(Color::from_hex);
            let stroke = n.props.stroke.as_deref().and_then(Color::from_hex);
            Ok(vec![PositionedElement::Rect(RectElement {
                x,
                y,
                width: w,
                height: h,
                fill,
                stroke,
                stroke_width: n.props.stroke_width.unwrap_or(0.0) as f32,
                border_radius: 0.0,
            })])
        }
        EvaluatedNode::Table(t) => layout_table_elements(t, x, y, w, h),
        EvaluatedNode::Barcode(n) => {
            let format = crate::render::barcode_gen::BarcodeFormat::from_str(&n.props.format)
                .unwrap_or(crate::render::barcode_gen::BarcodeFormat::Code128);
            let svg_content =
                crate::render::barcode_gen::generate_barcode_svg(format, &n.props.data, w, h)?;
            Ok(vec![PositionedElement::BarcodeSvg(SvgElement {
                x,
                y,
                width: w,
                height: h,
                svg_content,
            })])
        }
        EvaluatedNode::QRCode(n) => {
            let correction =
                crate::render::barcode_gen::QrErrorCorrection::from_str(&n.props.error_correction);
            let svg_content = crate::render::barcode_gen::generate_qr_svg(
                &n.props.data,
                w,
                correction,
                "#000000",
                "#ffffff",
            )?;
            Ok(vec![PositionedElement::QrCodeSvg(SvgElement {
                x,
                y,
                width: w,
                height: h,
                svg_content,
            })])
        }
        EvaluatedNode::Separator(n) => {
            let col = Color::from_hex(n.props.color.as_deref().unwrap_or("#cccccc"))
                .unwrap_or(Color::new(204, 204, 204));
            let x2 = if n.props.direction == "horizontal" {
                x + w
            } else {
                x
            };
            let y2 = if n.props.direction == "horizontal" {
                y
            } else {
                y + h
            };

            Ok(vec![PositionedElement::Line(LineElement {
                x1: x,
                y1: y,
                x2,
                y2,
                color: col,
                width: n.props.thickness.unwrap_or(1.0) as f32,
            })])
        }
        _ => Err(DocforgeError::Render(
            "Unsupported node type for absolute layout".to_string(),
        )),
    }
}

/// Computes positioned elements for a table node.
pub fn layout_table_elements(
    table: &EvaluatedTable,
    x: f32,
    y: f32,
    table_w: f32,
    _table_h: f32,
) -> Result<Vec<PositionedElement>, DocforgeError> {
    let cols = &table.props.columns;
    if cols.is_empty() {
        return Ok(Vec::new());
    }

    let mut elements = Vec::new();
    let num_cols = cols.len();

    // 1. Resolve target table width
    let target_w = if table_w > 0.0 { table_w } else { 500.0 };

    // 2. Compute individual column widths
    let mut col_widths = Vec::with_capacity(num_cols);
    let mut auto_count = 0usize;
    let mut fixed_sum = 0.0f32;

    for col in cols {
        if let Some(ref w_str) = col.width {
            let trimmed = w_str.trim();
            if trimmed.ends_with('%') {
                if let Ok(pct) = trimmed[..trimmed.len() - 1].parse::<f32>() {
                    let w = target_w * (pct / 100.0);
                    col_widths.push(Some(w));
                    fixed_sum += w;
                    continue;
                }
            } else if trimmed.ends_with("px") {
                if let Ok(px) = trimmed[..trimmed.len() - 2].parse::<f32>() {
                    let w = px * PX_TO_PT;
                    col_widths.push(Some(w));
                    fixed_sum += w;
                    continue;
                }
            } else if trimmed.ends_with("mm") {
                if let Ok(mm) = trimmed[..trimmed.len() - 2].parse::<f64>() {
                    let w = mm_to_pt(mm);
                    col_widths.push(Some(w));
                    fixed_sum += w;
                    continue;
                }
            } else if trimmed.ends_with("pt") {
                if let Ok(pt) = trimmed[..trimmed.len() - 2].parse::<f32>() {
                    col_widths.push(Some(pt));
                    fixed_sum += pt;
                    continue;
                }
            } else if let Ok(val) = trimmed.parse::<f64>() {
                let w = mm_to_pt(val);
                col_widths.push(Some(w));
                fixed_sum += w;
                continue;
            }
        }
        col_widths.push(None);
        auto_count += 1;
    }

    // Distribute remaining width to auto columns
    let auto_w = if auto_count > 0 {
        ((target_w - fixed_sum).max(0.0)) / (auto_count as f32)
    } else {
        0.0
    };

    let resolved_col_widths: Vec<f32> = col_widths
        .into_iter()
        .map(|w_opt| w_opt.unwrap_or_else(|| auto_w.max(25.0)))
        .collect();

    let actual_table_w: f32 = resolved_col_widths.iter().sum();

    // 3. Direction (RTL vs LTR)
    let is_rtl = table
        .props
        .direction
        .as_deref()
        .or_else(|| {
            table
                .props
                .style
                .as_ref()
                .and_then(|s| s.direction.as_deref())
        })
        .map(|d| d.eq_ignore_ascii_case("rtl"))
        .unwrap_or_else(|| {
            cols.iter().any(|c| {
                c.header
                    .chars()
                    .any(|ch| ('\u{0600}'..='\u{06FF}').contains(&ch))
                    || c.header_ar
                        .as_ref()
                        .map(|ar| ar.chars().any(|ch| ('\u{0600}'..='\u{06FF}').contains(&ch)))
                        .unwrap_or(false)
            })
        });

    let mut col_x = Vec::with_capacity(num_cols);
    if is_rtl {
        let mut accum = 0.0f32;
        for &w in &resolved_col_widths {
            col_x.push(x + actual_table_w - accum - w);
            accum += w;
        }
    } else {
        let mut accum = 0.0f32;
        for &w in &resolved_col_widths {
            col_x.push(x + accum);
            accum += w;
        }
    }

    // 4. Styles
    let style = table.props.style.as_ref();
    let header_bg_col = style
        .and_then(|s| s.header_bg_color.as_deref())
        .and_then(Color::from_hex)
        .or_else(|| Some(Color::new(241, 245, 249))); // subtle slate #f1f5f9
    let header_text_col = style
        .and_then(|s| s.header_text_color.as_deref())
        .and_then(Color::from_hex)
        .unwrap_or(Color::new(30, 41, 59)); // #1e293b
    let header_font_size = style.and_then(|s| s.header_font_size).unwrap_or(9.0) as f32;
    let header_font_weight = style
        .and_then(|s| s.header_font_weight.as_deref())
        .map(|w| FontWeight::from_str_opt(Some(w)))
        .unwrap_or(FontWeight::Bold);
    let row_bg_col = style
        .and_then(|s| s.row_bg_color.as_deref())
        .and_then(Color::from_hex);
    let alt_row_bg_col = style
        .and_then(|s| s.alt_row_bg_color.as_deref())
        .and_then(Color::from_hex);
    let font_size = style.and_then(|s| s.font_size).unwrap_or(8.5) as f32;
    let font_family = style
        .and_then(|s| s.font_family.as_deref())
        .unwrap_or("Amiri")
        .to_string();
    let header_h = style.and_then(|s| s.row_height).unwrap_or(24.0) as f32;
    let row_h = style.and_then(|s| s.row_height).unwrap_or(20.0) as f32;
    let padding = style.and_then(|s| s.padding).unwrap_or(3.5) as f32;
    let border_color = style
        .and_then(|s| s.border_color.as_deref())
        .and_then(Color::from_hex)
        .unwrap_or(Color::new(148, 163, 184)); // slate-400
    let border_width = style.and_then(|s| s.border_width).unwrap_or(0.75) as f32;
    let show_grid = table.props.show_grid;
    let show_border = table.props.show_border;

    let mut current_y = y;

    // 5. Render Header Row
    if table.props.show_header {
        if let Some(bg) = header_bg_col {
            elements.push(PositionedElement::Rect(RectElement {
                x,
                y: current_y,
                width: actual_table_w,
                height: header_h,
                fill: Some(bg),
                stroke: None,
                stroke_width: 0.0,
                border_radius: 0.0,
            }));
        }

        for (i, col) in cols.iter().enumerate() {
            let col_w = resolved_col_widths[i];
            let cell_x = col_x[i];
            let text_content = if is_rtl {
                col.header_ar.as_deref().unwrap_or(&col.header)
            } else {
                &col.header
            };

            let align = TextAlign::from_str_opt(col.alignment.as_deref());
            let text_y = current_y + ((header_h - header_font_size) * 0.35).max(1.0);

            elements.push(PositionedElement::Text(TextElement {
                x: cell_x + padding,
                y: text_y,
                width: (col_w - 2.0 * padding).max(1.0),
                height: header_h,
                content: text_content.to_string(),
                font_family: font_family.clone(),
                font_size: header_font_size,
                font_weight: header_font_weight,
                font_style: FontStyle::Normal,
                color: header_text_col,
                text_align: align,
                direction: if is_rtl {
                    TextDirection::Rtl
                } else {
                    TextDirection::Ltr
                },
                line_height: header_font_size * 1.2,
            }));
        }

        current_y += header_h;
    }

    // 6. Render Data Rows
    let rows = table.props.rows.as_deref().unwrap_or(&[]);
    for (row_idx, row) in rows.iter().enumerate() {
        // Row background
        let bg = if row_idx % 2 == 1 {
            alt_row_bg_col.or(row_bg_col)
        } else {
            row_bg_col
        };

        if let Some(fill) = bg {
            elements.push(PositionedElement::Rect(RectElement {
                x,
                y: current_y,
                width: actual_table_w,
                height: row_h,
                fill: Some(fill),
                stroke: None,
                stroke_width: 0.0,
                border_radius: 0.0,
            }));
        }

        // Cell contents
        for (i, col) in cols.iter().enumerate() {
            let col_w = resolved_col_widths[i];
            let cell_x = col_x[i];
            let raw_val = row.get(&col.field);

            let formatted = match raw_val {
                Some(Value::String(s)) => s.clone(),
                Some(Value::Number(n)) => {
                    if let Some(i) = n.as_i64() {
                        i.to_string()
                    } else if let Some(f) = n.as_f64() {
                        if f.fract() == 0.0 {
                            format!("{:.0}", f)
                        } else {
                            format!("{:.1}", f)
                        }
                    } else {
                        n.to_string()
                    }
                }
                Some(Value::Bool(b)) => b.to_string(),
                Some(Value::Null) | None => {
                    col.default_value.clone().unwrap_or_else(|| "-".to_string())
                }
                Some(other) => other.to_string(),
            };

            let align = if let Some(ref a) = col.alignment {
                TextAlign::from_str_opt(Some(a.as_str()))
            } else if raw_val.map(|v| v.is_number()).unwrap_or(false) {
                TextAlign::Center
            } else if is_rtl {
                TextAlign::End
            } else {
                TextAlign::Start
            };

            let text_y = current_y + ((row_h - font_size) * 0.35).max(1.0);

            elements.push(PositionedElement::Text(TextElement {
                x: cell_x + padding,
                y: text_y,
                width: (col_w - 2.0 * padding).max(1.0),
                height: row_h,
                content: formatted,
                font_family: font_family.clone(),
                font_size,
                font_weight: FontWeight::Normal,
                font_style: FontStyle::Normal,
                color: Color::BLACK,
                text_align: align,
                direction: if is_rtl {
                    TextDirection::Rtl
                } else {
                    TextDirection::Ltr
                },
                line_height: font_size * 1.2,
            }));
        }

        current_y += row_h;
    }

    // 7. Render Totals Row
    if table.props.show_totals && !rows.is_empty() {
        let totals_h = row_h;
        let totals_bg = Color::new(241, 245, 249);

        elements.push(PositionedElement::Rect(RectElement {
            x,
            y: current_y,
            width: actual_table_w,
            height: totals_h,
            fill: Some(totals_bg),
            stroke: None,
            stroke_width: 0.0,
            border_radius: 0.0,
        }));

        let mut label_placed = false;

        for (i, col) in cols.iter().enumerate() {
            let col_w = resolved_col_widths[i];
            let cell_x = col_x[i];

            let cell_text = if let Some(ref total_type) = col.total_type {
                match total_type.to_lowercase().as_str() {
                    "sum" => {
                        let sum: f64 = rows
                            .iter()
                            .filter_map(|r| r.get(&col.field).and_then(|v| v.as_f64()))
                            .sum();
                        if sum.fract() == 0.0 {
                            format!("{:.0}", sum)
                        } else {
                            format!("{:.1}", sum)
                        }
                    }
                    "avg" => {
                        let nums: Vec<f64> = rows
                            .iter()
                            .filter_map(|r| r.get(&col.field).and_then(|v| v.as_f64()))
                            .collect();
                        if !nums.is_empty() {
                            let avg = nums.iter().sum::<f64>() / nums.len() as f64;
                            format!("{:.1}", avg)
                        } else {
                            "-".to_string()
                        }
                    }
                    "count" => {
                        let cnt = rows
                            .iter()
                            .filter(|r| r.get(&col.field).map(|v| !v.is_null()).unwrap_or(false))
                            .count();
                        cnt.to_string()
                    }
                    "label" => {
                        label_placed = true;
                        if is_rtl {
                            "المجموع".to_string()
                        } else {
                            "Total".to_string()
                        }
                    }
                    _ => String::new(),
                }
            } else if !label_placed && i == 0 {
                label_placed = true;
                if is_rtl {
                    "المجموع العام".to_string()
                } else {
                    "Total".to_string()
                }
            } else {
                String::new()
            };

            if !cell_text.is_empty() {
                let align = TextAlign::from_str_opt(col.alignment.as_deref());
                let text_y = current_y + ((totals_h - font_size) * 0.35).max(1.0);

                elements.push(PositionedElement::Text(TextElement {
                    x: cell_x + padding,
                    y: text_y,
                    width: (col_w - 2.0 * padding).max(1.0),
                    height: totals_h,
                    content: cell_text,
                    font_family: font_family.clone(),
                    font_size,
                    font_weight: FontWeight::Bold,
                    font_style: FontStyle::Normal,
                    color: Color::new(15, 23, 42),
                    text_align: align,
                    direction: if is_rtl {
                        TextDirection::Rtl
                    } else {
                        TextDirection::Ltr
                    },
                    line_height: font_size * 1.2,
                }));
            }
        }

        current_y += totals_h;
    }

    let total_table_h = current_y - y;

    // 8. Grid Lines
    if show_grid {
        let mut row_cursor_y = y;
        if table.props.show_header {
            row_cursor_y += header_h;
            elements.push(PositionedElement::Line(LineElement {
                x1: x,
                y1: row_cursor_y,
                x2: x + actual_table_w,
                y2: row_cursor_y,
                color: border_color,
                width: border_width,
            }));
        }

        for _ in 0..rows.len() {
            row_cursor_y += row_h;
            elements.push(PositionedElement::Line(LineElement {
                x1: x,
                y1: row_cursor_y,
                x2: x + actual_table_w,
                y2: row_cursor_y,
                color: border_color,
                width: border_width * 0.8,
            }));
        }

        // Vertical divider lines between columns
        for i in 1..num_cols {
            let div_x = if is_rtl { col_x[i - 1] } else { col_x[i] };
            elements.push(PositionedElement::Line(LineElement {
                x1: div_x,
                y1: y,
                x2: div_x,
                y2: current_y,
                color: border_color,
                width: border_width * 0.8,
            }));
        }
    }

    // 9. Outer Border
    if show_border {
        elements.push(PositionedElement::Rect(RectElement {
            x,
            y,
            width: actual_table_w,
            height: total_table_h,
            fill: None,
            stroke: Some(border_color),
            stroke_width: border_width,
            border_radius: 0.0,
        }));
    }

    Ok(elements)
}

// ── Taffy Tree Layout Builder ─────────────────────────────────

fn build_taffy_node(
    taffy: &mut taffy::TaffyTree<()>,
    node: &EvaluatedNode,
    text_measure_registry: &mut HashMap<taffy::prelude::NodeId, TextProps>,
) -> Result<taffy::prelude::NodeId, DocforgeError> {
    let mut style = taffy::style::Style::default();
    style.display = taffy::style::Display::Flex;

    match node {
        EvaluatedNode::Text(n) => {
            style.size.width = taffy::style::Dimension::Auto;
            style.size.height = taffy::style::Dimension::Auto;

            let taffy_node_id = taffy.new_leaf(style).map_err(|e| {
                DocforgeError::Render(format!("Failed to create taffy text node: {:?}", e))
            })?;

            text_measure_registry.insert(taffy_node_id, n.props.clone());
            Ok(taffy_node_id)
        }
        EvaluatedNode::Frame(n) => {
            if n.props.layout == "flex" {
                style.display = taffy::style::Display::Flex;
                style.flex_direction = taffy::style::FlexDirection::Column;
            } else {
                style.display = taffy::style::Display::Flex;
            }

            if let Some(gap) = n.props.gap {
                style.gap = taffy::geometry::Size {
                    width: taffy::style::LengthPercentage::Length(mm_to_pt(gap)),
                    height: taffy::style::LengthPercentage::Length(mm_to_pt(gap)),
                };
            }

            let mut children_ids = Vec::new();
            for child in &n.children {
                // If child is absolutely positioned, Taffy doesn't layout it in flow
                if get_absolute_position(child).is_none() {
                    children_ids.push(build_taffy_node(taffy, child, text_measure_registry)?);
                }
            }

            taffy.new_with_children(style, &children_ids).map_err(|e| {
                DocforgeError::Render(format!("Failed to create taffy frame node: {:?}", e))
            })
        }
        EvaluatedNode::Spacer(n) => {
            style.size.width = n
                .props
                .width
                .map(|w| taffy::style::Dimension::Length(mm_to_pt(w)))
                .unwrap_or(taffy::style::Dimension::Auto);
            style.size.height = n
                .props
                .height
                .map(|h| taffy::style::Dimension::Length(mm_to_pt(h)))
                .unwrap_or(taffy::style::Dimension::Auto);
            taffy.new_leaf(style).map_err(|e| {
                DocforgeError::Render(format!("Failed to create taffy spacer node: {:?}", e))
            })
        }
        EvaluatedNode::Separator(n) => {
            let thickness = n.props.thickness.unwrap_or(1.0);
            if n.props.direction == "horizontal" {
                style.size.width = taffy::style::Dimension::Percent(1.0);
                style.size.height = taffy::style::Dimension::Length(thickness as f32);
            } else {
                style.size.width = taffy::style::Dimension::Length(thickness as f32);
                style.size.height = taffy::style::Dimension::Percent(1.0);
            }
            taffy.new_leaf(style).map_err(|e| {
                DocforgeError::Render(format!("Failed to create taffy separator node: {:?}", e))
            })
        }
        EvaluatedNode::Table(t) => {
            style.size.width = taffy::style::Dimension::Percent(1.0);
            let row_h = t
                .props
                .style
                .as_ref()
                .and_then(|s| s.row_height)
                .unwrap_or(20.0);
            let header_h = if t.props.show_header {
                t.props
                    .style
                    .as_ref()
                    .and_then(|s| s.row_height)
                    .unwrap_or(24.0)
            } else {
                0.0
            };
            let rows_count = t.props.rows.as_ref().map(|r| r.len()).unwrap_or(0) as f64;
            let totals_h = if t.props.show_totals { row_h } else { 0.0 };
            let total_h = header_h + (rows_count * row_h) + totals_h;
            style.size.height = taffy::style::Dimension::Length(total_h as f32);

            taffy.new_leaf(style).map_err(|e| {
                DocforgeError::Render(format!("Failed to create taffy table node: {:?}", e))
            })
        }
        _ => {
            // Fallback default leaf node
            taffy.new_leaf(style).map_err(|e| {
                DocforgeError::Render(format!("Failed to create taffy leaf node: {:?}", e))
            })
        }
    }
}

#[allow(dead_code)]
fn measure_text_height(
    content: &str,
    family: &str,
    font_size: f32,
    line_height: f32,
    max_width: f32,
    fonts: &FontCache,
) -> f32 {
    let (mut font_system, shaping_family) = if let Some(cached_font) = fonts.get_font(
        family,
        400,
        crate::render::font_cache::FontStyleKind::Normal,
    ) {
        create_font_system_for_font(cached_font)
    } else {
        (cosmic_text::FontSystem::new(), family.to_string())
    };

    let attrs = cosmic_text::Attrs::new().family(cosmic_text::Family::Name(&shaping_family));
    let metrics = cosmic_text::Metrics::new(font_size, line_height);
    let mut buffer = cosmic_text::Buffer::new(&mut font_system, metrics);

    buffer.set_size(&mut font_system, Some(max_width), None);
    buffer.set_text(
        &mut font_system,
        content,
        attrs,
        cosmic_text::Shaping::Advanced,
    );
    buffer.shape_until_scroll(&mut font_system, false);

    // Sum layout runs
    let mut height = 0.0;
    for run in buffer.layout_runs() {
        height = run.line_y + line_height;
    }
    if height == 0.0 {
        line_height
    } else {
        height
    }
}

fn measure_text_size(
    content: &str,
    family: &str,
    font_size: f32,
    line_height: f32,
    max_width: f32,
    fonts: &FontCache,
) -> (f32, f32) {
    let (mut font_system, shaping_family) = if let Some(cached_font) = fonts.get_font(
        family,
        400,
        crate::render::font_cache::FontStyleKind::Normal,
    ) {
        create_font_system_for_font(cached_font)
    } else {
        (cosmic_text::FontSystem::new(), family.to_string())
    };

    let attrs = cosmic_text::Attrs::new().family(cosmic_text::Family::Name(&shaping_family));
    let metrics = cosmic_text::Metrics::new(font_size, line_height);
    let mut buffer = cosmic_text::Buffer::new(&mut font_system, metrics);

    buffer.set_size(&mut font_system, Some(max_width), None);
    buffer.set_text(
        &mut font_system,
        content,
        attrs,
        cosmic_text::Shaping::Advanced,
    );
    buffer.shape_until_scroll(&mut font_system, false);

    let mut max_run_width: f32 = 0.0;
    let mut height = 0.0;

    for run in buffer.layout_runs() {
        height = run.line_y + line_height;
        let mut run_w = 0.0;
        for glyph in run.glyphs {
            let right_edge = glyph.x + glyph.w;
            if right_edge > run_w {
                run_w = right_edge;
            }
        }
        if run_w > max_run_width {
            max_run_width = run_w;
        }
    }

    let final_height = if height == 0.0 { line_height } else { height };
    (max_run_width, final_height)
}

// ── Extract Positioned Elements ────────────────────────────────

fn extract_positioned_elements(
    taffy: &taffy::TaffyTree<()>,
    node_id: taffy::prelude::NodeId,
    evaluated_node: &EvaluatedNode,
    accum_x: f32,
    accum_y: f32,
    elements: &mut Vec<PositionedElement>,
) -> Result<(), DocforgeError> {
    let layout = taffy
        .layout(node_id)
        .map_err(|e| DocforgeError::Render(format!("Failed to get layout for node: {:?}", e)))?;

    let abs_x = accum_x + layout.location.x;
    let abs_y = accum_y + layout.location.y;
    let w = layout.size.width;
    let h = layout.size.height;

    match evaluated_node {
        EvaluatedNode::Text(n) => {
            let weight = FontWeight::from_str_opt(n.props.font_weight.as_deref());
            let align = TextAlign::from_str_opt(n.props.text_align.as_deref());
            let dir = TextDirection::from_str_opt(n.props.direction.as_deref());
            let col = Color::from_hex(n.props.color.as_deref().unwrap_or("#000000"))
                .unwrap_or(Color::BLACK);

            elements.push(PositionedElement::Text(TextElement {
                x: abs_x,
                y: abs_y,
                width: w,
                height: h,
                content: n.props.content.clone(),
                font_family: n.props.font_family.clone(),
                font_size: n.props.font_size as f32,
                font_weight: weight,
                font_style: FontStyle::Normal,
                color: col,
                text_align: align,
                direction: dir,
                line_height: n.props.line_height.unwrap_or(n.props.font_size * 1.2) as f32,
            }));
        }
        EvaluatedNode::Separator(n) => {
            let col = Color::from_hex(n.props.color.as_deref().unwrap_or("#cccccc"))
                .unwrap_or(Color::new(204, 204, 204));
            let x2 = if n.props.direction == "horizontal" {
                abs_x + w
            } else {
                abs_x
            };
            let y2 = if n.props.direction == "horizontal" {
                abs_y
            } else {
                abs_y + h
            };

            elements.push(PositionedElement::Line(LineElement {
                x1: abs_x,
                y1: abs_y,
                x2,
                y2,
                color: col,
                width: n.props.thickness.unwrap_or(1.0) as f32,
            }));
        }
        EvaluatedNode::Frame(n) => {
            // Recursively extract in-flow children
            let child_nodes = taffy.children(node_id).map_err(|e| {
                DocforgeError::Render(format!("Failed to query Taffy children: {:?}", e))
            })?;

            // Only extract in-flow children (which correspond to non-absolute child nodes)
            let in_flow_child_evals: Vec<&EvaluatedNode> = n
                .children
                .iter()
                .filter(|child| get_absolute_position(child).is_none())
                .collect();

            for (i, &child_node_id) in child_nodes.iter().enumerate() {
                if let Some(eval_child) = in_flow_child_evals.get(i) {
                    extract_positioned_elements(
                        taffy,
                        child_node_id,
                        eval_child,
                        abs_x,
                        abs_y,
                        elements,
                    )?;
                }
            }
        }
        EvaluatedNode::Table(t) => {
            let table_elems = layout_table_elements(t, abs_x, abs_y, w, h)?;
            elements.extend(table_elems);
        }
        _ => {}
    }

    Ok(())
}

// ── Page Size Resolution Helpers ──────────────────────────────

fn get_page_dimensions(setup: &PageSetup) -> (f32, f32) {
    let (w_mm, h_mm) = if let PageSize::Custom = setup.size {
        let (default_width, default_height) = setup.size.dimensions();
        (
            setup.width.unwrap_or(default_width),
            setup.height.unwrap_or(default_height),
        )
    } else {
        setup.size.dimensions()
    };

    let (w_mm, h_mm) = match setup.orientation {
        Orientation::Portrait => (w_mm, h_mm),
        Orientation::Landscape => (h_mm, w_mm),
    };

    (mm_to_pt(w_mm), mm_to_pt(h_mm))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::document::*;
    use crate::model::nodes::*;
    use serde_json::json;

    #[test]
    fn page_dimensions_follow_orientation_for_presets_and_custom_sizes() {
        let landscape_a4 = PageSetup {
            size: PageSize::A4,
            width: None,
            height: None,
            orientation: Orientation::Landscape,
            margins: Spacing {
                top: 0.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            },
            unit: "mm".to_string(),
        };
        let (width, height) = get_page_dimensions(&landscape_a4);
        assert!((width - mm_to_pt(297.0)).abs() < 0.01);
        assert!((height - mm_to_pt(210.0)).abs() < 0.01);

        let landscape_custom = PageSetup {
            size: PageSize::Custom,
            width: Some(100.0),
            height: Some(200.0),
            orientation: Orientation::Landscape,
            margins: Spacing {
                top: 0.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            },
            unit: "mm".to_string(),
        };
        let (width, height) = get_page_dimensions(&landscape_custom);
        assert!((width - mm_to_pt(200.0)).abs() < 0.01);
        assert!((height - mm_to_pt(100.0)).abs() < 0.01);
    }

    #[test]
    fn test_layout_simple_flex() {
        let setup = PageSetup {
            size: PageSize::A4,
            width: None,
            height: None,
            orientation: Orientation::Portrait,
            margins: Spacing {
                top: 10.0,
                bottom: 10.0,
                left: 10.0,
                right: 10.0,
            },
            unit: "mm".to_string(),
        };

        // Create a root page node containing a Frame layout, containing text nodes
        let frame_node = TemplateNode::Frame(FrameNode {
            id: "frame-1".to_string(),
            name: None,
            position: None,
            props: FrameProps {
                position: None,
                layout: "flex".to_string(),
                alignment: Some("start".to_string()),
                gap: Some(5.0), // 5mm gap
            },
            children: vec![
                TemplateNode::Text(TextNode {
                    id: "text-1".to_string(),
                    name: None,
                    position: None,
                    props: TextProps {
                        content: "Hello {{name}}".to_string(),
                        position: None,
                        font_size: 12.0,
                        font_family: "Rubik".to_string(),
                        font_weight: Some("normal".to_string()),
                        color: Some("#ff0000".to_string()),
                        text_align: Some("start".to_string()),
                        direction: Some("ltr".to_string()),
                        line_height: Some(15.0),
                    },
                }),
                TemplateNode::Text(TextNode {
                    id: "text-2".to_string(),
                    name: None,
                    position: None,
                    props: TextProps {
                        content: "Orbis TMS".to_string(),
                        position: None,
                        font_size: 10.0,
                        font_family: "Rubik".to_string(),
                        font_weight: Some("bold".to_string()),
                        color: Some("#00ff00".to_string()),
                        text_align: Some("center".to_string()),
                        direction: Some("ltr".to_string()),
                        line_height: Some(12.0),
                    },
                }),
            ],
        });

        let doc = TemplateDocument {
            version: "1.0".to_string(),
            meta: DocumentMeta {
                id: None,
                name: "Test Doc".to_string(),
                name_ar: None,
                document_type: "CERTIFICATE".to_string(),
                entity_type: "STUDENT".to_string(),
                locale: "en".to_string(),
                page_setup: setup,
            },
            variables: HashMap::new(),
            assets: HashMap::new(),
            root: TemplateNode::Page(PageNode {
                id: "page-root".to_string(),
                name: None,
                children: vec![frame_node],
            }),
        };

        let fonts = FontCache::new("Rubik");
        let engine = LayoutEngine::new(&fonts);
        let context = json!({ "name": "Antigravity" });

        let result = engine
            .layout(&doc, &context)
            .expect("Should successfully layout document");
        assert_eq!(result.pages.len(), 1);
        let page = &result.pages[0];

        // A4 standard width is 210mm (~595pt), height is 297mm (~841pt)
        assert!(page.width_pt > 590.0 && page.width_pt < 600.0);

        // We should have 2 text elements positioned
        let text_elems: Vec<&TextElement> = page
            .elements
            .iter()
            .filter_map(|e| {
                if let PositionedElement::Text(t) = e {
                    Some(t)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(text_elems.len(), 2);
        assert_eq!(text_elems[0].content, "Hello Antigravity");
        assert_eq!(text_elems[1].content, "Orbis TMS");
    }

    #[test]
    fn test_layout_and_render_integration() {
        let setup = PageSetup {
            size: PageSize::A4,
            width: None,
            height: None,
            orientation: Orientation::Portrait,
            margins: Spacing {
                top: 15.0,
                bottom: 15.0,
                left: 15.0,
                right: 15.0,
            },
            unit: "mm".to_string(),
        };

        let font_candidates = [
            ("Tahoma", "C:\\Windows\\Fonts\\tahoma.ttf"),
            (
                "DejaVu Sans",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ),
            ("Arial", "/System/Library/Fonts/Supplemental/Arial.ttf"),
        ];
        let (font_family, font_path) = font_candidates
            .iter()
            .find(|(_, path)| std::path::Path::new(path).exists())
            .copied()
            .expect("A supported system font must be installed for PDF rendering tests");
        let mut fonts = FontCache::new(font_family);
        fonts
            .load_font_file(
                font_family,
                400,
                crate::render::font_cache::FontStyleKind::Normal,
                font_path,
            )
            .expect("Should load the system font successfully");

        // Layout containing flex container with text, shape, spacer, barcode and qr code
        let frame_node = TemplateNode::Frame(FrameNode {
            id: "flex-container".to_string(),
            name: None,
            position: None,
            props: FrameProps {
                position: None,
                layout: "flex".to_string(),
                alignment: Some("start".to_string()),
                gap: Some(8.0), // 8mm gap
            },
            children: vec![
                // 1. Text node with interpolation (Arabic)
                TemplateNode::Text(TextNode {
                    id: "txt-1".to_string(),
                    name: None,
                    position: None,
                    props: TextProps {
                        content: "مرحبا بك يا {{name}} في نظام Orbis TMS v3 الجديد".to_string(),
                        position: None,
                        font_size: 16.0,
                        font_family: font_family.to_string(),
                        font_weight: Some("bold".to_string()),
                        color: Some("#003366".to_string()),
                        text_align: Some("center".to_string()),
                        direction: Some("rtl".to_string()),
                        line_height: Some(22.0),
                    },
                }),
                // 2. Spacer
                TemplateNode::Spacer(SpacerNode {
                    id: "space-1".to_string(),
                    props: SpacerProps {
                        height: Some(5.0), // 5mm vertical space
                        width: None,
                    },
                }),
                // 3. Separator
                TemplateNode::Separator(SeparatorNode {
                    id: "sep-1".to_string(),
                    name: None,
                    position: None,
                    props: SeparatorProps {
                        direction: "horizontal".to_string(),
                        color: Some("#0066cc".to_string()),
                        thickness: Some(2.0),
                        position: None,
                    },
                }),
                // 4. Barcode
                TemplateNode::Barcode(BarcodeNode {
                    id: "barcode-1".to_string(),
                    name: None,
                    position: None,
                    props: BarcodeProps {
                        data: "ORBIS-{{std_id}}".to_string(),
                        format: "Code128".to_string(),
                        position: None,
                        height: 15.0, // 15mm high
                        show_text: Some(true),
                        color: Some("#000000".to_string()),
                    },
                }),
                // 5. QR Code
                TemplateNode::QRCode(QRCodeNode {
                    id: "qr-1".to_string(),
                    name: None,
                    position: None,
                    props: QRCodeProps {
                        data: "https://orbis.edu/student/{{std_id}}".to_string(),
                        position: None,
                        size: 30.0, // 30mm square
                        error_correction: "High".to_string(),
                        color: Some("#000000".to_string()),
                        background_color: Some("#ffffff".to_string()),
                    },
                }),
            ],
        });

        let doc = TemplateDocument {
            version: "1.0".to_string(),
            meta: DocumentMeta {
                id: None,
                name: "Integration Test Doc".to_string(),
                name_ar: None,
                document_type: "CERTIFICATE".to_string(),
                entity_type: "STUDENT".to_string(),
                locale: "ar".to_string(),
                page_setup: setup,
            },
            variables: HashMap::new(),
            assets: HashMap::new(),
            root: TemplateNode::Page(PageNode {
                id: "page-root".to_string(),
                children: vec![frame_node],
                name: None,
            }),
        };

        let context = json!({
            "name": "عبد الرحمن",
            "std_id": "99581-2026"
        });

        let engine = LayoutEngine::new(&fonts);
        let layout_result = engine
            .layout(&doc, &context)
            .expect("Should compute full layout");

        // Assert output PageLayout structure
        assert_eq!(layout_result.pages.len(), 1);
        let page = &layout_result.pages[0];

        // Ensure text elements exist and are resolved
        let has_interpolated_text = page.elements.iter().any(|el| {
            if let PositionedElement::Text(t) = el {
                t.content == "مرحبا بك يا عبد الرحمن في نظام Orbis TMS v3 الجديد"
            } else {
                false
            }
        });
        assert!(has_interpolated_text);

        // Render to PDF
        let pdf_output = crate::render::pdf_renderer::render_to_pdf(
            &layout_result,
            &fonts,
            &crate::render::pdf_renderer::PdfRenderConfig::default(),
        )
        .expect("Should render layout result to PDF");

        assert!(!pdf_output.bytes.is_empty());

        let artifact_dir =
            tempfile::tempdir().expect("Should create a temporary artifact directory");
        let pdf_path = artifact_dir.path().join("arabic_layout_poc_proof.pdf");
        std::fs::write(&pdf_path, &pdf_output.bytes)
            .expect("Should successfully write integrated layout PDF to artifact folder");

        println!(
            "PoC layout PDF successfully written to: {}",
            pdf_path.display()
        );
    }

    #[test]
    fn test_layout_table_node_and_render_pdf() {
        use crate::model::document::Spacing;
        use crate::model::nodes::{
            TableColumnDef, TableNode, TableProps, TablePropsWrapper, TableStyle,
        };

        let setup = PageSetup {
            size: PageSize::A4,
            width: None,
            height: None,
            orientation: Orientation::Portrait,
            margins: Spacing {
                top: 15.0,
                bottom: 15.0,
                left: 15.0,
                right: 15.0,
            },
            unit: "mm".to_string(),
        };

        let font_candidates = [
            ("Tahoma", "C:\\Windows\\Fonts\\tahoma.ttf"),
            (
                "DejaVu Sans",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ),
            ("Arial", "/System/Library/Fonts/Supplemental/Arial.ttf"),
        ];
        let (font_family, font_path) = font_candidates
            .iter()
            .find(|(_, path)| std::path::Path::new(path).exists())
            .copied()
            .expect("A supported system font must be installed for PDF rendering tests");
        let mut fonts = FontCache::new(font_family);
        fonts
            .load_font_file(
                font_family,
                400,
                crate::render::font_cache::FontStyleKind::Normal,
                font_path,
            )
            .expect("Should load the system font successfully");

        let table_props = TableProps {
            columns: vec![
                TableColumnDef {
                    id: "seq".to_string(),
                    header: "م".to_string(),
                    header_ar: Some("م".to_string()),
                    field: "seq".to_string(),
                    width: Some("10%".to_string()),
                    alignment: Some("center".to_string()),
                    default_value: None,
                    format: None,
                    is_visible: true,
                    total_type: Some("label".to_string()),
                },
                TableColumnDef {
                    id: "subject".to_string(),
                    header: "المادة".to_string(),
                    header_ar: Some("المادة".to_string()),
                    field: "subject".to_string(),
                    width: Some("50%".to_string()),
                    alignment: Some("right".to_string()),
                    default_value: None,
                    format: None,
                    is_visible: true,
                    total_type: None,
                },
                TableColumnDef {
                    id: "score".to_string(),
                    header: "الدرجة".to_string(),
                    header_ar: Some("الدرجة".to_string()),
                    field: "score".to_string(),
                    width: Some("40%".to_string()),
                    alignment: Some("center".to_string()),
                    default_value: None,
                    format: None,
                    is_visible: true,
                    total_type: Some("sum".to_string()),
                },
            ],
            data_source: Some("subjects".to_string()),
            rows: None,
            show_header: true,
            show_totals: true,
            show_subtotals: false,
            group_by: None,
            sort_by: None,
            repeating_header: false,
            position: Some(Position {
                x: 15.0,
                y: 20.0,
                width: Value::String("180mm".to_string()),
                height: None,
            }),
            show_grid: true,
            show_border: true,
            direction: Some("rtl".to_string()),
            style: Some(TableStyle {
                border_color: Some("#334155".to_string()),
                border_width: Some(1.0),
                header_bg_color: Some("#e2e8f0".to_string()),
                header_text_color: Some("#0f172a".to_string()),
                header_font_size: Some(10.0),
                header_font_weight: Some("bold".to_string()),
                row_bg_color: Some("#ffffff".to_string()),
                alt_row_bg_color: Some("#f8fafc".to_string()),
                font_size: Some(9.0),
                font_family: Some(font_family.to_string()),
                row_height: Some(22.0),
                padding: Some(4.0),
                direction: Some("rtl".to_string()),
            }),
            conditional_formatting: None,
        };

        let doc = TemplateDocument {
            version: "1.0".to_string(),
            meta: DocumentMeta {
                id: None,
                name: "Academic Certificate Table".to_string(),
                name_ar: None,
                document_type: "CERTIFICATE".to_string(),
                entity_type: "STUDENT".to_string(),
                locale: "ar".to_string(),
                page_setup: setup,
            },
            variables: HashMap::new(),
            assets: HashMap::new(),
            root: TemplateNode::Table(TableNode {
                id: "academic-grades-table".to_string(),
                name: Some("Grades Table".to_string()),
                position: None,
                props: TablePropsWrapper::Typed(table_props),
            }),
        };

        let context = json!({
            "subjects": [
                { "seq": 1, "subject": "القرآن الكريم", "score": 98 },
                { "seq": 2, "subject": "التربية الإسلامية", "score": 94 },
                { "seq": 3, "subject": "اللغة العربية", "score": 91 },
                { "seq": 4, "subject": "الرياضيات", "score": 95 }
            ]
        });

        let engine = LayoutEngine::new(&fonts);
        let layout_result = engine
            .layout(&doc, &context)
            .expect("Should compute table layout");

        assert_eq!(layout_result.pages.len(), 1);
        let page = &layout_result.pages[0];

        // Ensure header text is present
        let has_header_subject = page.elements.iter().any(|el| {
            if let PositionedElement::Text(t) = el {
                t.content == "المادة"
            } else {
                false
            }
        });
        assert!(
            has_header_subject,
            "Table header 'المادة' should be rendered"
        );

        // Ensure row content is present
        let has_row_quran = page.elements.iter().any(|el| {
            if let PositionedElement::Text(t) = el {
                t.content == "القرآن الكريم"
            } else {
                false
            }
        });
        assert!(
            has_row_quran,
            "Table row 'القرآن الكريم' should be rendered"
        );

        // Ensure total sum (98 + 94 + 91 + 95 = 378) is computed and present
        let has_total_sum = page.elements.iter().any(|el| {
            if let PositionedElement::Text(t) = el {
                t.content == "378"
            } else {
                false
            }
        });
        assert!(
            has_total_sum,
            "Table totals sum '378' should be computed and rendered"
        );

        // Ensure grid lines and rects are present
        let lines_count = page
            .elements
            .iter()
            .filter(|el| matches!(el, PositionedElement::Line(_)))
            .count();
        assert!(lines_count > 0, "Grid lines should be drawn");

        let rects_count = page
            .elements
            .iter()
            .filter(|el| matches!(el, PositionedElement::Rect(_)))
            .count();
        assert!(rects_count > 0, "Borders and backgrounds should be drawn");

        // Render to genuine PDF
        let pdf_output = crate::render::pdf_renderer::render_to_pdf(
            &layout_result,
            &fonts,
            &crate::render::pdf_renderer::PdfRenderConfig::default(),
        )
        .expect("Should render table to PDF");

        assert!(
            !pdf_output.bytes.is_empty(),
            "PDF bytes should not be empty"
        );
        assert!(
            pdf_output.bytes.starts_with(b"%PDF-"),
            "Output must be valid PDF"
        );
    }
}
