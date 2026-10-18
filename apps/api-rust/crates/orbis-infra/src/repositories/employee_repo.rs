use async_trait::async_trait;
use sea_orm::sea_query::{Expr, Func};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    JoinType, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
    TransactionTrait,
};
use uuid::Uuid;

use orbis_app::ports::repositories::{EmployeeFilter, EmployeeRepository};
use orbis_domain::entities::employee::{
    Employee, EmployeeDetail, EmployeeStatus, EmployeeType, EmploymentType,
};
use orbis_domain::entities::group::Group;
use orbis_domain::entities::person::Person;
use orbis_domain::entities::user::User;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::PaginatedList;

use crate::models::employee::{self, Entity as EmployeeEntity};
use crate::models::group::{self, Entity as GroupEntity};
use crate::models::group_permission::{self, Entity as GroupPermissionEntity};
use crate::models::org_unit::Entity as OrgUnitEntity;
use crate::models::person::{self, Entity as PersonEntity};
use crate::models::role_permission::{self, Entity as RolePermissionEntity};
use crate::models::user::{self, Entity as UserEntity};
use crate::models::user_group_membership::{self, Entity as UserGroupMembershipEntity};
use crate::models::user_role_assignment::{self, Entity as UserRoleAssignmentEntity};

pub struct EmployeeRepoImpl {
    pub db: DatabaseConnection,
}

impl EmployeeRepoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_employee(m: employee::Model) -> Employee {
        Employee {
            id: m.id,
            org_id: m.org_id,
            person_id: m.person_id,
            user_id: m.user_id,
            employee_number: m.employee_number,
            employee_type: m.employee_type,
            job_title: m.job_title,
            department: m.department,
            org_unit_id: m.org_unit_id,
            hire_date: chrono::NaiveDate::parse_from_str(&m.hire_date, "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            status: EmployeeStatus::from(m.status.as_str()),
            employment_type: EmploymentType::from(m.employment_type.as_str()),
            work_shift: m.work_shift,
            notes: m.notes,
            is_deleted: m.is_deleted,
            deleted_at: m.deleted_at,
            created_by: m.created_by,
            updated_by: m.updated_by,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }

    fn model_to_person(m: person::Model) -> Person {
        Person {
            id: m.id,
            org_id: m.org_id,
            first_name: m.first_name,
            middle_name: m.middle_name,
            last_name: m.last_name,
            first_name_ar: m.first_name_ar,
            middle_name_ar: m.middle_name_ar,
            last_name_ar: m.last_name_ar,
            display_name: m.display_name,
            national_id: m.national_id,
            passport_number: m.passport_number,
            gender: m.gender,
            birth_date: m.birth_date,
            place_of_birth: m.place_of_birth,
            nationality: m.nationality,
            blood_type: m.blood_type,
            marital_status: m.marital_status,
            medical_notes: m.medical_notes,
            special_needs_notes: m.special_needs_notes,
            phone: m.phone,
            email: m.email,
            address: m.address,
            avatar_url: m.avatar_url,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }

    fn model_to_user(m: user::Model) -> User {
        User {
            id: m.id,
            person_id: m.person_id,
            org_id: m.org_id,
            email: m.email,
            is_email_verified: m.is_email_verified,
            phone: m.phone,
            is_phone_verified: m.is_phone_verified,
            password_hash: m.password_hash,
            is_mfa_enabled: m.is_mfa_enabled,
            status: m.status,
            login_count: m.login_count,
            failed_login_attempts: m.failed_login_attempts,
            locked_until: m.locked_until,
            is_deleted: m.is_deleted,
            created_by: m.created_by,
            updated_by: m.updated_by,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[async_trait]
impl EmployeeRepository for EmployeeRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employee>, DomainError> {
        let model = EmployeeEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(Self::model_to_employee))
    }

    async fn find_by_person_id(
        &self,
        org_id: Uuid,
        person_id: Uuid,
    ) -> Result<Option<Employee>, DomainError> {
        let model = EmployeeEntity::find()
            .filter(employee::Column::OrgId.eq(org_id))
            .filter(employee::Column::PersonId.eq(person_id))
            .filter(employee::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(Self::model_to_employee))
    }

    async fn find_by_user_id(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Employee>, DomainError> {
        let model = EmployeeEntity::find()
            .filter(employee::Column::OrgId.eq(org_id))
            .filter(employee::Column::UserId.eq(user_id))
            .filter(employee::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(Self::model_to_employee))
    }

    async fn find_by_employee_number(
        &self,
        org_id: Uuid,
        employee_number: &str,
    ) -> Result<Option<Employee>, DomainError> {
        let model = EmployeeEntity::find()
            .filter(employee::Column::OrgId.eq(org_id))
            .filter(employee::Column::EmployeeNumber.eq(employee_number))
            .filter(employee::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(Self::model_to_employee))
    }

    async fn find_detail_by_id(&self, id: Uuid) -> Result<Option<EmployeeDetail>, DomainError> {
        let emp_model = match EmployeeEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
        {
            Some(m) => m,
            None => return Ok(None),
        };

        let person_model = PersonEntity::find_by_id(emp_model.person_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Person not found for employee".into()))?;

        let user_model = if let Some(uid) = emp_model.user_id {
            UserEntity::find_by_id(uid)
                .one(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?
        } else {
            None
        };

        // Org unit name
        let org_unit_name = if let Some(ouid) = emp_model.org_unit_id {
            OrgUnitEntity::find_by_id(ouid)
                .one(&self.db)
                .await
                .ok()
                .flatten()
                .map(|u| u.name)
        } else {
            None
        };

        // User roles & groups & effective permissions
        let mut roles = Vec::new();
        let mut groups = Vec::new();
        let mut effective_permissions = Vec::new();

        if let Some(ref u) = user_model {
            // Direct roles
            let user_roles = UserRoleAssignmentEntity::find()
                .filter(user_role_assignment::Column::UserId.eq(u.id))
                .all(&self.db)
                .await
                .unwrap_or_default();
            let role_ids: Vec<String> = user_roles.into_iter().map(|r| r.role_id).collect();
            roles = role_ids.clone();

            if !role_ids.is_empty() {
                let r_perms = RolePermissionEntity::find()
                    .filter(role_permission::Column::RoleId.is_in(role_ids))
                    .all(&self.db)
                    .await
                    .unwrap_or_default();
                for rp in r_perms {
                    let perm = if rp.resource == "*" {
                        "*".to_string()
                    } else if rp.resource.contains(':') {
                        rp.resource.to_lowercase()
                    } else if rp.action.is_empty() || rp.action == "*" {
                        rp.resource.to_lowercase()
                    } else {
                        format!(
                            "{}:{}",
                            rp.resource.to_lowercase(),
                            rp.action.to_lowercase()
                        )
                    };
                    if !effective_permissions.contains(&perm) {
                        effective_permissions.push(perm);
                    }
                }
            }

            // User groups
            let memberships = UserGroupMembershipEntity::find()
                .filter(user_group_membership::Column::UserId.eq(u.id))
                .all(&self.db)
                .await
                .unwrap_or_default();
            let group_ids: Vec<Uuid> = memberships.into_iter().map(|m| m.group_id).collect();

            if !group_ids.is_empty() {
                let group_models = GroupEntity::find()
                    .filter(group::Column::Id.is_in(group_ids.clone()))
                    .all(&self.db)
                    .await
                    .unwrap_or_default();
                for gm in group_models {
                    groups.push(Group {
                        id: gm.id,
                        org_id: gm.org_id,
                        name: gm.name,
                        name_ar: gm.name_ar,
                        description: gm.description,
                        is_system: gm.is_system,
                        status: gm.status,
                        created_by: gm.created_by,
                        updated_by: gm.updated_by,
                        created_at: gm.created_at,
                        updated_at: gm.updated_at,
                    });
                }

                // Group permissions
                let g_perms = GroupPermissionEntity::find()
                    .filter(group_permission::Column::GroupId.is_in(group_ids))
                    .all(&self.db)
                    .await
                    .unwrap_or_default();
                for gp in g_perms {
                    if !effective_permissions.contains(&gp.permission) {
                        effective_permissions.push(gp.permission);
                    }
                }
            }
        }

        effective_permissions.sort();

        Ok(Some(EmployeeDetail {
            employee: Self::model_to_employee(emp_model),
            person: Self::model_to_person(person_model),
            user: user_model.map(Self::model_to_user),
            roles,
            groups,
            effective_permissions,
            org_unit_name,
        }))
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: EmployeeFilter,
    ) -> Result<PaginatedList<EmployeeDetail>, DomainError> {
        let mut query = EmployeeEntity::find()
            .filter(employee::Column::OrgId.eq(org_id))
            .filter(employee::Column::IsDeleted.eq(false));

        if let Some(ref status_str) = filter.status {
            let s_clean = status_str.trim();
            if !s_clean.is_empty() && s_clean != "all" {
                let status = EmployeeStatus::from(s_clean);
                let canonical_status = status.to_string();
                query = query.filter(
                    Expr::expr(Func::lower(Expr::col(employee::Column::Status)))
                        .eq(&canonical_status),
                );
            }
        }

        if let Some(ref emp_type_str) = filter.employee_type {
            let t_clean = emp_type_str.trim();
            if !t_clean.is_empty() && t_clean != "all" {
                let emp_type = EmployeeType::from(t_clean);
                let canonical_type = emp_type.to_string();
                query = query.filter(
                    Expr::expr(Func::lower(Expr::col(employee::Column::EmployeeType)))
                        .eq(&canonical_type),
                );
            }
        }

        if let Some(ref dept) = filter.department {
            if !dept.is_empty() && dept != "all" {
                query = query.filter(employee::Column::Department.eq(dept));
            }
        }

        if let Some(ouid) = filter.org_unit_id {
            query = query.filter(employee::Column::OrgUnitId.eq(ouid));
        }

        if let Some(search) = filter.search.as_deref() {
            let terms: Vec<_> = search
                .split_whitespace()
                .filter(|term| !term.is_empty())
                .collect();
            if !terms.is_empty() {
                query = query.join(JoinType::InnerJoin, employee::Relation::Person.def());

                let mut all_terms = Condition::all();
                for term in terms {
                    let search_pat = format!("%{term}%");
                    all_terms = all_terms.add(
                        Condition::any()
                            .add(employee::Column::EmployeeNumber.like(&search_pat))
                            .add(employee::Column::EmployeeType.like(&search_pat))
                            .add(employee::Column::JobTitle.like(&search_pat))
                            .add(employee::Column::Department.like(&search_pat))
                            .add(person::Column::FirstName.like(&search_pat))
                            .add(person::Column::MiddleName.like(&search_pat))
                            .add(person::Column::LastName.like(&search_pat))
                            .add(person::Column::FirstNameAr.like(&search_pat))
                            .add(person::Column::MiddleNameAr.like(&search_pat))
                            .add(person::Column::LastNameAr.like(&search_pat))
                            .add(person::Column::DisplayName.like(&search_pat))
                            .add(person::Column::NationalId.like(&search_pat))
                            .add(person::Column::PassportNumber.like(&search_pat))
                            .add(person::Column::Email.like(&search_pat))
                            .add(person::Column::Phone.like(&search_pat)),
                    );
                }
                query = query.filter(all_terms);
            }
        }

        query = query.order_by_desc(employee::Column::CreatedAt);

        let limit = filter.limit.unwrap_or(50);
        let offset = filter.offset.unwrap_or(0);

        let total = query
            .clone()
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let emp_models = query
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut items = Vec::new();
        for emp in emp_models {
            let person = PersonEntity::find_by_id(emp.person_id)
                .one(&self.db)
                .await
                .ok()
                .flatten()
                .map(Self::model_to_person)
                .unwrap_or_else(|| Person {
                    id: emp.person_id,
                    org_id: Some(emp.org_id),
                    first_name: "Unknown".into(),
                    middle_name: None,
                    last_name: "Person".into(),
                    first_name_ar: None,
                    middle_name_ar: None,
                    last_name_ar: None,
                    display_name: None,
                    national_id: None,
                    passport_number: None,
                    gender: None,
                    birth_date: None,
                    place_of_birth: None,
                    nationality: None,
                    blood_type: None,
                    marital_status: None,
                    medical_notes: None,
                    special_needs_notes: None,
                    phone: None,
                    email: None,
                    address: None,
                    avatar_url: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                });

            let user = if let Some(uid) = emp.user_id {
                UserEntity::find_by_id(uid)
                    .one(&self.db)
                    .await
                    .ok()
                    .flatten()
                    .map(Self::model_to_user)
            } else {
                None
            };

            let org_unit_name = if let Some(ouid) = emp.org_unit_id {
                OrgUnitEntity::find_by_id(ouid)
                    .one(&self.db)
                    .await
                    .ok()
                    .flatten()
                    .map(|u| u.name)
            } else {
                None
            };

            items.push(EmployeeDetail {
                employee: Self::model_to_employee(emp),
                person,
                user,
                roles: vec![],
                groups: vec![],
                effective_permissions: vec![],
                org_unit_name,
            });
        }

        let page = (offset / limit) + 1;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u64;

        Ok(PaginatedList {
            items,
            total_count: total,
            page,
            page_size: limit,
            total_pages,
        })
    }

    async fn list_by_type(
        &self,
        org_id: Uuid,
        employee_type: &str,
    ) -> Result<Vec<Employee>, DomainError> {
        let emp_type = EmployeeType::from(employee_type);
        let canonical_type = emp_type.to_string();
        let models = EmployeeEntity::find()
            .filter(employee::Column::OrgId.eq(org_id))
            .filter(
                Expr::expr(Func::lower(Expr::col(employee::Column::EmployeeType)))
                    .eq(&canonical_type),
            )
            .filter(employee::Column::IsDeleted.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(Self::model_to_employee).collect())
    }

    async fn count_by_status(
        &self,
        org_id: Uuid,
    ) -> Result<std::collections::HashMap<String, i64>, DomainError> {
        let models = EmployeeEntity::find()
            .filter(employee::Column::OrgId.eq(org_id))
            .filter(employee::Column::IsDeleted.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut counts: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        counts.insert("all".into(), models.len() as i64);

        for m in models {
            let status = m.status.to_lowercase();
            *counts.entry(status).or_insert(0) += 1;
        }

        Ok(counts)
    }

    async fn save(&self, employee: &Employee) -> Result<(), DomainError> {
        let active_model = employee_to_active_model(employee);

        if EmployeeEntity::find_by_id(employee.id)
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

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let mut model = match EmployeeEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
        {
            Some(m) => m.into_active_model(),
            None => return Ok(()),
        };

        model.is_deleted = Set(true);
        model.updated_at = Set(chrono::Utc::now());
        model
            .update(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
    async fn create_with_license_limit(
        &self,
        employee: &Employee,
        max_staff: Option<u64>,
    ) -> Result<(), DomainError> {
        let Some(max_staff) = max_staff else {
            return EmployeeRepository::save(self, employee).await;
        };

        let transaction = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        crate::repositories::license_quota::acquire_organization_quota_lock(
            &transaction,
            employee.org_id,
        )
        .await?;

        let current = EmployeeEntity::find()
            .filter(employee::Column::OrgId.eq(employee.org_id))
            .filter(employee::Column::IsDeleted.eq(false))
            .filter(employee::Column::Status.is_not_in(["terminated", "resigned"]))
            .count(&transaction)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if current.saturating_add(1) > max_staff {
            return Err(DomainError::LicenseRequired(format!(
                "The staff quota of {max_staff} has been reached"
            )));
        }

        employee_to_active_model(employee)
            .insert(&transaction)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        transaction
            .commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }
}

fn employee_to_active_model(employee: &Employee) -> employee::ActiveModel {
    employee::ActiveModel {
        id: Set(employee.id),
        org_id: Set(employee.org_id),
        person_id: Set(employee.person_id),
        user_id: Set(employee.user_id),
        employee_number: Set(employee.employee_number.clone()),
        employee_type: Set(employee.employee_type.clone()),
        job_title: Set(employee.job_title.clone()),
        department: Set(employee.department.clone()),
        org_unit_id: Set(employee.org_unit_id),
        hire_date: Set(employee.hire_date.to_string()),
        status: Set(employee.status.to_string()),
        employment_type: Set(employee.employment_type.to_string()),
        work_shift: Set(employee.work_shift.clone()),
        notes: Set(employee.notes.clone()),
        is_deleted: Set(employee.is_deleted),
        deleted_at: Set(employee.deleted_at),
        created_by: Set(employee.created_by),
        updated_by: Set(employee.updated_by),
        created_at: Set(employee.created_at),
        updated_at: Set(employee.updated_at),
    }
}
