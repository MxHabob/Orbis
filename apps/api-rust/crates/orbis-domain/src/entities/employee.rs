use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::person::Person;
use super::user::User;
use super::group::Group;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeStatus {
    Active,
    OnLeave,
    Suspended,
    Terminated,
    Resigned,
}

impl std::fmt::Display for EmployeeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmployeeStatus::Active => write!(f, "active"),
            EmployeeStatus::OnLeave => write!(f, "on_leave"),
            EmployeeStatus::Suspended => write!(f, "suspended"),
            EmployeeStatus::Terminated => write!(f, "terminated"),
            EmployeeStatus::Resigned => write!(f, "resigned"),
        }
    }
}

impl From<&str> for EmployeeStatus {
    fn from(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "active" => EmployeeStatus::Active,
            "on_leave" | "onleave" | "leave" => EmployeeStatus::OnLeave,
            "suspended" => EmployeeStatus::Suspended,
            "terminated" => EmployeeStatus::Terminated,
            "resigned" => EmployeeStatus::Resigned,
            _ => EmployeeStatus::Active,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeType {
    Teacher,
    Admin,
    Staff,
    Principal,
    Accountant,
    Supervisor,
    Driver,
    Worker,
    Custom(String),
}

impl std::fmt::Display for EmployeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmployeeType::Teacher => write!(f, "teacher"),
            EmployeeType::Admin => write!(f, "admin"),
            EmployeeType::Staff => write!(f, "staff"),
            EmployeeType::Principal => write!(f, "principal"),
            EmployeeType::Accountant => write!(f, "accountant"),
            EmployeeType::Supervisor => write!(f, "supervisor"),
            EmployeeType::Driver => write!(f, "driver"),
            EmployeeType::Worker => write!(f, "worker"),
            EmployeeType::Custom(s) => write!(f, "{}", s.to_lowercase()),
        }
    }
}

impl EmployeeType {
    pub fn is_teacher(&self) -> bool {
        matches!(self, EmployeeType::Teacher)
    }
}

impl From<&str> for EmployeeType {
    fn from(s: &str) -> Self {
        let clean = s.trim().to_lowercase();
        match clean.as_str() {
            "teacher" => EmployeeType::Teacher,
            "admin" | "administrative" => EmployeeType::Admin,
            "staff" | "support" => EmployeeType::Staff,
            "principal" | "director" => EmployeeType::Principal,
            "accountant" => EmployeeType::Accountant,
            "supervisor" => EmployeeType::Supervisor,
            "driver" => EmployeeType::Driver,
            "worker" => EmployeeType::Worker,
            _ => EmployeeType::Custom(clean),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Hourly,
}

impl std::fmt::Display for EmploymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmploymentType::FullTime => write!(f, "full_time"),
            EmploymentType::PartTime => write!(f, "part_time"),
            EmploymentType::Contract => write!(f, "contract"),
            EmploymentType::Hourly => write!(f, "hourly"),
        }
    }
}

impl From<&str> for EmploymentType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "full_time" | "fulltime" => EmploymentType::FullTime,
            "part_time" | "parttime" => EmploymentType::PartTime,
            "contract" => EmploymentType::Contract,
            "hourly" => EmploymentType::Hourly,
            _ => EmploymentType::FullTime,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
    pub employee_number: String,
    pub employee_type: String, // dynamic/configurable (e.g. "teacher", "admin", "driver", "accountant", etc.)
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub hire_date: NaiveDate,
    pub status: EmployeeStatus,
    pub employment_type: EmploymentType,
    pub work_shift: Option<String>,
    pub notes: Option<String>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeDetail {
    pub employee: Employee,
    pub person: Person,
    pub user: Option<User>,
    pub roles: Vec<String>,
    pub groups: Vec<Group>,
    pub effective_permissions: Vec<String>,
    pub org_unit_name: Option<String>,
}

impl Employee {
    pub fn is_active(&self) -> bool {
        matches!(self.status, EmployeeStatus::Active)
    }

    pub fn is_teacher(&self) -> bool {
        EmployeeType::from(self.employee_type.as_str()).is_teacher()
            || self.job_title.as_deref().unwrap_or("").to_lowercase().contains("teacher")
    }
}
