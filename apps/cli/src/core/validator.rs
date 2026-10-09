use anyhow::{Result, anyhow};
use regex::Regex;

/// Validates that an organization name is not empty and within reasonable length.
pub fn validate_org_name(name: &str) -> Result<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Organization name cannot be empty."));
    }
    if trimmed.len() < 2 {
        return Err(anyhow!("Organization name must be at least 2 characters."));
    }
    if trimmed.len() > 100 {
        return Err(anyhow!("Organization name must be 100 characters or less."));
    }
    Ok(())
}

/// Validates a slug: lowercase alphanumeric + hyphens, 3-50 chars.
pub fn validate_slug(slug: &str) -> Result<()> {
    let trimmed = slug.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Slug cannot be empty."));
    }
    if trimmed.len() < 3 {
        return Err(anyhow!("Slug must be at least 3 characters."));
    }
    if trimmed.len() > 50 {
        return Err(anyhow!("Slug must be 50 characters or less."));
    }
    let re = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    if !re.is_match(trimmed) {
        return Err(anyhow!(
            "Slug must contain only lowercase letters, numbers, and hyphens (e.g. my-school)."
        ));
    }
    Ok(())
}

/// Validates an email address format.
pub fn validate_email(email: &str) -> Result<()> {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Email cannot be empty."));
    }
    let re = Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap();
    if !re.is_match(trimmed) {
        return Err(anyhow!("Invalid email format. Example: admin@school.com"));
    }
    Ok(())
}

/// Validates a full name: at least 2 characters, not purely numbers.
pub fn validate_full_name(name: &str) -> Result<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Full name cannot be empty."));
    }
    if trimmed.len() < 2 {
        return Err(anyhow!("Full name must be at least 2 characters."));
    }
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Err(anyhow!("Full name cannot be purely numeric."));
    }
    Ok(())
}

/// Validates password strength: minimum 8 chars, at least 1 uppercase, 1 lowercase, 1 digit.
pub fn validate_password(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(anyhow!("Password must be at least 8 characters."));
    }
    if password.len() > 128 {
        return Err(anyhow!("Password must be 128 characters or less."));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(anyhow!(
            "Password must contain at least one uppercase letter."
        ));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(anyhow!(
            "Password must contain at least one lowercase letter."
        ));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(anyhow!("Password must contain at least one digit."));
    }
    Ok(())
}

/// Returns a human-readable password strength label.
pub fn password_strength(password: &str) -> &'static str {
    let len = password.len();
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    let mut score = 0;
    if len >= 8 {
        score += 1;
    }
    if len >= 12 {
        score += 1;
    }
    if len >= 16 {
        score += 1;
    }
    if has_upper {
        score += 1;
    }
    if has_lower {
        score += 1;
    }
    if has_digit {
        score += 1;
    }
    if has_special {
        score += 1;
    }

    match score {
        0..=2 => "Weak",
        3..=4 => "Fair",
        5..=6 => "Strong",
        _ => "Very Strong",
    }
}

/// Validates a non-empty text input with custom label.
pub fn validate_non_empty(value: &str, field_label: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{} cannot be empty.", field_label));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("admin@school.com").is_ok());
        assert!(validate_email("user.name@domain.co").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("").is_err());
        assert!(validate_email("@domain.com").is_err());
    }

    #[test]
    fn test_validate_slug() {
        assert!(validate_slug("my-school").is_ok());
        assert!(validate_slug("school123").is_ok());
        assert!(validate_slug("ab").is_err());
        assert!(validate_slug("My-School").is_err());
        assert!(validate_slug("my--school").is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("Abc12345").is_ok());
        assert!(validate_password("short").is_err());
        assert!(validate_password("alllowercase1").is_err());
        assert!(validate_password("ALLUPPERCASE1").is_err());
        assert!(validate_password("NoDigitsHere").is_err());
    }

    #[test]
    fn test_password_strength() {
        assert_eq!(password_strength("ab"), "Weak");
        assert_eq!(password_strength("Abcdef12"), "Fair");
        assert_eq!(password_strength("Abcdef12!"), "Strong");
        assert_eq!(password_strength("MyStr0ng!Pa$$w0rd"), "Very Strong");
    }
}
