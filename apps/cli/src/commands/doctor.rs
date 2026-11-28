use anyhow::Result;
use colored::*;
use orbis_app::services::SystemService;
use orbis_infra::repositories::system_repo::SystemRepoImpl;
use std::sync::Arc;

use crate::core::config::{CliState, DeployTarget, find_server_binary, http_client};
use crate::core::{database, docker, license};
use crate::ui::output::{self, COLOR_BORDER, COLOR_EMERALD, COLOR_MUTED, COLOR_ROSE};

pub async fn execute() -> Result<()> {
    output::print_step("Running Comprehensive System Diagnostics...");
    output::print_divider();

    let border = |s: &str| s.truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2);

    let Some(state) = CliState::load() else {
        output::print_error("Orbis CLI is not initialized. Run `orbis init` first.");
        return Ok(());
    };

    let db_url = state.host_database_url();
    let health_url = format!("{}/api/v1/health", state.api_base_url());

    // Run independent diagnostics concurrently for fast execution
    let runtime_fut = tokio::task::spawn_blocking({
        let target = state.target;
        let container_name = state.container_name.clone();
        move || match target {
            DeployTarget::Docker => {
                let daemon = docker::is_docker_available();
                let running = daemon && docker::container_running(&container_name);
                vec![
                    (
                        "Docker Daemon",
                        if daemon {
                            "Running".into()
                        } else {
                            "Unavailable".into()
                        },
                        daemon,
                    ),
                    (
                        "API Container",
                        if running {
                            format!("Running ({container_name})")
                        } else {
                            format!("Stopped ({container_name})")
                        },
                        running,
                    ),
                ]
            }
            DeployTarget::Native => {
                let bin = find_server_binary();
                vec![(
                    "Server Binary",
                    bin.as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "Not found".into()),
                    bin.is_some(),
                )]
            }
        }
    });

    let db_fut = async {
        let mut out = Vec::new();
        if let Ok(db) = database::fast_connect(&db_url).await {
            out.push((
                "Database Store",
                format!("Connected ({})", state.db_provider.label()),
                true,
            ));

            match database::pending_migrations_count(&db_url).await {
                Ok(0) => out.push(("Schema Migrations", "Up to date".into(), true)),
                Ok(n) => out.push((
                    "Schema Migrations",
                    format!("{n} pending migration(s)"),
                    false,
                )),
                Err(e) => out.push(("Schema Migrations", format!("Error: {e}"), false)),
            }

            let system_repo = Arc::new(SystemRepoImpl { db });
            let system_service = SystemService::new(system_repo);
            match system_service.get_readiness().await {
                Ok(r) => {
                    out.push((
                        "Organization",
                        if r.has_organization {
                            r.org_name.unwrap_or_else(|| "Configured".into())
                        } else {
                            "Not initialized (complete onboarding)".into()
                        },
                        r.has_organization,
                    ));
                    out.push((
                        "Admin Account",
                        if r.has_admin {
                            "Provisioned".into()
                        } else {
                            "Pending".into()
                        },
                        r.has_admin,
                    ));
                }
                Err(e) => out.push(("System Readiness", format!("Query error: {e}"), false)),
            }
        } else {
            out.push(("Database Store", "Connection Failed".into(), false));
        }
        out
    };

    let lic_fut = async {
        match license::get_status(&state).await {
            Ok(dto) => (
                "License State",
                format!(
                    "{:?} ({})",
                    dto.state,
                    dto.plan_name.as_deref().unwrap_or("None")
                ),
                dto.is_operational,
            ),
            Err(e) => ("License State", format!("Error: {e}"), false),
        }
    };

    let http_fut = async {
        let ok = http_client()
            .get(&health_url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);
        (
            "API Health Endpoint",
            if ok {
                health_url.clone()
            } else {
                format!("Unreachable ({health_url})")
            },
            ok,
        )
    };

    let (runtime_res, db_checks, lic_check, http_check) =
        tokio::join!(runtime_fut, db_fut, lic_fut, http_fut);

    let mut checks = runtime_res.unwrap_or_default();
    checks.extend(db_checks);
    checks.push(lic_check);
    checks.push(http_check);

    let all_good = checks.iter().all(|(_, _, ok)| *ok);

    println!();
    for (name, detail, ok) in &checks {
        let icon = if *ok {
            "✔"
                .truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2)
                .bold()
        } else {
            "✖"
                .truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2)
                .bold()
        };
        let label = format!("{:<22}", name);
        let detail_col = if *ok {
            detail.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
        } else {
            detail.truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2)
        };
        println!("  {} {} {} {}", icon, label.bold(), border("│"), detail_col);
    }
    println!();

    if all_good {
        output::print_success("All diagnostics passed — Orbis is healthy.");
    } else {
        output::print_warning("Some checks need attention. See details above.");
    }
    Ok(())
}
