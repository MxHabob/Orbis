//! Exam logistics service — manages automated student seating assignment and proctor allocations.

use std::sync::Arc;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_kernel::errors::AppError;
use crate::entities::{exam_hall_allocation, exam_proctor};
use crate::params::{ExamHallParams, ExamScheduleParams, ExamProctorParams};
use crate::repository::{ExamCommandRepo, ExamQueryRepo};
use crate::services::exam_session::ExamSessionService;
use crate::services::student_ordering::StudentOrderingService;

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

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ProctorAssignResult {
    pub assigned_count: usize,
    pub halls_covered: usize,
    pub total_halls: usize,
    pub uncovered_halls: Vec<String>,
}

pub struct ExamLogisticsService {
    pub cmd: Arc<ExamCommandRepo>,
    pub query: Arc<ExamQueryRepo>,
    pub session_service: Arc<ExamSessionService>,
    pub ordering_service: Arc<StudentOrderingService>,
}

impl ExamLogisticsService {
    pub fn new(
        cmd: Arc<ExamCommandRepo>,
        query: Arc<ExamQueryRepo>,
        session_service: Arc<ExamSessionService>,
        ordering_service: Arc<StudentOrderingService>,
    ) -> Self {
        Self {
            cmd,
            query,
            session_service,
            ordering_service,
        }
    }

    /// Automatically allocate students to exam halls using balanced bin packing.
    pub async fn allocate_students_to_halls(
        &self,
        org_id: &str,
        exam_session_id: &str,
        config: AllocationConfig,
    ) -> Result<AllocationResult, AppError> {
        let session = self.session_service.get_exam_session(org_id, exam_session_id).await?;

        let params = ExamHallParams { exam_session_id: Some(exam_session_id.to_string()), room_id: None };
        let halls = self.query.get_exam_halls(exam_session_id, params).await?;
        if halls.is_empty() {
            return Err(AppError::validation("no exam halls found for this session"));
        }

        let students = self.ordering_service.get_ordered_students(org_id, &session.cycle_id, None).await?;
        if students.is_empty() {
            return Err(AppError::validation("no students found. Please rebuild student ordering first."));
        }

        let total_capacity: i32 = halls.iter().map(|h| h.capacity).sum();
        if (total_capacity as usize) < students.len() {
            return Err(AppError::validation(format!(
                "insufficient capacity: {} seats for {} students",
                total_capacity,
                students.len()
            )));
        }

        self.cmd.delete_hall_allocations(exam_session_id).await?;

        // Balanced Bin Packing
        let mut hall_slots: Vec<(String, String, i32, i32)> = halls
            .iter()
            .map(|h| (h.id.clone(), h.hall_number.clone(), h.capacity, 0i32))
            .collect();
        hall_slots.sort_by(|a, b| b.2.cmp(&a.2));

        let now = Utc::now().to_rfc3339();

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

                    self.cmd.insert_hall_allocation(model).await?;
                    *count += 1;
                    student_idx += 1;
                }
            }

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

                        self.cmd.insert_hall_allocation(model).await?;
                        *count += 1;
                        student_idx += 1;
                    }
                }
            }
        } else {
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

                    self.cmd.insert_hall_allocation(model).await?;
                    *count += 1;
                    student_idx += 1;
                }
            }
        }

        for (hall_id, _, _, count) in &hall_slots {
            self.cmd.update_exam_hall_allocated_count(hall_id, *count).await?;
        }

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

    /// Retrieve students allocated to a hall.
    pub async fn get_hall_students(
        &self,
        org_id: &str,
        exam_session_id: &str,
        hall_id: &str,
    ) -> Result<Vec<exam_hall_allocation::Model>, AppError> {
        let _ = self.session_service.get_exam_session(org_id, exam_session_id).await?;
        Ok(self.query.get_hall_allocations(exam_session_id, Some(hall_id)).await?)
    }

    /// Automatically assign available proctors to exam halls.
    pub async fn auto_assign_proctors(
        &self,
        org_id: &str,
        exam_session_id: &str,
        exam_schedule_id: &str,
        config: ProctorAssignConfig,
    ) -> Result<ProctorAssignResult, AppError> {
        let _session = self.session_service.get_exam_session(org_id, exam_session_id).await?;

        let params = ExamHallParams { exam_session_id: Some(exam_session_id.to_string()), room_id: None };
        let halls = self.query.get_exam_halls(exam_session_id, params).await?;
        if halls.is_empty() {
            return Err(AppError::validation("no exam halls found"));
        }

        let sched_params = ExamScheduleParams { exam_session_id: Some(exam_session_id.to_string()), exam_date: None, status: None };
        let schedules = self.query.get_exam_schedules(exam_session_id, sched_params).await?;
        let schedule = schedules
            .iter()
            .find(|s| s.id == exam_schedule_id)
            .ok_or_else(|| AppError::not_found("exam_schedule", exam_schedule_id))?;

        let available_teachers = self.query.get_available_teachers_for_date(org_id, exam_session_id, &schedule.exam_date).await?;
        if available_teachers.is_empty() {
            return Err(AppError::validation("no available teachers for this date"));
        }

        let proc_params = ExamProctorParams { exam_session_id: Some(exam_session_id.to_string()), exam_schedule_id: None, teacher_id: None };
        let existing_proctors = self.query.get_exam_proctors(exam_session_id, proc_params).await?;

        let mut teacher_load: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for p in &existing_proctors {
            *teacher_load.entry(p.teacher_id.clone()).or_insert(0) += 1;
        }

        let mut eligible: Vec<(String, i32)> = available_teachers
            .into_iter()
            .map(|tid| {
                let load = teacher_load.get(&tid).copied().unwrap_or(0);
                (tid, load)
            })
            .filter(|(_, load)| *load < config.max_sessions_per_teacher)
            .collect();

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
                let role = if hall_assigned == 0 { "head_proctor" } else { "proctor" };

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

                self.cmd.insert_exam_proctor(model).await?;
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

    /// Retrieve proctors for a session.
    pub async fn get_exam_proctors(
        &self,
        org_id: &str,
        exam_session_id: &str,
        schedule_id: Option<&str>,
    ) -> Result<Vec<exam_proctor::Model>, AppError> {
        let _ = self.session_service.get_exam_session(org_id, exam_session_id).await?;
        let params = ExamProctorParams { exam_session_id: Some(exam_session_id.to_string()), exam_schedule_id: schedule_id.map(String::from), teacher_id: None };
        Ok(self.query.get_exam_proctors(exam_session_id, params).await?)
    }
}
