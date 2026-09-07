use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideType {
    GradeChange,
    PrerequisiteBypass,
    LateEnrollment,
    CapacityOverride,
    CreditOverload,
    DiscountOverride,
    AcademicStandingOverride,
    Custom(u8),
}

impl std::fmt::Display for OverrideType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverrideType::GradeChange => write!(f, "grade_change"),
            OverrideType::PrerequisiteBypass => write!(f, "prerequisite_bypass"),
            OverrideType::LateEnrollment => write!(f, "late_enrollment"),
            OverrideType::CapacityOverride => write!(f, "capacity_override"),
            OverrideType::CreditOverload => write!(f, "credit_overload"),
            OverrideType::DiscountOverride => write!(f, "discount_override"),
            OverrideType::AcademicStandingOverride => write!(f, "standing_override"),
            OverrideType::Custom(code) => write!(f, "custom_{}", code),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "low"),
            RiskLevel::Medium => write!(f, "medium"),
            RiskLevel::High => write!(f, "high"),
            RiskLevel::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Executed,
    Cancelled,
}

impl std::fmt::Display for OverrideStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverrideStatus::Pending => write!(f, "pending"),
            OverrideStatus::Approved => write!(f, "approved"),
            OverrideStatus::Rejected => write!(f, "rejected"),
            OverrideStatus::Expired => write!(f, "expired"),
            OverrideStatus::Executed => write!(f, "executed"),
            OverrideStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalStep {
    pub step_order: u32,
    pub approver_role: String,
    pub approver_id: Option<Uuid>,
    pub approver_name: Option<String>,
    pub status: OverrideStatus,
    pub decision_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicOverrideRequest {
    pub id: Uuid,
    pub org_id: Uuid,
    pub override_type: OverrideType,
    pub risk_level: RiskLevel,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
    pub requested_by: Uuid,
    pub requested_by_name: Option<String>,
    pub status: OverrideStatus,
    pub current_approver_role: Option<String>,
    pub approval_chain: Vec<ApprovalStep>,
    pub resolved_by: Option<Uuid>,
    pub resolved_by_name: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AcademicOverrideRequest {
    pub fn is_active(&self) -> bool {
        matches!(self.status, OverrideStatus::Approved) && 
        self.expires_at.map(|exp| exp > Utc::now()).unwrap_or(true)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.status, OverrideStatus::Pending)
    }
}
