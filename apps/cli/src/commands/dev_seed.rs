use anyhow::{Result, anyhow};
use bcrypt::{DEFAULT_COST, hash};
use chrono::Utc;
use inquire::Text;
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, Statement, TransactionTrait};
use uuid::Uuid;

use crate::core::config::CliConfig;
use crate::ui::output;

const DEFAULT_TEACHER_COUNT: u32 = 5;
const MAX_TEACHER_COUNT: u32 = 500;
const DEV_PASSWORD: &str = "Teacher123!";

pub async fn execute() -> Result<()> {
    if !is_development() {
        return Err(anyhow!(
            "Default data is available only in development mode."
        ));
    }

    let count = prompt_teacher_count()?;
    let config = CliConfig::default();
    let db = Database::connect(&config.db_url).await?;
    let txn = db.begin().await?;

    let backend = txn.get_database_backend();
    let org_id = find_organization(&txn, backend)
        .await?
        .ok_or_else(|| anyhow!("No organization found. Run init first."))?;
    let password_hash = hash(DEV_PASSWORD, DEFAULT_COST)?;
    let now = Utc::now().to_rfc3339();
    let mut created = 0u32;
    let mut skipped = 0u32;

    for index in 1..=count {
        let email = format!("dev.teacher.{index}@example.test");
        if teacher_exists(&txn, backend, &org_id, &email).await? {
            skipped += 1;
            continue;
        }

        let person_id = Uuid::new_v4().to_string();
        let user_id = Uuid::new_v4().to_string();
        let assignment_id = Uuid::new_v4().to_string();
        let first_name = format!("Dev Teacher {index}");

        execute_with_values(
            &txn,
            backend,
            "INSERT INTO persons (id, org_id, first_name, last_name, full_name, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            vec![
                person_id.clone().into(), org_id.clone().into(),
                first_name.clone().into(), "".into(), first_name.clone().into(),
                now.clone().into(), now.clone().into(),
            ],
        ).await?;
        execute_with_values(
            &txn,
            backend,
            "INSERT INTO users (id, person_id, org_id, email, email_verified, password_hash, full_name, status, is_deleted, created_at, updated_at) VALUES (?, ?, ?, ?, 1, ?, ?, 'active', 0, ?, ?)",
            vec![
                user_id.clone().into(), person_id.into(), org_id.clone().into(), email.into(),
                password_hash.clone().into(), first_name.into(),
                now.clone().into(), now.clone().into(),
            ],
        ).await?;
        execute_with_values(
            &txn,
            backend,
            "INSERT INTO user_role_assignments (id, user_id, role_id, org_id, scope_type, scope_id, assigned_by, created_at) VALUES (?, ?, 'teacher', ?, 'global', NULL, NULL, ?)",
            vec![
                assignment_id.into(), user_id.into(), org_id.clone().into(), now.clone().into(),
            ],
        ).await?;
        created += 1;
    }

    txn.commit().await?;
    output::print_success(&format!("Created {created} development teacher(s)."));
    if skipped > 0 {
        output::print_info(&format!(
            "Skipped {skipped} existing development teacher(s)."
        ));
    }
    output::print_info(&format!(
        "Test password for generated teachers: {DEV_PASSWORD}"
    ));
    Ok(())
}

fn is_development() -> bool {
    matches!(
        std::env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str(),
        "development" | "dev"
    )
}

fn prompt_teacher_count() -> Result<u32> {
    loop {
        let raw = Text::new("Number of fake teachers:")
            .with_default(&DEFAULT_TEACHER_COUNT.to_string())
            .prompt()?;
        match raw.trim().parse::<u32>() {
            Ok(value) if (1..=MAX_TEACHER_COUNT).contains(&value) => return Ok(value),
            _ => println!("Enter a number between 1 and {MAX_TEACHER_COUNT}."),
        }
    }
}

async fn find_organization(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
) -> Result<Option<String>> {
    let row = db
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT id FROM organizations ORDER BY created_at LIMIT 1",
            vec![],
        ))
        .await?;
    Ok(row.and_then(|value| value.try_get::<String>("", "id").ok()))
}

async fn teacher_exists(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &str,
    email: &str,
) -> Result<bool> {
    let row = db
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT id FROM users WHERE org_id = ? AND email = ? AND is_deleted = 0 LIMIT 1",
            vec![org_id.to_string().into(), email.to_string().into()],
        ))
        .await?;
    Ok(row.is_some())
}

async fn execute_with_values(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    sql: &str,
    values: Vec<sea_orm::Value>,
) -> Result<()> {
    let sql = if backend == DatabaseBackend::Postgres {
        replace_placeholders(sql)
    } else {
        sql.to_string()
    };
    db.execute(Statement::from_sql_and_values(backend, sql, values))
        .await?;
    Ok(())
}

fn replace_placeholders(sql: &str) -> String {
    let mut index = 0;
    sql.chars()
        .map(|character| {
            if character == '?' {
                index += 1;
                format!("${index}")
            } else {
                character.to_string()
            }
        })
        .collect()
}
