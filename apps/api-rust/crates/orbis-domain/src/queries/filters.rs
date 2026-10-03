use crate::entities::enrollment::{EnrollmentStatus, StudyMode};
use crate::entities::payment::PaymentMethod;
use crate::entities::student::{AcademicStanding, StudentStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

impl PaginationParams {
    pub const DEFAULT_PAGE_SIZE: u64 = 20;
    pub const MAX_PAGE_SIZE: u64 = 100;

    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> u64 {
        self.page_size
            .unwrap_or(Self::DEFAULT_PAGE_SIZE)
            .min(Self::MAX_PAGE_SIZE)
            .max(1)
    }

    pub fn offset(&self) -> u64 {
        (self.page() - 1) * self.page_size()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaginatedList<T: Serialize> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

impl<T: Serialize> PaginatedList<T> {
    pub fn new(items: Vec<T>, total_count: u64, page: u64, page_size: u64) -> Self {
        let total_pages = if page_size > 0 {
            (total_count + page_size - 1) / page_size
        } else {
            1
        };
        Self {
            items,
            total_count,
            page,
            page_size,
            total_pages,
        }
    }

    pub fn empty(page_size: u64) -> Self {
        Self {
            items: Vec::new(),
            total_count: 0,
            page: 1,
            page_size,
            total_pages: 0,
        }
    }
}

// ── Domain Filter Structs ────────────────────────────────────────

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct StudentFilter {
    pub period_id: Option<Uuid>,
    pub class_id: Option<Uuid>,
    pub status: Option<StudentStatus>,
    pub standing: Option<AcademicStanding>,
    pub search: Option<String>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct EnrollmentFilter {
    pub period_id: Option<Uuid>,
    pub class_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub study_mode: Option<StudyMode>,
    pub status: Option<EnrollmentStatus>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct SubjectFilter {
    pub unit_id: Option<Uuid>,
    pub is_core: Option<bool>,
    pub is_active: Option<bool>,
    pub search: Option<String>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ClassFilter {
    pub unit_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub level: Option<String>,
    pub search: Option<String>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ClassSubjectFilter {
    pub class_id: Option<Uuid>,
    pub subject_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct GradeFilter {
    pub student_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub class_subject_id: Option<Uuid>,
    pub component_id: Option<Uuid>,
    pub is_final: Option<bool>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct AttendanceFilter {
    pub class_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub class_subject_id: Option<Uuid>,
    pub date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct PaymentFilter {
    pub student_id: Option<Uuid>,
    pub fee_id: Option<Uuid>,
    pub enrollment_id: Option<Uuid>,
    pub method: Option<PaymentMethod>,
    pub from_date: Option<chrono::DateTime<chrono::Utc>>,
    pub to_date: Option<chrono::DateTime<chrono::Utc>>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct StudentFeeFilter {
    pub student_id: Option<Uuid>,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Option<Uuid>,
    pub status: Option<String>,
    pub is_overdue: Option<bool>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct AccountFilter {
    pub account_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub search: Option<String>,
    pub pagination: Option<PaginationParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ExamScheduleFilter {
    pub session_id: Option<Uuid>,
    pub subject_id: Option<Uuid>,
    pub date: Option<chrono::NaiveDate>,
    pub pagination: Option<PaginationParams>,
}
