use async_trait::async_trait;
use orbis_app::ports::repositories::OrgUnitRepository;
use orbis_domain::entities::org_unit::OrgUnit as DomainOrgUnit;
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use uuid::Uuid;

use crate::models::org_unit::{self, Entity as OrgUnitEntity};

pub struct OrgUnitRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl OrgUnitRepository for OrgUnitRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainOrgUnit>, DomainError> {
        let model = OrgUnitEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(into_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainOrgUnit>, DomainError> {
        let models = OrgUnitEntity::find()
            .filter(org_unit::Column::OrgId.eq(org_id))
            .filter(org_unit::Column::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(into_domain).collect())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        OrgUnitEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn find_children(&self, parent_id: Uuid) -> Result<Vec<DomainOrgUnit>, DomainError> {
        let models = OrgUnitEntity::find()
            .filter(org_unit::Column::ParentId.eq(parent_id))
            .filter(org_unit::Column::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(into_domain).collect())
    }

    async fn find_descendants(
        &self,
        org_id: Uuid,
        path_prefix: &str,
    ) -> Result<Vec<DomainOrgUnit>, DomainError> {
        let models = OrgUnitEntity::find()
            .filter(org_unit::Column::OrgId.eq(org_id))
            .filter(org_unit::Column::IsActive.eq(true))
            .filter(
                org_unit::Column::Path
                    .starts_with(format!("{}.", path_prefix))
                    .or(org_unit::Column::Path.starts_with(format!("{}/", path_prefix))),
            )
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(into_domain).collect())
    }

    async fn count_branches(&self, org_id: Uuid) -> Result<u32, DomainError> {
        let count = OrgUnitEntity::find()
            .filter(org_unit::Column::OrgId.eq(org_id))
            .filter(org_unit::Column::UnitType.eq("branch"))
            .filter(org_unit::Column::IsActive.eq(true))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(count as u32)
    }

    async fn save(&self, unit: &DomainOrgUnit) -> Result<(), DomainError> {
        let active_model = org_unit::ActiveModel {
            id: Set(unit.id),
            org_id: Set(unit.org_id),
            parent_id: Set(unit.parent_id),
            path: Set(unit.path.clone()),
            depth: Set(unit.depth),
            name: Set(unit.name.clone()),
            name_ar: Set(unit.name_ar.clone()),
            code: Set(unit.code.clone()),
            unit_type: Set(unit.unit_type.clone()),
            head_id: Set(unit.head_id),
            capacity: Set(unit.capacity),
            room_number: Set(unit.room_number.clone()),
            floor: Set(unit.floor),
            building: Set(unit.building.clone()),
            settings: Set(unit.settings.clone()),
            local_policies: Set(unit.local_policies.clone()),
            data_isolation: Set(unit.data_isolation.clone()),
            children_count: Set(unit.children_count),
            descendant_count: Set(unit.descendant_count),
            sort_order: Set(unit.sort_order),
            is_active: Set(unit.is_active),
            created_at: Set(unit.created_at),
            updated_at: Set(unit.updated_at),
            ..Default::default()
        };

        if OrgUnitEntity::find_by_id(unit.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }
}

fn into_domain(m: org_unit::Model) -> DomainOrgUnit {
    DomainOrgUnit {
        id: m.id,
        org_id: m.org_id,
        parent_id: m.parent_id,
        path: m.path,
        depth: m.depth,
        name: m.name,
        name_ar: m.name_ar,
        code: m.code,
        unit_type: m.unit_type,
        head_id: m.head_id,
        capacity: m.capacity,
        room_number: m.room_number,
        floor: m.floor,
        building: m.building,
        settings: m.settings,
        local_policies: m.local_policies,
        data_isolation: m.data_isolation,
        children_count: m.children_count,
        descendant_count: m.descendant_count,
        sort_order: m.sort_order,
        is_active: m.is_active,
        created_by: m.created_by,
        updated_by: m.updated_by,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}
