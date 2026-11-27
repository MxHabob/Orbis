//! Native Desktop Solver IPC Commands (Timefold Sidecar Integration).

use crate::features::solver_sidecar::{SidecarStatus, SolverSetupStatus};
use crate::state::AppState;
use orbis_app::ports::gateways::SchedulingSolverPort;
use orbis_domain::entities::solver::{
    ExamProblem, ExamSolveJobStatus, JobStatus, SolveJobStatus, TimetableProblem,
};
use serde::Serialize;
use tauri::State;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
}

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[tauri::command]
pub async fn solver_get_sidecar_status(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SidecarStatus>, String> {
    let status = state.solver_sidecar.status().await;
    Ok(ok(status))
}

#[tauri::command]
pub async fn solver_get_setup_status(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SolverSetupStatus>, String> {
    Ok(ok(state.solver_sidecar.setup_status().await))
}

#[tauri::command]
pub async fn solver_start_setup(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SolverSetupStatus>, String> {
    Ok(ok(state.solver_sidecar.start_setup(app).await))
}

#[tauri::command]
pub async fn solver_stop_sidecar(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SidecarStatus>, String> {
    state.solver_sidecar.stop().await;
    let status = state.solver_sidecar.status().await;
    Ok(ok(status))
}

#[tauri::command]
pub async fn solver_submit_timetable(
    problem: TimetableProblem,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::parse_str(&problem.org_id).unwrap_or_else(|_| Uuid::nil()),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
        source: orbis_app::context::ActionSource::TauriIpc,
        org_unit_path: None,
    };
    let settings = configured_solver_settings(&state, ctx.tenant_id).await?;
    let embedded = uses_embedded_solver(settings.as_ref())?;
    if embedded {
        state
            .solver_sidecar
            .ensure_running()
            .await
            .map_err(|error| error.to_string())?;
        state.solver_sidecar.begin_job(&problem.job_id).await?;
    }

    if let Err(error) = state
        .server_state
        .integration_resolver
        .submit_timetable(&ctx, &problem)
        .await
    {
        if embedded {
            state.solver_sidecar.finish_job(&problem.job_id).await;
        }
        return Err(error.to_string());
    }

    Ok(ok(()))
}

#[tauri::command]
pub async fn solver_get_timetable_status(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SolveJobStatus>, String> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::nil(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
        source: orbis_app::context::ActionSource::TauriIpc,
        org_unit_path: None,
    };
    state.solver_sidecar.mark_job_activity(&job_id).await;

    let status = state
        .server_state
        .integration_resolver
        .get_timetable_status(&ctx, &job_id)
        .await
        .map_err(|e| e.to_string())?;

    if matches!(
        status.status,
        JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled
    ) {
        state.solver_sidecar.finish_job(&job_id).await;
    }

    Ok(ok(status))
}

#[tauri::command]
pub async fn solver_submit_exam(
    problem: ExamProblem,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::parse_str(&problem.org_id).unwrap_or_else(|_| Uuid::nil()),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
        source: orbis_app::context::ActionSource::TauriIpc,
        org_unit_path: None,
    };
    let settings = configured_solver_settings(&state, ctx.tenant_id).await?;
    let embedded = uses_embedded_solver(settings.as_ref())?;
    if embedded {
        state
            .solver_sidecar
            .ensure_running()
            .await
            .map_err(|error| error.to_string())?;
        state.solver_sidecar.begin_job(&problem.job_id).await?;
    }

    if let Err(error) = state
        .server_state
        .integration_resolver
        .submit_exam(&ctx, &problem)
        .await
    {
        if embedded {
            state.solver_sidecar.finish_job(&problem.job_id).await;
        }
        return Err(error.to_string());
    }

    Ok(ok(()))
}

#[tauri::command]
pub async fn solver_get_exam_status(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<ExamSolveJobStatus>, String> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::nil(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
        source: orbis_app::context::ActionSource::TauriIpc,
        org_unit_path: None,
    };
    state.solver_sidecar.mark_job_activity(&job_id).await;

    let status = state
        .server_state
        .integration_resolver
        .get_exam_status(&ctx, &job_id)
        .await
        .map_err(|e| e.to_string())?;

    if matches!(
        status.status,
        JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled
    ) {
        state.solver_sidecar.finish_job(&job_id).await;
    }

    Ok(ok(status))
}

async fn configured_solver_settings(
    state: &AppState,
    org_id: Uuid,
) -> Result<Option<serde_json::Value>, String> {
    for provider_name in ["solver_service", "solver-service"] {
        let config = state
            .server_state
            .integration_service
            .get_integration_by_provider(org_id, provider_name)
            .await
            .map_err(|error| error.to_string())?;
        if let Some(config) = config {
            return Ok(config.is_enabled.then(|| config.settings).flatten());
        }
    }
    Ok(None)
}

fn uses_embedded_solver(settings: Option<&serde_json::Value>) -> Result<bool, String> {
    let Some(settings) = settings else {
        return Ok(true);
    };
    if let Some(endpoint) = settings.get("endpoint").and_then(|value| value.as_str()) {
        if !endpoint.trim().is_empty() {
            return Ok(false);
        }
    }
    let mode = settings
        .get("mode")
        .and_then(|value| value.as_str())
        .unwrap_or("embedded");
    Ok(mode == "embedded")
}
