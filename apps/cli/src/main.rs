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
    /// Initialize the Orbis system (download binaries, setup database)
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
            let (db_exists, setup_completed) = core::database::get_system_status().await;
            ui::output::print_banner(db_exists, setup_completed);
            
            let options = if setup_completed {
                vec![
                    "1. Start System",
                    "2. Run Diagnostics (Doctor)",
                    "3. Backup Database",
                    "4. Manage License",
                    "5. Repair / Reinstall",
                    "6. Reset Database (Wipe All Data)",
                    "7. Exit",
                ]
            } else if db_exists {
                vec![
                    "1. Start System (Complete Onboarding)",
                    "2. Run Diagnostics (Doctor)",
                    "3. Repair / Reinstall",
                    "4. Backup Database",
                    "5. Reset Database (Wipe All Data)",
                    "6. Exit",
                ]
            } else {
                vec![
                    "1. Initialize System (Required)",
                    "2. Run Diagnostics (Doctor)",
                    "3. Start System (Skipped Init)",
                    "4. Reset Database (Wipe All Data)",
                    "5. Exit",
                ]
            };

            let choice = ui::prompts::prompt_select("Select an action:", options);

            match choice {
                Ok(choice) => {
                    if choice.contains("Exit") {
                        break;
                    }

                    println!("{}", "───────────────────────────────────────────────────────────".truecolor(50, 50, 50));
                    
                    if choice.contains("Start System") {
                        if let Err(e) = handle_command(&Commands::Start).await {
                            ui::output::print_error(&e.to_string());
                        }
                    } else if choice.contains("Initialize") || choice.contains("Repair") {
                        if let Err(e) = handle_command(&Commands::Init).await {
                            ui::output::print_error(&e.to_string());
                        }
                    } else if choice.contains("Diagnostics") {
                        if let Err(e) = handle_command(&Commands::Doctor).await {
                            ui::output::print_error(&e.to_string());
                        }
                    } else if choice.contains("Backup") {
                        if let Err(e) = handle_command(&Commands::Backup).await {
                            ui::output::print_error(&e.to_string());
                        }
                    } else if choice.contains("License") {
                        if let Err(e) = handle_command(&Commands::License).await {
                            ui::output::print_error(&e.to_string());
                        }
                    } else if choice.contains("Reset Database") {
                        let confirm = inquire::Confirm::new("Are you sure you want to completely wipe the database? This action CANNOT BE UNDONE!")
                            .with_default(false)
                            .prompt();
                            
                        match confirm {
                            Ok(true) => {
                                ui::output::print_warning("Wiping database...");
                                let config = core::config::CliConfig::default();
                                let path = config.db_url.replace("sqlite://", "").replace("?mode=rwc", "").replace("?mode=ro", "");
                                if std::path::Path::new(&path).exists() {
                                    if let Err(e) = std::fs::remove_file(&path) {
                                        ui::output::print_error(&format!("Failed to delete database: {}", e));
                                    } else {
                                        ui::output::print_success("Database reset successfully. You can now re-initialize the system.");
                                    }
                                } else {
                                    ui::output::print_warning(&format!("Database file not found, nothing to reset. Path attempted: {}", path));
                                }
                            }
                            _ => {
                                println!("{}", "[-] Reset cancelled.".dimmed());
                            }
                        }
                    }

                    println!("\n{}", "Press Enter to return to menu...".dimmed());
                    let mut discard = String::new();
                    std::io::stdin().read_line(&mut discard).unwrap_or(0);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    Ok(())
}
