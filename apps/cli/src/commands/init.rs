use anyhow::{anyhow, Result};
use colored::*;
use sea_orm::{Database, ConnectionTrait};
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
    let generated_slug = transliterate_arabic_to_ascii(&org_name);
    let org_slug = prompts::prompt_text_with_default("Enter Organization Slug (e.g. my-school):", &generated_slug)?;
    let admin_name = prompts::prompt_text("Enter Admin Full Name:")?;
    let admin_username = prompts::prompt_text("Enter Admin Username (or Email):")?;
    let admin_password = prompts::prompt_password("Enter Admin Password:", Some("Must be strong"))?;

    output::print_step("Creating database and running schema migrations...");
    let db = Database::connect(&db_url).await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    
    let schema_sql = include_str!("../../../api-rust/crates/infra-persistence/src/schemas/schema_baseline.sql");
    db.execute_unprepared(schema_sql).await?;

    let blueprint_options = vec!["1. School (K-12 / Basic Education)", "2. University (Higher Education / Credit Hours)", "3. Training Center / Institute (Quarterly / Pass-Fail)"];
    let bp_choice = prompts::prompt_select("Select organization blueprint:", blueprint_options)?;
    let blueprint = if bp_choice.contains("University") {
        infra_persistence::bootstrap::BlueprintType::University
    } else if bp_choice.contains("Training") {
        infra_persistence::bootstrap::BlueprintType::TrainingCenter
    } else {
        infra_persistence::bootstrap::BlueprintType::School
    };

    let hashed_password = hash(&admin_password, DEFAULT_COST).unwrap_or_default();
    let params = infra_persistence::bootstrap::BootstrapParams {
        org_name,
        org_slug,
        admin_name,
        admin_username,
        admin_password_hash: hashed_password,
        blueprint,
    };
    
    let result = infra_persistence::bootstrap::bootstrap_system(&db, params).await?;
    let org_id = result.org_id;

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

fn transliterate_arabic_to_ascii(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            'ا' | 'أ' | 'إ' | 'آ' | 'ى' | 'ء' | 'ئ' | 'ؤ' => result.push('a'),
            'ب' => result.push('b'),
            'ت' | 'ة' => result.push('t'),
            'ث' => result.push_str("th"),
            'ج' => result.push('j'),
            'ح' => result.push('h'),
            'خ' => result.push_str("kh"),
            'د' => result.push('d'),
            'ذ' => result.push_str("dh"),
            'ر' => result.push('r'),
            'ز' => result.push('z'),
            'س' => result.push('s'),
            'ش' => result.push_str("sh"),
            'ص' => result.push('s'),
            'ض' => result.push('d'),
            'ط' => result.push('t'),
            'ظ' => result.push_str("zh"),
            'ع' => result.push('a'),
            'غ' => result.push_str("gh"),
            'ف' => result.push('f'),
            'ق' => result.push('q'),
            'ك' => result.push('k'),
            'ل' => result.push('l'),
            'م' => result.push('m'),
            'ن' => result.push('n'),
            'ه' => result.push('h'),
            'و' => result.push('w'),
            'ي' => result.push('y'),
            other => {
                if other.is_ascii_alphanumeric() {
                    result.push(other.to_ascii_lowercase());
                } else {
                    result.push('-');
                }
            }
        }
    }
    
    // Clean up multiple consecutive hyphens, and leading/trailing hyphens
    let mut cleaned = String::new();
    let mut last_was_hyphen = false;
    for c in result.chars() {
        if c == '-' {
            if !last_was_hyphen {
                cleaned.push('-');
                last_was_hyphen = true;
            }
        } else {
            cleaned.push(c);
            last_was_hyphen = false;
        }
    }
    
    cleaned.trim_matches('-').to_string()
}
