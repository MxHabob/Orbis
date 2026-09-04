use orbis_domain::entities::org_unit::OrgUnit;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::OrgUnitRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct CreateOrgUnitUseCase {
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateOrgUnitUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        unit_type: String,
        parent_id: Option<Uuid>,
        data_isolation: String,
    ) -> Result<OrgUnit, DomainError> {
        // Find parent if exists
        let (depth, path) = if let Some(pid) = parent_id {
            let parent = self.org_unit_repo.find_by_id(pid).await?
                .ok_or_else(|| DomainError::NotFound("Parent OrgUnit not found".to_string()))?;
            (parent.depth + 1, format!("{}/{}", parent.path, pid))
        } else {
            (0, "/".to_string())
        };

        if unit_type == "branch" {
            let count = self.org_unit_repo.count_branches(org_id).await?;
            if count >= 10 { // Example policy limit
                return Err(DomainError::BusinessRule("Branch limit reached".to_string()));
            }
        }

        let now = Utc::now();
        let unit_id = Uuid::new_v4();

        let unit = OrgUnit {
            id: unit_id,
            org_id,
            parent_id,
            path,
            depth,
            name,
            name_ar: None,
            code: None,
            unit_type: unit_type.clone(),
            head_id: None,
            capacity: None,
            room_number: None,
            floor: None,
            building: None,
            settings: None,
            local_policies: None,
            data_isolation,
            children_count: 0,
            descendant_count: 0,
            sort_order: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.org_unit_repo.save(&unit).await?;

        // Update parent children count logic should go here or be handled by the repo/event handler

        let event = DomainEvent::OrgUnitCreated {
            unit_id,
            org_id,
            parent_id,
            unit_type,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(unit)
    }
}
