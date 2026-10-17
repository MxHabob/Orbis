pub use orbis_domain::entities::academic_calculation::{AstNode, WeightedItem};
use orbis_domain::entities::academic_calculation::{
    CalculatedMeasure, CalculatedOutcome, CalculationInput, CalculationOutput, CalculationPlan,
    CalculationResult, CalculationUnit, OutcomePolicy, RoundingRule, WeightedMeasure,
};
#[allow(unused_imports)] // ComponentScore used by tests via super::*
use orbis_domain::entities::academic_policy::{
    AcademicPolicy, AcademicStatus, ComponentScore, PeriodAssessmentResult, PeriodComponentScores,
    RoundingMode, SubjectAnnualResult, SubjectTermResult, TiePolicy,
};
use orbis_domain::errors::DomainError;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::collections::HashMap;

pub struct CalcEngine {
    pub context: HashMap<String, Decimal>,
}

const MAX_DEPTH: usize = 64;

impl CalcEngine {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Decimal) {
        self.context.insert(name.to_string(), value);
    }

    pub fn evaluate(&self, node: &AstNode) -> Result<Decimal, DomainError> {
        self.evaluate_inner(node, 0)
    }

    fn evaluate_inner(&self, node: &AstNode, depth: usize) -> Result<Decimal, DomainError> {
        if depth > MAX_DEPTH {
            return Err(DomainError::Validation(format!(
                "Calculation formula exceeded maximum recursion depth ({})",
                MAX_DEPTH
            )));
        }
        match node {
            AstNode::Literal { value } => Ok(*value),
            AstNode::MeasureLiteral { value, .. } => Ok(*value),
            AstNode::Variable { name } => self.context.get(name).copied().ok_or_else(|| {
                DomainError::Validation(format!(
                    "Variable '{}' not found in evaluation context",
                    name
                ))
            }),
            AstNode::Add {
                operands,
                left,
                right,
            } => {
                let mut ops = operands.clone();
                if let Some(l) = left {
                    ops.insert(0, *l.clone());
                }
                if let Some(r) = right {
                    ops.push(*r.clone());
                }
                if ops.is_empty() {
                    return Err(empty_operand_error("addition"));
                }
                let mut sum = Decimal::ZERO;
                for op in &ops {
                    sum = sum
                        .checked_add(self.evaluate_inner(op, depth + 1)?)
                        .ok_or_else(|| arithmetic_overflow("addition"))?;
                }
                Ok(sum)
            }
            AstNode::Subtract { left, right } => {
                let l = self.evaluate_inner(left, depth + 1)?;
                let r = self.evaluate_inner(right, depth + 1)?;
                l.checked_sub(r)
                    .ok_or_else(|| arithmetic_overflow("subtraction"))
            }
            AstNode::Multiply {
                operands,
                left,
                right,
            } => {
                let mut ops = operands.clone();
                if let Some(l) = left {
                    ops.insert(0, *l.clone());
                }
                if let Some(r) = right {
                    ops.push(*r.clone());
                }
                if ops.is_empty() {
                    return Err(empty_operand_error("multiplication"));
                }
                let mut prod = Decimal::ONE;
                for op in &ops {
                    prod = prod
                        .checked_mul(self.evaluate_inner(op, depth + 1)?)
                        .ok_or_else(|| arithmetic_overflow("multiplication"))?;
                }
                Ok(prod)
            }
            AstNode::Divide { left, right } => {
                let l = self.evaluate_inner(left, depth + 1)?;
                let r = self.evaluate_inner(right, depth + 1)?;
                if r == Decimal::ZERO {
                    return Err(DomainError::Validation(
                        "Division by zero in calculation formula".to_string(),
                    ));
                }
                l.checked_div(r)
                    .ok_or_else(|| arithmetic_overflow("division"))
            }
            AstNode::Max { operands } => {
                if operands.is_empty() {
                    return Err(empty_operand_error("maximum"));
                }
                let mut max_val: Option<Decimal> = None;
                for op in operands {
                    let val = self.evaluate_inner(op, depth + 1)?;
                    max_val = Some(match max_val {
                        Some(current) => current.max(val),
                        None => val,
                    });
                }
                max_val.ok_or_else(|| empty_operand_error("maximum"))
            }
            AstNode::Min { operands } => {
                if operands.is_empty() {
                    return Err(empty_operand_error("minimum"));
                }
                let mut min_val: Option<Decimal> = None;
                for op in operands {
                    let val = self.evaluate_inner(op, depth + 1)?;
                    min_val = Some(match min_val {
                        Some(current) => current.min(val),
                        None => val,
                    });
                }
                min_val.ok_or_else(|| empty_operand_error("minimum"))
            }
            AstNode::Average { operands } => {
                if operands.is_empty() {
                    return Err(empty_operand_error("average"));
                }
                let mut sum = Decimal::ZERO;
                for op in operands {
                    sum = sum
                        .checked_add(self.evaluate_inner(op, depth + 1)?)
                        .ok_or_else(|| arithmetic_overflow("average"))?;
                }
                let count = Decimal::from(operands.len());
                sum.checked_div(count)
                    .ok_or_else(|| arithmetic_overflow("average"))
            }
            AstNode::WeightedSum { items, normalize } => {
                if items.is_empty() {
                    return Err(empty_operand_error("weighted sum"));
                }
                let mut sum = Decimal::ZERO;
                let mut total_weight = Decimal::ZERO;
                for item in items {
                    if item.weight < Decimal::ZERO {
                        return Err(DomainError::Validation(
                            "Weighted calculation terms cannot have negative weights".into(),
                        ));
                    }
                    let val = self.evaluate_inner(&item.node, depth + 1)?;
                    let weighted = val
                        .checked_mul(item.weight)
                        .ok_or_else(|| arithmetic_overflow("weighted multiplication"))?;
                    sum = sum
                        .checked_add(weighted)
                        .ok_or_else(|| arithmetic_overflow("weighted sum"))?;
                    total_weight = total_weight
                        .checked_add(item.weight)
                        .ok_or_else(|| arithmetic_overflow("weight total"))?;
                }
                if total_weight == Decimal::ZERO {
                    return Err(DomainError::Validation(
                        "Weighted calculations require a positive total weight".into(),
                    ));
                }
                if *normalize {
                    sum.checked_div(total_weight)
                        .ok_or_else(|| arithmetic_overflow("weighted average"))
                } else {
                    Ok(sum)
                }
            }
            AstNode::Convert {
                value,
                factor,
                offset,
                ..
            } => {
                if factor.is_zero() {
                    return Err(DomainError::Validation(
                        "Calculation conversion factor must not be zero".into(),
                    ));
                }
                self.evaluate_inner(value, depth + 1)?
                    .checked_mul(*factor)
                    .and_then(|converted| converted.checked_add(*offset))
                    .ok_or_else(|| arithmetic_overflow("conversion"))
            }
        }
    }
}

fn arithmetic_overflow(operation: &str) -> DomainError {
    DomainError::Validation(format!("Decimal overflow while evaluating {operation}"))
}

fn empty_operand_error(operation: &str) -> DomainError {
    DomainError::Validation(format!(
        "Calculation {operation} requires at least one operand"
    ))
}

fn validate_name(name: &str, kind: &str) -> Result<(), DomainError> {
    if name.trim().is_empty() {
        return Err(DomainError::Validation(format!("{kind} name is required")));
    }
    Ok(())
}

fn validate_unit(unit: &CalculationUnit) -> Result<(), DomainError> {
    if matches!(
        unit,
        CalculationUnit::Custom { scale: name } | CalculationUnit::Score { scale: name }
            if name.trim().is_empty()
    ) {
        return Err(DomainError::Validation(
            "Custom and score units must have a non-empty name".into(),
        ));
    }
    Ok(())
}

fn infer_unit(
    node: &AstNode,
    input_units: &HashMap<&str, CalculationUnit>,
    depth: usize,
) -> Result<CalculationUnit, DomainError> {
    if depth > MAX_DEPTH {
        return Err(DomainError::Validation(format!(
            "Calculation formula exceeded maximum recursion depth ({MAX_DEPTH})"
        )));
    }

    match node {
        AstNode::Literal { .. } => Ok(CalculationUnit::Dimensionless),
        AstNode::MeasureLiteral { unit, .. } => {
            validate_unit(unit)?;
            Ok(unit.clone())
        }
        AstNode::Variable { name } => input_units.get(name.as_str()).cloned().ok_or_else(|| {
            DomainError::Validation(format!(
                "Calculation formula references unknown input '{}'",
                name
            ))
        }),
        AstNode::Add {
            operands,
            left,
            right,
        } => {
            let mut nodes: Vec<&AstNode> = operands.iter().collect();
            if let Some(left) = left {
                nodes.insert(0, left);
            }
            if let Some(right) = right {
                nodes.push(right);
            }
            infer_same_unit(&nodes, input_units, depth, "addition")
        }
        AstNode::Subtract { left, right } => {
            let left = infer_unit(left, input_units, depth + 1)?;
            let right = infer_unit(right, input_units, depth + 1)?;
            ensure_same_unit(left, right, "subtraction")
        }
        AstNode::Multiply {
            operands,
            left,
            right,
        } => {
            let mut nodes: Vec<&AstNode> = operands.iter().collect();
            if let Some(left) = left {
                nodes.insert(0, left);
            }
            if let Some(right) = right {
                nodes.push(right);
            }
            if nodes.is_empty() {
                return Err(empty_operand_error("multiplication"));
            }
            let mut result = CalculationUnit::Dimensionless;
            for node in nodes {
                let unit = infer_unit(node, input_units, depth + 1)?;
                if result == CalculationUnit::Dimensionless {
                    result = unit;
                } else if unit != CalculationUnit::Dimensionless && unit != result {
                    return Err(DomainError::Validation(
                        "Multiplication of two dimensional measures requires an explicit derived unit"
                            .into(),
                    ));
                }
            }
            Ok(result)
        }
        AstNode::Divide { left, right } => {
            let numerator = infer_unit(left, input_units, depth + 1)?;
            let denominator = infer_unit(right, input_units, depth + 1)?;
            if denominator == CalculationUnit::Dimensionless {
                Ok(numerator)
            } else if numerator == denominator {
                Ok(CalculationUnit::Dimensionless)
            } else {
                Err(DomainError::Validation(format!(
                    "Cannot divide measures with units {:?} and {:?} without an explicit conversion",
                    numerator, denominator
                )))
            }
        }
        AstNode::Max { operands } => infer_same_unit(
            operands.iter().collect::<Vec<_>>().as_slice(),
            input_units,
            depth,
            "maximum",
        ),
        AstNode::Min { operands } => infer_same_unit(
            operands.iter().collect::<Vec<_>>().as_slice(),
            input_units,
            depth,
            "minimum",
        ),
        AstNode::Average { operands } => infer_same_unit(
            operands.iter().collect::<Vec<_>>().as_slice(),
            input_units,
            depth,
            "average",
        ),
        AstNode::WeightedSum { items, .. } => {
            if items.is_empty() {
                return Err(empty_operand_error("weighted sum"));
            }
            let mut total_weight = Decimal::ZERO;
            for item in items {
                if item.weight < Decimal::ZERO {
                    return Err(DomainError::Validation(
                        "Weighted calculation terms cannot have negative weights".into(),
                    ));
                }
                total_weight = total_weight
                    .checked_add(item.weight)
                    .ok_or_else(|| arithmetic_overflow("weight total"))?;
            }
            if total_weight == Decimal::ZERO {
                return Err(DomainError::Validation(
                    "Weighted calculations require a positive total weight".into(),
                ));
            }
            let nodes: Vec<&AstNode> = items.iter().map(|item| &item.node).collect();
            infer_same_unit(&nodes, input_units, depth, "weighted sum")
        }
        AstNode::Convert {
            value,
            from,
            to,
            factor,
            ..
        } => {
            validate_unit(from)?;
            validate_unit(to)?;
            if factor.is_zero() {
                return Err(DomainError::Validation(
                    "Calculation conversion factor must not be zero".into(),
                ));
            }
            let actual = infer_unit(value, input_units, depth + 1)?;
            if actual != *from {
                return Err(DomainError::Validation(format!(
                    "Conversion expected unit {:?}, but its value has unit {:?}",
                    from, actual
                )));
            }
            Ok(to.clone())
        }
    }
}

fn infer_same_unit(
    nodes: &[&AstNode],
    input_units: &HashMap<&str, CalculationUnit>,
    depth: usize,
    operation: &str,
) -> Result<CalculationUnit, DomainError> {
    if nodes.is_empty() {
        return Err(empty_operand_error(operation));
    }
    let mut units = nodes
        .iter()
        .map(|node| infer_unit(node, input_units, depth + 1));
    let Some(first) = units.next() else {
        return Err(empty_operand_error(operation));
    };
    let first = first?;
    for unit in units {
        let unit = unit?;
        if unit != first {
            return Err(DomainError::Validation(format!(
                "Calculation {operation} requires compatible units; found {:?} and {:?}",
                first, unit
            )));
        }
    }
    Ok(first)
}

fn ensure_same_unit(
    left: CalculationUnit,
    right: CalculationUnit,
    operation: &str,
) -> Result<CalculationUnit, DomainError> {
    if left != right {
        return Err(DomainError::Validation(format!(
            "Calculation {operation} requires compatible units; found {:?} and {:?}",
            left, right
        )));
    }
    Ok(left)
}

fn rounding_rule(policy: &AcademicPolicy) -> RoundingRule {
    RoundingRule {
        mode: policy.rounding_mode,
        decimal_places: policy.decimal_places,
    }
}

fn output_value(result: &CalculationResult, name: &str) -> Result<Decimal, DomainError> {
    result
        .outputs
        .iter()
        .find(|output| output.name == name)
        .map(|output| output.value)
        .ok_or_else(|| {
            DomainError::Internal(format!(
                "Calculation plan did not produce the required output '{name}'"
            ))
        })
}

/// Applies rounding according to the specified policy and decimal precision.
pub fn apply_rounding(value: Decimal, mode: RoundingMode, dp: u32) -> Decimal {
    match mode {
        RoundingMode::RoundHalfUp => {
            value.round_dp_with_strategy(dp, RoundingStrategy::MidpointAwayFromZero)
        }
        RoundingMode::RoundUp => value.round_dp_with_strategy(dp, RoundingStrategy::AwayFromZero),
        RoundingMode::RoundDown => value.round_dp_with_strategy(dp, RoundingStrategy::ToZero),
        RoundingMode::Truncate => value.trunc_with_scale(dp),
    }
}

/// Authoritative School Academic Calculation Service.
pub struct AcademicCalculationEngine;

impl AcademicCalculationEngine {
    pub fn validate_plan(plan: &CalculationPlan) -> Result<(), DomainError> {
        if plan.version == 0 {
            return Err(DomainError::Validation(
                "Calculation plan version must be greater than zero".into(),
            ));
        }
        if plan.outputs.is_empty() {
            return Err(DomainError::Validation(
                "Calculation plan must declare at least one output".into(),
            ));
        }

        let mut known_units = HashMap::new();
        for input in &plan.inputs {
            validate_name(&input.name, "Calculation input")?;
            validate_unit(&input.unit)?;
            if input
                .minimum
                .zip(input.maximum)
                .is_some_and(|(minimum, maximum)| maximum < minimum)
            {
                return Err(DomainError::Validation(format!(
                    "Input '{}' maximum must be greater than or equal to minimum",
                    input.name
                )));
            }
            if known_units
                .insert(input.name.as_str(), input.unit.clone())
                .is_some()
            {
                return Err(DomainError::Validation(format!(
                    "Duplicate calculation input '{}'",
                    input.name
                )));
            }
        }

        let mut output_names = std::collections::HashSet::new();
        for output in &plan.outputs {
            validate_name(&output.name, "Calculation output")?;
            validate_unit(&output.unit)?;
            if !output_names.insert(output.name.as_str())
                || known_units.contains_key(output.name.as_str())
            {
                return Err(DomainError::Validation(format!(
                    "Calculation output '{}' conflicts with another input or output",
                    output.name
                )));
            }
            if output.rounding.is_some_and(|rule| rule.decimal_places > 28) {
                return Err(DomainError::Validation(
                    "Calculation outputs support at most 28 decimal places".into(),
                ));
            }
            let expression_unit = infer_unit(&output.expression, &known_units, 0)?;
            if expression_unit != output.unit {
                return Err(DomainError::Validation(format!(
                    "Output '{}' declares unit {:?}, but its expression produces {:?}",
                    output.name, output.unit, expression_unit
                )));
            }
            known_units.insert(output.name.as_str(), output.unit.clone());
        }

        Ok(())
    }

    pub fn calculate_plan(
        plan: &CalculationPlan,
        values: &HashMap<String, Decimal>,
    ) -> Result<CalculationResult, DomainError> {
        Self::validate_plan(plan)?;
        let inputs_by_name: HashMap<&str, &CalculationInput> = plan
            .inputs
            .iter()
            .map(|input| (input.name.as_str(), input))
            .collect();

        if let Some(unknown) = values
            .keys()
            .find(|name| !inputs_by_name.contains_key(name.as_str()))
        {
            return Err(DomainError::Validation(format!(
                "Unknown calculation input '{}'",
                unknown
            )));
        }

        let mut evaluator = CalcEngine::new();
        for input in &plan.inputs {
            let Some(value) = values.get(&input.name) else {
                if input.is_required {
                    return Err(DomainError::Validation(format!(
                        "Required calculation input '{}' is missing",
                        input.name
                    )));
                }
                continue;
            };
            if input.minimum.is_some_and(|minimum| *value < minimum)
                || input.maximum.is_some_and(|maximum| *value > maximum)
            {
                return Err(DomainError::Validation(format!(
                    "Calculation input '{}' is outside its declared range",
                    input.name
                )));
            }
            evaluator.set_variable(&input.name, *value);
        }

        let mut outputs = Vec::with_capacity(plan.outputs.len());
        for output in &plan.outputs {
            let mut value = evaluator.evaluate(&output.expression)?;
            if let Some(rounding) = output.rounding {
                value = apply_rounding(value, rounding.mode, rounding.decimal_places);
            }
            evaluator.set_variable(&output.name, value);
            outputs.push(CalculatedMeasure {
                name: output.name.clone(),
                unit: output.unit.clone(),
                value,
            });
        }

        Ok(CalculationResult {
            policy_version: plan.version,
            outputs,
        })
    }

    pub fn evaluate_outcome(
        value: Decimal,
        unit: &CalculationUnit,
        policy: &OutcomePolicy,
    ) -> Result<CalculatedOutcome, DomainError> {
        validate_unit(&policy.unit)?;
        if unit != &policy.unit {
            return Err(DomainError::Validation(
                "Outcome policy unit does not match the calculated measure".into(),
            ));
        }
        if policy
            .pass_at_or_above
            .zip(policy.honors_at_or_above)
            .is_some_and(|(pass, honors)| honors < pass)
            || (policy.honors_at_or_above.is_some() && policy.pass_at_or_above.is_none())
        {
            return Err(DomainError::Validation(
                "An honors threshold must not be lower than, and requires, a passing threshold"
                    .into(),
            ));
        }
        let mut codes = std::collections::HashSet::new();
        let mut thresholds = std::collections::HashSet::new();
        for band in &policy.grade_bands {
            validate_name(&band.code, "Grade band code")?;
            validate_name(&band.label, "Grade band label")?;
            if !codes.insert(band.code.as_str()) || !thresholds.insert(band.minimum_score) {
                return Err(DomainError::Validation(
                    "Grade band codes and minimum scores must be unique".into(),
                ));
            }
        }

        let grade_code = policy
            .grade_bands
            .iter()
            .filter(|band| value >= band.minimum_score)
            .max_by_key(|band| band.minimum_score)
            .map(|band| band.code.clone());
        Ok(CalculatedOutcome {
            is_passed: policy.pass_at_or_above.map(|threshold| value >= threshold),
            is_honors: policy
                .honors_at_or_above
                .map(|threshold| value >= threshold),
            grade_code,
        })
    }

    pub fn calculate_weighted_average(
        measures: &[WeightedMeasure],
        rounding: Option<RoundingRule>,
    ) -> Result<Decimal, DomainError> {
        if let Some(rule) = rounding
            && rule.decimal_places > 28
        {
            return Err(DomainError::Validation(
                "Weighted average supports at most 28 decimal places".into(),
            ));
        }

        let mut weighted_total = Decimal::ZERO;
        let mut total_weight = Decimal::ZERO;
        for measure in measures {
            if measure.weight < Decimal::ZERO {
                return Err(DomainError::Validation(
                    "Weighted average weights cannot be negative".into(),
                ));
            }
            if !measure.included {
                continue;
            }
            weighted_total = weighted_total
                .checked_add(
                    measure
                        .value
                        .checked_mul(measure.weight)
                        .ok_or_else(|| arithmetic_overflow("weighted average multiplication"))?,
                )
                .ok_or_else(|| arithmetic_overflow("weighted average total"))?;
            total_weight = total_weight
                .checked_add(measure.weight)
                .ok_or_else(|| arithmetic_overflow("weighted average weight total"))?;
        }
        if total_weight <= Decimal::ZERO {
            return Err(DomainError::Validation(
                "Weighted average requires a positive included weight".into(),
            ));
        }

        let average = weighted_total
            .checked_div(total_weight)
            .ok_or_else(|| arithmetic_overflow("weighted average division"))?;
        Ok(rounding.map_or(average, |rule| {
            apply_rounding(average, rule.mode, rule.decimal_places)
        }))
    }

    /// Compatibility adapter for legacy period score DTOs.
    pub fn calculate_monthly_total(
        breakdown: &PeriodComponentScores,
        policy: &AcademicPolicy,
    ) -> Result<Decimal, DomainError> {
        let unit = CalculationUnit::Score {
            scale: "period_total".into(),
        };
        let mut inputs = Vec::with_capacity(breakdown.components.len());
        let mut values = HashMap::with_capacity(breakdown.components.len());
        let mut capped_terms = Vec::with_capacity(breakdown.components.len());

        for (index, component) in breakdown.components.iter().enumerate() {
            let name = format!("component_{index}");
            inputs.push(CalculationInput {
                name: name.clone(),
                unit: unit.clone(),
                minimum: None,
                maximum: None,
                is_required: true,
            });
            values.insert(name.clone(), component.score);
            capped_terms.push(AstNode::Min {
                operands: vec![
                    AstNode::Variable { name },
                    AstNode::MeasureLiteral {
                        value: component.max_score,
                        unit: unit.clone(),
                    },
                ],
            });
        }
        let expression = match capped_terms.len() {
            0 => AstNode::MeasureLiteral {
                value: Decimal::ZERO,
                unit: unit.clone(),
            },
            1 => capped_terms.remove(0),
            _ => AstNode::Add {
                operands: capped_terms,
                left: None,
                right: None,
            },
        };
        let plan = CalculationPlan {
            version: 1,
            inputs,
            outputs: vec![CalculationOutput {
                name: "period_total".into(),
                unit,
                expression,
                rounding: Some(rounding_rule(policy)),
            }],
        };
        let result = Self::calculate_plan(&plan, &values)?;
        output_value(&result, "period_total")
    }

    /// Compatibility adapter that maps the legacy term DTO to a general calculation plan.
    pub fn calculate_subject_term_result(
        subject_id: uuid::Uuid,
        subject_code: String,
        subject_name: String,
        subject_name_ar: Option<String>,
        credit_hours: Decimal,
        monthly_results: Vec<PeriodAssessmentResult>,
        term_final_exam: Option<Decimal>,
        policy: &AcademicPolicy,
    ) -> Result<SubjectTermResult, DomainError> {
        let mut monthly_totals = Vec::new();
        let mut monthly_total_max = Decimal::from(100); // Default, overridden by actual breakdown

        for par in &monthly_results {
            let total = Self::calculate_monthly_total(&par.breakdown, policy)?;
            monthly_totals.push(total);
            monthly_total_max = par.breakdown.total_max;
        }

        let monthly_unit = CalculationUnit::Score {
            scale: "period_total".into(),
        };
        let exam_unit = CalculationUnit::Score {
            scale: "term_exam".into(),
        };
        let term_unit = CalculationUnit::Score {
            scale: "term_total".into(),
        };
        let mut inputs = Vec::with_capacity(monthly_totals.len() + 2);
        let mut values = HashMap::with_capacity(monthly_totals.len() + 2);
        let mut month_nodes = Vec::with_capacity(monthly_totals.len());
        for (index, total) in monthly_totals.iter().enumerate() {
            let name = format!("period_{index}");
            inputs.push(CalculationInput {
                name: name.clone(),
                unit: monthly_unit.clone(),
                minimum: None,
                maximum: None,
                is_required: true,
            });
            values.insert(name.clone(), *total);
            month_nodes.push(AstNode::Variable { name });
        }
        inputs.push(CalculationInput {
            name: "period_total_max".into(),
            unit: monthly_unit.clone(),
            minimum: Some(Decimal::ZERO),
            maximum: None,
            is_required: true,
        });
        values.insert("period_total_max".into(), monthly_total_max);
        inputs.push(CalculationInput {
            name: "term_exam".into(),
            unit: exam_unit.clone(),
            minimum: None,
            maximum: None,
            is_required: true,
        });
        values.insert("term_exam".into(), term_final_exam.unwrap_or(Decimal::ZERO));

        let coursework_max = policy.coursework_max_score;
        let exam_max = policy.exam_max_score;
        let term_max = if policy.term_max_score > Decimal::ZERO {
            policy.term_max_score
        } else {
            coursework_max + exam_max
        };

        let monthly_average_expression = if month_nodes.is_empty() {
            AstNode::MeasureLiteral {
                value: Decimal::ZERO,
                unit: monthly_unit.clone(),
            }
        } else {
            AstNode::Average {
                operands: month_nodes,
            }
        };
        let monthly_contribution_expression = AstNode::Multiply {
            operands: vec![
                AstNode::Divide {
                    left: Box::new(monthly_average_expression.clone()),
                    right: Box::new(AstNode::Variable {
                        name: "period_total_max".into(),
                    }),
                },
                AstNode::MeasureLiteral {
                    value: coursework_max,
                    unit: term_unit.clone(),
                },
            ],
            left: None,
            right: None,
        };
        let capped_exam = AstNode::Min {
            operands: vec![
                AstNode::Variable {
                    name: "term_exam".into(),
                },
                AstNode::MeasureLiteral {
                    value: exam_max,
                    unit: exam_unit.clone(),
                },
            ],
        };
        let converted_exam = AstNode::Convert {
            value: Box::new(capped_exam),
            from: exam_unit,
            to: term_unit.clone(),
            factor: Decimal::ONE,
            offset: Decimal::ZERO,
        };
        let raw_term_total = AstNode::Add {
            operands: vec![monthly_contribution_expression.clone(), converted_exam],
            left: None,
            right: None,
        };
        let percentage_expression = AstNode::Multiply {
            operands: vec![
                AstNode::Divide {
                    left: Box::new(raw_term_total.clone()),
                    right: Box::new(AstNode::MeasureLiteral {
                        value: term_max,
                        unit: term_unit.clone(),
                    }),
                },
                AstNode::Literal {
                    value: Decimal::from(100),
                },
            ],
            left: None,
            right: None,
        };
        let plan = CalculationPlan {
            version: 1,
            inputs,
            outputs: vec![
                CalculationOutput {
                    name: "monthly_average".into(),
                    unit: monthly_unit,
                    expression: monthly_average_expression,
                    rounding: Some(rounding_rule(policy)),
                },
                CalculationOutput {
                    name: "monthly_contribution".into(),
                    unit: term_unit.clone(),
                    expression: monthly_contribution_expression,
                    rounding: Some(rounding_rule(policy)),
                },
                CalculationOutput {
                    name: "term_total".into(),
                    unit: term_unit,
                    expression: raw_term_total,
                    rounding: Some(rounding_rule(policy)),
                },
                CalculationOutput {
                    name: "percentage".into(),
                    unit: CalculationUnit::Dimensionless,
                    expression: percentage_expression,
                    rounding: Some(rounding_rule(policy)),
                },
            ],
        };
        let calculated = Self::calculate_plan(&plan, &values)?;
        let monthly_avg = output_value(&calculated, "monthly_average")?;
        let monthly_contribution = output_value(&calculated, "monthly_contribution")?;
        let term_total = output_value(&calculated, "term_total")?;
        let percentage = output_value(&calculated, "percentage")?;

        let is_incomplete = term_final_exam.is_none();
        let failed_min_exam = if let (Some(min_exam), Some(exam_score)) = (policy.minimum_final_exam_score, term_final_exam) {
            exam_score < min_exam
        } else {
            false
        };

        let status = if is_incomplete {
            AcademicStatus::Incomplete
        } else if failed_min_exam {
            AcademicStatus::Failed
        } else if percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        let letter_grade = Self::resolve_letter_grade(percentage);

        Ok(SubjectTermResult {
            subject_id,
            subject_code,
            subject_name,
            subject_name_ar,
            credit_hours,
            monthly_results,
            monthly_average: monthly_avg,
            monthly_contribution,
            monthly_contribution_max: coursework_max,
            term_final_exam,
            term_final_exam_max: exam_max,
            term_total,
            term_max,
            percentage,
            letter_grade: Some(letter_grade),
            status,
        })
    }

    /// Compatibility adapter that maps the legacy annual DTO to a general calculation plan.
    pub fn calculate_subject_annual_result(
        subject_id: uuid::Uuid,
        subject_code: String,
        subject_name: String,
        subject_name_ar: Option<String>,
        credit_hours: Decimal,
        term_1_result: Option<SubjectTermResult>,
        term_2_result: Option<SubjectTermResult>,
        policy: &AcademicPolicy,
    ) -> Result<SubjectAnnualResult, DomainError> {
        let t1_score = term_1_result
            .as_ref()
            .map(|r| r.term_total)
            .unwrap_or(Decimal::ZERO);
        let t2_score = term_2_result
            .as_ref()
            .map(|r| r.term_total)
            .unwrap_or(Decimal::ZERO);

        let term_unit = CalculationUnit::Score {
            scale: "term_total".into(),
        };
        let annual_unit = CalculationUnit::Score {
            scale: "annual_total".into(),
        };
        let plan = CalculationPlan {
            version: 1,
            inputs: vec![
                CalculationInput {
                    name: "term_1".into(),
                    unit: term_unit.clone(),
                    minimum: None,
                    maximum: None,
                    is_required: true,
                },
                CalculationInput {
                    name: "term_2".into(),
                    unit: term_unit.clone(),
                    minimum: None,
                    maximum: None,
                    is_required: true,
                },
            ],
            outputs: vec![
                CalculationOutput {
                    name: "annual_total".into(),
                    unit: annual_unit.clone(),
                    expression: AstNode::Add {
                        operands: vec![
                            AstNode::Convert {
                                value: Box::new(AstNode::Variable {
                                    name: "term_1".into(),
                                }),
                                from: term_unit.clone(),
                                to: annual_unit.clone(),
                                factor: Decimal::ONE,
                                offset: Decimal::ZERO,
                            },
                            AstNode::Convert {
                                value: Box::new(AstNode::Variable {
                                    name: "term_2".into(),
                                }),
                                from: term_unit.clone(),
                                to: annual_unit.clone(),
                                factor: Decimal::ONE,
                                offset: Decimal::ZERO,
                            },
                        ],
                        left: None,
                        right: None,
                    },
                    rounding: Some(rounding_rule(policy)),
                },
                CalculationOutput {
                    name: "percentage".into(),
                    unit: CalculationUnit::Dimensionless,
                    expression: AstNode::Multiply {
                        operands: vec![
                            AstNode::Divide {
                                left: Box::new(AstNode::Variable {
                                    name: "annual_total".into(),
                                }),
                                right: Box::new(AstNode::MeasureLiteral {
                                    value: policy.annual_max_score,
                                    unit: annual_unit.clone(),
                                }),
                            },
                            AstNode::Literal {
                                value: Decimal::from(100),
                            },
                        ],
                        left: None,
                        right: None,
                    },
                    rounding: Some(rounding_rule(policy)),
                },
            ],
        };
        let values = HashMap::from([("term_1".into(), t1_score), ("term_2".into(), t2_score)]);
        let result = Self::calculate_plan(&plan, &values)?;
        let annual_total = output_value(&result, "annual_total")?;
        let percentage = output_value(&result, "percentage")?;

        let is_incomplete = term_1_result.is_none() && term_2_result.is_none();
        let status = if is_incomplete {
            AcademicStatus::Incomplete
        } else if percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        let letter_grade = Self::resolve_letter_grade(percentage);

        Ok(SubjectAnnualResult {
            subject_id,
            subject_code,
            subject_name,
            subject_name_ar,
            credit_hours,
            term_1_result,
            term_2_result,
            annual_total,
            annual_max: policy.annual_max_score,
            percentage,
            letter_grade: Some(letter_grade),
            status,
        })
    }

    /// Calculates a vocational training module result combining theory, practical skills, and mandatory competency gate.
    pub fn calculate_vocational_module_result(
        subject_id: uuid::Uuid,
        subject_code: String,
        subject_name: String,
        subject_name_ar: Option<String>,
        theory_score: Option<Decimal>,
        practical_score: Option<Decimal>,
        competency_mastered: bool,
        policy: &AcademicPolicy,
    ) -> Result<SubjectTermResult, DomainError> {
        let theory = theory_score.unwrap_or(Decimal::ZERO);
        let practical = practical_score.unwrap_or(Decimal::ZERO);
        let theory_max = policy.coursework_max_score; // e.g. 40
        let practical_max = policy.exam_max_score;     // e.g. 60
        let total_max = theory_max + practical_max;

        let total_score = theory.min(theory_max) + practical.min(practical_max);
        let percentage = if total_max > Decimal::ZERO {
            apply_rounding(
                (total_score / total_max) * Decimal::from(100),
                policy.rounding_mode,
                policy.decimal_places,
            )
        } else {
            Decimal::ZERO
        };

        let is_incomplete = theory_score.is_none() || practical_score.is_none();
        let min_practical_met = if let Some(min_p) = policy.minimum_final_exam_score {
            practical >= min_p
        } else {
            true
        };

        let status = if is_incomplete {
            AcademicStatus::Incomplete
        } else if !competency_mastered || !min_practical_met {
            AcademicStatus::Failed
        } else if percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        let letter_grade = Self::resolve_letter_grade(percentage);

        Ok(SubjectTermResult {
            subject_id,
            subject_code,
            subject_name,
            subject_name_ar,
            credit_hours: Decimal::from(3),
            monthly_results: vec![],
            monthly_average: theory,
            monthly_contribution: theory,
            monthly_contribution_max: theory_max,
            term_final_exam: practical_score,
            term_final_exam_max: practical_max,
            term_total: total_score,
            term_max: total_max,
            percentage,
            letter_grade: Some(letter_grade),
            status,
        })
    }

    // ═══════════════════════════════════════════════════════════════
    // Certificate Examination (Thanawiya / Basic Certificate)
    // ═══════════════════════════════════════════════════════════════

    /// Resolves the cumulative prior-year contribution score.
    ///
    /// Implements the verified Sana'a MoE formula:
    ///   cumulative = avg(prior_year_annual_scores) / divisor
    ///
    /// For Thanawiya (Grade 12):
    ///   - prior_year_annual_scores = [Grade 9 annual, Grade 10 annual, Grade 11 annual]
    ///   - Each out of 100
    ///   - divisor = 5 (100 → 20 scale)
    ///   - Result: max 20 points
    ///
    /// Returns `None` if no prior-year scores are available.
    pub fn resolve_cumulative_score(
        prior_year_annual_scores: &[Decimal],
        cumulative_max_score: Decimal,
        annual_max_score: Decimal,
        policy: &AcademicPolicy,
    ) -> Option<Decimal> {
        if prior_year_annual_scores.is_empty() || annual_max_score == Decimal::ZERO {
            return None;
        }

        let sum: Decimal = prior_year_annual_scores.iter().copied().sum();
        let count = Decimal::from(prior_year_annual_scores.len());
        let average = sum / count;

        // Normalize: average (out of annual_max) → cumulative_max scale
        // e.g., avg=80 out of 100, cumulative_max=20 → 80/100*20 = 16
        let cumulative = (average / annual_max_score) * cumulative_max_score;
        let rounded = apply_rounding(cumulative, policy.rounding_mode, policy.decimal_places);
        Some(rounded.min(cumulative_max_score))
    }

    /// Calculates a certificate examination result for a single subject.
    ///
    /// Combines the ministerial exam score and the cumulative prior-year
    /// contribution into a final certificate score.
    ///
    /// For Yemen Thanawiya (Grade 12):
    ///   - exam_score: out of 80 (ministerial exam)
    ///   - cumulative_score: out of 20 (resolved from prior years)
    ///   - total: out of 100
    pub fn calculate_certificate_result(
        subject_id: uuid::Uuid,
        subject_code: String,
        subject_name: String,
        subject_name_ar: Option<String>,
        credit_hours: Decimal,
        exam_score: Option<Decimal>,
        cumulative_score: Option<Decimal>,
        policy: &AcademicPolicy,
    ) -> Result<SubjectTermResult, DomainError> {
        let exam_max = policy.certificate_exam_max_score;       // 80
        let cumul_max = policy.certificate_cumulative_max_score; // 20
        let total_max = exam_max + cumul_max;                    // 100

        let exam = exam_score.unwrap_or(Decimal::ZERO).min(exam_max);
        let cumul = cumulative_score.unwrap_or(Decimal::ZERO).min(cumul_max);
        let total_score = exam + cumul;

        let percentage = if total_max > Decimal::ZERO {
            apply_rounding(
                (total_score / total_max) * Decimal::from(100),
                policy.rounding_mode,
                policy.decimal_places,
            )
        } else {
            Decimal::ZERO
        };

        let is_incomplete = exam_score.is_none();
        let status = if is_incomplete {
            AcademicStatus::Incomplete
        } else if percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        let letter_grade = Self::resolve_letter_grade(percentage);

        // Map onto SubjectTermResult — reuse the existing structure:
        //   monthly_contribution = cumulative_score (prior-year contribution)
        //   term_final_exam = exam_score (ministerial exam)
        //   term_total = total (certificate result)
        Ok(SubjectTermResult {
            subject_id,
            subject_code,
            subject_name,
            subject_name_ar,
            credit_hours,
            monthly_results: vec![], // No monthly breakdown for certificates
            monthly_average: cumul,  // cumulative score
            monthly_contribution: cumul,
            monthly_contribution_max: cumul_max,
            term_final_exam: exam_score,
            term_final_exam_max: exam_max,
            term_total: total_score,
            term_max: total_max,
            percentage,
            letter_grade: Some(letter_grade),
            status,
        })
    }

    /// Computes policy-driven ranks, including exact fractional tie ranks.
    pub fn compute_rankings<T: Copy + Eq + std::hash::Hash>(
        items: &[(T, Decimal)],
        tie_policy: TiePolicy,
    ) -> Vec<(T, Decimal)> {
        let mut sorted = items.to_vec();
        sorted.sort_by(|left, right| right.1.cmp(&left.1));

        let mut results = Vec::with_capacity(sorted.len());
        let mut start = 0;
        let mut dense_rank = 0u32;
        while start < sorted.len() {
            let mut end = start + 1;
            while end < sorted.len() && sorted[end].1 == sorted[start].1 {
                end += 1;
            }
            dense_rank += 1;
            let rank = match tie_policy {
                TiePolicy::StandardCompetition => Decimal::from(start + 1),
                TiePolicy::Dense => Decimal::from(dense_rank),
                TiePolicy::Fractional => {
                    (Decimal::from(start + 1) + Decimal::from(end)) / Decimal::from(2)
                }
            };
            results.extend(sorted[start..end].iter().map(|(id, _)| (*id, rank)));
            start = end;
        }

        results
    }

    pub fn compute_rankings_legacy<T: Copy + Eq + std::hash::Hash>(
        items: &[(T, Decimal)],
        tie_policy: TiePolicy,
    ) -> Result<Vec<(T, u32)>, DomainError> {
        Self::compute_rankings(items, tie_policy)
            .into_iter()
            .map(|(id, rank)| {
                rank.to_u32()
                    .map(|integer_rank| (id, integer_rank))
                    .ok_or_else(|| {
                        DomainError::Validation(
                            "Fractional ranks require the typed decimal result contract".into(),
                        )
                    })
            })
            .collect()
    }

    /// Resolve letter grade based on standard 100-point scale.
    pub fn resolve_letter_grade(percentage: Decimal) -> String {
        if percentage >= Decimal::from(90) {
            "A".into()
        } else if percentage >= Decimal::from(80) {
            "B".into()
        } else if percentage >= Decimal::from(70) {
            "C".into()
        } else if percentage >= Decimal::from(60) {
            "D".into()
        } else {
            "F".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbis_domain::entities::academic_calculation::{
        CalculationInput, CalculationOutput, GradeBand, OutcomePolicy, RoundingRule,
    };
    use uuid::Uuid;

    fn d(s: &str) -> Decimal {
        Decimal::from_str_exact(s).unwrap()
    }

    fn score_unit(scale: &str) -> CalculationUnit {
        CalculationUnit::Score {
            scale: scale.into(),
        }
    }

    #[test]
    fn typed_plan_calculates_weighted_outputs_with_version_and_units() {
        let unit = score_unit("percentage");
        let plan = CalculationPlan {
            version: 3,
            inputs: vec![
                CalculationInput {
                    name: "coursework".into(),
                    unit: unit.clone(),
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
                CalculationInput {
                    name: "practical".into(),
                    unit: unit.clone(),
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
            ],
            outputs: vec![CalculationOutput {
                name: "overall".into(),
                unit,
                expression: AstNode::WeightedSum {
                    items: vec![
                        WeightedItem {
                            node: AstNode::Variable {
                                name: "coursework".into(),
                            },
                            weight: d("0.4"),
                        },
                        WeightedItem {
                            node: AstNode::Variable {
                                name: "practical".into(),
                            },
                            weight: d("0.6"),
                        },
                    ],
                    normalize: true,
                },
                rounding: Some(RoundingRule {
                    mode: RoundingMode::RoundHalfUp,
                    decimal_places: 2,
                }),
            }],
        };
        let values = HashMap::from([
            ("coursework".into(), d("80")),
            ("practical".into(), d("90")),
        ]);

        let result = AcademicCalculationEngine::calculate_plan(&plan, &values).unwrap();
        assert_eq!(result.policy_version, 3);
        assert_eq!(result.outputs[0].value, d("86"));
        assert_eq!(result.outputs[0].unit, score_unit("percentage"));
    }

    #[test]
    fn institutional_profiles_cover_optional_months_credits_and_competencies() {
        let monthly_school = CalculationPlan {
            version: 1,
            inputs: vec![
                CalculationInput {
                    name: "month_1".into(),
                    unit: score_unit("percent"),
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
                CalculationInput {
                    name: "month_2".into(),
                    unit: score_unit("percent"),
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
            ],
            outputs: vec![CalculationOutput {
                name: "term".into(),
                unit: score_unit("percent"),
                expression: AstNode::Average {
                    operands: vec![
                        AstNode::Variable {
                            name: "month_1".into(),
                        },
                        AstNode::Variable {
                            name: "month_2".into(),
                        },
                    ],
                },
                rounding: None,
            }],
        };
        let monthly_result = AcademicCalculationEngine::calculate_plan(
            &monthly_school,
            &HashMap::from([("month_1".into(), d("80")), ("month_2".into(), d("90"))]),
        )
        .unwrap();
        assert_eq!(monthly_result.outputs[0].value, d("85"));

        let school_without_months = CalculationPlan {
            version: 1,
            inputs: vec![
                CalculationInput {
                    name: "coursework".into(),
                    unit: score_unit("percent"),
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
                CalculationInput {
                    name: "exam".into(),
                    unit: score_unit("percent"),
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
            ],
            outputs: vec![CalculationOutput {
                name: "final".into(),
                unit: score_unit("percent"),
                expression: AstNode::WeightedSum {
                    items: vec![
                        WeightedItem {
                            node: AstNode::Variable {
                                name: "coursework".into(),
                            },
                            weight: d("0.4"),
                        },
                        WeightedItem {
                            node: AstNode::Variable {
                                name: "exam".into(),
                            },
                            weight: d("0.6"),
                        },
                    ],
                    normalize: true,
                },
                rounding: None,
            }],
        };
        let school_result = AcademicCalculationEngine::calculate_plan(
            &school_without_months,
            &HashMap::from([("coursework".into(), d("80")), ("exam".into(), d("90"))]),
        )
        .unwrap();
        assert_eq!(school_result.outputs[0].value, d("86"));

        let university_gpa = AcademicCalculationEngine::calculate_weighted_average(
            &[
                WeightedMeasure {
                    value: d("3.7"),
                    weight: d("4"),
                    included: true,
                },
                WeightedMeasure {
                    value: d("3.3"),
                    weight: d("2"),
                    included: true,
                },
            ],
            None,
        )
        .unwrap();
        assert_eq!(university_gpa, d("3.5666666666666666666666666667"));

        let competency_unit = CalculationUnit::Custom {
            scale: "competency-level".into(),
        };
        let institute = CalculationPlan {
            version: 1,
            inputs: vec![
                CalculationInput {
                    name: "practical".into(),
                    unit: score_unit("practical-rubric"),
                    minimum: Some(d("1")),
                    maximum: Some(d("5")),
                    is_required: true,
                },
                CalculationInput {
                    name: "competency".into(),
                    unit: competency_unit.clone(),
                    minimum: Some(d("1")),
                    maximum: Some(d("4")),
                    is_required: true,
                },
            ],
            outputs: vec![
                CalculationOutput {
                    name: "practical-result".into(),
                    unit: score_unit("practical-rubric"),
                    expression: AstNode::Variable {
                        name: "practical".into(),
                    },
                    rounding: None,
                },
                CalculationOutput {
                    name: "competency-result".into(),
                    unit: competency_unit,
                    expression: AstNode::Variable {
                        name: "competency".into(),
                    },
                    rounding: None,
                },
            ],
        };
        let institute_result = AcademicCalculationEngine::calculate_plan(
            &institute,
            &HashMap::from([
                ("practical".into(), d("4.5")),
                ("competency".into(), d("3")),
            ]),
        )
        .unwrap();
        assert_eq!(institute_result.outputs[0].value, d("4.5"));
        assert_eq!(
            institute_result.outputs[1].unit,
            CalculationUnit::Custom {
                scale: "competency-level".into(),
            }
        );
        assert_eq!(institute_result.outputs[1].value, d("3"));
    }

    #[test]
    fn typed_plan_requires_explicit_scale_conversion() {
        let source_unit = score_unit("out_of_20");
        let target_unit = score_unit("percentage");
        let plan = CalculationPlan {
            version: 1,
            inputs: vec![CalculationInput {
                name: "exam".into(),
                unit: source_unit.clone(),
                minimum: Some(d("0")),
                maximum: Some(d("20")),
                is_required: true,
            }],
            outputs: vec![CalculationOutput {
                name: "converted".into(),
                unit: target_unit.clone(),
                expression: AstNode::Convert {
                    value: Box::new(AstNode::Variable {
                        name: "exam".into(),
                    }),
                    from: source_unit,
                    to: target_unit,
                    factor: d("5"),
                    offset: Decimal::ZERO,
                },
                rounding: None,
            }],
        };
        let values = HashMap::from([("exam".into(), d("17"))]);

        let result = AcademicCalculationEngine::calculate_plan(&plan, &values).unwrap();
        assert_eq!(result.outputs[0].value, d("85"));
    }

    #[test]
    fn typed_plan_rejects_incompatible_units_unknown_references_and_bad_inputs() {
        let percent = score_unit("percentage");
        let points = score_unit("points");
        let mut plan = CalculationPlan {
            version: 1,
            inputs: vec![
                CalculationInput {
                    name: "left".into(),
                    unit: percent.clone(),
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
                CalculationInput {
                    name: "right".into(),
                    unit: points,
                    minimum: Some(d("0")),
                    maximum: Some(d("100")),
                    is_required: true,
                },
            ],
            outputs: vec![CalculationOutput {
                name: "sum".into(),
                unit: percent,
                expression: AstNode::Add {
                    operands: vec![
                        AstNode::Variable {
                            name: "left".into(),
                        },
                        AstNode::Variable {
                            name: "right".into(),
                        },
                    ],
                    left: None,
                    right: None,
                },
                rounding: None,
            }],
        };
        assert!(AcademicCalculationEngine::validate_plan(&plan).is_err());

        plan.outputs[0].expression = AstNode::Variable {
            name: "missing".into(),
        };
        assert!(AcademicCalculationEngine::validate_plan(&plan).is_err());

        plan.outputs[0].expression = AstNode::Variable {
            name: "left".into(),
        };
        let values = HashMap::from([("left".into(), d("101")), ("right".into(), d("50"))]);
        assert!(AcademicCalculationEngine::calculate_plan(&plan, &values).is_err());
    }

    #[test]
    fn typed_plan_reports_division_by_zero() {
        let score = score_unit("points");
        let plan = CalculationPlan {
            version: 1,
            inputs: vec![
                CalculationInput {
                    name: "score".into(),
                    unit: score.clone(),
                    minimum: None,
                    maximum: None,
                    is_required: true,
                },
                CalculationInput {
                    name: "divisor".into(),
                    unit: CalculationUnit::Dimensionless,
                    minimum: None,
                    maximum: None,
                    is_required: true,
                },
            ],
            outputs: vec![CalculationOutput {
                name: "result".into(),
                unit: score,
                expression: AstNode::Divide {
                    left: Box::new(AstNode::Variable {
                        name: "score".into(),
                    }),
                    right: Box::new(AstNode::Variable {
                        name: "divisor".into(),
                    }),
                },
                rounding: None,
            }],
        };
        let values = HashMap::from([("score".into(), d("10")), ("divisor".into(), Decimal::ZERO)]);

        assert!(AcademicCalculationEngine::calculate_plan(&plan, &values).is_err());
    }

    /// Helper to create a standard school monthly breakdown.
    fn school_breakdown(hw: &str, oral: &str, att: &str, written: &str) -> PeriodComponentScores {
        let mut b = PeriodComponentScores {
            components: vec![
                ComponentScore {
                    code: "HW".into(),
                    name: "Homework".into(),
                    name_ar: None,
                    score: d(hw),
                    max_score: d("20"),
                },
                ComponentScore {
                    code: "ORAL".into(),
                    name: "Oral".into(),
                    name_ar: None,
                    score: d(oral),
                    max_score: d("20"),
                },
                ComponentScore {
                    code: "ATT".into(),
                    name: "Attendance".into(),
                    name_ar: None,
                    score: d(att),
                    max_score: d("20"),
                },
                ComponentScore {
                    code: "EXAM_M".into(),
                    name: "Written".into(),
                    name_ar: None,
                    score: d(written),
                    max_score: d("40"),
                },
            ],
            total: Decimal::ZERO,
            total_max: d("100"),
        };
        b.compute_total();
        b
    }

    /// Helper to wrap a breakdown into a PeriodAssessmentResult.
    fn month_result(
        code: &str,
        name: &str,
        breakdown: PeriodComponentScores,
    ) -> PeriodAssessmentResult {
        PeriodAssessmentResult {
            period_id: Uuid::new_v4(),
            period_code: code.into(),
            period_name: name.into(),
            period_name_ar: None,
            breakdown,
        }
    }

    #[test]
    fn test_monthly_assessment_calculation() {
        let policy = AcademicPolicy::default();
        let breakdown = school_breakdown("18", "17", "19", "35");

        let total =
            AcademicCalculationEngine::calculate_monthly_total(&breakdown, &policy).unwrap();
        assert_eq!(total, d("89.00"));
    }

    #[test]
    fn test_school_term_normalized_calculation_exact_prompt_example() {
        let policy = AcademicPolicy {
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            ..Default::default()
        };

        // Month 1 = 82 (20+20+20+22)
        let m1 = month_result("M1", "Month 1", school_breakdown("20", "20", "20", "22"));
        // Month 2 = 91 (20+20+20+31)
        let m2 = month_result("M2", "Month 2", school_breakdown("20", "20", "20", "31"));
        // Month 3 = 87 (20+20+20+27)
        let m3 = month_result("M3", "Month 3", school_breakdown("20", "20", "20", "27"));

        let final_exam = Some(d("24")); // 24 / 30

        let result = AcademicCalculationEngine::calculate_subject_term_result(
            Uuid::new_v4(),
            "MATH101".into(),
            "Mathematics".into(),
            Some("الرياضيات".into()),
            d("3"),
            vec![m1, m2, m3],
            final_exam,
            &policy,
        )
        .unwrap();

        // Monthly average: (82 + 91 + 87) / 3 = 86.6666... -> rounded to 86.67
        assert_eq!(result.monthly_average, d("86.67"));
        // Normalized contribution: 86.6666... / 100 * 20 = 17.3333... -> rounded to 17.33
        assert_eq!(result.monthly_contribution, d("17.33"));
        // Term total: 17.3333... + 24 = 41.3333... -> rounded to 41.33 / 50
        assert_eq!(result.term_total, d("41.33"));
        assert_eq!(result.term_max, d("50"));
        // Percentage: 41.333... / 50 * 100 = 82.67%
        assert_eq!(result.percentage, d("82.67"));
        assert_eq!(result.status, AcademicStatus::Passed);
        assert_eq!(result.letter_grade, Some("B".to_string()));
    }

    #[test]
    fn test_annual_aggregation() {
        let policy = AcademicPolicy::default();

        let mut t1 = AcademicCalculationEngine::calculate_subject_term_result(
            Uuid::new_v4(),
            "MATH101".into(),
            "Mathematics".into(),
            None,
            d("3"),
            vec![],
            None,
            &policy,
        )
        .unwrap();
        t1.term_total = d("41.33");

        let mut t2 = t1.clone();
        t2.term_total = d("45.00");

        let annual = AcademicCalculationEngine::calculate_subject_annual_result(
            t1.subject_id,
            t1.subject_code.clone(),
            t1.subject_name.clone(),
            None,
            d("3"),
            Some(t1),
            Some(t2),
            &policy,
        )
        .unwrap();

        assert_eq!(annual.annual_total, d("86.33"));
        assert_eq!(annual.percentage, d("86.33"));
        assert_eq!(annual.status, AcademicStatus::PassedWithHonors);
        assert_eq!(annual.letter_grade, Some("B".to_string()));
    }

    #[test]
    fn test_rankings_standard_competition_and_dense() {
        let scores = vec![
            ("StudentA", d("95")),
            ("StudentB", d("90")),
            ("StudentC", d("90")),
            ("StudentD", d("85")),
        ];

        let std_rank =
            AcademicCalculationEngine::compute_rankings(&scores, TiePolicy::StandardCompetition);
        assert_eq!(
            std_rank,
            vec![
                ("StudentA", d("1")),
                ("StudentB", d("2")),
                ("StudentC", d("2")),
                ("StudentD", d("4")),
            ]
        );

        let dense_rank = AcademicCalculationEngine::compute_rankings(&scores, TiePolicy::Dense);
        assert_eq!(
            dense_rank,
            vec![
                ("StudentA", d("1")),
                ("StudentB", d("2")),
                ("StudentC", d("2")),
                ("StudentD", d("3")),
            ]
        );

        let fractional_rank =
            AcademicCalculationEngine::compute_rankings(&scores, TiePolicy::Fractional);
        assert_eq!(
            fractional_rank,
            vec![
                ("StudentA", d("1")),
                ("StudentB", d("2.5")),
                ("StudentC", d("2.5")),
                ("StudentD", d("4")),
            ]
        );
    }

    #[test]
    fn rounding_is_applied_after_aggregation_and_respects_midpoint_policy() {
        let unit = score_unit("percentage");
        let plan = CalculationPlan {
            version: 1,
            inputs: vec![
                CalculationInput {
                    name: "first".into(),
                    unit: unit.clone(),
                    minimum: None,
                    maximum: None,
                    is_required: true,
                },
                CalculationInput {
                    name: "second".into(),
                    unit: unit.clone(),
                    minimum: None,
                    maximum: None,
                    is_required: true,
                },
            ],
            outputs: vec![CalculationOutput {
                name: "average".into(),
                unit,
                expression: AstNode::Average {
                    operands: vec![
                        AstNode::Variable {
                            name: "first".into(),
                        },
                        AstNode::Variable {
                            name: "second".into(),
                        },
                    ],
                },
                rounding: Some(RoundingRule {
                    mode: RoundingMode::RoundHalfUp,
                    decimal_places: 2,
                }),
            }],
        };
        let values = HashMap::from([("first".into(), d("1.004")), ("second".into(), d("1.005"))]);
        let result = AcademicCalculationEngine::calculate_plan(&plan, &values).unwrap();
        assert_eq!(result.outputs[0].value, d("1.00"));
        assert_eq!(
            apply_rounding(d("86.665"), RoundingMode::RoundHalfUp, 2),
            d("86.67")
        );
        assert_eq!(
            apply_rounding(d("1.231"), RoundingMode::RoundUp, 2),
            d("1.24")
        );
        assert_eq!(
            apply_rounding(d("1.239"), RoundingMode::RoundDown, 2),
            d("1.23")
        );
        assert_eq!(
            apply_rounding(d("1.239"), RoundingMode::Truncate, 2),
            d("1.23")
        );
    }

    #[test]
    fn pass_grade_and_honors_rules_are_institution_defined() {
        let unit = score_unit("institutional_points");
        let policy = OutcomePolicy {
            unit: unit.clone(),
            pass_at_or_above: Some(d("42")),
            honors_at_or_above: Some(d("88")),
            grade_bands: vec![
                GradeBand {
                    code: "P".into(),
                    label: "Pass".into(),
                    minimum_score: d("42"),
                },
                GradeBand {
                    code: "M".into(),
                    label: "Merit".into(),
                    minimum_score: d("70"),
                },
                GradeBand {
                    code: "D".into(),
                    label: "Distinction".into(),
                    minimum_score: d("88"),
                },
            ],
        };

        let outcome = AcademicCalculationEngine::evaluate_outcome(d("88"), &unit, &policy).unwrap();
        assert_eq!(outcome.is_passed, Some(true));
        assert_eq!(outcome.is_honors, Some(true));
        assert_eq!(outcome.grade_code.as_deref(), Some("D"));
        let failed =
            AcademicCalculationEngine::evaluate_outcome(d("41.99"), &unit, &policy).unwrap();
        assert_eq!(failed.is_passed, Some(false));
        assert_eq!(failed.grade_code, None);
        let passing_boundary =
            AcademicCalculationEngine::evaluate_outcome(d("42"), &unit, &policy).unwrap();
        assert_eq!(passing_boundary.is_passed, Some(true));
        assert_eq!(passing_boundary.grade_code.as_deref(), Some("P"));
    }

    #[test]
    fn credit_weighted_average_obeys_inclusion_and_rounding() {
        let measures = [
            WeightedMeasure {
                value: d("3.7"),
                weight: d("4"),
                included: true,
            },
            WeightedMeasure {
                value: d("2.8"),
                weight: d("2"),
                included: true,
            },
            WeightedMeasure {
                value: d("0"),
                weight: d("3"),
                included: false,
            },
        ];
        let average = AcademicCalculationEngine::calculate_weighted_average(
            &measures,
            Some(RoundingRule {
                mode: RoundingMode::RoundHalfUp,
                decimal_places: 3,
            }),
        )
        .unwrap();
        assert_eq!(average, d("3.4"));
        assert!(
            AcademicCalculationEngine::calculate_weighted_average(
                &[WeightedMeasure {
                    value: d("4"),
                    weight: Decimal::ZERO,
                    included: true,
                }],
                None,
            )
            .is_err()
        );
    }

    // ═══════════════════════════════════════════════════════════
    // Certificate calculation tests
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn test_cumulative_score_resolution_thanawiya_example() {
        // Worked example from MoE: avg of 80/100 across 3 years → 16/20
        let policy = AcademicPolicy {
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            ..Default::default()
        };
        let prior_scores = vec![d("80"), d("85"), d("75")]; // avg = 80
        let result = AcademicCalculationEngine::resolve_cumulative_score(
            &prior_scores,
            d("20"),  // cumulative_max
            d("100"), // annual_max
            &policy,
        );
        assert_eq!(result, Some(d("16.00")));
    }

    #[test]
    fn test_cumulative_score_empty_returns_none() {
        let policy = AcademicPolicy::default();
        let result = AcademicCalculationEngine::resolve_cumulative_score(
            &[],
            d("20"),
            d("100"),
            &policy,
        );
        assert_eq!(result, None);
    }

    #[test]
    fn test_certificate_result_thanawiya_passed() {
        let policy = AcademicPolicy {
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            certificate_exam_max_score: d("80"),
            certificate_cumulative_max_score: d("20"),
            ..Default::default()
        };
        let result = AcademicCalculationEngine::calculate_certificate_result(
            Uuid::new_v4(),
            "MATH".into(),
            "Mathematics".into(),
            Some("الرياضيات".into()),
            d("1"),
            Some(d("75")), // exam: 75/80
            Some(d("16")), // cumulative: 16/20
            &policy,
        )
        .unwrap();

        assert_eq!(result.term_total, d("91"));  // 75 + 16 = 91
        assert_eq!(result.term_max, d("100"));   // 80 + 20
        assert_eq!(result.percentage, d("91.00"));
        assert_eq!(result.status, AcademicStatus::PassedWithHonors);
    }

    #[test]
    fn test_certificate_result_failed() {
        let policy = AcademicPolicy {
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            certificate_exam_max_score: d("80"),
            certificate_cumulative_max_score: d("20"),
            ..Default::default()
        };
        let result = AcademicCalculationEngine::calculate_certificate_result(
            Uuid::new_v4(),
            "PHYS".into(),
            "Physics".into(),
            None,
            d("1"),
            Some(d("30")), // exam: 30/80
            Some(d("10")), // cumulative: 10/20
            &policy,
        )
        .unwrap();

        assert_eq!(result.term_total, d("40")); // 30 + 10
        assert_eq!(result.percentage, d("40.00"));
        assert_eq!(result.status, AcademicStatus::Failed);
    }

    #[test]
    fn test_certificate_result_incomplete_without_exam() {
        let policy = AcademicPolicy::default();
        let result = AcademicCalculationEngine::calculate_certificate_result(
            Uuid::new_v4(),
            "ARAB".into(),
            "Arabic".into(),
            None,
            d("1"),
            None,          // no exam score
            Some(d("15")),
            &policy,
        )
        .unwrap();

        assert_eq!(result.status, AcademicStatus::Incomplete);
    }
}
