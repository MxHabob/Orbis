use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::academic_policy::RoundingMode;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "scale", rename_all = "snake_case")]
pub enum CalculationUnit {
    Dimensionless,
    Score { scale: String },
    Credit,
    Count,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AstNode {
    Literal {
        value: Decimal,
    },
    MeasureLiteral {
        value: Decimal,
        unit: CalculationUnit,
    },
    Variable {
        name: String,
    },
    Add {
        #[serde(default)]
        operands: Vec<AstNode>,
        #[serde(default)]
        left: Option<Box<AstNode>>,
        #[serde(default)]
        right: Option<Box<AstNode>>,
    },
    Subtract {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Multiply {
        #[serde(default)]
        operands: Vec<AstNode>,
        #[serde(default)]
        left: Option<Box<AstNode>>,
        #[serde(default)]
        right: Option<Box<AstNode>>,
    },
    Divide {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Max {
        operands: Vec<AstNode>,
    },
    Min {
        operands: Vec<AstNode>,
    },
    Average {
        operands: Vec<AstNode>,
    },
    WeightedSum {
        items: Vec<WeightedItem>,
        #[serde(default)]
        normalize: bool,
    },
    Convert {
        value: Box<AstNode>,
        from: CalculationUnit,
        to: CalculationUnit,
        factor: Decimal,
        #[serde(default)]
        offset: Decimal,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WeightedItem {
    pub node: AstNode,
    pub weight: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculationInput {
    pub name: String,
    pub unit: CalculationUnit,
    pub minimum: Option<Decimal>,
    pub maximum: Option<Decimal>,
    #[serde(default = "default_required")]
    pub is_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct RoundingRule {
    pub mode: RoundingMode,
    pub decimal_places: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct GradeBand {
    pub code: String,
    pub label: String,
    pub minimum_score: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct OutcomePolicy {
    pub unit: CalculationUnit,
    pub pass_at_or_above: Option<Decimal>,
    pub honors_at_or_above: Option<Decimal>,
    pub grade_bands: Vec<GradeBand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculatedOutcome {
    pub is_passed: Option<bool>,
    pub is_honors: Option<bool>,
    pub grade_code: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WeightedMeasure {
    pub value: Decimal,
    pub weight: Decimal,
    pub included: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculationOutput {
    pub name: String,
    pub unit: CalculationUnit,
    pub expression: AstNode,
    pub rounding: Option<RoundingRule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculationPlan {
    pub version: u32,
    pub inputs: Vec<CalculationInput>,
    pub outputs: Vec<CalculationOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculatedMeasure {
    pub name: String,
    pub unit: CalculationUnit,
    pub value: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculationResult {
    pub policy_version: u32,
    pub outputs: Vec<CalculatedMeasure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculationOutputOutcome {
    pub output_name: String,
    pub outcome: CalculatedOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CalculationPreview {
    pub result: CalculationResult,
    pub outcomes: Vec<CalculationOutputOutcome>,
}

fn default_required() -> bool {
    true
}
