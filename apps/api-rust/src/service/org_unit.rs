use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::entities::org_unit;
use crate::repository::org_unit::OrgUnitUpdate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateOrgUnitRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: String,
    pub parent_id: Option<String>,
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,
    pub head_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateOrgUnitRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub unit_type: Option<String>,
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,
    pub head_id: Option<String>,
    pub is_active: Option<i32>,
}

impl super::Service {
    pub async fn get_org_units(
        &self,
        org_id: &str,
        params: crate::handler::params::OrgUnitParams,
    ) -> Result<Vec<org_unit::Model>, String> {
        self.repo.get_org_units(org_id, params).await
            .map_err(|e| e.to_string())
    }

    pub async fn create_org_unit(&self, org_id: &str, req: CreateOrgUnitRequest) -> Result<org_unit::Model, String> {
        let id = Uuid::new_v4().to_string();
        let path = match &req.parent_id {
            Some(pid) => format!("{}/{}", pid, id),
            None => id.clone(),
        };
        let depth = path.matches('/').count() as i32;

        let unit = org_unit::Model {
            id,
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            unit_type: req.unit_type,
            parent_id: req.parent_id,
            path,
            depth,
            head_id: req.head_id,
            capacity: req.capacity,
            room_number: req.room_number,
            floor: req.floor,
            building: req.building,
            settings: Some("{}".to_string()),
            sort_order: Some(0),
            is_active: 1,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        self.repo.create_org_unit(unit).await
            .map_err(|e| e.to_string())
    }

    pub async fn update_org_unit(&self, id: &str, req: UpdateOrgUnitRequest) -> Result<org_unit::Model, String> {
        let updates = OrgUnitUpdate {
            name: req.name,
            name_ar: req.name_ar,
            unit_type: req.unit_type,
            capacity: req.capacity,
            room_number: req.room_number,
            floor: req.floor,
            building: req.building,
            head_id: req.head_id,
            is_active: req.is_active,
        };

        self.repo.update_org_unit(id, updates).await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_org_unit(&self, id: &str) -> Result<(), String> {
        let affected = self.repo.delete_org_unit(id).await
            .map_err(|e| e.to_string())?;
        if affected == 0 {
            return Err("org unit not found".to_string());
        }
        Ok(())
    }
}
