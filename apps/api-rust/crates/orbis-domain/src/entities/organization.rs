use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::DomainError;

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
    pub hijri_enabled: bool,
    pub max_users: i32,
    pub max_students: i32,
    pub max_storage_gb: i32,
    pub base_currency_id: Option<String>,
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
            return Err(DomainError::Validation("Organization name cannot be empty".into()));
        }
        if slug.trim().is_empty() {
            return Err(DomainError::Validation("Organization slug cannot be empty".into()));
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
            hijri_enabled: true,
            max_users: 1000,
            max_students: 5000,
            max_storage_gb: 100,
            base_currency_id: None,
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
            created_at: now,
            updated_at: now,
        })
    }
}
