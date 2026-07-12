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
    let db_name = "orbis_registry.db";
    let db_path = Path::new(db_name);

    if !db_path.exists() {
        return Err(anyhow!("Cannot backup: {} does not exist.", db_name));
    }

    let backup_filename = format!("orbis_registry_{}.db.bak", timestamp);
    let backup_path = backup_dir.join(&backup_filename);

    println!("{} Copying {} -> {}...", "[*]".cyan(), db_name, backup_filename);
    
    fs::copy(db_path, &backup_path)?;

    println!("{} Backup successful! Saved to {:?}", "[+]".green().bold(), backup_path);

    Ok(())
}
