//! Orbis Organization Domain — Organization & Academic Year Management.
//!
//! This crate owns:
//! - **Organization**: Tenant configuration, branding, limits
//! - **Academic Periods**: Years, semesters, quarters (hierarchical)
//! - **Calendar Events**: Academic milestones with execution modes
//! - **Policies**: Academic grading defaults, promotion rules, overrides
//!
//! # Platform Trait Implementations
//!
//! This crate provides concrete implementations for:
//! - `SettingsProvider` → `OrgSettingsProvider` (reads org.settings JSON)
//! - `PolicyEngine`     → `OrgPolicyEngine` (reads org_academic_policies)

pub mod entities;
pub mod repository;
pub mod validation;
pub mod params;

pub mod contracts;
pub mod ops;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bootstrap_full() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let provider = infra_persistence::provider::DatabaseProvider::init(temp_dir.path())
            .await
            .expect("init database provider");

        let payload = ops::bootstrap::BootstrapPayload {
            org_name: "Test School".into(),
            org_name_ar: Some("مدرسة اختبار".into()),
            org_type: "school".into(),
            currency: "SAR".into(),
            timezone: Some("Asia/Riyadh".into()),
            locale: Some("ar".into()),
            admin_name: "Mohamed Admin".into(),
            admin_username: "admin@test.edu".into(),
            admin_password_hash: bcrypt::hash("Password123!", bcrypt::DEFAULT_COST).unwrap(),
        };

        let result = ops::bootstrap::bootstrap_full(&provider.db(), payload).await;
        assert!(result.is_ok(), "Bootstrap should succeed without errors: {:?}", result.err());
    }
}

