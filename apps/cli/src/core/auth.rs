use anyhow::{Result, anyhow};
use colored::*;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use orbis_app::ports::security::TotpService;
use orbis_infra::security::totp::TotpServiceImpl;

use crate::ui::output::{self, COLOR_BORDER, COLOR_CYAN, COLOR_EMERALD, COLOR_MUTED, COLOR_PURPLE, COLOR_ROSE, COLOR_WHITE};

#[derive(Debug, Clone)]
pub struct AuthenticatedAdmin {
    pub user_id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
}

struct AdminRecord {
    id: String,
    email: String,
    full_name: String,
    password_hash: Option<String>,
    mfa_enabled: bool,
    role_id: String,
}

/// Enforces authentication for the CLI session when the system is initialized.
pub async fn authenticate_admin(db: &DatabaseConnection) -> Result<AuthenticatedAdmin> {
    let backend = db.get_database_backend();

    // 1. Fetch all active administrators
    let sql = r#"
        SELECT u.id, u.email, u.full_name, u.password_hash, u.mfa_enabled, ura.role_id
        FROM users u
        JOIN user_role_assignments ura ON u.id = ura.user_id
        WHERE ura.role_id IN ('super_admin', 'admin')
          AND u.status = 'active'
          AND u.is_deleted = 0
        ORDER BY ura.role_id ASC, u.created_at ASC;
    "#;

    let rows = db
        .query_all(Statement::from_sql_and_values(backend, sql, vec![]))
        .await
        .map_err(|e| anyhow!("Failed to query administrator accounts: {e}"))?;

    if rows.is_empty() {
        return Err(anyhow!("No active administrator accounts found in the database."));
    }

    let admins: Vec<AdminRecord> = rows
        .into_iter()
        .filter_map(|row| {
            let id: String = row.try_get("", "id").ok()?;
            let email: String = row.try_get("", "email").ok().flatten().unwrap_or_default();
            let full_name: String = row.try_get("", "full_name").ok()?;
            let password_hash: Option<String> = row.try_get("", "password_hash").ok().flatten();
            let mfa_enabled: bool = row.try_get("", "mfa_enabled").unwrap_or(false);
            let role_id: String = row.try_get("", "role_id").unwrap_or_else(|_| "admin".to_string());
            Some(AdminRecord {
                id,
                email,
                full_name,
                password_hash,
                mfa_enabled,
                role_id,
            })
        })
        .collect();

    if admins.is_empty() {
        return Err(anyhow!("No valid administrator credentials found."));
    }

    let border_color = |s: &str| s.truecolor(COLOR_BORDER.0, COLOR_BORDER.1, COLOR_BORDER.2);

    println!();
    println!("  {}", border_color("╭─ Security Verification ─────────────────────────────────────"));
    println!("  {}  {} {}", border_color("│"), "✦".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2), "Administrator authentication required to continue.".truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2));
    println!("  {}", border_color("╰─────────────────────────────────────────────────────────────"));
    println!();

    // 2. Select admin account if multiple, or select the single admin
    let selected_admin = if admins.len() == 1 {
        let admin = &admins[0];
        println!("  {} Account: {} ({})", "●".truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2), admin.full_name.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold(), admin.email.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2));
        admin
    } else {
        let options: Vec<String> = admins
            .iter()
            .map(|a| format!("{} ({}) [{}]", a.full_name, a.email, a.role_id))
            .collect();
        let selected_label = crate::ui::prompts::prompt_select(
            "Select administrator account:",
            options.iter().map(|s| s.as_str()).collect(),
        )?;
        admins
            .iter()
            .find(|a| selected_label.contains(&a.email))
            .ok_or_else(|| anyhow!("Invalid account selection."))?
    };

    let password_hash = selected_admin.password_hash.as_deref().ok_or_else(|| {
        anyhow!("Selected administrator has no password set. Please use password recovery.")
    })?;

    // 3. Password Verification Loop (up to 3 attempts)
    let max_attempts = 3;
    let mut authenticated = false;

    for attempt in 1..=max_attempts {
        let prompt_msg = if attempt == 1 {
            "Administrator Password:"
        } else {
            "Re-enter Password:"
        };

        let password = crate::ui::prompts::prompt_password(prompt_msg, None)?;

        if bcrypt::verify(&password, password_hash).unwrap_or(false) {
            authenticated = true;
            break;
        } else {
            let remaining = max_attempts - attempt;
            if remaining > 0 {
                println!(
                    "  {} Invalid password. ({} attempt(s) remaining)",
                    "▲".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2),
                    remaining
                );
            }
        }
    }

    if !authenticated {
        output::print_error("Authentication failed: Too many incorrect password attempts.");
        return Err(anyhow!("Access denied: Invalid administrator password."));
    }

    // 4. Two-Factor Authentication (MFA / TOTP) Verification
    let mfa_sql = r#"
        SELECT id, secret_encrypted, is_verified
        FROM user_mfa_factors
        WHERE user_id = $1 AND factor_type = 'totp' AND is_verified = 1
        LIMIT 1;
    "#;

    let mfa_row = db
        .query_one(Statement::from_sql_and_values(
            backend,
            mfa_sql,
            vec![selected_admin.id.clone().into()],
        ))
        .await
        .ok()
        .flatten();

    let requires_mfa = selected_admin.mfa_enabled || mfa_row.is_some();

    if requires_mfa {
        println!();
        println!("  {}", border_color("╭─ Two-Factor Authentication (MFA) ───────────────────────────"));
        println!("  {}  {} {}", border_color("│"), "✦".truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2), "Enter 6-digit TOTP code or an 8-character recovery code.".truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2));
        println!("  {}", border_color("╰─────────────────────────────────────────────────────────────"));
        println!();

        let totp_service = TotpServiceImpl;
        let secret = mfa_row.as_ref().and_then(|r| r.try_get::<String>("", "secret_encrypted").ok());

        let mut mfa_verified = false;

        for attempt in 1..=max_attempts {
            let code_input = crate::ui::prompts::prompt_text("Authenticator Code / Recovery Code:")?;
            let trimmed = code_input.trim();

            // Try TOTP code first
            if let Some(ref sec) = secret {
                if totp_service.verify_code(sec, trimmed) {
                    mfa_verified = true;
                    // Update factor last_used_at
                    if let Some(ref r) = mfa_row {
                        if let Ok(factor_id) = r.try_get::<String>("", "id") {
                            let now = chrono::Utc::now().to_rfc3339();
                            let _ = db.execute(Statement::from_sql_and_values(
                                backend,
                                "UPDATE user_mfa_factors SET last_used_at = $1 WHERE id = $2",
                                vec![now.into(), factor_id.into()],
                            )).await;
                        }
                    }
                    break;
                }
            }

            // If not verified by TOTP, check if it's an unused recovery code
            let rec_sql = r#"
                SELECT id, code_hash
                FROM user_recovery_codes
                WHERE user_id = $1 AND is_used = 0;
            "#;

            if let Ok(rec_rows) = db.query_all(Statement::from_sql_and_values(
                backend,
                rec_sql,
                vec![selected_admin.id.clone().into()],
            )).await {
                for rec_row in rec_rows {
                    if let (Ok(rec_id), Ok(code_hash)) = (
                        rec_row.try_get::<String>("", "id"),
                        rec_row.try_get::<String>("", "code_hash"),
                    ) {
                        if bcrypt::verify(trimmed, &code_hash).unwrap_or(false) {
                            let now = chrono::Utc::now().to_rfc3339();
                            let _ = db.execute(Statement::from_sql_and_values(
                                backend,
                                "UPDATE user_recovery_codes SET is_used = 1, used_at = $1 WHERE id = $2",
                                vec![now.into(), rec_id.into()],
                            )).await;
                            println!("    {} Recovery code verified and consumed.", "✔".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2));
                            mfa_verified = true;
                            break;
                        }
                    }
                }
            }

            if mfa_verified {
                break;
            }

            let remaining = max_attempts - attempt;
            if remaining > 0 {
                println!(
                    "  {} Invalid code. ({} attempt(s) remaining)",
                    "▲".truecolor(COLOR_ROSE.0, COLOR_ROSE.1, COLOR_ROSE.2),
                    remaining
                );
            }
        }

        if !mfa_verified {
            output::print_error("Two-Factor Authentication failed: Too many invalid attempts.");
            return Err(anyhow!("Access denied: Invalid MFA authenticator code."));
        }
    }

    output::print_success(&format!(
        "Authenticated as {} ({})",
        selected_admin.full_name, selected_admin.email
    ));

    Ok(AuthenticatedAdmin {
        user_id: selected_admin.id.clone(),
        email: selected_admin.email.clone(),
        full_name: selected_admin.full_name.clone(),
        role: selected_admin.role_id.clone(),
    })
}
