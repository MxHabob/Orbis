use crate::entities::org_unit;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    pub async fn get_org_units(
        &self,
        org_id: &str,
        params: crate::handler::params::OrgUnitParams,
    ) -> Result<Vec<org_unit::Model>, DbErr> {
        let mut query = org_unit::Entity::find()
            .filter(org_unit::Column::OrgId.eq(org_id));

        if let Some(ref unit_type) = params.unit_type {
            if !unit_type.trim().is_empty() {
                query = query.filter(org_unit::Column::UnitType.eq(unit_type));
            }
        }

        if let Some(ref parent_id) = params.parent_id {
            if !parent_id.trim().is_empty() {
                query = query.filter(org_unit::Column::ParentId.eq(parent_id));
            }
        }

        if let Some(is_active) = params.is_active {
            query = query.filter(org_unit::Column::IsActive.eq(is_active));
        }

        if let Some(ref search) = params.q {
            if !search.trim().is_empty() {
                let s = format!("%{}%", search);
                query = query.filter(
                    org_unit::Column::Name.like(&s)
                        .or(org_unit::Column::Code.like(&s))
                );
            }
        }

        query = query.order_by_asc(org_unit::Column::SortOrder);

        query.all(&self.db).await
    }

    pub async fn create_org_unit(&self, u: org_unit::Model) -> Result<org_unit::Model, DbErr> {
        let active_model = org_unit::ActiveModel {
            id: Set(u.id),
            org_id: Set(u.org_id),
            name: Set(u.name),
            name_ar: Set(u.name_ar),
            code: Set(u.code),
            unit_type: Set(u.unit_type),
            parent_id: Set(u.parent_id),
            path: Set(u.path),
            depth: Set(u.depth),
            head_id: Set(u.head_id),
            capacity: Set(u.capacity),
            room_number: Set(u.room_number),
            floor: Set(u.floor),
            building: Set(u.building),
            settings: Set(u.settings),
            sort_order: Set(u.sort_order),
            is_active: Set(u.is_active),
            created_at: Set(u.created_at),
            updated_at: Set(u.updated_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn update_org_unit(&self, id: &str, updates: OrgUnitUpdate) -> Result<org_unit::Model, DbErr> {
        let unit = org_unit::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("org unit not found".to_string()))?;

        let mut active: org_unit::ActiveModel = unit.into();

        if let Some(name) = updates.name {
            active.name = Set(name);
        }
        if let Some(name_ar) = updates.name_ar {
            active.name_ar = Set(Some(name_ar));
        }
        if let Some(unit_type) = updates.unit_type {
            active.unit_type = Set(unit_type);
        }
        if let Some(capacity) = updates.capacity {
            active.capacity = Set(Some(capacity));
        }
        if let Some(room_number) = updates.room_number {
            active.room_number = Set(Some(room_number));
        }
        if let Some(floor) = updates.floor {
            active.floor = Set(Some(floor));
        }
        if let Some(building) = updates.building {
            active.building = Set(Some(building));
        }
        if let Some(head_id) = updates.head_id {
            active.head_id = Set(Some(head_id));
        }
        if let Some(is_active) = updates.is_active {
            active.is_active = Set(is_active);
        }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    pub async fn delete_org_unit(&self, id: &str) -> Result<u64, DbErr> {
        let result = org_unit::Entity::delete_by_id(id.to_string())
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OrgUnitUpdate {
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
