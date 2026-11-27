use anyhow::{Result, anyhow};
use std::process::Command;

use crate::core::config::{
    CliState, DeployTarget, find_server_binary, get_app_executable_paths, native_license_dir,
};
use crate::core::docker;
use crate::ui::output;

pub async fn execute() -> Result<()> {
    let state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;
    start_api_only(&state).await
}

pub async fn start_api_only(state: &CliState) -> Result<()> {
    match state.target {
        DeployTarget::Docker => {
            output::print_step(&format!("Starting Docker container '{}'...", state.container_name));
            docker::start_or_recreate_api(
                &state.container_name,
                &state.api_image,
                &state.docker_network,
                state.api_port,
                false,
            )?;
            wait_for_health(&state.api_base_url()).await?;
            output::print_success(&format!(
                "Orbis API is running at {}",
                state.api_base_url()
            ));
        }
        DeployTarget::Native => {
            if is_native_service_installed() {
                output::print_step("Starting native Orbis API service...");
                control_native_service("start")?;
                wait_for_health(&state.api_base_url()).await?;
                output::print_success(&format!(
                    "Orbis API service is running at {}",
                    state.api_base_url()
                ));
                return Ok(());
            }

            let bin = find_server_binary().ok_or_else(|| {
                anyhow!("Could not locate `orbis-server` binary. Install the API package or build it first.")
            })?;
            let cfg_path = orbis_infra::config::server_config_path()
                .map_err(anyhow::Error::msg)?;
            let lic_dir = native_license_dir();
            std::fs::create_dir_all(&lic_dir)?;

            output::print_step(&format!("Starting native API server ({})...", bin.display()));
            let child = Command::new(bin)
                .env("BIND_HOST", "127.0.0.1")
                .env("PORT", state.api_port.to_string())
                .env("ORBIS_SERVER_CONFIG_FILE", cfg_path)
                .env("ORBIS_LICENSE_DATA_DIR", lic_dir)
                .spawn()
                .map_err(|e| anyhow!("Failed to spawn orbis-server: {e}"))?;

            wait_for_health(&state.api_base_url()).await?;
            output::print_success(&format!(
                "Orbis API is running at {} (PID: {})",
                state.api_base_url(),
                child.id()
            ));
        }
    }
    Ok(())
}

pub async fn stop() -> Result<()> {
    let state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;
    match state.target {
        DeployTarget::Docker => {
            docker::stop_api(&state.container_name)?;
            output::print_success("Stopped Orbis API container.");
        }
        DeployTarget::Native => {
            if is_native_service_installed() {
                control_native_service("stop")?;
                output::print_success("Stopped native Orbis API service.");
            } else {
                output::print_info("No OS service registered; stop the `orbis-server` process directly.");
            }
        }
    }
    Ok(())
}

pub async fn restart() -> Result<()> {
    let state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;
    match state.target {
        DeployTarget::Docker => {
            docker::restart_api(&state.container_name)?;
            wait_for_health(&state.api_base_url()).await?;
            output::print_success("Restarted Orbis API container.");
        }
        DeployTarget::Native => {
            if is_native_service_installed() {
                control_native_service("restart")?;
                wait_for_health(&state.api_base_url()).await?;
                output::print_success("Restarted native Orbis API service.");
            } else {
                output::print_info("No OS service registered; restart the `orbis-server` process directly.");
            }
        }
    }
    Ok(())
}

pub async fn logs(lines: u32) -> Result<()> {
    let state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;
    match state.target {
        DeployTarget::Docker => docker::tail_logs(&state.container_name, lines),
        DeployTarget::Native => {
            #[cfg(target_os = "linux")]
            if is_native_service_installed() {
                Command::new("journalctl")
                    .args(["-u", "orbis-api.service", "-n", &lines.to_string(), "-f"])
                    .status()?;
                return Ok(());
            }
            #[cfg(target_os = "windows")]
            if let Ok(pd) = std::env::var("PROGRAMDATA") {
                let logs_dir = std::path::PathBuf::from(pd).join("Orbis").join("logs");
                output::print_info(&format!("Windows service logs directory: {}", logs_dir.display()));
                return Ok(());
            }
            output::print_info("Native logs are emitted to stdout/stderr of `orbis-server`.");
            Ok(())
        }
    }
}

fn is_native_service_installed() -> bool {
    #[cfg(target_os = "windows")]
    {
        return Command::new("sc.exe")
            .args(["query", "OrbisApi"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
    }
    #[cfg(target_os = "linux")]
    {
        return Command::new("systemctl")
            .args(["cat", "orbis-api.service"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
    }
    #[allow(unreachable_code)]
    false
}

fn control_native_service(action: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let cmd = match action {
            "start" => "Start-Service -Name OrbisApi",
            "stop" => "Stop-Service -Name OrbisApi",
            "restart" => "Restart-Service -Name OrbisApi",
            _ => return Err(anyhow!("Unsupported service action: {action}")),
        };
        let status = Command::new("powershell")
            .args(["-NoProfile", "-Command", cmd])
            .status()?;
        if !status.success() {
            return Err(anyhow!("Failed to {action} Windows service OrbisApi (run as Administrator)."));
        }
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    {
        let status = Command::new("systemctl")
            .args([action, "orbis-api.service"])
            .status()?;
        if !status.success() {
            return Err(anyhow!("Failed to {action} orbis-api.service (run with sudo)."));
        }
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err(anyhow!("Native service control is supported on Windows and Linux only."))
}

pub async fn upgrade(new_image: Option<String>) -> Result<()> {
    let mut state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;
    if state.target != DeployTarget::Docker {
        return Err(anyhow!("`orbis upgrade` is currently supported for Docker targets."));
    }

    if let Some(img) = new_image {
        state.api_image = img;
        state.save()?;
    }

    output::print_step(&format!("Pulling {}...", state.api_image));
    docker::pull_image(&state.api_image)?;

    output::print_step("Stopping API and running migrations...");
    docker::stop_api(&state.container_name)?;
    docker::run_migrations(&state.api_image, &state.docker_network)?;

    output::print_step("Recreating API container...");
    docker::start_or_recreate_api(
        &state.container_name,
        &state.api_image,
        &state.docker_network,
        state.api_port,
        true,
    )?;
    wait_for_health(&state.api_base_url()).await?;
    output::print_success("Orbis API upgraded and healthy.");
    Ok(())
}

pub fn launch_desktop() -> Result<()> {
    let found = get_app_executable_paths().into_iter().find(|p| p.exists());
    match found {
        Some(path) => {
            let child = Command::new(&path)
                .spawn()
                .map_err(|e| anyhow!("Failed to launch {}: {e}", path.display()))?;
            output::print_success(&format!("Desktop app launched (PID: {})", child.id()));
            Ok(())
        }
        None => Err(anyhow!("Could not find Orbis Desktop executable.")),
    }
}

async fn wait_for_health(base_url: &str) -> Result<()> {
    let url = format!("{base_url}/api/v1/health");
    let client = crate::core::config::http_client();

    output::print_progress_pending(&format!("Waiting for API health at {url}..."));
    for _ in 0..20 {
        if let Ok(resp) = client.get(&url).send().await
            && resp.status().is_success()
        {
            output::print_progress_ok("API health check passed");
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    output::print_warning("API did not respond to /api/v1/health within 10s — check `orbis logs`.");
    Ok(())
}
