use anyhow::Result;
use colored::*;
use std::path::Path;

pub async fn execute() -> Result<()> {
    println!("{}", "[*] Running System Diagnostics...".cyan().bold());
    println!("{}", "──────────────────────────────────────────────────".dimmed());

    let mut all_good = true;

    // 1. Check for DB file (SQLite)
    let config = crate::config::CliConfig::default();
    let db_path_str = config.db_url
        .replace("sqlite://", "")
        .replace("?mode=rwc", "")
        .replace("?mode=ro", "");
    
    let db_path = Path::new(&db_path_str);
    
    print!("{: <40}", "Main Database (orbis.db):");
    if db_path.exists() {
        println!("{}", "[+] Found".green());
    } else {
        println!("{} (Expected at {:?})", "[-] Missing".red(), db_path);
        all_good = false;
    }

    // 2. Check for App executable
    let app_name = if cfg!(windows) { "Orbis-Svelte.exe" } else { "Orbis-Svelte" };
    
    // Look in current dir, or relative to workspace root
    let possible_paths = crate::config::CliConfig::get_app_executable_paths();

    print!("{: <40}", format!("Desktop App ({}):", app_name));
    let mut app_found = false;
    for path in possible_paths.iter() {
        if path.exists() {
            app_found = true;
            break;
        }
    }

    if app_found {
        println!("{}", "[+] Found".green());
    } else {
        println!("{}", "[-] Missing".red());
        all_good = false;
    }

    // 3. Environment variables
    print!("{: <40}", "Environment Mode:");
    if cfg!(debug_assertions) {
        println!("{}", "[+] Development (Debug)".blue());
    } else {
        println!("{}", "[+] Production (Release)".green());
    }

    println!("{}", "──────────────────────────────────────────────────".dimmed());
    if all_good {
        println!("{}", "[*] System is healthy and ready to run!".green().bold());
    } else {
        println!("{}", "[!] Some issues were found. Run `cli init` to repair or build the desktop app.".yellow().bold());
    }

    Ok(())
}
