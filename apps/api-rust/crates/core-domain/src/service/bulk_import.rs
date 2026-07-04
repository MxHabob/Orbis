use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entities::{user, grade};

// ── Request / Response DTOs ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkStudentRow {
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub national_id: Option<String>,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub primary_role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct BulkGradeRow {
    pub student_email: String,
    pub subject_name: String,
    pub score: f64,
    pub max_score: Option<f64>,
    pub exam_type: Option<String>,
    pub semester: String,
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
}

// ── Service Implementation ──────────────────────────────────────────────────

impl super::Service {
    /// Bulk create students/teachers from JSON rows.
    /// Validates each row, checks duplicates, and creates users.
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

        let valid_roles = ["admin", "super_admin", "principal", "teacher", "student", "parent", "staff"];
        let valid_sex = ["male", "female"];

        // Pre-collect emails and national_ids for duplicate detection within the batch
        let mut seen_emails: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut seen_nids: std::collections::HashSet<String> = std::collections::HashSet::new();

        for (idx, row) in rows.iter().enumerate() {
            let row_index = idx + 1; // 1-indexed for user display

            // ── Required field validation ──
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
            if trimmed_email.is_empty() {
                errors.push(BulkRowError {
                    row_index,
                    field: "email".to_string(),
                    message: "البريد الإلكتروني مطلوب".to_string(),
                });
                failed += 1;
                continue;
            }

            // Email format validation
            if !trimmed_email.contains('@') || !trimmed_email.contains('.') {
                errors.push(BulkRowError {
                    row_index,
                    field: "email".to_string(),
                    message: format!("بريد إلكتروني غير صحيح: '{}'", trimmed_email),
                });
                failed += 1;
                continue;
            }

            // ── Batch duplicate detection (email) ──
            if !seen_emails.insert(trimmed_email.clone()) {
                errors.push(BulkRowError {
                    row_index,
                    field: "email".to_string(),
                    message: format!("البريد '{}' مكرر في هذه الدفعة", trimmed_email),
                });
                failed += 1;
                continue;
            }

            // ── DB duplicate detection (email) ──
            if let Ok(Some(_)) = self.repo.get_user_by_email(&trimmed_email).await {
                errors.push(BulkRowError {
                    row_index,
                    field: "email".to_string(),
                    message: format!("البريد '{}' مسجل مسبقاً في النظام", trimmed_email),
                });
                failed += 1;
                continue;
            }

            // ── Optional field validation ──
            let role = row.primary_role.as_deref().unwrap_or(default_role).to_lowercase();
            if !valid_roles.contains(&role.as_str()) {
                errors.push(BulkRowError {
                    row_index,
                    field: "primary_role".to_string(),
                    message: format!("دور غير صحيح: '{}'. الأدوار المتاحة: {:?}", role, valid_roles),
                });
                failed += 1;
                continue;
            }

            if let Some(ref sex) = row.sex {
                let trimmed_sex = sex.trim().to_lowercase();
                if !trimmed_sex.is_empty() && !valid_sex.contains(&trimmed_sex.as_str()) {
                    errors.push(BulkRowError {
                        row_index,
                        field: "sex".to_string(),
                        message: format!("الجنس يجب أن يكون 'male' أو 'female', وجد: '{}'", trimmed_sex),
                    });
                    failed += 1;
                    continue;
                }
            }

            // National ID batch duplicate check
            if let Some(ref nid) = row.national_id {
                let trimmed_nid = nid.trim().to_string();
                if !trimmed_nid.is_empty() {
                    if !seen_nids.insert(trimmed_nid.clone()) {
                        errors.push(BulkRowError {
                            row_index,
                            field: "national_id".to_string(),
                            message: format!("الرقم الوطني '{}' مكرر في هذه الدفعة", trimmed_nid),
                        });
                        failed += 1;
                        continue;
                    }
                }
            }

            // ── Create user ──
            let pwd_hash = match bcrypt::hash("Orbis@2026", bcrypt::DEFAULT_COST) {
                Ok(h) => h,
                Err(e) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "password".to_string(),
                        message: format!("خطأ في تشفير كلمة المرور: {}", e),
                    });
                    failed += 1;
                    continue;
                }
            };

            let new_user = user::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                email: Some(trimmed_email),
                email_verified: 0,
                email_verified_at: None,
                phone: row.phone.as_ref().map(|p| p.trim().to_string()).filter(|p| !p.is_empty()),
                phone_verified: 0,
                phone_verified_at: None,
                password_hash: Some(pwd_hash),
                password_changed_at: None,
                password_expires_at: None,
                mfa_enabled: 0,
                mfa_secret: None,
                mfa_backup_codes: None,
                full_name: trimmed_name.to_string(),
                name_ar: None,
                name_en: None,
                avatar: None,
                sex: row.sex.as_ref().map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()),
                date_of_birth: row.date_of_birth.as_ref().map(|d| d.trim().to_string()).filter(|d| !d.is_empty()),
                place_of_birth: None,
                nationality: None,
                national_id: row.national_id.as_ref().map(|n| n.trim().to_string()).filter(|n| !n.is_empty()),
                passport_number: None,
                address: None,
                city: None,
                country: None,
                postal_code: None,
                emergency_contact: None,
                blood_type: None,
                allergies: None,
                medical_conditions: None,
                medical_notes: None,
                social_links: None,
                bio: None,
                primary_role: role,
                status: "active".to_string(),
                last_login_at: None,
                last_login_ip: None,
                login_count: Some(0),
                failed_logins: Some(0),
                locked_until: None,
                prefs: "{}".to_string(),
                meta: "{}".to_string(),
                is_deleted: 0,
                deleted_at: None,
                deleted_by: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
                created_by: Some(created_by.to_string()),
            };

            match self.repo.create_user(new_user).await {
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
        })
    }

    /// Bulk create grades from JSON rows.
    /// Resolves student by email, finds enrollment, and creates grade records.
    pub async fn bulk_create_grades(
        &self,
        org_id: &str,
        graded_by: &str,
        rows: Vec<BulkGradeRow>,
    ) -> Result<BulkImportResult, String> {
        let total = rows.len();
        let mut created = 0usize;
        let mut failed = 0usize;
        let mut errors: Vec<BulkRowError> = Vec::new();

        // Get current cycle
        let cycle = self.repo.get_current_cycle(org_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "لا توجد دورة أكاديمية نشطة".to_string())?;

        // Check if grade entry is open
        self.check_cycle_open_for_grading(&cycle.id).await?;

        for (idx, row) in rows.iter().enumerate() {
            let row_index = idx + 1;

            // ── Required field validation ──
            let trimmed_email = row.student_email.trim().to_lowercase();
            if trimmed_email.is_empty() {
                errors.push(BulkRowError {
                    row_index,
                    field: "student_email".to_string(),
                    message: "بريد الطالب مطلوب".to_string(),
                });
                failed += 1;
                continue;
            }

            let trimmed_subject = row.subject_name.trim();
            if trimmed_subject.is_empty() {
                errors.push(BulkRowError {
                    row_index,
                    field: "subject_name".to_string(),
                    message: "اسم المادة مطلوب".to_string(),
                });
                failed += 1;
                continue;
            }

            let trimmed_semester = row.semester.trim();
            if trimmed_semester.is_empty() {
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

            // ── Find student by email ──
            let student = match self.repo.get_user_by_email(&trimmed_email).await {
                Ok(Some(s)) => s,
                Ok(None) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "student_email".to_string(),
                        message: format!("الطالب '{}' غير موجود", trimmed_email),
                    });
                    failed += 1;
                    continue;
                }
                Err(e) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "student_email".to_string(),
                        message: format!("خطأ في البحث: {}", e),
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
                        field: "student_email".to_string(),
                        message: format!("لا يوجد تسجيل نشط للطالب '{}'", trimmed_email),
                    });
                    failed += 1;
                    continue;
                }
                Err(e) => {
                    errors.push(BulkRowError {
                        row_index,
                        field: "student_email".to_string(),
                        message: format!("خطأ في البحث: {}", e),
                    });
                    failed += 1;
                    continue;
                }
            };

            // ── Calculate grade letter and points ──
            let percentage = (row.score / max_score) * 100.0;
            let grade_letter = Self::score_to_grade_letter(percentage);
            let grade_points = Self::score_to_grade_points(percentage);
            let exam_type = row.exam_type.as_deref().unwrap_or("final").to_string();

            let g = grade::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                cycle_id: cycle.id.clone(),
                enrollment_id: enrollment.id.clone(),
                assessment_id: None,
                class_subject_id: None,
                subject_name: trimmed_subject.to_string(),
                subject_name_ar: None,
                score: row.score,
                max_score,
                weight: 1.0,
                grade_letter: Some(grade_letter),
                grade_points: Some(grade_points),
                exam_type,
                semester: trimmed_semester.to_string(),
                notes: None,
                graded_by: graded_by.to_string(),
                graded_at: Utc::now().to_rfc3339(),
                is_final: 1,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
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
        })
    }
}
