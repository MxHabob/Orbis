use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentRecordStatus {
    NotRecorded,
    Submitted,
    Absent,
    Excused,
    Exempt,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaleOption {
    pub code: String,
    pub label: String,
    pub label_ar: Option<String>,
    pub rank: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AssessmentScale {
    Numeric {
        minimum: Decimal,
        maximum: Decimal,
        decimal_places: u32,
    },
    Categorical {
        options: Vec<ScaleOption>,
    },
    Competency {
        levels: Vec<ScaleOption>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AssessmentValue {
    Numeric { value: Decimal },
    Categorical { code: String },
    Competency { code: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssessmentItem {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub scale: AssessmentScale,
    pub is_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssessmentInstrument {
    pub id: uuid::Uuid,
    pub org_id: uuid::Uuid,
    pub code: String,
    pub name: String,
    pub offering_id: Option<uuid::Uuid>,
    pub period_id: Option<uuid::Uuid>,
    pub components: Vec<AssessmentItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssessmentRecord {
    pub status: AssessmentRecordStatus,
    pub value: Option<AssessmentValue>,
}

impl AssessmentScale {
    pub fn validate(&self) -> Result<(), DomainError> {
        match self {
            Self::Numeric {
                minimum,
                maximum,
                decimal_places,
            } => {
                if maximum < minimum {
                    return Err(DomainError::Validation(
                        "Numeric assessment scale maximum must be greater than or equal to minimum"
                            .into(),
                    ));
                }
                if *decimal_places > 28 {
                    return Err(DomainError::Validation(
                        "Numeric assessment scale supports at most 28 decimal places".into(),
                    ));
                }
            }
            Self::Categorical { options } => validate_options(options, false)?,
            Self::Competency { levels } => validate_options(levels, true)?,
        }
        Ok(())
    }

    pub fn validate_value(&self, value: &AssessmentValue) -> Result<(), DomainError> {
        self.validate()?;
        match (self, value) {
            (
                Self::Numeric {
                    minimum,
                    maximum,
                    decimal_places,
                },
                AssessmentValue::Numeric { value },
            ) => {
                if value < minimum || value > maximum {
                    return Err(DomainError::Validation(format!(
                        "Numeric assessment value must be between {minimum} and {maximum}"
                    )));
                }
                if value.normalize().scale() > *decimal_places {
                    return Err(DomainError::Validation(format!(
                        "Numeric assessment value exceeds the scale precision of {decimal_places} decimal places"
                    )));
                }
            }
            (Self::Categorical { options }, AssessmentValue::Categorical { code })
            | (Self::Competency { levels: options }, AssessmentValue::Competency { code }) => {
                if !options.iter().any(|option| option.code == *code) {
                    return Err(DomainError::Validation(format!(
                        "Assessment value code '{}' is not defined by the selected scale",
                        code
                    )));
                }
            }
            _ => {
                return Err(DomainError::Validation(
                    "Assessment value type does not match the declared measurement scale".into(),
                ));
            }
        }
        Ok(())
    }
}

impl AssessmentItem {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.code.trim().is_empty() {
            return Err(DomainError::Validation(
                "Assessment item code is required".into(),
            ));
        }
        if self.name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Assessment item name is required".into(),
            ));
        }
        self.scale.validate()
    }

    pub fn validate_record(&self, record: &AssessmentRecord) -> Result<(), DomainError> {
        self.validate()?;
        match (&record.status, &record.value) {
            (AssessmentRecordStatus::Submitted, Some(value))
            | (AssessmentRecordStatus::Incomplete, Some(value)) => self.scale.validate_value(value),
            (AssessmentRecordStatus::Submitted, None) => Err(DomainError::Validation(
                "A submitted assessment record must contain a value".into(),
            )),
            (
                AssessmentRecordStatus::NotRecorded
                | AssessmentRecordStatus::Absent
                | AssessmentRecordStatus::Excused
                | AssessmentRecordStatus::Exempt,
                Some(_),
            ) => Err(DomainError::Validation(
                "This assessment record status cannot contain a value".into(),
            )),
            _ => Ok(()),
        }
    }
}

impl AssessmentInstrument {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.code.trim().is_empty() || self.name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Assessment instrument code and name are required".into(),
            ));
        }
        if self.components.is_empty() {
            return Err(DomainError::Validation(
                "Assessment instrument must define at least one component".into(),
            ));
        }

        let mut component_codes = std::collections::HashSet::new();
        for component in &self.components {
            component.validate()?;
            if !component_codes.insert(component.code.as_str()) {
                return Err(DomainError::Validation(format!(
                    "Assessment component code '{}' is duplicated",
                    component.code
                )));
            }
        }
        Ok(())
    }
}

fn validate_options(options: &[ScaleOption], require_rank: bool) -> Result<(), DomainError> {
    if options.is_empty() {
        return Err(DomainError::Validation(
            "Categorical and competency scales must define at least one value".into(),
        ));
    }

    let mut codes = std::collections::HashSet::new();
    let mut ranks = std::collections::HashSet::new();
    for option in options {
        let code = option.code.trim();
        if code.is_empty() || code != option.code || !codes.insert(code) {
            return Err(DomainError::Validation(
                "Scale option codes must be non-empty and unique".into(),
            ));
        }
        if option.label.trim().is_empty() {
            return Err(DomainError::Validation(
                "Scale option labels are required".into(),
            ));
        }
        if require_rank && option.rank.is_none() {
            return Err(DomainError::Validation(
                "Every competency level must have an explicit rank".into(),
            ));
        }
        if let Some(rank) = option.rank
            && !ranks.insert(rank)
        {
            return Err(DomainError::Validation(
                "Scale option ranks must be unique when specified".into(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        AssessmentInstrument, AssessmentItem, AssessmentRecord, AssessmentRecordStatus,
        AssessmentScale, AssessmentValue, ScaleOption,
    };
    use rust_decimal::Decimal;

    fn numeric_item() -> AssessmentItem {
        AssessmentItem {
            code: "PRACTICAL".into(),
            name: "Practical task".into(),
            name_ar: None,
            scale: AssessmentScale::Numeric {
                minimum: Decimal::ZERO,
                maximum: Decimal::new(20, 0),
                decimal_places: 2,
            },
            is_required: true,
        }
    }

    fn option(code: &str, rank: Option<i32>) -> ScaleOption {
        ScaleOption {
            code: code.into(),
            label: code.into(),
            label_ar: None,
            rank,
        }
    }

    #[test]
    fn numeric_values_are_inclusive_and_respect_scale_precision() {
        let item = numeric_item();
        for value in [Decimal::ZERO, Decimal::new(20, 0), Decimal::new(1234, 2)] {
            assert!(
                item.scale
                    .validate_value(&AssessmentValue::Numeric { value })
                    .is_ok()
            );
        }
        assert!(
            item.scale
                .validate_value(&AssessmentValue::Numeric {
                    value: Decimal::new(2001, 2)
                })
                .is_err()
        );
        assert!(
            item.scale
                .validate_value(&AssessmentValue::Numeric {
                    value: Decimal::new(1, 3)
                })
                .is_err()
        );
    }

    #[test]
    fn categorical_and_competency_values_must_match_defined_codes() {
        let categorical = AssessmentScale::Categorical {
            options: vec![option("pass", None), option("revise", None)],
        };
        assert!(
            categorical
                .validate_value(&AssessmentValue::Categorical {
                    code: "pass".into()
                })
                .is_ok()
        );
        assert!(
            categorical
                .validate_value(&AssessmentValue::Categorical {
                    code: "unknown".into()
                })
                .is_err()
        );
        assert!(
            categorical
                .validate_value(&AssessmentValue::Numeric {
                    value: Decimal::ONE
                })
                .is_err()
        );

        let competency = AssessmentScale::Competency {
            levels: vec![option("developing", Some(1)), option("mastered", Some(2))],
        };
        assert!(
            competency
                .validate_value(&AssessmentValue::Competency {
                    code: "mastered".into()
                })
                .is_ok()
        );
        assert!(competency.validate().is_ok());
        assert!(
            AssessmentScale::Competency {
                levels: vec![option("mastered", None)]
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn record_statuses_do_not_turn_absence_or_exemption_into_zero() {
        let item = numeric_item();
        for status in [
            AssessmentRecordStatus::NotRecorded,
            AssessmentRecordStatus::Absent,
            AssessmentRecordStatus::Excused,
            AssessmentRecordStatus::Exempt,
        ] {
            assert!(
                item.validate_record(&AssessmentRecord {
                    status,
                    value: None
                })
                .is_ok()
            );
        }
        assert!(
            item.validate_record(&AssessmentRecord {
                status: AssessmentRecordStatus::Submitted,
                value: None
            })
            .is_err()
        );
        assert!(
            item.validate_record(&AssessmentRecord {
                status: AssessmentRecordStatus::Absent,
                value: Some(AssessmentValue::Numeric {
                    value: Decimal::ZERO
                })
            })
            .is_err()
        );
    }

    #[test]
    fn instruments_validate_component_identity_and_allow_period_free_context() {
        let component = AssessmentItem {
            code: "SKILL".into(),
            name: "Skill demonstration".into(),
            name_ar: None,
            scale: AssessmentScale::Competency {
                levels: vec![option("mastered", Some(1))],
            },
            is_required: true,
        };
        let instrument = AssessmentInstrument {
            id: uuid::Uuid::new_v4(),
            org_id: uuid::Uuid::new_v4(),
            code: "PRACTICE".into(),
            name: "Practical assessment".into(),
            offering_id: None,
            period_id: None,
            components: vec![component.clone()],
        };
        assert!(instrument.validate().is_ok());

        let duplicate = AssessmentInstrument {
            components: vec![component.clone(), component],
            ..instrument
        };
        assert!(duplicate.validate().is_err());
    }
}
