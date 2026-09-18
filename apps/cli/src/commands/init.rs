use anyhow::{Result, anyhow};
use colored::*;
use sea_orm::Database;

use crate::core::config::CliConfig;
use crate::core::database;
use crate::core::github;
use crate::core::license;
use crate::ui::{output, prompts};

const TOTAL_PHASES: u8 = 6;

pub async fn execute() -> Result<()> {
    let status = database::get_full_status().await;

    if status.is_setup_complete() {
        output::print_success("The system is already fully configured and ready.");
        output::print_info(
            "Action: Run `orbis start` (or select 'Start System' from the menu) to boot.",
        );
        return Ok(());
    } else if status.is_initialized() {
        output::print_warning(
            "The core database exists but Initial Setup (Onboarding) is pending.",
        );
        output::print_info(
            "Action: Run `orbis start` to boot the application and complete the setup from the Desktop UI.",
        );
        return Ok(());
    }

    println!(
        "\n  Welcome to the {}. Let's get you set up.\n",
        "Orbis Setup Wizard".truecolor(output::COLOR_PURPLE.0, output::COLOR_PURPLE.1, output::COLOR_PURPLE.2).bold()
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
                info.plan,
                info.days_remaining()
            ));
            info
        }
        _ => {
            let activation_options = vec!["1. Enter License Key", "2. Start Free Trial (30 days)"];
            let choice =
                prompts::prompt_select("How would you like to activate?", activation_options)?;

            if choice.contains("Trial") {
                let info = license::activate_trial()?;
                output::print_success(&format!(
                    "Trial activated! Plan: {} | Expires: {} days",
                    info.plan,
                    info.days_remaining()
                ));
                info
            } else {
                loop {
                    let key =
                        prompts::prompt_license_key("Enter License Key (XXXX-XXXX-XXXX-XXXX):")?;
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

    let db_options = vec![
        "1. SQLite (Recommended for local/offline)",
        "2. PostgreSQL (Enterprise / Dedicated On-Premise Server)",
    ];
    let db_choice = prompts::prompt_select("Select database provider:", db_options)?;

    let config = CliConfig::default();
    let db_url = if db_choice.contains("SQLite") {
        output::print_progress_ok("SQLite selected — local storage");
        config.db_url.clone()
    } else {
        prompts::prompt_text(
            "Enter PostgreSQL Connection String (e.g. postgres://user:pass@localhost/orbis):",
        )?
    };

    output::print_progress_pending("Creating database and running schema migrations...");
    let db = Database::connect(&db_url)
        .await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;

    use orbis_infra::migration::Migrator;
    use sea_orm_migration::MigratorTrait;
    Migrator::up(&db, None)
        .await
        .map_err(|e| anyhow!("Failed running migrations: {}", e))?;

    output::print_progress_ok("Database schema created successfully");

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    let mut org_name;
    let mut org_name_ar;
    let mut org_slug;
    let mut blueprint;
    let mut institution_type_label;
    let mut timezone;
    let mut currency;
    let mut language;

    let mut admin_name;
    let mut admin_username;
    let mut admin_password;

    // Phase 3: Organization Setup
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(3, TOTAL_PHASES, "Organization Setup");

    org_name = prompts::prompt_org_name("Organization Name (EN):")?;
    org_name_ar = prompts::prompt_org_name("Organization Name (AR):")?;
    let generated_slug = transliterate_arabic_to_ascii(&org_name);
    org_slug = prompts::prompt_slug("Organization Slug:", &generated_slug)?;

    // Institution Type / Blueprint
    let blueprint_options = vec![
        "1. School (K-12 / Basic Education)",
        "2. University (Higher Education / Credit Hours)",
        "3. Training Center / Institute (Quarterly / Pass-Fail)",
    ];
    let bp_choice = prompts::prompt_select("Select institution type:", blueprint_options.clone())?;
    blueprint = if bp_choice.contains("University") {
        "university".to_string()
    } else if bp_choice.contains("Training") {
        "training_center".to_string()
    } else {
        "school".to_string()
    };

    institution_type_label = match blueprint.as_str() {
        "university" => "University",
        "training_center" => "Training Center",
        _ => "School",
    };

    // Auto-detect locale defaults with spinner
    let (detected_tz, detected_currency, detected_language) = output::with_spinner(
        "Detecting system region, timezone & default OS language...",
        || {
            let tz = detect_timezone();
            let curr = detect_currency(&tz);
            let lang = detect_language();
            (tz, curr, lang)
        },
    );
    output::print_progress_ok(&format!(
        "Auto-detected system region — Timezone: {} | Currency: {} | Language: {}",
        detected_tz, detected_currency, detected_language
    ));

    timezone = prompts::prompt_text_with_default("Timezone:", &detected_tz)?;
    let currency_input =
        prompts::prompt_text_with_default("Base Currency (ISO 4217):", &detected_currency)?;
    currency = currency_input.trim().to_uppercase();
    let language_input = prompts::prompt_text_with_default(
        "Default System Language (ar / en):",
        &detected_language,
    )?;
    language = language_input.trim().to_lowercase();

    // Phase 4: Administrator Account
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(4, TOTAL_PHASES, "Administrator Account");

    admin_name = prompts::prompt_full_name("Admin Full Name:")?;
    admin_username = prompts::prompt_email("Admin Email:")?;
    admin_password = prompts::prompt_password_confirmed("Admin Password:")?;

    // Review & Edit Loop
    loop {
        output::print_summary_box(
            "Configuration Review",
            &[
                ("Organization (EN):", &org_name),
                ("Organization (AR):", &org_name_ar),
                ("Slug:", &org_slug),
                ("Institution Type:", institution_type_label),
                ("Timezone:", &timezone),
                ("Base Currency:", &currency),
                ("Default Language:", &language),
                ("Admin Name:", &admin_name),
                ("Admin Email:", &admin_username),
            ],
        );

        let review_options = vec![
            "1. Confirm & Initialize System (Press Enter to finish)",
            "2. Edit Organization Setup (Name, Type, Timezone, Currency, Language)",
            "3. Edit Administrator Account (Name, Email, Password)",
            "4. Cancel Setup",
        ];

        let review_choice = prompts::prompt_select(
            "Would you like to proceed or edit any previous step?",
            review_options,
        )?;

        if review_choice.contains("Confirm") {
            break;
        } else if review_choice.contains("Cancel") {
            return Err(anyhow::anyhow!("Setup wizard cancelled by user."));
        } else if review_choice.contains("Edit Administrator Account") {
            output::print_phase_header(4, TOTAL_PHASES, "Administrator Account (Editing)");
            admin_name = prompts::prompt_text_with_default("Admin Full Name:", &admin_name)?;
            admin_username = prompts::prompt_text_with_default("Admin Email:", &admin_username)?;
            println!("  {}", "Re-enter admin password to confirm change:".cyan());
            admin_password = prompts::prompt_password_confirmed("Admin Password:")?;
        } else if review_choice.contains("Edit Organization Setup") {
            output::print_phase_header(3, TOTAL_PHASES, "Organization Setup (Editing)");
            org_name = prompts::prompt_text_with_default("Organization Name (EN):", &org_name)?;
            org_name_ar =
                prompts::prompt_text_with_default("Organization Name (AR):", &org_name_ar)?;
            let re_slug = transliterate_arabic_to_ascii(&org_name);
            org_slug = prompts::prompt_text_with_default("Organization Slug:", &re_slug)?;

            let bp_choice =
                prompts::prompt_select("Select institution type:", blueprint_options.clone())?;
            blueprint = if bp_choice.contains("University") {
                "university".to_string()
            } else if bp_choice.contains("Training") {
                "training_center".to_string()
            } else {
                "school".to_string()
            };
            institution_type_label = match blueprint.as_str() {
                "university" => "University",
                "training_center" => "Training Center",
                _ => "School",
            };

            timezone = prompts::prompt_text_with_default("Timezone:", &timezone)?;
            let c_in = prompts::prompt_text_with_default("Base Currency (ISO 4217):", &currency)?;
            currency = c_in.trim().to_uppercase();
            let l_in =
                prompts::prompt_text_with_default("Default System Language (ar / en):", &language)?;
            language = l_in.trim().to_lowercase();
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Phase 5: Execution
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    output::print_phase_header(5, TOTAL_PHASES, "Initializing System");

    // 5a. Bootstrap organization + admin + core data
    output::print_progress_pending(
        "Creating organization, admin account, chart of accounts, and cash/bank accounts...",
    );
    let system_repo = std::sync::Arc::new(orbis_infra::repositories::system_repo::SystemRepoImpl { db: db.clone() });
    let entitlement_provider: std::sync::Arc<dyn orbis_app::ports::entitlement::EntitlementProvider> = std::sync::Arc::new(
        orbis_infra::adapters::entitlement_adapter::UnlicensedEntitlementProvider,
    );
    let bootstrap_port: std::sync::Arc<dyn orbis_app::ports::bootstrap::BulkBootstrapPort> = std::sync::Arc::new(
        orbis_infra::adapters::bootstrap_adapter::BootstrapAdapterImpl { db: db.clone() },
    );
    let password_hasher = std::sync::Arc::new(orbis_infra::security::bcrypt::BcryptHasher);
    let token_service = std::sync::Arc::new(orbis_infra::security::jwt::JwtTokenService::new("cli-init-token-secret".to_string()));

    let bootstrap_service = orbis_app::services::BootstrapService::new(
        bootstrap_port,
        system_repo,
        password_hasher,
        token_service,
        entitlement_provider,
    );

    let request = orbis_app::services::BootstrapRequest {
        org_name: org_name.clone(),
        org_name_ar: Some(org_name_ar.clone()),
        org_type: blueprint,
        timezone: timezone.clone(),
        locale: language.clone(),
        currency: currency.clone(),
        admin_full_name: admin_name.clone(),
        admin_email: admin_username.clone(),
        admin_password: admin_password.clone(),
    };

    let result = bootstrap_service.bootstrap_organization(request).await
        .map_err(|e| anyhow!("Bootstrap failed: {e}"))?;
    let _org_id = result.org_id;
    output::print_progress_ok(
        "Organization, chart of accounts, and cash/bank accounts created successfully",
    );

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
            println!(
                "\n{}",
                "Orbis Desktop is distributed via GitHub Releases.".cyan()
            );
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

    let db_provider = if db_url.starts_with("sqlite") {
        "SQLite"
    } else {
        "PostgreSQL"
    };

    output::print_summary_box(
        "System Initialized Successfully!",
        &[
            ("Organization:", &org_name),
            ("Organization (AR):", &org_name_ar),
            ("Slug:", &org_slug),
            ("Admin:", &admin_username),
            ("Database:", db_provider),
            ("Institution:", institution_type_label),
            ("License:", &license_info.plan.to_string()),
            ("Timezone:", &timezone),
            ("Currency:", &currency),
        ],
    );

    println!("  {}", "Next Steps:".bold());
    println!(
        "  1. Run {} to boot the system",
        "orbis start".cyan().bold()
    );
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

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "[TimeZoneInfo]::Local.Id"])
            .output()
        {
            if output.status.success() {
                let win_tz = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let iana = match win_tz.as_str() {
                    "Arab Standard Time" => "Asia/Riyadh",
                    "Arabic Standard Time" => "Asia/Baghdad",
                    "Arabian Standard Time" => "Asia/Dubai",
                    "Jordan Standard Time" => "Asia/Amman",
                    "Egypt Standard Time" => "Africa/Cairo",
                    "Syria Standard Time" => "Asia/Damascus",
                    "E. Africa Standard Time" => "Asia/Aden",
                    "Sudan Standard Time" => "Africa/Khartoum",
                    "Turkey Standard Time" => "Europe/Istanbul",
                    "GMT Standard Time" => "Europe/London",
                    "Eastern Standard Time" => "America/New_York",
                    "Pacific Standard Time" => "America/Los_Angeles",
                    "Central Standard Time" => "America/Chicago",
                    _ => "",
                };
                if !iana.is_empty() {
                    return iana.to_string();
                }
            }
        }
    }

    "Asia/Riyadh".to_string()
}

fn detect_currency(timezone: &str) -> String {
    let tz_lower = timezone.to_lowercase();
    if tz_lower.contains("riyadh") || tz_lower.contains("saudi") {
        "SAR".to_string()
    } else if tz_lower.contains("aden") || tz_lower.contains("sanaa") || tz_lower.contains("yemen")
    {
        "YER".to_string()
    } else if tz_lower.contains("dubai")
        || tz_lower.contains("uae")
        || tz_lower.contains("abu_dhabi")
    {
        "AED".to_string()
    } else if tz_lower.contains("cairo") || tz_lower.contains("egypt") {
        "EGP".to_string()
    } else if tz_lower.contains("amman") || tz_lower.contains("jordan") {
        "JOD".to_string()
    } else if tz_lower.contains("kuwait") {
        "KWD".to_string()
    } else if tz_lower.contains("doha") || tz_lower.contains("qatar") {
        "QAR".to_string()
    } else if tz_lower.contains("bahrain") {
        "BHD".to_string()
    } else if tz_lower.contains("muscat") || tz_lower.contains("oman") {
        "OMR".to_string()
    } else if tz_lower.contains("baghdad") || tz_lower.contains("iraq") {
        "IQD".to_string()
    } else if tz_lower.contains("istanbul") || tz_lower.contains("turkey") {
        "TRY".to_string()
    } else if tz_lower.contains("london") || tz_lower.contains("uk") {
        "GBP".to_string()
    } else if tz_lower.contains("europe") {
        "EUR".to_string()
    } else {
        "USD".to_string()
    }
}

fn detect_language() -> String {
    if let Ok(lang) = std::env::var("LANG").or_else(|_| std::env::var("LC_ALL")) {
        let l = lang.to_lowercase();
        if l.starts_with("ar") {
            return "ar".to_string();
        } else if l.starts_with("en") {
            return "en".to_string();
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "(Get-UICulture).Name"])
            .output()
        {
            if output.status.success() {
                let win_lang = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_lowercase();
                if win_lang.starts_with("ar") {
                    return "ar".to_string();
                } else if win_lang.starts_with("en") {
                    return "en".to_string();
                }
            }
        }
    }

    "ar".to_string()
}

fn transliterate_arabic_to_ascii(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '\u{0627}' | '\u{0623}' | '\u{0625}' | '\u{0622}' | '\u{0649}' | '\u{0621}'
            | '\u{0626}' | '\u{0624}' => result.push('a'),
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
