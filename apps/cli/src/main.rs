use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod commands;
pub mod core;
pub mod ui;

#[derive(Parser)]
#[command(name = "orbis")]
#[command(
    about = "Orbis Management CLI",
    long_about = "Unified CLI for deploying, licensing, and managing the Orbis Educational System."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Guided setup for Docker or Native host, database, migrations, and license
    Init,
    /// Start the Orbis API server
    Start,
    /// Stop the Orbis API server
    Stop,
    /// Restart the Orbis API server
    Restart,
    /// Stream API server logs
    Logs {
        #[arg(short, long, default_value_t = 100)]
        lines: u32,
    },
    /// Pull latest API image, run migrations, and restart container
    Upgrade {
        #[arg(long)]
        image: Option<String>,
    },
    /// Manage Orbis license (status, import, activate, refresh, deactivate)
    License {
        #[command(subcommand)]
        action: Option<commands::license::LicenseCommand>,
    },
    /// Run comprehensive diagnostics on API, database, and license
    Doctor,
    /// Backup the configured database (PostgreSQL or SQLite)
    Backup,
    /// Restore the configured database from a backup file
    Restore { file: std::path::PathBuf },
}

#[derive(Debug, Clone, PartialEq)]
enum MenuChoice {
    Init,
    Start,
    Stop,
    Restart,
    Logs,
    License,
    Doctor,
    Backup,
    LaunchDesktop,
    ResetDatabase,
    Exit,
}

struct MenuItem {
    label: String,
    choice: MenuChoice,
}

async fn handle_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Init => commands::init::execute().await,
        Commands::Start => commands::start::execute().await,
        Commands::Stop => commands::start::stop().await,
        Commands::Restart => commands::start::restart().await,
        Commands::Logs { lines } => commands::start::logs(*lines).await,
        Commands::Upgrade { image } => commands::start::upgrade(image.clone()).await,
        Commands::License { action } => commands::license::execute(action.clone()).await,
        Commands::Doctor => commands::doctor::execute().await,
        Commands::Backup => commands::backup::execute().await,
        Commands::Restore { file } => commands::backup::restore(file).await,
    }
}

fn build_menu(has_state: bool, status: &core::database::SystemStatus) -> Vec<MenuItem> {
    let mut items = Vec::new();
    let mut idx = 1u32;
    let mut push = |label: &str, choice: MenuChoice| {
        items.push(MenuItem {
            label: format!("{}. {}", idx, label),
            choice,
        });
        idx += 1;
    };

    if has_state {
        push("Start API Server", MenuChoice::Start);
        push("Stop API Server", MenuChoice::Stop);
        push("Restart API Server", MenuChoice::Restart);
        push("Stream API Logs", MenuChoice::Logs);
        push("Manage License", MenuChoice::License);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
        push("Backup Database", MenuChoice::Backup);
        push("Launch Desktop App", MenuChoice::LaunchDesktop);
        push("Reconfigure Environment (Init)", MenuChoice::Init);
        if status.db_exists {
            push("Reset Database Store", MenuChoice::ResetDatabase);
        }
    } else {
        push("Initialize Environment (Setup Wizard)", MenuChoice::Init);
        push("Run Diagnostics (Doctor)", MenuChoice::Doctor);
    }

    push("Exit Console", MenuChoice::Exit);
    items
}

async fn handle_reset_database() {
    let Some(state) = core::config::CliState::load() else {
        ui::output::print_warning("No environment configured yet.");
        return;
    };

    let Ok(true) = inquire::Confirm::new(
        "Are you sure you want to wipe the database? This action CANNOT BE UNDONE!",
    )
    .with_default(false)
    .prompt() else {
        ui::output::print_info("Database reset cancelled.");
        return;
    };

    ui::output::print_warning("Wiping database...");
    match core::database::reset_database(&state.host_database_url()).await {
        Ok(()) => ui::output::print_success("Database wiped. Run `orbis init` to re-initialize."),
        Err(e) => ui::output::print_error(&format!("Failed to reset database: {e}")),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    if let Some(ref command) = cli.command {
        if let Err(e) = handle_command(command).await {
            ui::output::print_error(&format!("Error: {e}"));
            std::process::exit(1);
        }
        return Ok(());
    }

    loop {
        let has_state = core::config::CliState::load().is_some();
        let status = core::database::get_full_status().await;
        ui::output::print_banner(status.db_exists, status.is_setup_complete());

        let menu = build_menu(has_state, &status);
        let labels: Vec<&str> = menu.iter().map(|m| m.label.as_str()).collect();
        let Ok(selection) = ui::prompts::prompt_select("Select an action:", labels) else {
            break;
        };
        let selection = selection.to_string();

        let Some(item) = menu.into_iter().find(|m| m.label == selection) else {
            break;
        };

        let res = match item.choice {
            MenuChoice::Init => commands::init::execute().await,
            MenuChoice::Start => commands::start::execute().await,
            MenuChoice::Stop => commands::start::stop().await,
            MenuChoice::Restart => commands::start::restart().await,
            MenuChoice::Logs => commands::start::logs(100).await,
            MenuChoice::License => commands::license::execute(None).await,
            MenuChoice::Doctor => commands::doctor::execute().await,
            MenuChoice::Backup => commands::backup::execute().await,
            MenuChoice::LaunchDesktop => commands::start::launch_desktop(),
            MenuChoice::ResetDatabase => {
                handle_reset_database().await;
                Ok(())
            }
            MenuChoice::Exit => break,
        };

        if let Err(e) = res {
            ui::output::print_error(&format!("Action failed: {e}"));
        }

        println!();
        let _ = inquire::Text::new("Press Enter to continue...").prompt();
    }

    Ok(())
}
