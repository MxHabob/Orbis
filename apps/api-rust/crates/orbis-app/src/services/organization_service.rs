use chrono::Utc;
use orbis_domain::entities::org_unit::OrgUnit;
use orbis_domain::entities::organization::Organization;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    DashboardCounts, DashboardRepository, OrgUnitRepository, OrganizationRepository,
    SettingsRepository,
};

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct UpdateOrgRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub date_format: Option<String>,
    pub is_hijri_enabled: Option<bool>,
    pub base_currency_code: Option<String>,
    pub primary_color: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct CreateOrgUnitInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: String,
    pub parent_id: Option<Uuid>,
    pub head_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub room_number: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub local_policies: Option<serde_json::Value>,
    pub data_isolation: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct UpdateOrgUnitRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub head_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub room_number: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub local_policies: Option<serde_json::Value>,
    pub data_isolation: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

pub struct OrganizationService {
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
    pub dashboard_repo: Arc<dyn DashboardRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl OrganizationService {
    pub fn new(
        org_repo: Arc<dyn OrganizationRepository>,
        org_unit_repo: Arc<dyn OrgUnitRepository>,
        settings_repo: Arc<dyn SettingsRepository>,
        dashboard_repo: Arc<dyn DashboardRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            org_repo,
            org_unit_repo,
            settings_repo,
            dashboard_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Create a new tenant organization
    pub async fn create_organization(
        &self,
        ctx: &RequestContext,
        name: String,
        slug: String,
        timezone: String,
        locale: String,
    ) -> Result<Organization, DomainError> {
        let now = Utc::now();
        let org = Organization::new(name.clone(), slug, timezone, locale)?;
        let org_id = org.id;

        self.org_repo.save(&org).await?;

        let event = DomainEvent::OrganizationCreated {
            org_id,
            name,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(org)
    }

    /// Update organization profile
    pub async fn update_organization(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        req: UpdateOrgRequest,
    ) -> Result<Organization, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;

        let mut org =
            self.org_repo.find_by_id(org_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Organization {} not found", org_id))
            })?;

        if let Some(n) = req.name {
            org.name = n;
        }
        if let Some(na) = req.name_ar {
            org.name_ar = Some(na);
        }
        if let Some(cd) = req.code {
            org.code = Some(cd);
        }
        if let Some(t) = req.timezone {
            org.timezone = t;
        }
        if let Some(l) = req.locale {
            org.locale = l;
        }
        if let Some(df) = req.date_format {
            org.date_format = Some(df);
        }
        if let Some(h) = req.is_hijri_enabled {
            org.is_hijri_enabled = h;
        }
        if let Some(curr) = req.base_currency_code {
            org.base_currency_code = Some(curr);
        }
        if let Some(c) = req.primary_color {
            org.primary_color = Some(c);
        }
        if let Some(lg) = req.logo {
            org.logo = Some(lg);
        }
        if let Some(fav) = req.favicon {
            org.favicon = Some(fav);
        }
        org.updated_at = Utc::now();
        org.version += 1;

        self.org_repo.save(&org).await?;

        let event = DomainEvent::OrganizationUpdated {
            org_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(org)
    }

    /// Get organization by ID
    pub async fn get_organization(
        &self,
        org_id: Uuid,
    ) -> Result<Option<Organization>, DomainError> {
        self.org_repo.find_by_id(org_id).await
    }

    /// Create an organizational unit (Branch, Campus, Department, etc.)
    pub async fn create_org_unit(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        req: CreateOrgUnitInput,
    ) -> Result<OrgUnit, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrgUnitWrite)
            .await?;

        let now = Utc::now();
        let unit_id = Uuid::new_v4();

        let (path, depth) = if let Some(pid) = req.parent_id {
            let parent = self.org_unit_repo.find_by_id(pid).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Parent OrgUnit {} not found", pid))
            })?;
            (format!("{}.{}", parent.path, unit_id), parent.depth + 1)
        } else {
            (unit_id.to_string(), 0)
        };

        let unit = OrgUnit {
            id: unit_id,
            org_id,
            parent_id: req.parent_id,
            path,
            depth,
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            unit_type: req.unit_type.clone(),
            head_id: req.head_id,
            capacity: req.capacity,
            room_number: req.room_number,
            floor: req.floor,
            building: req.building,
            settings: req.settings.or_else(|| Some(serde_json::json!({}))),
            local_policies: req.local_policies,
            data_isolation: req.data_isolation.unwrap_or_else(|| "shared".to_string()),
            children_count: 0,
            descendant_count: 0,
            sort_order: req.sort_order.or(Some(0)),
            is_active: req.is_active.unwrap_or(true),
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.org_unit_repo.save(&unit).await?;

        let event = DomainEvent::OrgUnitCreated {
            unit_id,
            org_id,
            parent_id: req.parent_id,
            unit_type: req.unit_type,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(unit)
    }

    /// Update an organizational unit
    pub async fn update_org_unit(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        req: UpdateOrgUnitRequest,
    ) -> Result<OrgUnit, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrgUnitWrite)
            .await?;

        let mut unit = self
            .org_unit_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("OrgUnit {} not found", id)))?;

        let now = Utc::now();

        if let Some(name) = req.name {
            unit.name = name;
        }
        if let Some(name_ar) = req.name_ar {
            unit.name_ar = Some(name_ar);
        }
        if let Some(code) = req.code {
            unit.code = Some(code);
        }
        if let Some(unit_type) = req.unit_type {
            unit.unit_type = unit_type;
        }
        if let Some(head_id) = req.head_id {
            unit.head_id = Some(head_id);
        }
        if let Some(capacity) = req.capacity {
            unit.capacity = Some(capacity);
        }
        if let Some(building) = req.building {
            unit.building = Some(building);
        }
        if let Some(floor) = req.floor {
            unit.floor = Some(floor);
        }
        if let Some(room_number) = req.room_number {
            unit.room_number = Some(room_number);
        }
        if let Some(settings) = req.settings {
            unit.settings = Some(settings);
        }
        if let Some(local_policies) = req.local_policies {
            unit.local_policies = Some(local_policies);
        }
        if let Some(data_isolation) = req.data_isolation {
            unit.data_isolation = data_isolation;
        }
        if let Some(sort_order) = req.sort_order {
            unit.sort_order = Some(sort_order);
        }
        if let Some(is_active) = req.is_active {
            unit.is_active = is_active;
        }

        if let Some(new_parent_id) = req.parent_id {
            if Some(new_parent_id) != unit.parent_id {
                let parent = self
                    .org_unit_repo
                    .find_by_id(new_parent_id)
                    .await?
                    .ok_or_else(|| {
                        DomainError::NotFound(format!("Parent OrgUnit {} not found", new_parent_id))
                    })?;
                unit.parent_id = Some(new_parent_id);
                unit.path = format!("{}.{}", parent.path, unit.id);
                unit.depth = parent.depth + 1;
            }
        }

        unit.updated_by = ctx.user_id();
        unit.updated_at = now;

        self.org_unit_repo.save(&unit).await?;

        let event = DomainEvent::OrgUnitUpdated {
            unit_id: id,
            org_id: unit.org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(unit)
    }

    /// List org units for an organization
    pub async fn list_org_units(&self, org_id: Uuid) -> Result<Vec<OrgUnit>, DomainError> {
        self.org_unit_repo.find_by_org(org_id).await
    }

    /// Get org unit by ID
    pub async fn get_org_unit(&self, id: Uuid) -> Result<Option<OrgUnit>, DomainError> {
        self.org_unit_repo.find_by_id(id).await
    }

    /// Delete org unit
    pub async fn delete_org_unit(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrgUnitDelete)
            .await?;
        let unit = self
            .org_unit_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("OrgUnit {} not found", id)))?;

        self.org_unit_repo.delete(id).await?;

        let event = DomainEvent::OrgUnitDeleted {
            unit_id: id,
            org_id: unit.org_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }

    /// Get organization settings
    pub async fn get_settings(&self, org_id: Uuid) -> Result<OrganizationSettings, DomainError> {
        self.settings_repo.get_settings(org_id).await
    }

    /// Update organization settings
    pub async fn update_settings(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        settings: OrganizationSettings,
    ) -> Result<OrganizationSettings, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;
        self.settings_repo.save_settings(org_id, &settings).await?;
        Ok(settings)
    }

    /// Get dashboard counts and metrics
    pub async fn get_dashboard_counts(&self, org_id: Uuid) -> Result<DashboardCounts, DomainError> {
        self.dashboard_repo.get_counts(org_id).await
    }
}
