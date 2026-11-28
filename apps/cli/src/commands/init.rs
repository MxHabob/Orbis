use anyhow::{Result, anyhow};
use colored::*;
use std::path::PathBuf;

use crate::core::config::{
    CliState, DEFAULT_API_IMAGE, DEFAULT_API_PORT, DEFAULT_POSTGRES_IMAGE, DbProvider,
    DeployTarget, default_sqlite_url, ensure_server_config_file,
};
use crate::core::{database, docker, license};
use crate::ui::{output, prompts};

const TOTAL_PHASES: u8 = 5;

pub async fn execute() -> Result<()> {
    println!(
        "\n  Welcome to the {}. Let's configure your Orbis environment.\n",
        "Orbis Setup Wizard"
            .truecolor(
                output::COLOR_PURPLE.0,
                output::COLOR_PURPLE.1,
                output::COLOR_PURPLE.2
            )
            .bold()
    );

    let mut state = CliState::load().unwrap_or_default();

    // ── Phase 1: Deployment Target ────────────────────────────────────
    output::print_phase_header(1, TOTAL_PHASES, "Deployment Target");

    let docker_ok = docker::is_docker_available();
    let target_options = if docker_ok {
        vec![
            "1. Docker Container (Recommended for Server / Production)",
            "2. Native Host (Windows / Local Binary)",
        ]
    } else {
        output::print_warning("Docker daemon not detected — defaulting to Native Host.");
        vec![
            "2. Native Host (Windows / Local Binary)",
            "1. Docker Container (Requires Docker Engine running)",
        ]
    };

    let target_choice = prompts::prompt_select("Select deployment target:", target_options)?;
    state.target = if target_choice.starts_with("1.") {
        if !docker_ok {
            return Err(anyhow!(
                "Docker is not running. Start Docker Desktop/Engine and try again."
            ));
        }
        DeployTarget::Docker
    } else {
        DeployTarget::Native
    };

    let port_str = prompts::prompt_text_with_default("API Port:", &state.api_port.to_string())?;
    state.api_port = port_str.parse::<u16>().unwrap_or(DEFAULT_API_PORT);

    if state.target == DeployTarget::Docker {
        let default_img = if state.api_image.is_empty() {
            DEFAULT_API_IMAGE
        } else {
            &state.api_image
        };
        state.api_image =
            prompts::prompt_text_with_default("Orbis API Docker Image:", default_img)?;
    }

    // ── Phase 2: Database Selection ───────────────────────────────────
    output::print_phase_header(2, TOTAL_PHASES, "Database Provider");

    let db_options = match state.target {
        DeployTarget::Docker => vec![
            "1. Managed PostgreSQL (Auto-create Docker container)",
            "2. External PostgreSQL (Existing server / host PostgreSQL)",
            "3. SQLite (Local file — single-node only)",
        ],
        DeployTarget::Native => vec![
            "2. External PostgreSQL (Existing server / host PostgreSQL)",
            "3. SQLite (Recommended for local standalone host)",
            "1. Managed PostgreSQL (Create Docker container)",
        ],
    };

    let db_choice = prompts::prompt_select("Select database engine:", db_options)?;

    if db_choice.starts_with("1.") {
        if !docker_ok {
            return Err(anyhow!("Docker is required for Managed PostgreSQL."));
        }
        state.db_provider = DbProvider::DockerPostgres;
        let pg_db = prompts::prompt_text_with_default("PostgreSQL Database Name:", "orbis")?;
        let pg_user = prompts::prompt_text_with_default("PostgreSQL Username:", "orbis")?;
        let pg_pass = prompts::prompt_text_with_default("PostgreSQL Password:", "orbis_pass_123")?;
        let pg_port_str = prompts::prompt_text_with_default("PostgreSQL Host Port:", "5432")?;
        let pg_port = pg_port_str.parse::<u16>().unwrap_or(5432);

        output::print_progress_pending("Starting managed PostgreSQL container...");
        docker::ensure_postgres_container(
            &state.postgres_container,
            &state.docker_network,
            DEFAULT_POSTGRES_IMAGE,
            &pg_user,
            &pg_pass,
            &pg_db,
            pg_port,
        )?;
        state.database_url = format!("postgres://{pg_user}:{pg_pass}@127.0.0.1:{pg_port}/{pg_db}");
        output::print_progress_ok("Managed PostgreSQL container is ready");
    } else if db_choice.starts_with("2.") {
        state.db_provider = DbProvider::ExternalPostgres;
        loop {
            let url = prompts::prompt_text_with_default(
                "PostgreSQL Connection String:",
                if state.database_url.starts_with("postgres") {
                    &state.database_url
                } else {
                    "postgres://orbis:orbis_pass_123@localhost:5432/orbis"
                },
            )?;
            output::print_progress_pending("Testing database connectivity...");
            let test_url = url
                .replace("host.docker.internal", "127.0.0.1")
                .trim()
                .to_string();
            match database::test_connection(&test_url).await {
                Ok(()) => {
                    state.database_url = url.trim().to_string();
                    output::print_progress_ok("PostgreSQL connection verified");
                    break;
                }
                Err(e) => {
                    output::print_error(&format!("Connection failed: {e}"));
                    if !prompts::prompt_confirm("Retry with another connection string?", true)? {
                        return Err(e);
                    }
                }
            }
        }
    } else {
        state.db_provider = DbProvider::Sqlite;
        if state.target == DeployTarget::Docker {
            output::print_warning(
                "SQLite is not shared across containers; PostgreSQL is recommended for Docker.",
            );
        }
        state.database_url = default_sqlite_url();
        output::print_progress_ok(&format!("SQLite configured: {}", state.database_url));
    }

    // ── Phase 3: Server Config & Schema Migrations ────────────────────
    output::print_phase_header(3, TOTAL_PHASES, "Server Config & Migrations");

    match state.target {
        DeployTarget::Docker => {
            let staging_cfg = CliState::state_dir().join("server.json");
            let container_db_url = state.container_database_url();
            ensure_server_config_file(&staging_cfg, &container_db_url)?;

            output::print_progress_pending("Writing server.json to Docker config volume...");
            docker::write_file_to_volume(
                docker::CONFIG_VOLUME,
                "/etc/orbis",
                &staging_cfg,
                "server.json",
                &state.api_image,
            )?;
            output::print_progress_ok("server.json persisted with 10001:10001 permissions");

            output::print_progress_pending("Running database migrations inside Docker...");
            docker::run_migrations(&state.api_image, &state.docker_network)?;
            output::print_progress_ok("Database schema is up to date");
        }
        DeployTarget::Native => {
            let cfg_path = orbis_infra::config::server_config_path().map_err(anyhow::Error::msg)?;
            ensure_server_config_file(&cfg_path, &state.database_url)?;
            output::print_progress_ok(&format!("Saved server config at {}", cfg_path.display()));

            output::print_progress_pending("Running database migrations...");
            database::run_migrations(&state.host_database_url()).await?;
            output::print_progress_ok("Database schema is up to date");
        }
    }

    state.save()?;

    // ── Phase 4: License Provisioning ─────────────────────────────────
    output::print_phase_header(4, TOTAL_PHASES, "License Provisioning");

    let current_lic = license::get_status(&state).await.ok();
    if let Some(ref dto) = current_lic
        && dto.is_operational
    {
        output::print_progress_ok(&format!(
            "Active license detected ({} — {:?})",
            dto.plan_name.as_deref().unwrap_or("Licensed"),
            dto.state
        ));
    } else {
        let mut lic_options = vec![
            "1. Import existing license_snapshot.json",
            "2. Activate with Product Key",
            "3. Skip for now (configure later via `orbis license`)",
        ];
        if let Some(detected) = detect_desktop_license_snapshot() {
            output::print_info(&format!(
                "Found Desktop license snapshot: {}",
                detected.display()
            ));
            lic_options.insert(0, "0. Import detected Desktop license_snapshot.json");
        }

        let lic_choice =
            prompts::prompt_select("How would you like to configure the license?", lic_options)?;
        if lic_choice.starts_with("0.") {
            if let Some(detected) = detect_desktop_license_snapshot() {
                let dto = license::import_snapshot_file(&state, &detected).await?;
                output::print_progress_ok(&format!("License imported (state: {:?})", dto.state));
            }
        } else if lic_choice.starts_with("1.") {
            let path_str = prompts::prompt_text("Path to license_snapshot.json:")?;
            let path = PathBuf::from(path_str.trim().trim_matches('"'));
            let dto = license::import_snapshot_file(&state, &path).await?;
            output::print_progress_ok(&format!("License imported (state: {:?})", dto.state));
        } else if lic_choice.starts_with("2.") {
            let key = prompts::prompt_text("Enter Product Key:")?;
            let org_id_str = prompts::prompt_text_with_default(
                "Organization UUID (leave blank for nil/pending):",
                &uuid::Uuid::nil().to_string(),
            )?;
            let org_id =
                uuid::Uuid::parse_str(org_id_str.trim()).unwrap_or_else(|_| uuid::Uuid::nil());
            let dto = license::activate_key(&state, key.trim(), org_id, None).await?;
            output::print_progress_ok(&format!("License activated (state: {:?})", dto.state));
        } else {
            output::print_info("Skipped license setup. Run `orbis license` anytime to configure.");
        }
    }

    // ── Phase 5: Start API & Summary ──────────────────────────────────
    output::print_phase_header(5, TOTAL_PHASES, "Start & Verify API");

    crate::commands::start::start_api_only(&state).await?;

    output::print_summary_box(
        "ORBIS ENVIRONMENT READY",
        &[
            ("Deployment Target", state.target.label()),
            ("Database Engine", state.db_provider.label()),
            ("API Endpoint", &state.api_base_url()),
            (
                "Health Check",
                &format!("{}/api/v1/health", state.api_base_url()),
            ),
        ],
    );

    Ok(())
}

fn detect_desktop_license_snapshot() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    if let Ok(appdata) = std::env::var("APPDATA") {
        for folder in ["com.mulverse.orbis.dev", "com.mulverse.orbis"] {
            let candidate = PathBuf::from(&appdata)
                .join(folder)
                .join("license_snapshot.json");
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }
    None
}
