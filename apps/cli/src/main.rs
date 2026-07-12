use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

mod commands;
mod config;

#[derive(Parser)]
#[command(name = "orbis")]
#[command(about = "Orbis Management CLI", long_about = "Standalone CLI for setting up and managing the Orbis Educational System.")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
enum Commands {
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

use sea_orm::{Database, Statement, DatabaseBackend, ConnectionTrait};

async fn get_system_status() -> (bool, bool) {
    let config = crate::config::CliConfig::default();
    let db_url = config.db_url.replace("mode=rwc", "mode=ro");

    let Ok(db) = Database::connect(&db_url).await else {
        return (false, false);
    };
    
    // Check if organizations exist. If not, the CLI init hasn't been done.
    let org_res = db.query_one(Statement::from_string(
        DatabaseBackend::Sqlite,
        "SELECT COUNT(*) as cnt FROM organizations;".to_owned()
    )).await;
    
    let is_initialized = if let Ok(Some(row)) = org_res {
        let count: i64 = row.try_get("", "cnt").unwrap_or(0);
        count > 0
    } else {
        false
    };

    if !is_initialized {
        return (false, false); // Treat as not initialized
    }

    // Check if there are any academic_years, meaning Svelte onboarding has been done
    let res = db.query_one(Statement::from_string(
        DatabaseBackend::Sqlite,
        "SELECT COUNT(*) as cnt FROM academic_years;".to_owned()
    )).await;
    
    if let Ok(Some(row)) = res {
        let count: i64 = row.try_get("", "cnt").unwrap_or(0);
        return (true, count > 0);
    }
    
    (true, false)
}

fn get_environment() -> (&'static str, colored::ColoredString) {
    let env_var = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    match env_var.to_lowercase().as_str() {
        "production" | "prod" => ("Production", "Production".green()),
        "staging" | "stage" => ("Staging", "Staging".yellow()),
        "testing" | "test" => ("Testing", "Testing".magenta()),
        _ => ("Development", "Development".cyan()),
    }
}

async fn print_banner() -> (bool, bool) {
    // Clear the terminal to prevent scrolling bugs with the interactive menu
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let orbis_art = r#"
   ____  ____  ____  _________   ________    ____
  / __ \/ __ \/ __ )/  _/ ___/  / ____/ /   /  _/
 / / / / /_/ / __  |/ / \__ \  / /   / /    / /  
/ /_/ / _, _/ /_/ // / ___/ / / /___/ /____/ /   
\____/_/ |_/_____/___//____/  \____/_____/___/   
"#;

    let (db_exists, setup_completed) = get_system_status().await;
    let status_text = if setup_completed {
        "Fully Initialized".green()
    } else if db_exists {
        "Pending Onboarding".yellow()
    } else {
        "Not Initialized".red()
    };

    let (_, env_colored) = get_environment();

    println!("\n{}", "╭──────────────────────────────────────────────────────────╮".truecolor(100, 100, 100));
    println!("{}", orbis_art.bright_cyan().bold());
    println!("  {}", "Developed by mulverse".truecolor(180, 0, 255).italic());
    println!("  {} {}  |  {} {}", "Status:".dimmed(), status_text, "Environment:".dimmed(), env_colored);
    println!("{}", "╰──────────────────────────────────────────────────────────╯\n".truecolor(100, 100, 100));

    (db_exists, setup_completed)
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
        print_banner().await;
        handle_command(cmd).await?;
    } else {
        // Interactive Launcher Mode (when double-clicked without arguments)
        use inquire::Select;
        
        loop {
            let (db_exists, setup_completed) = print_banner().await;
            
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

            let choice = Select::new("Select an action:", options)
                .with_help_message("Use arrow keys to navigate, Enter to select")
                .prompt();

            match choice {
                Ok(choice) => {
                    if choice.contains("Exit") {
                        break;
                    }

                    // Print a separator before running the command
                    println!("{}", "───────────────────────────────────────────────────────────".truecolor(50, 50, 50));
                    
                    if choice.contains("Start System") {
                        if let Err(e) = handle_command(&Commands::Start).await {
                            println!("{} {}", "Error:".red(), e);
                        }
                    } else if choice.contains("Initialize") || choice.contains("Repair") {
                        if let Err(e) = handle_command(&Commands::Init).await {
                            println!("{} {}", "Error:".red(), e);
                        }
                    } else if choice.contains("Diagnostics") {
                        if let Err(e) = handle_command(&Commands::Doctor).await {
                            println!("{} {}", "Error:".red(), e);
                        }
                    } else if choice.contains("Backup") {
                        if let Err(e) = handle_command(&Commands::Backup).await {
                            println!("{} {}", "Error:".red(), e);
                        }
                    } else if choice.contains("License") {
                        if let Err(e) = handle_command(&Commands::License).await {
                            println!("{} {}", "Error:".red(), e);
                        }
                    } else if choice.contains("Reset Database") {
                        let confirm = inquire::Confirm::new("Are you sure you want to completely wipe the database? This action CANNOT BE UNDONE!")
                            .with_default(false)
                            .prompt();
                            
                        match confirm {
                            Ok(true) => {
                                println!("{}", "[*] Wiping database...".red().bold());
                                let config = crate::config::CliConfig::default();
                                let path = config.db_url.replace("sqlite://", "").replace("?mode=rwc", "").replace("?mode=ro", "");
                                if std::path::Path::new(&path).exists() {
                                    if let Err(e) = std::fs::remove_file(&path) {
                                        println!("{} Failed to delete database: {}", "Error:".red(), e);
                                    } else {
                                        println!("{}", "[+] Database reset successfully. You can now re-initialize the system.".green());
                                    }
                                } else {
                                    println!("{} Path attempted: {}", "[-] Database file not found, nothing to reset.".yellow(), path);
                                }
                            }
                            _ => {
                                println!("{}", "[-] Reset cancelled.".dimmed());
                            }
                        }
                    }

                    // Pause before showing the menu again
                    println!("\n{}", "Press Enter to return to menu...".dimmed());
                    let mut discard = String::new();
                    std::io::stdin().read_line(&mut discard).unwrap_or(0);
                    
                    // Use standard cross-platform screen clear (if possible) or just print newlines
                    println!("\n\n\n");
                }
                Err(_) => {
                    // User pressed Ctrl+C or Esc
                    break;
                }
            }
        }
    }

    Ok(())
}
