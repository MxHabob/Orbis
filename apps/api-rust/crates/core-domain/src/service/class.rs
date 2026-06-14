use uuid::Uuid;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, ConnectionTrait};

use crate::entities::class;
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateClassRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub division: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
    pub homeroom_teacher_id: Option<String>,
    pub is_active: Option<i32>,
}

impl super::Service {
    pub async fn get_classes(&self, org_id: &str, params: crate::params::ClassParams) -> Result<Vec<class::Model>, String> {
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

    pub async fn update_class(&self, id: &str, req: UpdateClassRequest) -> Result<class::Model, String> {
        let c = class::Entity::find_by_id(id.to_string())
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "class not found".to_string())?;

        let mut active: class::ActiveModel = c.into();

        if let Some(name) = req.name {
            active.name = Set(name);
        }
        if let Some(name_ar) = req.name_ar {
            active.name_ar = Set(Some(name_ar));
        }
        if let Some(level) = req.level {
            active.level = Set(level);
        }
        if let Some(division) = req.division {
            active.division = Set(Some(division));
        }
        if let Some(capacity) = req.capacity {
            active.capacity = Set(Some(capacity));
        }
        if let Some(color) = req.color {
            active.color = Set(Some(color));
        }
        if let Some(teacher_id) = req.homeroom_teacher_id {
            active.homeroom_teacher_id = Set(Some(teacher_id));
        }
        if let Some(is_active) = req.is_active {
            active.is_active = Set(is_active);
        }
        active.updated_at = Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_class(&self, id: &str) -> Result<(), String> {
        let c = class::Entity::find_by_id(id.to_string())
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "class not found".to_string())?;

        let mut active: class::ActiveModel = c.into();
        active.is_deleted = Set(1);
        active.deleted_at = Set(Some(Utc::now().to_rfc3339()));
        active.is_active = Set(0);
        active.updated_at = Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

