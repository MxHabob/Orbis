use uuid::Uuid;
use chrono::Utc;
use sea_orm::{DbErr, ConnectionTrait};

use crate::entities::class;

impl super::Service {
    pub async fn get_classes(&self, org_id: &str, params: crate::handler::params::ClassParams) -> Result<Vec<class::Model>, String> {
        self.repo.get_classes(org_id, params).await.map_err(|e| e.to_string())
    }

    pub async fn create_class(
        &self,
        name: &str,
        name_ar: &str,
        level: &str,
        division: &str,
        org_id: &str,
    ) -> Result<class::Model, String> {
        // 1. Find current academic cycle
        let cycle = self.repo.get_current_cycle(org_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "no active academic cycle found".to_string())?;

        // 2. Ensure default OrgUnit exists
        let unit_id = Uuid::new_v4().to_string();
        let db = &self.repo.db;
        let query = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT OR IGNORE INTO org_units (id, org_id, name, type, path) VALUES (?, ?, 'Academic Department', 'academic', ?)",
            vec![unit_id.clone().into(), org_id.to_string().into(), unit_id.clone().into()],
        );
        let _ = db.execute(query).await;

        // 3. Create class
        let new_class = class::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            unit_id,
            cycle_id: cycle.id,
            name: name.to_string(),
            name_ar: Some(name_ar.to_string()),
            level: level.to_string(),
            division: Some(division.to_string()),
            homeroom_teacher_id: None,
            capacity: Some(30),
            current_students: Some(0),
            room_id: None,
            color: Some("#6366f1".to_string()),
            schedule_template: None,
            is_active: 1,
            is_deleted: 0,
            deleted_at: None,
            created_at: Utc::now().naive_utc().to_string(),
            updated_at: Utc::now().naive_utc().to_string(),
        };

        self.repo.create_class(new_class).await.map_err(|e| e.to_string())
    }
}
