//! Parameter Validation — validates submitted `ReportRequest` parameters
//! against a `ReportDefinition`'s declared parameter schema.
//!
//! Validation is **strict by default**: unknown parameters are rejected,
//! required parameters must be present, and types must match.

use crate::definition::{ParamType, ParameterDef, ReportDefinition};
use crate::error::ReportError;
use crate::request::{ParamValue, ReportRequest};

/// Validate that the parameters in a `ReportRequest` conform to the
/// `ReportDefinition`'s parameter schema.
///
/// Returns `Ok(())` if all validations pass, or the first `ReportError`
/// encountered.
///
/// # Validations Performed
///
/// 1. Every **required** parameter must be present in the request.
/// 2. Every submitted parameter key must exist in the definition.
/// 3. Every submitted value must match the expected `ParamType`.
/// 4. For `Enum` parameters, the value must be in the allowed options.
/// 5. For `DateRange`, both `from` and `to` must be present and `from <= to`.
pub fn validate_parameters(
    request: &ReportRequest,
    definition: &ReportDefinition,
) -> Result<(), ReportError> {
    // 1. Check required parameters are present
    for param_def in &definition.parameters {
        if param_def.required && !request.parameters.contains_key(&param_def.key) {
            // Check if there's a default value
            if param_def.default_value.is_none() {
                return Err(ReportError::MissingParameter(param_def.key.clone()));
            }
        }
    }

    // 2. Check submitted parameters exist in definition and match types
    for (key, value) in &request.parameters {
        let param_def = definition
            .parameters
            .iter()
            .find(|p| p.key == *key)
            .ok_or_else(|| ReportError::InvalidParameter {
                key: key.clone(),
                reason: format!("Unknown parameter '{}'", key),
            })?;

        validate_param_type(key, value, param_def)?;
    }

    Ok(())
}

/// Validate that a single parameter value matches the expected type.
fn validate_param_type(
    key: &str,
    value: &ParamValue,
    def: &ParameterDef,
) -> Result<(), ReportError> {
    let type_matches = match (&def.param_type, value) {
        (ParamType::Text, ParamValue::Text(_)) => true,
        (ParamType::Integer, ParamValue::Integer(_)) => true,
        (ParamType::Decimal, ParamValue::Decimal(_)) => true,
        // Accept integers for decimal params (common in JSON)
        (ParamType::Decimal, ParamValue::Integer(_)) => true,
        (ParamType::Boolean, ParamValue::Boolean(_)) => true,
        (ParamType::Date, ParamValue::Date(_)) => true,
        (ParamType::DateRange, ParamValue::DateRange { .. }) => true,
        (ParamType::Id, ParamValue::Id(_)) => true,
        (ParamType::IdList, ParamValue::IdList(_)) => true,
        (ParamType::Enum, ParamValue::Enum(_)) => true,
        (ParamType::Threshold, ParamValue::Threshold { .. }) => true,
        (ParamType::Period, ParamValue::Period(_)) => true,
        // Also accept Id for Period (they're semantically equivalent)
        (ParamType::Period, ParamValue::Id(_)) => true,
        _ => false,
    };

    if !type_matches {
        return Err(ReportError::InvalidParameter {
            key: key.to_string(),
            reason: format!(
                "Expected type '{:?}', got '{}'",
                def.param_type,
                param_value_type_name(value)
            ),
        });
    }

    // Additional validations per type
    match value {
        ParamValue::Enum(val) => {
            if let Some(options) = &def.options {
                if !options.iter().any(|o| o.value == *val) {
                    let allowed: Vec<&str> = options.iter().map(|o| o.value.as_str()).collect();
                    return Err(ReportError::InvalidParameter {
                        key: key.to_string(),
                        reason: format!(
                            "Value '{}' not in allowed options: [{}]",
                            val,
                            allowed.join(", ")
                        ),
                    });
                }
            }
        }
        ParamValue::DateRange { from, to } => {
            if from > to {
                return Err(ReportError::InvalidParameter {
                    key: key.to_string(),
                    reason: format!("Date range 'from' ({}) must be <= 'to' ({})", from, to),
                });
            }
        }
        ParamValue::IdList(ids) => {
            if ids.is_empty() {
                return Err(ReportError::InvalidParameter {
                    key: key.to_string(),
                    reason: "ID list cannot be empty".to_string(),
                });
            }
        }
        ParamValue::Text(s) => {
            if s.is_empty() && def.required {
                return Err(ReportError::InvalidParameter {
                    key: key.to_string(),
                    reason: "Required text parameter cannot be empty".to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Return a human-readable name for a `ParamValue` variant.
fn param_value_type_name(value: &ParamValue) -> &'static str {
    match value {
        ParamValue::Text(_) => "text",
        ParamValue::Integer(_) => "integer",
        ParamValue::Decimal(_) => "decimal",
        ParamValue::Boolean(_) => "boolean",
        ParamValue::Date(_) => "date",
        ParamValue::DateRange { .. } => "date_range",
        ParamValue::Id(_) => "id",
        ParamValue::IdList(_) => "id_list",
        ParamValue::Enum(_) => "enum",
        ParamValue::Threshold { .. } => "threshold",
        ParamValue::Period(_) => "period",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::*;
    use crate::request::*;
    use std::collections::HashMap;

    fn make_def(params: Vec<ParameterDef>) -> ReportDefinition {
        ReportDefinition {
            code: "test.report".into(),
            name: "Test Report".into(),
            name_ar: "تقرير اختبار".into(),
            domain: ReportDomain::Finance,
            category: "test".into(),
            description: None,
            description_ar: None,
            parameters: params,
            required_permissions: vec![],
            supported_formats: vec![OutputFormatKind::DataOnly],
            default_template_id: None,
            is_system: true,
            is_active: true,
            sort_order: 0,
            version: 1,
        }
    }

    #[test]
    fn test_missing_required_param() {
        let def = make_def(vec![ParameterDef {
            key: "date_range".into(),
            label: "Period".into(),
            label_ar: "الفترة".into(),
            param_type: ParamType::DateRange,
            required: true,
            default_value: None,
            options: None,
            entity_type: None,
            placeholder: None,
            placeholder_ar: None,
            help_text: None,
            help_text_ar: None,
            sort_order: 0,
            group: None,
        }]);

        let request = ReportRequest {
            report_code: "test.report".into(),
            parameters: HashMap::new(),
            output_format: OutputFormat::DataOnly,
            locale: ReportLocale::Ar,
            pagination: None,
        };

        let result = validate_parameters(&request, &def);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ReportError::MissingParameter(_)));
    }

    #[test]
    fn test_type_mismatch() {
        let def = make_def(vec![ParameterDef {
            key: "amount".into(),
            label: "Amount".into(),
            label_ar: "المبلغ".into(),
            param_type: ParamType::Decimal,
            required: false,
            default_value: None,
            options: None,
            entity_type: None,
            placeholder: None,
            placeholder_ar: None,
            help_text: None,
            help_text_ar: None,
            sort_order: 0,
            group: None,
        }]);

        let mut params = HashMap::new();
        params.insert("amount".to_string(), ParamValue::Text("not_a_number".into()));

        let request = ReportRequest {
            report_code: "test.report".into(),
            parameters: params,
            output_format: OutputFormat::DataOnly,
            locale: ReportLocale::Ar,
            pagination: None,
        };

        let result = validate_parameters(&request, &def);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ReportError::InvalidParameter { .. }
        ));
    }

    #[test]
    fn test_valid_params() {
        let def = make_def(vec![
            ParameterDef {
                key: "date_range".into(),
                label: "Period".into(),
                label_ar: "الفترة".into(),
                param_type: ParamType::DateRange,
                required: true,
                default_value: None,
                options: None,
                entity_type: None,
                placeholder: None,
                placeholder_ar: None,
                help_text: None,
                help_text_ar: None,
                sort_order: 0,
                group: None,
            },
            ParameterDef {
                key: "status".into(),
                label: "Status".into(),
                label_ar: "الحالة".into(),
                param_type: ParamType::Enum,
                required: false,
                default_value: None,
                options: Some(vec![
                    ParamOption { value: "paid".into(), label: "Paid".into(), label_ar: "مدفوع".into() },
                    ParamOption { value: "unpaid".into(), label: "Unpaid".into(), label_ar: "غير مدفوع".into() },
                ]),
                entity_type: None,
                placeholder: None,
                placeholder_ar: None,
                help_text: None,
                help_text_ar: None,
                sort_order: 1,
                group: None,
            },
        ]);

        let mut params = HashMap::new();
        params.insert(
            "date_range".to_string(),
            ParamValue::DateRange {
                from: "2025-09-01".into(),
                to: "2026-06-30".into(),
            },
        );
        params.insert("status".to_string(), ParamValue::Enum("paid".into()));

        let request = ReportRequest {
            report_code: "test.report".into(),
            parameters: params,
            output_format: OutputFormat::DataOnly,
            locale: ReportLocale::Ar,
            pagination: None,
        };

        let result = validate_parameters(&request, &def);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_enum_value() {
        let def = make_def(vec![ParameterDef {
            key: "status".into(),
            label: "Status".into(),
            label_ar: "الحالة".into(),
            param_type: ParamType::Enum,
            required: false,
            default_value: None,
            options: Some(vec![
                ParamOption { value: "paid".into(), label: "Paid".into(), label_ar: "مدفوع".into() },
            ]),
            entity_type: None,
            placeholder: None,
            placeholder_ar: None,
            help_text: None,
            help_text_ar: None,
            sort_order: 0,
            group: None,
        }]);

        let mut params = HashMap::new();
        params.insert("status".to_string(), ParamValue::Enum("invalid".into()));

        let request = ReportRequest {
            report_code: "test.report".into(),
            parameters: params,
            output_format: OutputFormat::DataOnly,
            locale: ReportLocale::Ar,
            pagination: None,
        };

        let result = validate_parameters(&request, &def);
        assert!(result.is_err());
    }
}
