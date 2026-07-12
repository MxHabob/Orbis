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

fn print_banner() {
    let orbis_art = r#"
   ____       __    _     
  / __ \_____/ /_  (_)____
 / / / / ___/ __ \/ / ___/
/ /_/ / /  / /_/ / (__  ) 
\____/_/  /_.___/_/____/  
"#;

    println!("\n{}", "╭──────────────────────────────────────────────────────────╮".truecolor(100, 100, 100));
    println!("{}", orbis_art.truecolor(0, 204, 255).bold());
    println!("\n  {} {}", "Welcome to the".dimmed(), "Orbis Management System!".bold());
    println!("  {}", "Developed by mulverse".truecolor(180, 0, 255).italic());
    println!("  {} {}  |  {} {}", "Status:".dimmed(), "Not Initialized".red(), "Environment:".dimmed(), "Production".green());
    println!("{}", "╰──────────────────────────────────────────────────────────╯\n".truecolor(100, 100, 100));
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

    // Configure basic tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    
    if let Some(cmd) = &cli.command {
        // Run specific command passed via CLI args directly
        print_banner();
        handle_command(cmd).await?;
    } else {
        // Interactive Launcher Mode (when double-clicked without arguments)
        use inquire::Select;
        
        loop {
            print_banner();
            
            let options = vec![
                "1. Start System",
                "2. Initialize / Download App",
                "3. Run Diagnostics (Doctor)",
                "4. Backup Database",
                "5. Manage License",
                "6. Exit"
            ];

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
                    } else if choice.contains("Initialize") {
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
