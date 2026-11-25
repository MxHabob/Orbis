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
use crate::ports::iam::{PermissionCheckerPort, PermissionScope};
use crate::ports::repositories::{
    DashboardCounts, DashboardRepository, OrgUnitRepository, OrganizationRepository,
    SettingsRepository,
};
use crate::services::calculation_engine::AcademicCalculationEngine;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct UpdateOrgRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub date_format: Option<String>,
    pub format_locale: Option<String>,
    pub time_format: Option<String>,
    pub is_hijri_enabled: Option<bool>,
    pub base_currency_code: Option<String>,
    pub primary_color: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub metadata: Option<serde_json::Value>,
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

#[derive(serde::Serialize)]
pub struct DashboardBranch {
    pub id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DashboardAccess {
    pub organization_wide: bool,
    pub units: Vec<DashboardBranch>,
}

fn descendants(units: &[OrgUnit], roots: &[Uuid]) -> std::collections::HashSet<Uuid> {
    let mut children = std::collections::HashMap::<Uuid, Vec<Uuid>>::new();
    let active: std::collections::HashSet<_> = units
        .iter()
        .filter(|unit| unit.is_active)
        .map(|unit| unit.id)
        .collect();
    for unit in units.iter().filter(|unit| unit.is_active) {
        if let Some(parent) = unit.parent_id {
            children.entry(parent).or_default().push(unit.id);
        }
    }
    let mut pending: Vec<_> = roots
        .iter()
        .copied()
        .filter(|id| active.contains(id))
        .collect();
    let mut visited = std::collections::HashSet::new();
    while let Some(id) = pending.pop() {
        if visited.insert(id) {
            pending.extend(children.get(&id).into_iter().flatten().copied());
        }
    }
    visited
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
        if let Some(fl) = req.format_locale {
            org.format_locale = Some(fl);
        }
        if let Some(tf) = req.time_format {
            org.time_format = Some(tf);
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
        if let Some(metadata) = req.metadata {
            let mut merged = org.metadata.take().unwrap_or_else(|| serde_json::json!({}));
            if let (Some(existing), Some(incoming)) = (merged.as_object_mut(), metadata.as_object())
            {
                existing.extend(incoming.clone());
            } else {
                merged = metadata;
            }
            org.metadata = Some(merged);
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
        if org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden(
                "Cannot create a branch in another organization".into(),
            ));
        }

        let now = Utc::now();
        let unit_id = Uuid::new_v4();

        let (path, depth) = if let Some(pid) = req.parent_id {
            let parent = self
                .authorized_unit(ctx, pid, Permission::OrgUnitWrite)
                .await?;
            (format!("{}.{}", parent.path, unit_id), parent.depth + 1)
        } else {
            if self.operation_scope(ctx, Permission::OrgUnitWrite).await?
                != PermissionScope::Organization
            {
                return Err(DomainError::Forbidden(
                    "Creating a root unit requires organization-wide permission".into(),
                ));
            }
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
        let mut unit = self
            .authorized_unit(ctx, id, Permission::OrgUnitWrite)
            .await?;

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
                    .authorized_unit(ctx, new_parent_id, Permission::OrgUnitWrite)
                    .await?;
                if parent.id == unit.id
                    || !self.org_unit_repo.find_children(unit.id).await?.is_empty()
                {
                    return Err(DomainError::Validation(
                        "Only leaf units can be moved to a different parent".into(),
                    ));
                }
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
    pub async fn list_org_units(&self, ctx: &RequestContext) -> Result<Vec<OrgUnit>, DomainError> {
        let scope = self.operation_scope(ctx, Permission::OrgUnitRead).await?;
        let units = self.org_unit_repo.find_by_org(ctx.tenant_id).await?;
        Ok(dashboard_units(&scope, &units)
            .into_iter()
            .cloned()
            .collect())
    }

    /// Get org unit by ID
    pub async fn get_org_unit(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<Option<OrgUnit>, DomainError> {
        match self.authorized_unit(ctx, id, Permission::OrgUnitRead).await {
            Ok(unit) => Ok(Some(unit)),
            Err(DomainError::NotFound(_)) => Ok(None),
            Err(error) => Err(error),
        }
    }

    /// Delete org unit
    pub async fn delete_org_unit(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        let unit = self
            .authorized_unit(ctx, id, Permission::OrgUnitDelete)
            .await?;

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
        mut settings: OrganizationSettings,
    ) -> Result<OrganizationSettings, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;

        if settings.finance.cashier_cash_accounts.is_none() {
            settings.finance.cashier_cash_accounts = self
                .settings_repo
                .get_settings(org_id)
                .await?
                .finance
                .cashier_cash_accounts;
        }

        if settings.student_sort.then_by.len() > 2 {
            return Err(DomainError::Validation(
                "Student sorting supports up to three criteria".into(),
            ));
        }
        let mut sort_fields = vec![settings.student_sort.field];
        for criterion in &settings.student_sort.then_by {
            if sort_fields.contains(&criterion.field) {
                return Err(DomainError::Validation(
                    "Student sort criteria must use distinct fields".into(),
                ));
            }
            sort_fields.push(criterion.field);
        }

        if let Some(plan) = settings.academic.calculation_plan.as_mut() {
            AcademicCalculationEngine::validate_plan(plan)?;
            let current = self.settings_repo.get_settings(org_id).await?;
            plan.version = current
                .academic
                .calculation_plan
                .as_ref()
                .map_or(Some(1), |active| active.version.checked_add(1))
                .ok_or_else(|| {
                    DomainError::Validation("Calculation policy version overflow".into())
                })?;
        }

        if let Some(outcome_policy) = settings.academic.outcome_policy.as_ref() {
            let plan = settings.academic.calculation_plan.as_ref().ok_or_else(|| {
                DomainError::Validation(
                    "An outcome policy requires an active calculation plan".into(),
                )
            })?;
            if !plan
                .outputs
                .iter()
                .any(|output| output.unit == outcome_policy.unit)
            {
                return Err(DomainError::Validation(
                    "Outcome policy unit must match at least one calculation output".into(),
                ));
            }
            AcademicCalculationEngine::evaluate_outcome(
                rust_decimal::Decimal::ZERO,
                &outcome_policy.unit,
                outcome_policy,
            )?;
        }

        self.settings_repo.save_settings(org_id, &settings).await?;
        Ok(settings)
    }

    async fn operation_scope(
        &self,
        ctx: &RequestContext,
        permission: Permission,
    ) -> Result<PermissionScope, DomainError> {
        let actor_id = ctx
            .actor
            .id()
            .ok_or_else(|| DomainError::Unauthorized("Authentication required".into()))?;
        self.permission_checker
            .permission_scope(actor_id, ctx.tenant_id, permission.as_str())
            .await
    }

    async fn authorized_unit(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        permission: Permission,
    ) -> Result<OrgUnit, DomainError> {
        let missing = || DomainError::NotFound(format!("OrgUnit {id} not found"));
        let unit = self
            .org_unit_repo
            .find_by_id(id)
            .await?
            .filter(|unit| unit.org_id == ctx.tenant_id)
            .ok_or_else(missing)?;
        if let PermissionScope::Units(roots) = self.operation_scope(ctx, permission).await? {
            let units = self.org_unit_repo.find_by_org(ctx.tenant_id).await?;
            if !descendants(&units, &roots).contains(&id) {
                return Err(missing());
            }
        }
        Ok(unit)
    }

    pub async fn get_dashboard_access(
        &self,
        ctx: &RequestContext,
    ) -> Result<DashboardAccess, DomainError> {
        let scope = self.operation_scope(ctx, Permission::DashboardRead).await?;
        let units = self.org_unit_repo.find_by_org(ctx.tenant_id).await?;
        let mut visible: Vec<_> = dashboard_units(&scope, &units)
            .into_iter()
            .filter(|unit| {
                matches!(unit.unit_type.as_str(), "branch" | "campus")
                    || matches!(&scope, PermissionScope::Units(roots) if roots.contains(&unit.id))
            })
            .collect();
        visible.sort_by(|left, right| left.name.cmp(&right.name).then(left.id.cmp(&right.id)));
        Ok(DashboardAccess {
            organization_wide: scope == PermissionScope::Organization,
            units: visible
                .into_iter()
                .map(|unit| DashboardBranch {
                    id: unit.id,
                    name: unit.name.clone(),
                    name_ar: unit.name_ar.clone(),
                })
                .collect(),
        })
    }

    pub async fn get_dashboard_counts(
        &self,
        ctx: &RequestContext,
        selected_unit: Option<Uuid>,
    ) -> Result<DashboardCounts, DomainError> {
        let scope = self.operation_scope(ctx, Permission::DashboardRead).await?;
        if !scope.has_access() {
            return Err(DomainError::Forbidden(
                "Dashboard access is not granted".into(),
            ));
        }
        let finance = self.operation_scope(ctx, Permission::FinanceRead).await?;
        if selected_unit.is_none() && scope == PermissionScope::Organization {
            return self
                .dashboard_repo
                .get_counts(
                    ctx.tenant_id,
                    None,
                    finance == PermissionScope::Organization,
                )
                .await;
        }

        let units = self.org_unit_repo.find_by_org(ctx.tenant_id).await?;
        let accessible = dashboard_units(&scope, &units);
        let selected = selected_unit
            .map(|id| {
                accessible
                    .iter()
                    .find(|unit| unit.id == id)
                    .copied()
                    .ok_or_else(|| {
                        DomainError::Forbidden(
                            "The selected branch is outside your dashboard scope".into(),
                        )
                    })
            })
            .transpose()?;
        let selected_descendants = selected.map(|root| descendants(&units, &[root.id]));
        let unit_ids: Vec<_> = accessible
            .iter()
            .filter(|unit| {
                selected_descendants
                    .as_ref()
                    .is_none_or(|ids| ids.contains(&unit.id))
            })
            .map(|unit| unit.id)
            .collect();
        if unit_ids.is_empty() {
            return Err(DomainError::Forbidden(
                "No active branch grants dashboard access".into(),
            ));
        }
        let finance_units: std::collections::HashSet<_> = dashboard_units(&finance, &units)
            .into_iter()
            .map(|unit| unit.id)
            .collect();
        let include_finance = unit_ids.iter().all(|id| finance_units.contains(id));
        self.dashboard_repo
            .get_counts(ctx.tenant_id, Some(&unit_ids), include_finance)
            .await
    }
}

fn dashboard_units<'a>(scope: &PermissionScope, units: &'a [OrgUnit]) -> Vec<&'a OrgUnit> {
    let active: Vec<_> = units.iter().filter(|unit| unit.is_active).collect();
    let PermissionScope::Units(ids) = scope else {
        return active;
    };
    let allowed = descendants(units, ids);
    active
        .into_iter()
        .filter(|unit| allowed.contains(&unit.id))
        .collect()
}
