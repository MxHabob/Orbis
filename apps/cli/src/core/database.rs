use sea_orm::{ConnectionTrait, Database, DatabaseBackend, Statement};

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

    // Check organizations
    let (has_org, org_name) = match db
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "SELECT name FROM organizations LIMIT 1;".to_owned(),
        ))
        .await
    {
        Ok(Some(row)) => {
            let name: Option<String> = row.try_get("", "name").ok();
            (true, name)
        }
        _ => (false, None),
    };

    if !has_org {
        return SystemStatus {
            db_exists: true,
            has_organization: false,
            has_admin: false,
            has_academic_years: false,
            license_status: lic_status,
            org_name: None,
        };
    }

    // Check admin users
    let has_admin = match db
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "SELECT COUNT(*) as cnt FROM user_role_assignments WHERE role_id IN ('super_admin', 'admin');".to_owned(),
        ))
        .await
    {
        Ok(Some(row)) => {
            let count: i64 = row.try_get("", "cnt").unwrap_or(0);
            count > 0
        }
        _ => false,
    };

    // Check academic years (setup completed via Desktop)
    let has_academic_years = match db
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "SELECT COUNT(*) as cnt FROM academic_years;".to_owned(),
        ))
        .await
    {
        Ok(Some(row)) => {
            let count: i64 = row.try_get("", "cnt").unwrap_or(0);
            count > 0
        }
        _ => false,
    };

    SystemStatus {
        db_exists: true,
        has_organization: has_org,
        has_admin,
        has_academic_years,
        license_status: lic_status,
        org_name,
    }
}

/// Legacy compatibility: Returns (db_exists, setup_completed) tuple.
pub async fn get_system_status() -> (bool, bool) {
    let status = get_full_status().await;
    (status.is_initialized(), status.is_setup_complete())
}
