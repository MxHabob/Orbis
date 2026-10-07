#[allow(unused_imports)] // ComponentScore used by tests via super::*
use orbis_domain::entities::academic_policy::{
    AcademicPolicy, AcademicStatus, ComponentScore, PeriodAssessmentResult,
    PeriodComponentScores, RoundingMode, SubjectAnnualResult, SubjectTermResult, TiePolicy,
};
use orbis_domain::errors::DomainError;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Defines the Abstract Syntax Tree (AST) for custom calculation formulas using exact Decimal precision.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AstNode {
    /// A literal numeric value
    Literal { value: Decimal },
    /// A variable representing a component score
    Variable { name: String },
    /// Addition of multiple nodes
    Add {
        #[serde(default)]
        operands: Vec<AstNode>,
        #[serde(default)]
        left: Option<Box<AstNode>>,
        #[serde(default)]
        right: Option<Box<AstNode>>,
    },
    /// Subtraction of two nodes
    Subtract {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    /// Multiplication of multiple nodes
    Multiply {
        #[serde(default)]
        operands: Vec<AstNode>,
        #[serde(default)]
        left: Option<Box<AstNode>>,
        #[serde(default)]
        right: Option<Box<AstNode>>,
    },
    /// Division of two nodes
    Divide {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
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
    pub weight: Decimal,
}

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
                let mut sum = Decimal::ZERO;
                for op in &ops {
                    sum += self.evaluate_inner(op, depth + 1)?;
                }
                Ok(sum)
            }
            AstNode::Subtract { left, right } => {
                let l = self.evaluate_inner(left, depth + 1)?;
                let r = self.evaluate_inner(right, depth + 1)?;
                Ok(l - r)
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
                let mut prod = Decimal::ONE;
                for op in &ops {
                    prod *= self.evaluate_inner(op, depth + 1)?;
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
                Ok(l / r)
            }
            AstNode::Max { operands } => {
                let mut max_val: Option<Decimal> = None;
                for op in operands {
                    let val = self.evaluate_inner(op, depth + 1)?;
                    max_val = Some(match max_val {
                        Some(current) => current.max(val),
                        None => val,
                    });
                }
                Ok(max_val.unwrap_or(Decimal::ZERO))
            }
            AstNode::Min { operands } => {
                let mut min_val: Option<Decimal> = None;
                for op in operands {
                    let val = self.evaluate_inner(op, depth + 1)?;
                    min_val = Some(match min_val {
                        Some(current) => current.min(val),
                        None => val,
                    });
                }
                Ok(min_val.unwrap_or(Decimal::ZERO))
            }
            AstNode::Average { operands } => {
                if operands.is_empty() {
                    return Ok(Decimal::ZERO);
                }
                let mut sum = Decimal::ZERO;
                for op in operands {
                    sum += self.evaluate_inner(op, depth + 1)?;
                }
                let count = Decimal::from(operands.len());
                Ok(sum / count)
            }
            AstNode::WeightedSum { items } => {
                let mut sum = Decimal::ZERO;
                for item in items {
                    let val = self.evaluate_inner(&item.node, depth + 1)?;
                    sum += val * item.weight;
                }
                Ok(sum)
            }
        }
    }
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
    /// Calculate period total from generic component scores.
    /// Each component score is capped at its max before summing.
    pub fn calculate_monthly_total(
        breakdown: &PeriodComponentScores,
        policy: &AcademicPolicy,
    ) -> Decimal {
        let raw_total: Decimal = breakdown
            .components
            .iter()
            .map(|c| c.score.min(c.max_score))
            .sum();

        apply_rounding(raw_total, policy.rounding_mode, policy.decimal_places)
    }

    /// Calculate term subject result:
    /// 1. Average of available monthly periods (out of total_max).
    /// 2. Normalized monthly contribution out of 20: `(Monthly Avg / total_max) * 20`.
    /// 3. Term Final Exam score (out of 30).
    /// 4. Term Total (out of 50) = Monthly Contribution (20) + Term Final Exam (30).
    pub fn calculate_subject_term_result(
        subject_id: uuid::Uuid,
        subject_code: String,
        subject_name: String,
        subject_name_ar: Option<String>,
        credit_hours: Decimal,
        monthly_results: Vec<PeriodAssessmentResult>,
        term_final_exam: Option<Decimal>,
        policy: &AcademicPolicy,
    ) -> SubjectTermResult {
        let mut monthly_totals = Vec::new();
        let mut monthly_total_max = Decimal::from(100); // Default, overridden by actual breakdown

        for par in &monthly_results {
            let total = Self::calculate_monthly_total(&par.breakdown, policy);
            monthly_totals.push(total);
            monthly_total_max = par.breakdown.total_max;
        }

        let count = monthly_totals.len();
        let monthly_avg = if count > 0 {
            let sum: Decimal = monthly_totals.iter().sum();
            sum / Decimal::from(count)
        } else {
            Decimal::ZERO
        };

        // Normalize month avg (0..total_max) to 20 points
        let monthly_contribution = if monthly_total_max > Decimal::ZERO {
            (monthly_avg / monthly_total_max) * Decimal::from(20)
        } else {
            Decimal::ZERO
        };
        let final_exam_score = term_final_exam
            .unwrap_or(Decimal::ZERO)
            .min(Decimal::from(30));

        let raw_term_total = monthly_contribution + final_exam_score;
        let term_total =
            apply_rounding(raw_term_total, policy.rounding_mode, policy.decimal_places);

        // Percentage out of 50 computed from unrounded raw value
        let raw_percentage = (raw_term_total / Decimal::from(50)) * Decimal::from(100);
        let percentage =
            apply_rounding(raw_percentage, policy.rounding_mode, policy.decimal_places);

        let status = if percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        let letter_grade = Self::resolve_letter_grade(percentage);

        SubjectTermResult {
            subject_id,
            subject_code,
            subject_name,
            subject_name_ar,
            credit_hours,
            monthly_results,
            monthly_average: apply_rounding(
                monthly_avg,
                policy.rounding_mode,
                policy.decimal_places,
            ),
            monthly_contribution: apply_rounding(
                monthly_contribution,
                policy.rounding_mode,
                policy.decimal_places,
            ),
            monthly_contribution_max: Decimal::from(20),
            term_final_exam,
            term_final_exam_max: Decimal::from(30),
            term_total,
            term_max: Decimal::from(50),
            percentage,
            letter_grade: Some(letter_grade),
            status,
        }
    }

    /// Calculate subject annual result:
    /// Combine Term 1 (out of 50) + Term 2 (out of 50) = Annual Total (out of 100).
    pub fn calculate_subject_annual_result(
        subject_id: uuid::Uuid,
        subject_code: String,
        subject_name: String,
        subject_name_ar: Option<String>,
        credit_hours: Decimal,
        term_1_result: Option<SubjectTermResult>,
        term_2_result: Option<SubjectTermResult>,
        policy: &AcademicPolicy,
    ) -> SubjectAnnualResult {
        let t1_score = term_1_result
            .as_ref()
            .map(|r| r.term_total)
            .unwrap_or(Decimal::ZERO);
        let t2_score = term_2_result
            .as_ref()
            .map(|r| r.term_total)
            .unwrap_or(Decimal::ZERO);

        let raw_annual_total = t1_score + t2_score;
        let annual_total = apply_rounding(
            raw_annual_total,
            policy.rounding_mode,
            policy.decimal_places,
        );

        let percentage = annual_total; // Since max is 100
        let status = if percentage >= policy.honors_threshold_percentage {
            AcademicStatus::PassedWithHonors
        } else if percentage >= policy.passing_threshold_percentage {
            AcademicStatus::Passed
        } else {
            AcademicStatus::Failed
        };

        let letter_grade = Self::resolve_letter_grade(percentage);

        SubjectAnnualResult {
            subject_id,
            subject_code,
            subject_name,
            subject_name_ar,
            credit_hours,
            term_1_result,
            term_2_result,
            annual_total,
            annual_max: Decimal::from(100),
            percentage,
            letter_grade: Some(letter_grade),
            status,
        }
    }

    /// Compute ranking for a list of items with their scores.
    /// Returns a list of (Item ID, Rank, Score).
    pub fn compute_rankings<T: Copy + Eq + std::hash::Hash>(
        items: &[(T, Decimal)],
        tie_policy: TiePolicy,
    ) -> Vec<(T, u32)> {
        let mut sorted = items.to_vec();
        sorted.sort_by(|a, b| b.1.cmp(&a.1)); // Descending score

        let mut results: Vec<(T, u32)> = Vec::new();
        let mut dense_rank = 0u32;

        for i in 0..sorted.len() {
            let (id, score) = sorted[i];
            if i > 0 && score == sorted[i - 1].1 {
                match tie_policy {
                    TiePolicy::StandardCompetition => {
                        let prev_rank = results[i - 1].1;
                        results.push((id, prev_rank));
                    }
                    TiePolicy::Dense => {
                        results.push((id, dense_rank));
                    }
                    TiePolicy::Fractional => {
                        let prev_rank = results[i - 1].1;
                        results.push((id, prev_rank));
                    }
                }
            } else {
                dense_rank += 1;
                let current_rank = (i + 1) as u32;
                let rank = match tie_policy {
                    TiePolicy::Dense => dense_rank,
                    _ => current_rank,
                };
                results.push((id, rank));
            }
        }

        results
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
    use uuid::Uuid;

    fn d(s: &str) -> Decimal {
        Decimal::from_str_exact(s).unwrap()
    }

    /// Helper to create a standard school monthly breakdown.
    fn school_breakdown(hw: &str, oral: &str, att: &str, written: &str) -> PeriodComponentScores {
        let mut b = PeriodComponentScores {
            components: vec![
                ComponentScore { code: "HW".into(), name: "Homework".into(), name_ar: None, score: d(hw), max_score: d("20") },
                ComponentScore { code: "ORAL".into(), name: "Oral".into(), name_ar: None, score: d(oral), max_score: d("20") },
                ComponentScore { code: "ATT".into(), name: "Attendance".into(), name_ar: None, score: d(att), max_score: d("20") },
                ComponentScore { code: "EXAM_M".into(), name: "Written".into(), name_ar: None, score: d(written), max_score: d("40") },
            ],
            total: Decimal::ZERO,
            total_max: d("100"),
        };
        b.compute_total();
        b
    }

    /// Helper to wrap a breakdown into a PeriodAssessmentResult.
    fn month_result(code: &str, name: &str, breakdown: PeriodComponentScores) -> PeriodAssessmentResult {
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

        let total = AcademicCalculationEngine::calculate_monthly_total(&breakdown, &policy);
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
        );

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
        );
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
        );

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
                ("StudentA", 1),
                ("StudentB", 2),
                ("StudentC", 2),
                ("StudentD", 4), // skips 3
            ]
        );

        let dense_rank = AcademicCalculationEngine::compute_rankings(&scores, TiePolicy::Dense);
        assert_eq!(
            dense_rank,
            vec![
                ("StudentA", 1),
                ("StudentB", 2),
                ("StudentC", 2),
                ("StudentD", 3), // dense next is 3
            ]
        );
    }
}
