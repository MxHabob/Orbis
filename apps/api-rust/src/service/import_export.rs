use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entities::{user, grade, attendance, enrollment};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ImportResult {
    pub total_rows: usize,
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

impl super::Service {
    /// Import students from CSV text
    /// Expected columns: full_name, email, national_id (optional), class_name, role (optional)
    pub async fn import_students_csv(&self, org_id: &str, csv_text: &str, created_by: &str) -> Result<ImportResult, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(csv_text.as_bytes());

        let headers = reader.headers().map_err(|e| format!("CSV header error: {}", e))?.clone();

        // Find column indices
        let find_col = |names: &[&str]| -> Option<usize> {
            for (i, h) in headers.iter().enumerate() {
                let lower = h.trim().to_lowercase();
                for name in names {
                    if lower == *name { return Some(i); }
                }
            }
            None
        };

        let name_col = find_col(&["full_name", "name", "الاسم الكامل", "الاسم"]);
        let email_col = find_col(&["email", "البريد الإلكتروني", "بريد"]);
        let nid_col = find_col(&["national_id", "الرقم الوطني", "هوية"]);
        let class_col = find_col(&["class_name", "class", "اسم الصف", "الصف"]);
        let role_col = find_col(&["role", "الدور"]);

        if name_col.is_none() || email_col.is_none() {
            return Err("CSV must contain 'Full Name' and 'Email' columns".to_string());
        }

        let name_idx = name_col.unwrap();
        let email_idx = email_col.unwrap();

        let mut result = ImportResult { total_rows: 0, successful: 0, failed: 0, errors: Vec::new() };

        for (row_num, record) in reader.records().enumerate() {
            result.total_rows += 1;
            let row = row_num + 2; // 1-indexed + header

            let record = match record {
                Ok(r) => r,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: parse error: {}", row, e));
                    continue;
                }
            };

            let full_name = record.get(name_idx).unwrap_or("").trim();
            let email = record.get(email_idx).unwrap_or("").trim();

            if full_name.is_empty() || email.is_empty() {
                result.failed += 1;
                result.errors.push(format!("Row {}: full_name and email are required", row));
                continue;
            }

            // Validate email format
            if !email.contains('@') {
                result.failed += 1;
                result.errors.push(format!("Row {}: invalid email '{}'", row, email));
                continue;
            }

            // Check duplicate
            let existing = self.repo.get_user_by_email(email).await.map_err(|e| e.to_string())?;
            if existing.is_some() {
                result.failed += 1;
                result.errors.push(format!("Row {}: email '{}' already exists", row, email));
                continue;
            }

            let national_id = nid_col.and_then(|i| record.get(i)).map(|s| s.trim().to_string());
            let role = role_col
                .and_then(|i| record.get(i))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "student".to_string());

            let pwd_hash = bcrypt::hash("Orbis@2026", bcrypt::DEFAULT_COST)
                .map_err(|e| e.to_string())?;

            let new_user = user::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                email: Some(email.to_string()),
                email_verified: 0,
                email_verified_at: None,
                phone: None, phone_verified: 0, phone_verified_at: None,
                password_hash: Some(pwd_hash),
                password_changed_at: None, password_expires_at: None,
                mfa_enabled: 0, mfa_secret: None, mfa_backup_codes: None,
                full_name: full_name.to_string(),
                name_ar: None, name_en: None, avatar: None, sex: None,
                date_of_birth: None, place_of_birth: None, nationality: None,
                national_id,
                passport_number: None, address: None, city: None, country: None,
                postal_code: None, emergency_contact: None, blood_type: None,
                allergies: None, medical_conditions: None, medical_notes: None,
                social_links: None, bio: None,
                primary_role: role,
                status: "active".to_string(),
                last_login_at: None, last_login_ip: None,
                login_count: Some(0), failed_logins: Some(0), locked_until: None,
                prefs: "{}".to_string(), meta: "{}".to_string(),
                is_deleted: 0, deleted_at: None, deleted_by: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
                created_by: Some(created_by.to_string()),
            };

            match self.repo.create_user(new_user).await {
                Ok(_) => result.successful += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: database error: {}", row, e));
                }
            }
        }

        Ok(result)
    }

    /// Import grades from CSV text
    /// Expected columns: student_email, subject, score, max_score (optional), term
    pub async fn import_grades_csv(&self, org_id: &str, csv_text: &str, graded_by: &str) -> Result<ImportResult, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(csv_text.as_bytes());

        let headers = reader.headers().map_err(|e| format!("CSV header error: {}", e))?.clone();

        let find_col = |names: &[&str]| -> Option<usize> {
            for (i, h) in headers.iter().enumerate() {
                let lower = h.trim().to_lowercase();
                for name in names {
                    if lower == *name { return Some(i); }
                }
            }
            None
        };

        let email_col = find_col(&["student_email", "email", "بريد الطالب"]).ok_or("Missing 'Student Email' column")?;
        let subject_col = find_col(&["subject", "المادة"]).ok_or("Missing 'Subject' column")?;
        let score_col = find_col(&["score", "grade", "الدرجة"]).ok_or("Missing 'Score' column")?;
        let max_col = find_col(&["max_score", "max", "الدرجة القصوى"]);
        let term_col = find_col(&["term", "semester", "الفصل الدراسي"]).ok_or("Missing 'Term' column")?;

        let mut result = ImportResult { total_rows: 0, successful: 0, failed: 0, errors: Vec::new() };

        // Get current cycle
        let cycle = self.repo.get_current_cycle(org_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "no active academic cycle".to_string())?;

        for (row_num, record) in reader.records().enumerate() {
            result.total_rows += 1;
            let row = row_num + 2;

            let record = match record {
                Ok(r) => r,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: parse error: {}", row, e));
                    continue;
                }
            };

            let email = record.get(email_col).unwrap_or("").trim();
            let subject = record.get(subject_col).unwrap_or("").trim();
            let score_str = record.get(score_col).unwrap_or("").trim();
            let max_score_str = max_col.and_then(|i| record.get(i)).unwrap_or("100").trim();
            let term = record.get(term_col).unwrap_or("").trim();

            if email.is_empty() || subject.is_empty() || score_str.is_empty() || term.is_empty() {
                result.failed += 1;
                result.errors.push(format!("Row {}: all fields are required", row));
                continue;
            }

            let score: f64 = match score_str.parse() {
                Ok(s) => s,
                Err(_) => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: invalid score '{}'", row, score_str));
                    continue;
                }
            };

            let max_score: f64 = max_score_str.parse().unwrap_or(100.0);

            // Find student
            let student = match self.repo.get_user_by_email(email).await.map_err(|e| e.to_string())? {
                Some(s) => s,
                None => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: student '{}' not found", row, email));
                    continue;
                }
            };

            // Find enrollment
            let enrollment_opt = self.repo.get_enrollment_by_student_id(&student.id, &cycle.id).await
                .map_err(|e| e.to_string())?;
            let enroll = match enrollment_opt {
                Some(e) => e,
                None => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: no active enrollment for '{}'", row, email));
                    continue;
                }
            };

            let percentage = (score / max_score) * 100.0;
            let grade_letter = Self::score_to_grade_letter(percentage);
            let grade_points = Self::score_to_grade_points(percentage);

            let g = grade::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                cycle_id: cycle.id.clone(),
                enrollment_id: enroll.id.clone(),
                subject_name: subject.to_string(),
                subject_name_ar: None,
                score,
                max_score,
                weight: 1.0,
                grade_letter: Some(grade_letter),
                grade_points: Some(grade_points),
                exam_type: "final".to_string(),
                semester: term.to_string(),
                notes: None,
                graded_by: graded_by.to_string(),
                graded_at: Utc::now().to_rfc3339(),
                is_final: 1,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            };

            match self.repo.create_grade(g).await {
                Ok(_) => result.successful += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: database error: {}", row, e));
                }
            }
        }

        Ok(result)
    }

    /// Import attendance from CSV
    /// Expected columns: student_email, date, status, period (optional)
    pub async fn import_attendance_csv(&self, org_id: &str, csv_text: &str, recorded_by: &str) -> Result<ImportResult, String> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(csv_text.as_bytes());

        let headers = reader.headers().map_err(|e| format!("CSV header error: {}", e))?.clone();

        let find_col = |names: &[&str]| -> Option<usize> {
            for (i, h) in headers.iter().enumerate() {
                let lower = h.trim().to_lowercase();
                for name in names {
                    if lower == *name { return Some(i); }
                }
            }
            None
        };

        let email_col = find_col(&["student_email", "email", "بريد الطالب"]).ok_or("Missing 'Student Email' column")?;
        let date_col = find_col(&["date", "التاريخ"]).ok_or("Missing 'Date' column")?;
        let status_col = find_col(&["status", "الحالة"]).ok_or("Missing 'Status' column")?;
        let period_col = find_col(&["period", "الحصة"]);

        let mut result = ImportResult { total_rows: 0, successful: 0, failed: 0, errors: Vec::new() };

        let cycle = self.repo.get_current_cycle(org_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "no active academic cycle".to_string())?;

        for (row_num, record) in reader.records().enumerate() {
            result.total_rows += 1;
            let row = row_num + 2;

            let record = match record {
                Ok(r) => r,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: parse error: {}", row, e));
                    continue;
                }
            };

            let email = record.get(email_col).unwrap_or("").trim();
            let date = record.get(date_col).unwrap_or("").trim();
            let status = record.get(status_col).unwrap_or("").trim().to_lowercase();
            let period = period_col.and_then(|i| record.get(i)).map(|s| s.trim().to_string());

            if email.is_empty() || date.is_empty() || status.is_empty() {
                result.failed += 1;
                result.errors.push(format!("Row {}: email, date, and status are required", row));
                continue;
            }

            let valid_statuses = ["present", "absent", "late", "excused", "sick"];
            if !valid_statuses.contains(&status.as_str()) {
                result.failed += 1;
                result.errors.push(format!("Row {}: invalid status '{}'. Valid: {:?}", row, status, valid_statuses));
                continue;
            }

            let student = match self.repo.get_user_by_email(email).await.map_err(|e| e.to_string())? {
                Some(s) => s,
                None => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: student '{}' not found", row, email));
                    continue;
                }
            };

            let enroll = match self.repo.get_enrollment_by_student_id(&student.id, &cycle.id).await
                .map_err(|e| e.to_string())? {
                Some(e) => e,
                None => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: no active enrollment for '{}'", row, email));
                    continue;
                }
            };

            let att = crate::entities::attendance::Model {
                id: Uuid::new_v4().to_string(),
                org_id: org_id.to_string(),
                cycle_id: cycle.id.clone(),
                enrollment_id: enroll.id.clone(),
                class_id: enroll.class_id.clone(),
                date: date.to_string(),
                status,
                check_in_time: None,
                check_out_time: None,
                period,
                subject_name: None,
                recorded_by: recorded_by.to_string(),
                notes: None,
                excuse_type: None,
                excuse_document: None,
                is_excused: 0,
                created_at: Utc::now().to_rfc3339(),
            };

            match self.repo.create_attendance(att).await {
                Ok(_) => result.successful += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Row {}: database error: {}", row, e));
                }
            }
        }

        Ok(result)
    }

    /// Export students as CSV string
    pub async fn export_students_csv(&self, org_id: &str, lang: &str) -> Result<String, String> {
        let users = self.repo.get_users(org_id, crate::handler::params::UserParams {
            q: None, role: Some("student".to_string()), status: None, limit: None, offset: None,
        }).await.map_err(|e| e.to_string())?;

        let mut wtr = csv::Writer::from_writer(vec![]);

        if lang == "ar" {
            wtr.write_record(&["الاسم الكامل", "البريد الإلكتروني", "الرقم الوطني", "الجنسية", "تاريخ الميلاد", "الجنس", "الحالة"])
                .map_err(|e| e.to_string())?;
        } else {
            wtr.write_record(&["Full Name", "Email", "National ID", "Nationality", "Date of Birth", "Sex", "Status"])
                .map_err(|e| e.to_string())?;
        }

        for u in &users {
            wtr.write_record(&[
                &u.full_name,
                u.email.as_deref().unwrap_or(""),
                u.national_id.as_deref().unwrap_or(""),
                u.nationality.as_deref().unwrap_or(""),
                u.date_of_birth.as_deref().unwrap_or(""),
                u.sex.as_deref().unwrap_or(""),
                &u.status,
            ]).map_err(|e| e.to_string())?;
        }

        let data = wtr.into_inner().map_err(|e| e.to_string())?;
        String::from_utf8(data).map_err(|e| e.to_string())
    }

    /// Export grades as CSV string
    pub async fn export_grades_csv(&self, org_id: &str, params: crate::handler::params::GradeParams, lang: &str) -> Result<String, String> {
        let grades = self.repo.get_grades(org_id, params).await.map_err(|e| e.to_string())?;

        let mut wtr = csv::Writer::from_writer(vec![]);

        if lang == "ar" {
            wtr.write_record(&["رقم التسجيل", "المادة", "الدرجة", "الدرجة القصوى", "النسبة", "التقدير", "نقاط التقدير", "نوع الامتحان", "الفصل"])
                .map_err(|e| e.to_string())?;
        } else {
            wtr.write_record(&["Enrollment ID", "Subject", "Score", "Max Score", "Percentage", "Grade", "GPA Points", "Exam Type", "Semester"])
                .map_err(|e| e.to_string())?;
        }

        for g in &grades {
            let pct = (g.score / g.max_score) * 100.0;
            wtr.write_record(&[
                &g.enrollment_id,
                &g.subject_name,
                &format!("{:.1}", g.score),
                &format!("{:.1}", g.max_score),
                &format!("{:.1}%", pct),
                g.grade_letter.as_deref().unwrap_or(""),
                &g.grade_points.map(|p| format!("{:.2}", p)).unwrap_or_default(),
                &g.exam_type,
                &g.semester,
            ]).map_err(|e| e.to_string())?;
        }

        let data = wtr.into_inner().map_err(|e| e.to_string())?;
        String::from_utf8(data).map_err(|e| e.to_string())
    }

    /// Export attendance as CSV string
    pub async fn export_attendance_csv(&self, org_id: &str, params: crate::handler::params::AttendanceParams, lang: &str) -> Result<String, String> {
        let records = self.repo.get_attendance(org_id, params).await.map_err(|e| e.to_string())?;

        let mut wtr = csv::Writer::from_writer(vec![]);

        if lang == "ar" {
            wtr.write_record(&["رقم التسجيل", "الصف", "التاريخ", "الحالة", "الحصة", "المادة", "ملاحظات"])
                .map_err(|e| e.to_string())?;
        } else {
            wtr.write_record(&["Enrollment ID", "Class ID", "Date", "Status", "Period", "Subject", "Notes"])
                .map_err(|e| e.to_string())?;
        }

        for r in &records {
            wtr.write_record(&[
                &r.enrollment_id,
                &r.class_id,
                &r.date,
                &r.status,
                r.period.as_deref().unwrap_or(""),
                r.subject_name.as_deref().unwrap_or(""),
                r.notes.as_deref().unwrap_or(""),
            ]).map_err(|e| e.to_string())?;
        }

        let data = wtr.into_inner().map_err(|e| e.to_string())?;
        String::from_utf8(data).map_err(|e| e.to_string())
    }

    /// Export class report cards as CSV
    pub async fn export_report_cards_csv(&self, org_id: &str, class_id: &str, lang: &str) -> Result<String, String> {
        let enrollments = self.repo.get_enrollments_by_class(class_id).await
            .map_err(|e| e.to_string())?;

        let mut wtr = csv::Writer::from_writer(vec![]);

        if lang == "ar" {
            wtr.write_record(&["اسم الطالب", "النسبة العامة", "المعدل التراكمي", "التقدير العام", "المواد الناجحة", "المواد الراسبة"])
                .map_err(|e| e.to_string())?;
        } else {
            wtr.write_record(&["Student Name", "Overall %", "GPA", "Grade", "Passed", "Failed"])
                .map_err(|e| e.to_string())?;
        }

        for enroll in &enrollments {
            match self.generate_report_card(&enroll.id).await {
                Ok(rc) => {
                    wtr.write_record(&[
                        &rc.student_name,
                        &format!("{:.1}%", rc.overall_percentage),
                        &format!("{:.2}", rc.overall_gpa),
                        &rc.overall_letter,
                        &rc.passed_subjects.to_string(),
                        &rc.failed_subjects.to_string(),
                    ]).map_err(|e| e.to_string())?;
                }
                Err(_) => continue,
            }
        }

        let data = wtr.into_inner().map_err(|e| e.to_string())?;
        String::from_utf8(data).map_err(|e| e.to_string())
    }
}
