use serde::{Deserialize, Serialize};

/// Comprehensive exam session dashboard data
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ExamDashboard {
    pub session_summary: SessionSummary,
    pub schedule_overview: ScheduleOverview,
    pub hall_statistics: HallStatistics,
    pub proctor_statistics: ProctorStatistics,
    pub student_statistics: StudentStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SessionSummary {
    pub session_id: String,
    pub name: String,
    pub status: String,
    pub session_type: String,
    pub start_date: String,
    pub end_date: String,
    pub total_subjects: usize,
    pub total_students: usize,
    pub total_halls: usize,
    pub total_proctors: usize,
    pub completion_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ScheduleOverview {
    pub total_scheduled: usize,
    pub completed: usize,
    pub pending: usize,
    pub postponed: usize,
    pub cancelled: usize,
    pub exams_by_date: Vec<DateExamCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DateExamCount {
    pub date: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HallStatistics {
    pub total_halls: usize,
    pub total_capacity: i32,
    pub total_allocated: i32,
    pub average_utilization_pct: f64,
    pub halls: Vec<HallUtilization>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HallUtilization {
    pub hall_id: String,
    pub hall_number: String,
    pub capacity: i32,
    pub allocated: i32,
    pub utilization_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ProctorStatistics {
    pub total_assigned: usize,
    pub confirmed: usize,
    pub pending_confirmation: usize,
    pub load_distribution: Vec<ProctorLoad>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ProctorLoad {
    pub teacher_id: String,
    pub sessions_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct StudentStatistics {
    pub total_enrolled: usize,
    pub with_seating_number: usize,
    pub with_secret_number: usize,
    pub allocated_to_halls: usize,
    pub coverage_pct: f64,
}

// ── Report Generation ───────────────────────────────────────────────────────

/// Available report types
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub enum ExamReportType {
    /// Full exam schedule timetable
    ScheduleTimetable,
    /// Seating number list per class
    SeatingNumberList,
    /// Hall allocation map
    HallAllocationMap,
    /// Proctor assignment sheet
    ProctorAssignmentSheet,
    /// Student exam card (individual)
    StudentExamCard,
    /// Exam attendance sheet (per hall per exam)
    AttendanceSheet,
    /// Session summary report
    SessionSummaryReport,
}

/// Report request
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GenerateReportRequest {
    pub exam_session_id: String,
    pub report_type: ExamReportType,
    /// Optional: filter by class_id
    pub class_id: Option<String>,
    /// Optional: filter by hall_id
    pub hall_id: Option<String>,
    /// Optional: filter by schedule_id
    pub schedule_id: Option<String>,
    /// Output format: "json", "csv", "html"
    pub format: Option<String>,
}

/// Generated report data
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ExamReport {
    pub report_type: String,
    pub title: String,
    pub title_ar: String,
    pub generated_at: String,
    pub session_name: String,
    pub format: String,
    /// JSON-encoded report payload (structure varies by report type)
    pub data: serde_json::Value,
    /// Summary statistics
    pub summary: serde_json::Value,
}

// ── Seating Number List Report ──────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatingListRow {
    pub seq: i32,
    pub seating_number: String,
    pub student_name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub class_id: String,
}

// ── Hall Allocation Report ──────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallAllocationRow {
    pub hall_number: String,
    pub seat_position: Option<String>,
    pub enrollment_id: String,
    pub seating_number: Option<String>,
    pub student_name_ar: Option<String>,
}

// ── Proctor Assignment Report ───────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProctorAssignmentRow {
    pub hall_number: String,
    pub teacher_id: String,
    pub teacher_name: Option<String>,
    pub role: String,
    pub exam_date: String,
    pub start_time: String,
}

// ── Attendance Sheet ────────────────────────────────────────────────────────
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceSheetRow {
    pub seq: i32,
    pub seating_number: String,
    pub student_name_ar: Option<String>,
    pub seat_position: Option<String>,
    pub is_present: Option<bool>,
    pub signature: String,
}

impl super::Service {
    /// Generate the comprehensive exam dashboard
    pub async fn get_exam_dashboard(
        &self,
        org_id: &str,
        exam_session_id: &str,
    ) -> Result<ExamDashboard, String> {
        let session = self.get_exam_session(org_id, exam_session_id).await?;

        // Fetch all data
        let subjects = self.repo.get_exam_subjects(exam_session_id).await.map_err(|e| e.to_string())?;
        let schedules = self.repo.get_exam_schedules(exam_session_id).await.map_err(|e| e.to_string())?;
        let halls = self.repo.get_exam_halls(exam_session_id).await.map_err(|e| e.to_string())?;
        let proctors = self.repo.get_exam_proctors(exam_session_id, None).await.map_err(|e| e.to_string())?;
        let seating = self.repo.get_seating_numbers(exam_session_id, None).await.map_err(|e| e.to_string())?;
        let allocations = self.repo.get_hall_allocations(exam_session_id, None).await.map_err(|e| e.to_string())?;

        // Schedule overview
        let completed = schedules.iter().filter(|s| s.status == "completed").count();
        let postponed = schedules.iter().filter(|s| s.status == "postponed").count();
        let cancelled = schedules.iter().filter(|s| s.status == "cancelled").count();
        let pending = schedules.len() - completed - postponed - cancelled;

        // Exams by date
        let mut date_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for s in &schedules {
            if s.status != "cancelled" && s.status != "postponed" {
                *date_counts.entry(s.exam_date.clone()).or_insert(0) += 1;
            }
        }
        let mut exams_by_date: Vec<DateExamCount> = date_counts
            .into_iter()
            .map(|(date, count)| DateExamCount { date, count })
            .collect();
        exams_by_date.sort_by(|a, b| a.date.cmp(&b.date));

        // Hall statistics
        let total_capacity: i32 = halls.iter().map(|h| h.capacity).sum();
        let total_allocated: i32 = halls.iter().map(|h| h.allocated_count).sum();
        let avg_util = if total_capacity > 0 {
            (total_allocated as f64 / total_capacity as f64) * 100.0
        } else {
            0.0
        };
        let hall_utils: Vec<HallUtilization> = halls
            .iter()
            .map(|h| HallUtilization {
                hall_id: h.id.clone(),
                hall_number: h.hall_number.clone(),
                capacity: h.capacity,
                allocated: h.allocated_count,
                utilization_pct: if h.capacity > 0 {
                    (h.allocated_count as f64 / h.capacity as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();

        // Proctor statistics
        let confirmed = proctors.iter().filter(|p| p.is_confirmed == 1).count();
        let mut teacher_loads: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for p in &proctors {
            *teacher_loads.entry(p.teacher_id.clone()).or_insert(0) += 1;
        }
        let load_dist: Vec<ProctorLoad> = teacher_loads
            .into_iter()
            .map(|(tid, cnt)| ProctorLoad {
                teacher_id: tid,
                sessions_count: cnt,
            })
            .collect();

        // Student statistics
        let total_enrolled = seating.len();
        let coverage = if total_enrolled > 0 {
            (allocations.len() as f64 / total_enrolled as f64) * 100.0
        } else {
            0.0
        };

        let completion = if schedules.is_empty() {
            0.0
        } else {
            (completed as f64 / schedules.len() as f64) * 100.0
        };

        Ok(ExamDashboard {
            session_summary: SessionSummary {
                session_id: session.id,
                name: session.name,
                status: session.status,
                session_type: session.session_type,
                start_date: session.start_date,
                end_date: session.end_date,
                total_subjects: subjects.len(),
                total_students: total_enrolled,
                total_halls: halls.len(),
                total_proctors: proctors.len(),
                completion_pct: completion,
            },
            schedule_overview: ScheduleOverview {
                total_scheduled: schedules.len(),
                completed,
                pending,
                postponed,
                cancelled,
                exams_by_date,
            },
            hall_statistics: HallStatistics {
                total_halls: halls.len(),
                total_capacity,
                total_allocated,
                average_utilization_pct: avg_util,
                halls: hall_utils,
            },
            proctor_statistics: ProctorStatistics {
                total_assigned: proctors.len(),
                confirmed,
                pending_confirmation: proctors.len() - confirmed,
                load_distribution: load_dist,
            },
            student_statistics: StudentStatistics {
                total_enrolled,
                with_seating_number: seating.iter().filter(|s| s.is_active == 1).count(),
                with_secret_number: 0, // Will be populated when we query secrets
                allocated_to_halls: allocations.len(),
                coverage_pct: coverage,
            },
        })
    }

    /// Generate an exam report in the requested format
    pub async fn generate_exam_report(
        &self,
        org_id: &str,
        req: GenerateReportRequest,
    ) -> Result<ExamReport, String> {
        let session = self.get_exam_session(org_id, &req.exam_session_id).await?;
        let format = req.format.as_deref().unwrap_or("json");
        let now = chrono::Utc::now().to_rfc3339();

        match req.report_type {
            ExamReportType::ScheduleTimetable => {
                let schedules = self
                    .repo
                    .get_exam_schedules(&req.exam_session_id)
                    .await
                    .map_err(|e| e.to_string())?;

                let subjects = self
                    .repo
                    .get_exam_subjects(&req.exam_session_id)
                    .await
                    .map_err(|e| e.to_string())?;

                // Enrich schedule entries with subject names
                let entries: Vec<serde_json::Value> = schedules
                    .iter()
                    .filter(|s| s.status != "cancelled")
                    .map(|s| {
                        let subj = subjects.iter().find(|sub| sub.id == s.exam_subject_id);
                        serde_json::json!({
                            "date": s.exam_date,
                            "start_time": s.start_time,
                            "end_time": s.end_time,
                            "period": s.period,
                            "subject_id": s.exam_subject_id,
                            "subject_name": subj.map(|su| &su.subject_id),
                            "duration_minutes": subj.map(|su| su.duration_minutes),
                            "status": s.status,
                        })
                    })
                    .collect();

                Ok(ExamReport {
                    report_type: "schedule_timetable".to_string(),
                    title: "Exam Schedule Timetable".to_string(),
                    title_ar: "جدول الامتحانات".to_string(),
                    generated_at: now,
                    session_name: session.name,
                    format: format.to_string(),
                    data: serde_json::json!(entries),
                    summary: serde_json::json!({
                        "total_exams": schedules.len(),
                        "exam_days": schedules.iter().map(|s| &s.exam_date).collect::<std::collections::HashSet<_>>().len(),
                    }),
                })
            }

            ExamReportType::SeatingNumberList => {
                let class_id = req.class_id.as_deref();
                let seating = self
                    .repo
                    .get_seating_numbers(&req.exam_session_id, class_id)
                    .await
                    .map_err(|e| e.to_string())?;

                let rows: Vec<SeatingListRow> = seating
                    .iter()
                    .map(|s| SeatingListRow {
                        seq: s.sequence_number,
                        seating_number: s.seating_number.clone(),
                        student_name_ar: s.student_name_ar.clone(),
                        level: s.level.clone(),
                        division: s.division.clone(),
                        class_id: s.class_id.clone(),
                    })
                    .collect();

                Ok(ExamReport {
                    report_type: "seating_number_list".to_string(),
                    title: "Seating Number List".to_string(),
                    title_ar: "كشف أرقام الجلوس".to_string(),
                    generated_at: now,
                    session_name: session.name,
                    format: format.to_string(),
                    data: serde_json::to_value(&rows).unwrap_or_default(),
                    summary: serde_json::json!({
                        "total_students": rows.len(),
                        "class_filter": class_id,
                    }),
                })
            }

            ExamReportType::HallAllocationMap => {
                let halls = self
                    .repo
                    .get_exam_halls(&req.exam_session_id)
                    .await
                    .map_err(|e| e.to_string())?;

                let mut hall_data: Vec<serde_json::Value> = Vec::new();

                for hall in &halls {
                    let allocs = self
                        .repo
                        .get_hall_allocations(&req.exam_session_id, Some(&hall.id))
                        .await
                        .map_err(|e| e.to_string())?;

                    hall_data.push(serde_json::json!({
                        "hall_number": hall.hall_number,
                        "capacity": hall.capacity,
                        "allocated": allocs.len(),
                        "building": hall.building,
                        "floor": hall.floor_number,
                        "students": allocs.iter().map(|a| serde_json::json!({
                            "enrollment_id": a.enrollment_id,
                            "seat_position": a.seat_position,
                        })).collect::<Vec<_>>(),
                    }));
                }

                Ok(ExamReport {
                    report_type: "hall_allocation_map".to_string(),
                    title: "Hall Allocation Map".to_string(),
                    title_ar: "خريطة توزيع اللجان".to_string(),
                    generated_at: now,
                    session_name: session.name,
                    format: format.to_string(),
                    data: serde_json::json!(hall_data),
                    summary: serde_json::json!({
                        "total_halls": halls.len(),
                        "total_capacity": halls.iter().map(|h| h.capacity).sum::<i32>(),
                    }),
                })
            }

            ExamReportType::ProctorAssignmentSheet => {
                let schedule_id = req.schedule_id.as_deref();
                let proctors = self
                    .repo
                    .get_exam_proctors(&req.exam_session_id, schedule_id)
                    .await
                    .map_err(|e| e.to_string())?;

                let rows: Vec<serde_json::Value> = proctors
                    .iter()
                    .map(|p| {
                        serde_json::json!({
                            "teacher_id": p.teacher_id,
                            "hall_id": p.exam_hall_id,
                            "role": p.role,
                            "is_confirmed": p.is_confirmed == 1,
                            "schedule_id": p.exam_schedule_id,
                        })
                    })
                    .collect();

                Ok(ExamReport {
                    report_type: "proctor_assignment_sheet".to_string(),
                    title: "Proctor Assignment Sheet".to_string(),
                    title_ar: "كشف توزيع المراقبين".to_string(),
                    generated_at: now,
                    session_name: session.name,
                    format: format.to_string(),
                    data: serde_json::json!(rows),
                    summary: serde_json::json!({
                        "total_proctors": proctors.len(),
                        "confirmed": proctors.iter().filter(|p| p.is_confirmed == 1).count(),
                    }),
                })
            }

            ExamReportType::AttendanceSheet => {
                let hall_id = req.hall_id.ok_or("hall_id is required for attendance sheet")?;
                let allocs = self
                    .repo
                    .get_hall_allocations(&req.exam_session_id, Some(&hall_id))
                    .await
                    .map_err(|e| e.to_string())?;

                let rows: Vec<AttendanceSheetRow> = allocs
                    .iter()
                    .enumerate()
                    .map(|(i, a)| AttendanceSheetRow {
                        seq: (i + 1) as i32,
                        seating_number: a.seating_number_id.clone().unwrap_or_default(),
                        student_name_ar: None,
                        seat_position: a.seat_position.clone(),
                        is_present: a.is_present.map(|v| v == 1),
                        signature: "____________".to_string(),
                    })
                    .collect();

                Ok(ExamReport {
                    report_type: "attendance_sheet".to_string(),
                    title: "Exam Attendance Sheet".to_string(),
                    title_ar: "كشف حضور الامتحان".to_string(),
                    generated_at: now,
                    session_name: session.name,
                    format: format.to_string(),
                    data: serde_json::to_value(&rows).unwrap_or_default(),
                    summary: serde_json::json!({
                        "hall_id": hall_id,
                        "total_students": rows.len(),
                    }),
                })
            }

            ExamReportType::SessionSummaryReport => {
                let dashboard = self.get_exam_dashboard(org_id, &req.exam_session_id).await?;

                Ok(ExamReport {
                    report_type: "session_summary".to_string(),
                    title: "Exam Session Summary Report".to_string(),
                    title_ar: "تقرير ملخص الدورة الامتحانية".to_string(),
                    generated_at: now,
                    session_name: session.name,
                    format: format.to_string(),
                    data: serde_json::to_value(&dashboard).unwrap_or_default(),
                    summary: serde_json::json!({
                        "completion": dashboard.session_summary.completion_pct,
                        "status": dashboard.session_summary.status,
                    }),
                })
            }

            ExamReportType::StudentExamCard => {
                // Individual student exam card — needs enrollment_id via class_id filter
                let seating = self
                    .repo
                    .get_seating_numbers(&req.exam_session_id, req.class_id.as_deref())
                    .await
                    .map_err(|e| e.to_string())?;

                let schedules = self
                    .repo
                    .get_exam_schedules(&req.exam_session_id)
                    .await
                    .map_err(|e| e.to_string())?;

                let cards: Vec<serde_json::Value> = seating
                    .iter()
                    .map(|s| {
                        serde_json::json!({
                            "student_name_ar": s.student_name_ar,
                            "seating_number": s.seating_number,
                            "level": s.level,
                            "division": s.division,
                            "session_name": session.name,
                            "session_type": session.session_type,
                            "exams": schedules.iter()
                                .filter(|sch| sch.status == "scheduled")
                                .map(|sch| serde_json::json!({
                                    "date": sch.exam_date,
                                    "start_time": sch.start_time,
                                    "end_time": sch.end_time,
                                    "subject_id": sch.exam_subject_id,
                                }))
                                .collect::<Vec<_>>(),
                        })
                    })
                    .collect();

                Ok(ExamReport {
                    report_type: "student_exam_card".to_string(),
                    title: "Student Exam Cards".to_string(),
                    title_ar: "بطاقات الامتحان للطلاب".to_string(),
                    generated_at: now,
                    session_name: session.name.clone(),
                    format: format.to_string(),
                    data: serde_json::json!(cards),
                    summary: serde_json::json!({
                        "total_cards": cards.len(),
                    }),
                })
            }
        }
    }
}
