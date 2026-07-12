use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set, DbErr, PaginatorTrait,
};

use crate::entities::academic_year;

/// Repository for academic year operations against the registry database.
pub struct AcademicYearRepo;

impl AcademicYearRepo {
    /// Get all academic years for an organization, ordered by start_date descending.
    pub async fn get_all(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<academic_year::Model>, DbErr> {
        academic_year::Entity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .order_by_desc(academic_year::Column::StartDate)
            .all(db)
            .await
    }

    /// Get a single academic year by ID.
    pub async fn get_by_id(
        db: &DatabaseConnection,
        year_id: &str,
    ) -> Result<Option<academic_year::Model>, DbErr> {
        academic_year::Entity::find_by_id(year_id).one(db).await
    }

    /// Get the current active academic year for an organization.
    pub async fn get_current(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Option<academic_year::Model>, DbErr> {
        use crate::entities::organization;
        
        let org = organization::Entity::find_by_id(org_id).one(db).await?;
        if let Some(org) = org {
            if let Some(year_id) = org.current_year_id {
                return academic_year::Entity::find_by_id(year_id).one(db).await;
            }
        }
        Ok(None)
    }

    /// Create a new academic year record.
    pub async fn create(
        db: &DatabaseConnection,
        year: academic_year::Model,
    ) -> Result<academic_year::Model, DbErr> {
        let active: academic_year::ActiveModel = year.into();
        active.insert(db).await
    }

    /// Set a year as the current active year.
    pub async fn set_current(
        db: &DatabaseConnection,
        org_id: &str,
        year_id: &str,
    ) -> Result<(), DbErr> {
        use crate::entities::organization;
        use sea_orm::sea_query::Expr;

        organization::Entity::update_many()
            .col_expr(organization::Column::CurrentYearId, Expr::value(year_id))
            .filter(organization::Column::Id.eq(org_id))
            .exec(db)
            .await?;

        Ok(())
    }

    /// Update the status of an academic year.
    pub async fn update_status(
        db: &DatabaseConnection,
        year_id: &str,
        status: &str,
        archived_by: Option<&str>,
    ) -> Result<academic_year::Model, DbErr> {
        let year = academic_year::Entity::find_by_id(year_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("academic year not found".into()))?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut active: academic_year::ActiveModel = year.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(now.clone());

        if status == "archived" {
            active.archived_at = Set(Some(now));
            active.archived_by = Set(archived_by.map(|s| s.to_string()));
        }

        active.update(db).await
    }



    /// Check if a year name already exists for an organization.
    pub async fn name_exists(
        db: &DatabaseConnection,
        org_id: &str,
        name: &str,
    ) -> Result<bool, DbErr> {
        let count = academic_year::Entity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .filter(academic_year::Column::Name.eq(name))
            .count(db)
            .await?;
        Ok(count > 0)
    }
}
