use sea_orm::{Condition, sea_query::{Expr, Value as SeaValue, SimpleExpr}, ColumnTrait, EntityTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct UniversalFilter {
    pub conditions: Vec<FilterCondition>,
    pub logic: FilterLogic,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<SortDirection>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FilterLogic {
    #[default]
    And,
    Or,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Between,
    DateBefore,
    DateAfter,
    DateBetween,
    DateInLast,
    DateInNext,
    DateIsToday,
    DateIsThisWeek,
    DateIsThisMonth,
    BoolTrue,
    BoolFalse,
}

pub trait FilterApplier {
    /// Convert the UniversalFilter into a SeaORM Condition
    fn apply_filter(&self, filter: &UniversalFilter) -> Condition;
}

impl UniversalFilter {
    /// Build a SeaORM Condition using a column mapper
    pub fn build_condition<F>(&self, mut col_mapper: F) -> Condition
    where
        F: FnMut(&str) -> Option<SimpleExpr>,
    {
        let mut root = match self.logic {
            FilterLogic::And => Condition::all(),
            FilterLogic::Or => Condition::any(),
        };

        for cond in &self.conditions {
            if let Some(expr) = col_mapper(&cond.field) {
                let val = json_to_sea_expr(&cond.value);
                let filter_cond = match cond.operator {
                    FilterOperator::Eq => Expr::expr(expr).eq(val),
                    FilterOperator::Neq => Expr::expr(expr).ne(val),
                    FilterOperator::Gt => Expr::expr(expr).gt(val),
                    FilterOperator::Gte => Expr::expr(expr).gte(val),
                    FilterOperator::Lt => Expr::expr(expr).lt(val),
                    FilterOperator::Lte => Expr::expr(expr).lte(val),
                    FilterOperator::Contains => Expr::expr(expr).like(format!("%{}%", cond.value.as_str().unwrap_or(""))),
                    FilterOperator::NotContains => Expr::expr(expr).not_like(format!("%{}%", cond.value.as_str().unwrap_or(""))),
                    FilterOperator::StartsWith => Expr::expr(expr).like(format!("{}%", cond.value.as_str().unwrap_or(""))),
                    FilterOperator::EndsWith => Expr::expr(expr).like(format!("%{}", cond.value.as_str().unwrap_or(""))),
                    FilterOperator::IsNull => Expr::expr(expr).is_null(),
                    FilterOperator::IsNotNull => Expr::expr(expr).is_not_null(),
                    FilterOperator::BoolTrue => Expr::expr(expr).eq(true),
                    FilterOperator::BoolFalse => Expr::expr(expr).eq(false),
                    // For IN and NOT IN, we need to extract the array
                    FilterOperator::In => {
                        if let serde_json::Value::Array(arr) = &cond.value {
                            let vals: Vec<SeaValue> = arr.iter().filter_map(json_to_sea_value).collect();
                            Expr::expr(expr).is_in(vals)
                        } else {
                            Expr::expr(expr).eq(val)
                        }
                    },
                    FilterOperator::NotIn => {
                        if let serde_json::Value::Array(arr) = &cond.value {
                            let vals: Vec<SeaValue> = arr.iter().filter_map(json_to_sea_value).collect();
                            Expr::expr(expr).is_not_in(vals)
                        } else {
                            Expr::expr(expr).ne(val)
                        }
                    },
                    _ => Expr::expr(expr).eq(val), // Fallback for unsupported yet
                };
                root = root.add(filter_cond);
            }
        }
        root
    }
}

pub fn json_to_sea_value(val: &serde_json::Value) -> Option<SeaValue> {
    match val {
        serde_json::Value::Null => Some(SeaValue::String(None)),
        serde_json::Value::Bool(b) => Some(SeaValue::Bool(Some(*b))),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(SeaValue::BigInt(Some(i)))
            } else if let Some(f) = n.as_f64() {
                Some(SeaValue::Double(Some(f)))
            } else {
                Some(SeaValue::String(Some(Box::new(n.to_string()))))
            }
        },
        serde_json::Value::String(s) => Some(SeaValue::String(Some(Box::new(s.clone())))),
        _ => None,
    }
}

/// Helper function to parse JSON value into SeaORM SimpleExpr
pub fn json_to_sea_expr(val: &serde_json::Value) -> SimpleExpr {
    match val {
        serde_json::Value::Null => Expr::val(SeaValue::String(None)).into(),
        serde_json::Value::Bool(b) => Expr::val(*b).into(),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Expr::val(i).into()
            } else if let Some(f) = n.as_f64() {
                Expr::val(f).into()
            } else {
                Expr::val(n.to_string()).into()
            }
        },
        serde_json::Value::String(s) => Expr::val(s.clone()).into(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            Expr::val(val.to_string()).into()
        }
    }
}
