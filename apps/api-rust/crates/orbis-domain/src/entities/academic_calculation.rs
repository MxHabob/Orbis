use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::academic_policy::RoundingMode;

fn deserialize_flexible_scale<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum FlexibleScale {
        Direct(String),
        Nested { scale: String },
    }

    match FlexibleScale::deserialize(deserializer)? {
        FlexibleScale::Direct(s) => Ok(s),
        FlexibleScale::Nested { scale } => Ok(scale),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CalculationUnit {
    Dimensionless,
    Score {
        #[serde(deserialize_with = "deserialize_flexible_scale")]
        scale: String,
    },
    Credit,
    Count,
    Custom {
        #[serde(deserialize_with = "deserialize_flexible_scale")]
        scale: String,
    },
}

impl CalculationUnit {
    pub fn score(scale: impl Into<String>) -> Self {
        Self::Score {
            scale: scale.into(),
        }
    }

    pub fn custom(scale: impl Into<String>) -> Self {
        Self::Custom {
            scale: scale.into(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation_unit_clean_serialization() {
        let unit = CalculationUnit::Score {
            scale: "percentage".to_string(),
        };
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, r#"{"type":"score","scale":"percentage"}"#);

        let custom = CalculationUnit::Custom {
            scale: "rubric-1-4".to_string(),
        };
        let json_custom = serde_json::to_string(&custom).unwrap();
        assert_eq!(json_custom, r#"{"type":"custom","scale":"rubric-1-4"}"#);
    }

    #[test]
    fn test_calculation_unit_flexible_deserialization() {
        // Direct string scale (Clean format)
        let direct_json = r#"{"type":"score","scale":"percentage"}"#;
        let direct: CalculationUnit = serde_json::from_str(direct_json).unwrap();
        assert_eq!(
            direct,
            CalculationUnit::Score {
                scale: "percentage".to_string()
            }
        );

        // Nested scale object (Legacy format from Serde content="scale")
        let nested_json = r#"{"type":"score","scale":{"scale":"percentage"}}"#;
        let nested: CalculationUnit = serde_json::from_str(nested_json).unwrap();
        assert_eq!(
            nested,
            CalculationUnit::Score {
                scale: "percentage".to_string()
            }
        );

        // Custom unit direct and nested
        let custom_direct = r#"{"type":"custom","scale":"rubric-1-4"}"#;
        let res_custom_direct: CalculationUnit = serde_json::from_str(custom_direct).unwrap();
        assert_eq!(
            res_custom_direct,
            CalculationUnit::Custom {
                scale: "rubric-1-4".to_string()
            }
        );

        let custom_nested = r#"{"type":"custom","scale":{"scale":"rubric-1-4"}}"#;
        let res_custom_nested: CalculationUnit = serde_json::from_str(custom_nested).unwrap();
        assert_eq!(
            res_custom_nested,
            CalculationUnit::Custom {
                scale: "rubric-1-4".to_string()
            }
        );

        // Dimensionless, Credit, Count
        let dim: CalculationUnit = serde_json::from_str(r#"{"type":"dimensionless"}"#).unwrap();
        assert_eq!(dim, CalculationUnit::Dimensionless);

        let credit: CalculationUnit = serde_json::from_str(r#"{"type":"credit"}"#).unwrap();
        assert_eq!(credit, CalculationUnit::Credit);

        let count: CalculationUnit = serde_json::from_str(r#"{"type":"count"}"#).unwrap();
        assert_eq!(count, CalculationUnit::Count);
    }
}
