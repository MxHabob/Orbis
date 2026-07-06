use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::{behavior_record, notification, org_unit, audit_log},
    params::{BehaviorParams, NotificationParams, OrgUnitParams},
    service::behavior::{AIBehaviorSubmission, UpdateBehaviorRequest},
    service::org_unit::{CreateOrgUnitRequest, UpdateOrgUnitRequest},
    service::dashboard::DashboardStats,
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Behavior ───────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_behavior_records(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: BehaviorParams,
) -> CmdResult<Vec<behavior_record::Model>> {
    state.service().await
        .get_behavior_records(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn submit_ai_behavior(
    state: State<'_, Arc<AppState>>,
    request: AIBehaviorSubmission,
) -> CmdResult<behavior_record::Model> {
    state.service().await
        .submit_ai_behavior(request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_behavior_record(
    state: State<'_, Arc<AppState>>,
    record_id: String,
    reviewed_by: String,
    request: UpdateBehaviorRequest,
) -> CmdResult<behavior_record::Model> {
    state.service().await
        .update_behavior_record(&record_id, request, &reviewed_by)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_behavior_record(
    state: State<'_, Arc<AppState>>,
    record_id: String,
) -> CmdResult<()> {
    state.service().await
        .delete_behavior_record(&record_id)
        .await
        .map_err(CommandError::from)
}

// ── Notifications ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_notifications(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    params: NotificationParams,
) -> CmdResult<Vec<notification::Model>> {
    state.service().await
        .get_notifications(&user_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn mark_notification_read(
    state: State<'_, Arc<AppState>>,
    notification_id: String,
) -> CmdResult<()> {
    state.service().await
        .mark_notification_read(&notification_id)
        .await
        .map_err(CommandError::from)
}

// ── Org Units ──────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_org_units(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: OrgUnitParams,
) -> CmdResult<Vec<org_unit::Model>> {
    state.service().await
        .get_org_units(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_org_unit(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateOrgUnitRequest,
) -> CmdResult<org_unit::Model> {
    state.service().await
        .create_org_unit(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_org_unit(
    state: State<'_, Arc<AppState>>,
    unit_id: String,
    request: UpdateOrgUnitRequest,
) -> CmdResult<org_unit::Model> {
    state.service().await
        .update_org_unit(&unit_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_org_unit(
    state: State<'_, Arc<AppState>>,
    unit_id: String,
) -> CmdResult<()> {
    state.service().await
        .delete_org_unit(&unit_id)
        .await
        .map_err(CommandError::from)
}

// ── Dashboard ──────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_dashboard_stats(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<DashboardStats> {
    state.service().await
        .get_dashboard_stats(&org_id)
        .await
        .map_err(CommandError::from)
}

// ── Audit Logs ─────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_audit_logs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<audit_log::Model>> {
    state.service().await
        .repo
        .get_audit_logs(&org_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}

// ── Onboarding / School Setup ──────────────────────────────────────────────

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

#[tauri::command]
pub async fn initialize_school_setup(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    config: SchoolSetupConfig,
) -> CmdResult<()> {
    use sea_orm::{TransactionTrait, ConnectionTrait, Statement};
    let svc = state.service().await;
    let db = &svc.repo.db;
    let now_str = chrono::Utc::now().to_rfc3339();

    // Cache values we need outside or multiple times to avoid borrow moves
    let root_cycle_id = config.cycles.first().map(|c| c.id.clone()).unwrap_or_default();
    let root_cycle = config.cycles.iter().find(|c| c.parent_id.is_none()).map(|c| (c.id.clone(), c.name.clone(), c.name_ar.clone(), c.start_date.clone(), c.end_date.clone()));
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

            // 2. Clear Old Structures (except users and core audit logs)
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
                    config.root_unit_name.into(),
                    config.root_unit_name_ar.into(),
                    now_str.clone().into(),
                    now_str.clone().into(),
                ],
            ))
            .await
            .map_err(|e| e.to_string())?;

            // 4. Insert Academic Cycles
            for cyc in &config.cycles {
                let is_root = cyc.parent_id.is_none();
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO academic_cycles (id, org_id, name, name_ar, parent_id, start_date, end_date, type, grading_system, weight, is_current, is_locked, is_enrollment_open, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'percentage', ?, ?, 0, 1, ?, ?)",
                    vec![
                        cyc.id.clone().into(),
                        org_id.clone().into(),
                        cyc.name.clone().into(),
                        cyc.name_ar.clone().into(),
                        cyc.parent_id.clone().into(),
                        cyc.start_date.clone().into(),
                        cyc.end_date.clone().into(),
                        cyc.cycle_type.clone().into(),
                        cyc.weight.into(),
                        (if is_root { 1 } else { 0 }).into(), // Set root cycle as current
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            // 5. Insert Time Slots
            for (idx, slot) in config.time_slots.iter().enumerate() {
                let sh: i32 = slot.start_time.split(':').next().unwrap_or("0").parse().unwrap_or(0);
                let sm: i32 = slot.start_time.split(':').nth(1).unwrap_or("0").parse().unwrap_or(0);
                let eh: i32 = slot.end_time.split(':').next().unwrap_or("0").parse().unwrap_or(0);
                let em: i32 = slot.end_time.split(':').nth(1).unwrap_or("0").parse().unwrap_or(0);
                let duration = (eh * 60 + em) - (sh * 60 + sm);

                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO time_slots (id, org_id, cycle_id, slot_number, name, name_ar, start_time, end_time, duration_minutes, is_break, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 1, ?, ?)",
                    vec![
                        uuid::Uuid::new_v4().to_string().into(),
                        org_id.clone().into(),
                        root_cycle_id.clone().into(),
                        ((idx + 1) as i32).into(),
                        slot.name.clone().into(),
                        slot.name_ar.clone().into(),
                        slot.start_time.clone().into(),
                        slot.end_time.clone().into(),
                        duration.into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            // 6. Insert Rooms & Org Units
            let mut unique_buildings = Vec::new();
            for rm in &config.rooms {
                if !rm.building.is_empty() && !unique_buildings.contains(&rm.building) {
                    unique_buildings.push(rm.building.clone());
                }
            }

            let mut building_id_map = std::collections::HashMap::new();
            for (idx, bld_name) in unique_buildings.iter().enumerate() {
                let bld_id = format!("unit-bld-{}-{}", org_id, idx + 1);
                building_id_map.insert(bld_name.clone(), bld_id.clone());

                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO org_units (id, org_id, name, name_ar, type, parent_id, path, depth, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, 'building', ?, ?, 1, 1, ?, ?)",
                    vec![
                        bld_id.clone().into(),
                        org_id.clone().into(),
                        bld_name.clone().into(),
                        bld_name.clone().into(),
                        config.root_unit_id.clone().into(),
                        format!("{}/{}", config.root_unit_id, bld_id).into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;
            }

            for rm in &config.rooms {
                // First insert into rooms table
                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, building, floor, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)",
                    vec![
                        rm.id.clone().into(),
                        org_id.clone().into(),
                        rm.name.clone().into(),
                        rm.name_ar.clone().into(),
                        rm.capacity.into(),
                        rm.room_type.clone().into(),
                        rm.building.clone().into(),
                        rm.floor.into(),
                        now_str.clone().into(),
                        now_str.clone().into(),
                    ],
                ))
                .await
                .map_err(|e| e.to_string())?;

                // Then insert into org_units table
                let parent_id = building_id_map.get(&rm.building).cloned().unwrap_or_else(|| config.root_unit_id.clone());
                let depth = if building_id_map.contains_key(&rm.building) { 2 } else { 1 };
                let path = if parent_id == config.root_unit_id {
                    format!("{}/{}", config.root_unit_id, rm.id)
                } else {
                    format!("{}/{}/{}", config.root_unit_id, parent_id, rm.id)
                };

                txn.execute(Statement::from_sql_and_values(
                    backend,
                    "INSERT INTO org_units (id, org_id, name, name_ar, type, parent_id, path, depth, capacity, room_number, floor, building, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, 'classroom', ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)",
                    vec![
                        rm.id.clone().into(),
                        org_id.clone().into(),
                        rm.name.clone().into(),
                        rm.name_ar.clone().into(),
                        parent_id.into(),
                        path.into(),
                        depth.into(),
                        rm.capacity.into(),
                        rm.name.clone().into(),
                        rm.floor.into(),
                        rm.building.clone().into(),
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
                    "INSERT INTO subjects (id, org_id, unit_id, code, name, name_ar, is_core, is_elective, requires_lab, requires_practical, color, icon, sort_order, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, 1, 0, 0, 0, ?, ?, ?, 1, ?, ?)",
                    vec![
                        sub.id.into(),
                        org_id.clone().into(),
                        config.root_unit_id.clone().into(),
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

    // 8.5. Register the year in the registry database
    if let Some((root_id, root_name, root_name_ar, root_start, root_end)) = root_cycle {
        use sea_orm::ConnectionTrait;
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
        
        // Also set this year as active in provider
        let _ = state.provider.switch_year(&root_id).await;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_setup_status(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<bool> {
    use sea_orm::EntityTrait;
    use core_domain::entities::organization;

    let svc = state.service().await;
    let db = &svc.repo.db;
    let org = organization::Entity::find_by_id(&org_id)
        .one(db)
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

