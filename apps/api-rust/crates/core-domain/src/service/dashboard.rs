use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DashboardStats {
    pub total_students: u64,
    pub total_teachers: u64,
    pub total_staff: u64,
    pub total_classes: u64,
    pub total_enrollments: u64,
    pub total_organizations: u64,
    pub behavior_summary: BehaviorSummary,
    pub attendance_overview: AttendanceOverview,
    pub enrollment_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BehaviorSummary {
    pub total_records: usize,
    pub positive_points: i32,
    pub negative_points: i32,
    pub net_points: i32,
    pub critical_count: usize,
    pub major_count: usize,
    pub minor_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AttendanceOverview {
    pub total_records: usize,
    pub present_rate: f64,
    pub absent_rate: f64,
    pub late_rate: f64,
}

impl super::Service {
    pub async fn get_dashboard_stats(&self, org_id: &str) -> Result<DashboardStats, String> {
        // Student count
        let total_students = self.repo.get_users_by_org_role_count(org_id, "student").await
            .map_err(|e| e.to_string())?;

        // Teacher count
        let total_teachers = self.repo.get_users_by_org_role_count(org_id, "teacher").await
            .map_err(|e| e.to_string())?;

        // Staff count (admin + staff + principal)
        let admin_count = self.repo.get_users_by_org_role_count(org_id, "admin").await
            .map_err(|e| e.to_string())?;
        let staff_count = self.repo.get_users_by_org_role_count(org_id, "staff").await
            .map_err(|e| e.to_string())?;
        let principal_count = self.repo.get_users_by_org_role_count(org_id, "principal").await
            .map_err(|e| e.to_string())?;
        let total_staff = admin_count + staff_count + principal_count;

        // Class count
        let classes = self.repo.get_classes(org_id, crate::params::ClassParams {
            q: None,
            is_active: Some(1),
        }).await.map_err(|e| e.to_string())?;
        let total_classes = classes.len() as u64;

        // Enrollment count
        let total_enrollments = self.repo.get_enrollment_count(org_id).await
            .map_err(|e| e.to_string())?;

        // Enrollment rate (students enrolled / total students)
        let enrollment_rate = if total_students > 0 {
            (total_enrollments as f64 / total_students as f64) * 100.0
        } else {
            0.0
        };

        // Organization count
        let total_organizations = self.repo.get_organization_count().await
            .map_err(|e| e.to_string())?;

        // Behavior summary
        let behavior_records = self.repo.get_behavior_records(org_id, crate::params::BehaviorParams {
            student_id: None,
            severity: None,
            status: None,
            q: None,
            limit: Some(1000),
            offset: None,
        }).await.map_err(|e| e.to_string())?;

        let mut positive_points = 0i32;
        let mut negative_points = 0i32;
        let mut critical_count = 0usize;
        let mut major_count = 0usize;
        let mut minor_count = 0usize;

        for r in &behavior_records {
            let pts = r.points.unwrap_or(0);
            if pts > 0 { positive_points += pts; } else { negative_points += pts; }
            match r.severity.as_str() {
                "critical" => critical_count += 1,
                "major" => major_count += 1,
                "minor" => minor_count += 1,
                _ => {}
            }
        }

        // Attendance overview
        let attendance_records = self.repo.get_attendance(org_id, crate::params::AttendanceParams {
            enrollment_id: None,
            class_id: None,
            cycle_id: None,
            date: None,
            date_from: None,
            date_to: None,
            status: None,
            limit: Some(5000),
            offset: None,
        }).await.map_err(|e| e.to_string())?;

        let att_total = attendance_records.len();
        let present_count = attendance_records.iter().filter(|r| r.status == "present").count();
        let absent_count = attendance_records.iter().filter(|r| r.status == "absent").count();
        let late_count = attendance_records.iter().filter(|r| r.status == "late").count();

        let present_rate = if att_total > 0 { (present_count as f64 / att_total as f64) * 100.0 } else { 0.0 };
        let absent_rate = if att_total > 0 { (absent_count as f64 / att_total as f64) * 100.0 } else { 0.0 };
        let late_rate = if att_total > 0 { (late_count as f64 / att_total as f64) * 100.0 } else { 0.0 };

        Ok(DashboardStats {
            total_students,
            total_teachers,
            total_staff,
            total_classes,
            total_enrollments,
            total_organizations,
            behavior_summary: BehaviorSummary {
                total_records: behavior_records.len(),
                positive_points,
                negative_points,
                net_points: positive_points + negative_points,
                critical_count,
                major_count,
                minor_count,
            },
            attendance_overview: AttendanceOverview {
                total_records: att_total,
                present_rate: (present_rate * 100.0).round() / 100.0,
                absent_rate: (absent_rate * 100.0).round() / 100.0,
                late_rate: (late_rate * 100.0).round() / 100.0,
            },
            enrollment_rate: (enrollment_rate * 100.0).round() / 100.0,
        })
    }
}

