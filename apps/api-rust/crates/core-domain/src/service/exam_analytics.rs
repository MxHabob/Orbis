use serde::{Deserialize, Serialize};

/// Exam analytics request
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ExamAnalyticsRequest {
    pub exam_session_id: String,
    /// "readiness" | "workload" | "capacity" | "timeline" | "full"
    pub analysis_type: String,
}

/// Full analytics response
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ExamAnalytics {
    pub readiness: ReadinessAnalysis,
    pub workload: WorkloadAnalysis,
    pub capacity: CapacityAnalysis,
    pub timeline: TimelineAnalysis,
    pub recommendations: Vec<Recommendation>,
}

/// How ready is this exam session to go live?
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ReadinessAnalysis {
    pub overall_score: f64,
    pub checklist: Vec<ReadinessItem>,
    pub blocking_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ReadinessItem {
    pub item: String,
    pub item_ar: String,
    pub is_complete: bool,
    pub severity: String,
    pub details: Option<String>,
}

/// Teacher workload distribution analysis
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct WorkloadAnalysis {
    pub total_teachers_assigned: usize,
    pub average_sessions_per_teacher: f64,
    pub max_sessions: usize,
    pub min_sessions: usize,
    pub overloaded_teachers: Vec<String>,
    pub unassigned_teachers: usize,
    pub fairness_index: f64,
}

/// Hall capacity planning analysis
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CapacityAnalysis {
    pub total_capacity: i32,
    pub total_demand: i32,
    pub surplus_or_deficit: i32,
    pub utilization_pct: f64,
    pub overflowing_halls: Vec<String>,
    pub underutilized_halls: Vec<String>,
}

/// Timeline analysis for scheduling density
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TimelineAnalysis {
    pub total_exam_days: usize,
    pub exams_per_day_avg: f64,
    pub busiest_day: Option<String>,
    pub busiest_day_count: usize,
    pub lightest_day: Option<String>,
    pub lightest_day_count: usize,
    pub gap_days: Vec<String>,
}

/// AI-style recommendation
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Recommendation {
    pub priority: String,
    pub category: String,
    pub message: String,
    pub message_ar: String,
    pub action: Option<String>,
}

impl super::Service {
    /// Run comprehensive analytics on an exam session
    pub async fn analyze_exam_session(
        &self,
        org_id: &str,
        req: ExamAnalyticsRequest,
    ) -> Result<ExamAnalytics, String> {
        let session = self.get_exam_session(org_id, &req.exam_session_id).await?;
        let session_id = &req.exam_session_id;

        let subjects = self.repo.get_exam_subjects(session_id).await.map_err(|e| e.to_string())?;
        let schedules = self.repo.get_exam_schedules(session_id).await.map_err(|e| e.to_string())?;
        let halls = self.repo.get_exam_halls(session_id).await.map_err(|e| e.to_string())?;
        let proctors = self.repo.get_exam_proctors(session_id, None).await.map_err(|e| e.to_string())?;
        let seating = self.repo.get_seating_numbers(session_id, None).await.map_err(|e| e.to_string())?;
        let allocations = self.repo.get_hall_allocations(session_id, None).await.map_err(|e| e.to_string())?;

        // ── Readiness Analysis ──────────────────────────────────────────────
        let mut checklist = Vec::new();
        let mut blocking = Vec::new();

        // Subjects defined?
        checklist.push(ReadinessItem {
            item: "Exam subjects defined".to_string(),
            item_ar: "تحديد مواد الامتحان".to_string(),
            is_complete: !subjects.is_empty(),
            severity: "critical".to_string(),
            details: Some(format!("{} subjects", subjects.len())),
        });
        if subjects.is_empty() {
            blocking.push("No exam subjects defined".to_string());
        }

        // Schedule created?
        let all_scheduled = subjects.len() == schedules.iter().filter(|s| s.status == "scheduled").count();
        checklist.push(ReadinessItem {
            item: "All subjects scheduled".to_string(),
            item_ar: "جدولة جميع المواد".to_string(),
            is_complete: all_scheduled && !subjects.is_empty(),
            severity: "critical".to_string(),
            details: Some(format!("{}/{} scheduled", schedules.len(), subjects.len())),
        });
        if !all_scheduled && !subjects.is_empty() {
            blocking.push(format!(
                "Only {}/{} subjects have been scheduled",
                schedules.len(),
                subjects.len()
            ));
        }

        // Seating numbers generated?
        checklist.push(ReadinessItem {
            item: "Seating numbers generated".to_string(),
            item_ar: "إصدار أرقام الجلوس".to_string(),
            is_complete: !seating.is_empty(),
            severity: "critical".to_string(),
            details: Some(format!("{} numbers", seating.len())),
        });
        if seating.is_empty() {
            blocking.push("Seating numbers not generated".to_string());
        }

        // Halls configured?
        checklist.push(ReadinessItem {
            item: "Exam halls configured".to_string(),
            item_ar: "تجهيز اللجان".to_string(),
            is_complete: !halls.is_empty(),
            severity: "high".to_string(),
            details: Some(format!("{} halls", halls.len())),
        });

        // Students allocated to halls?
        checklist.push(ReadinessItem {
            item: "Students allocated to halls".to_string(),
            item_ar: "توزيع الطلاب على اللجان".to_string(),
            is_complete: !allocations.is_empty(),
            severity: "high".to_string(),
            details: Some(format!("{} allocations", allocations.len())),
        });

        // Proctors assigned?
        checklist.push(ReadinessItem {
            item: "Proctors assigned".to_string(),
            item_ar: "تعيين المراقبين".to_string(),
            is_complete: !proctors.is_empty(),
            severity: "medium".to_string(),
            details: Some(format!("{} proctors", proctors.len())),
        });

        let completed_count = checklist.iter().filter(|c| c.is_complete).count();
        let overall_score = if checklist.is_empty() {
            0.0
        } else {
            (completed_count as f64 / checklist.len() as f64) * 100.0
        };

        let readiness = ReadinessAnalysis {
            overall_score,
            checklist,
            blocking_issues: blocking,
        };

        // ── Workload Analysis ───────────────────────────────────────────────
        let mut teacher_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for p in &proctors {
            *teacher_counts.entry(p.teacher_id.clone()).or_insert(0) += 1;
        }

        let counts: Vec<usize> = teacher_counts.values().copied().collect();
        let max_sessions = counts.iter().max().copied().unwrap_or(0);
        let min_sessions = counts.iter().min().copied().unwrap_or(0);
        let avg = if counts.is_empty() {
            0.0
        } else {
            counts.iter().sum::<usize>() as f64 / counts.len() as f64
        };

        // Fairness index (1.0 = perfectly fair)
        let fairness = if max_sessions > 0 {
            min_sessions as f64 / max_sessions as f64
        } else {
            1.0
        };

        let overloaded: Vec<String> = teacher_counts
            .iter()
            .filter(|(_, &c)| c as f64 > avg * 1.5)
            .map(|(id, _)| id.clone())
            .collect();

        let workload = WorkloadAnalysis {
            total_teachers_assigned: teacher_counts.len(),
            average_sessions_per_teacher: avg,
            max_sessions,
            min_sessions,
            overloaded_teachers: overloaded,
            unassigned_teachers: 0,
            fairness_index: fairness,
        };

        // ── Capacity Analysis ───────────────────────────────────────────────
        let total_capacity: i32 = halls.iter().map(|h| h.capacity).sum();
        let total_demand = seating.len() as i32;
        let surplus = total_capacity - total_demand;
        let util = if total_capacity > 0 {
            (total_demand as f64 / total_capacity as f64) * 100.0
        } else {
            0.0
        };

        let overflowing: Vec<String> = halls
            .iter()
            .filter(|h| h.allocated_count > h.capacity)
            .map(|h| h.hall_number.clone())
            .collect();

        let underutilized: Vec<String> = halls
            .iter()
            .filter(|h| h.capacity > 0 && (h.allocated_count as f64 / h.capacity as f64) < 0.5)
            .map(|h| h.hall_number.clone())
            .collect();

        let capacity = CapacityAnalysis {
            total_capacity,
            total_demand,
            surplus_or_deficit: surplus,
            utilization_pct: util,
            overflowing_halls: overflowing,
            underutilized_halls: underutilized,
        };

        // ── Timeline Analysis ───────────────────────────────────────────────
        let mut date_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for s in &schedules {
            if s.status != "cancelled" && s.status != "postponed" {
                *date_counts.entry(s.exam_date.clone()).or_insert(0) += 1;
            }
        }

        let exam_days = date_counts.len();
        let avg_per_day = if exam_days > 0 {
            date_counts.values().sum::<usize>() as f64 / exam_days as f64
        } else {
            0.0
        };

        let busiest = date_counts.iter().max_by_key(|(_, c)| *c);
        let lightest = date_counts.iter().min_by_key(|(_, c)| *c);

        // Find gap days (dates between start and end with no exams)
        let mut all_dates: Vec<String> = date_counts.keys().cloned().collect();
        all_dates.sort();

        let gap_days: Vec<String> = if all_dates.len() > 1 {
            // Simple gap detection between consecutive scheduled dates
            let mut gaps = Vec::new();
            for i in 0..all_dates.len() - 1 {
                // If dates are non-consecutive, there's a gap
                if all_dates[i + 1] > all_dates[i] {
                    // This is a simplified check — real gap detection would parse dates
                    gaps.push(format!("gap between {} and {}", all_dates[i], all_dates[i + 1]));
                }
            }
            gaps
        } else {
            Vec::new()
        };

        let timeline = TimelineAnalysis {
            total_exam_days: exam_days,
            exams_per_day_avg: avg_per_day,
            busiest_day: busiest.map(|(d, _)| d.clone()),
            busiest_day_count: busiest.map(|(_, c)| *c).unwrap_or(0),
            lightest_day: lightest.map(|(d, _)| d.clone()),
            lightest_day_count: lightest.map(|(_, c)| *c).unwrap_or(0),
            gap_days,
        };

        // ── Recommendations Engine ──────────────────────────────────────────
        let mut recommendations = Vec::new();

        if readiness.overall_score < 50.0 {
            recommendations.push(Recommendation {
                priority: "critical".to_string(),
                category: "readiness".to_string(),
                message: format!(
                    "Session readiness is only {:.0}%. Complete all critical items before activating.",
                    readiness.overall_score
                ),
                message_ar: format!(
                    "جاهزية الدورة {:.0}% فقط. أكمل جميع العناصر الحرجة قبل التفعيل.",
                    readiness.overall_score
                ),
                action: None,
            });
        }

        if capacity.surplus_or_deficit < 0 {
            recommendations.push(Recommendation {
                priority: "critical".to_string(),
                category: "capacity".to_string(),
                message: format!(
                    "Capacity deficit: {} more seats needed. Add more halls or increase capacity.",
                    -capacity.surplus_or_deficit
                ),
                message_ar: format!(
                    "عجز في السعة: يلزم {} مقعد إضافي. أضف لجان أو زد السعة.",
                    -capacity.surplus_or_deficit
                ),
                action: Some("add_halls".to_string()),
            });
        }

        if workload.fairness_index < 0.5 {
            recommendations.push(Recommendation {
                priority: "high".to_string(),
                category: "workload".to_string(),
                message: format!(
                    "Teacher workload is unbalanced (fairness: {:.0}%). Redistribute proctor assignments.",
                    workload.fairness_index * 100.0
                ),
                message_ar: format!(
                    "توزيع عبء العمل غير متوازن (العدالة: {:.0}%). أعد توزيع المراقبين.",
                    workload.fairness_index * 100.0
                ),
                action: Some("reassign_proctors".to_string()),
            });
        }

        if !capacity.underutilized_halls.is_empty() {
            recommendations.push(Recommendation {
                priority: "medium".to_string(),
                category: "capacity".to_string(),
                message: format!(
                    "{} halls are underutilized (<50%). Consider consolidating.",
                    capacity.underutilized_halls.len()
                ),
                message_ar: format!(
                    "{} لجان مستخدمة بأقل من 50%. فكّر في الدمج.",
                    capacity.underutilized_halls.len()
                ),
                action: Some("reallocate_students".to_string()),
            });
        }

        if session.status == "draft" && readiness.overall_score >= 80.0 {
            recommendations.push(Recommendation {
                priority: "info".to_string(),
                category: "status".to_string(),
                message: "Session is ready to be scheduled. Consider changing status to 'scheduled'.".to_string(),
                message_ar: "الدورة جاهزة للجدولة. فكّر في تغيير الحالة إلى 'مجدولة'.".to_string(),
                action: Some("schedule_session".to_string()),
            });
        }

        Ok(ExamAnalytics {
            readiness,
            workload,
            capacity,
            timeline,
            recommendations,
        })
    }
}
