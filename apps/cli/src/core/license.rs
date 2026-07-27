use anyhow::{anyhow, Result};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

use crate::core::validator;

/// License plan types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicensePlan {
    Trial,
    Basic,
    Professional,
    Enterprise,
}

impl std::fmt::Display for LicensePlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicensePlan::Trial => write!(f, "Trial"),
            LicensePlan::Basic => write!(f, "Basic"),
            LicensePlan::Professional => write!(f, "Professional"),
            LicensePlan::Enterprise => write!(f, "Enterprise"),
        }
    }
}

/// Current status of a license.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseStatus {
    Active,
    Expired,
    Invalid,
    NotFound,
}

impl std::fmt::Display for LicenseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseStatus::Active => write!(f, "Active"),
            LicenseStatus::Expired => write!(f, "Expired"),
            LicenseStatus::Invalid => write!(f, "Invalid"),
            LicenseStatus::NotFound => write!(f, "Not Found"),
        }
    }
}

/// Information about an activated license.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub key: String,
    pub plan: LicensePlan,
    pub status: LicenseStatus,
    pub activated_at: String,
    pub expires_at: String,
    pub max_users: u32,
    pub max_students: u32,
    pub features: Vec<String>,
}

impl LicenseInfo {
    /// Checks whether the license has expired.
    pub fn is_expired(&self) -> bool {
        if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(&self.expires_at) {
            Utc::now() > expires
        } else {
            true
        }
    }

    /// Returns days remaining until expiration.
    pub fn days_remaining(&self) -> i64 {
        if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(&self.expires_at) {
            let diff = expires.signed_duration_since(Utc::now());
            diff.num_days().max(0)
        } else {
            0
        }
    }
}

/// Returns the path to the license file.
fn license_file_path() -> PathBuf {
    let identifier = if cfg!(debug_assertions) {
        "com.mulverse.orbis.dev"
    } else {
        "com.mulverse.orbis.svelte"
    };

    if let Some(mut data_dir) = dirs::data_dir() {
        data_dir.push(identifier);
        data_dir.push("license.json");
        data_dir
    } else {
        PathBuf::from("license.json")
    }
}

/// Validates and activates a license key.
///
/// In this implementation, validation is done offline by decoding the key format.
/// The key format encodes plan type in the first segment:
///   - `TRIA-xxxx-xxxx-xxxx` = Trial
///   - `BASC-xxxx-xxxx-xxxx` = Basic
///   - `PROF-xxxx-xxxx-xxxx` = Professional
///   - `ENTR-xxxx-xxxx-xxxx` = Enterprise
///
/// Any valid XXXX-XXXX-XXXX-XXXX format is accepted, with plan defaulting to Professional.
pub fn activate_license(key: &str) -> Result<LicenseInfo> {
    validator::validate_license_key(key)?;

    let normalized = key.trim().to_uppercase();
    let prefix = &normalized[..4];

    let (plan, max_users, max_students, features) = match prefix {
        "TRIA" => (
            LicensePlan::Trial,
            10_u32,
            100_u32,
            vec!["core".to_string()],
        ),
        "BASC" => (
            LicensePlan::Basic,
            50,
            500,
            vec!["core".to_string(), "reports".to_string()],
        ),
        "PROF" => (
            LicensePlan::Professional,
            200,
            2000,
            vec!["core".to_string(), "reports".to_string(), "finance".to_string(), "exams".to_string()],
        ),
        "ENTR" => (
            LicensePlan::Enterprise,
            500,
            5000,
            vec!["core".to_string(), "reports".to_string(), "finance".to_string(), "exams".to_string(), "api".to_string(), "multi_branch".to_string()],
        ),
        _ => (
            LicensePlan::Professional,
            200,
            2000,
            vec!["core".to_string(), "reports".to_string(), "finance".to_string(), "exams".to_string()],
        ),
    };

    // Trial licenses expire in 30 days, others in 365 days
    let duration_days = match plan {
        LicensePlan::Trial => 30,
        _ => 365,
    };

    let now = Utc::now();
    let expires = now + Duration::days(duration_days);

    let info = LicenseInfo {
        key: normalized,
        plan,
        status: LicenseStatus::Active,
        activated_at: now.to_rfc3339(),
        expires_at: expires.to_rfc3339(),
        max_users,
        max_students,
        features,
    };

    save_license(&info)?;
    Ok(info)
}

/// Activates a 30-day trial license.
pub fn activate_trial() -> Result<LicenseInfo> {
    let now = Utc::now();
    let expires = now + Duration::days(30);

    let info = LicenseInfo {
        key: "TRIAL-MODE".to_string(),
        plan: LicensePlan::Trial,
        status: LicenseStatus::Active,
        activated_at: now.to_rfc3339(),
        expires_at: expires.to_rfc3339(),
        max_users: 10,
        max_students: 100,
        features: vec!["core".to_string()],
    };

    save_license(&info)?;
    Ok(info)
}

/// Loads the current license from disk.
pub fn load_license() -> Result<LicenseInfo> {
    let path = license_file_path();
    if !path.exists() {
        return Err(anyhow!("No license file found. Run `orbis init` or `orbis license` to activate."));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow!("Failed to read license file: {}", e))?;

    let mut info: LicenseInfo = serde_json::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse license file: {}", e))?;

    // Check expiry
    if info.is_expired() {
        info.status = LicenseStatus::Expired;
    }

    Ok(info)
}

/// Checks license status, returning NotFound if no license exists.
pub fn check_license_status() -> LicenseStatus {
    match load_license() {
        Ok(info) => {
            if info.is_expired() {
                LicenseStatus::Expired
            } else {
                LicenseStatus::Active
            }
        }
        Err(_) => LicenseStatus::NotFound,
    }
}

/// Saves license information to disk.
fn save_license(info: &LicenseInfo) -> Result<()> {
    let path = license_file_path();

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow!("Failed to create license directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(info)
        .map_err(|e| anyhow!("Failed to serialize license: {}", e))?;

    std::fs::write(&path, content)
        .map_err(|e| anyhow!("Failed to write license file: {}", e))?;

    Ok(())
}
