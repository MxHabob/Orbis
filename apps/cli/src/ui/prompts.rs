use anyhow::Result;
use inquire::{Password, PasswordDisplayMode, Select, Text, Confirm};
use colored::*;

use crate::core::validator;

// ── Basic Prompts ─────────────────────────────────────────────────────

pub fn prompt_text(message: &str) -> Result<String> {
    Ok(Text::new(message).prompt()?)
}

pub fn prompt_text_with_default(message: &str, default: &str) -> Result<String> {
    Ok(Text::new(message).with_default(default).prompt()?)
}

pub fn prompt_password(message: &str, help: Option<&str>) -> Result<String> {
    let mut prompt = Password::new(message).without_confirmation();
    if let Some(h) = help {
        prompt = prompt.with_help_message(h);
    }
    Ok(prompt.prompt()?)
}

pub fn prompt_select<'a>(message: &str, options: Vec<&'a str>) -> Result<&'a str> {
    Ok(Select::new(message, options)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .prompt()?)
}

pub fn prompt_confirm(message: &str, default: bool) -> Result<bool> {
    Ok(Confirm::new(message).with_default(default).prompt()?)
}

// ── Validated Prompts ─────────────────────────────────────────────────

/// Prompts for text input, retrying until the validator passes.
pub fn prompt_text_validated<F>(message: &str, validate: F) -> Result<String>
where
    F: Fn(&str) -> Result<()>,
{
    loop {
        let value = Text::new(message).prompt()?;
        match validate(&value) {
            Ok(()) => return Ok(value),
            Err(e) => {
                println!("{} {}", "[!]".yellow(), e);
            }
        }
    }
}

/// Prompts for text input with a default value, retrying until the validator passes.
pub fn prompt_text_validated_with_default<F>(message: &str, default: &str, validate: F) -> Result<String>
where
    F: Fn(&str) -> Result<()>,
{
    loop {
        let value = Text::new(message).with_default(default).prompt()?;
        match validate(&value) {
            Ok(()) => return Ok(value),
            Err(e) => {
                println!("{} {}", "[!]".yellow(), e);
            }
        }
    }
}

/// Prompts for an organization name with validation.
pub fn prompt_org_name(message: &str) -> Result<String> {
    prompt_text_validated(message, |v| validator::validate_org_name(v))
}

/// Prompts for a slug with validation.
pub fn prompt_slug(message: &str, default: &str) -> Result<String> {
    prompt_text_validated_with_default(message, default, |v| validator::validate_slug(v))
}

/// Prompts for an email address with validation.
pub fn prompt_email(message: &str) -> Result<String> {
    prompt_text_validated(message, |v| validator::validate_email(v))
}

/// Prompts for a full name with validation.
pub fn prompt_full_name(message: &str) -> Result<String> {
    prompt_text_validated(message, |v| validator::validate_full_name(v))
}

/// Prompts for a password with confirmation and strength indicator.
pub fn prompt_password_confirmed(message: &str) -> Result<String> {
    loop {
        let password = Password::new(message)
            .with_display_mode(PasswordDisplayMode::Masked)
            .with_help_message("Min 8 chars, 1 uppercase, 1 lowercase, 1 digit")
            .without_confirmation()
            .prompt()?;

        // Validate strength
        if let Err(e) = validator::validate_password(&password) {
            println!("{} {}", "[!]".yellow(), e);
            continue;
        }

        let strength = validator::password_strength(&password);
        let strength_display = match strength {
            "Weak" => strength.red(),
            "Fair" => strength.yellow(),
            "Strong" => strength.green(),
            "Very Strong" => strength.green().bold(),
            _ => strength.normal(),
        };
        println!("{} Password Strength: {}", "[i]".blue(), strength_display);

        // Confirmation
        let confirm = Password::new("Confirm password:")
            .with_display_mode(PasswordDisplayMode::Masked)
            .without_confirmation()
            .prompt()?;

        if password != confirm {
            println!("{} Passwords do not match. Please try again.", "[!]".yellow());
            continue;
        }

        return Ok(password);
    }
}

/// Prompts for a license key with format validation.
pub fn prompt_license_key(message: &str) -> Result<String> {
    prompt_text_validated(message, |v| validator::validate_license_key(v))
}
