use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::person::Person;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
            Gender::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudentStatus {
    Active,
    OnLeave,
    Suspended,
    Withdrawn,
    TransferredOut,
    Graduated,
    Expelled,
}

impl std::fmt::Display for StudentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StudentStatus::Active => write!(f, "active"),
            StudentStatus::OnLeave => write!(f, "on_leave"),
            StudentStatus::Suspended => write!(f, "suspended"),
            StudentStatus::Withdrawn => write!(f, "withdrawn"),
            StudentStatus::TransferredOut => write!(f, "transferred_out"),
            StudentStatus::Graduated => write!(f, "graduated"),
            StudentStatus::Expelled => write!(f, "expelled"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicStanding {
    GoodStanding,
    HonorList,
    AcademicWarning1,
    AcademicWarning2,
    AcademicWarning3,
    Probation,
    Dismissed,
}

impl std::fmt::Display for AcademicStanding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcademicStanding::GoodStanding => write!(f, "good_standing"),
            AcademicStanding::HonorList => write!(f, "honor_list"),
            AcademicStanding::AcademicWarning1 => write!(f, "warning_1"),
            AcademicStanding::AcademicWarning2 => write!(f, "warning_2"),
            AcademicStanding::AcademicWarning3 => write!(f, "warning_3"),
            AcademicStanding::Probation => write!(f, "probation"),
            AcademicStanding::Dismissed => write!(f, "dismissed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
    pub student_number: String,
    pub status: StudentStatus,
    pub standing: AcademicStanding,
    pub admitted_at: DateTime<Utc>,
    pub graduated_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub meta: serde_json::Value,
    pub org_unit_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentDetail {
    pub student: Student,
    pub person: Person,
}

impl Student {
    pub fn can_enroll(&self) -> bool {
        matches!(self.status, StudentStatus::Active)
    }

    pub fn is_in_good_standing(&self) -> bool {
        matches!(self.standing, AcademicStanding::GoodStanding | AcademicStanding::HonorList)
    }
}

