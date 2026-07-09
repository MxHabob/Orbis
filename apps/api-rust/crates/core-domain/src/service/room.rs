use chrono::Utc;
use infra_scheduler::RoomDto;
use uuid::Uuid;

use crate::entities::room;
use crate::service::schedule_common::room_entity_to_dto;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct CreateRoomRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub capacity: Option<i32>,
    #[serde(default = "default_room_type")]
    pub room_type: String,
    pub building: Option<String>,
    pub floor: Option<i32>,
}

fn default_room_type() -> String {
    "classroom".to_string()
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub capacity: Option<i32>,
    pub room_type: Option<String>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub is_active: Option<i32>,
}

impl super::Service {
    pub async fn get_rooms(&self, org_id: &str) -> Result<Vec<room::Model>, String> {
        self.repo.get_rooms(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_rooms_for_solver(&self, org_id: &str) -> Result<Vec<RoomDto>, String> {
        let rooms = self.get_rooms(org_id).await?;
        Ok(rooms.iter().map(room_entity_to_dto).collect())
    }

    pub async fn create_room(
        &self,
        org_id: &str,
        req: CreateRoomRequest,
    ) -> Result<room::Model, String> {
        let now = Utc::now().to_rfc3339();
        let model = room::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            capacity: req.capacity,
            room_type: req.room_type,
            building: req.building,
            floor: req.floor,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        self.repo.insert_room(model).await.map_err(|e| e.to_string())
    }

    pub async fn update_room(
        &self,
        org_id: &str,
        id: &str,
        req: UpdateRoomRequest,
    ) -> Result<room::Model, String> {
        let existing = self
            .repo
            .get_room_by_id(id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "room not found".to_string())?;

        if existing.org_id != org_id {
            return Err("room not found".to_string());
        }

        self.repo
            .update_room(id, crate::repository::schedule::RoomUpdate {
                name: req.name,
                name_ar: req.name_ar,
                capacity: req.capacity,
                room_type: req.room_type,
                building: req.building,
                floor: req.floor,
                is_active: req.is_active,
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_room(&self, org_id: &str, id: &str) -> Result<(), String> {
        let existing = self
            .repo
            .get_room_by_id(id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "room not found".to_string())?;

        if existing.org_id != org_id {
            return Err("room not found".to_string());
        }

        self.repo.soft_delete_room(id).await.map_err(|e| e.to_string())
    }
}
