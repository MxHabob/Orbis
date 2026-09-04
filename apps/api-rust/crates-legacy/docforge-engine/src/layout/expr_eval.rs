// ═══════════════════════════════════════════════════════════════
// docforge-engine — Expression Evaluator for Node Tree
// Pre-evaluates template node tree to generate absolute layout nodes.
// Resolves repeaters, conditionals and template string variables.
// ═══════════════════════════════════════════════════════════════

use serde_json::Value;
use crate::model::nodes::*;
use crate::expr::evaluator::{evaluate_expression, evaluate_condition, resolve_path};

#[derive(Debug, Clone)]
pub enum EvaluatedNode {
    Page(EvaluatedPage),
    Frame(EvaluatedFrame),
    Grid(EvaluatedGrid),
    Text(EvaluatedText),
    Image(EvaluatedImage),
    Shape(EvaluatedShape),
    Table(EvaluatedTable),
    Barcode(EvaluatedBarcode),
    QRCode(EvaluatedQRCode),
    Separator(EvaluatedSeparator),
    Spacer(EvaluatedSpacer),
    PageBreak,
    Header(EvaluatedHeader),
    Footer(EvaluatedFooter),
    Watermark(EvaluatedWatermark),
}

#[derive(Debug, Clone)]
pub struct EvaluatedPage {
    pub id: String,
    pub name: Option<String>,
    pub children: Vec<EvaluatedNode>,
}

#[derive(Debug, Clone)]
pub struct EvaluatedFrame {
    pub id: String,
    pub name: Option<String>,
    pub props: FrameProps,
    pub children: Vec<EvaluatedNode>,
}

#[derive(Debug, Clone)]
pub struct EvaluatedGrid {
    pub id: String,
    pub name: Option<String>,
    pub props: GridProps,
    pub children: Vec<EvaluatedNode>,
}

#[derive(Debug, Clone)]
pub struct EvaluatedText {
    pub id: String,
    pub name: Option<String>,
    pub props: TextProps,
}

#[derive(Debug, Clone)]
pub struct EvaluatedImage {
    pub id: String,
    pub name: Option<String>,
    pub props: ImageProps,
}

#[derive(Debug, Clone)]
pub struct EvaluatedShape {
    pub id: String,
    pub name: Option<String>,
    pub props: ShapeProps,
}

#[derive(Debug, Clone)]
pub struct EvaluatedTable {
    pub id: String,
    pub name: Option<String>,
    pub props: Value, // Complex table props
}

#[derive(Debug, Clone)]
pub struct EvaluatedBarcode {
    pub id: String,
    pub name: Option<String>,
    pub props: BarcodeProps,
}

#[derive(Debug, Clone)]
pub struct EvaluatedQRCode {
    pub id: String,
    pub name: Option<String>,
    pub props: QRCodeProps,
}

#[derive(Debug, Clone)]
pub struct EvaluatedSeparator {
    pub id: String,
    pub name: Option<String>,
    pub props: SeparatorProps,
}

#[derive(Debug, Clone)]
pub struct EvaluatedSpacer {
    pub id: String,
    pub props: SpacerProps,
}

#[derive(Debug, Clone)]
pub struct EvaluatedHeader {
    pub id: String,
    pub name: Option<String>,
    pub props: HeaderProps,
    pub children: Vec<EvaluatedNode>,
}

#[derive(Debug, Clone)]
pub struct EvaluatedFooter {
    pub id: String,
    pub name: Option<String>,
    pub props: FooterProps,
    pub children: Vec<EvaluatedNode>,
}

#[derive(Debug, Clone)]
pub struct EvaluatedWatermark {
    pub id: String,
    pub props: WatermarkProps,
}

/// Evaluates a template node tree against a JSON data context to produce a resolved layout node tree.
pub fn evaluate_node_tree(node: &TemplateNode, context: &Value) -> Vec<EvaluatedNode> {
    match node {
        TemplateNode::Page(n) => {
            let mut children = Vec::new();
            for child in &n.children {
                children.extend(evaluate_node_tree(child, context));
            }
            vec![EvaluatedNode::Page(EvaluatedPage {
                id: n.id.clone(),
                name: n.name.clone(),
                children,
            })]
        }
        TemplateNode::Frame(n) => {
            let mut children = Vec::new();
            for child in &n.children {
                children.extend(evaluate_node_tree(child, context));
            }
            vec![EvaluatedNode::Frame(EvaluatedFrame {
                id: n.id.clone(),
                name: n.name.clone(),
                props: n.props.clone(),
                children,
            })]
        }
        TemplateNode::Grid(n) => {
            let mut children = Vec::new();
            for child in &n.children {
                children.extend(evaluate_node_tree(child, context));
            }
            vec![EvaluatedNode::Grid(EvaluatedGrid {
                id: n.id.clone(),
                name: n.name.clone(),
                props: n.props.clone(),
                children,
            })]
        }
        TemplateNode::Text(n) => {
            let mut props = n.props.clone();
            props.content = evaluate_expression(&props.content, context);
            vec![EvaluatedNode::Text(EvaluatedText {
                id: n.id.clone(),
                name: n.name.clone(),
                props,
            })]
        }
        TemplateNode::Image(n) => {
            let mut props = n.props.clone();
            if let Some(ref src) = props.src {
                props.src = Some(evaluate_expression(src, context));
            }
            vec![EvaluatedNode::Image(EvaluatedImage {
                id: n.id.clone(),
                name: n.name.clone(),
                props,
            })]
        }
        TemplateNode::Shape(n) => {
            vec![EvaluatedNode::Shape(EvaluatedShape {
                id: n.id.clone(),
                name: n.name.clone(),
                props: n.props.clone(),
            })]
        }
        TemplateNode::Table(n) => {
            // Table structure is evaluated inside layout engine or passed through
            vec![EvaluatedNode::Table(EvaluatedTable {
                id: n.id.clone(),
                name: n.name.clone(),
                props: serde_json::to_value(&n.props).unwrap_or(serde_json::Value::Null),
            })]
        }
        TemplateNode::Barcode(n) => {
            let mut props = n.props.clone();
            props.data = evaluate_expression(&props.data, context);
            vec![EvaluatedNode::Barcode(EvaluatedBarcode {
                id: n.id.clone(),
                name: n.name.clone(),
                props,
            })]
        }
        TemplateNode::QRCode(n) => {
            let mut props = n.props.clone();
            props.data = evaluate_expression(&props.data, context);
            vec![EvaluatedNode::QRCode(EvaluatedQRCode {
                id: n.id.clone(),
                name: n.name.clone(),
                props,
            })]
        }
        TemplateNode::Separator(n) => {
            vec![EvaluatedNode::Separator(EvaluatedSeparator {
                id: n.id.clone(),
                name: n.name.clone(),
                props: n.props.clone(),
            })]
        }
        TemplateNode::Spacer(n) => {
            vec![EvaluatedNode::Spacer(EvaluatedSpacer {
                id: n.id.clone(),
                props: n.props.clone(),
            })]
        }
        TemplateNode::PageBreak(_) => {
            vec![EvaluatedNode::PageBreak]
        }
        TemplateNode::Header(n) => {
            let mut children = Vec::new();
            for child in &n.children {
                children.extend(evaluate_node_tree(child, context));
            }
            vec![EvaluatedNode::Header(EvaluatedHeader {
                id: n.id.clone(),
                name: n.name.clone(),
                props: n.props.clone(),
                children,
            })]
        }
        TemplateNode::Footer(n) => {
            let mut children = Vec::new();
            for child in &n.children {
                children.extend(evaluate_node_tree(child, context));
            }
            vec![EvaluatedNode::Footer(EvaluatedFooter {
                id: n.id.clone(),
                name: n.name.clone(),
                props: n.props.clone(),
                children,
            })]
        }
        TemplateNode::Watermark(n) => {
            let mut props = n.props.clone();
            if let Some(ref text) = props.text {
                props.text = Some(evaluate_expression(text, context));
            }
            vec![EvaluatedNode::Watermark(EvaluatedWatermark {
                id: n.id.clone(),
                props,
            })]
        }
        TemplateNode::Conditional(n) => {
            let cond_true = evaluate_condition(&n.props.condition, context);
            if cond_true {
                let mut results = Vec::new();
                for child in &n.children {
                    results.extend(evaluate_node_tree(child, context));
                }
                results
            } else if let Some(ref else_children) = n.else_children {
                let mut results = Vec::new();
                for child in else_children {
                    results.extend(evaluate_node_tree(child, context));
                }
                results
            } else {
                Vec::new()
            }
        }
        TemplateNode::Repeater(n) => {
            let mut results = Vec::new();
            let data_val = resolve_path(context, &n.props.data_source).unwrap_or(Value::Null);
            
            if let Value::Array(arr) = data_val {
                for (idx, item) in arr.iter().enumerate() {
                    // Create scoped context
                    let mut scoped_ctx = context.clone();
                    if let Value::Object(ref mut map) = scoped_ctx {
                        map.insert(n.props.item_variable.clone(), item.clone());
                        if let Some(ref idx_var) = n.props.index_variable {
                            map.insert(idx_var.clone(), Value::Number(idx.into()));
                        }
                    }
                    
                    // Evaluate children in this scoped context
                    for child in &n.children {
                        results.extend(evaluate_node_tree(child, &scoped_ctx));
                    }
                }
            }
            results
        }
        TemplateNode::ComponentRef(n) => {
            // Components resolution would ideally search template registry.
            // For PoC / basic engine we log a warning or pass through empty
            tracing::warn!("Component references not fully supported in standalone PoC layout engine (id: {})", n.props.component_id);
            Vec::new()
        }
    }
}
