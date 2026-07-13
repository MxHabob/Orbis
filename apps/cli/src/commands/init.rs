use anyhow::{anyhow, Result};
use colored::*;
use sea_orm::{Database, ConnectionTrait, Statement, DatabaseBackend};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

use crate::core::config::CliConfig;
use crate::core::database;
use crate::core::github;
use crate::core::seeder;
use crate::ui::{output, prompts};

pub async fn execute() -> Result<()> {
    let (db_exists, setup_completed) = database::get_system_status().await;

    if setup_completed {
        output::print_success("The system is already fully configured and ready.");
        output::print_info("Action: Run `cli start` (or select 'Start System' from the menu) to boot.");
        return Ok(());
    } else if db_exists {
        output::print_warning("The core database exists but Initial Setup (Onboarding) is pending.");
        output::print_info("Action: Run `cli start` to boot the application and complete the setup from the Desktop UI.");
        return Ok(());
    }

    println!("Welcome to the {}. You are currently not initialized.\n", "Orbis CLI".truecolor(0, 204, 255).bold());

    // 1. Database Selection
    let db_options = vec!["1. SQLite (Local/Offline)", "2. PostgreSQL (Enterprise/Cloud)"];
    let db_choice = prompts::prompt_select("Select database provider:", db_options)?;

    let config = CliConfig::default();
    let db_url = if db_choice.contains("SQLite") {
        output::print_step("Setting up local SQLite databases...");
        config.db_url.clone()
    } else {
        prompts::prompt_text("Enter PostgreSQL Connection String (e.g. postgres://user:pass@localhost/orbis):")?
    };

    println!("\n{}", "--- Orbis Initial Setup ---".truecolor(0, 204, 255).bold());
    let org_name = prompts::prompt_text("Enter Organization Name (e.g. My School):")?;
    let admin_name = prompts::prompt_text("Enter Admin Full Name:")?;
    let admin_username = prompts::prompt_text("Enter Admin Username (or Email):")?;
    let admin_password = prompts::prompt_password("Enter Admin Password:", Some("Must be strong"))?;

    output::print_step("Creating database and running schema migrations...");
    let db = Database::connect(&db_url).await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    
    let schema_sql = include_str!("../../../api-rust/crates/infra-persistence/src/schemas/schema_baseline.sql");
    db.execute_unprepared(schema_sql).await?;

    let org_id = Uuid::new_v4().to_string();
    let now = chrono::Local::now().to_rfc3339();
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO organizations (id, name, is_active, subscription_plan, created_at, updated_at) VALUES (?, ?, 1, 'basic', ?, ?)",
        vec![org_id.clone().into(), org_name.into(), now.clone().into(), now.clone().into()]
    )).await?;

    let user_id = Uuid::new_v4().to_string();
    let hashed_password = hash(&admin_password, DEFAULT_COST).unwrap_or_default();
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO users (id, org_id, email, full_name, password_hash, status, primary_role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'active', 'super_admin', ?, ?)",
        vec![user_id.into(), org_id.clone().into(), admin_username.into(), admin_name.into(), hashed_password.into(), now.clone().into(), now.into()]
    )).await?;

    output::print_success("Organization and Admin account created successfully!");
    
    // Seed Period Templates, Currencies, Chart of Accounts
    if let Err(e) = seeder::seed_essential_data(&db, &org_id).await {
        output::print_error(&format!("Failed to seed essential data: {}", e));
    }

    let env_var = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    if env_var.to_lowercase() == "development" || env_var.to_lowercase() == "dev" {
        output::print_info("Development Environment Detected.");
        println!("{}", "Skipping Orbis Desktop download from GitHub.".dimmed());
        output::print_success("You can now start the application locally using the menu.");
        return Ok(());
    }

    println!("\n{}", "Orbis Desktop is distributed via GitHub Releases.".cyan());
    let token = prompts::prompt_password("Enter GitHub Personal Access Token (Leave empty for public repositories):", None)?;

    output::print_step("Fetching latest release from GitHub...");
    if let Err(e) = github::download_latest_release(&token).await {
        output::print_error(&e.to_string());
    }

    output::print_success("Initialization complete! Run `cli start` to boot the system.");
    Ok(())
}
