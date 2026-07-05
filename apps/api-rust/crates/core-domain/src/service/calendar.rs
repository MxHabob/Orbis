//! Calendar Service — manages academic calendar events with tri-modal execution.
//!
//! Execution modes:
//! - `manual`:    No automation. User must act explicitly.
//! - `suggested`: System detects the event is due, validates prerequisites,
//!                and creates a notification. User confirms to execute.
//! - `automatic`: System validates prerequisites and executes immediately
//!                when the event's scheduled date arrives.

use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::entities::{academic_calendar_event, org_academic_policy, academic_cycle};

// ── Request DTOs ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCalendarEventRequest {
    pub cycle_id: String,
    pub event_type: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub description: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub execution_mode: Option<String>,
    pub auto_action: Option<String>,
    pub prerequisites: Option<String>,
    pub is_recurring: Option<i32>,
    pub recurrence_rule: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCalendarEventRequest {
    pub title: Option<String>,
    pub title_ar: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub execution_mode: Option<String>,
    pub auto_action: Option<String>,
    pub prerequisites: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<i32>,
}

// ── Response DTOs ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProcessingResult {
    pub event_id: String,
    pub event_type: String,
    pub title: String,
    pub execution_mode: String,
    pub outcome: String, // executed | suggested | skipped | blocked
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarProcessingReport {
    pub executed: Vec<EventProcessingResult>,
    pub suggested: Vec<EventProcessingResult>,
    pub skipped: Vec<EventProcessingResult>,
    pub blocked: Vec<EventProcessingResult>,
}

impl CalendarProcessingReport {
    pub fn new() -> Self {
        Self {
            executed: Vec::new(),
            suggested: Vec::new(),
            skipped: Vec::new(),
            blocked: Vec::new(),
        }
    }
}

// ── Policy DTO ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrgPolicyRequest {
    pub default_grading_system: Option<String>,
    pub default_passing_grade: Option<i32>,
    pub default_max_grade: Option<i32>,
    pub enrollment_auto_close_on_capacity: Option<i32>,
    pub allow_late_enrollment: Option<i32>,
    pub late_enrollment_requires_approval: Option<i32>,
    pub grade_entry_auto_close_on_deadline: Option<i32>,
    pub allow_grade_modification_after_close: Option<i32>,
    pub grade_modification_requires_approval: Option<i32>,
    pub default_event_execution_mode: Option<String>,
    pub settings: Option<String>,
}

// ── Service ────────────────────────────────────────────────────────

impl super::Service {
    // ── Calendar Events CRUD ──────────────────────────────────────

    /// Get all calendar events for a cycle.
    pub async fn get_calendar_events(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<academic_calendar_event::Model>, String> {
        academic_calendar_event::Entity::find()
            .filter(academic_calendar_event::Column::OrgId.eq(org_id))
            .filter(academic_calendar_event::Column::CycleId.eq(cycle_id))
            .order_by_asc(academic_calendar_event::Column::StartDate)
            .order_by_asc(academic_calendar_event::Column::SortOrder)
            .all(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get all pending (unexecuted, active) events up to a given date.
    pub async fn get_pending_events(
        &self,
        org_id: &str,
        before_date: &str,
    ) -> Result<Vec<academic_calendar_event::Model>, String> {
        academic_calendar_event::Entity::find()
            .filter(academic_calendar_event::Column::OrgId.eq(org_id))
            .filter(academic_calendar_event::Column::IsActive.eq(1))
            .filter(academic_calendar_event::Column::ExecutedAt.is_null())
            .filter(academic_calendar_event::Column::StartDate.lte(before_date))
            .order_by_asc(academic_calendar_event::Column::StartDate)
            .all(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Create a calendar event.
    pub async fn create_calendar_event(
        &self,
        org_id: &str,
        req: CreateCalendarEventRequest,
    ) -> Result<academic_calendar_event::Model, String> {
        let valid_modes = ["manual", "suggested", "automatic"];
        let mode = req.execution_mode.as_deref().unwrap_or("suggested");
        if !valid_modes.contains(&mode) {
            return Err(format!(
                "Invalid execution_mode: {}. Valid: {:?}",
                mode, valid_modes
            ));
        }

        let now = Utc::now().to_rfc3339();
        let event = academic_calendar_event::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: req.cycle_id,
            event_type: req.event_type,
            title: req.title,
            title_ar: req.title_ar,
            description: req.description,
            start_date: req.start_date,
            end_date: req.end_date,
            start_time: req.start_time,
            end_time: req.end_time,
            execution_mode: mode.to_string(),
            auto_action: req.auto_action,
            prerequisites: req.prerequisites,
            executed_at: None,
            executed_by: None,
            execution_result: None,
            execution_notes: None,
            is_recurring: req.is_recurring.unwrap_or(0),
            recurrence_rule: req.recurrence_rule,
            color: req.color,
            is_active: 1,
            sort_order: req.sort_order,
            created_by: None,
            created_at: now.clone(),
            updated_at: now,
        };

        let active: academic_calendar_event::ActiveModel = event.into();
        active
            .insert(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Update a calendar event.
    pub async fn update_calendar_event(
        &self,
        event_id: &str,
        req: UpdateCalendarEventRequest,
    ) -> Result<academic_calendar_event::Model, String> {
        let existing = academic_calendar_event::Entity::find_by_id(event_id)
            .one(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Calendar event not found".to_string())?;

        // Cannot update already-executed events
        if existing.executed_at.is_some() {
            return Err("Cannot modify an already-executed event".to_string());
        }

        let mut active: academic_calendar_event::ActiveModel = existing.into();
        let now = Utc::now().to_rfc3339();

        if let Some(v) = req.title { active.title = Set(v); }
        if let Some(v) = req.title_ar { active.title_ar = Set(Some(v)); }
        if let Some(v) = req.description { active.description = Set(Some(v)); }
        if let Some(v) = req.start_date { active.start_date = Set(v); }
        if let Some(v) = req.end_date { active.end_date = Set(v); }
        if let Some(v) = req.start_time { active.start_time = Set(Some(v)); }
        if let Some(v) = req.end_time { active.end_time = Set(Some(v)); }
        if let Some(v) = req.execution_mode { active.execution_mode = Set(v); }
        if let Some(v) = req.auto_action { active.auto_action = Set(Some(v)); }
        if let Some(v) = req.prerequisites { active.prerequisites = Set(Some(v)); }
        if let Some(v) = req.color { active.color = Set(Some(v)); }
        if let Some(v) = req.sort_order { active.sort_order = Set(Some(v)); }
        if let Some(v) = req.is_active { active.is_active = Set(v); }
        active.updated_at = Set(now);

        active
            .update(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Delete a calendar event.
    pub async fn delete_calendar_event(&self, event_id: &str) -> Result<(), String> {
        let result = academic_calendar_event::Entity::delete_by_id(event_id)
            .exec(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected == 0 {
            return Err("Calendar event not found".to_string());
        }
        Ok(())
    }

    /// Mark an event as executed.
    pub async fn mark_event_executed(
        &self,
        event_id: &str,
        executed_by: &str,
        result: &str,
        notes: Option<&str>,
    ) -> Result<academic_calendar_event::Model, String> {
        let existing = academic_calendar_event::Entity::find_by_id(event_id)
            .one(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Calendar event not found".to_string())?;

        let mut active: academic_calendar_event::ActiveModel = existing.into();
        let now = Utc::now().to_rfc3339();

        active.executed_at = Set(Some(now.clone()));
        active.executed_by = Set(Some(executed_by.to_string()));
        active.execution_result = Set(Some(result.to_string()));
        if let Some(n) = notes {
            active.execution_notes = Set(Some(n.to_string()));
        }
        active.updated_at = Set(now);

        active
            .update(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Execute a calendar event's action (open enrollment, close grades, etc.).
    pub async fn execute_calendar_event_action(
        &self,
        event: &academic_calendar_event::Model,
        user_id: &str,
    ) -> Result<EventProcessingResult, String> {
        // Parse the auto_action JSON
        let action: Option<serde_json::Value> = event
            .auto_action
            .as_ref()
            .and_then(|a| serde_json::from_str(a).ok());

        let action_str = action
            .as_ref()
            .and_then(|a| a.get("action"))
            .and_then(|a| a.as_str())
            .unwrap_or("");

        // Execute the action based on type
        let exec_result = match action_str {
            "open_enrollment" => {
                self.repo
                    .update_cycle(
                        &event.cycle_id,
                        crate::repository::academic_cycle::CycleUpdate {
                            is_enrollment_open: Some(1),
                            ..Default::default()
                        },
                    )
                    .await
                    .map(|_| "Enrollment opened".to_string())
                    .map_err(|e| e.to_string())
            }
            "close_enrollment" => {
                self.repo
                    .update_cycle(
                        &event.cycle_id,
                        crate::repository::academic_cycle::CycleUpdate {
                            is_enrollment_open: Some(0),
                            ..Default::default()
                        },
                    )
                    .await
                    .map(|_| "Enrollment closed".to_string())
                    .map_err(|e| e.to_string())
            }
            "open_grade_entry" => {
                self.open_grade_entry(&event.cycle_id, None, None)
                    .await
                    .map(|_| "Grade entry opened".to_string())
            }
            "close_grade_entry" => {
                self.close_grade_entry(&event.cycle_id)
                    .await
                    .map(|_| "Grade entry closed".to_string())
            }
            "lock_cycle" => {
                self.lock_cycle(&event.cycle_id, user_id)
                    .await
                    .map(|_| "Cycle locked".to_string())
            }
            _ => {
                // No automated action — just mark as executed
                Ok(format!("Event '{}' marked as executed (no auto-action)", event.title))
            }
        };

        match exec_result {
            Ok(msg) => {
                // Mark the event as executed
                self.mark_event_executed(&event.id, user_id, "success", Some(&msg))
                    .await?;
                Ok(EventProcessingResult {
                    event_id: event.id.clone(),
                    event_type: event.event_type.clone(),
                    title: event.title.clone(),
                    execution_mode: event.execution_mode.clone(),
                    outcome: "executed".to_string(),
                    message: Some(msg),
                })
            }
            Err(err) => {
                self.mark_event_executed(&event.id, user_id, "failed", Some(&err))
                    .await?;
                Ok(EventProcessingResult {
                    event_id: event.id.clone(),
                    event_type: event.event_type.clone(),
                    title: event.title.clone(),
                    execution_mode: event.execution_mode.clone(),
                    outcome: "failed".to_string(),
                    message: Some(err),
                })
            }
        }
    }

    // ── Default Event Generation ──────────────────────────────────

    /// Generate default calendar events for a new cycle based on its dates.
    pub async fn generate_default_calendar_events(
        &self,
        org_id: &str,
        cycle: &academic_cycle::Model,
        default_mode: &str,
    ) -> Result<Vec<academic_calendar_event::Model>, String> {
        let now = Utc::now().to_rfc3339();
        let mut events = Vec::new();

        // Helper to build a default event
        let make_event = |event_type: &str,
                          title: &str,
                          title_ar: &str,
                          start: &str,
                          end: &str,
                          action: Option<&str>,
                          order: i32| {
            academic_calendar_event::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                cycle_id: cycle.id.clone(),
                event_type: event_type.to_string(),
                title: title.to_string(),
                title_ar: Some(title_ar.to_string()),
                description: None,
                start_date: start.to_string(),
                end_date: end.to_string(),
                start_time: None,
                end_time: None,
                execution_mode: default_mode.to_string(),
                auto_action: action.map(|a| a.to_string()),
                prerequisites: None,
                executed_at: None,
                executed_by: None,
                execution_result: None,
                execution_notes: None,
                is_recurring: 0,
                recurrence_rule: None,
                color: None,
                is_active: 1,
                sort_order: Some(order),
                created_by: Some("system".to_string()),
                created_at: now.clone(),
                updated_at: now.clone(),
            }
        };

        // 1. Cycle Start / Enrollment Open
        events.push(make_event(
            "enrollment_open",
            "Enrollment Opens",
            "فتح التسجيل",
            &cycle.start_date,
            &cycle.start_date,
            Some(r#"{"action":"open_enrollment"}"#),
            10,
        ));

        // 2. Classes Start
        events.push(make_event(
            "classes_start",
            "Classes Begin",
            "بدء الدراسة",
            &cycle.start_date,
            &cycle.start_date,
            None,
            20,
        ));

        // 3. Exam period (if dates exist)
        if let (Some(ref exam_start), Some(ref exam_end)) =
            (&cycle.exam_start_date, &cycle.exam_end_date)
        {
            if !exam_start.is_empty() && !exam_end.is_empty() {
                events.push(make_event(
                    "exam_start",
                    "Exams Begin",
                    "بدء الامتحانات",
                    exam_start,
                    exam_start,
                    None,
                    50,
                ));

                events.push(make_event(
                    "exam_end",
                    "Exams End",
                    "انتهاء الامتحانات",
                    exam_end,
                    exam_end,
                    None,
                    60,
                ));

                // Grade entry opens after exams
                events.push(make_event(
                    "grade_entry_open",
                    "Grade Entry Opens",
                    "فتح إدخال الدرجات",
                    exam_end,
                    exam_end,
                    Some(r#"{"action":"open_grade_entry"}"#),
                    70,
                ));
            }
        }

        // 4. Enrollment Close (a week into the cycle, if possible)
        events.push(make_event(
            "enrollment_close",
            "Enrollment Closes",
            "إغلاق التسجيل",
            &cycle.start_date, // Simplified; ideally start_date + 7 days
            &cycle.start_date,
            Some(r#"{"action":"close_enrollment"}"#),
            30,
        ));

        // 5. Grade entry close / Cycle end
        events.push(make_event(
            "grade_entry_close",
            "Grade Entry Closes",
            "إغلاق إدخال الدرجات",
            &cycle.end_date,
            &cycle.end_date,
            Some(r#"{"action":"close_grade_entry"}"#),
            80,
        ));

        events.push(make_event(
            "cycle_end",
            "Cycle Ends",
            "نهاية الدورة",
            &cycle.end_date,
            &cycle.end_date,
            Some(r#"{"action":"lock_cycle"}"#),
            90,
        ));

        // Insert all events
        let mut created = Vec::new();
        for event in events {
            let active: academic_calendar_event::ActiveModel = event.into();
            let inserted = active
                .insert(&*self.repo.db)
                .await
                .map_err(|e| e.to_string())?;
            created.push(inserted);
        }

        Ok(created)
    }

    // ── Org Academic Policies ─────────────────────────────────────

    /// Get the academic policy for an organization. Returns default values if none exists.
    pub async fn get_org_academic_policy(
        &self,
        org_id: &str,
    ) -> Result<org_academic_policy::Model, String> {
        let existing = org_academic_policy::Entity::find()
            .filter(org_academic_policy::Column::OrgId.eq(org_id))
            .one(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())?;

        match existing {
            Some(policy) => Ok(policy),
            None => {
                // Create default policy
                let now = Utc::now().to_rfc3339();
                let default = org_academic_policy::Model {
                    id: Uuid::new_v4().to_string(),
                    org_id: org_id.to_string(),
                    default_grading_system: "percentage".to_string(),
                    default_passing_grade: 50,
                    default_max_grade: 100,
                    enrollment_auto_close_on_capacity: 0,
                    allow_late_enrollment: 1,
                    late_enrollment_requires_approval: 1,
                    grade_entry_auto_close_on_deadline: 1,
                    allow_grade_modification_after_close: 0,
                    grade_modification_requires_approval: 1,
                    default_event_execution_mode: "suggested".to_string(),
                    settings: None,
                    created_at: now.clone(),
                    updated_at: now,
                };

                let active: org_academic_policy::ActiveModel = default.into();
                active
                    .insert(&*self.repo.db)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }

    /// Update the academic policy for an organization.
    pub async fn update_org_academic_policy(
        &self,
        org_id: &str,
        req: CreateOrgPolicyRequest,
    ) -> Result<org_academic_policy::Model, String> {
        // Ensure policy exists first
        let policy = self.get_org_academic_policy(org_id).await?;

        let mut active: org_academic_policy::ActiveModel = policy.into();
        let now = Utc::now().to_rfc3339();

        if let Some(v) = req.default_grading_system { active.default_grading_system = Set(v); }
        if let Some(v) = req.default_passing_grade { active.default_passing_grade = Set(v); }
        if let Some(v) = req.default_max_grade { active.default_max_grade = Set(v); }
        if let Some(v) = req.enrollment_auto_close_on_capacity { active.enrollment_auto_close_on_capacity = Set(v); }
        if let Some(v) = req.allow_late_enrollment { active.allow_late_enrollment = Set(v); }
        if let Some(v) = req.late_enrollment_requires_approval { active.late_enrollment_requires_approval = Set(v); }
        if let Some(v) = req.grade_entry_auto_close_on_deadline { active.grade_entry_auto_close_on_deadline = Set(v); }
        if let Some(v) = req.allow_grade_modification_after_close { active.allow_grade_modification_after_close = Set(v); }
        if let Some(v) = req.grade_modification_requires_approval { active.grade_modification_requires_approval = Set(v); }
        if let Some(v) = req.default_event_execution_mode { active.default_event_execution_mode = Set(v); }
        if let Some(v) = req.settings { active.settings = Set(Some(v)); }
        active.updated_at = Set(now);

        active
            .update(&*self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }
}
