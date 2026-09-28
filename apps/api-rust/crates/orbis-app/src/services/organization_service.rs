use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use orbis_domain::entities::organization::Organization;
use orbis_domain::entities::org_unit::OrgUnit;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::{
    OrganizationRepository, OrgUnitRepository, SettingsRepository,
    DashboardRepository, DashboardCounts,
};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;

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
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;

        let mut org = self.org_repo.find_by_id(org_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Organization {} not found", org_id)))?;

        if let Some(n) = req.name { org.name = n; }
        if let Some(na) = req.name_ar { org.name_ar = Some(na); }
        if let Some(cd) = req.code { org.code = Some(cd); }
        if let Some(t) = req.timezone { org.timezone = t; }
        if let Some(l) = req.locale { org.locale = l; }
        if let Some(df) = req.date_format { org.date_format = Some(df); }
        if let Some(h) = req.is_hijri_enabled { org.is_hijri_enabled = h; }
        if let Some(curr) = req.base_currency_code { org.base_currency_code = Some(curr); }
        if let Some(c) = req.primary_color { org.primary_color = Some(c); }
        if let Some(lg) = req.logo { org.logo = Some(lg); }
        if let Some(fav) = req.favicon { org.favicon = Some(fav); }
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
    pub async fn get_organization(&self, org_id: Uuid) -> Result<Option<Organization>, DomainError> {
        self.org_repo.find_by_id(org_id).await
    }

    /// Create an organizational unit (Branch, Campus, Department, etc.)
    pub async fn create_org_unit(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        unit_type: String,
        parent_id: Option<Uuid>,
        data_isolation: String,
    ) -> Result<OrgUnit, DomainError> {
        self.permission_checker.check(ctx, Permission::OrgUnitWrite).await?;

        let now = Utc::now();
        let unit_id = Uuid::new_v4();

        let (path, depth) = if let Some(pid) = parent_id {
            let parent = self.org_unit_repo.find_by_id(pid).await?
                .ok_or_else(|| DomainError::NotFound(format!("Parent OrgUnit {} not found", pid)))?;
            (format!("{}.{}", parent.path, unit_id), parent.depth + 1)
        } else {
            (unit_id.to_string(), 0)
        };

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
            settings: Some(serde_json::json!({})),
            local_policies: None,
            data_isolation,
            children_count: 0,
            descendant_count: 0,
            sort_order: Some(0),
            is_active: true,
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.org_unit_repo.save(&unit).await?;

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
        self.permission_checker.check(ctx, Permission::OrgUnitDelete).await?;
        self.org_unit_repo.delete(id).await
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
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;
        self.settings_repo.save_settings(org_id, &settings).await?;
        Ok(settings)
    }

    /// Get dashboard counts and metrics
    pub async fn get_dashboard_counts(&self, org_id: Uuid) -> Result<DashboardCounts, DomainError> {
        self.dashboard_repo.get_counts(org_id).await
    }
}
