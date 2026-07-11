//! Attendance service — record, verify, and query daily student attendance.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::EntityTrait;

use shared_kernel::errors::AppError;
use crate::entities::attendance;
use crate::params::AttendanceParams;
use crate::repository::{AssessmentCommandRepo, AssessmentQueryRepo};

use svc_academic::entities::enrollment;
use svc_iam::entities::user;

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

pub struct AttendanceService {
    pub cmd: Arc<AssessmentCommandRepo>,
    pub query: Arc<AssessmentQueryRepo>,
}

impl AttendanceService {
    pub fn new(cmd: Arc<AssessmentCommandRepo>, query: Arc<AssessmentQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Record attendance for a student.
    pub async fn record_attendance(
        &self,
        org_id: &str,
        recorded_by: &str,
        req: CreateAttendanceRequest,
    ) -> Result<attendance::Model, AppError> {
        let valid_statuses = ["present", "absent", "late", "excused", "sick"];
        if !valid_statuses.contains(&req.status.as_str()) {
            return Err(AppError::validation(format!(
                "Invalid status: {}. Allowed: {:?}",
                req.status, valid_statuses
            )));
        }

        let enrollment = enrollment::Entity::find_by_id(req.enrollment_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", &req.enrollment_id))?;

        let record = attendance::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: enrollment.cycle_id,
            enrollment_id: req.enrollment_id,
            class_id: req.class_id,
            date: req.date,
            status: req.status,
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

        let created = self.cmd.create_attendance(record).await?;
        Ok(created)
    }

    /// Record attendance in bulk for a class.
    pub async fn record_bulk_attendance(
        &self,
        org_id: &str,
        recorded_by: &str,
        req: BulkAttendanceRequest,
    ) -> Result<Vec<attendance::Model>, AppError> {
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

    /// Get attendance records matching filters.
    pub async fn get_attendance(
        &self,
        org_id: &str,
        params: AttendanceParams,
    ) -> Result<Vec<attendance::Model>, AppError> {
        Ok(self.query.get_attendance(org_id, params).await?)
    }

    /// Generate attendance report for a student enrollment.
    pub async fn get_attendance_report(&self, enrollment_id: &str) -> Result<AttendanceReport, AppError> {
        let enrollment = enrollment::Entity::find_by_id(enrollment_id.to_string())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", enrollment_id))?;

        let student = user::Entity::find_by_id(enrollment.student_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("student", &enrollment.student_id))?;

        let total_days = self.query.get_attendance_total(enrollment_id).await?;
        let present_days = self.query.get_attendance_count_by_status(enrollment_id, "present").await?;
        let absent_days = self.query.get_attendance_count_by_status(enrollment_id, "absent").await?;
        let late_days = self.query.get_attendance_count_by_status(enrollment_id, "late").await?;
        let excused_days = self.query.get_attendance_count_by_status(enrollment_id, "excused").await?;
        let sick_days = self.query.get_attendance_count_by_status(enrollment_id, "sick").await?;

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

    /// Retrieve school-wide attendance analytics.
    pub async fn get_attendance_analytics(
        &self,
        org_id: &str,
        params: AttendanceParams,
    ) -> Result<AttendanceAnalytics, AppError> {
        let records = self.query.get_attendance(org_id, params).await?;

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
