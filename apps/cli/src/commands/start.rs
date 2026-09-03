use crate::core::config::CliConfig;
use crate::ui::output;
use anyhow::{Result, anyhow};
use std::process::Command;

pub async fn execute() -> Result<()> {
    output::print_step("Booting Orbis System...");

    let app_name = if cfg!(windows) {
        "Orbis-Svelte.exe"
    } else {
        "Orbis-Svelte"
    };

    let possible_paths = CliConfig::get_app_executable_paths();

    let mut found_path = None;
    for path in possible_paths.iter() {
        if path.exists() {
            found_path = Some(path.clone());
            break;
        }
    }

    match found_path {
        Some(path) => {
            output::print_info(&format!("Found Orbis application at {:?}", path));
            output::print_step("Spawning application process...");

            let child = Command::new(path)
                .spawn()
                .map_err(|e| anyhow!("Failed to start Orbis-Svelte: {}", e))?;

            output::print_success(&format!("System is running (PID: {})!", child.id()));
        }
        None => {
            return Err(anyhow!(
                "Could not find Orbis executable ('{}'). Ensure it is downloaded or built.",
                app_name
            ));
        }
    }

    Ok(())
}
