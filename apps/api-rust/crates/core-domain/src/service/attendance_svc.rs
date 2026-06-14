use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::entities::attendance;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateAttendanceRequest {
    pub enrollment_id: String,
    pub class_id: String,
    pub date: String,
    pub status: String,
    pub check_in_time: Option<String>,
    pub check_out_time: Option<String>,
    pub period: Option<String>,
    pub subject_name: Option<String>,
    pub notes: Option<String>,
    pub excuse_type: Option<String>,
    pub is_excused: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkAttendanceRequest {
    pub class_id: String,
    pub date: String,
    pub period: Option<String>,
    pub subject_name: Option<String>,
    pub records: Vec<BulkAttendanceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkAttendanceRecord {
    pub enrollment_id: String,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AttendanceReport {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub total_days: u64,
    pub present_days: u64,
    pub absent_days: u64,
    pub late_days: u64,
    pub excused_days: u64,
    pub sick_days: u64,
    pub attendance_rate: f64,
    pub is_at_risk: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AttendanceAnalytics {
    pub total_records: usize,
    pub present_count: usize,
    pub absent_count: usize,
    pub late_count: usize,
    pub excused_count: usize,
    pub sick_count: usize,
    pub overall_rate: f64,
    pub at_risk_count: usize,
}

const ABSENCE_THRESHOLD: u64 = 5;

impl super::Service {
    pub async fn record_attendance(
        &self,
        org_id: &str,
        recorded_by: &str,
        req: CreateAttendanceRequest,
    ) -> Result<attendance::Model, String> {
        // Validate status
        let valid_statuses = ["present", "absent", "late", "excused", "sick"];
        if !valid_statuses.contains(&req.status.as_str()) {
            return Err(format!("invalid status: {}. Valid: {:?}", req.status, valid_statuses));
        }

        // Get enrollment to find cycle
        let enrollment = self.repo.get_enrollment_by_id(&req.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "enrollment not found".to_string())?;

        let record = attendance::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: enrollment.cycle_id.clone(),
            enrollment_id: req.enrollment_id.clone(),
            class_id: req.class_id,
            date: req.date,
            status: req.status.clone(),
            check_in_time: req.check_in_time,
            check_out_time: req.check_out_time,
            period: req.period,
            subject_name: req.subject_name,
            recorded_by: recorded_by.to_string(),
            notes: req.notes,
            excuse_type: req.excuse_type,
            excuse_document: None,
            is_excused: req.is_excused.unwrap_or(0),
            created_at: Utc::now().to_rfc3339(),
        };

        let created = self.repo.create_attendance(record).await
            .map_err(|e| e.to_string())?;

        // Check absence threshold and auto-notify
        if req.status == "absent" {
            let absent_count = self.repo.get_attendance_count_by_status(&req.enrollment_id, "absent").await
                .map_err(|e| e.to_string())?;

            if absent_count >= ABSENCE_THRESHOLD {
                let _ = self.send_notification(
                    &enrollment.student_id,
                    org_id,
                    "Absence Alert",
                    "تنبيه غياب",
                    &format!("Student has reached {} absences this semester. Immediate attention required.", absent_count),
                    &format!("وصل عدد حالات الغياب إلى {} هذا الفصل. يرجى الانتباه الفوري.", absent_count),
                    "high",
                    "attendance",
                ).await;
            }
        }

        Ok(created)
    }

    pub async fn record_bulk_attendance(
        &self,
        org_id: &str,
        recorded_by: &str,
        req: BulkAttendanceRequest,
    ) -> Result<Vec<attendance::Model>, String> {
        let mut results = Vec::new();
        for record in req.records {
            let att_req = CreateAttendanceRequest {
                enrollment_id: record.enrollment_id,
                class_id: req.class_id.clone(),
                date: req.date.clone(),
                status: record.status,
                check_in_time: None,
                check_out_time: None,
                period: req.period.clone(),
                subject_name: req.subject_name.clone(),
                notes: record.notes,
                excuse_type: None,
                is_excused: Some(0),
            };
            let result = self.record_attendance(org_id, recorded_by, att_req).await?;
            results.push(result);
        }
        Ok(results)
    }

    pub async fn get_attendance(
        &self,
        org_id: &str,
        params: crate::params::AttendanceParams,
    ) -> Result<Vec<attendance::Model>, String> {
        self.repo.get_attendance(org_id, params).await
            .map_err(|e| e.to_string())
    }

    /// Generate attendance report for a single student enrollment
    /// Attendance Rate = (present + late) / total_days × 100
    pub async fn get_attendance_report(&self, enrollment_id: &str) -> Result<AttendanceReport, String> {
        let enrollment = self.repo.get_enrollment_by_id(enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "enrollment not found".to_string())?;

        let student = self.repo.get_user_by_id(&enrollment.student_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "student not found".to_string())?;

        let total_days = self.repo.get_attendance_total(enrollment_id).await
            .map_err(|e| e.to_string())?;
        let present_days = self.repo.get_attendance_count_by_status(enrollment_id, "present").await
            .map_err(|e| e.to_string())?;
        let absent_days = self.repo.get_attendance_count_by_status(enrollment_id, "absent").await
            .map_err(|e| e.to_string())?;
        let late_days = self.repo.get_attendance_count_by_status(enrollment_id, "late").await
            .map_err(|e| e.to_string())?;
        let excused_days = self.repo.get_attendance_count_by_status(enrollment_id, "excused").await
            .map_err(|e| e.to_string())?;
        let sick_days = self.repo.get_attendance_count_by_status(enrollment_id, "sick").await
            .map_err(|e| e.to_string())?;

        let attendance_rate = if total_days > 0 {
            ((present_days + late_days) as f64 / total_days as f64) * 100.0
        } else {
            100.0
        };

        let is_at_risk = absent_days >= ABSENCE_THRESHOLD || attendance_rate < 75.0;

        Ok(AttendanceReport {
            enrollment_id: enrollment_id.to_string(),
            student_id: enrollment.student_id,
            student_name: student.full_name,
            total_days,
            present_days,
            absent_days,
            late_days,
            excused_days,
            sick_days,
            attendance_rate: (attendance_rate * 100.0).round() / 100.0,
            is_at_risk,
        })
    }

    /// School-wide attendance analytics
    pub async fn get_attendance_analytics(&self, org_id: &str, params: crate::params::AttendanceParams) -> Result<AttendanceAnalytics, String> {
        let records = self.repo.get_attendance(org_id, params).await
            .map_err(|e| e.to_string())?;

        let total = records.len();
        let present_count = records.iter().filter(|r| r.status == "present").count();
        let absent_count = records.iter().filter(|r| r.status == "absent").count();
        let late_count = records.iter().filter(|r| r.status == "late").count();
        let excused_count = records.iter().filter(|r| r.status == "excused").count();
        let sick_count = records.iter().filter(|r| r.status == "sick").count();

        let overall_rate = if total > 0 {
            ((present_count + late_count) as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        // Count at-risk students (unique enrollments with > threshold absences)
        let mut enrollment_absences: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for r in &records {
            if r.status == "absent" {
                *enrollment_absences.entry(r.enrollment_id.clone()).or_default() += 1;
            }
        }
        let at_risk_count = enrollment_absences.values().filter(|&&c| c >= ABSENCE_THRESHOLD as usize).count();

        Ok(AttendanceAnalytics {
            total_records: total,
            present_count,
            absent_count,
            late_count,
            excused_count,
            sick_count,
            overall_rate: (overall_rate * 100.0).round() / 100.0,
            at_risk_count,
        })
    }
}

