use std::sync::Arc;

use serde_json::Value;
use tauri::State;

use crate::commands::error::{CmdResult, CommandError};
use crate::state::AppState;
use shared_kernel::traits::Permission;
use svc_report::definition::ParamType;
use svc_report::request::{OutputFormat, ParamValue, ReportRequest};

fn report_permission(domain: &svc_report::definition::ReportDomain) -> Permission {
    match domain {
        svc_report::definition::ReportDomain::Finance => Permission::FinanceRead,
        svc_report::definition::ReportDomain::Academic => Permission::EnrollmentRead,
        svc_report::definition::ReportDomain::Exam => Permission::ExamSessionRead,
        svc_report::definition::ReportDomain::Staff => Permission::UserRead,
        svc_report::definition::ReportDomain::System => Permission::DashboardRead,
    }
}

fn parameter_value(value: Value, param_type: &ParamType) -> Result<ParamValue, CommandError> {
    match param_type {
        ParamType::Text => value
            .as_str()
            .map(|value| ParamValue::Text(value.to_string()))
            .ok_or_else(|| CommandError::new("Expected text report parameter")),
        ParamType::Integer => value
            .as_i64()
            .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
            .map(ParamValue::Integer)
            .ok_or_else(|| CommandError::new("Expected integer report parameter")),
        ParamType::Decimal => value
            .as_f64()
            .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
            .map(ParamValue::Decimal)
            .ok_or_else(|| CommandError::new("Expected decimal report parameter")),
        ParamType::Boolean => value
            .as_bool()
            .map(ParamValue::Boolean)
            .ok_or_else(|| CommandError::new("Expected boolean report parameter")),
        ParamType::Date => value
            .as_str()
            .map(|value| ParamValue::Date(value.to_string()))
            .ok_or_else(|| CommandError::new("Expected date report parameter")),
        ParamType::Period => value
            .as_str()
            .map(|value| ParamValue::Period(value.to_string()))
            .ok_or_else(|| CommandError::new("Expected period report parameter")),
        ParamType::Id => value
            .as_str()
            .map(|value| ParamValue::Id(value.to_string()))
            .ok_or_else(|| CommandError::new("Expected ID report parameter")),
        ParamType::Enum => value
            .as_str()
            .map(|value| ParamValue::Enum(value.to_string()))
            .ok_or_else(|| CommandError::new("Expected enum report parameter")),
        ParamType::IdList => value
            .as_array()
            .map(|values| {
                ParamValue::IdList(
                    values
                        .iter()
                        .filter_map(Value::as_str)
                        .map(ToString::to_string)
                        .collect(),
                )
            })
            .ok_or_else(|| CommandError::new("Expected ID list report parameter")),
        ParamType::DateRange | ParamType::Threshold => {
            serde_json::from_value(value)
                .map_err(|error| CommandError::new(format!("Invalid report parameter: {error}")))
        }
    }
}

fn build_request(
    service: &svc_report::service::ReportService,
    report_code: String,
    filters: Value,
    format: Option<String>,
) -> Result<ReportRequest, CommandError> {
    let definition = service
        .get_definition(&report_code)
        .map_err(|error| CommandError::new(error.to_string()))?;
    let filter_map = filters
        .as_object()
        .ok_or_else(|| CommandError::new("Report filters must be an object"))?;
    let mut parameters = std::collections::HashMap::new();

    for definition_parameter in &definition.parameters {
        let Some(value) = filter_map.get(&definition_parameter.key) else {
            continue;
        };
        if value.is_null() {
            continue;
        }
        parameters.insert(
            definition_parameter.key.clone(),
            parameter_value(value.clone(), &definition_parameter.param_type)?,
        );
    }

    let output_format = format
        .map(|format| {
            serde_json::from_value::<OutputFormat>(Value::String(format))
                .map_err(|error| CommandError::new(format!("Invalid report format: {error}")))
        })
        .transpose()?
        .unwrap_or_default();

    Ok(ReportRequest {
        report_code,
        parameters,
        output_format,
        locale: Default::default(),
        pagination: None,
    })
}

async fn authorize_report(
    state: &AppState,
    service: &svc_report::service::ReportService,
    report_code: &str,
) -> Result<(), CommandError> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let definition = service
        .get_definition(report_code)
        .map_err(|error| CommandError::new(error.to_string()))?;
    ctx.permissions
        .check(&ctx.actor, report_permission(&definition.domain))
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn reporting_list_reports(
    state: State<'_, Arc<AppState>>,
    domain: Option<String>,
) -> CmdResult<serde_json::Value> {
    let service = state.report_service.clone();
    let domain = domain.and_then(|value| serde_json::from_value(Value::String(value)).ok());
    let reports = service.list_reports(domain.as_ref());
    let mut authorized = Vec::with_capacity(reports.len());
    for report in reports {
        if authorize_report(&state, &service, &report.code).await.is_ok() {
            authorized.push(report);
        }
    }
    serde_json::to_value(authorized).map_err(|error| CommandError::new(error.to_string()))
}

#[tauri::command]
pub async fn reporting_get_report_definition(
    state: State<'_, Arc<AppState>>,
    definition_id: String,
) -> CmdResult<serde_json::Value> {
    let service = state.report_service.clone();
    authorize_report(&state, &service, &definition_id).await?;
    let definition = service
        .get_definition(&definition_id)
        .map_err(|error| CommandError::new(error.to_string()))?;
    serde_json::to_value(definition).map_err(|error| CommandError::new(error.to_string()))
}

#[tauri::command]
pub async fn reporting_query_report(
    state: State<'_, Arc<AppState>>,
    definition_id: String,
    filters: Value,
) -> CmdResult<serde_json::Value> {
    let service = state.report_service.clone();
    authorize_report(&state, &service, &definition_id).await?;
    let request = build_request(&service, definition_id, filters, None)?;
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let org = state.report_org_context(&ctx.org_id).await.map_err(CommandError::from)?;
    let dataset = service
        .generate(&request, &org)
        .await
        .map_err(|error| CommandError::new(error.to_string()))?;
    serde_json::to_value(dataset).map_err(|error| CommandError::new(error.to_string()))
}

#[tauri::command]
pub async fn reporting_export_report(
    state: State<'_, Arc<AppState>>,
    definition_id: String,
    filters: Value,
    format: String,
    _template_id: Option<String>,
) -> CmdResult<Vec<u8>> {
    let service = state.report_service.clone();
    authorize_report(&state, &service, &definition_id).await?;
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    ctx.permissions
        .check(&ctx.actor, Permission::ExportData)
        .await
        .map_err(CommandError::from)?;
    let request = build_request(&service, definition_id, filters, Some(format))?;
    let org = state.report_org_context(&ctx.org_id).await.map_err(CommandError::from)?;
    let result = service
        .export(&request, &org)
        .await
        .map_err(|error| CommandError::new(error.to_string()))?;
    Ok(result.data)
}
