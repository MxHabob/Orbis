use colored::*;
use crate::core::license;

// ── Refined Modern Color Palette (Claude / Antigravity Aesthetic) ────────
pub const COLOR_PURPLE: (u8, u8, u8) = (168, 85, 247);    // #A855F7 (Orbis Signature Dot)
pub const COLOR_WHITE: (u8, u8, u8) = (240, 240, 245);    // #F0F0F5 (Crisp Text & Arc)
pub const COLOR_CYAN: (u8, u8, u8) = (56, 189, 248);      // #38BDF8 (Accents)
pub const COLOR_EMERALD: (u8, u8, u8) = (52, 211, 153);   // #34D399 (Ready / Success)
pub const COLOR_AMBER: (u8, u8, u8) = (251, 191, 36);     // #FBBF24 (Warning / Pending)
pub const COLOR_ROSE: (u8, u8, u8) = (248, 113, 113);     // #F87171 (Error / Uninitialized)
pub const COLOR_MUTED: (u8, u8, u8) = (115, 115, 130);    // #737382 (Subtle text)
pub const COLOR_BORDER: (u8, u8, u8) = (60, 60, 75);      // #3C3C4B (Divider & Borders)

pub fn get_environment() -> (&'static str, colored::ColoredString) {
    let env_var = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    match env_var.to_lowercase().as_str() {
        "production" | "prod" => ("Production", "Production".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2).bold()),
        "staging" | "stage" => ("Staging", "Staging".truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2)),
        "testing" | "test" => ("Testing", "Testing".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2)),
        _ => ("Development", "Development".truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2)),
    }
}

/// Renders the authentic, minimal, sophisticated Orbis Header with genuine logo.png geometry
pub fn print_banner(db_exists: bool, setup_completed: bool) {
    // Clear screen cleanly
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

    let status_badge = if setup_completed {
        "● Ready".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2).bold()
    } else if db_exists {
        "◐ Pending Setup".truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2).bold()
    } else {
        "○ Uninitialized".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2).bold()
    };

    let (_, env_display) = get_environment();

    let license_badge = match license::check_license_status() {
        license::LicenseStatus::Active => {
            if let Ok(info) = license::load_license() {
                let days = info.days_remaining();
                if days <= 30 {
                    format!("✦ {} ({}d left)", info.plan, days).truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2)
                } else {
                    format!("✦ {}", info.plan).truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2).bold()
                }
            } else {
                "✦ Active".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2).bold()
            }
        }
        license::LicenseStatus::Expired => "✖ Expired".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2),
        license::LicenseStatus::Invalid => "✖ Invalid".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2),
        license::LicenseStatus::NotFound => "○ Community".truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2),
    };

    let p = |s: &str| s.truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2).bold();
    let w = |s: &str| s.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold();
    let m = |s: &str| s.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2);
    let dot = m("•");

    // 6-row vector rendering of Orbis logo (White orbit arc with detached purple dot)
    let logo_rows = [
        format!("    {}   {}", w("▄▄████▄▄"), p("▄▄▄")),
        format!("  {}  {}", w("▄██▀    ▀▀"), p("█████")),
        format!(" {}           {}", w("███"), p("▀▀▀")),
        format!(" {}           {}", w("███"), w("▄▄▄")),
        format!("  {}", w("▀██▄▄    ▄▄██▀▀")),
        format!("    {}", w("▀▀██████▀▀")),
    ];

    let meta_rows = [
        format!("{}  {}", "ORBIS".truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold(), m("v0.1.0")),
        format!("{}", m("Enterprise Educational Management Platform")),
        format!("{}", "──────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2)),
        format!("{}  {}  {}  {}  {}", status_badge, dot, license_badge, dot, env_display),
        format!("{} {}  {}  {} {}", m("Storage:"), "orbis.db".truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2), dot, m("Engine:"), "SeaORM Dual-Engine".truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)),
        String::new(),
    ];

    println!();
    for (logo, meta) in logo_rows.iter().zip(meta_rows.iter()) {
        println!("  {:<26}  {}", logo, meta);
    }
    println!();
}

// ── Minimal, Sophisticated Primitives ─────────────────────────────────

pub fn print_step(message: &str) {
    println!("  {} {}", "✦".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2).bold(), message.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());
}

pub fn print_success(message: &str) {
    println!("  {} {}", "✔".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2).bold(), message.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2));
}

pub fn print_warning(message: &str) {
    println!("  {} {}", "▲".truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2).bold(), message.truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2));
}

pub fn print_error(message: &str) {
    println!("  {} {}", "✖".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2).bold(), message.truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2));
}

pub fn print_info(message: &str) {
    println!("  {} {}", "ℹ".truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2), message.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2));
}

pub fn print_divider() {
    println!("  {}", "──────────────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
}

pub fn print_phase_header(phase: u8, total: u8, title: &str) {
    println!();
    println!("  {}", format!("╭─ Phase {}/{} ──────────────────────────────────────────────", phase, total).truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
    println!("  {}  {} {}", "│".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2), "✦".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2), title.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());
    println!("  {}", "╰─────────────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
    println!();
}

pub fn print_progress_ok(message: &str) {
    println!("    {} {}", "✔".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2), message);
}

pub fn print_progress_fail(message: &str) {
    println!("    {} {}", "✖".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2), message.truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2));
}

pub fn print_progress_pending(message: &str) {
    println!("    {} {}", "◐".truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2), message.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2));
}

pub fn print_summary_box(title: &str, items: &[(&str, &str)]) {
    println!();
    println!("  {}", format!("╭─ {} ───────────────────────────────────────", title).truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
    for (label, value) in items {
        println!("  {}  {:<20} {}", "│".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2), label.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2), value.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());
    }
    println!("  {}", "╰─────────────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
    println!();
}

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
            .template("  {spinner:.magenta} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(70));

    let result = f();

    pb.finish_and_clear();
    result
}
