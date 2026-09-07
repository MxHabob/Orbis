use colored::*;
use crate::core::license;

// ── Antigravity / Gemini CLI Color Tokens ───────────────────────────────
pub const COLOR_CYAN: (u8, u8, u8) = (56, 189, 248);       // Sky Cyan (#38BDF8)
pub const COLOR_INDIGO: (u8, u8, u8) = (99, 102, 241);     // Indigo (#6366F1)
pub const COLOR_PURPLE: (u8, u8, u8) = (168, 85, 247);     // Purple (#A855F7)
pub const COLOR_PINK: (u8, u8, u8) = (236, 72, 153);       // Pink (#EC4899)
pub const COLOR_DOT: (u8, u8, u8) = (216, 180, 254);       // Light Purple Dot (#D8B4FE)
pub const COLOR_EMERALD: (u8, u8, u8) = (52, 211, 153);    // Green (#34D399)
pub const COLOR_AMBER: (u8, u8, u8) = (251, 191, 36);      // Amber (#FBBF24)
pub const COLOR_ROSE: (u8, u8, u8) = (248, 113, 113);      // Red (#F87171)
pub const COLOR_WHITE: (u8, u8, u8) = (245, 245, 250);     // Crisp White (#F5F5FA)
pub const COLOR_MUTED: (u8, u8, u8) = (115, 115, 130);     // Muted Gray (#737382)
pub const COLOR_BORDER: (u8, u8, u8) = (60, 60, 75);       // Border (#3C3C4B)

pub fn get_environment() -> (&'static str, colored::ColoredString) {
    let env_var = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    match env_var.to_lowercase().as_str() {
        "production" | "prod" => ("Production", "production".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2)),
        "staging" | "stage" => ("Staging", "staging".truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2)),
        "testing" | "test" => ("Testing", "testing".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2)),
        _ => ("Development", "development".truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2)),
    }
}

/// Renders the authentic Antigravity / Gemini CLI Header Layout
pub fn print_banner(db_exists: bool, setup_completed: bool) {
    // Clear screen cleanly
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

    let status_text = if setup_completed {
        "● ready".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2)
    } else if db_exists {
        "◐ pending setup".truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2)
    } else {
        "○ uninitialized".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2)
    };

    let (_, _env_display) = get_environment();

    let license_text = match license::check_license_status() {
        license::LicenseStatus::Active => {
            if let Ok(info) = license::load_license() {
                let days = info.days_remaining();
                if days <= 30 {
                    format!("✦ {} ({}d left)", info.plan, days).truecolor(COLOR_AMBER.0, COLOR_AMBER.1, COLOR_AMBER.2)
                } else {
                    format!("✦ {}", info.plan).truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2)
                }
            } else {
                "✦ active".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2)
            }
        }
        license::LicenseStatus::Expired => "✖ expired".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2),
        license::LicenseStatus::Invalid => "✖ invalid".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2),
        license::LicenseStatus::NotFound => "community".truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2),
    };

    let c1 = |s: &str| s.truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2).bold();
    let c2 = |s: &str| s.truecolor(COLOR_INDIGO.0, COLOR_INDIGO.1, COLOR_INDIGO.2).bold();
    let c3 = |s: &str| s.truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2).bold();
    let dot = |s: &str| s.truecolor(COLOR_DOT.0, COLOR_DOT.1, COLOR_DOT.2).bold();
    let dot_core = |s: &str| s.truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2).bold();
    let w = |s: &str| s.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2);
    let m = |s: &str| s.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2);

    // Pixel-art Orbis Orbit Logo (8 rows)
    let logo_rows = [
        format!("      {}        ", c1("▄▄██████▄▄")),
        format!("    {}    {}", format!("{}{}", c1("████▀▀  ▀▀"), c2("████")), dot("▄██▄")),
        format!("   {}        {}   {}", c1("████"), c2("▀▀▀"), dot_core("██████")),
        format!("  {}                {}", c1("████"), dot("▀██▀")),
        format!("  {}                    ", c1("████")),
        format!("  {}                {}", c1("████"), c3("████")),
        format!("   {}          {} ", c1("████▄▄"), c3("▄▄████")),
        format!("     {}   ", format!("{}{}", c1("▀▀██████████████"), c3("▀▀"))),
    ];

    let meta_rows = [
        format!("{}  {}  {}", w("Orbis CLI 1.0.0").bold(), m("•"), "by Mulverse".truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2).bold()),
        format!("{}  {}  {}", m("Enterprise Educational Platform"), m("•"), "https://www.mulverse.com".truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)),
        format!("{}", m("SeaORM Dual-Engine • Embedded & Remote IPC/REST")),
        format!("{}", "─────────────────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2)),
        format!("{}", m("Tips for getting started:")),
        format!("  1. Run {} to configure organization and administrator identity.", c3("init")),
        format!("  2. Run {} to check system requirements and database connectivity.", c1("doctor")),
        format!("  3. Run {} to launch the desktop application or daemon.", c1("start")),
    ];

    println!();
    for (logo, meta) in logo_rows.iter().zip(meta_rows.iter()) {
        println!("  {:<32}  {}", logo, meta);
    }
    println!();

    // Bottom status bar (Gemini CLI Style)
    println!("  {} ({})    {}    {}    {}",
        m("~/projects/Orbis"),
        "orbis.db".truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2),
        status_text,
        license_text,
        "https://www.mulverse.com".truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!("  {}\n", "────────────────────────────────────────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
}

// ── Antigravity / Gemini Output Primitives ─────────────────────────────

pub fn print_step(message: &str) {
    println!("  {} {}", "▸".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2).bold(), message.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());
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
    println!("  {}", "────────────────────────────────────────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
}

pub fn print_phase_header(phase: u8, total: u8, title: &str) {
    println!();
    println!("  {} {}", format!("[ Phase {}/{} ]", phase, total).truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2).bold(), title.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());
    println!("  {}", "────────────────────────────────────────────────────────────────────────".truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2));
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

/// Renders a Gemini-style action card
pub fn print_summary_box(title: &str, items: &[(&str, &str)]) {
    let border = |s: &str| s.truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2);
    let box_width: usize = 68;

    println!();
    println!("  {}", border(&format!("╭─ {} {}", title, "─".repeat(box_width.saturating_sub(title.len() + 4)) + "╮")));
    for (label, value) in items {
        let content = format!("  {:<22} {}", label.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2), value.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());
        let raw_len = 2 + 22 + 1 + value.len();
        let padding = box_width.saturating_sub(raw_len);
        println!("  {} {}{} {}", border("│"), content, " ".repeat(padding), border("│"));
    }
    println!("  {}", border(&format!("╰{}╯", "─".repeat(box_width))));
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
