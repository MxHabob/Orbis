use chrono::{DateTime, Duration, Utc};
use orbis_domain::entities::org_unit::OrgUnit;
use orbis_domain::entities::override_request::{
    AcademicOverrideRequest, ApprovalStep, OverrideStatus, OverrideType, RiskLevel,
};
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::actor::Actor;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{PaginatedList, PaginationParams};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::{PermissionCheckerPort, PermissionScope};
use crate::ports::repositories::{
    OrgUnitRepository, OverrideRequestFilter, OverrideRequestRepository,
};

const POLICY_VERSION: i32 = 1;
const BRANCH_REVIEW_HOURS: i64 = 48;
const REQUEST_VALID_DAYS: i64 = 14;

#[derive(Debug, Clone)]
pub struct SubmitOverrideInput {
    pub override_type: OverrideType,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub period_id: Option<Uuid>,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
}

pub struct WorkflowService {
    override_repo: Arc<dyn OverrideRequestRepository>,
    org_unit_repo: Arc<dyn OrgUnitRepository>,
    event_bus: Arc<dyn EventBus>,
    permission_checker: Arc<dyn PermissionCheckerPort>,
}

struct Access {
    scope: PermissionScope,
    units: HashMap<Uuid, Option<Uuid>>,
}

impl Access {
    fn allows(&self, targets: &[Option<Uuid>]) -> bool {
        match &self.scope {
            PermissionScope::Organization => !targets.is_empty(),
            PermissionScope::Units(roots) => {
                !targets.is_empty()
                    && targets.iter().all(|target| {
                        let Some(mut current) = *target else {
                            return false;
                        };
                        let mut visited = HashSet::new();
                        while visited.insert(current) {
                            let Some(parent) = self.units.get(&current) else {
                                return false;
                            };
                            if roots.contains(&current) {
                                return true;
                            }
                            let Some(parent) = parent else { return false };
                            current = *parent;
                        }
                        false
                    })
            }
        }
    }
}

fn human(ctx: &RequestContext) -> Result<Uuid, DomainError> {
    match ctx.actor {
        Actor::User(user_id) => Ok(user_id),
        _ => Err(DomainError::Forbidden(
            "A signed-in user is required for override workflows".into(),
        )),
    }
}

fn missing_request() -> DomainError {
    DomainError::NotFound("Override request not found".into())
}

fn matches_override_search(request: &AcademicOverrideRequest, search: &str) -> bool {
    let terms = search
        .split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>();
    if terms.is_empty() {
        return true;
    }

    let override_type = request.override_type.to_string();
    let status = request.status.to_string();
    let searchable = [
        request.reason.as_str(),
        request.requested_by_name.as_deref().unwrap_or_default(),
        override_type.as_str(),
        request.entity_type.as_str(),
        request.field_name.as_deref().unwrap_or_default(),
        request.old_value.as_deref().unwrap_or_default(),
        request.new_value.as_deref().unwrap_or_default(),
        status.as_str(),
        request.resolved_by_name.as_deref().unwrap_or_default(),
        request.resolution_notes.as_deref().unwrap_or_default(),
    ]
    .join(" ")
    .to_lowercase();
    terms.iter().all(|term| searchable.contains(term))
}

fn policy(kind: OverrideType, assigned_to_unit: bool) -> (RiskLevel, &'static [&'static str]) {
    let risk = match kind {
        OverrideType::GradeChange
        | OverrideType::DiscountOverride
        | OverrideType::AcademicStandingOverride
        | OverrideType::Custom(_) => RiskLevel::High,
        OverrideType::AdmissionApproval | OverrideType::LateEnrollment => RiskLevel::Low,
        _ => RiskLevel::Medium,
    };
    match (risk, assigned_to_unit) {
        (RiskLevel::High | RiskLevel::Critical, true) => (risk, &["principal", "org_admin"]),
        (RiskLevel::High | RiskLevel::Critical, false) => (risk, &["org_admin", "super_admin"]),
        (_, true) => (risk, &["principal"]),
        (_, false) => (risk, &["org_admin"]),
    }
}

fn pending_step(order: usize, role: &str) -> ApprovalStep {
    ApprovalStep {
        step_order: order as u32,
        approver_role: role.to_owned(),
        approver_id: None,
        approver_name: None,
        status: OverrideStatus::Pending,
        decision_at: None,
        notes: None,
    }
}

fn apply_policy(request: &mut AcademicOverrideRequest, targets: &[Option<Uuid>]) {
    let (risk, reviewers) = policy(request.override_type, targets.iter().all(Option::is_some));
    request.risk_level = risk;
    request.approval_chain = reviewers
        .iter()
        .enumerate()
        .map(|(index, role)| pending_step(index + 1, role))
        .collect();
    request.current_approver_role = reviewers.first().map(|role| (*role).to_owned());
    request.approval_policy_version = Some(POLICY_VERSION);
}

fn prepare_pending(
    request: &mut AcademicOverrideRequest,
    targets: &[Option<Uuid>],
    now: DateTime<Utc>,
) {
    if request.status != OverrideStatus::Pending {
        return;
    }
    if request
        .expires_at
        .unwrap_or(request.created_at + Duration::days(REQUEST_VALID_DAYS))
        <= now
    {
        request.status = OverrideStatus::Expired;
        request.current_approver_role = None;
        return;
    }
    if request.approval_policy_version != Some(POLICY_VERSION) {
        // Pending records submitted before the server policy receive a fresh review.
        apply_policy(request, targets);
    }
    let Some(index) = request
        .approval_chain
        .iter()
        .position(|step| step.status == OverrideStatus::Pending)
    else {
        return;
    };
    let step_started = request.approval_chain[..index]
        .iter()
        .filter_map(|step| step.decision_at)
        .max()
        .unwrap_or(request.created_at);
    let escalation_at = step_started + Duration::hours(BRANCH_REVIEW_HOURS);
    if request.approval_chain[index].approver_role == "principal" && now >= escalation_at {
        let replacement = if request.approval_chain[index + 1..]
            .iter()
            .any(|step| step.approver_role == "org_admin")
        {
            "super_admin"
        } else {
            "org_admin"
        };
        let step = &mut request.approval_chain[index];
        step.status = OverrideStatus::Expired;
        step.decision_at = Some(escalation_at);
        step.notes =
            Some("Branch review deadline elapsed; routed to central administration".into());
        request
            .approval_chain
            .insert(index + 1, pending_step(index + 2, replacement));
        for (index, step) in request.approval_chain.iter_mut().enumerate() {
            step.step_order = (index + 1) as u32;
        }
    }
    request.current_approver_role = request
        .approval_chain
        .iter()
        .find(|step| step.status == OverrideStatus::Pending)
        .map(|step| step.approver_role.clone());
}

fn ensure_independent_reviewer(
    request: &AcademicOverrideRequest,
    reviewer: Uuid,
) -> Result<(), DomainError> {
    if request.requested_by == reviewer {
        return Err(DomainError::Forbidden(
            "You cannot decide your own override request".into(),
        ));
    }
    if request
        .approval_chain
        .iter()
        .any(|step| step.status == OverrideStatus::Approved && step.approver_id == Some(reviewer))
    {
        return Err(DomainError::Forbidden(
            "A different reviewer must decide the next step".into(),
        ));
    }
    Ok(())
}

impl WorkflowService {
    pub fn new(
        override_repo: Arc<dyn OverrideRequestRepository>,
        org_unit_repo: Arc<dyn OrgUnitRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            override_repo,
            org_unit_repo,
            event_bus,
            permission_checker,
        }
    }

    async fn access(
        &self,
        ctx: &RequestContext,
        permission: Permission,
    ) -> Result<Access, DomainError> {
        let scope = self
            .permission_checker
            .permission_scope(human(ctx)?, ctx.tenant_id, permission.as_str())
            .await?;
        if !scope.has_access() {
            return Err(DomainError::Forbidden(format!(
                "Missing permission: {}",
                permission.as_str()
            )));
        }
        let units = self
            .org_unit_repo
            .find_by_org(ctx.tenant_id)
            .await?
            .into_iter()
            .filter(|unit| unit.is_active)
            .map(|OrgUnit { id, parent_id, .. }| (id, parent_id))
            .collect();
        Ok(Access { scope, units })
    }

    async fn targets(
        &self,
        request: &AcademicOverrideRequest,
    ) -> Result<Vec<Option<Uuid>>, DomainError> {
        let internal_admission = matches!(
            request.override_type,
            OverrideType::AdmissionApproval | OverrideType::CapacityOverride
        );
        let entity_type = if internal_admission {
            "student"
        } else {
            request.entity_type.as_str()
        };
        let section_id = if internal_admission {
            request
                .metadata
                .get("class_id")
                .and_then(serde_json::Value::as_str)
                .map(|id| Uuid::parse_str(id).map_err(|_| missing_request()))
                .transpose()?
        } else {
            None
        };
        let period_id = request
            .metadata
            .get("period_id")
            .and_then(serde_json::Value::as_str)
            .map(Uuid::parse_str)
            .transpose()
            .map_err(|_| missing_request())?;
        self.override_repo
            .target_units(
                request.org_id,
                entity_type,
                request.entity_id,
                section_id,
                period_id,
            )
            .await
    }

    pub async fn submit_override(
        &self,
        ctx: &RequestContext,
        input: SubmitOverrideInput,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        let requested_by = human(ctx)?;
        let access = self.access(ctx, Permission::WorkflowWrite).await?;
        if matches!(
            input.override_type,
            OverrideType::AdmissionApproval | OverrideType::CapacityOverride
        ) {
            return Err(DomainError::Validation(
                "Submit admission and capacity requests through enrollment".into(),
            ));
        }
        let valid_target = match input.override_type {
            OverrideType::GradeChange => {
                input.entity_type == "grade"
                    || (input.entity_type == "section_subject" && input.period_id.is_some())
            }
            OverrideType::DiscountOverride => {
                matches!(input.entity_type.as_str(), "fee" | "student_fee")
            }
            OverrideType::AcademicStandingOverride | OverrideType::AgeBypass => {
                input.entity_type == "student"
            }
            OverrideType::LateEnrollment => {
                matches!(input.entity_type.as_str(), "student" | "enrollment")
                    || (matches!(input.entity_type.as_str(), "section" | "academic_period")
                        && input.period_id.is_some())
            }
            OverrideType::PrerequisiteBypass | OverrideType::CreditOverload => {
                matches!(input.entity_type.as_str(), "student" | "enrollment")
            }
            _ => true,
        };
        if !valid_target {
            return Err(DomainError::Validation(
                "The target type does not match this override".into(),
            ));
        }
        let reason = input.reason.trim();
        if reason.is_empty() {
            return Err(DomainError::Validation(
                "An override reason is required".into(),
            ));
        }
        let now = Utc::now();
        let mut request = AcademicOverrideRequest {
            id: Uuid::new_v4(),
            org_id: ctx.tenant_id,
            override_type: input.override_type,
            risk_level: RiskLevel::Medium,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            field_name: input.field_name,
            old_value: input.old_value,
            new_value: input.new_value,
            reason: reason.to_owned(),
            requested_by,
            requested_by_name: None,
            status: OverrideStatus::Pending,
            current_approver_role: None,
            approval_chain: Vec::new(),
            approval_policy_version: None,
            resolved_by: None,
            resolved_by_name: None,
            resolved_at: None,
            resolution_notes: None,
            expires_at: Some(now + Duration::days(REQUEST_VALID_DAYS)),
            metadata: input.period_id.map_or_else(
                || serde_json::json!({}),
                |id| serde_json::json!({ "period_id": id }),
            ),
            created_at: now,
            updated_at: now,
        };
        let targets = self.targets(&request).await?;
        if !access.allows(&targets) {
            return Err(DomainError::Forbidden(
                "Override target is outside your permission scope".into(),
            ));
        }
        apply_policy(&mut request, &targets);
        self.override_repo.save(&request).await?;
        self.event_bus
            .publish(
                ctx,
                &DomainEvent::OverrideSubmitted {
                    request_id: request.id,
                    org_id: request.org_id,
                    override_type: request.override_type.to_string(),
                    requested_by,
                    timestamp: now,
                },
            )
            .await?;
        Ok(request)
    }

    pub async fn approve_override(
        &self,
        ctx: &RequestContext,
        request_id: Uuid,
        notes: Option<String>,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        self.decide(ctx, request_id, OverrideStatus::Approved, notes)
            .await
    }

    pub async fn reject_override(
        &self,
        ctx: &RequestContext,
        request_id: Uuid,
        reason: String,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        let reason = reason.trim();
        if reason.is_empty() {
            return Err(DomainError::Validation(
                "A rejection reason is required".into(),
            ));
        }
        self.decide(
            ctx,
            request_id,
            OverrideStatus::Rejected,
            Some(reason.to_owned()),
        )
        .await
    }

    async fn decide(
        &self,
        ctx: &RequestContext,
        request_id: Uuid,
        decision: OverrideStatus,
        notes: Option<String>,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        let reviewer = human(ctx)?;
        let access = self.access(ctx, Permission::WorkflowApprove).await?;
        let mut request = self
            .override_repo
            .find_by_id(request_id)
            .await?
            .filter(|request| request.org_id == ctx.tenant_id)
            .ok_or_else(missing_request)?;
        let targets = self.targets(&request).await?;
        if !access.allows(&targets) {
            return Err(missing_request());
        }
        let now = Utc::now();
        prepare_pending(&mut request, &targets, now);
        if request.status != OverrideStatus::Pending {
            return Err(DomainError::Conflict(
                "The override is expired or already resolved".into(),
            ));
        }
        ensure_independent_reviewer(&request, reviewer)?;
        let step = request
            .approval_chain
            .iter_mut()
            .find(|step| step.status == OverrideStatus::Pending)
            .ok_or_else(|| {
                DomainError::Conflict("The override has no pending review step".into())
            })?;
        let role_scope = self
            .permission_checker
            .role_scope(reviewer, ctx.tenant_id, &step.approver_role)
            .await?;
        if step.approver_role != "principal"
            && (role_scope != PermissionScope::Organization
                || access.scope != PermissionScope::Organization)
        {
            return Err(DomainError::Forbidden(
                "This step requires a central administrator".into(),
            ));
        }
        if !(Access {
            scope: role_scope,
            units: access.units,
        })
        .allows(&targets)
        {
            return Err(DomainError::Forbidden(
                "You are not the authorized reviewer for this step".into(),
            ));
        }
        step.status = decision;
        step.approver_id = Some(reviewer);
        step.decision_at = Some(now);
        step.notes = notes.clone();
        let next = request
            .approval_chain
            .iter()
            .find(|step| step.status == OverrideStatus::Pending);
        if decision == OverrideStatus::Rejected || next.is_none() {
            request.status = decision;
            request.resolved_by = Some(reviewer);
            request.resolved_at = Some(now);
            request.resolution_notes = notes.clone();
            request.current_approver_role = None;
        } else {
            request.current_approver_role = next.map(|step| step.approver_role.clone());
        }
        let expected = request.updated_at;
        request.updated_at = now.max(expected + Duration::microseconds(1));
        self.override_repo.save_decision(&request, expected).await?;
        let event = match request.status {
            OverrideStatus::Approved => Some(DomainEvent::OverrideApproved {
                request_id,
                org_id: request.org_id,
                approver_id: reviewer,
                timestamp: now,
            }),
            OverrideStatus::Rejected => Some(DomainEvent::OverrideRejected {
                request_id,
                org_id: request.org_id,
                rejecter_id: reviewer,
                reason: notes.unwrap_or_default(),
                timestamp: now,
            }),
            _ => None,
        };
        if let Some(event) = event {
            self.event_bus.publish(ctx, &event).await?;
        }
        Ok(request)
    }

    async fn visible(
        &self,
        access: &Access,
        mut request: AcademicOverrideRequest,
        now: DateTime<Utc>,
    ) -> Result<Option<AcademicOverrideRequest>, DomainError> {
        if access.scope == PermissionScope::Organization
            && request.status != OverrideStatus::Pending
        {
            return Ok(Some(request));
        }
        let targets = match self.targets(&request).await {
            Ok(targets) => targets,
            Err(DomainError::NotFound(_) | DomainError::Validation(_)) => return Ok(None),
            Err(error) => return Err(error),
        };
        if !access.allows(&targets) {
            return Ok(None);
        }
        prepare_pending(&mut request, &targets, now);
        Ok(Some(request))
    }

    async fn filter_visible_override_requests(
        &self,
        requester: Uuid,
        access: Option<&Access>,
        requests: Vec<AcademicOverrideRequest>,
        now: DateTime<Utc>,
        status: Option<&str>,
        role: Option<&str>,
    ) -> Result<Vec<AcademicOverrideRequest>, DomainError> {
        let mut visible = Vec::new();
        for mut request in requests {
            let request = if request.requested_by == requester {
                match self.targets(&request).await {
                    Ok(targets) => prepare_pending(&mut request, &targets, now),
                    Err(DomainError::NotFound(_) | DomainError::Validation(_)) => {}
                    Err(error) => return Err(error),
                }
                Some(request)
            } else if let Some(access) = access {
                self.visible(access, request, now).await?
            } else {
                None
            };
            if let Some(request) = request {
                let matches_status = match status {
                    None | Some("all") => true,
                    Some("history") => request.status != OverrideStatus::Pending,
                    Some(status) => request.status.to_string() == status,
                };
                if matches_status
                    && role
                        .is_none_or(|role| request.current_approver_role.as_deref() == Some(role))
                {
                    visible.push(request);
                }
            }
        }
        Ok(visible)
    }

    pub async fn list_overrides(
        &self,
        ctx: &RequestContext,
        status: Option<&str>,
        role: Option<&str>,
    ) -> Result<Vec<AcademicOverrideRequest>, DomainError> {
        let requester = human(ctx)?;
        let access = match self.access(ctx, Permission::WorkflowRead).await {
            Ok(access) => Some(access),
            Err(DomainError::Forbidden(_)) => {
                self.access(ctx, Permission::WorkflowWrite).await?;
                None
            }
            Err(error) => return Err(error),
        };
        let now = Utc::now();
        // Pending records may be effectively expired under the server policy.
        let stored_status =
            status.filter(|status| !matches!(*status, "expired" | "all" | "history"));
        let requests = self
            .override_repo
            .find_by_org(ctx.tenant_id, stored_status)
            .await?;
        self.filter_visible_override_requests(
            requester,
            access.as_ref(),
            requests,
            now,
            status,
            role,
        )
        .await
    }

    pub async fn list_overrides_paginated(
        &self,
        ctx: &RequestContext,
        status: Option<&str>,
        role: Option<&str>,
        search: Option<&str>,
        mine: bool,
        pagination: PaginationParams,
    ) -> Result<PaginatedList<AcademicOverrideRequest>, DomainError> {
        let requester = human(ctx)?;
        let access = match self.access(ctx, Permission::WorkflowRead).await {
            Ok(access) => Some(access),
            Err(DomainError::Forbidden(_)) => {
                self.access(ctx, Permission::WorkflowWrite).await?;
                None
            }
            Err(error) => return Err(error),
        };
        let page = pagination.page();
        let page_size = pagination.page_size();

        if !mine
            && (role.is_some()
                || access
                    .as_ref()
                    .is_some_and(|access| matches!(&access.scope, PermissionScope::Units(_))))
        {
            let mut requests = self
                .list_overrides(ctx, status, role)
                .await?
                .into_iter()
                .filter(|request| {
                    search.is_none_or(|search| matches_override_search(request, search))
                })
                .collect::<Vec<_>>();
            requests.sort_by(|left, right| {
                right
                    .created_at
                    .cmp(&left.created_at)
                    .then_with(|| right.id.cmp(&left.id))
            });
            let total_count = requests.len() as u64;
            let items = requests
                .into_iter()
                .skip(pagination.offset() as usize)
                .take(page_size as usize)
                .collect();
            return Ok(PaginatedList::new(items, total_count, page, page_size));
        }

        let requests = self
            .override_repo
            .list(
                ctx.tenant_id,
                OverrideRequestFilter {
                    status: status.map(str::to_owned),
                    search: search.map(str::to_owned),
                    requested_by: (mine || access.is_none()).then_some(requester),
                    pagination: Some(pagination),
                },
            )
            .await?;
        let visible = self
            .filter_visible_override_requests(
                requester,
                access.as_ref(),
                requests.items,
                Utc::now(),
                status,
                role,
            )
            .await?;
        Ok(PaginatedList::new(
            visible,
            requests.total_count,
            requests.page,
            requests.page_size,
        ))
    }

    pub async fn get_override(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<Option<AcademicOverrideRequest>, DomainError> {
        let access = self.access(ctx, Permission::WorkflowRead).await?;
        let Some(request) = self
            .override_repo
            .find_by_id(id)
            .await?
            .filter(|request| request.org_id == ctx.tenant_id)
        else {
            return Ok(None);
        };
        self.visible(&access, request, Utc::now()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_scope_covers_every_target_and_requires_active_hierarchy() {
        let region = Uuid::new_v4();
        let branch = Uuid::new_v4();
        let sibling = Uuid::new_v4();
        let inactive = Uuid::new_v4();
        let units = HashMap::from([
            (region, None),
            (branch, Some(region)),
            (sibling, Some(region)),
        ]);
        let branch_access = Access {
            scope: PermissionScope::Units(vec![branch]),
            units: units.clone(),
        };
        assert!(branch_access.allows(&[Some(branch)]));
        assert!(!branch_access.allows(&[Some(branch), Some(sibling)]));
        assert!(!branch_access.allows(&[Some(branch), None]));
        let region_access = Access {
            scope: PermissionScope::Units(vec![region]),
            units,
        };
        assert!(region_access.allows(&[Some(branch), Some(sibling)]));
        assert!(!region_access.allows(&[Some(inactive)]));
    }

    #[test]
    fn rebuilds_old_chain_escalates_and_requires_independent_reviewers() {
        let now = Utc::now();
        let requester = Uuid::new_v4();
        let prior_reviewer = Uuid::new_v4();
        let targets = [Some(Uuid::new_v4())];
        let mut request: AcademicOverrideRequest = serde_json::from_value(serde_json::json!({
            "id": Uuid::new_v4(), "org_id": Uuid::new_v4(), "entity_type": "grade",
            "entity_id": Uuid::new_v4(), "override_type": "grade_change", "risk_level": "low",
            "reason": "Correction", "requested_by": requester, "status": "pending",
            "approval_chain": [], "created_at": now, "updated_at": now,
            "metadata": {"approval_policy_version": POLICY_VERSION}
        }))
        .unwrap();
        let mut old_step = pending_step(1, "org_admin");
        old_step.status = OverrideStatus::Approved;
        old_step.approver_id = Some(prior_reviewer);
        request.approval_chain = vec![old_step];

        let deadline = now + Duration::hours(BRANCH_REVIEW_HOURS);
        prepare_pending(&mut request, &targets, deadline - Duration::seconds(1));
        assert_eq!(request.risk_level, RiskLevel::High);
        assert_eq!(request.current_approver_role.as_deref(), Some("principal"));
        assert!(
            request
                .approval_chain
                .iter()
                .all(|step| step.approver_id.is_none())
        );

        prepare_pending(&mut request, &targets, deadline);
        prepare_pending(&mut request, &targets, deadline + Duration::seconds(1));
        assert_eq!(request.approval_chain.len(), 3);
        assert_eq!(request.approval_chain[0].status, OverrideStatus::Expired);
        assert_eq!(
            request.current_approver_role.as_deref(),
            Some("super_admin")
        );
        assert_eq!(request.approval_chain[2].approver_role, "org_admin");
        request.approval_chain[1].status = OverrideStatus::Approved;
        request.approval_chain[1].approver_id = Some(prior_reviewer);
        assert!(matches!(
            ensure_independent_reviewer(&request, requester),
            Err(DomainError::Forbidden(_))
        ));
        assert!(matches!(
            ensure_independent_reviewer(&request, prior_reviewer),
            Err(DomainError::Forbidden(_))
        ));
        assert!(ensure_independent_reviewer(&request, Uuid::new_v4()).is_ok());

        prepare_pending(
            &mut request,
            &targets,
            now + Duration::days(REQUEST_VALID_DAYS),
        );
        assert_eq!(request.status, OverrideStatus::Expired);
        assert!(request.current_approver_role.is_none());
    }
}
