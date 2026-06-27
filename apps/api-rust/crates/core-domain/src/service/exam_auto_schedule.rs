use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to auto-generate an optimal exam schedule
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AutoScheduleRequest {
    pub exam_session_id: String,
    /// Working days (ISO format: 2026-01-15, 2026-01-16, ...)
    pub available_dates: Vec<String>,
    /// Max exams per day (default: 2)
    pub max_exams_per_day: Option<i32>,
    /// Preferred period: "morning", "afternoon", or "both"
    pub preferred_period: Option<String>,
    /// Default exam duration in minutes (default: 90)
    pub default_duration_minutes: Option<i32>,
    /// Morning period start time (default: "08:00")
    pub morning_start: Option<String>,
    /// Afternoon period start time (default: "13:00")
    pub afternoon_start: Option<String>,
    /// Gap between consecutive exams in minutes (default: 30)
    pub gap_minutes: Option<i32>,
    /// Subjects that should not be on the same day
    pub conflict_groups: Option<Vec<Vec<String>>>,
    /// Priority subjects to schedule first (e.g., core subjects)
    pub priority_subject_ids: Option<Vec<String>>,
}

/// Result of the auto-scheduling operation
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AutoScheduleResult {
    pub total_subjects: usize,
    pub scheduled_count: usize,
    pub unscheduled: Vec<UnscheduledSubject>,
    pub schedule: Vec<ScheduleEntry>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ScheduleEntry {
    pub exam_schedule_id: String,
    pub subject_id: String,
    pub subject_name: Option<String>,
    pub exam_date: String,
    pub start_time: String,
    pub end_time: String,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UnscheduledSubject {
    pub subject_id: String,
    pub reason: String,
}

/// Internal slot representation for the scheduling algorithm
#[derive(Debug, Clone)]
struct TimeSlot {
    date: String,
    period: String,
    start_time: String,
    end_time: String,
    is_used: bool,
}

/// Subject with metadata for scheduling priority
#[derive(Debug, Clone)]
struct SubjectToSchedule {
    exam_subject_id: String,
    subject_id: String,
    duration_minutes: i32,
    priority: i32,
    conflict_group: Option<usize>,
}

impl super::Service {
    /// Auto-generate an optimal exam schedule using a constraint-based greedy algorithm.
    ///
    /// Algorithm:
    /// 1. Collect all subjects to schedule, sorted by priority
    /// 2. Generate available time slots from available_dates
    /// 3. Apply constraint propagation (conflict groups, max per day)
    /// 4. Greedy assignment: highest priority subjects first, respecting constraints
    pub async fn auto_schedule_exams(
        &self,
        org_id: &str,
        req: AutoScheduleRequest,
    ) -> Result<AutoScheduleResult, String> {
        let session = self.get_exam_session(org_id, &req.exam_session_id).await?;

        if session.status != "draft" && session.status != "scheduled" {
            return Err("can only auto-schedule for draft or scheduled sessions".to_string());
        }

        // 1. Get all subjects for this session
        let subjects = self
            .repo
            .get_exam_subjects(&req.exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        if subjects.is_empty() {
            return Err("no exam subjects found for this session".to_string());
        }

        if req.available_dates.is_empty() {
            return Err("available_dates cannot be empty".to_string());
        }

        let max_per_day = req.max_exams_per_day.unwrap_or(2) as usize;
        let default_duration = req.default_duration_minutes.unwrap_or(90);
        let morning_start = req.morning_start.as_deref().unwrap_or("08:00");
        let afternoon_start = req.afternoon_start.as_deref().unwrap_or("13:00");
        let gap = req.gap_minutes.unwrap_or(30);
        let preferred = req.preferred_period.as_deref().unwrap_or("both");
        let priority_ids = req.priority_subject_ids.unwrap_or_default();
        let conflict_groups = req.conflict_groups.unwrap_or_default();

        // 2. Build subjects to schedule with priorities
        let mut to_schedule: Vec<SubjectToSchedule> = subjects
            .iter()
            .filter(|s| s.is_included == 1)
            .map(|s| {
                let priority = if priority_ids.contains(&s.subject_id) { 10 } else { 1 };
                let conflict_group = conflict_groups
                    .iter()
                    .position(|group| group.contains(&s.subject_id));

                SubjectToSchedule {
                    exam_subject_id: s.id.clone(),
                    subject_id: s.subject_id.clone(),
                    duration_minutes: s.duration_minutes,
                    priority,
                    conflict_group,
                }
            })
            .collect();

        // Sort by priority descending (highest priority first)
        to_schedule.sort_by(|a, b| b.priority.cmp(&a.priority));

        // 3. Generate time slots
        let mut slots: Vec<TimeSlot> = Vec::new();
        let mut sorted_dates = req.available_dates.clone();
        sorted_dates.sort();

        for date in &sorted_dates {
            if preferred == "morning" || preferred == "both" {
                let end = add_minutes(morning_start, default_duration);
                slots.push(TimeSlot {
                    date: date.clone(),
                    period: "morning".to_string(),
                    start_time: morning_start.to_string(),
                    end_time: end.clone(),
                    is_used: false,
                });

                // Second morning slot if max_per_day > 1
                if max_per_day > 1 && preferred != "both" {
                    let start2 = add_minutes(&end, gap);
                    let end2 = add_minutes(&start2, default_duration);
                    slots.push(TimeSlot {
                        date: date.clone(),
                        period: "morning".to_string(),
                        start_time: start2,
                        end_time: end2,
                        is_used: false,
                    });
                }
            }

            if preferred == "afternoon" || preferred == "both" {
                let end = add_minutes(afternoon_start, default_duration);
                slots.push(TimeSlot {
                    date: date.clone(),
                    period: "afternoon".to_string(),
                    start_time: afternoon_start.to_string(),
                    end_time: end,
                    is_used: false,
                });
            }
        }

        // 4. Greedy assignment with constraint checking
        let mut schedule_entries: Vec<ScheduleEntry> = Vec::new();
        let mut unscheduled: Vec<UnscheduledSubject> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();

        // Track which conflict groups are used on which dates
        let mut date_conflict_map: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();
        // Track exams per day
        let mut date_exam_count: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        let now = Utc::now().to_rfc3339();

        for subject in &to_schedule {
            let mut assigned = false;

            for slot in slots.iter_mut() {
                if slot.is_used {
                    continue;
                }

                // Check max exams per day
                let day_count = date_exam_count.get(&slot.date).copied().unwrap_or(0);
                if day_count >= max_per_day {
                    continue;
                }

                // Check conflict group constraint
                if let Some(group_idx) = subject.conflict_group {
                    let day_groups = date_conflict_map
                        .get(&slot.date)
                        .cloned()
                        .unwrap_or_default();
                    if day_groups.contains(&group_idx) {
                        continue;
                    }
                }

                // Assign this subject to this slot
                let duration = subject.duration_minutes;
                let end_time = add_minutes(&slot.start_time, duration);

                let schedule_id = Uuid::new_v4().to_string();

                // Persist to database
                let model = crate::entities::exam_schedule::Model {
                    id: schedule_id.clone(),
                    exam_session_id: req.exam_session_id.clone(),
                    exam_subject_id: subject.exam_subject_id.clone(),
                    exam_date: slot.date.clone(),
                    start_time: slot.start_time.clone(),
                    end_time: end_time.clone(),
                    period: slot.period.clone(),
                    applicable_class_ids: None,
                    room_ids: None,
                    status: "scheduled".to_string(),
                    postponed_to: None,
                    postponement_reason: None,
                    notes: Some("auto-generated".to_string()),
                    created_at: now.clone(),
                    updated_at: now.clone(),
                };

                self.repo
                    .insert_exam_schedule(model)
                    .await
                    .map_err(|e| e.to_string())?;

                schedule_entries.push(ScheduleEntry {
                    exam_schedule_id: schedule_id,
                    subject_id: subject.subject_id.clone(),
                    subject_name: None,
                    exam_date: slot.date.clone(),
                    start_time: slot.start_time.clone(),
                    end_time,
                    period: slot.period.clone(),
                });

                // Update tracking
                slot.is_used = true;
                *date_exam_count.entry(slot.date.clone()).or_insert(0) += 1;
                if let Some(group_idx) = subject.conflict_group {
                    date_conflict_map
                        .entry(slot.date.clone())
                        .or_default()
                        .push(group_idx);
                }

                assigned = true;
                break;
            }

            if !assigned {
                unscheduled.push(UnscheduledSubject {
                    subject_id: subject.subject_id.clone(),
                    reason: "no available slot satisfying all constraints".to_string(),
                });
            }
        }

        // Generate warnings
        if !unscheduled.is_empty() {
            warnings.push(format!(
                "{} subjects could not be scheduled — consider adding more dates",
                unscheduled.len()
            ));
        }

        let available_slots_remaining = slots.iter().filter(|s| !s.is_used).count();
        if available_slots_remaining == 0 && to_schedule.len() > schedule_entries.len() {
            warnings.push("all available time slots have been exhausted".to_string());
        }

        Ok(AutoScheduleResult {
            total_subjects: to_schedule.len(),
            scheduled_count: schedule_entries.len(),
            unscheduled,
            schedule: schedule_entries,
            warnings,
        })
    }

    /// Postpone a scheduled exam to a new date
    pub async fn postpone_exam(
        &self,
        org_id: &str,
        exam_session_id: &str,
        schedule_id: &str,
        new_date: &str,
        reason: &str,
    ) -> Result<crate::entities::exam_schedule::Model, String> {
        let _session = self.get_exam_session(org_id, exam_session_id).await?;

        let schedules = self
            .repo
            .get_exam_schedules(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        let schedule = schedules
            .iter()
            .find(|s| s.id == schedule_id)
            .ok_or_else(|| "exam schedule not found".to_string())?;

        let mut updated = schedule.clone();
        updated.status = "postponed".to_string();
        updated.postponed_to = Some(new_date.to_string());
        updated.postponement_reason = Some(reason.to_string());
        updated.updated_at = Utc::now().to_rfc3339();

        // Create new schedule entry for the new date
        let new_schedule = crate::entities::exam_schedule::Model {
            id: Uuid::new_v4().to_string(),
            exam_session_id: exam_session_id.to_string(),
            exam_subject_id: schedule.exam_subject_id.clone(),
            exam_date: new_date.to_string(),
            start_time: schedule.start_time.clone(),
            end_time: schedule.end_time.clone(),
            period: schedule.period.clone(),
            applicable_class_ids: schedule.applicable_class_ids.clone(),
            room_ids: schedule.room_ids.clone(),
            status: "scheduled".to_string(),
            postponed_to: None,
            postponement_reason: None,
            notes: Some(format!("postponed from {} — {}", schedule.exam_date, reason)),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        self.repo
            .insert_exam_schedule(new_schedule.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(new_schedule)
    }
}

/// Add minutes to a time string (HH:MM) and return new time string
fn add_minutes(time: &str, minutes: i32) -> String {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        return format!("{}:{:02}", minutes / 60, minutes % 60);
    }
    let h: i32 = parts[0].parse().unwrap_or(0);
    let m: i32 = parts[1].parse().unwrap_or(0);
    let total = h * 60 + m + minutes;
    format!("{:02}:{:02}", total / 60, total % 60)
}
