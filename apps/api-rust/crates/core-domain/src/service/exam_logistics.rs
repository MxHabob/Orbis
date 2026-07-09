use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{exam_hall_allocation, exam_proctor};

/// Configuration for hall allocation algorithm
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AllocationConfig {
    #[serde(default)]
    pub separate_divisions: bool,
    #[serde(default)]
    pub separate_genders: bool,
    #[serde(default = "default_true")]
    pub balance_halls: bool,
}

fn default_true() -> bool { true }

/// Result of an allocation operation
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AllocationResult {
    pub total_students: usize,
    pub total_halls: usize,
    pub allocations: Vec<HallAllocationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HallAllocationSummary {
    pub hall_id: String,
    pub hall_number: String,
    pub capacity: i32,
    pub allocated: i32,
    pub utilization_pct: f64,
}

/// Configuration for proctor assignment
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ProctorAssignConfig {
    #[serde(default = "default_2")]
    pub proctors_per_hall: i32,
    #[serde(default)]
    pub prevent_own_subject: bool,
    #[serde(default = "default_5")]
    pub max_sessions_per_teacher: i32,
}

fn default_2() -> i32 { 2 }
fn default_5() -> i32 { 5 }

/// Result of proctor assignment
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ProctorAssignResult {
    pub assigned_count: usize,
    pub halls_covered: usize,
    pub total_halls: usize,
    pub uncovered_halls: Vec<String>,
}

impl super::Service {
    // ── Hall Allocation ─────────────────────────────────────────────────────

    /// Automatically allocate students to exam halls using balanced bin packing.
    pub async fn allocate_students_to_halls(
        &self,
        org_id: &str,
        exam_session_id: &str,
        config: AllocationConfig,
    ) -> Result<AllocationResult, String> {
        let session = self.get_exam_session(org_id, exam_session_id).await?;

        // Get halls
        let halls = self
            .repo
            .get_exam_halls(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        if halls.is_empty() {
            return Err("no exam halls found for this session".to_string());
        }

        // Get ordered students
        let students = self
            .get_ordered_students(org_id, &session.cycle_id, None)
            .await?;

        if students.is_empty() {
            return Err("no students found".to_string());
        }

        let total_capacity: i32 = halls.iter().map(|h| h.capacity).sum();
        if (total_capacity as usize) < students.len() {
            return Err(format!(
                "insufficient capacity: {} seats for {} students",
                total_capacity,
                students.len()
            ));
        }

        // Clear existing allocations
        self.repo
            .delete_hall_allocations(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        // === Balanced Bin Packing Algorithm ===
        // Sort halls by capacity descending
        let mut hall_slots: Vec<(String, String, i32, i32)> = halls
            .iter()
            .map(|h| (h.id.clone(), h.hall_number.clone(), h.capacity, 0i32))
            .collect();
        hall_slots.sort_by(|a, b| b.2.cmp(&a.2));

        let now = Utc::now().to_rfc3339();

        // If balancing, distribute evenly
        if config.balance_halls {
            let halls_count = hall_slots.len();
            let per_hall = students.len() / halls_count;
            let remainder = students.len() % halls_count;

            let mut student_idx = 0;

            for (hall_idx, (hall_id, _hall_num, capacity, count)) in hall_slots.iter_mut().enumerate() {
                let target = per_hall + if hall_idx < remainder { 1 } else { 0 };
                let actual = target.min(*capacity as usize);

                for _ in 0..actual {
                    if student_idx >= students.len() {
                        break;
                    }

                    let student = &students[student_idx];
                    let row = (student_idx % actual) / 5 + 1;
                    let seat = (student_idx % actual) % 5 + 1;
                    let seat_position = format!("R{}-S{}", row, seat);

                    let model = exam_hall_allocation::Model {
                        id: Uuid::new_v4().to_string(),
                        exam_hall_id: hall_id.clone(),
                        exam_session_id: exam_session_id.to_string(),
                        enrollment_id: student.enrollment_id.clone(),
                        seating_number_id: None,
                        seat_position: Some(seat_position),
                        is_present: None,
                        created_at: now.clone(),
                        updated_at: now.clone(),
                    };

                    self.repo
                        .insert_hall_allocation(model)
                        .await
                        .map_err(|e| e.to_string())?;

                    *count += 1;
                    student_idx += 1;
                }
            }

            // Handle overflow: assign remaining students to halls with capacity
            while student_idx < students.len() {
                for (hall_id, _, capacity, count) in hall_slots.iter_mut() {
                    if *count < *capacity && student_idx < students.len() {
                        let student = &students[student_idx];
                        let model = exam_hall_allocation::Model {
                            id: Uuid::new_v4().to_string(),
                            exam_hall_id: hall_id.clone(),
                            exam_session_id: exam_session_id.to_string(),
                            enrollment_id: student.enrollment_id.clone(),
                            seating_number_id: None,
                            seat_position: Some(format!("R{}-S{}", (*count / 5) + 1, (*count % 5) + 1)),
                            is_present: None,
                            created_at: now.clone(),
                            updated_at: now.clone(),
                        };

                        self.repo
                            .insert_hall_allocation(model)
                            .await
                            .map_err(|e| e.to_string())?;

                        *count += 1;
                        student_idx += 1;
                    }
                }
            }
        } else {
            // First Fit Decreasing: fill halls one by one
            let mut student_idx = 0;
            for (hall_id, _, capacity, count) in hall_slots.iter_mut() {
                while *count < *capacity && student_idx < students.len() {
                    let student = &students[student_idx];
                    let model = exam_hall_allocation::Model {
                        id: Uuid::new_v4().to_string(),
                        exam_hall_id: hall_id.clone(),
                        exam_session_id: exam_session_id.to_string(),
                        enrollment_id: student.enrollment_id.clone(),
                        seating_number_id: None,
                        seat_position: Some(format!("R{}-S{}", (*count / 5) + 1, (*count % 5) + 1)),
                        is_present: None,
                        created_at: now.clone(),
                        updated_at: now.clone(),
                    };

                    self.repo
                        .insert_hall_allocation(model)
                        .await
                        .map_err(|e| e.to_string())?;

                    *count += 1;
                    student_idx += 1;
                }
            }
        }

        // Update allocated counts on halls
        for (hall_id, _, _, count) in &hall_slots {
            self.repo
                .update_exam_hall_allocated_count(hall_id, *count)
                .await
                .map_err(|e| e.to_string())?;
        }

        // Build result
        let allocations: Vec<HallAllocationSummary> = hall_slots
            .iter()
            .map(|(id, num, cap, cnt)| HallAllocationSummary {
                hall_id: id.clone(),
                hall_number: num.clone(),
                capacity: *cap,
                allocated: *cnt,
                utilization_pct: if *cap > 0 {
                    (*cnt as f64 / *cap as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();

        Ok(AllocationResult {
            total_students: students.len(),
            total_halls: halls.len(),
            allocations,
        })
    }

    /// Get students allocated to a specific hall
    pub async fn get_hall_students(
        &self,
        org_id: &str,
        exam_session_id: &str,
        hall_id: &str,
    ) -> Result<Vec<exam_hall_allocation::Model>, String> {
        let _ = self.get_exam_session(org_id, exam_session_id).await?;

        self.repo
            .get_hall_allocations(exam_session_id, Some(hall_id))
            .await
            .map_err(|e| e.to_string())
    }

    // ── Proctor Assignment ──────────────────────────────────────────────────

    /// Automatically assign proctors to halls for a specific exam schedule.
    /// Uses a greedy fair-distribution algorithm.
    pub async fn auto_assign_proctors(
        &self,
        org_id: &str,
        exam_session_id: &str,
        exam_schedule_id: &str,
        config: ProctorAssignConfig,
    ) -> Result<ProctorAssignResult, String> {
        let _session = self.get_exam_session(org_id, exam_session_id).await?;

        // Get halls
        let halls = self
            .repo
            .get_exam_halls(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        if halls.is_empty() {
            return Err("no exam halls found".to_string());
        }

        // Get the exam schedule to find the date
        let schedules = self
            .repo
            .get_exam_schedules(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        let schedule = schedules
            .iter()
            .find(|s| s.id == exam_schedule_id)
            .ok_or_else(|| "exam schedule not found".to_string())?;

        // Get available teachers for this date
        let available_teachers = self
            .repo
            .get_available_teachers_for_date(org_id, exam_session_id, &schedule.exam_date)
            .await
            .map_err(|e| e.to_string())?;

        if available_teachers.is_empty() {
            return Err("no available teachers for this date".to_string());
        }

        // Get existing assignments to track load
        let existing_proctors = self
            .repo
            .get_exam_proctors(exam_session_id, None)
            .await
            .map_err(|e| e.to_string())?;

        let mut teacher_load: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for p in &existing_proctors {
            *teacher_load.entry(p.teacher_id.clone()).or_insert(0) += 1;
        }

        // Filter teachers who haven't exceeded max sessions
        let mut eligible: Vec<(String, i32)> = available_teachers
            .into_iter()
            .map(|tid| {
                let load = teacher_load.get(&tid).copied().unwrap_or(0);
                (tid, load)
            })
            .filter(|(_, load)| *load < config.max_sessions_per_teacher)
            .collect();

        // Sort by load ascending (fairness: least loaded first)
        eligible.sort_by_key(|(_, load)| *load);

        let now = Utc::now().to_rfc3339();
        let mut assigned = 0usize;
        let mut halls_covered = 0usize;
        let mut uncovered_halls: Vec<String> = Vec::new();
        let mut teacher_idx = 0;

        for hall in &halls {
            let proctors_needed = config.proctors_per_hall as usize;
            let mut hall_assigned = 0;

            for _ in 0..proctors_needed {
                if teacher_idx >= eligible.len() {
                    break;
                }

                let (ref teacher_id, _) = eligible[teacher_idx];

                let role = if hall_assigned == 0 {
                    "head_proctor"
                } else {
                    "proctor"
                };

                let model = exam_proctor::Model {
                    id: Uuid::new_v4().to_string(),
                    org_id: org_id.to_string(),
                    exam_session_id: exam_session_id.to_string(),
                    exam_schedule_id: exam_schedule_id.to_string(),
                    exam_hall_id: hall.id.clone(),
                    teacher_id: teacher_id.clone(),
                    role: role.to_string(),
                    is_confirmed: 0,
                    confirmed_at: None,
                    attendance_status: None,
                    check_in_time: None,
                    notes: None,
                    created_at: now.clone(),
                    updated_at: now.clone(),
                };

                self.repo
                    .insert_exam_proctor(model)
                    .await
                    .map_err(|e| e.to_string())?;

                teacher_idx += 1;
                hall_assigned += 1;
                assigned += 1;
            }

            if hall_assigned > 0 {
                halls_covered += 1;
            } else {
                uncovered_halls.push(hall.hall_number.clone());
            }
        }

        Ok(ProctorAssignResult {
            assigned_count: assigned,
            halls_covered,
            total_halls: halls.len(),
            uncovered_halls,
        })
    }

    /// Get proctors for an exam session
    pub async fn get_exam_proctors(
        &self,
        org_id: &str,
        exam_session_id: &str,
        schedule_id: Option<&str>,
    ) -> Result<Vec<exam_proctor::Model>, String> {
        let _ = self.get_exam_session(org_id, exam_session_id).await?;

        self.repo
            .get_exam_proctors(exam_session_id, schedule_id)
            .await
            .map_err(|e| e.to_string())
    }
}
