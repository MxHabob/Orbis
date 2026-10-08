use anyhow::{Result, anyhow};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::core::validator;
use orbis_domain::entities::license::{
    ENTITLEMENT_AI_GRADING, ENTITLEMENT_CAMPUS_ANALYTICS, ENTITLEMENT_EXAM_TOOLS,
    ENTITLEMENT_MULTI_BRANCH, ENTITLEMENT_OFFLINE_SYNC, ENTITLEMENT_PARENT_PORTAL,
    ENTITLEMENT_SSO, ENTITLEMENT_STUDENTS,
};

/// License plan types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicensePlan {
    Trial,
    School,
    SchoolPro,
    University,
    Enterprise,
}

impl std::fmt::Display for LicensePlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicensePlan::Trial => write!(f, "Trial"),
            LicensePlan::School => write!(f, "School"),
            LicensePlan::SchoolPro => write!(f, "School Pro"),
            LicensePlan::University => write!(f, "University"),
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

/// Information about an activated license in CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub key: String,
    pub plan: LicensePlan,
    pub status: LicenseStatus,
    pub activated_at: String,
    pub expires_at: String,
    pub max_staff: u32,
    pub max_students: u32,
    pub max_devices: u32,
    pub max_branches: u32,
    pub max_classes: u32,
    pub max_storage_gb: u32,
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
        "com.mulverse.orbis"
    };

    if let Some(mut data_dir) = dirs::data_dir() {
        data_dir.push(identifier);
        data_dir.push("license.json");
        data_dir
    } else {
        PathBuf::from("license.json")
    }
}

/// Validates and activates an Orbis license key (ORBIS-XXXXX-XXXXX-XXXXX-XXXXX).
pub fn activate_license(key: &str) -> Result<LicenseInfo> {
    validator::validate_license_key(key)?;

    let normalized = key.trim().to_uppercase();

    // Standard plan features per ORBIS_INTEGRATION_CONTRACT.md
    let plan = LicensePlan::SchoolPro;
    let max_students = 5000;
    let max_staff = 250;
    let max_devices = 200;
    let max_branches = 12;
    let max_classes = 120;
    let max_storage_gb = 50;

    let features = vec![
        ENTITLEMENT_STUDENTS.to_string(),
        ENTITLEMENT_AI_GRADING.to_string(),
        ENTITLEMENT_MULTI_BRANCH.to_string(),
        ENTITLEMENT_OFFLINE_SYNC.to_string(),
        ENTITLEMENT_PARENT_PORTAL.to_string(),
        ENTITLEMENT_CAMPUS_ANALYTICS.to_string(),
        ENTITLEMENT_SSO.to_string(),
        ENTITLEMENT_EXAM_TOOLS.to_string(),
    ];

    let now = Utc::now();
    let expires = now + Duration::days(365);

    let info = LicenseInfo {
        key: normalized,
        plan,
        status: LicenseStatus::Active,
        activated_at: now.to_rfc3339(),
        expires_at: expires.to_rfc3339(),
        max_staff,
        max_students,
        max_devices,
        max_branches,
        max_classes,
        max_storage_gb,
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
        key: "ORBIS-TRIAL-00000-00000-00000".to_string(),
        plan: LicensePlan::Trial,
        status: LicenseStatus::Active,
        activated_at: now.to_rfc3339(),
        expires_at: expires.to_rfc3339(),
        max_staff: 25,
        max_students: 100,
        max_devices: 10,
        max_branches: 1,
        max_classes: 10,
        max_storage_gb: 5,
        features: vec![
            ENTITLEMENT_STUDENTS.to_string(),
            ENTITLEMENT_OFFLINE_SYNC.to_string(),
            ENTITLEMENT_EXAM_TOOLS.to_string(),
        ],
    };

    save_license(&info)?;
    Ok(info)
}

/// Loads the current license from disk.
pub fn load_license() -> Result<LicenseInfo> {
    let path = license_file_path();
    if !path.exists() {
        return Err(anyhow!(
            "No license file found. Run `orbis init` or `orbis license` to activate."
        ));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow!("Failed to read license file: {}", e))?;

    let mut info: LicenseInfo = serde_json::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse license file: {}", e))?;

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

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow!("Failed to create license directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(info)
        .map_err(|e| anyhow!("Failed to serialize license: {}", e))?;

    std::fs::write(&path, content).map_err(|e| anyhow!("Failed to write license file: {}", e))?;

    Ok(())
}
