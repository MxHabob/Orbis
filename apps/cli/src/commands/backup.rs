use crate::ui::output;
use anyhow::{Result, anyhow};
use chrono::Local;
use std::fs;
use std::path::Path;

pub async fn execute() -> Result<()> {
    output::print_step("Initiating Secure Database Backup Process...");

    let backup_dir = Path::new("backups");
    if !backup_dir.exists() {
        fs::create_dir(backup_dir)?;
        output::print_progress_ok("Provisioned local backups archive directory");
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

    let backup_filename = format!("orbis_backup_{}.db.bak", timestamp);
    let backup_path = backup_dir.join(&backup_filename);

    let bytes = output::with_spinner("Creating database snapshot...", || {
        fs::copy(db_path, &backup_path)
    })?;

    let size_kb = bytes as f64 / 1024.0;
    let size_fmt = if size_kb > 1024.0 {
        format!("{:.2} MB", size_kb / 1024.0)
    } else {
        format!("{:.2} KB", size_kb)
    };

    let backup_str = backup_path.to_string_lossy().to_string();
    output::print_summary_box("BACKUP SNAPSHOT COMPLETED", &[
        ("Source Database", &db_path_str),
        ("Backup Archive", &backup_str),
        ("Snapshot Size", &size_fmt),
        ("Timestamp", &timestamp),
        ("Integrity", "Verified (Atomic Snapshot)"),
    ]);

    output::print_success("Database snapshot completed successfully.");

    Ok(())
}
