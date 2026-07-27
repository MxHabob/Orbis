use anyhow::Result;
use colored::*;

use crate::core::license;
use crate::ui::{output, prompts};

pub async fn execute() -> Result<()> {
    output::print_step("Managing Orbis License...");
    println!();

    match license::check_license_status() {
        license::LicenseStatus::Active => {
            let info = license::load_license()?;
            show_license_info(&info);

            let options = vec![
                "1. View License Details",
                "2. Activate New License Key",
                "3. Back",
            ];
            let choice = prompts::prompt_select("What would you like to do?", options)?;

            if choice.contains("View") {
                show_license_details(&info);
            } else if choice.contains("Activate New") {
                let confirm = prompts::prompt_confirm(
                    "This will replace your current license. Continue?",
                    false,
                )?;
                if confirm {
                    activate_new_license()?;
                }
            }
        }
        license::LicenseStatus::Expired => {
            output::print_warning("Your license has expired!");
            if let Ok(info) = license::load_license() {
                println!("  Plan: {}  |  Expired at: {}", info.plan, info.expires_at);
            }
            println!();

            let options = vec![
                "1. Enter New License Key",
                "2. Start Free Trial (30 days)",
                "3. Back",
            ];
            let choice = prompts::prompt_select("Select an action:", options)?;

            if choice.contains("New License") {
                activate_new_license()?;
            } else if choice.contains("Trial") {
                let info = license::activate_trial()?;
                output::print_success(&format!(
                    "Trial activated! Expires in {} days.",
                    info.days_remaining()
                ));
            }
        }
        license::LicenseStatus::NotFound | license::LicenseStatus::Invalid => {
            output::print_info("No active license found.");
            println!();

            let options = vec![
                "1. Enter License Key",
                "2. Start Free Trial (30 days)",
                "3. Back",
            ];
            let choice = prompts::prompt_select("Select an action:", options)?;

            if choice.contains("License Key") {
                activate_new_license()?;
            } else if choice.contains("Trial") {
                let info = license::activate_trial()?;
                output::print_success(&format!(
                    "Trial activated! Plan: {} | Expires in {} days.",
                    info.plan,
                    info.days_remaining()
                ));
            }
        }
    }

    Ok(())
}

fn show_license_info(info: &license::LicenseInfo) {
    let status_display = match info.status {
        license::LicenseStatus::Active => "Active".green().bold(),
        license::LicenseStatus::Expired => "Expired".red().bold(),
        _ => "Unknown".dimmed(),
    };

    output::print_summary_box("License Information", &[
        ("Status:", &status_display.to_string()),
        ("Plan:", &info.plan.to_string()),
        ("Days Remaining:", &info.days_remaining().to_string()),
    ]);
}

fn show_license_details(info: &license::LicenseInfo) {
    output::print_summary_box("License Details", &[
        ("Key:", &info.key),
        ("Plan:", &info.plan.to_string()),
        ("Status:", &info.status.to_string()),
        ("Activated:", &info.activated_at),
        ("Expires:", &info.expires_at),
        ("Max Users:", &info.max_users.to_string()),
        ("Max Students:", &info.max_students.to_string()),
        ("Features:", &info.features.join(", ")),
    ]);
}

fn activate_new_license() -> Result<()> {
    let key = prompts::prompt_license_key("Enter License Key (XXXX-XXXX-XXXX-XXXX):")?;
    match license::activate_license(&key) {
        Ok(info) => {
            output::print_success(&format!(
                "License activated! Plan: {} | Max Users: {} | Max Students: {}",
                info.plan, info.max_users, info.max_students
            ));
            show_license_info(&info);
            Ok(())
        }
        Err(e) => {
            output::print_error(&format!("Activation failed: {}", e));
            Err(e)
        }
    }
}
