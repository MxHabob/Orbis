use async_trait::async_trait;
use orbis_app::ports::repositories::{StudentRepository, EnrollmentRepository, StudentGuardianRepository};
use orbis_domain::entities::student::{Student as DomainStudent, StudentDetail, StudentStatus, AcademicStanding};
use orbis_domain::entities::person::Person as DomainPerson;
use orbis_domain::entities::enrollment::{Enrollment as DomainEnrollment, StudyMode, EnrollmentStatus};
use orbis_domain::entities::student_guardian::StudentGuardian as DomainStudentGuardian;
use orbis_domain::queries::filters::{StudentFilter, EnrollmentFilter, PaginatedList};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait};
use uuid::Uuid;

use crate::models::{
    student::{self, Entity as StudentEntity},
    person::{self, Entity as PersonEntity},
    enrollment::{self, Entity as EnrollmentEntity},
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
        Ok(models.into_iter().map(map_student_model_to_domain).collect())
    }

    async fn list(&self, org_id: Uuid, filter: StudentFilter) -> Result<PaginatedList<StudentDetail>, DomainError> {
        let mut query = StudentEntity::find()
            .find_also_related(PersonEntity)
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::IsDeleted.eq(false));

        if let Some(status) = filter.status {
            query = query.filter(student::Column::Status.eq(status.to_string()));
        }

        if let Some(standing) = filter.standing {
            query = query.filter(student::Column::Standing.eq(standing.to_string()));
        }

        if let Some(ref s) = filter.search {
            if !s.is_empty() {
                let search_pat = format!("%{}%", s);
                query = query.filter(
                    student::Column::StudentNumber.like(&search_pat)
                        .or(student::Column::FullName.like(&search_pat))
                        .or(student::Column::NationalId.like(&search_pat))
                );
            }
        }

        query = query.order_by_asc(student::Column::StudentNumber);

        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();

        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator.num_items().await.map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator.fetch_page(page - 1).await.map_err(|e| DomainError::Internal(e.to_string()))?;

        let items = rows.into_iter().filter_map(|(sm, pm_opt)| {
            let pm = pm_opt.unwrap_or_else(|| person::Model {
                id: sm.person_id,
                org_id: Some(sm.org_id),
                first_name: sm.first_name.clone(),
                middle_name: None,
                last_name: sm.last_name.clone(),
                first_name_ar: None,
                middle_name_ar: None,
                last_name_ar: None,
                display_name: None,
                national_id: sm.national_id.clone(),
                passport_number: sm.passport_number.clone(),
                gender: Some(sm.gender.clone()),
                birth_date: sm.date_of_birth.map(|d| d.to_string()),
                place_of_birth: sm.place_of_birth.clone(),
                nationality: sm.nationality.clone(),
                blood_type: sm.blood_type.clone(),
                medical_notes: sm.medical_notes.clone(),
                special_needs_notes: sm.special_needs_notes.clone(),
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
        }).collect();

        Ok(PaginatedList::new(items, total_count, page, page_size))
    }

    async fn find_by_student_number(&self, org_id: Uuid, number: &str) -> Result<Option<DomainStudent>, DomainError> {
        let model = StudentEntity::find()
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::StudentNumber.eq(number))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_student_model_to_domain))
    }

    async fn find_by_national_id(&self, org_id: Uuid, national_id: &str) -> Result<Option<DomainStudent>, DomainError> {
        let model = StudentEntity::find()
            .filter(student::Column::OrgId.eq(org_id))
            .filter(student::Column::NationalId.eq(national_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_student_model_to_domain))
    }

    async fn save(&self, s: &DomainStudent) -> Result<(), DomainError> {
        let active = student::ActiveModel {
            id: Set(s.id),
            org_id: Set(s.org_id),
            person_id: Set(s.person_id),
            user_id: Set(s.user_id),
            student_number: Set(s.student_number.clone()),
            national_id: Set(None),
            passport_number: Set(None),
            first_name: Set("".into()),
            last_name: Set("".into()),
            full_name: Set("".into()),
            name_ar: Set(None),
            gender: Set("male".into()),
            date_of_birth: Set(None),
            place_of_birth: Set(None),
            nationality: Set(None),
            blood_type: Set(None),
            medical_notes: Set(None),
            special_needs_notes: Set(None),
            org_unit_id: Set(s.org_unit_id),
            status: Set(s.status.to_string()),
            standing: Set(s.standing.to_string()),
            admitted_at: Set(s.admitted_at),
            graduated_at: Set(s.graduated_at),
            is_deleted: Set(s.is_deleted),
            deleted_at: Set(s.deleted_at),
            created_by: Set(s.created_by),
            updated_by: Set(s.updated_by),
            meta: Set(Some(s.meta.clone())),
            created_at: Set(s.created_at),
            updated_at: Set(s.updated_at),
        };

        if StudentEntity::find_by_id(s.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), DomainError> {
        if let Some(s) = StudentEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))? {
            let mut active: student::ActiveModel = s.into();
            active.status = Set(status.to_string());
            active.updated_at = Set(chrono::Utc::now());
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
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

fn map_student_model_to_domain(m: student::Model) -> DomainStudent {
    DomainStudent {
        id: m.id,
        org_id: m.org_id,
        person_id: m.person_id,
        user_id: m.user_id,
        student_number: m.student_number,
        status: match m.status.as_str() {
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
        birth_date: m.birth_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
        place_of_birth: m.place_of_birth,
        nationality: m.nationality,
        blood_type: m.blood_type,
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

    async fn list(&self, org_id: Uuid, filter: EnrollmentFilter) -> Result<PaginatedList<DomainEnrollment>, DomainError> {
        let mut query = EnrollmentEntity::find()
            .filter(enrollment::Column::OrgId.eq(org_id));

        if let Some(period_id) = filter.period_id {
            query = query.filter(enrollment::Column::PeriodId.eq(period_id));
        }

        if let Some(class_id) = filter.class_id {
            query = query.filter(enrollment::Column::ClassId.eq(class_id));
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
        let total_count = paginator.num_items().await.map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator.fetch_page(page - 1).await.map_err(|e| DomainError::Internal(e.to_string()))?;

        let items = rows.into_iter().map(map_enrollment_model_to_domain).collect();
        Ok(PaginatedList::new(items, total_count, page, page_size))
    }

    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_enrollment_model_to_domain).collect())
    }

    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_enrollment_model_to_domain).collect())
    }

    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_enrollment_model_to_domain).collect())
    }

    async fn save(&self, e: &DomainEnrollment) -> Result<(), DomainError> {
        let active = enrollment::ActiveModel {
            id: Set(e.id),
            org_id: Set(e.org_id),
            period_id: Set(e.period_id),
            student_id: Set(e.student_id),
            class_id: Set(e.class_id),
            student_number: Set(e.student_number.clone()),
            status: Set(e.status.to_string()),
            study_mode: Set(Some(e.study_mode.to_string())),
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
        };

        if EnrollmentEntity::find_by_id(e.id).one(&self.db).await.map_err(|err| DomainError::Internal(err.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|err| DomainError::Internal(err.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|err| DomainError::Internal(err.to_string()))?;
        }
        Ok(())
    }

    async fn save_batch(&self, enrollments: &[DomainEnrollment]) -> Result<(), DomainError> {
        for e in enrollments {
            EnrollmentRepository::save(self, e).await?;
        }
        Ok(())
    }

    async fn update_status_batch(&self, enrollment_ids: &[Uuid], status: EnrollmentStatus) -> Result<(), DomainError> {
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

fn map_enrollment_model_to_domain(m: enrollment::Model) -> DomainEnrollment {
    DomainEnrollment {
        id: m.id,
        org_id: m.org_id,
        period_id: m.period_id,
        student_id: m.student_id,
        class_id: m.class_id,
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
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<DomainStudentGuardian>, DomainError> {
        let models = StudentGuardianEntity::find()
            .filter(student_guardian::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_guardian_model_to_domain).collect())
    }

    async fn find_by_guardian(&self, guardian_id: Uuid) -> Result<Vec<DomainStudentGuardian>, DomainError> {
        let models = StudentGuardianEntity::find()
            .filter(student_guardian::Column::GuardianId.eq(guardian_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_guardian_model_to_domain).collect())
    }

    async fn save(&self, g: &DomainStudentGuardian) -> Result<(), DomainError> {
        let active = student_guardian::ActiveModel {
            id: Set(g.id),
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

        if StudentGuardianEntity::find_by_id(g.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
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
