use anyhow::{Context, Result, anyhow};
use chrono::Local;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use crate::core::config::{CliState, DbProvider};
use crate::ui::output;

pub async fn execute() -> Result<()> {
    output::print_step("Initiating Secure Database Backup...");
    let state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;

    let backup_dir = Path::new("backups");
    fs::create_dir_all(backup_dir)?;
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let (backup_path, method) = match state.db_provider {
        DbProvider::Sqlite => {
            let db_url = state.host_database_url();
            let filename = format!("orbis_sqlite_{timestamp}.db.bak");
            let path = backup_dir.join(filename);
            let path_str = path.to_string_lossy().into_owned();

            output::print_progress_pending("Creating consistent SQLite snapshot...");
            let db = Database::connect(db_url.replace("mode=rwc", "mode=ro"))
                .await
                .context("Could not open SQLite database")?;
            db.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "VACUUM INTO ?",
                [path_str.into()],
            ))
            .await
            .context("SQLite VACUUM INTO failed")?;
            (path, "SQLite online snapshot")
        }
        DbProvider::DockerPostgres => {
            let filename = format!("orbis_pg_{timestamp}.sql");
            let path = backup_dir.join(filename);
            let (user, db_name) = parse_pg_user_db(&state.database_url);

            output::print_progress_pending("Dumping PostgreSQL from Docker container...");
            let out = Command::new("docker")
                .args([
                    "exec",
                    &state.postgres_container,
                    "pg_dump",
                    "-U",
                    &user,
                    &db_name,
                ])
                .output()
                .context("Failed to execute `docker exec pg_dump`")?;
            if !out.status.success() {
                return Err(anyhow!(
                    "pg_dump failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                ));
            }
            let mut file = File::create(&path)?;
            std::io::Write::write_all(&mut file, &out.stdout)?;
            (path, "PostgreSQL pg_dump (Docker)")
        }
        DbProvider::ExternalPostgres => {
            let filename = format!("orbis_pg_{timestamp}.sql");
            let path = backup_dir.join(filename);
            let db_url = state.host_database_url();

            output::print_progress_pending("Running local pg_dump...");
            let out = Command::new("pg_dump").arg(&db_url).output().context(
                "Failed to execute `pg_dump` (ensure PostgreSQL client tools are in PATH)",
            )?;
            if !out.status.success() {
                return Err(anyhow!(
                    "pg_dump failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                ));
            }
            let mut file = File::create(&path)?;
            std::io::Write::write_all(&mut file, &out.stdout)?;
            (path, "PostgreSQL pg_dump")
        }
    };

    let bytes = fs::metadata(&backup_path)?.len();
    let size_fmt = if bytes > 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    };

    output::print_summary_box(
        "BACKUP COMPLETED",
        &[
            ("Database Engine", state.db_provider.label()),
            ("Backup File", &backup_path.to_string_lossy()),
            ("Size", &size_fmt),
            ("Method", method),
        ],
    );
    output::print_success("Backup created successfully.");
    Ok(())
}

pub async fn restore(backup_file: &Path) -> Result<()> {
    if !backup_file.exists() {
        return Err(anyhow!("Backup file not found: {}", backup_file.display()));
    }
    let state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;

    output::print_step(&format!(
        "Restoring database from {}...",
        backup_file.display()
    ));
    match state.db_provider {
        DbProvider::Sqlite => {
            let db_url = state.host_database_url();
            let dest_str = db_url
                .replace("sqlite://", "")
                .replace("?mode=rwc", "")
                .replace("?mode=ro", "");
            fs::copy(backup_file, &dest_str)
                .with_context(|| format!("Failed to restore SQLite file to {dest_str}"))?;
        }
        DbProvider::DockerPostgres => {
            let sql = fs::read(backup_file)?;
            let (user, db_name) = parse_pg_user_db(&state.database_url);
            let mut child = Command::new("docker")
                .args([
                    "exec",
                    "-i",
                    &state.postgres_container,
                    "psql",
                    "-U",
                    &user,
                    "-d",
                    &db_name,
                ])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .context("Failed to spawn `docker exec psql`")?;
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                stdin.write_all(&sql)?;
            }
            let out = child.wait_with_output()?;
            if !out.status.success() {
                return Err(anyhow!(
                    "psql restore failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                ));
            }
        }
        DbProvider::ExternalPostgres => {
            let db_url = state.host_database_url();
            let status = Command::new("psql")
                .args([&db_url, "-f", &backup_file.to_string_lossy()])
                .status()
                .context("Failed to execute `psql` (ensure PostgreSQL client tools are in PATH)")?;
            if !status.success() {
                return Err(anyhow!("psql restore failed"));
            }
        }
    }

    output::print_success("Database restored successfully.");
    Ok(())
}

fn parse_pg_user_db(url: &str) -> (String, String) {
    let without_scheme = url
        .strip_prefix("postgres://")
        .or_else(|| url.strip_prefix("postgresql://"))
        .unwrap_or(url);
    let user = without_scheme
        .split('@')
        .next()
        .and_then(|creds| creds.split(':').next())
        .filter(|s| !s.is_empty())
        .unwrap_or("orbis")
        .to_string();
    let db = without_scheme
        .rsplit('/')
        .next()
        .and_then(|s| s.split('?').next())
        .filter(|s| !s.is_empty())
        .unwrap_or("orbis")
        .to_string();
    (user, db)
}
