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
const DEV_PASSWORD: &str = "Super.12";
const TEST_FAMILY_PASSWORD: &str = "Super.12";

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
        let email = format!("teacher{index}@orbis.edu");
        if teacher_exists(&txn, backend, &org_id, &email).await? {
            skipped += 1;
            continue;
        }

        let person_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let assignment_id = Uuid::new_v4();
        let first_name = format!("Dev Teacher {index}");

        execute_with_values(
            &txn,
            backend,
            "INSERT INTO persons (id, org_id, first_name, last_name, full_name, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            vec![
                person_id.into(), org_id.into(),
                first_name.clone().into(), "".into(), first_name.clone().into(),
                now.clone().into(), now.clone().into(),
            ],
        ).await?;
        execute_with_values(
            &txn,
            backend,
            "INSERT INTO users (id, person_id, org_id, email, email_verified, password_hash, full_name, status, is_deleted, created_at, updated_at) VALUES (?, ?, ?, ?, 1, ?, ?, 'active', 0, ?, ?)",
            vec![
                user_id.into(), person_id.into(), org_id.into(), email.into(),
                password_hash.clone().into(), first_name.into(),
                now.clone().into(), now.clone().into(),
            ],
        ).await?;
        execute_with_values(
            &txn,
            backend,
            "INSERT INTO user_role_assignments (id, user_id, role_id, org_id, scope_type, scope_id, assigned_by, created_at) VALUES (?, ?, 'teacher', ?, 'global', NULL, NULL, ?)",
            vec![
                assignment_id.into(), user_id.into(), org_id.into(), now.clone().into(),
            ],
        ).await?;
        created += 1;
    }

    let (created_families, skipped_families) =
        seed_test_families(&txn, backend, &org_id, &password_hash, &now).await?;

    txn.commit().await?;
    
    let created_str = created.to_string();
    let skipped_str = skipped.to_string();
    output::print_summary_box("DEVELOPMENT DATA SEEDED", &[
        ("Created Teachers", &created_str),
        ("Skipped (Existing)", &skipped_str),
        ("Default Password", DEV_PASSWORD),
        ("Assigned Role", "teacher (Global Scope)"),
        ("Test Families Created", &created_families.to_string()),
        ("Test Families Skipped", &skipped_families.to_string()),
        ("Student/Parent Password", TEST_FAMILY_PASSWORD),
    ]);
    output::print_success("Development data successfully seeded.");
    Ok(())
}

async fn seed_test_families(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &Uuid,
    _teacher_password_hash: &str,
    now: &str,
) -> Result<(u32, u32)> {
    let password_hash = hash(TEST_FAMILY_PASSWORD, DEFAULT_COST)?;
    let families = [
        (
            "Mohamed Student one",
            "student1@orbis.edu",
            "STU-TEST-001",
            "male",
        ),
        (
            "Mohamed Student two",
            "student2@orbis.edu",
            "STU-TEST-002",
            "female",
        ),
    ];
    let parent_name = "Mohamed Parent";
    let parent_email = "parent@orbis.edu";
        let (parent_user_id, _) =
        if let Some(identity) = find_user_identity(db, backend, org_id, parent_email).await? {
            update_user_password(db, backend, &identity.0, &password_hash).await?;
            identity
        } else {
            let parent_person_id = Uuid::new_v4();
            let parent_user_id = Uuid::new_v4();
            insert_person(db, backend, org_id, &parent_person_id, parent_name, now).await?;
            insert_user(
                db,
                backend,
                org_id,
                &parent_user_id,
                &parent_person_id,
                parent_email,
                parent_name,
                &password_hash,
                now,
            )
            .await?;
            assign_role(db, backend, org_id, &parent_user_id, "parent", now).await?;
            (parent_user_id, parent_person_id)
        };
    let mut created = 0;
    let mut skipped = 0;

    for (student_name, student_email, student_number, gender) in families {
        if user_exists(db, backend, org_id, student_email).await? {
            if let Some((student_user_id, _)) =
                find_user_identity(db, backend, org_id, student_email).await?
            {
                update_user_password(db, backend, &student_user_id, &password_hash).await?;
                repair_guardian_link(db, backend, org_id, student_email, &parent_user_id).await?;
            }
            skipped += 1;
            continue;
        }

        let student_person_id = Uuid::new_v4();
        let student_user_id = Uuid::new_v4();
        let student_id = Uuid::new_v4();

        insert_person(db, backend, org_id, &student_person_id, student_name, now).await?;
        insert_user(
            db,
            backend,
            org_id,
            &student_user_id,
            &student_person_id,
            student_email,
            student_name,
            &password_hash,
            now,
        )
        .await?;
        assign_role(db, backend, org_id, &student_user_id, "student", now).await?;

        execute_with_values(
            db,
            backend,
            "INSERT INTO students (id, org_id, person_id, user_id, student_number, first_name, last_name, full_name, gender, nationality, status, standing, admitted_at, is_deleted, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'active', 'good_standing', ?, 0, ?, ?)",
            vec![
                student_id.into(), (*org_id).into(), student_person_id.into(),
                student_user_id.into(), student_number.into(), student_name.into(), "Test".into(),
                student_name.into(), gender.into(), "Yemeni".into(), now.into(), now.into(), now.into(),
            ],
        )
        .await?;
        execute_with_values(
            db,
            backend,
            "INSERT INTO student_guardians (id, student_id, guardian_id, relationship, relationship_ar, is_primary, is_emergency_contact, is_authorized_pickup, can_view_grades, can_view_attendance, can_view_behavior, can_view_finance, can_view_schedule, can_receive_notifications, is_payer, payment_share_percent, created_at, updated_at) VALUES (?, ?, ?, 'parent', 'ولي أمر', 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 100, ?, ?)",
            vec![
                Uuid::new_v4().into(), student_id.into(), parent_user_id.into(),
                now.into(), now.into(),
            ],
        )
        .await?;
        created += 1;
    }

    Ok((created, skipped))
}

async fn repair_guardian_link(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &Uuid,
    student_email: &str,
    guardian_id: &Uuid,
) -> Result<()> {
    let student = db
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT s.id FROM students s JOIN users u ON u.id = s.user_id WHERE s.org_id = ? AND u.org_id = ? AND u.email = ? AND s.is_deleted = 0 LIMIT 1",
            vec![
                (*org_id).into(),
                (*org_id).into(),
                student_email.to_string().into(),
            ],
        ))
        .await?
        .ok_or_else(|| anyhow!("Student account not found for {student_email}"))?;
    let student_id = student.try_get::<Uuid>("", "id")?;

    let link_exists = db
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT id FROM student_guardians WHERE student_id = ? AND relationship = 'parent' LIMIT 1",
            vec![student_id.into()],
        ))
        .await?
        .is_some();

    if link_exists {
        execute_with_values(
            db,
            backend,
            "UPDATE student_guardians SET guardian_id = ?, updated_at = ? WHERE student_id = ? AND relationship = 'parent'",
            vec![(*guardian_id).into(), Utc::now().to_rfc3339().into(), student_id.into()],
        )
        .await
    } else {
        let now = Utc::now().to_rfc3339();
        execute_with_values(
            db,
            backend,
            "INSERT INTO student_guardians (id, student_id, guardian_id, relationship, relationship_ar, is_primary, is_emergency_contact, is_authorized_pickup, can_view_grades, can_view_attendance, can_view_behavior, can_view_finance, can_view_schedule, can_receive_notifications, is_payer, payment_share_percent, created_at, updated_at) VALUES (?, ?, ?, 'parent', 'ولي أمر', 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 100, ?, ?)",
            vec![
                Uuid::new_v4().into(),
                student_id.into(),
                (*guardian_id).into(),
                now.clone().into(),
                now.into(),
            ],
        )
        .await
    }
}

async fn find_user_identity(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &Uuid,
    email: &str,
) -> Result<Option<(Uuid, Uuid)>> {
    let row = db
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT id, person_id FROM users WHERE org_id = ? AND email = ? LIMIT 1",
            vec![org_id.to_string().into(), email.to_string().into()],
        ))
        .await?;
    Ok(row.and_then(|value| {
        let user_id = value.try_get::<Uuid>("", "id").ok()?;
        let person_id = value.try_get::<Uuid>("", "person_id").ok()?;
        Some((user_id, person_id))
    }))
}

async fn user_exists(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &Uuid,
    email: &str,
) -> Result<bool> {
    let row = db
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT id FROM users WHERE org_id = ? AND email = ? LIMIT 1",
            vec![org_id.to_string().into(), email.to_string().into()],
        ))
        .await?;
    Ok(row.is_some())
}

async fn update_user_password(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    user_id: &Uuid,
    password_hash: &str,
) -> Result<()> {
    execute_with_values(
        db,
        backend,
        "UPDATE users SET password_hash = ?, status = 'active', is_deleted = 0, updated_at = ? WHERE id = ?",
        vec![
            password_hash.to_string().into(),
            Utc::now().to_rfc3339().into(),
            (*user_id).into(),
        ],
    )
    .await
}

async fn insert_person(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &Uuid,
    person_id: &Uuid,
    full_name: &str,
    now: &str,
) -> Result<()> {
    execute_with_values(
        db,
        backend,
        "INSERT INTO persons (id, org_id, first_name, last_name, full_name, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        vec![
            (*person_id).into(), (*org_id).into(), full_name.to_string().into(),
            "Test".into(), full_name.to_string().into(), now.to_string().into(), now.to_string().into(),
        ],
    )
    .await
}

async fn insert_user(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &Uuid,
    user_id: &Uuid,
    person_id: &Uuid,
    email: &str,
    full_name: &str,
    password_hash: &str,
    now: &str,
) -> Result<()> {
    execute_with_values(
        db,
        backend,
        "INSERT INTO users (id, person_id, org_id, email, email_verified, password_hash, full_name, status, is_deleted, created_at, updated_at) VALUES (?, ?, ?, ?, 1, ?, ?, 'active', 0, ?, ?)",
        vec![
            (*user_id).into(), (*person_id).into(), (*org_id).into(),
            email.to_string().into(), password_hash.to_string().into(), full_name.to_string().into(),
            now.to_string().into(), now.to_string().into(),
        ],
    )
    .await
}

async fn assign_role(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    org_id: &Uuid,
    user_id: &Uuid,
    role_id: &str,
    now: &str,
) -> Result<()> {
    execute_with_values(
        db,
        backend,
        "INSERT INTO user_role_assignments (id, user_id, role_id, org_id, scope_type, scope_id, assigned_by, created_at) VALUES (?, ?, ?, ?, 'global', NULL, NULL, ?)",
        vec![
            Uuid::new_v4().into(), (*user_id).into(), role_id.to_string().into(),
            (*org_id).into(), now.to_string().into(),
        ],
    )
    .await
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
) -> Result<Option<Uuid>> {
    let row = db
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT id FROM organizations ORDER BY created_at LIMIT 1",
            vec![],
        ))
        .await?;
    Ok(row.and_then(|value| value.try_get::<Uuid>("", "id").ok()))
}

async fn teacher_exists(
    db: &sea_orm::DatabaseTransaction,
    backend: DatabaseBackend,
    _org_id: &Uuid,
    email: &str,
) -> Result<bool> {
    let row = db
        .query_one(Statement::from_sql_and_values(
            backend,
            // users.email is globally unique, including soft-deleted users.
            "SELECT id FROM users WHERE email = ? LIMIT 1",
            vec![email.to_string().into()],
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
