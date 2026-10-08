use crate::errors::DomainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Institution type — determines blueprints for grading, periods, COA, and defaults.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstitutionType {
    School,
    University,
    Institute,
}

impl InstitutionType {
    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "university" => Self::University,
            "institute" | "training_center" => Self::Institute,
            _ => Self::School,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::School => "school",
            Self::University => "university",
            Self::Institute => "institute",
        }
    }
}

impl std::fmt::Display for InstitutionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub slug: String,
    pub code: Option<String>,
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub format_locale: Option<String>,
    pub time_format: Option<String>,
    pub is_hijri_enabled: bool,
    pub base_currency_code: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub primary_color: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub current_period_id: Option<String>, // Usually references an academic period ID from Calendar Context
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Organization {
    pub fn new(
        name: String,
        slug: String,
        timezone: String,
        locale: String,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Organization name cannot be empty".into(),
            ));
        }
        if slug.trim().is_empty() {
            return Err(DomainError::Validation(
                "Organization slug cannot be empty".into(),
            ));
        }

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            name_ar: None,
            slug,
            code: None,
            timezone,
            locale,
            date_format: Some("DD/MM/YYYY".to_string()),
            format_locale: None,
            time_format: None,
            is_hijri_enabled: true,
            base_currency_code: None,
            settings: Some(serde_json::json!({})),
            features: Some(serde_json::json!({})),
            logo: None,
            favicon: None,
            primary_color: Some("#6366f1".to_string()),
            is_active: true,
            is_verified: false,
            is_deleted: false,
            deleted_at: None,
            deleted_by: None,
            current_period_id: None,
            metadata: None,
            created_by: None,
            updated_by: None,
            version: 1,
            created_at: now,
            updated_at: now,
        })
    }
}
