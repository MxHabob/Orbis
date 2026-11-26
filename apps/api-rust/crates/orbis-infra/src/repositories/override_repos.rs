use async_trait::async_trait;
use chrono::{Duration, Utc};
use orbis_app::ports::repositories::{OverrideRequestFilter, OverrideRequestRepository};
use orbis_domain::entities::override_request::{
    AcademicOverrideRequest, ApprovalStep, OverrideStatus, OverrideType, RiskLevel,
};
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::models::override_request::{self, Entity as OverrideEntity};
use crate::models::{
    academic_period, enrollment, grade, section, section_subject, student, student_fee,
};

pub struct OverrideRepoImpl {
    pub db: DatabaseConnection,
}

fn target_missing() -> DomainError {
    DomainError::NotFound("Override target not found".into())
}

fn database_error(error: sea_orm::DbErr) -> DomainError {
    DomainError::Internal(error.to_string())
}

impl OverrideRepoImpl {
    async fn period(&self, org_id: Uuid, id: Uuid) -> Result<academic_period::Model, DomainError> {
        academic_period::Entity::find_by_id(id)
            .filter(academic_period::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(database_error)?
            .ok_or_else(target_missing)
    }

    async fn student_unit(&self, org_id: Uuid, id: Uuid) -> Result<Option<Uuid>, DomainError> {
        student::Entity::find_by_id(id)
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(database_error)?
            .map(|student| student.org_unit_id)
            .ok_or_else(target_missing)
    }

    async fn section_unit(&self, org_id: Uuid, id: Uuid) -> Result<Option<Uuid>, DomainError> {
        section::Entity::find_by_id(id)
            .filter(section::Column::OrgId.eq(org_id))
            .filter(section::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(database_error)?
            .map(|section| section.org_unit_id)
            .ok_or_else(target_missing)
    }
}

#[async_trait]
impl OverrideRequestRepository for OverrideRepoImpl {
    async fn target_units(
        &self,
        org_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        linked_section_id: Option<Uuid>,
        period_id: Option<Uuid>,
    ) -> Result<Vec<Option<Uuid>>, DomainError> {
        let period = match period_id {
            Some(id) => Some(self.period(org_id, id).await?),
            None => None,
        };
        let mut units = match entity_type {
            "academic_period" if period_id == Some(entity_id) => vec![None],
            "section_subject" => {
                let assignment = section_subject::Entity::find_by_id(entity_id)
                    .filter(section_subject::Column::OrgId.eq(org_id))
                    .one(&self.db)
                    .await
                    .map_err(database_error)?
                    .ok_or_else(target_missing)?;
                let mut current = period.ok_or_else(target_missing)?;
                let calendar = current.calendar_id;
                let mut visited = std::collections::HashSet::new();
                while current.id != assignment.period_id {
                    if !visited.insert(current.id) {
                        return Err(target_missing());
                    }
                    current = self
                        .period(org_id, current.parent_id.ok_or_else(target_missing)?)
                        .await?;
                    if current.calendar_id != calendar {
                        return Err(target_missing());
                    }
                }
                vec![self.section_unit(org_id, assignment.section_id).await?]
            }
            "student" => vec![self.student_unit(org_id, entity_id).await?],
            "class" | "section" => vec![self.section_unit(org_id, entity_id).await?],
            "enrollment" => {
                let enrollment = enrollment::Entity::find_by_id(entity_id)
                    .filter(enrollment::Column::OrgId.eq(org_id))
                    .one(&self.db)
                    .await
                    .map_err(database_error)?
                    .ok_or_else(target_missing)?;
                let mut units = vec![self.student_unit(org_id, enrollment.student_id).await?];
                if let Some(section_id) = enrollment.section_id {
                    units.push(self.section_unit(org_id, section_id).await?);
                }
                units
            }
            "grade" => {
                let grade = grade::Entity::find_by_id(entity_id)
                    .filter(grade::Column::OrgId.eq(org_id))
                    .one(&self.db)
                    .await
                    .map_err(database_error)?
                    .ok_or_else(target_missing)?;
                let assignment = section_subject::Entity::find_by_id(grade.section_subject_id)
                    .filter(section_subject::Column::OrgId.eq(org_id))
                    .one(&self.db)
                    .await
                    .map_err(database_error)?
                    .ok_or_else(target_missing)?;
                vec![
                    self.student_unit(org_id, grade.student_id).await?,
                    self.section_unit(org_id, assignment.section_id).await?,
                ]
            }
            "fee" | "student_fee" => {
                let fee = student_fee::Entity::find_by_id(entity_id)
                    .filter(student_fee::Column::OrgId.eq(org_id))
                    .one(&self.db)
                    .await
                    .map_err(database_error)?
                    .ok_or_else(target_missing)?;
                vec![self.student_unit(org_id, fee.student_id).await?]
            }
            _ => {
                return Err(DomainError::Validation(
                    "Unsupported override target type".into(),
                ));
            }
        };
        if let Some(section_id) = linked_section_id {
            units.push(self.section_unit(org_id, section_id).await?);
        }
        units.sort_unstable();
        units.dedup();
        Ok(units)
    }

    async fn save_decision(
        &self,
        request: &AcademicOverrideRequest,
        expected_updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), DomainError> {
        let active = override_to_active_model(request)?;
        let result = OverrideEntity::update_many()
            .set(override_request::ActiveModel {
                status: active.status,
                current_approver_role: active.current_approver_role,
                approval_chain: active.approval_chain,
                approval_policy_version: active.approval_policy_version,
                resolved_by: active.resolved_by,
                resolved_by_name: active.resolved_by_name,
                resolved_at: active.resolved_at,
                resolution_notes: active.resolution_notes,
                updated_at: active.updated_at,
                ..Default::default()
            })
            .filter(override_request::Column::Id.eq(request.id))
            .filter(override_request::Column::OrgId.eq(request.org_id))
            .filter(override_request::Column::Status.eq(OverrideStatus::Pending.to_string()))
            .filter(override_request::Column::UpdatedAt.eq(expected_updated_at))
            .exec(&self.db)
            .await
            .map_err(database_error)?;
        if result.rows_affected != 1 {
            return Err(DomainError::Conflict(
                "The override changed before the decision was saved".into(),
            ));
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AcademicOverrideRequest>, DomainError> {
        let model = OverrideEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_model_to_domain))
    }

    async fn find_by_org(
        &self,
        org_id: Uuid,
        status: Option<&str>,
    ) -> Result<Vec<AcademicOverrideRequest>, DomainError> {
        let mut query = OverrideEntity::find().filter(override_request::Column::OrgId.eq(org_id));
        if let Some(st) = status {
            query = query.filter(override_request::Column::Status.eq(st));
        }
        let models = query
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_model_to_domain).collect())
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: OverrideRequestFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<AcademicOverrideRequest>, DomainError>
    {
        let mut query = OverrideEntity::find().filter(override_request::Column::OrgId.eq(org_id));

        if let Some(status) = filter
            .status
            .as_deref()
            .map(str::trim)
            .filter(|status| !status.is_empty() && *status != "all")
        {
            let now = Utc::now();
            let default_expiry = now - Duration::days(14);
            let expired_pending = Condition::all()
                .add(override_request::Column::Status.eq("pending"))
                .add(
                    Condition::any()
                        .add(override_request::Column::ExpiresAt.lte(now))
                        .add(
                            Condition::all()
                                .add(override_request::Column::ExpiresAt.is_null())
                                .add(override_request::Column::CreatedAt.lte(default_expiry)),
                        ),
                );
            let status_filter = match status {
                "expired" => Condition::any()
                    .add(override_request::Column::Status.eq("expired"))
                    .add(expired_pending),
                "history" => Condition::any()
                    .add(override_request::Column::Status.ne("pending"))
                    .add(expired_pending),
                "pending" => Condition::all()
                    .add(override_request::Column::Status.eq("pending"))
                    .add(
                        Condition::any()
                            .add(override_request::Column::ExpiresAt.gt(now))
                            .add(
                                Condition::all()
                                    .add(override_request::Column::ExpiresAt.is_null())
                                    .add(override_request::Column::CreatedAt.gt(default_expiry)),
                            ),
                    ),
                status => Condition::all().add(override_request::Column::Status.eq(status)),
            };
            query = query.filter(status_filter);
        }
        if let Some(requested_by) = filter.requested_by {
            query = query.filter(override_request::Column::RequestedBy.eq(requested_by));
        }
        if let Some(search) = filter.search.as_deref() {
            let terms = search
                .split_whitespace()
                .filter(|term| !term.is_empty())
                .collect::<Vec<_>>();
            if !terms.is_empty() {
                let mut all_terms = Condition::all();
                for term in terms {
                    let search_pattern = format!("%{term}%");
                    all_terms = all_terms.add(
                        Condition::any()
                            .add(override_request::Column::Reason.like(&search_pattern))
                            .add(override_request::Column::RequestedByName.like(&search_pattern))
                            .add(override_request::Column::OverrideType.like(&search_pattern))
                            .add(override_request::Column::EntityType.like(&search_pattern))
                            .add(override_request::Column::FieldName.like(&search_pattern))
                            .add(override_request::Column::Status.like(&search_pattern))
                            .add(override_request::Column::ResolvedByName.like(&search_pattern))
                            .add(override_request::Column::ResolutionNotes.like(&search_pattern)),
                    );
                }
                query = query.filter(all_terms);
            }
        }

        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query
            .order_by_desc(override_request::Column::CreatedAt)
            .order_by_desc(override_request::Column::Id)
            .paginate(&self.db, page_size);
        let total_count = paginator.num_items().await.map_err(database_error)?;
        let models = paginator
            .fetch_page(page - 1)
            .await
            .map_err(database_error)?;

        Ok(orbis_domain::queries::filters::PaginatedList::new(
            models.into_iter().map(map_model_to_domain).collect(),
            total_count,
            page,
            page_size,
        ))
    }

    async fn find_active_override(
        &self,
        org_id: Uuid,
        override_type: &str,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Option<AcademicOverrideRequest>, DomainError> {
        let model = OverrideEntity::find()
            .filter(override_request::Column::OrgId.eq(org_id))
            .filter(override_request::Column::OverrideType.eq(override_type))
            .filter(override_request::Column::EntityType.eq(entity_type))
            .filter(override_request::Column::EntityId.eq(entity_id))
            .filter(override_request::Column::Status.eq("approved"))
            .filter(
                sea_orm::Condition::any()
                    .add(override_request::Column::ExpiresAt.is_null())
                    .add(override_request::Column::ExpiresAt.gt(chrono::Utc::now())),
            )
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_model_to_domain))
    }

    async fn save(&self, r: &AcademicOverrideRequest) -> Result<(), DomainError> {
        let active = override_to_active_model(r)?;

        if OverrideEntity::find_by_id(r.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

pub(crate) fn override_to_active_model(
    r: &AcademicOverrideRequest,
) -> Result<override_request::ActiveModel, DomainError> {
    let chain_json = serde_json::to_value(&r.approval_chain)
        .map_err(|error| DomainError::Internal(error.to_string()))?;
    Ok(override_request::ActiveModel {
        id: Set(r.id),
        org_id: Set(r.org_id),
        override_type: Set(r.override_type.to_string()),
        risk_level: Set(r.risk_level.to_string()),
        entity_type: Set(r.entity_type.clone()),
        entity_id: Set(r.entity_id),
        field_name: Set(r.field_name.clone()),
        old_value: Set(r.old_value.clone()),
        new_value: Set(r.new_value.clone()),
        reason: Set(r.reason.clone()),
        requested_by: Set(r.requested_by),
        requested_by_name: Set(r.requested_by_name.clone()),
        status: Set(r.status.to_string()),
        current_approver_role: Set(r.current_approver_role.clone()),
        approval_chain: Set(chain_json),
        approval_policy_version: Set(r.approval_policy_version),
        resolved_by: Set(r.resolved_by),
        resolved_by_name: Set(r.resolved_by_name.clone()),
        resolved_at: Set(r.resolved_at),
        resolution_notes: Set(r.resolution_notes.clone()),
        expires_at: Set(r.expires_at),
        metadata: Set(Some(r.metadata.clone())),
        created_at: Set(r.created_at),
        updated_at: Set(r.updated_at),
    })
}

fn map_model_to_domain(m: override_request::Model) -> AcademicOverrideRequest {
    let chain: Vec<ApprovalStep> = serde_json::from_value(m.approval_chain).unwrap_or_default();
    AcademicOverrideRequest {
        id: m.id,
        org_id: m.org_id,
        override_type: m.override_type.parse().unwrap_or(OverrideType::Custom(0)),
        risk_level: match m.risk_level.as_str() {
            "low" => RiskLevel::Low,
            "high" => RiskLevel::High,
            "critical" => RiskLevel::Critical,
            _ => RiskLevel::Medium,
        },
        entity_type: m.entity_type,
        entity_id: m.entity_id,
        field_name: m.field_name,
        old_value: m.old_value,
        new_value: m.new_value,
        reason: m.reason,
        requested_by: m.requested_by,
        requested_by_name: m.requested_by_name,
        status: match m.status.as_str() {
            "pending" => OverrideStatus::Pending,
            "approved" => OverrideStatus::Approved,
            "rejected" => OverrideStatus::Rejected,
            "expired" => OverrideStatus::Expired,
            "executed" => OverrideStatus::Executed,
            _ => OverrideStatus::Cancelled,
        },
        current_approver_role: m.current_approver_role,
        approval_chain: chain,
        approval_policy_version: m.approval_policy_version,
        resolved_by: m.resolved_by,
        resolved_by_name: m.resolved_by_name,
        resolved_at: m.resolved_at,
        resolution_notes: m.resolution_notes,
        expires_at: m.expires_at,
        metadata: m.metadata.unwrap_or_default(),
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use sea_orm::{ConnectionTrait, Database, Schema};

    #[tokio::test]
    async fn a_stale_decision_cannot_overwrite_an_intermediate_approval() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        let backend = db.get_database_backend();
        let table = Schema::new(backend).create_table_from_entity(OverrideEntity);
        db.execute(backend.build(&table)).await.unwrap();
        let repo = OverrideRepoImpl { db };
        let now = Utc::now();
        let mut request: AcademicOverrideRequest = serde_json::from_value(serde_json::json!({
            "id": Uuid::new_v4(), "org_id": Uuid::new_v4(), "entity_type": "grade",
            "entity_id": Uuid::new_v4(), "override_type": "grade_change", "risk_level": "high",
            "reason": "Correction", "requested_by": Uuid::new_v4(), "status": "pending",
            "approval_chain": [], "created_at": now, "updated_at": now, "metadata": {}
        }))
        .unwrap();
        repo.save(&request).await.unwrap();
        let mut stale = repo.find_by_id(request.id).await.unwrap().unwrap();
        let expected = stale.updated_at;
        request.current_approver_role = Some("org_admin".into());
        request.updated_at = expected + Duration::seconds(1);
        repo.save_decision(&request, expected).await.unwrap();

        stale.status = OverrideStatus::Rejected;
        stale.updated_at = expected + Duration::seconds(2);
        assert!(matches!(
            repo.save_decision(&stale, expected).await,
            Err(DomainError::Conflict(_))
        ));
        let stored = repo.find_by_id(request.id).await.unwrap().unwrap();
        assert_eq!(stored.status, OverrideStatus::Pending);
        assert_eq!(stored.current_approver_role.as_deref(), Some("org_admin"));
    }

    #[tokio::test]
    async fn list_filters_before_paginating() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        let backend = db.get_database_backend();
        let table = Schema::new(backend).create_table_from_entity(OverrideEntity);
        db.execute(backend.build(&table)).await.unwrap();
        let repo = OverrideRepoImpl { db };
        let org_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let older_id = Uuid::new_v4();
        let now = Utc::now();

        for (id, status, requester_name, created_at) in [
            (older_id, "pending", "Alice", now - Duration::minutes(2)),
            (
                Uuid::new_v4(),
                "pending",
                "Alice",
                now - Duration::minutes(1),
            ),
            (Uuid::new_v4(), "rejected", "Alice", now),
            (Uuid::new_v4(), "pending", "Bob", now),
        ] {
            let request: AcademicOverrideRequest = serde_json::from_value(serde_json::json!({
                "id": id,
                "org_id": org_id,
                "entity_type": "student",
                "entity_id": Uuid::new_v4(),
                "override_type": "grade_change",
                "risk_level": "high",
                "reason": "Grade correction",
                "requested_by": requester_id,
                "requested_by_name": requester_name,
                "status": status,
                "approval_chain": [],
                "created_at": created_at,
                "updated_at": created_at,
                "metadata": {}
            }))
            .unwrap();
            repo.save(&request).await.unwrap();
        }

        let page = repo
            .list(
                org_id,
                OverrideRequestFilter {
                    status: Some("pending".into()),
                    search: Some("Alice".into()),
                    requested_by: Some(requester_id),
                    pagination: Some(orbis_domain::queries::filters::PaginationParams {
                        page: Some(2),
                        page_size: Some(1),
                        ..Default::default()
                    }),
                },
            )
            .await
            .unwrap();

        assert_eq!(page.total_count, 2);
        assert_eq!(page.page, 2);
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].id, older_id);
    }
}
