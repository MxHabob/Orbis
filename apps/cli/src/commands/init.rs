use anyhow::{anyhow, Result};
use colored::*;
use sea_orm::{Database, ConnectionTrait};
use bcrypt::{hash, DEFAULT_COST};

use crate::core::config::CliConfig;
use crate::core::database;
use crate::core::github;
use crate::core::license;
use crate::core::seeder;
use crate::ui::{output, prompts};

const TOTAL_PHASES: u8 = 6;

pub async fn execute() -> Result<()> {
    let status = database::get_full_status().await;

    if status.is_setup_complete() {
        output::print_success("The system is already fully configured and ready.");
        output::print_info("Action: Run `orbis start` (or select 'Start System' from the menu) to boot.");
        return Ok(());
    } else if status.is_initialized() {
        output::print_warning("The core database exists but Initial Setup (Onboarding) is pending.");
        output::print_info("Action: Run `orbis start` to boot the application and complete the setup from the Desktop UI.");
        return Ok(());
    }

    println!(
        "\n  Welcome to the {}. Let's get you set up.\n",
        "Orbis Setup Wizard".truecolor(0, 204, 255).bold()
    );

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 1: Product Activation
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(1, TOTAL_PHASES, "Product Activation");

    let license_info = match license::check_license_status() {
        license::LicenseStatus::Active => {
            let info = license::load_license()?;
            output::print_success(&format!(
                "License already active: {} (expires in {} days)",
                info.plan, info.days_remaining()
            ));
            info
        }
        _ => {
            let activation_options = vec![
                "1. Enter License Key",
                "2. Start Free Trial (30 days)",
            ];
            let choice = prompts::prompt_select("How would you like to activate?", activation_options)?;

            if choice.contains("Trial") {
                let info = license::activate_trial()?;
                output::print_success(&format!(
                    "Trial activated! Plan: {} | Expires: {} days",
                    info.plan, info.days_remaining()
                ));
                info
            } else {
                loop {
                    let key = prompts::prompt_license_key("Enter License Key (XXXX-XXXX-XXXX-XXXX):")?;
                    match license::activate_license(&key) {
                        Ok(info) => {
                            output::print_success(&format!(
                                "License activated! Plan: {} | Max Users: {} | Max Students: {}",
                                info.plan, info.max_users, info.max_students
                            ));
                            break info;
                        }
                        Err(e) => {
                            output::print_error(&format!("Activation failed: {}", e));
                            let retry = prompts::prompt_confirm("Try again?", true)?;
                            if !retry {
                                return Err(anyhow!("License activation cancelled."));
                            }
                        }
                    }
                }
            }
        }
    };

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 2: Infrastructure Setup
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(2, TOTAL_PHASES, "Infrastructure Setup");

    let db_options = vec!["1. SQLite (Recommended for local/offline)", "2. PostgreSQL (Enterprise/Cloud)"];
    let db_choice = prompts::prompt_select("Select database provider:", db_options)?;

    let config = CliConfig::default();
    let db_url = if db_choice.contains("SQLite") {
        output::print_progress_ok("SQLite selected — local storage");
        config.db_url.clone()
    } else {
        prompts::prompt_text("Enter PostgreSQL Connection String (e.g. postgres://user:pass@localhost/orbis):")?
    };

    output::print_progress_pending("Creating database and running schema migrations...");
    let db = Database::connect(&db_url).await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;

    let schema_sql = include_str!("../../../api-rust/crates/infra-persistence/src/schemas/schema_baseline.sql");
    db.execute_unprepared(schema_sql).await?;
    output::print_progress_ok("Database schema created successfully");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 3: Organization Setup
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(3, TOTAL_PHASES, "Organization Setup");

    let org_name = prompts::prompt_org_name("Organization Name (EN):")?;
    let org_name_ar = prompts::prompt_org_name("Organization Name (AR):")?;
    let generated_slug = transliterate_arabic_to_ascii(&org_name);
    let org_slug = prompts::prompt_slug("Organization Slug:", &generated_slug)?;

    // Institution Type / Blueprint
    let blueprint_options = vec![
        "1. School (K-12 / Basic Education)",
        "2. University (Higher Education / Credit Hours)",
        "3. Training Center / Institute (Quarterly / Pass-Fail)",
    ];
    let bp_choice = prompts::prompt_select("Select institution type:", blueprint_options)?;
    let blueprint = if bp_choice.contains("University") {
        infra_persistence::bootstrap::BlueprintType::University
    } else if bp_choice.contains("Training") {
        infra_persistence::bootstrap::BlueprintType::TrainingCenter
    } else {
        infra_persistence::bootstrap::BlueprintType::School
    };

    let institution_type_label = if bp_choice.contains("University") {
        "University"
    } else if bp_choice.contains("Training") {
        "Training Center"
    } else {
        "School"
    };

    // Auto-detect locale defaults
    let detected_tz = detect_timezone();
    let detected_currency = detect_currency(&detected_tz);
    output::print_info(&format!("Detected timezone: {} | Currency: {}", detected_tz, detected_currency));

    let timezone = prompts::prompt_text_with_default("Timezone:", &detected_tz)?;
    let currency = prompts::prompt_text_with_default("Base Currency:", &detected_currency)?;

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 4: Administrator Account
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(4, TOTAL_PHASES, "Administrator Account");

    let admin_name = prompts::prompt_full_name("Admin Full Name:")?;
    let admin_username = prompts::prompt_email("Admin Email:")?;
    let admin_password = prompts::prompt_password_confirmed("Admin Password:")?;

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 5: Execution
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(5, TOTAL_PHASES, "Initializing System");

    let bp_type_str = match blueprint {
        infra_persistence::bootstrap::BlueprintType::School => "school",
        infra_persistence::bootstrap::BlueprintType::University => "university",
        infra_persistence::bootstrap::BlueprintType::TrainingCenter => "training_center",
    };

    // 5a. Bootstrap organization + admin
    output::print_progress_pending("Creating organization and admin account...");
    let hashed_password = hash(&admin_password, DEFAULT_COST).unwrap_or_default();
    let params = infra_persistence::bootstrap::BootstrapParams {
        org_name: org_name.clone(),
        org_slug: org_slug.clone(),
        admin_name: admin_name.clone(),
        admin_username: admin_username.clone(),
        admin_password_hash: hashed_password,
        blueprint,
        currency: Some(currency.clone()),
    };

    let result = infra_persistence::bootstrap::bootstrap_system(&db, params).await?;
    let org_id = result.org_id;
    output::print_progress_ok("Organization and admin account created");

    // 5b. Update organization with additional fields (timezone, name_ar, settings)
    // Note: currency is already set by bootstrap_full via the payload.
    output::print_progress_pending("Updating organization settings...");
    let settings_json = serde_json::json!({
        "registration_config": {
            "institution_type": bp_type_str
        }
    }).to_string();

    db.execute(sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(),
        "UPDATE organizations SET name_ar = $1, timezone = $2, locale = 'ar', settings = $3 WHERE id = $4",
        vec![
            org_name_ar.clone().into(),
            timezone.clone().into(),
            settings_json.into(),
            org_id.clone().into(),
        ]
    )).await?;
    output::print_progress_ok("Organization settings updated");

    // 5c. Core data (currencies, grade scales, permissions, period templates)
    //     is created by bootstrap_full(). Only Chart of Accounts needs the seeder.
    output::print_progress_pending("Seeding Chart of Accounts...");
    match seeder::seed_chart_of_accounts(&db, &org_id).await {
        Ok(()) => output::print_progress_ok("Chart of Accounts seeded"),
        Err(e) => output::print_progress_fail(&format!("COA seeding failed: {}", e)),
    }

    // 5d. Store license limits in organization
    output::print_progress_pending("Applying license limits...");
    db.execute(sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(),
        "UPDATE organizations SET max_users = $1, max_students = $2, subscription_plan = $3 WHERE id = $4",
        vec![
            (license_info.max_users as i64).into(),
            (license_info.max_students as i64).into(),
            license_info.plan.to_string().to_lowercase().into(),
            org_id.clone().into(),
        ]
    )).await?;
    output::print_progress_ok(&format!("License limits applied (Plan: {})", license_info.plan));

    // 5e. Desktop download (production only)
    let env_var = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    if env_var.to_lowercase() == "development" || env_var.to_lowercase() == "dev" {
        output::print_info("Development environment — skipping desktop download.");
    } else {
        output::print_progress_pending("Checking for desktop application...");
        let possible_paths = CliConfig::get_app_executable_paths();
        let app_exists = possible_paths.iter().any(|p| p.exists());

        if app_exists {
            output::print_progress_ok("Desktop application found");
        } else {
            println!("\n{}", "Orbis Desktop is distributed via GitHub Releases.".cyan());
            let token = prompts::prompt_password(
                "Enter GitHub Personal Access Token (Leave empty for public repos):",
                None,
            )?;

            output::print_progress_pending("Fetching latest release from GitHub...");
            match github::download_latest_release(&token).await {
                Ok(()) => output::print_progress_ok("Desktop application downloaded"),
                Err(e) => output::print_progress_fail(&format!("Download failed: {}", e)),
            }
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 6: Summary
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(6, TOTAL_PHASES, "Summary");

    let db_provider = if db_url.starts_with("sqlite") { "SQLite" } else { "PostgreSQL" };

    output::print_summary_box("System Initialized Successfully!", &[
        ("Organization:", &org_name),
        ("Organization (AR):", &org_name_ar),
        ("Slug:", &org_slug),
        ("Admin:", &admin_username),
        ("Database:", db_provider),
        ("Institution:", institution_type_label),
        ("License:", &license_info.plan.to_string()),
        ("Timezone:", &timezone),
        ("Currency:", &currency),
    ]);

    println!("  {}", "Next Steps:".bold());
    println!("  1. Run {} to boot the system", "orbis start".cyan().bold());
    println!("  2. The Desktop app will guide you through academic setup");
    println!("     (calendar, classes, subjects, etc.)\n");

    Ok(())
}

// ── Helper Functions ──────────────────────────────────────────────────

fn detect_timezone() -> String {
    if let Ok(tz) = std::env::var("TZ") {
        if !tz.is_empty() {
            return tz;
        }
    }
    "Asia/Riyadh".to_string()
}

fn detect_currency(timezone: &str) -> String {
    let tz_lower = timezone.to_lowercase();
    if tz_lower.contains("riyadh") || tz_lower.contains("saudi") {
        "SAR".to_string()
    } else if tz_lower.contains("dubai") || tz_lower.contains("uae") || tz_lower.contains("muscat") {
        "AED".to_string()
    } else if tz_lower.contains("aden") || tz_lower.contains("yemen") || tz_lower.contains("sanaa") {
        "YER".to_string()
    } else if tz_lower.contains("cairo") || tz_lower.contains("egypt") {
        "EGP".to_string()
    } else if tz_lower.contains("doha") || tz_lower.contains("qatar") {
        "QAR".to_string()
    } else if tz_lower.contains("shanghai") || tz_lower.contains("beijing") {
        "CNY".to_string()
    } else {
        "USD".to_string()
    }
}

fn transliterate_arabic_to_ascii(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '\u{0627}' | '\u{0623}' | '\u{0625}' | '\u{0622}' | '\u{0649}' | '\u{0621}' | '\u{0626}' | '\u{0624}' => result.push('a'),
            '\u{0628}' => result.push('b'),
            '\u{062A}' | '\u{0629}' => result.push('t'),
            '\u{062B}' => result.push_str("th"),
            '\u{062C}' => result.push('j'),
            '\u{062D}' => result.push('h'),
            '\u{062E}' => result.push_str("kh"),
            '\u{062F}' => result.push('d'),
            '\u{0630}' => result.push_str("dh"),
            '\u{0631}' => result.push('r'),
            '\u{0632}' => result.push('z'),
            '\u{0633}' => result.push('s'),
            '\u{0634}' => result.push_str("sh"),
            '\u{0635}' => result.push('s'),
            '\u{0636}' => result.push('d'),
            '\u{0637}' => result.push('t'),
            '\u{0638}' => result.push_str("zh"),
            '\u{0639}' => result.push('a'),
            '\u{063A}' => result.push_str("gh"),
            '\u{0641}' => result.push('f'),
            '\u{0642}' => result.push('q'),
            '\u{0643}' => result.push('k'),
            '\u{0644}' => result.push('l'),
            '\u{0645}' => result.push('m'),
            '\u{0646}' => result.push('n'),
            '\u{0647}' => result.push('h'),
            '\u{0648}' => result.push('w'),
            '\u{064A}' => result.push('y'),
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
