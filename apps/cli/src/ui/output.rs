use colored::*;

use crate::core::license;

pub fn get_environment() -> (&'static str, colored::ColoredString) {
    let env_var = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    match env_var.to_lowercase().as_str() {
        "production" | "prod" => ("Production", "Production".green()),
        "staging" | "stage" => ("Staging", "Staging".yellow()),
        "testing" | "test" => ("Testing", "Testing".magenta()),
        _ => ("Development", "Development".cyan()),
    }
}

pub fn print_banner(db_exists: bool, setup_completed: bool) {
    // Clear the terminal to prevent scrolling bugs with the interactive menu
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

    let orbis_art = r#"
   ____  ____  ____  _________   ________    ____
  / __ \/ __ \/ __ )/  _/ ___/  / ____/ /   /  _/
 / / / / /_/ / __  |/ / \__ \  / /   / /    / /  
/ /_/ / _, _/ /_/ // / ___/ / / /___/ /____/ /   
\____/_/ |_/_____/___//____/  \____/_____/___/   
"#;

    let status_text = if setup_completed {
        "Fully Initialized".green()
    } else if db_exists {
        "Pending Onboarding".yellow()
    } else {
        "Not Initialized".red()
    };

    let (_, env_display) = get_environment();

    // License status
    let license_display = match license::check_license_status() {
        license::LicenseStatus::Active => {
            if let Ok(info) = license::load_license() {
                let days = info.days_remaining();
                if days <= 30 {
                    format!("{} ({} days left)", info.plan, days).yellow()
                } else {
                    format!("{}", info.plan).green()
                }
            } else {
                "Active".green()
            }
        }
        license::LicenseStatus::Expired => "Expired".red().bold(),
        license::LicenseStatus::Invalid => "Invalid".red(),
        license::LicenseStatus::NotFound => "No License".dimmed(),
    };

    println!("{}", orbis_art.truecolor(0, 204, 255).bold());
    println!("  Enterprise Management Console");
    println!("  ════════════════════════════════════════");
    println!("  Status:      {}", status_text);
    println!("  License:     {}", license_display);
    println!("  Environment: {}", env_display);
    println!("  ════════════════════════════════════════\n");
}

// ── Basic Output Primitives ──────────────────────────────────────────

pub fn print_step(message: &str) {
    println!("{} {}", "[*]".cyan(), message);
}

pub fn print_success(message: &str) {
    println!("{} {}", "[+]".green().bold(), message);
}

pub fn print_warning(message: &str) {
    println!("{} {}", "[!]".yellow(), message);
}

pub fn print_error(message: &str) {
    println!("{} {}", "[x]".red().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "[i]".blue(), message);
}

// ── Enhanced Output (Phase Headers, Summaries, Progress) ─────────────

/// Prints a numbered phase header for the init wizard.
pub fn print_phase_header(phase: u8, total: u8, title: &str) {
    let header = format!("  Phase {}/{} ▸ {}", phase, total, title);
    println!();
    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".truecolor(0, 204, 255)
    );
    println!("{}", header.truecolor(0, 204, 255).bold());
    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".truecolor(0, 204, 255)
    );
}

/// Prints a progress step with a success icon.
pub fn print_progress_ok(message: &str) {
    println!("  {} {}", "✓".green().bold(), message);
}

pub fn print_progress_fail(message: &str) {
    println!("  {} {}", "✗".red().bold(), message);
}

pub fn print_progress_pending(message: &str) {
    println!("  {} {}", "*".yellow(), message);
}

/// Prints a bordered summary box with key-value pairs.
pub fn print_summary_box(title: &str, items: &[(&str, &str)]) {
    let box_width = 55;
    let border: String = "─".repeat(box_width);

    println!();
    println!("  ┌{}┐", border);
    println!("  │{:^width$}│", title.bold(), width = box_width);
    println!("  ├{}┤", border);

    for (label, value) in items {
        let content = format!("  {:<20} {}", label, value);
        let padding = box_width.saturating_sub(content.len());
        println!("  │{}{}│", content, " ".repeat(padding));
    }

    println!("  └{}┘", border);
    println!();
}

/// Runs a closure with an animated indicatif spinner.
pub fn with_spinner<F, T>(message: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    use indicatif::{ProgressBar, ProgressStyle};
    use std::time::Duration;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));

    let result = f();

    pb.finish_and_clear();
    result
}

/// Prints a simple section divider.
pub fn print_divider() {
    println!(
        "{}",
        "───────────────────────────────────────────────────────────".truecolor(50, 50, 50)
    );
}
