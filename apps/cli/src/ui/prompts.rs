use anyhow::Result;
use inquire::{Password, Select, Text};

pub fn prompt_text(message: &str) -> Result<String> {
    Ok(Text::new(message).prompt()?)
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
