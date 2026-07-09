//! Academic Year service — year lifecycle management.
//!
//! Operations: create, list, switch, archive.
//! All operate on the registry database via standalone repos.

use sea_orm::DatabaseConnection;
use tracing::info;

use crate::entities::academic_year;
use crate::repository::academic_year::AcademicYearRepo;

/// Service for academic year management (operates on registry DB).
pub struct AcademicYearService;

impl AcademicYearService {
    /// List all academic years for an organization.
    pub async fn list(
        registry_db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<academic_year::Model>, String> {
        AcademicYearRepo::get_all(registry_db, org_id)
            .await
            .map_err(|e| format!("Failed to list academic years: {}", e))
    }

    /// Get the current active year for an organization.
    pub async fn get_current(
        registry_db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Option<academic_year::Model>, String> {
        AcademicYearRepo::get_current(registry_db, org_id)
            .await
            .map_err(|e| format!("Failed to get current year: {}", e))
    }

    /// Get a year by ID.
    pub async fn get_by_id(
        registry_db: &DatabaseConnection,
        year_id: &str,
    ) -> Result<academic_year::Model, String> {
        AcademicYearRepo::get_by_id(registry_db, year_id)
            .await
            .map_err(|e| format!("Failed to get year: {}", e))?
            .ok_or_else(|| "Academic year not found".to_string())
    }

    /// Set a year as the current active year for its organization.
    pub async fn set_current(
        registry_db: &DatabaseConnection,
        org_id: &str,
        year_id: &str,
    ) -> Result<(), String> {
        // Verify the year exists and is not archived
        let year = Self::get_by_id(registry_db, year_id).await?;
        if year.status == "archived" {
            return Err("Cannot set an archived year as the active year".to_string());
        }

        AcademicYearRepo::set_current(registry_db, org_id, year_id)
            .await
            .map_err(|e| format!("Failed to set current year: {}", e))?;

        info!("✅ Set year '{}' as current for org '{}'", year.name, org_id);
        Ok(())
    }

    /// Archive a year (make it read-only).
    pub async fn archive(
        registry_db: &DatabaseConnection,
        year_id: &str,
        user_id: &str,
    ) -> Result<academic_year::Model, String> {
        let year = Self::get_by_id(registry_db, year_id).await?;
        if year.is_current == 1 {
            return Err(
                "Cannot archive the currently active year. Switch to another year first."
                    .to_string(),
            );
        }
        if year.status == "archived" {
            return Err("Year is already archived".to_string());
        }

        let updated = AcademicYearRepo::update_status(registry_db, year_id, "archived", Some(user_id))
            .await
            .map_err(|e| format!("Failed to archive year: {}", e))?;

        info!("📚 Archived year '{}' by user '{}'", year.name, user_id);
        Ok(updated)
    }

    /// Update year statistics (student/teacher/class counts).
    pub async fn refresh_stats(
        registry_db: &DatabaseConnection,
        year_id: &str,
        year_db: &DatabaseConnection,
    ) -> Result<(), String> {
        use sea_orm::EntityTrait;

        // Count students (users with role 'student' and active enrollments)
        let student_count = crate::entities::enrollment::Entity::find()
            .all(year_db)
            .await
            .map(|e| e.len() as i32)
            .unwrap_or(0);

        // Count classes
        let class_count = crate::entities::class::Entity::find()
            .all(year_db)
            .await
            .map(|c| c.len() as i32)
            .unwrap_or(0);

        AcademicYearRepo::update_stats(
            registry_db,
            year_id,
            Some(student_count),
            None, // teacher count requires a separate query
            Some(class_count),
            None,
        )
        .await
        .map_err(|e| format!("Failed to update stats: {}", e))
    }
}
