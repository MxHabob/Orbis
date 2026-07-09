use serde::{Deserialize, Serialize};

use crate::service::user::CreateUserRequest;
use crate::service::enrollment::CreateEnrollmentRequest;

// ── Request / Response DTOs ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkStudentRow {
    // Personal
    pub full_name: String,
    pub name_ar: Option<String>,
    pub email: String,
    pub password: Option<String>,
    pub phone: Option<String>,
    pub national_id: Option<String>,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub place_of_birth: Option<String>,
    pub passport_number: Option<String>,
    // Address
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    // Medical
    pub blood_type: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact: Option<String>,
    // Enrollment
    pub class_id: Option<String>,
    pub enrollment_type: Option<String>,
    pub enrollment_status: Option<String>,
    // Guardian
    pub guardian_name: Option<String>,
    pub guardian_phone: Option<String>,
    pub guardian_relationship: Option<String>,
    // Finance
    pub discount_template_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkGradeRow {
    pub student_identifier: String,
    pub identifier_type: Option<String>, // email | national_id | student_number | name | auto
    pub subject_name: String,
    pub score: f64,
    pub max_score: Option<f64>,
    pub exam_type: Option<String>,
    pub semester: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkRowError {
    pub row_index: usize,
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkImportResult {
    pub total: usize,
    pub created: usize,
    pub failed: usize,
    pub errors: Vec<BulkRowError>,
    pub active_cycle_name: Option<String>,
}

// ── Service Implementation ──────────────────────────────────────────────────

impl super::Service {
    /// Bulk create students using the SAME service as single-entry (create_user_in_org + create_enrollment).
    /// This ensures identical business rules: email uniqueness, role validation, fee assignment, etc.
    pub async fn bulk_create_users(
        &self,
        org_id: &str,
        created_by: &str,
        rows: Vec<BulkStudentRow>,
        default_role: &str,
    ) -> Result<BulkImportResult, String> {
        let total = rows.len();
        let mut created = 0usize;
        let mut failed = 0usize;
        let mut errors: Vec<BulkRowError> = Vec::new();

        for (idx, row) in rows.iter().enumerate() {
            let row_index = idx + 1;

            // ── Quick pre-validation ──
            let trimmed_name = row.full_name.trim();
            if trimmed_name.is_empty() || trimmed_name.len() < 2 {
                errors.push(BulkRowError {
                    row_index,
                    field: "full_name".to_string(),
                    message: "الاسم مطلوب (حرفان على الأقل)".to_string(),
                });
                failed += 1;
                continue;
            }

            let trimmed_email = row.email.trim().to_lowercase();
            let is_email_auto = trimmed_email.is_empty();
            let user_email = if is_email_auto {
                format!("temp_stu_{}@temp.orbis", &uuid::Uuid::new_v4().to_string()[..8])
            } else {
                trimmed_email.clone()
            };

            // ── Use create_user_in_org (same as single-entry) ──
            let password = row.password.as_deref().unwrap_or("Orbis@2026").to_string();

            let req = CreateUserRequest {
                email: user_email,
                password,
                full_name: trimmed_name.to_string(),
                name_ar: row.name_ar.clone(),
                primary_role: default_role.to_string(),
                sex: row.sex.clone(),
                phone: row.phone.clone(),
                national_id: row.national_id.clone(),
                date_of_birth: row.date_of_birth.clone(),
            };

            let user = match self.create_user_in_org(org_id, req, created_by).await {
                Ok(u) => u,
                Err(e) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "user".to_string(),
                        message: e,
                    });
                    failed += 1;
                    continue;
                }
            };

            // ── Update additional fields not covered by CreateUserRequest ──
            let has_extra = row.nationality.is_some() || row.address.is_some()
                || row.city.is_some() || row.country.is_some()
                || row.blood_type.is_some() || row.emergency_contact.is_some()
                || row.allergies.is_some() || row.medical_conditions.is_some();

            if has_extra {
                let update_req = crate::service::user::UpdateUserRequest {
                    full_name: None,
                    name_ar: None,
                    name_en: None,
                    email: None,
                    phone: None,
                    sex: None,
                    date_of_birth: None,
                    nationality: row.nationality.clone(),
                    national_id: None,
                    address: row.address.clone(),
                    city: row.city.clone(),
                    country: row.country.clone(),
                    emergency_contact: row.emergency_contact.clone(),
                    blood_type: row.blood_type.clone(),
                    medical_conditions: row.medical_conditions.clone(),
                    avatar: None,
                    status: None,
                };
                let _ = self.update_user(&user.id, update_req).await;
            }

            // ── Create enrollment if class_id provided ──
            let mut generated_student_number: Option<String> = None;
            if let Some(ref class_id) = row.class_id {
                let trimmed_class = class_id.trim();
                if !trimmed_class.is_empty() {
                    let enroll_req = CreateEnrollmentRequest {
                        student_id: user.id.clone(),
                        class_id: trimmed_class.to_string(),
                        cycle_id: None, // auto-detect current cycle
                        enrollment_type: row.enrollment_type.clone(),
                        discount_template_id: row.discount_template_id.clone(),
                        custom_discount_amount: None,
                        status: row.enrollment_status.clone(),
                    };

                    match self.create_enrollment(org_id, enroll_req).await {
                        Ok(enr) => {
                            generated_student_number = Some(enr.student_number);
                        }
                        Err(e) => {
                            // User created but enrollment failed — log error but don't fail the row
                            errors.push(BulkRowError {
                                row_index,
                                field: "enrollment".to_string(),
                                message: format!("تم إنشاء المستخدم لكن فشل التسجيل: {}", e),
                            });
                        }
                    }
                }
            }

            // ── Update email with the student number if email is auto-generated ──
            if is_email_auto {
                let final_email = if let Some(ref student_num) = generated_student_number {
                    format!("{}@orbis.edu", student_num.to_lowercase())
                } else {
                    format!("stu-temp-{}@orbis.edu", &uuid::Uuid::new_v4().to_string()[..8])
                };
                let update_email_req = crate::service::user::UpdateUserRequest {
                    full_name: None,
                    name_ar: None,
                    name_en: None,
                    email: Some(final_email),
                    phone: None,
                    sex: None,
                    date_of_birth: None,
                    nationality: None,
                    national_id: None,
                    address: None,
                    city: None,
                    country: None,
                    emergency_contact: None,
                    blood_type: None,
                    medical_conditions: None,
                    avatar: None,
                    status: None,
                };
                let _ = self.update_user(&user.id, update_email_req).await;
            }

            // ── Create guardian if provided ──
            if let Some(ref guardian_name) = row.guardian_name {
                let trimmed_gname = guardian_name.trim();
                if !trimmed_gname.is_empty() {
                    // Create guardian as a parent user
                    let guardian_email = format!("guardian_{}@temp.orbis", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("x"));
                    let g_user_req = CreateUserRequest {
                        email: guardian_email,
                        password: "Orbis@2026".to_string(),
                        full_name: trimmed_gname.to_string(),
                        name_ar: None,
                        primary_role: "parent".to_string(),
                        sex: None,
                        phone: row.guardian_phone.clone(),
                        national_id: None,
                        date_of_birth: None,
                    };
                    if let Ok(guardian_user) = self.create_user_in_org(org_id, g_user_req, created_by).await {
                        let link_req = crate::service::guardian::CreateGuardianLinkRequest {
                            student_id: user.id.clone(),
                            guardian_id: guardian_user.id.clone(),
                            relationship: row.guardian_relationship.clone().unwrap_or_else(|| "father".to_string()),
                            relationship_ar: None,
                            is_primary: Some(1),
                            is_emergency_contact: Some(1),
                            can_view_grades: Some(1),
                            can_view_attendance: Some(1),
                            can_view_behavior: Some(1),
                            can_view_finance: Some(0),
                            can_view_schedule: Some(1),
                            can_receive_notifications: Some(1),
                            is_payer: Some(1),
                            payment_share_percent: Some(100.0),
                            occupation: None,
                            employer: None,
                            work_phone: None,
                            notes: None,
                        };
                        let _ = self.create_guardian_link(link_req).await;
                    }
                }
            }

            created += 1;
        }

        Ok(BulkImportResult {
            total,
            created,
            failed,
            errors,
            active_cycle_name: None,
        })
    }

    /// Bulk create grades with smart student matching.
    /// Supports identification by: email, national_id, student_number, name, or auto-detect.
    pub async fn bulk_create_grades(
        &self,
        org_id: &str,
        graded_by: &str,
        cycle_id_opt: Option<String>,
        rows: Vec<BulkGradeRow>,
    ) -> Result<BulkImportResult, String> {
        let total = rows.len();
        let mut created = 0usize;
        let mut failed = 0usize;
        let mut errors: Vec<BulkRowError> = Vec::new();

        // Resolve cycle
        let cycle = if let Some(ref cid) = cycle_id_opt {
            self.repo.get_cycle_by_id(cid).await
                .map_err(|e| e.to_string())?
                .ok_or_else(|| "الدورة الأكاديمية غير موجودة".to_string())?
        } else {
            self.repo.get_current_cycle(org_id).await
                .map_err(|e| e.to_string())?
                .ok_or_else(|| "لا توجد دورة أكاديمية نشطة".to_string())?
        };

        let cycle_name = cycle.name.clone();

        // Check grade entry open
        self.check_cycle_open_for_grading(&cycle.id).await?;

        for (idx, row) in rows.iter().enumerate() {
            let row_index = idx + 1;

            let identifier = row.student_identifier.trim();
            if identifier.is_empty() {
                errors.push(BulkRowError {
                    row_index,
                    field: "student_identifier".to_string(),
                    message: "معرّف الطالب مطلوب".to_string(),
                });
                failed += 1;
                continue;
            }

            let subject = row.subject_name.trim();
            if subject.is_empty() {
                errors.push(BulkRowError {
                    row_index,
                    field: "subject_name".to_string(),
                    message: "اسم المادة مطلوب".to_string(),
                });
                failed += 1;
                continue;
            }

            let semester = row.semester.trim();
            if semester.is_empty() {
                errors.push(BulkRowError {
                    row_index,
                    field: "semester".to_string(),
                    message: "الفصل الدراسي مطلوب".to_string(),
                });
                failed += 1;
                continue;
            }

            let max_score = row.max_score.unwrap_or(100.0);
            if max_score <= 0.0 {
                errors.push(BulkRowError {
                    row_index,
                    field: "max_score".to_string(),
                    message: "الدرجة القصوى يجب أن تكون أكبر من صفر".to_string(),
                });
                failed += 1;
                continue;
            }

            if row.score < 0.0 || row.score > max_score {
                errors.push(BulkRowError {
                    row_index,
                    field: "score".to_string(),
                    message: format!("الدرجة يجب أن تكون بين 0 و {}", max_score),
                });
                failed += 1;
                continue;
            }

            // ── Smart student matching ──
            let id_type = row.identifier_type.as_deref().unwrap_or("auto");
            let student = match self.resolve_student(org_id, identifier, id_type).await {
                Ok(s) => s,
                Err(e) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "student_identifier".to_string(),
                        message: e,
                    });
                    failed += 1;
                    continue;
                }
            };

            // ── Find enrollment ──
            let enrollment = match self.repo.get_enrollment_by_student_id(&student.id, &cycle.id).await {
                Ok(Some(e)) => e,
                Ok(None) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "student_identifier".to_string(),
                        message: format!("لا يوجد تسجيل نشط للطالب '{}' في هذه الدورة", student.full_name),
                    });
                    failed += 1;
                    continue;
                }
                Err(e) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "student_identifier".to_string(),
                        message: format!("خطأ: {}", e),
                    });
                    failed += 1;
                    continue;
                }
            };

            // ── Create grade ──
            let percentage = (row.score / max_score) * 100.0;
            let grade_letter = Self::score_to_grade_letter(percentage);
            let grade_points = Self::score_to_grade_points(percentage);
            let exam_type = row.exam_type.as_deref().unwrap_or("final").to_string();

            let g = crate::entities::grade::Model {
                id: uuid::Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                cycle_id: cycle.id.clone(),
                enrollment_id: enrollment.id.clone(),
                assessment_id: None,
                class_subject_id: None,
                subject_name: subject.to_string(),
                subject_name_ar: None,
                score: row.score,
                max_score,
                weight: 1.0,
                grade_letter: Some(grade_letter),
                grade_points: Some(grade_points),
                exam_type,
                semester: semester.to_string(),
                notes: row.notes.clone(),
                graded_by: graded_by.to_string(),
                graded_at: chrono::Utc::now().to_rfc3339(),
                is_final: 1,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            };

            match self.repo.create_grade(g).await {
                Ok(_) => created += 1,
                Err(e) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "database".to_string(),
                        message: format!("خطأ في قاعدة البيانات: {}", e),
                    });
                    failed += 1;
                }
            }
        }

        Ok(BulkImportResult {
            total,
            created,
            failed,
            errors,
            active_cycle_name: Some(cycle_name),
        })
    }

    /// Smart student resolver: finds a student by various identifier types.
    /// Auto mode: detects type from format (@ → email, STU- → student_number, numeric → national_id, else → name).
    async fn resolve_student(
        &self,
        org_id: &str,
        identifier: &str,
        id_type: &str,
    ) -> Result<crate::entities::user::Model, String> {
        use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Condition};
        use crate::entities::{user, enrollment};

        match id_type {
            "email" => {
                self.repo.get_user_by_email(identifier).await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| format!("الطالب غير موجود (بريد: {})", identifier))
            }
            "national_id" => {
                user::Entity::find()
                    .filter(user::Column::OrgId.eq(org_id))
                    .filter(user::Column::NationalId.eq(identifier))
                    .filter(user::Column::IsDeleted.eq(0))
                    .one(&self.repo.db)
                    .await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| format!("الطالب غير موجود (رقم وطني: {})", identifier))
            }
            "student_number" => {
                let enrollment_opt = enrollment::Entity::find()
                    .filter(enrollment::Column::OrgId.eq(org_id))
                    .filter(enrollment::Column::StudentNumber.eq(identifier))
                    .one(&self.repo.db)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(enr) = enrollment_opt {
                    self.repo.get_user_by_id(&enr.student_id).await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| format!("الطالب غير موجود (رقم أكاديمي: {})", identifier))
                } else {
                    Err(format!("لا يوجد تسجيل بالرقم الأكاديمي: {}", identifier))
                }
            }
            "name" => {
                let students = user::Entity::find()
                    .filter(user::Column::OrgId.eq(org_id))
                    .filter(user::Column::IsDeleted.eq(0))
                    .filter(
                        Condition::any()
                            .add(user::Column::FullName.eq(identifier))
                            .add(user::Column::NameAr.eq(identifier))
                    )
                    .all(&self.repo.db)
                    .await
                    .map_err(|e| e.to_string())?;

                match students.len() {
                    0 => Err(format!("لا يوجد طالب بالاسم: {}", identifier)),
                    1 => Ok(students.into_iter().next().unwrap()),
                    n => Err(format!(
                        "تطابق متعدد: وُجد {} طالب بالاسم '{}'. استخدم البريد أو الرقم الوطني بدلاً من ذلك.",
                        n, identifier
                    )),
                }
            }
            _ => {
                // Auto-detect: inline the logic to avoid async recursion
                if identifier.contains('@') {
                    // Email lookup
                    self.repo.get_user_by_email(identifier).await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| format!("الطالب غير موجود (بريد: {})", identifier))
                } else if identifier.starts_with("STU-") || identifier.starts_with("stu-") {
                    // Student number lookup
                    let enrollment_opt = enrollment::Entity::find()
                        .filter(enrollment::Column::OrgId.eq(org_id))
                        .filter(enrollment::Column::StudentNumber.eq(identifier))
                        .one(&self.repo.db)
                        .await
                        .map_err(|e| e.to_string())?;
                    if let Some(enr) = enrollment_opt {
                        self.repo.get_user_by_id(&enr.student_id).await
                            .map_err(|e| e.to_string())?
                            .ok_or_else(|| format!("الطالب غير موجود (رقم أكاديمي: {})", identifier))
                    } else {
                        Err(format!("لا يوجد تسجيل بالرقم الأكاديمي: {}", identifier))
                    }
                } else if identifier.chars().all(|c| c.is_ascii_digit()) && identifier.len() >= 5 {
                    // National ID lookup
                    user::Entity::find()
                        .filter(user::Column::OrgId.eq(org_id))
                        .filter(user::Column::NationalId.eq(identifier))
                        .filter(user::Column::IsDeleted.eq(0))
                        .one(&self.repo.db)
                        .await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| format!("الطالب غير موجود (رقم وطني: {})", identifier))
                } else {
                    // Try email first, then name search
                    if let Ok(Some(u)) = self.repo.get_user_by_email(identifier).await {
                        Ok(u)
                    } else {
                        // Name search
                        let students = user::Entity::find()
                            .filter(user::Column::OrgId.eq(org_id))
                            .filter(user::Column::IsDeleted.eq(0))
                            .filter(
                                Condition::any()
                                    .add(user::Column::FullName.eq(identifier))
                                    .add(user::Column::NameAr.eq(identifier))
                            )
                            .all(&self.repo.db)
                            .await
                            .map_err(|e| e.to_string())?;
                        match students.len() {
                            0 => Err(format!("لا يوجد طالب بالمعرّف: {}", identifier)),
                            1 => Ok(students.into_iter().next().unwrap()),
                            n => Err(format!(
                                "تطابق متعدد: وُجد {} طالب بالاسم '{}'. استخدم البريد أو الرقم الوطني.",
                                n, identifier
                            )),
                        }
                    }
                }
            }
        }
    }
}
