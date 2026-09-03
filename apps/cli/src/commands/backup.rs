use crate::ui::output;
use anyhow::{Result, anyhow};
use chrono::Local;
use std::fs;
use std::path::Path;

pub async fn execute() -> Result<()> {
    output::print_step("Initializing Database Backup...");

    let backup_dir = Path::new("backups");
    if !backup_dir.exists() {
        fs::create_dir(backup_dir)?;
        output::print_success("Created backups directory");
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let config = crate::core::config::CliConfig::default();
    let db_path_str = config
        .db_url
        .replace("sqlite://", "")
        .replace("?mode=rwc", "")
        .replace("?mode=ro", "");
    let db_path = Path::new(&db_path_str);

    if !db_path.exists() {
        return Err(anyhow!(
            "Cannot backup: Database file at {:?} does not exist.",
            db_path
        ));
    }

    let backup_filename = format!("orbis_{}.db.bak", timestamp);
    let backup_path = backup_dir.join(&backup_filename);

    output::print_info(&format!("Copying {:?} -> {}...", db_path, backup_filename));

    fs::copy(db_path, &backup_path)?;

    output::print_success(&format!("Backup successful! Saved to {:?}", backup_path));

    Ok(())
}
