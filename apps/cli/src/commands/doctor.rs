use anyhow::Result;
use std::path::Path;
use crate::ui::output;

pub async fn execute() -> Result<()> {
    output::print_step("Running System Diagnostics...");
    println!("──────────────────────────────────────────────────");

    let mut all_good = true;

    // 1. Check for DB file (SQLite)
    let config = crate::core::config::CliConfig::default();
    let db_path_str = config.db_url
        .replace("sqlite://", "")
        .replace("?mode=rwc", "")
        .replace("?mode=ro", "");
    
    let db_path = Path::new(&db_path_str);
    
    print!("{: <40}", "Main Database (orbis.db):");
    if db_path.exists() {
        println!("[+] Found");
    } else {
        println!("[-] Missing (Expected at {:?})", db_path);
        all_good = false;
    }

    // 2. Check for App executable
    let app_name = if cfg!(windows) { "Orbis-Svelte.exe" } else { "Orbis-Svelte" };
    
    // Look in current dir, or relative to workspace root
    let possible_paths = crate::core::config::CliConfig::get_app_executable_paths();

    print!("{: <40}", format!("Desktop App ({}):", app_name));
    let mut app_found = false;
    for path in possible_paths.iter() {
        if path.exists() {
            app_found = true;
            break;
        }
    }

    if app_found {
        println!("[+] Found");
    } else {
        println!("[-] Missing");
        all_good = false;
    }

    // 3. Environment variables
    print!("{: <40}", "Environment Mode:");
    if cfg!(debug_assertions) {
        println!("[+] Development (Debug)");
    } else {
        println!("[+] Production (Release)");
    }

    println!("──────────────────────────────────────────────────");
    if all_good {
        output::print_success("System is healthy and ready to run!");
    } else {
        output::print_warning("Some issues were found. Run `cli init` to repair or build the desktop app.");
    }

    Ok(())
}
