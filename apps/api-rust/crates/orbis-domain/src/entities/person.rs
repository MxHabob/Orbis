use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Person {
    pub fn full_name(&self) -> String {
        if let Some(ref disp) = self.display_name {
            if !disp.trim().is_empty() {
                return disp.clone();
            }
        }
        match (&self.middle_name, &self.last_name) {
            (Some(mid), last) if !mid.trim().is_empty() => format!("{} {} {}", self.first_name, mid, last),
            _ => format!("{} {}", self.first_name, self.last_name),
        }
    }

    pub fn full_name_ar(&self) -> Option<String> {
        let first = self.first_name_ar.as_deref().unwrap_or("").trim();
        let middle = self.middle_name_ar.as_deref().unwrap_or("").trim();
        let last = self.last_name_ar.as_deref().unwrap_or("").trim();

        if first.is_empty() && last.is_empty() {
            return None;
        }

        if !middle.is_empty() {
            Some(format!("{} {} {}", first, middle, last).trim().to_string())
        } else {
            Some(format!("{} {}", first, last).trim().to_string())
        }
    }
}


