use crate::ui::output;
use anyhow::{Context, Result, anyhow};
use chrono::Local;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub async fn execute() -> Result<()> {
    output::print_step("Initiating Secure Database Backup Process...");
    let db_url = crate::core::config::CliConfig::sqlite_url()?;

    let backup_dir = Path::new("backups");
    fs::create_dir_all(backup_dir)?;

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let db_path_str = db_url
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

    let backup_filename = format!("orbis_backup_{}_{}.db.bak", timestamp, Uuid::new_v4());
    let backup_path = backup_dir.join(&backup_filename);
    let backup_str = backup_path.to_string_lossy().into_owned();
    output::print_progress_pending("Creating a consistent SQLite snapshot...");
    let db = Database::connect(db_url.replace("mode=rwc", "mode=ro"))
        .await
        .context("Could not open the SQLite database for backup")?;
    db.execute(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        "VACUUM INTO ?",
        [backup_str.clone().into()],
    ))
    .await
    .context("Could not create the SQLite snapshot")?;
    let bytes = fs::metadata(&backup_path)?.len();

    let size_kb = bytes as f64 / 1024.0;
    let size_fmt = if size_kb > 1024.0 {
        format!("{:.2} MB", size_kb / 1024.0)
    } else {
        format!("{:.2} KB", size_kb)
    };

    output::print_summary_box("BACKUP SNAPSHOT COMPLETED", &[
        ("Source Database", &db_path_str),
        ("Backup Archive", &backup_str),
        ("Snapshot Size", &size_fmt),
        ("Timestamp", &timestamp),
        ("Method", "SQLite online snapshot"),
    ]);

    output::print_success("Database snapshot completed successfully.");

    Ok(())
}
