use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use colored::*;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement, TransactionTrait};

pub mod commands;
pub mod core;
pub mod ui;

#[derive(Parser)]
#[command(name = "orbis")]
#[command(
    about = "Orbis Management CLI",
    long_about = "Standalone CLI for setting up and managing the Orbis Educational System."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Initialize the Orbis system (license, database, organization, admin)
    Init,
    /// Start the Orbis system
    Start,
    /// Check system health and database connectivity
    Doctor,
    /// Backup the database securely
    Backup,
    /// Manage product license and activation
    License,
}


/// Menu choices as an enum for robust matching.
#[derive(Debug, Clone, PartialEq)]
enum MenuChoice {
    Init,
    Start,
    Doctor,
    Backup,
    License,
    ResetDatabase,
    SeedDefaultData,
    Exit,
}

struct MenuItem {
    label: String,
    choice: MenuChoice,
}

async fn handle_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Init => {
            commands::init::execute().await?;
        }
        Commands::Start => {
            commands::start::execute().await?;
        }
        Commands::Doctor => {
            commands::doctor::execute().await?;
        }
        Commands::Backup => {
            commands::backup::execute().await?;
        }
        Commands::License => {
            commands::license::execute().await?;
        }
    }
    Ok(())
}

/// Build menu items dynamically based on system status.
fn build_menu(status: &core::database::SystemStatus) -> Vec<MenuItem> {
    let mut items = Vec::new();
    let mut idx = 1u32;

    let mut push = |label: &str, choice: MenuChoice| {
        items.push(MenuItem {
            label: format!("{}. {}", idx, label),
            choice,
        });
        idx += 1;
    };

    if status.is_setup_complete() {
        push("Start System (Launch Desktop)", MenuChoice::Start);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
        push("Backup Database Snapshot", MenuChoice::Backup);
        push("Manage License & Plan", MenuChoice::License);
        push("Workspace Setup / Repair", MenuChoice::Init);
        push("Reset Database Store", MenuChoice::ResetDatabase);
    } else if status.is_initialized() {
        push("Start System (Complete Onboarding)", MenuChoice::Start);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
        push("Manage License & Plan", MenuChoice::License);
        push("Workspace Setup / Repair", MenuChoice::Init);
        push("Backup Database Snapshot", MenuChoice::Backup);
        push("Reset Database Store", MenuChoice::ResetDatabase);
    } else {
        push("Initialize Workspace (Setup Database & Admin)", MenuChoice::Init);
        push("Manage Product License", MenuChoice::License);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
        push("Reset Database Store", MenuChoice::ResetDatabase);
    }

    if is_development_environment() {
        push(
            "Seed Development Records (Mock Data)",
            MenuChoice::SeedDefaultData,
        );
    }

    push("Exit Console", MenuChoice::Exit);
    items
}

fn is_development_environment() -> bool {
    matches!(
        std::env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str(),
        "development" | "dev"
    )
}

/// Handles the "Reset Database" action with confirmation.
async fn handle_reset_database() {
    let confirm = inquire::Confirm::new(
        "Are you sure you want to completely wipe the database? This action CANNOT BE UNDONE!",
    )
    .with_default(false)
    .prompt();

    match confirm {
        Ok(true) => {
            ui::output::print_warning("Wiping database...");
            let db_url = match std::env::var("DATABASE_URL") {
                Ok(url) if !url.trim().is_empty() => url,
                _ => {
                    let provider = inquire::Select::new(
                        "Select database to reset:",
                        vec!["SQLite", "PostgreSQL"],
                    )
                    .prompt();

                    match provider {
                        Ok("PostgreSQL") => {
                            match inquire::Text::new("Enter PostgreSQL Connection String:").prompt()
                            {
                                Ok(url) => url,
                                Err(e) => {
                                    ui::output::print_error(&format!("Reset cancelled: {e}"));
                                    return;
                                }
                            }
                        }
                        Ok("SQLite") => core::config::CliConfig::default().db_url,
                        Ok(_) | Err(_) => {
                            println!("{}", "[-] Reset cancelled.".dimmed());
                            return;
                        }
                    }
                }
            };

            if is_postgres_url(&db_url) {
                let postgres_confirm = inquire::Confirm::new(
                    "PostgreSQL data will be permanently deleted, while tables and the database remain. Continue?",
                )
                .with_default(false)
                .prompt();

                if !matches!(postgres_confirm, Ok(true)) {
                    println!("{}", "[-] Reset cancelled.".dimmed());
                    return;
                }

                match reset_postgres(&db_url).await {
                    Ok(()) => ui::output::print_success(
                        "PostgreSQL data reset successfully. You can now re-initialize the system.",
                    ),
                    Err(e) => ui::output::print_error(&format!("Failed to reset PostgreSQL: {e}")),
                }
            } else {
                let path = sqlite_path(&db_url);
                if std::path::Path::new(&path).exists() {
                    if let Err(e) = std::fs::remove_file(&path) {
                        ui::output::print_error(&format!("Failed to delete database: {}", e));
                    } else {
                        ui::output::print_success(
                            "Database reset successfully. You can now re-initialize the system.",
                        );
                    }
                } else {
                    ui::output::print_warning(&format!(
                        "Database file not found, nothing to reset. Path attempted: {}",
                        path
                    ));
                }
            }
        }
        _ => {
            println!("{}", "[-] Reset cancelled.".dimmed());
        }
    }
}

fn is_postgres_url(url: &str) -> bool {
    url.starts_with("postgres://") || url.starts_with("postgresql://")
}

fn sqlite_path(url: &str) -> String {
    url.replace("sqlite://", "")
        .replace("?mode=rwc", "")
        .replace("?mode=ro", "")
}

async fn reset_postgres(url: &str) -> Result<()> {
    let db = Database::connect(url).await?;
    let txn = db.begin().await?;
    let tables = txn
        .query_all(Statement::from_string(
            DbBackend::Postgres,
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE' ORDER BY table_name"
                .to_string(),
        ))
        .await?;

    let table_names: Vec<String> = tables
        .into_iter()
        .map(|row| row.try_get::<String>("", "table_name"))
        .collect::<Result<_, _>>()?;

    if !table_names.is_empty() {
        let qualified_tables = table_names
            .iter()
            .map(|name| format!("\"public\".\"{}\"", name.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(", ");
        txn.execute_unprepared(&format!(
            "TRUNCATE TABLE {qualified_tables} RESTART IDENTITY CASCADE"
        ))
        .await?;
    }

    txn.commit().await.map_err(|e| anyhow!(e))?;
    Ok(())
}

async fn ensure_authenticated(status: &core::database::SystemStatus) -> Result<Option<core::auth::AuthenticatedAdmin>> {
    if !status.is_initialized() {
        return Ok(None);
    }

    let config = core::config::CliConfig::default();
    let db = Database::connect(&config.db_url).await
        .map_err(|e| anyhow!("Failed to connect to database for authentication: {e}"))?;

    let admin = core::auth::authenticate_admin(&db).await?;
    Ok(Some(admin))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Enable ANSI escape codes on Windows CMD/PowerShell
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    // Configure basic tracing (Only ERROR to prevent SQLx from polluting CLI menu)
    tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::ERROR)
        .init();

    let cli = Cli::parse();
    let initial_status = core::database::get_full_status().await;

    if let Some(cmd) = &cli.command {
        // Run specific command passed via CLI args directly
        let (db_exists, setup_completed) =
            (initial_status.is_initialized(), initial_status.is_setup_complete());
        ui::output::print_banner(db_exists, setup_completed);

        // Enforce authentication for protected commands when initialized
        let needs_auth = match cmd {
            Commands::Doctor | Commands::Init => false,
            _ => initial_status.is_initialized(),
        };

        if needs_auth {
            ensure_authenticated(&initial_status).await?;
        }

        handle_command(cmd).await?;
    } else {
        // Interactive Launcher Mode (when double-clicked or run without arguments)
        let (db_exists, setup_completed) =
            (initial_status.is_initialized(), initial_status.is_setup_complete());
        ui::output::print_banner(db_exists, setup_completed);

        // Enforce authentication gate upon opening CLI if system is initialized
        let auth_admin = ensure_authenticated(&initial_status).await?;

        loop {
            let status = core::database::get_full_status().await;
            let (db_exists, setup_completed) =
                (status.is_initialized(), status.is_setup_complete());
            ui::output::print_banner(db_exists, setup_completed);

            // Show active org & authenticated admin
            if let Some(ref name) = status.org_name {
                let dot = "•".truecolor(ui::output::COLOR_MUTED.0, ui::output::COLOR_MUTED.1, ui::output::COLOR_MUTED.2);
                if let Some(ref admin) = auth_admin {
                    println!("  {} {} {}  {}  {} {} ({})\n",
                        "✦".truecolor(ui::output::COLOR_PURPLE.0, ui::output::COLOR_PURPLE.1, ui::output::COLOR_PURPLE.2),
                        "Workspace:".truecolor(ui::output::COLOR_MUTED.0, ui::output::COLOR_MUTED.1, ui::output::COLOR_MUTED.2),
                        name.truecolor(ui::output::COLOR_WHITE.0, ui::output::COLOR_WHITE.1, ui::output::COLOR_WHITE.2).bold(),
                        dot,
                        "Admin:".truecolor(ui::output::COLOR_MUTED.0, ui::output::COLOR_MUTED.1, ui::output::COLOR_MUTED.2),
                        admin.full_name.truecolor(ui::output::COLOR_WHITE.0, ui::output::COLOR_WHITE.1, ui::output::COLOR_WHITE.2).bold(),
                        admin.role.truecolor(ui::output::COLOR_EMERALD.0, ui::output::COLOR_EMERALD.1, ui::output::COLOR_EMERALD.2)
                    );
                } else {
                    println!("  {} {} {}\n",
                        "✦".truecolor(ui::output::COLOR_PURPLE.0, ui::output::COLOR_PURPLE.1, ui::output::COLOR_PURPLE.2),
                        "Workspace:".truecolor(ui::output::COLOR_MUTED.0, ui::output::COLOR_MUTED.1, ui::output::COLOR_MUTED.2),
                        name.truecolor(ui::output::COLOR_WHITE.0, ui::output::COLOR_WHITE.1, ui::output::COLOR_WHITE.2).bold()
                    );
                }
            }

            let menu = build_menu(&status);
            let labels: Vec<&str> = menu.iter().map(|m| m.label.as_str()).collect();

            let choice = ui::prompts::prompt_select("Select an action:", labels);

            match choice {
                Ok(selected) => {
                    // Find the matching menu item by label
                    let selected_item = menu.iter().find(|m| m.label == selected);

                    if let Some(item) = selected_item {
                        if item.choice == MenuChoice::Exit {
                            break;
                        }

                        ui::output::print_divider();

                        match item.choice {
                            MenuChoice::Start => {
                                if let Err(e) = handle_command(&Commands::Start).await {
                                    ui::output::print_error(&e.to_string());
                                }
                            }
                            MenuChoice::Init => {
                                if let Err(e) = handle_command(&Commands::Init).await {
                                    ui::output::print_error(&e.to_string());
                                }
                            }
                            MenuChoice::Doctor => {
                                if let Err(e) = handle_command(&Commands::Doctor).await {
                                    ui::output::print_error(&e.to_string());
                                }
                            }
                            MenuChoice::Backup => {
                                if let Err(e) = handle_command(&Commands::Backup).await {
                                    ui::output::print_error(&e.to_string());
                                }
                            }
                            MenuChoice::License => {
                                if let Err(e) = handle_command(&Commands::License).await {
                                    ui::output::print_error(&e.to_string());
                                }
                            }
                            MenuChoice::ResetDatabase => {
                                handle_reset_database().await;
                            }
                            MenuChoice::SeedDefaultData => {
                                if let Err(e) = commands::dev_seed::execute().await {
                                    ui::output::print_error(&e.to_string());
                                }
                            }

                            MenuChoice::Exit => break,
                        }

                        println!("\n{}", "Press Enter to return to menu...".dimmed());
                        let mut discard = String::new();
                        std::io::stdin().read_line(&mut discard).unwrap_or(0);
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    Ok(())
}

