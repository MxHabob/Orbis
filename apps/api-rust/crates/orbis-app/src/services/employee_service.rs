use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use orbis_domain::entities::employee::{
    Employee, EmployeeDetail, EmployeeStatus, EmployeeType, EmploymentType,
};
use orbis_domain::entities::group::UserGroupMembership;
use orbis_domain::entities::identity_provisioning::{
    AccountProvisioningResult, ActorType, DeliveryChannel,
};
use orbis_domain::entities::person::Person;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::PaginatedList;

use super::identity_provisioning_service::{IdentityProvisioningService, ProvisionAccountInput};
use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::license::LicenseRuntime;
use crate::ports::repositories::{
    EmployeeFilter, EmployeeRepository, GroupRepository, PersonRepository,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProvisioningConfig {
    pub enabled: bool,
    pub preferred_username: Option<String>,
    pub role: Option<String>,
    pub group_ids: Vec<Uuid>,
    pub delivery_channel: Option<DeliveryChannel>,
    pub custom_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterEmployeeInput {
    pub person_id: Option<Uuid>,
    pub employee_number: Option<String>,
    pub employee_type: String, // e.g. "teacher", "admin", "principal", "accountant", "receptionist", "supervisor", "worker", "cleaner", "driver", "support"
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub hire_date: Option<NaiveDate>,
    pub status: Option<EmployeeStatus>,
    pub employment_type: Option<EmploymentType>,
    pub work_shift: Option<String>,
    pub notes: Option<String>,

    // Person fields (used if person_id is None or for updating)
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,

    // Integrated account provisioning
    pub account: Option<AccountProvisioningConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployeeInput {
    pub employee_type: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub hire_date: Option<NaiveDate>,
    pub status: Option<EmployeeStatus>,
    pub employment_type: Option<EmploymentType>,
    pub work_shift: Option<String>,
    pub notes: Option<String>,

    // Person fields
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterEmployeeResult {
    pub detail: EmployeeDetail,
    pub provisioning: Option<AccountProvisioningResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRegisterEmployeeItemInput {
    pub row_index: usize,
    pub registration: RegisterEmployeeInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRegisterEmployeeItemResult {
    pub row_index: usize,
    pub success: bool,
    pub employee_id: Option<Uuid>,
    pub employee_number: Option<String>,
    pub user_id: Option<Uuid>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRegisterEmployeesInput {
    pub items: Vec<BulkRegisterEmployeeItemInput>,
    pub atomic: Option<bool>,
    pub dry_run: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRegisterEmployeeResult {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub is_atomic: bool,
    pub is_dry_run: bool,
    pub items: Vec<BulkRegisterEmployeeItemResult>,
}

pub struct EmployeeService {
    pub employee_repo: Arc<dyn EmployeeRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub identity_provisioning_service: Arc<IdentityProvisioningService>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
    license_runtime: Option<Arc<dyn LicenseRuntime>>,
}

impl EmployeeService {
    pub fn new(
        employee_repo: Arc<dyn EmployeeRepository>,
        person_repo: Arc<dyn PersonRepository>,
        group_repo: Arc<dyn GroupRepository>,
        identity_provisioning_service: Arc<IdentityProvisioningService>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            employee_repo,
            person_repo,
            group_repo,
            identity_provisioning_service,
            event_bus,
            permission_checker,
            license_runtime: None,
        }
    }

    pub fn with_license_runtime(mut self, runtime: Arc<dyn LicenseRuntime>) -> Self {
        self.license_runtime = Some(runtime);
        self
    }

    /// Register a new employee with optional unified identity provisioning
    pub async fn register_employee(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: RegisterEmployeeInput,
    ) -> Result<RegisterEmployeeResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeCreate)
            .await?;
        let max_staff = if let Some(runtime) = self.license_runtime.as_ref() {
            runtime.authorize(org_id, None)?;
            let max = runtime.get_limit("staff.max");
            if max.is_some() {
                let counts = self.employee_repo.count_by_status(org_id).await?;
                let ended = counts.get("terminated").copied().unwrap_or(0)
                    + counts.get("resigned").copied().unwrap_or(0);
                let current = counts
                    .get("all")
                    .copied()
                    .unwrap_or(0)
                    .saturating_sub(ended);
                runtime.check_limit("staff.max", current as u64, 1)?;
            }
            max
        } else {
            None
        };

        let now = Utc::now();

        // 1. Resolve or auto-generate employee number
        let employee_number = if let Some(ref num) = input.employee_number {
            let trimmed = num.trim().to_string();
            if trimmed.is_empty() {
                format!(
                    "EMP-{}-{}",
                    now.format("%Y"),
                    Uuid::new_v4()
                        .to_string()
                        .chars()
                        .take(6)
                        .collect::<String>()
                        .to_uppercase()
                )
            } else {
                trimmed
            }
        } else {
            format!(
                "EMP-{}-{}",
                now.format("%Y"),
                Uuid::new_v4()
                    .to_string()
                    .chars()
                    .take(6)
                    .collect::<String>()
                    .to_uppercase()
            )
        };

        if self
            .employee_repo
            .find_by_employee_number(org_id, &employee_number)
            .await?
            .is_some()
        {
            return Err(DomainError::Conflict(format!(
                "Employee number '{}' is already assigned in this organization",
                employee_number
            )));
        }

        // 2. Resolve or find or create Person (Avoid duplicate person creation!)
        let person_id = if let Some(pid) = input.person_id {
            pid
        } else {
            // Check if existing Person exists with same National ID or email or phone
            let existing_person = if let Some(ref nid) = input.national_id {
                if !nid.trim().is_empty() {
                    self.person_repo
                        .find_by_national_id(Some(org_id), nid.trim())
                        .await?
                } else {
                    None
                }
            } else if let Some(ref email) = input.email {
                if !email.trim().is_empty() {
                    self.person_repo
                        .find_by_email(Some(org_id), email.trim())
                        .await?
                } else {
                    None
                }
            } else if let Some(ref phone) = input.phone {
                if !phone.trim().is_empty() {
                    self.person_repo
                        .find_by_phone(Some(org_id), phone.trim())
                        .await?
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(p) = existing_person {
                p.id
            } else {
                let pid = Uuid::new_v4();
                let person = Person {
                    id: pid,
                    org_id: Some(org_id),
                    first_name: input.first_name,
                    middle_name: input.middle_name,
                    last_name: input.last_name,
                    first_name_ar: input.first_name_ar,
                    middle_name_ar: input.middle_name_ar,
                    last_name_ar: input.last_name_ar,
                    display_name: input.display_name,
                    national_id: input.national_id,
                    passport_number: input.passport_number,
                    gender: input.gender,
                    birth_date: input.birth_date,
                    place_of_birth: input.place_of_birth,
                    nationality: input.nationality,
                    blood_type: input.blood_type,
                    medical_notes: input.medical_notes,
                    special_needs_notes: input.special_needs_notes,
                    phone: input.phone,
                    email: input.email,
                    address: input.address,
                    avatar_url: input.avatar_url,
                    created_at: now,
                    updated_at: now,
                };
                self.person_repo.save(&person).await?;
                pid
            }
        };

        // 3. Create Employee Profile
        let employee_id = Uuid::new_v4();
        let hire_date = input.hire_date.unwrap_or_else(|| now.date_naive());
        let status = input.status.unwrap_or(EmployeeStatus::Active);
        let employment_type = input.employment_type.unwrap_or(EmploymentType::FullTime);

        let emp_type = EmployeeType::from(input.employee_type.as_str());
        let normalized_employee_type = emp_type.to_string();

        let mut employee = Employee {
            id: employee_id,
            org_id,
            person_id,
            user_id: None,
            employee_number: employee_number.clone(),
            employee_type: normalized_employee_type,
            job_title: input.job_title,
            department: input.department,
            org_unit_id: input.org_unit_id,
            hire_date,
            status,
            employment_type,
            work_shift: input.work_shift,
            notes: input.notes,
            is_deleted: false,
            deleted_at: None,
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        // 4. Optional Unified Account Provisioning
        let mut provisioning_result = None;
        if let Some(ref acc_config) = input.account {
            if acc_config.enabled {
                let default_role = if employee.is_teacher() {
                    "teacher".to_string()
                } else if matches!(emp_type, EmployeeType::Principal) {
                    "principal".to_string()
                } else if matches!(emp_type, EmployeeType::Accountant) {
                    "accountant".to_string()
                } else {
                    "staff".to_string()
                };

                let assigned_role = acc_config.role.clone().unwrap_or(default_role);

                let prov_res = self
                    .identity_provisioning_service
                    .provision_account(
                        ctx,
                        org_id,
                        ProvisionAccountInput {
                            person_id,
                            actor_type: ActorType::Employee,
                            preferred_username: acc_config.preferred_username.clone(),
                            email: None,
                            phone: None,
                            roles: vec![assigned_role],
                            group_ids: acc_config.group_ids.clone(),
                            delivery_channel: acc_config.delivery_channel,
                            custom_password: acc_config.custom_password.clone(),
                        },
                    )
                    .await?;

                employee.user_id = Some(prov_res.user_id);
                provisioning_result = Some(prov_res);
            }
        }

        self.employee_repo
            .create_with_license_limit(&employee, max_staff)
            .await?;

        // 5. Publish Domain Event
        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::EmployeeCreated {
                    employee_id,
                    person_id,
                    org_id,
                    timestamp: now,
                },
            )
            .await;

        // 6. Fetch full detail
        let detail = self
            .employee_repo
            .find_detail_by_id(employee_id)
            .await?
            .ok_or_else(|| {
                DomainError::Internal("Failed to load created employee detail".into())
            })?;

        Ok(RegisterEmployeeResult {
            detail,
            provisioning: provisioning_result,
        })
    }

    /// Update employee and person demographic details
    pub async fn update_employee(
        &self,
        ctx: &RequestContext,
        employee_id: Uuid,
        input: UpdateEmployeeInput,
    ) -> Result<EmployeeDetail, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeUpdate)
            .await?;

        let mut employee = self
            .employee_repo
            .find_by_id(employee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Employee {} not found", employee_id)))?;

        let mut person = self
            .person_repo
            .find_by_id(employee.person_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Person {} not found", employee.person_id))
            })?;

        let now = Utc::now();

        // Update employee fields
        if let Some(v) = input.employee_type {
            let emp_type = EmployeeType::from(v.as_str());
            employee.employee_type = emp_type.to_string();
        }
        if let Some(v) = input.job_title {
            employee.job_title = Some(v);
        }
        if let Some(v) = input.department {
            employee.department = Some(v);
        }
        if let Some(v) = input.org_unit_id {
            employee.org_unit_id = Some(v);
        }
        if let Some(v) = input.hire_date {
            employee.hire_date = v;
        }
        if let Some(v) = input.status {
            employee.status = v;
        }
        if let Some(v) = input.employment_type {
            employee.employment_type = v;
        }
        if let Some(v) = input.work_shift {
            employee.work_shift = Some(v);
        }
        if let Some(v) = input.notes {
            employee.notes = Some(v);
        }
        employee.updated_at = now;

        // Update person fields
        if let Some(v) = input.first_name {
            person.first_name = v;
        }
        if let Some(v) = input.middle_name {
            person.middle_name = Some(v);
        }
        if let Some(v) = input.last_name {
            person.last_name = v;
        }
        if let Some(v) = input.first_name_ar {
            person.first_name_ar = Some(v);
        }
        if let Some(v) = input.middle_name_ar {
            person.middle_name_ar = Some(v);
        }
        if let Some(v) = input.last_name_ar {
            person.last_name_ar = Some(v);
        }
        if let Some(v) = input.display_name {
            person.display_name = Some(v);
        }
        if let Some(v) = input.national_id {
            person.national_id = Some(v);
        }
        if let Some(v) = input.passport_number {
            person.passport_number = Some(v);
        }
        if let Some(v) = input.gender {
            person.gender = Some(v);
        }
        if let Some(v) = input.birth_date {
            person.birth_date = Some(v);
        }
        if let Some(v) = input.place_of_birth {
            person.place_of_birth = Some(v);
        }
        if let Some(v) = input.nationality {
            person.nationality = Some(v);
        }
        if let Some(v) = input.blood_type {
            person.blood_type = Some(v);
        }
        if let Some(v) = input.medical_notes {
            person.medical_notes = Some(v);
        }
        if let Some(v) = input.special_needs_notes {
            person.special_needs_notes = Some(v);
        }
        if let Some(v) = input.phone {
            person.phone = Some(v);
        }
        if let Some(v) = input.email {
            person.email = Some(v);
        }
        if let Some(v) = input.address {
            person.address = Some(v);
        }
        if let Some(v) = input.avatar_url {
            person.avatar_url = Some(v);
        }
        person.updated_at = now;

        self.person_repo.save(&person).await?;
        self.employee_repo.save(&employee).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::EmployeeUpdated {
                    employee_id,
                    org_id: employee.org_id,
                    timestamp: now,
                },
            )
            .await;

        let detail = self
            .employee_repo
            .find_detail_by_id(employee_id)
            .await?
            .ok_or_else(|| {
                DomainError::Internal("Failed to load updated employee detail".into())
            })?;

        Ok(detail)
    }

    /// Update employee employment status (Active, OnLeave, Suspended, Terminated, Resigned)
    pub async fn update_employee_status(
        &self,
        ctx: &RequestContext,
        employee_id: Uuid,
        new_status: EmployeeStatus,
    ) -> Result<EmployeeDetail, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeUpdate)
            .await?;

        let mut employee = self
            .employee_repo
            .find_by_id(employee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Employee {} not found", employee_id)))?;

        let old_status = employee.status;
        employee.status = new_status;
        employee.updated_at = Utc::now();
        self.employee_repo.save(&employee).await?;

        // If status changed to Terminated or Suspended, optionally toggle linked user account
        if matches!(
            new_status,
            EmployeeStatus::Terminated | EmployeeStatus::Suspended
        ) {
            if let Some(uid) = employee.user_id {
                let _ = self
                    .identity_provisioning_service
                    .toggle_user_status(ctx, uid, false)
                    .await;
            }
        } else if matches!(new_status, EmployeeStatus::Active) {
            if let Some(uid) = employee.user_id {
                let _ = self
                    .identity_provisioning_service
                    .toggle_user_status(ctx, uid, true)
                    .await;
            }
        }

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::EmployeeStatusChanged {
                    employee_id,
                    old_status: old_status.to_string(),
                    new_status: new_status.to_string(),
                    timestamp: Utc::now(),
                },
            )
            .await;

        let detail = self
            .employee_repo
            .find_detail_by_id(employee_id)
            .await?
            .ok_or_else(|| DomainError::Internal("Failed to load employee detail".into()))?;

        Ok(detail)
    }

    /// Provision account for existing employee if they don't have one
    pub async fn provision_employee_account(
        &self,
        ctx: &RequestContext,
        employee_id: Uuid,
        preferred_username: Option<String>,
        role: Option<String>,
        group_ids: Vec<Uuid>,
        delivery_channel: Option<DeliveryChannel>,
    ) -> Result<AccountProvisioningResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::UserWrite)
            .await?;

        let mut employee = self
            .employee_repo
            .find_by_id(employee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Employee {} not found", employee_id)))?;

        let default_role = if employee.is_teacher() {
            "teacher".to_string()
        } else {
            "staff".to_string()
        };

        let res = self
            .identity_provisioning_service
            .provision_account(
                ctx,
                employee.org_id,
                ProvisionAccountInput {
                    person_id: employee.person_id,
                    actor_type: ActorType::Employee,
                    preferred_username,
                    email: None,
                    phone: None,
                    roles: vec![role.unwrap_or(default_role)],
                    group_ids,
                    delivery_channel,
                    custom_password: None,
                },
            )
            .await?;

        employee.user_id = Some(res.user_id);
        employee.updated_at = Utc::now();
        self.employee_repo.save(&employee).await?;

        Ok(res)
    }

    /// Assign employee user to one or more user groups
    pub async fn assign_employee_groups(
        &self,
        ctx: &RequestContext,
        employee_id: Uuid,
        group_ids: Vec<Uuid>,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::GroupAssign)
            .await?;

        let employee = self
            .employee_repo
            .find_by_id(employee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Employee {} not found", employee_id)))?;

        let user_id = employee
            .user_id
            .ok_or_else(|| DomainError::Validation("Employee has no linked user account".into()))?;

        for group_id in group_ids {
            let membership = UserGroupMembership {
                id: Uuid::new_v4(),
                org_id: employee.org_id,
                user_id,
                group_id,
                created_at: Utc::now(),
            };
            let _ = self.group_repo.add_member(&membership).await;
        }

        Ok(())
    }

    /// Get full employee detail by ID
    pub async fn get_employee(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<Option<EmployeeDetail>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeRead)
            .await?;
        self.employee_repo.find_detail_by_id(id).await
    }

    /// List employees with filtering and pagination
    pub async fn list_employees(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        filter: EmployeeFilter,
    ) -> Result<PaginatedList<EmployeeDetail>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeRead)
            .await?;
        self.employee_repo.list(org_id, filter).await
    }

    /// Count employees by status (active, on_leave, terminated, all)
    pub async fn count_employees_by_status(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<std::collections::HashMap<String, i64>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeRead)
            .await?;
        self.employee_repo.count_by_status(org_id).await
    }

    /// Bulk register employees with demographic checks, person deduplication,
    /// account provisioning, group assignments, and atomic all-or-nothing execution.
    pub async fn bulk_register_employees(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: BulkRegisterEmployeesInput,
    ) -> Result<BulkRegisterEmployeeResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeCreate)
            .await?;

        let is_atomic = input.atomic.unwrap_or(true);
        let is_dry_run = input.dry_run.unwrap_or(false);
        let items = input.items;
        let total = items.len();

        // ─────────────────────────────────────────────────────────────────
        // STAGE 1: INTRA-BATCH INTEGRITY & CONFLICT PRE-VALIDATION
        // ─────────────────────────────────────────────────────────────────
        let mut pre_errors: std::collections::HashMap<usize, String> =
            std::collections::HashMap::new();
        let mut seen_nids: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        let mut seen_passports: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        let mut seen_emp_nums: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for item in &items {
            let row = item.row_index;

            // 1. Required Demographics Check
            if item.registration.first_name.trim().is_empty() {
                pre_errors.insert(row, "First name is required".to_string());
                continue;
            }
            if item.registration.last_name.trim().is_empty() {
                pre_errors.insert(row, "Last name is required".to_string());
                continue;
            }
            if item.registration.employee_type.trim().is_empty() {
                pre_errors.insert(row, "Employee type is required".to_string());
                continue;
            }

            // 2. Intra-Batch Duplicate National ID Check
            if let Some(nid) = item
                .registration
                .national_id
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                if let Some(prev_row) = seen_nids.get(nid) {
                    pre_errors.insert(
                        row,
                        format!(
                            "Duplicate National ID '{}' within batch (conflicts with row {})",
                            nid, prev_row
                        ),
                    );
                    continue;
                }
                seen_nids.insert(nid.to_string(), row);
            }

            // 3. Intra-Batch Duplicate Passport Check
            if let Some(pp) = item
                .registration
                .passport_number
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                if let Some(prev_row) = seen_passports.get(pp) {
                    pre_errors.insert(
                        row,
                        format!(
                            "Duplicate Passport Number '{}' within batch (conflicts with row {})",
                            pp, prev_row
                        ),
                    );
                    continue;
                }
                seen_passports.insert(pp.to_string(), row);
            }

            // 4. Intra-Batch Explicit Employee Number Check
            if let Some(ref sn) = item.registration.employee_number {
                let trimmed = sn.trim();
                if !trimmed.is_empty() {
                    if let Some(prev_row) = seen_emp_nums.get(trimmed) {
                        pre_errors.insert(row, format!("Duplicate Employee Number '{}' within batch (conflicts with row {})", trimmed, prev_row));
                        continue;
                    }
                    seen_emp_nums.insert(trimmed.to_string(), row);

                    if let Ok(Some(_)) = self
                        .employee_repo
                        .find_by_employee_number(org_id, trimmed)
                        .await
                    {
                        pre_errors.insert(
                            row,
                            format!(
                                "Employee number '{}' already exists in organization",
                                trimmed
                            ),
                        );
                        continue;
                    }
                }
            }
        }

        // ─────────────────────────────────────────────────────────────────
        // STAGE 2: ATOMIC ALL-OR-NOTHING / DRY-RUN DECISION GATE
        // ─────────────────────────────────────────────────────────────────
        if !pre_errors.is_empty() && is_atomic {
            let mut results = Vec::with_capacity(total);
            for item in &items {
                let row = item.row_index;
                if let Some(err) = pre_errors.get(&row) {
                    results.push(BulkRegisterEmployeeItemResult {
                        row_index: row,
                        success: false,
                        employee_id: None,
                        employee_number: None,
                        user_id: None,
                        error: Some(err.clone()),
                    });
                } else {
                    results.push(BulkRegisterEmployeeItemResult {
                        row_index: row,
                        success: false,
                        employee_id: None,
                        employee_number: None,
                        user_id: None,
                        error: Some("Batch aborted: another record in the batch failed validation (All-or-Nothing policy).".to_string()),
                    });
                }
            }

            return Ok(BulkRegisterEmployeeResult {
                total,
                succeeded: 0,
                failed: total,
                is_atomic: true,
                is_dry_run,
                items: results,
            });
        }

        if is_dry_run {
            let mut succeeded = 0;
            let mut failed = 0;
            let mut results = Vec::with_capacity(total);
            for item in &items {
                let row = item.row_index;
                if let Some(err) = pre_errors.get(&row) {
                    failed += 1;
                    results.push(BulkRegisterEmployeeItemResult {
                        row_index: row,
                        success: false,
                        employee_id: None,
                        employee_number: None,
                        user_id: None,
                        error: Some(err.clone()),
                    });
                } else {
                    succeeded += 1;
                    results.push(BulkRegisterEmployeeItemResult {
                        row_index: row,
                        success: true,
                        employee_id: None,
                        employee_number: item
                            .registration
                            .employee_number
                            .clone()
                            .or_else(|| Some("[Auto-Generated EMP]".to_string())),
                        user_id: None,
                        error: None,
                    });
                }
            }

            return Ok(BulkRegisterEmployeeResult {
                total,
                succeeded,
                failed,
                is_atomic,
                is_dry_run: true,
                items: results,
            });
        }

        // ─────────────────────────────────────────────────────────────────
        // STAGE 3: SEQUENTIAL EXECUTION & PERSISTENCE
        // ─────────────────────────────────────────────────────────────────
        let mut succeeded = 0usize;
        let mut failed = 0usize;
        let mut results = Vec::with_capacity(total);

        for item in items {
            let row_idx = item.row_index;

            if let Some(err) = pre_errors.get(&row_idx) {
                failed += 1;
                results.push(BulkRegisterEmployeeItemResult {
                    row_index: row_idx,
                    success: false,
                    employee_id: None,
                    employee_number: None,
                    user_id: None,
                    error: Some(err.clone()),
                });
                continue;
            }

            match self.register_employee(ctx, org_id, item.registration).await {
                Ok(reg_res) => {
                    succeeded += 1;
                    results.push(BulkRegisterEmployeeItemResult {
                        row_index: row_idx,
                        success: true,
                        employee_id: Some(reg_res.detail.employee.id),
                        employee_number: Some(reg_res.detail.employee.employee_number),
                        user_id: reg_res.detail.employee.user_id,
                        error: None,
                    });
                }
                Err(err) => {
                    failed += 1;
                    results.push(BulkRegisterEmployeeItemResult {
                        row_index: row_idx,
                        success: false,
                        employee_id: None,
                        employee_number: None,
                        user_id: None,
                        error: Some(err.to_string()),
                    });
                }
            }
        }

        Ok(BulkRegisterEmployeeResult {
            total,
            succeeded,
            failed,
            is_atomic,
            is_dry_run: false,
            items: results,
        })
    }

    /// Soft delete an employee
    pub async fn delete_employee(
        &self,
        ctx: &RequestContext,
        employee_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeDelete)
            .await?;

        let mut employee = self
            .employee_repo
            .find_by_id(employee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Employee {} not found", employee_id)))?;

        employee.is_deleted = true;
        employee.deleted_at = Some(Utc::now());
        employee.updated_at = Utc::now();
        employee.updated_by = ctx.user_id();
        self.employee_repo.save(&employee).await?;

        // Also deactivate user account if linked
        if let Some(uid) = employee.user_id {
            let _ = self
                .identity_provisioning_service
                .toggle_user_status(ctx, uid, false)
                .await;
        }

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::EmployeeDeleted {
                    employee_id,
                    org_id: employee.org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Restore a soft-deleted employee
    pub async fn restore_employee(
        &self,
        ctx: &RequestContext,
        employee_id: Uuid,
    ) -> Result<EmployeeDetail, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EmployeeDelete)
            .await?;

        let mut employee = self
            .employee_repo
            .find_by_id(employee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Employee {} not found", employee_id)))?;

        employee.is_deleted = false;
        employee.deleted_at = None;
        employee.updated_at = Utc::now();
        employee.updated_by = ctx.user_id();
        self.employee_repo.save(&employee).await?;

        if let Some(uid) = employee.user_id {
            let _ = self
                .identity_provisioning_service
                .toggle_user_status(ctx, uid, true)
                .await;
        }

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::EmployeeRestored {
                    employee_id,
                    org_id: employee.org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        let detail = self
            .employee_repo
            .find_detail_by_id(employee_id)
            .await?
            .ok_or_else(|| {
                DomainError::Internal("Failed to load restored employee detail".into())
            })?;

        Ok(detail)
    }
}
