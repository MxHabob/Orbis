use anyhow::{anyhow, Result};
use colored::*;
use std::process::Command;


pub async fn execute() -> Result<()> {
    println!("{}", "[*] Booting Orbis System...".green().bold());
    
    let app_name = if cfg!(windows) {
        "Orbis-Svelte.exe"
    } else {
        "Orbis-Svelte"
    };

    let possible_paths = crate::config::CliConfig::get_app_executable_paths();

    let mut found_path = None;
    for path in possible_paths.iter() {
        if path.exists() {
            found_path = Some(path.clone());
            break;
        }
    }

    match found_path {
        Some(path) => {
            println!("{} Found Orbis application at {:?}", "[+]", path);
            println!("{} Spawning application process...", "[*]");
            
            // Spawn as a detached child process (so CLI can exit, or wait)
            let child = Command::new(path)
                .spawn()
                .map_err(|e| anyhow!("Failed to start Orbis-Svelte: {}", e))?;
                
            println!("{} System is running (PID: {})!", "[*]".green().bold(), child.id());
            
            // We can optionally wait for the process to exit
            // child.wait()?;
        }
        None => {
            return Err(anyhow!(
                "{} Could not find Orbis executable ('{}'). Ensure it is downloaded or built.",
                "[!]".red(),
                app_name
            ));
        }
    }

    Ok(())
}
