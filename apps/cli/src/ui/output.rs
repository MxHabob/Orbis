use colored::*;

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
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

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

    println!("{}", orbis_art.truecolor(0, 204, 255).bold());
    println!("  Enterprise Management Console");
    println!("  ========================================");
    println!("  Status: {}", status_text);
    println!("  Environment: {}", env_display);
    println!("  ========================================\n");
}

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
