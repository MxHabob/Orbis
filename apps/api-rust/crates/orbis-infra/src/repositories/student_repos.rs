use async_trait::async_trait;
use orbis_app::ports::repositories::{
    EnrollmentRepository, EnrollmentSaveResult, StudentGuardianRepository, StudentRepository,
};
use orbis_domain::entities::enrollment::{
    Enrollment as DomainEnrollment, EnrollmentStatus, StudyMode,
};
use orbis_domain::entities::person::Person as DomainPerson;
use orbis_domain::entities::settings::{
    StudentSortCriterion, StudentSortDirection, StudentSortField,
};
use orbis_domain::entities::student::{
    AcademicStanding, Student as DomainStudent, StudentDetail, StudentStatus,
};
use orbis_domain::entities::student_guardian::StudentGuardian as DomainStudentGuardian;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::{EnrollmentFilter, PaginatedList, StudentFilter};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::models::{
    academic_period::{self, Entity as AcademicPeriodEntity},
    enrollment::{self, Entity as EnrollmentEntity},
    person::{self, Entity as PersonEntity},
    section::{self, Entity as SectionEntity},
    student::{self, Entity as StudentEntity},
    student_guardian::{self, Entity as StudentGuardianEntity},
};

pub struct StudentRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl StudentRepository for StudentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainStudent>, DomainError> {
        let model = StudentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_student_model_to_domain))
    }

    async fn find_by_user_id(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<DomainStudent>, DomainError> {
        let model = StudentEntity::find()
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::UserId.eq(user_id))
            .filter(student::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_student_model_to_domain))
    }

    async fn find_detail_by_id(&self, id: Uuid) -> Result<Option<StudentDetail>, DomainError> {
        let pair = StudentEntity::find_by_id(id)
            .find_also_related(PersonEntity)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some((sm, Some(pm))) = pair {
            Ok(Some(StudentDetail {
                student: map_student_model_to_domain(sm),
                person: map_person_model_to_domain(pm),
            }))
        } else {
            Ok(None)
        }
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainStudent>, DomainError> {
        let models = StudentEntity::find()
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::IsDeleted.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_student_model_to_domain)
            .collect())
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: StudentFilter,
    ) -> Result<PaginatedList<StudentDetail>, DomainError> {
        let mut query = StudentEntity::find()
            .find_also_related(PersonEntity)
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::IsDeleted.eq(false));

        if filter.period_id.is_some() || filter.section_id.is_some() {
            let mut enrollment_query = EnrollmentEntity::find()
                .filter(enrollment::Column::OrgId.eq(org_id))
                .filter(
                    enrollment::Column::Status
                        .is_in(["active".to_string(), "pending_approval".to_string()]),
                );
            if let Some(period_id) = filter.period_id {
                enrollment_query =
                    enrollment_query.filter(enrollment::Column::PeriodId.eq(period_id));
            }
            if let Some(section_id) = filter.section_id {
                enrollment_query =
                    enrollment_query.filter(enrollment::Column::SectionId.eq(section_id));
            }
            let enrolled_student_ids = enrollment_query
                .all(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?
                .into_iter()
                .map(|enrollment| enrollment.student_id)
                .collect::<Vec<_>>();
            query = query.filter(student::Column::Id.is_in(enrolled_student_ids));
        }

        if let Some(status) = filter.status {
            query = query.filter(student::Column::Status.eq(status.to_string()));
        }

        if let Some(standing) = filter.standing {
            query = query.filter(student::Column::Standing.eq(standing.to_string()));
        }

        if let Some(ref s) = filter.search {
            let terms: Vec<_> = s
                .split_whitespace()
                .filter(|term| !term.is_empty())
                .collect();
            if !terms.is_empty() {
                let mut all_terms = Condition::all();
                for term in terms {
                    let search_pat = format!("%{term}%");
                    all_terms = all_terms.add(
                        Condition::any()
                            .add(student::Column::StudentNumber.like(&search_pat))
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

        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();
        let sort_criteria = filter
            .sort_criteria
            .filter(|criteria| !criteria.is_empty())
            .unwrap_or_else(|| {
                vec![StudentSortCriterion {
                    field: pagination
                        .sort_by
                        .as_deref()
                        .and_then(StudentSortField::from_key)
                        .unwrap_or_default(),
                    direction: pagination
                        .sort_dir
                        .as_deref()
                        .and_then(StudentSortDirection::from_key)
                        .unwrap_or_default(),
                }]
            });

        for criterion in sort_criteria.iter().take(3) {
            let order = match criterion.direction {
                StudentSortDirection::Asc => Order::Asc,
                StudentSortDirection::Desc => Order::Desc,
            };
            query = match criterion.field {
                StudentSortField::StudentNumber => {
                    query.order_by(student::Column::StudentNumber, order)
                }
                StudentSortField::Name => query
                    .order_by(person::Column::FirstName, order.clone())
                    .order_by(person::Column::MiddleName, order.clone())
                    .order_by(person::Column::LastName, order),
                StudentSortField::CreatedAt => query.order_by(student::Column::CreatedAt, order),
                StudentSortField::AdmittedAt => query.order_by(student::Column::AdmittedAt, order),
                StudentSortField::Status => query.order_by(student::Column::Status, order),
                StudentSortField::BirthDate => query.order_by(person::Column::BirthDate, order),
                StudentSortField::Gender => query.order_by(person::Column::Gender, order),
            };
        }
        if !sort_criteria
            .iter()
            .any(|criterion| criterion.field == StudentSortField::StudentNumber)
        {
            query = query.order_by_asc(student::Column::StudentNumber);
        }
        query = query.order_by_asc(student::Column::Id);

        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .filter_map(|(sm, pm_opt)| {
                let pm = pm_opt.unwrap_or_else(|| person::Model {
                    id: sm.person_id,
                    org_id: Some(sm.org_id),
                    first_name: String::new(),
                    middle_name: None,
                    last_name: String::new(),
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
                    created_at: sm.created_at,
                    updated_at: sm.updated_at,
                });
                Some(StudentDetail {
                    student: map_student_model_to_domain(sm),
                    person: map_person_model_to_domain(pm),
                })
            })
            .collect();

        Ok(PaginatedList::new(items, total_count, page, page_size))
    }

    async fn find_by_student_number(
        &self,
        org_id: Uuid,
        number: &str,
    ) -> Result<Option<DomainStudent>, DomainError> {
        let model = StudentEntity::find()
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::StudentNumber.eq(number))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_student_model_to_domain))
    }

    async fn find_by_national_id(
        &self,
        org_id: Uuid,
        national_id: &str,
    ) -> Result<Option<DomainStudent>, DomainError> {
        let person = crate::models::person::Entity::find()
            .filter(crate::models::person::Column::OrgId.eq(Some(org_id)))
            .filter(crate::models::person::Column::NationalId.eq(Some(national_id.to_string())))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        if let Some(p) = person {
            self.find_by_person_id(p.id).await
        } else {
            Ok(None)
        }
    }

    async fn find_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Result<Option<DomainStudent>, DomainError> {
        let model = StudentEntity::find()
            .filter(student::Column::PersonId.eq(person_id))
            .filter(student::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_student_model_to_domain))
    }

    async fn find_active_details_by_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<StudentDetail>, DomainError> {
        let pairs = StudentEntity::find()
            .find_also_related(PersonEntity)
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::IsDeleted.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(pairs
            .into_iter()
            .filter_map(|(sm, pm)| {
                pm.map(|p| StudentDetail {
                    student: map_student_model_to_domain(sm),
                    person: map_person_model_to_domain(p),
                })
            })
            .collect())
    }

    async fn save(&self, s: &DomainStudent) -> Result<(), DomainError> {
        let active = student_to_active_model(s);

        if StudentEntity::find_by_id(s.id)
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

    async fn create_with_license_limit(
        &self,
        s: &DomainStudent,
        max_students: Option<u64>,
    ) -> Result<(), DomainError> {
        let Some(max_students) = max_students else {
            return StudentRepository::save(self, s).await;
        };

        let transaction = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        crate::repositories::license_quota::acquire_organization_quota_lock(&transaction, s.org_id)
            .await?;

        let current = StudentEntity::find()
            .filter(student::Column::OrgId.eq(s.org_id))
            .filter(student::Column::IsDeleted.eq(false))
            .count(&transaction)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if current.saturating_add(1) > max_students {
            return Err(DomainError::LicenseRequired(format!(
                "The student quota of {max_students} has been reached"
            )));
        }

        student_to_active_model(s)
            .insert(&transaction)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        transaction
            .commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), DomainError> {
        if let Some(s) = StudentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
        {
            let mut active: student::ActiveModel = s.into();
            active.status = Set(status.to_string());
            active.updated_at = Set(chrono::Utc::now());
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn update_status_batch(&self, ids: &[Uuid], status: &str) -> Result<usize, DomainError> {
        let mut count = 0;
        for id in ids {
            self.update_status(*id, status).await?;
            count += 1;
        }
        Ok(count)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        StudentEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

fn student_to_active_model(s: &DomainStudent) -> student::ActiveModel {
    student::ActiveModel {
        id: Set(s.id),
        org_id: Set(s.org_id),
        person_id: Set(s.person_id),
        user_id: Set(s.user_id),
        student_number: Set(s.student_number.clone()),
        status: Set(s.status.to_string()),
        standing: Set(s.standing.to_string()),
        admitted_at: Set(s.admitted_at),
        graduated_at: Set(s.graduated_at),
        is_deleted: Set(s.is_deleted),
        deleted_at: Set(s.deleted_at),
        meta: Set(Some(s.meta.clone())),
        org_unit_id: Set(s.org_unit_id),
        created_by: Set(s.created_by),
        updated_by: Set(s.updated_by),
        created_at: Set(s.created_at),
        updated_at: Set(s.updated_at),
    }
}

fn map_student_model_to_domain(m: student::Model) -> DomainStudent {
    DomainStudent {
        id: m.id,
        org_id: m.org_id,
        person_id: m.person_id,
        user_id: m.user_id,
        student_number: m.student_number,
        status: match m.status.as_str() {
            "inactive" => StudentStatus::Inactive,
            "on_leave" => StudentStatus::OnLeave,
            "suspended" => StudentStatus::Suspended,
            "withdrawn" => StudentStatus::Withdrawn,
            "transferred_out" => StudentStatus::TransferredOut,
            "graduated" => StudentStatus::Graduated,
            "expelled" => StudentStatus::Expelled,
            _ => StudentStatus::Active,
        },
        standing: match m.standing.as_str() {
            "honor_list" => AcademicStanding::HonorList,
            "warning_1" => AcademicStanding::AcademicWarning1,
            "warning_2" => AcademicStanding::AcademicWarning2,
            "warning_3" => AcademicStanding::AcademicWarning3,
            "probation" => AcademicStanding::Probation,
            "dismissed" => AcademicStanding::Dismissed,
            _ => AcademicStanding::GoodStanding,
        },
        admitted_at: m.admitted_at,
        graduated_at: m.graduated_at,
        is_deleted: m.is_deleted,
        deleted_at: m.deleted_at,
        meta: m.meta.unwrap_or_default(),
        org_unit_id: m.org_unit_id,
        created_by: m.created_by,
        updated_by: m.updated_by,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_person_model_to_domain(m: person::Model) -> DomainPerson {
    DomainPerson {
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

#[async_trait]
impl EnrollmentRepository for StudentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainEnrollment>, DomainError> {
        let model = EnrollmentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_enrollment_model_to_domain))
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: EnrollmentFilter,
    ) -> Result<PaginatedList<DomainEnrollment>, DomainError> {
        let mut query = EnrollmentEntity::find().filter(enrollment::Column::OrgId.eq(org_id));

        if let Some(period_id) = filter.period_id {
            query = query.filter(enrollment::Column::PeriodId.eq(period_id));
        }

        if let Some(section_id) = filter.section_id {
            query = query.filter(enrollment::Column::SectionId.eq(section_id));
        }

        if let Some(student_id) = filter.student_id {
            query = query.filter(enrollment::Column::StudentId.eq(student_id));
        }

        if let Some(status) = filter.status {
            query = query.filter(enrollment::Column::Status.eq(status.to_string()));
        }

        if let Some(study_mode) = filter.study_mode {
            query = query.filter(enrollment::Column::StudyMode.eq(study_mode.to_string()));
        }

        query = query.order_by_desc(enrollment::Column::EnrolledAt);

        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();

        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(map_enrollment_model_to_domain)
            .collect();
        Ok(PaginatedList::new(items, total_count, page, page_size))
    }

    async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_enrollment_model_to_domain)
            .collect())
    }

    async fn find_by_section(
        &self,
        section_id: Uuid,
    ) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::SectionId.eq(section_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_enrollment_model_to_domain)
            .collect())
    }

    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_enrollment_model_to_domain)
            .collect())
    }

    async fn save(&self, e: &DomainEnrollment) -> Result<(), DomainError> {
        let active = enrollment_to_active_model(e);

        if EnrollmentEntity::find_by_id(e.id)
            .one(&self.db)
            .await
            .map_err(|err| DomainError::Internal(err.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|err| DomainError::Internal(err.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|err| DomainError::Internal(err.to_string()))?;
        }
        Ok(())
    }

    async fn save_with_capacity(
        &self,
        enrollment_record: &mut DomainEnrollment,
        default_capacity: i32,
        allow_waitlist: bool,
        capacity_override: Option<
            orbis_domain::entities::override_request::AcademicOverrideRequest,
        >,
        admission_override: Option<
            orbis_domain::entities::override_request::AcademicOverrideRequest,
        >,
    ) -> Result<EnrollmentSaveResult, DomainError> {
        let transaction = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if let Some(section_id) = enrollment_record.section_id {
            crate::repositories::enrollment_capacity::acquire_capacity_lock(
                &transaction,
                enrollment_record.org_id,
                section_id,
                crate::repositories::enrollment_capacity::CapacityLockTarget::ClassSection,
            )
            .await?;
        }

        let existing = EnrollmentEntity::find_by_id(enrollment_record.id)
            .one(&transaction)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if existing
            .as_ref()
            .is_some_and(|record| record.org_id != enrollment_record.org_id)
        {
            return Err(DomainError::Forbidden(
                "Cannot update an enrollment owned by another organization".into(),
            ));
        }

        let period_exists = AcademicPeriodEntity::find_by_id(enrollment_record.period_id)
            .filter(academic_period::Column::OrgId.eq(enrollment_record.org_id))
            .one(&transaction)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .is_some();
        if !period_exists {
            return Err(DomainError::NotFound("Academic period not found".into()));
        }

        let student_exists = StudentEntity::find_by_id(enrollment_record.student_id)
            .filter(student::Column::OrgId.eq(enrollment_record.org_id))
            .filter(student::Column::IsDeleted.eq(false))
            .one(&transaction)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .is_some();
        if !student_exists {
            return Err(DomainError::NotFound("Student not found".into()));
        }

        let capacity_breach = if let Some(section_id) = enrollment_record.section_id {
            let section = SectionEntity::find_by_id(section_id)
                .filter(section::Column::OrgId.eq(enrollment_record.org_id))
                .filter(section::Column::IsActive.eq(true))
                .filter(section::Column::IsDeleted.eq(false))
                .one(&transaction)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?
                .ok_or_else(|| DomainError::NotFound("Class section not found".into()))?;
            let capacity = section.capacity.unwrap_or(default_capacity).max(0);

            let active_count = EnrollmentEntity::find()
                .filter(enrollment::Column::OrgId.eq(enrollment_record.org_id))
                .filter(enrollment::Column::SectionId.eq(Some(section_id)))
                .filter(enrollment::Column::Status.eq(EnrollmentStatus::Active.to_string()))
                .filter(enrollment::Column::Id.ne(enrollment_record.id))
                .count(&transaction)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            let active_count_i32 = i32::try_from(active_count)
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            let capacity_reached = active_count_i32 >= capacity;
            if capacity_reached && !allow_waitlist {
                return Err(DomainError::Validation(format!(
                    "Class capacity exceeded ({active_count_i32}/{capacity})"
                )));
            }

            let duplicate = EnrollmentEntity::find()
                .filter(enrollment::Column::OrgId.eq(enrollment_record.org_id))
                .filter(enrollment::Column::StudentId.eq(enrollment_record.student_id))
                .filter(enrollment::Column::PeriodId.eq(enrollment_record.period_id))
                .filter(enrollment::Column::SectionId.eq(Some(section_id)))
                .filter(enrollment::Column::AttemptNumber.eq(enrollment_record.attempt_number))
                .filter(enrollment::Column::Id.ne(enrollment_record.id))
                .one(&transaction)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            if duplicate.is_some() {
                return Err(DomainError::Conflict(
                    "Student already has an enrollment record for this class and period".into(),
                ));
            }

            if capacity_reached {
                enrollment_record.status = EnrollmentStatus::PendingApproval;
            }
            capacity_reached.then_some((active_count_i32, capacity))
        } else {
            None
        };

        let override_request = if let Some((active_count, capacity)) = capacity_breach {
            let mut request = capacity_override.ok_or_else(|| {
                DomainError::Internal(
                    "A capacity override request is required for a waitlisted enrollment".into(),
                )
            })?;
            if request.org_id != enrollment_record.org_id
                || request.entity_id != enrollment_record.student_id
            {
                return Err(DomainError::Validation(
                    "Capacity override request does not match the enrollment".into(),
                ));
            }
            let section_id = enrollment_record.section_id.ok_or_else(|| {
                DomainError::Internal(
                    "A capacity breach was reported without an assigned section".into(),
                )
            })?;
            request.reason =
                format!("Section capacity reached ({active_count}/{capacity}); review required.");
            request.metadata = serde_json::json!({
                "section_id": section_id,
                "active_count": active_count,
                "max_capacity": capacity
            });
            Some(request)
        } else {
            admission_override
        };

        let active = enrollment_to_active_model(enrollment_record);
        if existing.is_some() {
            active
                .update(&transaction)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
        } else {
            active
                .insert(&transaction)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
        }
        if let Some(request) = override_request.as_ref() {
            if request.org_id != enrollment_record.org_id
                || request.entity_id != enrollment_record.student_id
            {
                return Err(DomainError::Validation(
                    "Approval request does not match the enrollment".into(),
                ));
            }
            crate::repositories::override_repos::override_to_active_model(request)?
                .insert(&transaction)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
        }
        transaction
            .commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;

        Ok(EnrollmentSaveResult {
            capacity_breach,
            override_request,
        })
    }

    async fn save_batch(&self, enrollments: &[DomainEnrollment]) -> Result<(), DomainError> {
        for e in enrollments {
            EnrollmentRepository::save(self, e).await?;
        }
        Ok(())
    }

    async fn update_status_batch(
        &self,
        enrollment_ids: &[Uuid],
        status: EnrollmentStatus,
    ) -> Result<(), DomainError> {
        for id in enrollment_ids {
            if let Some(mut e) = EnrollmentRepository::find_by_id(self, *id).await? {
                e.status = status;
                e.updated_at = chrono::Utc::now();
                EnrollmentRepository::save(self, &e).await?;
            }
        }
        Ok(())
    }
}

pub(super) fn enrollment_to_active_model(e: &DomainEnrollment) -> enrollment::ActiveModel {
    enrollment::ActiveModel {
        id: Set(e.id),
        org_id: Set(e.org_id),
        period_id: Set(e.period_id),
        student_id: Set(e.student_id),
        section_id: Set(e.section_id),
        student_number: Set(e.student_number.clone()),
        status: Set(e.status.to_string()),
        study_mode: Set(Some(e.study_mode.to_string())),
        attempt_number: Set(e.attempt_number),
        is_attendance_exempt: Set(Some(e.is_attendance_exempt)),
        is_grade_exempt: Set(Some(e.is_grade_exempt)),
        enrolled_at: Set(e.enrolled_at.into()),
        enrollment_type: Set(Some(e.study_mode.to_string())),
        withdrawn_at: Set(e.withdrawn_at.map(Into::into)),
        withdrawal_reason: Set(e.withdrawal_reason.clone()),
        created_by: Set(e.created_by),
        updated_by: Set(e.updated_by),
        meta: Set(Some(e.meta.clone())),
        created_at: Set(e.created_at.into()),
        updated_at: Set(e.updated_at.into()),
    }
}

fn map_enrollment_model_to_domain(m: enrollment::Model) -> DomainEnrollment {
    DomainEnrollment {
        id: m.id,
        org_id: m.org_id,
        period_id: m.period_id,
        student_id: m.student_id,
        section_id: m.section_id,
        attempt_number: m.attempt_number,
        student_number: m.student_number,
        study_mode: match m.study_mode.as_deref().unwrap_or("regular") {
            "homeschooled" => StudyMode::Homeschooled,
            "auditor" => StudyMode::Auditor,
            "visiting" => StudyMode::Visiting,
            "distance_learning" => StudyMode::DistanceLearning,
            "part_time" => StudyMode::PartTime,
            "exchange" => StudyMode::Exchange,
            "special_needs" => StudyMode::SpecialNeeds,
            _ => StudyMode::Regular,
        },
        status: match m.status.as_str() {
            "pending_approval" => EnrollmentStatus::PendingApproval,
            "on_hold" => EnrollmentStatus::OnHold,
            "withdrawn" => EnrollmentStatus::Withdrawn,
            "transferred" => EnrollmentStatus::Transferred,
            "graduated" => EnrollmentStatus::Graduated,
            "suspended" => EnrollmentStatus::Suspended,
            _ => EnrollmentStatus::Active,
        },
        is_attendance_exempt: m.is_attendance_exempt.unwrap_or(false),
        is_grade_exempt: m.is_grade_exempt.unwrap_or(false),
        enrolled_at: m.enrolled_at.into(),
        withdrawn_at: m.withdrawn_at.map(Into::into),
        withdrawal_reason: m.withdrawal_reason,
        meta: m.meta.unwrap_or_default(),
        created_by: m.created_by,
        updated_by: m.updated_by,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

fn map_guardian_model_to_domain(m: student_guardian::Model) -> DomainStudentGuardian {
    DomainStudentGuardian {
        id: m.id,
        student_id: m.student_id,
        guardian_id: m.guardian_id,
        relationship: m.relationship,
        relationship_ar: m.relationship_ar,
        is_primary: m.is_primary,
        is_emergency_contact: m.is_emergency_contact,
        is_authorized_pickup: m.is_authorized_pickup,
        is_payer: m.is_payer,
        payment_share_percent: m.payment_share_percent,
        occupation: m.occupation,
        employer: m.employer,
        work_phone: m.work_phone,
        notes: m.notes,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl StudentGuardianRepository for StudentRepoImpl {
    async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<DomainStudentGuardian>, DomainError> {
        let models = StudentGuardianEntity::find()
            .filter(student_guardian::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_guardian_model_to_domain)
            .collect())
    }

    async fn find_by_guardian(
        &self,
        guardian_id: Uuid,
    ) -> Result<Vec<DomainStudentGuardian>, DomainError> {
        let models = StudentGuardianEntity::find()
            .filter(student_guardian::Column::GuardianId.eq(guardian_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_guardian_model_to_domain)
            .collect())
    }

    async fn save(&self, g: &DomainStudentGuardian) -> Result<(), DomainError> {
        let active = student_guardian::ActiveModel {
            id: Set(g.id),
            org_id: Set(None),
            student_id: Set(g.student_id),
            guardian_id: Set(g.guardian_id),
            relationship: Set(g.relationship.clone()),
            relationship_ar: Set(g.relationship_ar.clone()),
            is_primary: Set(g.is_primary),
            is_emergency_contact: Set(g.is_emergency_contact),
            is_authorized_pickup: Set(g.is_authorized_pickup),
            is_payer: Set(g.is_payer),
            payment_share_percent: Set(g.payment_share_percent),
            occupation: Set(g.occupation.clone()),
            employer: Set(g.employer.clone()),
            work_phone: Set(g.work_phone.clone()),
            notes: Set(g.notes.clone()),
            created_at: Set(g.created_at),
            updated_at: Set(g.updated_at),
        };

        if StudentGuardianEntity::find_by_id(g.id)
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

    async fn delete(&self, student_id: Uuid, guardian_id: Uuid) -> Result<(), DomainError> {
        student_guardian::Entity::delete_many()
            .filter(student_guardian::Column::StudentId.eq(student_id))
            .filter(student_guardian::Column::GuardianId.eq(guardian_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ConnectionTrait, Database, Schema};

    #[tokio::test]
    async fn capacity_is_checked_atomically_and_waitlisted_enrollments_are_not_active() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared("PRAGMA foreign_keys = OFF;")
            .await
            .unwrap();
        let schema = Schema::new(db.get_database_backend());
        for statement in [
            schema
                .create_table_from_entity(AcademicPeriodEntity)
                .if_not_exists()
                .to_owned(),
            schema
                .create_table_from_entity(StudentEntity)
                .if_not_exists()
                .to_owned(),
            schema
                .create_table_from_entity(SectionEntity)
                .if_not_exists()
                .to_owned(),
            schema
                .create_table_from_entity(EnrollmentEntity)
                .if_not_exists()
                .to_owned(),
            schema
                .create_table_from_entity(crate::models::override_request::Entity)
                .if_not_exists()
                .to_owned(),
        ] {
            db.execute(db.get_database_backend().build(&statement))
                .await
                .unwrap();
        }
        let org_id = Uuid::new_v4();
        let period_id = Uuid::new_v4();
        let section_id = Uuid::new_v4();
        let first_student_id = Uuid::new_v4();
        let second_student_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        academic_period::ActiveModel {
            id: Set(period_id),
            org_id: Set(org_id),
            calendar_id: Set(Uuid::new_v4()),
            period_type_id: Set(Uuid::new_v4()),
            name: Set("Term".into()),
            period_kind: Set("teaching".into()),
            is_enrollment_open: Set(true),
            is_grade_entry_open: Set(false),
            status: Set("active".into()),
            is_locked: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        for student_id in [first_student_id, second_student_id] {
            student::ActiveModel {
                id: Set(student_id),
                org_id: Set(org_id),
                person_id: Set(Uuid::new_v4()),
                student_number: Set(student_id.to_string()),
                status: Set("active".into()),
                standing: Set("good".into()),
                admitted_at: Set(now),
                is_deleted: Set(false),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            }
            .insert(&db)
            .await
            .unwrap();
        }

        section::ActiveModel {
            id: Set(section_id),
            org_id: Set(org_id),
            section_type: Set("class".into()),
            code: Set("A".into()),
            name: Set("Grade 1 / A".into()),
            capacity: Set(Some(1)),
            is_active: Set(true),
            is_deleted: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let repository = StudentRepoImpl { db };
        let mut first = DomainEnrollment::new(
            org_id,
            period_id,
            first_student_id,
            Some(section_id),
            "S-001".into(),
            StudyMode::Regular,
        );
        assert!(
            repository
                .save_with_capacity(&mut first, 30, false, None, None)
                .await
                .unwrap()
                .capacity_breach
                .is_none()
        );

        let mut second = DomainEnrollment::new(
            org_id,
            period_id,
            second_student_id,
            Some(section_id),
            "S-002".into(),
            StudyMode::Regular,
        );
        assert!(matches!(
            repository
                .save_with_capacity(&mut second, 30, false, None, None)
                .await,
            Err(DomainError::Validation(_))
        ));
        let capacity_override = orbis_domain::entities::override_request::AcademicOverrideRequest {
            id: Uuid::new_v4(),
            org_id,
            override_type: orbis_domain::entities::override_request::OverrideType::CapacityOverride,
            risk_level: orbis_domain::entities::override_request::RiskLevel::Medium,
            entity_type: "enrollment".into(),
            entity_id: second_student_id,
            field_name: Some("section_id".into()),
            old_value: None,
            new_value: Some(section_id.to_string()),
            reason: "Section capacity reached; review required.".into(),
            requested_by: Uuid::new_v4(),
            requested_by_name: None,
            status: orbis_domain::entities::override_request::OverrideStatus::Pending,
            current_approver_role: Some("admin".into()),
            approval_chain: vec![],
            approval_policy_version: None,
            resolved_by: None,
            resolved_by_name: None,
            resolved_at: None,
            resolution_notes: None,
            expires_at: None,
            metadata: serde_json::json!({ "section_id": section_id }),
            created_at: now,
            updated_at: now,
        };
        let save_result = repository
            .save_with_capacity(&mut second, 30, true, Some(capacity_override), None)
            .await
            .unwrap();

        assert_eq!(save_result.capacity_breach, Some((1, 1)));
        assert_eq!(second.status, EnrollmentStatus::PendingApproval);
        assert!(save_result.override_request.is_some());
        assert_eq!(
            EnrollmentEntity::find_by_id(second.id)
                .one(&repository.db)
                .await
                .unwrap()
                .unwrap()
                .status,
            EnrollmentStatus::PendingApproval.to_string()
        );
        assert_eq!(
            crate::models::override_request::Entity::find()
                .count(&repository.db)
                .await
                .unwrap(),
            1
        );
    }
}
