use std::sync::Arc;
use std::path::Path;
use anyhow::Result;
use colored::*;
use sea_orm::Database;
use orbis_app::services::SystemService;
use orbis_infra::repositories::system_repo::SystemRepoImpl;

use crate::ui::output::{self, COLOR_BORDER, COLOR_EMERALD, COLOR_MUTED, COLOR_ROSE};

pub async fn execute() -> Result<()> {
    output::print_step("Running Comprehensive System Diagnostics...");
    output::print_divider();

    let mut all_good = true;
    let border = |s: &str| s.truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2);

    let mut checks: Vec<(&str, String, bool)> = Vec::new();

    // 1. Check for DB connection
    let config = crate::core::config::CliConfig::default();
    let db_path_str = config
        .db_url
        .replace("sqlite://", "")
        .replace("?mode=rwc", "")
        .replace("?mode=ro", "");

    let db_path = Path::new(&db_path_str);
    let db_exists = if config.db_url.starts_with("postgres") {
        Database::connect(&config.db_url).await.is_ok()
    } else {
        db_path.exists()
    };

    if db_exists {
        checks.push(("Database Store", format!("Available ({})", if config.db_url.starts_with("postgres") { "PostgreSQL" } else { "orbis.db" }), true));
    } else {
        checks.push(("Database Store", "Missing (Run init to provision)".to_string(), false));
        all_good = false;
    }

    // 2. Check Database Schema & System Service Diagnostics
    if let Ok(db) = Database::connect(&config.db_url).await {
        let system_repo = Arc::new(SystemRepoImpl { db });
        let system_service = SystemService::new(system_repo);

        let health = system_service.health().await;
        if health.database_connected {
            checks.push(("Database Connection", "Connected & Queryable".to_string(), true));
        } else {
            checks.push(("Database Connection", "Connection Degraded".to_string(), false));
            all_good = false;
        }

        match system_service.get_readiness().await {
            Ok(readiness) => {
                if readiness.has_organization {
                    checks.push(("Workspace Entity", format!("Configured ({})", readiness.org_name.unwrap_or_else(|| "Active".to_string())), true));
                } else {
                    checks.push(("Workspace Entity", "Unconfigured".to_string(), false));
                    all_good = false;
                }

                if readiness.has_admin {
                    checks.push(("Admin Identity", "Provisioned (Bcrypt + IAM)".to_string(), true));
                } else {
                    checks.push(("Admin Identity", "No Admin Found".to_string(), false));
                    all_good = false;
                }
            }
            Err(e) => {
                checks.push(("System Readiness", format!("Query Error: {e}"), false));
                all_good = false;
            }
        }
    } else {
        checks.push(("Database Connection", "Connection Failed".to_string(), false));
        all_good = false;
    }

    // 3. License check
    match crate::core::license::check_license_status() {
        crate::core::license::LicenseStatus::Active => {
            checks.push(("Product License", "Active & Verified".to_string(), true));
        }
        crate::core::license::LicenseStatus::NotFound => {
            checks.push(("Product License", "Community / Evaluation".to_string(), true));
        }
        crate::core::license::LicenseStatus::Expired => {
            checks.push(("Product License", "License Expired".to_string(), false));
            all_good = false;
        }
        crate::core::license::LicenseStatus::Invalid => {
            checks.push(("Product License", "Signature Invalid".to_string(), false));
            all_good = false;
        }
    }

    // 4. Desktop binary check
    let app_name = if cfg!(windows) {
        "Orbis-Svelte.exe"
    } else {
        "Orbis-Svelte"
    };
    let possible_paths = crate::core::config::CliConfig::get_app_executable_paths();
    let mut app_found = false;
    for path in possible_paths.iter() {
        if path.exists() {
            app_found = true;
            break;
        }
    }

    if app_found {
        checks.push(("Desktop Application", format!("Found ({})", app_name), true));
    } else {
        checks.push(("Desktop Application", format!("Not Compiled ({})", app_name), false));
    }

    // 5. Environment mode
    let env_mode = if cfg!(debug_assertions) { "Development (Debug)" } else { "Production (Release)" };
    checks.push(("Runtime Mode", env_mode.to_string(), true));

    // Render Diagnostics Table Card
    println!();
    println!("  {}", border("╭─ DIAGNOSTICS REPORT ──────────────────────────────────────────╮"));
    for (label, detail, ok) in checks {
        let badge = if ok {
            " ✔ PASS ".black().on_truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2).bold()
        } else {
            " ✖ FAIL ".black().on_truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2).bold()
        };
        let label_fmt = format!("{:<24}", label.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2));
        let detail_fmt = format!("{:<22}", detail);
        println!("  {}  {} {} {} {}", border("│"), badge, label_fmt, detail_fmt.bold(), border("│"));
    }
    println!("  {}", border("╰───────────────────────────────────────────────────────────────╯\n"));

    if all_good {
        output::print_success("System integrity verified — All core services healthy and operational.");
    } else {
        output::print_warning("Integrity check identified issues. Run `cli init` to repair workspace provisioning.");
    }

    Ok(())
}
