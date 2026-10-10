// ═══════════════════════════════════════════════════════════════
// docforge-engine — Layout Engine
// Computes absolute positions using Taffy and cosmic-text.
// Outputs a fully positioned LayoutResult.
// ═══════════════════════════════════════════════════════════════

use crate::error::DocforgeError;
use crate::layout::expr_eval::{evaluate_node_tree, EvaluatedNode};
use crate::model::document::{Orientation, PageSetup, PageSize, TemplateDocument};
use crate::model::nodes::TextProps;
use crate::render::font_cache::FontCache;
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
                    let elem =
                        layout_absolute_node(node, &abs_pos, 0.0, 0.0, page_width, page_height)?;
                    positioned_elements.push(elem);
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
) -> Result<PositionedElement, DocforgeError> {
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

            Ok(PositionedElement::Text(TextElement {
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
            }))
        }
        EvaluatedNode::Image(n) => {
            let (mime, data) = match n.props.src.as_deref() {
                Some(src) => crate::render::image_loader::decode_data_uri(src)?
                    .unwrap_or_else(|| ("image/png".to_string(), Vec::new())),
                None => ("image/png".to_string(), Vec::new()),
            };
            Ok(PositionedElement::Image(ImageElement {
                x,
                y,
                width: w,
                height: h,
                data,
                mime_type: mime,
            }))
        }
        EvaluatedNode::Shape(n) => {
            let fill = n.props.fill.as_deref().and_then(Color::from_hex);
            let stroke = n.props.stroke.as_deref().and_then(Color::from_hex);
            Ok(PositionedElement::Rect(RectElement {
                x,
                y,
                width: w,
                height: h,
                fill,
                stroke,
                stroke_width: n.props.stroke_width.unwrap_or(0.0) as f32,
                border_radius: 0.0,
            }))
        }
        EvaluatedNode::Barcode(n) => {
            let format = crate::render::barcode_gen::BarcodeFormat::from_str(&n.props.format)
                .unwrap_or(crate::render::barcode_gen::BarcodeFormat::Code128);
            let svg_content =
                crate::render::barcode_gen::generate_barcode_svg(format, &n.props.data, w, h)?;
            Ok(PositionedElement::BarcodeSvg(SvgElement {
                x,
                y,
                width: w,
                height: h,
                svg_content,
            }))
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
            Ok(PositionedElement::QrCodeSvg(SvgElement {
                x,
                y,
                width: w,
                height: h,
                svg_content,
            }))
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

            Ok(PositionedElement::Line(LineElement {
                x1: x,
                y1: y,
                x2,
                y2,
                color: col,
                width: n.props.thickness.unwrap_or(1.0) as f32,
            }))
        }
        _ => Err(DocforgeError::Render(
            "Unsupported node type for absolute layout".to_string(),
        )),
    }
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
    let mut font_system = cosmic_text::FontSystem::new();
    // Load font data if cached
    if let Some(cached_font) = fonts.get_font(
        family,
        400,
        crate::render::font_cache::FontStyleKind::Normal,
    ) {
        font_system
            .db_mut()
            .load_font_data(cached_font.data.as_ref().clone());
    }

    let attrs = cosmic_text::Attrs::new().family(cosmic_text::Family::Name(family));
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
    let mut font_system = cosmic_text::FontSystem::new();
    if let Some(cached_font) = fonts.get_font(
        family,
        400,
        crate::render::font_cache::FontStyleKind::Normal,
    ) {
        font_system
            .db_mut()
            .load_font_data(cached_font.data.as_ref().clone());
    }

    let attrs = cosmic_text::Attrs::new().family(cosmic_text::Family::Name(family));
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
        _ => {}
    }

    Ok(())
}

// ── Page Size Resolution Helpers ──────────────────────────────

fn get_page_dimensions(setup: &PageSetup) -> (f32, f32) {
    let (w_mm, h_mm) = match setup.orientation {
        Orientation::Portrait => setup.size.dimensions(),
        Orientation::Landscape => {
            let (w, h) = setup.size.dimensions();
            (h, w)
        }
    };

    // Check custom size dimensions if preset is Custom
    let w_mm_final = if let PageSize::Custom = setup.size {
        setup.width.unwrap_or(w_mm)
    } else {
        w_mm
    };
    let h_mm_final = if let PageSize::Custom = setup.size {
        setup.height.unwrap_or(h_mm)
    } else {
        h_mm
    };

    (mm_to_pt(w_mm_final), mm_to_pt(h_mm_final))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::document::*;
    use crate::model::nodes::*;
    use serde_json::json;

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
}
