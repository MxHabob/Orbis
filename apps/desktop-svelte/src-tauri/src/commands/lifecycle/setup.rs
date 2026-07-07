use tauri::State;
use std::sync::Arc;
use sea_orm::{ConnectionTrait, Statement, EntityTrait};
use domain_lifecycle::entities::organization;
use domain_iam::entities::audit_log;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Onboarding / School Setup Types ─────────────────────────────────────────

#[derive(serde::Deserialize, Debug)]
pub struct CycleSetupItem {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub name_ar: String,
    pub start_date: String,
    pub end_date: String,
    pub cycle_type: String,
    pub weight: Option<i32>,
}

#[derive(serde::Deserialize, Debug)]
pub struct TimeSlotSetupItem {
    pub name: String,
    pub name_ar: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RoomSetupItem {
    pub id: String,
    pub name: String,
    pub name_ar: String,
    pub capacity: i32,
    pub room_type: String,
    pub building: String,
    pub floor: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct SubjectSetupItem {
    pub id: String,
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub color: String,
    pub icon: String,
    pub sort_order: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct ClassSetupItem {
    pub id: String,
    pub name: String,
    pub name_ar: String,
    pub level: String,
    pub division: String,
    pub color: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SchoolSetupConfig {
    pub org_name: String,
    pub org_name_ar: String,
    pub timezone: String,
    pub locale: String,
    pub currency: String,
    
    pub root_unit_id: String,
    pub root_unit_name: String,
    pub root_unit_name_ar: String,

    pub cycles: Vec<CycleSetupItem>,
    pub time_slots: Vec<TimeSlotSetupItem>,
    pub rooms: Vec<RoomSetupItem>,
    pub subjects: Vec<SubjectSetupItem>,
    pub classes: Vec<ClassSetupItem>,
}

// ── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn initialize_school_setup(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    config: SchoolSetupConfig,
) -> CmdResult<()> {
    use sea_orm::TransactionTrait;
    let db = state.active_db().await;
    let now_str = chrono::Utc::now().to_rfc3339();

    let root_cycle_id = config.cycles.first().map(|c| c.id.clone()).unwrap_or_default();
    let root_cycle = config.cycles.iter()
        .find(|c| c.parent_id.is_none())
        .map(|c| (c.id.clone(), c.name.clone(), c.name_ar.clone(), c.start_date.clone(), c.end_date.clone()));
        
    let org_id_for_registry = org_id.clone();
    let now_str_for_registry = now_str.clone();

    db.transaction::<_, (), String>(move |txn| {
        let org_id = org_id.clone();
        let root_cycle_id = root_cycle_id.clone();
        Box::pin(async move {
            let backend = txn.get_database_backend();

            // 1. Update Organization
            txn.execute(Statement::from_sql_and_values(
                backend,
                "UPDATE organizations SET name = ?, name_ar = ?, timezone = ?, locale = ?, currency = ?, updated_at = ? WHERE id = ?",
                vec![
                    config.org_name.into(),
                    config.org_name_ar.into(),
                    config.timezone.into(),
                    config.locale.into(),
                    config.currency.into(),
                    now_str.clone().into(),
                    org_id.clone().into(),
                ],
            ))
            .await
            .map_err(|e| e.to_string())?;

            // 2. Clear Old Structures
            txn.execute(Statement::from_sql_and_values(
                backend,
                "DELETE FROM class_subjects WHERE cycle_id IN (SELECT id FROM academic_cycles WHERE org_id = ?)",
                vec![org_id.clone().into()],
            ))
            .await
            .map_err(|e| e.to_string())?;

            for table in &["classes", "subjects", "rooms", "time_slots", "academic_cycles", "org_units"] {
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    &format!("DELETE FROM {} WHERE org_id = ?", table),
                    vec![org_id.clone().into()],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            // 3. Create Root Org Unit
            txn.execute(Statement::from_sql_and_values(
                backend,
                "INSERT INTO org_units (id, org_id, name, name_ar, type, path, depth, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, 'school', 'school', 0, 1, ?, ?)",
                vec![
                    config.root_unit_id.clone().into(),
                    org_id.clone().into(),
                    config.root_unit_name.clone().into(),
                    config.root_unit_name_ar.clone().into(),
                    now_str.clone().into(),
                    now_str.clone().into(),
                ],
            ))
            .await
            .map_err(|e| e.to_string())?;

            // 4. Insert Academic Cycles
            for cy in config.cycles {
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO academic_cycles (id, org_id, name, name_ar, type, parent_id, start_date, end_date, weight, grading_system, passing_grade, max_grade, is_current, is_locked, is_enrollment_open, is_grade_entry_open, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'saudi_5', 50, 100, 1, 0, 1, 0, ?, ?)",
                    vec![
                        cy.id.into(),
                        org_id.clone().into(),
                        cy.name.into(),
                        cy.name_ar.into(),
                        cy.cycle_type.into(),
                        cy.parent_id.into(),
                        cy.start_date.into(),
                        cy.end_date.into(),
                        cy.weight.unwrap_or(100).into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            // 5. Insert Time Slots
            for (idx, ts) in config.time_slots.into_iter().enumerate() {
                let id = uuid::Uuid::new_v4().to_string();
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO time_slots (id, org_id, cycle_id, slot_number, name, name_ar, start_time, end_time, duration_minutes, is_break, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 45, 0, 1, ?, ?)",
                    vec![
                        id.into(),
                        org_id.clone().into(),
                        root_cycle_id.clone().into(),
                        (idx as i32 + 1).into(),
                        ts.name.into(),
                        ts.name_ar.into(),
                        ts.start_time.into(),
                        ts.end_time.into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            // 6. Insert Rooms
            for rm in config.rooms {
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, building, floor, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)",
                    vec![
                        rm.id.into(),
                        org_id.clone().into(),
                        rm.name.into(),
                        rm.name_ar.into(),
                        rm.capacity.into(),
                        rm.room_type.into(),
                        rm.building.into(),
                        rm.floor.into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            // 7. Insert Subjects
            for sub in config.subjects {
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO subjects (id, org_id, code, name, name_ar, color, icon, sort_order, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)",
                    vec![
                        sub.id.into(),
                        org_id.clone().into(),
                        sub.code.into(),
                        sub.name.into(),
                        sub.name_ar.into(),
                        sub.color.into(),
                        sub.icon.into(),
                        sub.sort_order.into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            // 8. Insert Classes
            for cl in config.classes {
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO classes (id, org_id, unit_id, cycle_id, name, name_ar, level, division, capacity, color, is_active, is_deleted, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 30, ?, 1, 0, ?, ?)",
                    vec![
                        cl.id.into(),
                        org_id.clone().into(),
                        config.root_unit_id.clone().into(),
                        root_cycle_id.clone().into(),
                        cl.name.into(),
                        cl.name_ar.into(),
                        cl.level.into(),
                        cl.division.into(),
                        cl.color.into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }
            Ok(())
        })
    })
    .await
    .map_err(|e| e.to_string())
    .map_err(CommandError::from)?;

    // Register year in central registry
    if let Some((root_id, root_name, root_name_ar, root_start, root_end)) = root_cycle {
        let registry_db = state.provider.registry();
        let reg_backend = registry_db.get_database_backend();
        
        let _ = registry_db.execute(Statement::from_sql_and_values(
            reg_backend,
            "INSERT OR IGNORE INTO academic_years (id, org_id, name, name_ar, start_date, end_date, hijri_start_date, hijri_end_date, db_filename, status, is_current, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, NULL, NULL, '../orbis.db', 'active', 1, ?, ?)",
            vec![
                root_id.clone().into(),
                org_id_for_registry.into(),
                root_name.into(),
                root_name_ar.into(),
                root_start.into(),
                root_end.into(),
                now_str_for_registry.clone().into(),
                now_str_for_registry.into(),
            ]
        ))
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;
        
        let _ = state.provider.switch_year(&root_id).await;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_setup_status(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<bool> {
    let db = state.active_db().await;
    let org = organization::Entity::find_by_id(&org_id)
        .one(&db)
        .await
        .map_err(|e| e.to_string())
        .map_err(CommandError::from)?;

    let is_complete = if let Some(o) = org {
        if let Some(settings_str) = o.settings {
            settings_str.contains("registration_config")
        } else {
            false
        }
    } else {
        false
    };

    Ok(is_complete)
}

#[tauri::command]
pub async fn get_audit_logs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<audit_log::Model>> {
    use sea_orm::ColumnTrait;
    use sea_orm::QueryFilter;
    let db = state.active_db().await;
    let logs = audit_log::Entity::find()
        .filter(audit_log::Column::OrgId.eq(org_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(logs)
}
