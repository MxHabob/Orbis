use anyhow::{anyhow, Result};
use colored::*;
use inquire::{Select, Text, Password};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Client;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use sea_orm::{Database, ConnectionTrait, Statement, DatabaseBackend};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

const REPO_OWNER: &str = "orbis-edu";
const REPO_NAME: &str = "orbis";

#[derive(Deserialize, Debug)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    url: String,
}

pub async fn execute() -> Result<()> {
    // --- Smart Detection ---
    let config = crate::config::CliConfig::default();
    let ro_url = config.db_url.replace("mode=rwc", "mode=ro");
    if Database::connect(ro_url.clone()).await.is_ok() {
        let db_r = Database::connect(ro_url).await?;
        
        // Check if organizations exist
        let org_res = db_r.query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "SELECT COUNT(*) as cnt FROM organizations;".to_owned()
        )).await;
        
        let has_org = if let Ok(Some(row)) = org_res {
            row.try_get::<i64>("", "cnt").unwrap_or(0) > 0
        } else {
            false
        };

        if has_org {
            println!("\n{}", "[!] Existing Installation Detected!".yellow().bold());
            let res = db_r.query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT COUNT(*) as cnt FROM academic_years;".to_owned()
            )).await;
            
            if let Ok(Some(row)) = res {
                let count: i64 = row.try_get("", "cnt").unwrap_or(0);
                if count > 0 {
                    println!("{}", "The system is already fully configured and ready.".green());
                    println!("{}", "Action: Run `cli start` (or select 'Start System' from the menu) to boot.".cyan());
                    return Ok(());
                } else {
                    println!("{}", "The core database exists but Initial Setup (Onboarding) is pending.".yellow());
                    println!("{}", "Action: Run `cli start` to boot the application and complete the setup from the Desktop UI.".cyan());
                    return Ok(());
                }
            }
        }
    }

    println!("Welcome to the {}. You are currently not initialized.\n", "Orbis CLI".truecolor(0, 204, 255).bold());

    // 1. Database Selection
    let db_options = vec!["1. SQLite (Local/Offline)", "2. PostgreSQL (Enterprise/Cloud)"];
    let db_choice = Select::new("Select database provider:", db_options)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .prompt()?;

    let db_url = if db_choice.contains("SQLite") {
        println!("{}", "Setting up local SQLite databases...".green());
        config.db_url.clone()
    } else {
        Text::new("Enter PostgreSQL Connection String (e.g. postgres://user:pass@localhost/orbis):")
            .prompt()?
    };

    println!("{}", "[*] Running database migrations...".cyan());
    
    // Prompt for Organization and Admin Details
    println!("\n{}", "--- Orbis Initial Setup ---".truecolor(0, 204, 255).bold());
    let org_name = Text::new("Enter Organization Name (e.g. My School):").prompt()?;
    let admin_name = Text::new("Enter Admin Full Name:").prompt()?;
    let admin_username = Text::new("Enter Admin Username (or Email):").prompt()?;
    let admin_password = Password::new("Enter Admin Password:")
        .with_help_message("Must be strong")
        .without_confirmation()
        .prompt()?;

    // Create DB and run schema
    println!("{}", "[*] Creating database and running migrations...".cyan());
    
    // SQLite connection string with create mode
    let db = Database::connect(&db_url).await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    
    let schema_sql = include_str!("../../../api-rust/crates/infra-persistence/src/schemas/schema_baseline.sql");
    db.execute_unprepared(schema_sql).await?;

    // Insert Organization
    let org_id = Uuid::new_v4().to_string();
    let now = chrono::Local::now().to_rfc3339();
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO organizations (id, name, is_active, subscription_plan, created_at, updated_at) VALUES (?, ?, 1, 'basic', ?, ?)",
        vec![org_id.clone().into(), org_name.into(), now.clone().into(), now.clone().into()]
    )).await?;

    // Insert Admin User
    let user_id = Uuid::new_v4().to_string();
    let hashed_password = hash(&admin_password, DEFAULT_COST).unwrap_or_default();
    db.execute(Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        "INSERT INTO users (id, org_id, username, full_name, password_hash, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 1, ?, ?)",
        vec![user_id.into(), org_id.into(), admin_username.into(), admin_name.into(), hashed_password.into(), now.clone().into(), now.into()]
    )).await?;

    println!("{}", "[+] Organization and Admin account created successfully!".green().bold());
    println!("{}", "[*] Note: Login to the system to complete the rest of the onboarding process.".yellow());

    // --- 2. Check Environment before downloading ---
    let env_var = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    if env_var.to_lowercase() == "development" || env_var.to_lowercase() == "dev" {
        println!("\n{}", "[*] Development Environment Detected.".cyan());
        println!("{}", "Skipping Orbis Desktop download from GitHub.".dimmed());
        println!("{}", "You can now start the application locally using the menu or by running `bun run tauri dev`.".green());
        return Ok(());
    }

    // --- 3. Download Desktop App from GitHub (Production Only) ---
    println!("\n{}", "Orbis Desktop is distributed via GitHub Releases.".cyan());
    
    let token = Password::new("Enter GitHub Personal Access Token (Leave empty for public repositories):")
        .without_confirmation()
        .prompt()?;

    println!("{} fetching latest release from GitHub...", "[*]".cyan());

    let client = Client::new();
    let mut req = client
        .get(format!("https://api.github.com/repos/{}/{}/releases/latest", REPO_OWNER, REPO_NAME))
        .header(USER_AGENT, "orbis-cli/1.0")
        .header(ACCEPT, "application/vnd.github.v3+json");

    if !token.is_empty() {
        req = req.header(AUTHORIZATION, format!("Bearer {}", token));
    }

    let res = req.send().await?;

    if !res.status().is_success() {
        return Err(anyhow!("Failed to fetch release from GitHub. Status: {}. Ensure your token is valid if the repo is private.", res.status()));
    }

    let release: GitHubRelease = res.json().await?;
    println!("{} Found release: {}", "[+]".green(), release.tag_name.bold());

    // Find the Windows installer (.msi or .exe or .zip)
    // For now, let's look for a generic .zip or .msi
    let asset = release.assets.into_iter().find(|a| a.name.ends_with(".msi") || a.name.ends_with(".zip") || a.name.ends_with(".exe"));

    match asset {
        Some(asset) => {
            println!("{} Downloading asset: {}", "[v]".cyan(), asset.name);
            download_asset(&client, &asset, &token).await?;
        },
        None => {
            println!("{} No suitable desktop binary found in the latest release.", "[!]".red());
        }
    }

    println!("\n{}", "[*] Initialization complete! Run `cli start` to boot the system.".green().bold());
    Ok(())
}

async fn download_asset(client: &Client, asset: &GitHubAsset, token: &str) -> Result<()> {
    let mut req = client.get(&asset.url)
        .header(USER_AGENT, "orbis-cli/1.0")
        .header(ACCEPT, "application/octet-stream"); // Required to download the actual file, not JSON

    if !token.is_empty() {
        req = req.header(AUTHORIZATION, format!("Bearer {}", token));
    }

    let res = req.send().await?;

    if !res.status().is_success() {
        return Err(anyhow!("Failed to download asset. Status: {}", res.status()));
    }

    let total_size = res.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let file_path = Path::new(&asset.name);
    let mut file = File::create(file_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message("Download complete");
    println!("{} Saved to: {:?}", "[+]".green(), file_path.canonicalize()?);

    Ok(())
}
