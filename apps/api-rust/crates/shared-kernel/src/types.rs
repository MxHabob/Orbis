//! Strongly-typed domain identifiers.
//!
//! Using newtypes prevents mixing up IDs from different domains at compile time.
//! For example, passing a `UserId` where a `ClassId` is expected will be a type error.

use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            pub fn generate() -> Self {
                Self(uuid::Uuid::new_v4().to_string())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

// ── Domain Identifiers ──────────────────────────────────────────

define_id!(
    /// Organization identifier — tenant boundary.
    OrgId
);

define_id!(
    /// User identifier — students, teachers, staff, admins.
    UserId
);

define_id!(
    /// Academic period identifier.
    PeriodId
);

define_id!(
    /// Class identifier.
    ClassId
);

define_id!(
    /// Subject identifier.
    SubjectId
);

define_id!(
    /// Enrollment identifier.
    EnrollmentId
);

define_id!(
    /// Assessment identifier.
    AssessmentId
);

define_id!(
    /// Grade identifier.
    GradeId
);

define_id!(
    /// Exam session identifier.
    ExamSessionId
);

define_id!(
    /// Academic year identifier (registry).
    AcademicPeriodId
);

define_id!(
    /// Calendar event identifier.
    CalendarEventId
);

define_id!(
    /// Autopilot task identifier.
    AutopilotTaskId
);

define_id!(
    /// AI agent session identifier.
    AgentSessionId
);

define_id!(
    /// Correlation identifier — links related operations across the system.
    CorrelationId
);

// ── Risk Level ──────────────────────────────────────────────────

/// Risk level for operations — controls AI agent authorization gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "low" => Self::Low,
            "medium" => Self::Medium,
            "high" => Self::High,
            "critical" => Self::Critical,
            _ => Self::Medium,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ── Execution Mode ──────────────────────────────────────────────

/// Autopilot execution mode — determines how automated tasks are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    /// Execute automatically without human approval.
    Automatic,
    /// Suggest actions and wait for human approval.
    Suggested,
    /// Observe and notify only — no automated execution.
    Manual,
}

impl ExecutionMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "automatic" => Self::Automatic,
            "suggested" => Self::Suggested,
            "manual" => Self::Manual,
            _ => Self::Suggested, // safe default
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Suggested => "suggested",
            Self::Manual => "manual",
        }
    }

    /// Resolve effective mode: event override wins if set, otherwise inherit from config.
    pub fn resolve(config_mode: &str, event_override: Option<&str>) -> Self {
        event_override
            .filter(|m| !m.is_empty())
            .map(Self::from_str)
            .unwrap_or_else(|| Self::from_str(config_mode))
    }
}

impl fmt::Display for ExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
