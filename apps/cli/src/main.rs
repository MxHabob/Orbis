use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

pub mod commands;
pub mod core;
pub mod ui;

#[derive(Parser)]
#[command(name = "orbis")]
#[command(about = "Orbis Management CLI", long_about = "Standalone CLI for setting up and managing the Orbis Educational System.")]
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
        push("Start System", MenuChoice::Start);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
        push("Backup Database", MenuChoice::Backup);
        push("Manage License", MenuChoice::License);
        push("Repair / Reinstall", MenuChoice::Init);
        push("Reset Database (Wipe All Data)", MenuChoice::ResetDatabase);
    } else if status.is_initialized() {
        push("Start System (Complete Onboarding)", MenuChoice::Start);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
        push("Manage License", MenuChoice::License);
        push("Repair / Reinstall", MenuChoice::Init);
        push("Backup Database", MenuChoice::Backup);
        push("Reset Database (Wipe All Data)", MenuChoice::ResetDatabase);
    } else {
        push("Initialize System (Required)", MenuChoice::Init);
        push("Manage License", MenuChoice::License);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
        push("Reset Database (Wipe All Data)", MenuChoice::ResetDatabase);
    }

    push("Exit", MenuChoice::Exit);
    items
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
            let config = core::config::CliConfig::default();
            let path = config
                .db_url
                .replace("sqlite://", "")
                .replace("?mode=rwc", "")
                .replace("?mode=ro", "");
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
        _ => {
            println!("{}", "[-] Reset cancelled.".dimmed());
        }
    }
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

    if let Some(cmd) = &cli.command {
        // Run specific command passed via CLI args directly
        let (db_exists, setup_completed) = core::database::get_system_status().await;
        ui::output::print_banner(db_exists, setup_completed);
        handle_command(cmd).await?;
    } else {
        // Interactive Launcher Mode (when double-clicked without arguments)
        loop {
            let status = core::database::get_full_status().await;
            let (db_exists, setup_completed) = (status.is_initialized(), status.is_setup_complete());
            ui::output::print_banner(db_exists, setup_completed);

            // Show org name if available
            if let Some(ref name) = status.org_name {
                println!("  Organization: {}\n", name.bold());
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
