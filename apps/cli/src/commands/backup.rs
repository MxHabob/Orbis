use anyhow::{anyhow, Result};
use colored::*;
use std::fs;
use std::path::Path;
use chrono::Local;

pub async fn execute() -> Result<()> {
    println!("{}", "[*] Initializing Database Backup...".cyan().bold());

    let backup_dir = Path::new("backups");
    if !backup_dir.exists() {
        fs::create_dir(backup_dir)?;
        println!("{} Created backups directory", "[+]".green());
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    let config = crate::config::CliConfig::default();
    let db_path_str = config.db_url
        .replace("sqlite://", "")
        .replace("?mode=rwc", "")
        .replace("?mode=ro", "");
    let db_path = Path::new(&db_path_str);

    if !db_path.exists() {
        return Err(anyhow!("Cannot backup: Database file at {:?} does not exist.", db_path));
    }

    let backup_filename = format!("orbis_{}.db.bak", timestamp);
    let backup_path = backup_dir.join(&backup_filename);

    println!("{} Copying {:?} -> {}...", "[*]".cyan(), db_path, backup_filename);
    
    fs::copy(db_path, &backup_path)?;

    println!("{} Backup successful! Saved to {:?}", "[+]".green().bold(), backup_path);

    Ok(())
}
