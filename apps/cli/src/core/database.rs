use std::sync::Arc;
use sea_orm::Database;
use orbis_app::services::SystemService;
use orbis_infra::repositories::system_repo::SystemRepoImpl;

use crate::core::license;

/// Comprehensive system status information.
#[derive(Debug)]
pub struct SystemStatus {
    pub db_exists: bool,
    pub has_organization: bool,
    pub has_admin: bool,
    pub has_academic_years: bool,
    pub license_status: license::LicenseStatus,
    pub org_name: Option<String>,
}

impl SystemStatus {
    /// Returns true if CLI init has been completed (organization + admin exist).
    pub fn is_initialized(&self) -> bool {
        self.db_exists && self.has_organization && self.has_admin
    }

    /// Returns true if desktop onboarding is complete (academic years exist).
    pub fn is_setup_complete(&self) -> bool {
        self.is_initialized() && self.has_academic_years
    }
}

/// Returns comprehensive system status by inspecting the database and license.
pub async fn get_full_status() -> SystemStatus {
    let lic_status = license::check_license_status();
    let config = crate::core::config::CliConfig::default();
    let db_url = config.db_url.replace("mode=rwc", "mode=ro");

    let Ok(db) = Database::connect(&db_url).await else {
        return SystemStatus {
            db_exists: false,
            has_organization: false,
            has_admin: false,
            has_academic_years: false,
            license_status: lic_status,
            org_name: None,
        };
    };

    let system_repo = Arc::new(SystemRepoImpl { db });
    let system_service = SystemService::new(system_repo);

    match system_service.get_readiness().await {
        Ok(readiness) => SystemStatus {
            db_exists: true,
            has_organization: readiness.has_organization,
            has_admin: readiness.has_admin,
            has_academic_years: readiness.has_academic_year,
            license_status: lic_status,
            org_name: readiness.org_name,
        },
        Err(_) => SystemStatus {
            db_exists: true,
            has_organization: false,
            has_admin: false,
            has_academic_years: false,
            license_status: lic_status,
            org_name: None,
        },
    }
}

/// Legacy compatibility: Returns (db_exists, setup_completed) tuple.
pub async fn get_system_status() -> (bool, bool) {
    let status = get_full_status().await;
    (status.is_initialized(), status.is_setup_complete())
}
