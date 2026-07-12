use anyhow::Result;
use colored::*;
use std::path::Path;

pub async fn execute() -> Result<()> {
    println!("{}", "[*] Running System Diagnostics...".cyan().bold());
    println!("{}", "──────────────────────────────────────────────────".dimmed());

    let mut all_good = true;

    // 1. Check for DB file (SQLite)
    let db_path = Path::new("orbis_registry.db");
    print!("{: <40}", "Registry Database (orbis_registry.db):");
    if db_path.exists() {
        println!("{}", "[+] Found".green());
    } else {
        println!("{}", "[-] Missing".red());
        all_good = false;
    }

    // 2. Check for App executable
    let app_name = if cfg!(windows) { "Orbis-Svelte.exe" } else { "Orbis-Svelte" };
    let app_path = Path::new(app_name);
    let dev_path = Path::new("../desktop-svelte/src-tauri/target/release/Orbis-Svelte.exe");
    
    print!("{: <40}", format!("Desktop App ({}):", app_name));
    if app_path.exists() || dev_path.exists() {
        println!("{}", "[+] Found".green());
    } else {
        println!("{}", "[-] Missing".red());
        all_good = false;
    }

    // 3. Environment variables
    print!("{: <40}", "ORBIS_ENV variable:");
    if std::env::var("ORBIS_ENV").is_ok() {
        println!("{}", "[+] Set".green());
    } else {
        println!("{} (Defaulting to Production)", "[!] Not set".yellow());
    }

    println!("{}", "──────────────────────────────────────────────────".dimmed());
    if all_good {
        println!("{}", "[*] System is healthy and ready to run!".green().bold());
    } else {
        println!("{}", "[!] Some issues were found. Run `cli init` to repair.".yellow().bold());
    }

    Ok(())
}
