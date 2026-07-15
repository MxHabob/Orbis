use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use shared_kernel::errors::AppError;

/// Defines the Abstract Syntax Tree (AST) for calculation policies.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AstNode {
    /// A literal numeric value
    Literal { value: f64 },
    /// A variable representing a component score
    Variable { name: String },
    /// Addition of multiple nodes
    Add { operands: Vec<AstNode> },
    /// Subtraction of two nodes
    Subtract { left: Box<AstNode>, right: Box<AstNode> },
    /// Multiplication of multiple nodes
    Multiply { operands: Vec<AstNode> },
    /// Division of two nodes
    Divide { left: Box<AstNode>, right: Box<AstNode> },
    /// Maximum value among operands
    Max { operands: Vec<AstNode> },
    /// Minimum value among operands
    Min { operands: Vec<AstNode> },
    /// Average value among operands
    Average { operands: Vec<AstNode> },
    /// Weighted sum of nodes. Each item is (node, weight)
    WeightedSum { items: Vec<WeightedItem> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeightedItem {
    pub node: AstNode,
    pub weight: f64,
}

pub struct CalcEngine {
    pub context: HashMap<String, f64>,
}

impl CalcEngine {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.context.insert(name.to_string(), value);
    }

    pub fn evaluate(&self, node: &AstNode) -> Result<f64, AppError> {
        match node {
            AstNode::Literal { value } => Ok(*value),
            AstNode::Variable { name } => {
                self.context.get(name).copied().ok_or_else(|| {
                    AppError::validation(format!("Variable '{}' not found in evaluation context", name))
                })
            }
            AstNode::Add { operands } => {
                let mut sum = 0.0;
                for op in operands {
                    sum += self.evaluate(op)?;
                }
                Ok(sum)
            }
            AstNode::Subtract { left, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                Ok(l - r)
            }
            AstNode::Multiply { operands } => {
                let mut prod = 1.0;
                for op in operands {
                    prod *= self.evaluate(op)?;
                }
                Ok(prod)
            }
            AstNode::Divide { left, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                if r == 0.0 {
                    return Err(AppError::validation("Division by zero in calculation formula"));
                }
                Ok(l / r)
            }
            AstNode::Max { operands } => {
                let mut max_val = f64::NEG_INFINITY;
                for op in operands {
                    let val = self.evaluate(op)?;
                    if val > max_val {
                        max_val = val;
                    }
                }
                if max_val == f64::NEG_INFINITY { max_val = 0.0; }
                Ok(max_val)
            }
            AstNode::Min { operands } => {
                let mut min_val = f64::INFINITY;
                for op in operands {
                    let val = self.evaluate(op)?;
                    if val < min_val {
                        min_val = val;
                    }
                }
                if min_val == f64::INFINITY { min_val = 0.0; }
                Ok(min_val)
            }
            AstNode::Average { operands } => {
                if operands.is_empty() {
                    return Ok(0.0);
                }
                let mut sum = 0.0;
                for op in operands {
                    sum += self.evaluate(op)?;
                }
                Ok(sum / operands.len() as f64)
            }
            AstNode::WeightedSum { items } => {
                let mut sum = 0.0;
                for item in items {
                    let val = self.evaluate(&item.node)?;
                    sum += val * item.weight;
                }
                Ok(sum)
            }
        }
    }
}

pub fn apply_rounding(value: f64, policy: &str) -> f64 {
    match policy {
        "round_up" => value.ceil(),
        "round_down" => value.floor(),
        "round_half_up" => value.round(),
        _ => value, // "exact" or unknown implies no rounding
    }
}
