use anyhow::{Result, anyhow};
use colored::*;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use orbis_app::ports::security::TotpService;
use orbis_infra::security::totp::TotpServiceImpl;
use uuid::Uuid;

use crate::ui::output::{self, COLOR_CYAN, COLOR_EMERALD, COLOR_MUTED, COLOR_PURPLE, COLOR_ROSE, COLOR_WHITE};

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

        // 1. Fetch only active users with an administrator role
    let sql = r#"
                  SELECT u.id, u.email,
                      COALESCE(NULLIF(p.display_name, ''), TRIM(p.first_name || ' ' || p.last_name)) AS full_name,
                      u.password_hash, u.is_mfa_enabled, ura.role_id
                FROM users u
                  INNER JOIN persons p ON p.id = u.person_id
                INNER JOIN user_role_assignments ura ON ura.user_id = u.id
                WHERE u.status = 'active'
                    AND u.is_deleted = 0
                    AND ura.role_id IN ('super_admin', 'admin')
                ORDER BY u.created_at ASC;
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
            let id: String = if let Ok(u) = row.try_get::<uuid::Uuid>("", "id") {
                u.to_string()
            } else if let Ok(s) = row.try_get::<String>("", "id") {
                s
            } else {
                return None;
            };
            let email: String = row.try_get("", "email").ok().flatten().unwrap_or_default();
            let full_name: String = row.try_get("", "full_name").ok()?;
            let password_hash: Option<String> = row.try_get("", "password_hash").ok().flatten();
            let mfa_enabled: bool = row.try_get("", "mfa_enabled").unwrap_or(false);
            let role_id: String = row.try_get("", "role_id").ok()?;
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

    println!("  {} {}", "●".truecolor(COLOR_PURPLE.0, COLOR_PURPLE.1, COLOR_PURPLE.2), "Administrator authentication required to continue.".truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());

    // 2. Select admin account if multiple, or select the single admin
    let selected_admin = if admins.len() == 1 {
        let admin = &admins[0];
        println!("    {} {} ({})", "Account:".truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2), admin.full_name.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold(), admin.email.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2));
        println!();
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
    let selected_admin_id = Uuid::parse_str(&selected_admin.id)
        .map_err(|_| anyhow!("Administrator ID is not a valid UUID."))?;

    // 3. Password Verification Loop (up to 3 attempts)
    let max_attempts = 3;
    let mut authenticated = false;

    for attempt in 1..=max_attempts {
        let prompt_msg = if attempt == 1 {
            "Admin Password:"
        } else {
            "Re-enter Admin Password:"
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
    "#;

    let mfa_rows = db
        .query_all(Statement::from_sql_and_values(
            backend,
            mfa_sql,
            vec![selected_admin_id.into()],
        ))
        .await
        .unwrap_or_default();

    let requires_mfa = selected_admin.mfa_enabled || !mfa_rows.is_empty();

    if requires_mfa {
        println!("  {} {}", "●".truecolor(COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2), "Two-Factor Verification (TOTP / Recovery Code)".truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold());

        let totp_service = TotpServiceImpl;
        let mut mfa_verified = false;

        for attempt in 1..=max_attempts {
            let code_input = crate::ui::prompts::prompt_text("Authenticator Code / Recovery Code:")?;
            let trimmed = code_input.trim();

            // Try every verified TOTP factor; users may have re-enrolled MFA.
            for mfa_row in &mfa_rows {
                if let Ok(secret) = mfa_row.try_get::<String>("", "secret_encrypted") {
                    if totp_service.verify_code(&secret, trimmed) {
                        mfa_verified = true;
                        if let Ok(factor_id) = mfa_row.try_get::<Uuid>("", "id") {
                            let now = chrono::Utc::now().to_rfc3339();
                            let _ = db.execute(Statement::from_sql_and_values(
                                backend,
                                "UPDATE user_mfa_factors SET last_used_at = $1 WHERE id = $2",
                                vec![now.into(), factor_id.into()],
                            )).await;
                        }
                        break;
                    }
                }
            }

            if mfa_verified {
                break;
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
                vec![selected_admin_id.into()],
            )).await {
                for rec_row in rec_rows {
                    if let (Ok(rec_id), Ok(code_hash)) = (
                        rec_row.try_get::<Uuid>("", "id"),
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

    println!("  {} {} ({})\n", "✔ Verified:".truecolor(COLOR_EMERALD.0, COLOR_EMERALD.1, COLOR_EMERALD.2).bold(), selected_admin.full_name.truecolor(COLOR_WHITE.0, COLOR_WHITE.1, COLOR_WHITE.2).bold(), selected_admin.email.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2));

    Ok(AuthenticatedAdmin {
        user_id: selected_admin.id.clone(),
        email: selected_admin.email.clone(),
        full_name: selected_admin.full_name.clone(),
        role: selected_admin.role_id.clone(),
    })
}
