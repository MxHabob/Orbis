// ═══════════════════════════════════════════════════════════════
// docforge-orbis — Data Sources
// Fetches database records for specific entity types and converts them
// to dynamic JSON data contexts for template interpolation.
// ═══════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{json, Value};
use sea_orm::{DatabaseConnection, ConnectionTrait};

// ── Safe Query Helper ─────────────────────────────────────────

/// Build a parameterized SELECT * FROM table WHERE id = ? statement.
fn select_by_id(db: &DatabaseConnection, table: &str, id: &str) -> sea_orm::Statement {
    sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(),
        &format!("SELECT * FROM {} WHERE id = $1", table),
        vec![id.into()],
    )
}

// ── DataSource Trait ──────────────────────────────────────────

#[async_trait]
pub trait DataSource: Send + Sync {
    /// Return the entity type supported by this data source (e.g. "enrollment", "payment", "exam", "user")
    fn entity_type(&self) -> &str;
    
    /// Resolve variables for a given entity ID, returning a JSON context
    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String>;
    
    /// Return the JSON schema representing available variable paths and types for this data source
    fn schema(&self) -> Value;
}

// ── Enrollment DataSource ─────────────────────────────────────

pub struct EnrollmentDataSource;

#[async_trait]
impl DataSource for EnrollmentDataSource {
    fn entity_type(&self) -> &str {
        "enrollment"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        // 1. Fetch enrollment
        let enroll_stmt = select_by_id(db, "enrollments", entity_id);
        let enroll_opt = db.query_one(enroll_stmt).await.map_err(|e| e.to_string())?;
        let enroll = enroll_opt.ok_or_else(|| format!("Enrollment not found: {}", entity_id))?;
        
        let student_id: String = enroll.try_get("", "student_id").map_err(|e| e.to_string())?;
        let class_id: String = enroll.try_get("", "class_id").map_err(|e| e.to_string())?;
        
        // 2. Fetch student user
        let user_stmt = select_by_id(db, "users", &student_id);
        let user_opt = db.query_one(user_stmt).await.map_err(|e| e.to_string())?;
        
        // 3. Fetch class
        let class_stmt = select_by_id(db, "classes", &class_id);
        let class_opt = db.query_one(class_stmt).await.map_err(|e| e.to_string())?;

        // Construct JSON context
        let mut student_json = json!({});
        if let Some(user) = user_opt {
            student_json = json!({
                "id": student_id,
                "fullName": user.try_get::<String>("", "full_name").unwrap_or_default(),
                "nameAr": user.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
                "nameEn": user.try_get::<Option<String>>("", "name_en").unwrap_or_default(),
                "email": user.try_get::<Option<String>>("", "email").unwrap_or_default(),
                "phone": user.try_get::<Option<String>>("", "phone").unwrap_or_default(),
                "nationalId": user.try_get::<Option<String>>("", "national_id").unwrap_or_default(),
                "dateOfBirth": user.try_get::<Option<String>>("", "date_of_birth").unwrap_or_default(),
                "nationality": user.try_get::<Option<String>>("", "nationality").unwrap_or_default(),
            });
        }

        let mut class_json = json!({});
        if let Some(class) = class_opt {
            class_json = json!({
                "id": class_id,
                "name": class.try_get::<String>("", "name").unwrap_or_default(),
                "nameAr": class.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
                "level": class.try_get::<String>("", "level").unwrap_or_default(),
                "division": class.try_get::<Option<String>>("", "division").unwrap_or_default(),
            });
        }

        // 4. Fetch latest assessment snapshot for dynamic grade data
        //    (replaces hardcoded enrollment.final_grade / final_score / final_rank)
        let snapshot_stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT grade_label, total_score, gpa_points, rank_in_class, is_passing FROM student_assessment_snapshots WHERE student_id = $1 ORDER BY updated_at DESC LIMIT 1",
            vec![student_id.clone().into()],
        );
        let snap_opt = db.query_one(snapshot_stmt).await.ok().flatten();

        let (final_grade, final_score, final_rank, final_gpa) = if let Some(snap) = snap_opt {
            (
                snap.try_get::<Option<String>>("", "grade_label").unwrap_or_default(),
                snap.try_get::<Option<f64>>("", "total_score").unwrap_or_default(),
                snap.try_get::<Option<i32>>("", "rank_in_class").unwrap_or_default(),
                snap.try_get::<Option<f64>>("", "gpa_points").unwrap_or_default(),
            )
        } else {
            // Fallback to enrollment static fields if no snapshot exists yet
            (
                enroll.try_get::<Option<String>>("", "final_grade").unwrap_or_default(),
                enroll.try_get::<Option<f64>>("", "final_score").ok().flatten(),
                enroll.try_get::<Option<i32>>("", "final_rank").unwrap_or_default(),
                None,
            )
        };

        let ctx = json!({
            "id": entity_id,
            "studentNumber": enroll.try_get::<String>("", "student_number").unwrap_or_default(),
            "status": enroll.try_get::<String>("", "status").unwrap_or_default(),
            "enrollmentDate": enroll.try_get::<String>("", "enrollment_date").unwrap_or_default(),
            "finalGrade": final_grade,
            "finalScore": final_score,
            "finalRank": final_rank,
            "finalGpa": final_gpa,
            "student": student_json,
            "class": class_json,
        });

        Ok(ctx)
    }

    fn schema(&self) -> Value {
        json!({
            "studentNumber": { "type": "string", "label": "رقم الطالب / Student Number" },
            "status": { "type": "string", "label": "الحالة / Status" },
            "enrollmentDate": { "type": "string", "label": "تاريخ القبول / Enrollment Date" },
            "finalGrade": { "type": "string", "label": "التقدير النهائي / Final Grade" },
            "finalScore": { "type": "number", "label": "الدرجة النهائية / Final Score" },
            "finalRank": { "type": "number", "label": "الترتيب النهائي / Final Rank" },
            "finalGpa": { "type": "number", "label": "المعدل التراكمي / Final GPA" },
            "student": {
                "type": "object",
                "properties": {
                    "fullName": { "type": "string", "label": "الاسم الكامل / Full Name" },
                    "nameAr": { "type": "string", "label": "الاسم بالعربية / Name (AR)" },
                    "nameEn": { "type": "string", "label": "الاسم بالإنجليزية / Name (EN)" },
                    "email": { "type": "string", "label": "البريد الإلكتروني / Email" },
                    "phone": { "type": "string", "label": "الهاتف / Phone" },
                    "nationalId": { "type": "string", "label": "رقم الهوية / National ID" },
                    "nationality": { "type": "string", "label": "الجنسية / Nationality" },
                }
            },
            "class": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "label": "اسم الفصل / Class Name" },
                    "nameAr": { "type": "string", "label": "اسم الفصل بالعربية / Class Name (AR)" },
                    "level": { "type": "string", "label": "المستوى / Level" },
                    "division": { "type": "string", "label": "الشعبة / Division" }
                }
            }
        })
    }
}

// ── Payment DataSource ────────────────────────────────────────

pub struct PaymentDataSource;

#[async_trait]
impl DataSource for PaymentDataSource {
    fn entity_type(&self) -> &str {
        "payment"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        let stmt = select_by_id(db, "payments", entity_id);
        let opt = db.query_one(stmt).await.map_err(|e| e.to_string())?;
        let payment = opt.ok_or_else(|| format!("Payment not found: {}", entity_id))?;

        let ctx = json!({
            "id": entity_id,
            "amount": payment.try_get::<f64>("", "amount").unwrap_or(0.0),
            "currency": payment.try_get::<String>("", "currency").unwrap_or_default(),
            "method": payment.try_get::<String>("", "method").unwrap_or_default(),
            "receiptNumber": payment.try_get::<Option<String>>("", "receipt_number").unwrap_or_default(),
            "payerName": payment.try_get::<Option<String>>("", "payer_name").unwrap_or_default(),
            "payerPhone": payment.try_get::<Option<String>>("", "payer_phone").unwrap_or_default(),
            "paymentType": payment.try_get::<String>("", "payment_type").unwrap_or_default(),
            "notes": payment.try_get::<Option<String>>("", "notes").unwrap_or_default(),
            "paidAt": payment.try_get::<String>("", "paid_at").unwrap_or_default(),
        });

        Ok(ctx)
    }

    fn schema(&self) -> Value {
        json!({
            "amount": { "type": "number", "label": "القيمة / Amount" },
            "currency": { "type": "string", "label": "العملة / Currency" },
            "method": { "type": "string", "label": "طريقة الدفع / Payment Method" },
            "receiptNumber": { "type": "string", "label": "رقم السند / Receipt Number" },
            "payerName": { "type": "string", "label": "اسم دافع السند / Payer Name" },
            "payerPhone": { "type": "string", "label": "هاتف دافع السند / Payer Phone" },
            "paymentType": { "type": "string", "label": "نوع السند / Payment Type" },
            "notes": { "type": "string", "label": "ملاحظات / Notes" },
            "paidAt": { "type": "string", "label": "تاريخ السند / Paid Date" }
        })
    }
}

// ── Exam DataSource ───────────────────────────────────────────

pub struct ExamDataSource;

#[async_trait]
impl DataSource for ExamDataSource {
    fn entity_type(&self) -> &str {
        "exam"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        // Fetch seating number record
        let seat_stmt = select_by_id(db, "seating_numbers", entity_id);
        let seat_opt = db.query_one(seat_stmt).await.map_err(|e| e.to_string())?;
        let seat = seat_opt.ok_or_else(|| format!("Seating allocation record not found: {}", entity_id))?;

        let session_id: String = seat.try_get("", "exam_session_id").map_err(|e| e.to_string())?;
        
        // Fetch exam session details
        let session_stmt = select_by_id(db, "exam_sessions", &session_id);
        let session_opt = db.query_one(session_stmt).await.map_err(|e| e.to_string())?;

        let mut session_json = json!({});
        if let Some(sess) = session_opt {
            session_json = json!({
                "id": session_id,
                "name": sess.try_get::<String>("", "name").unwrap_or_default(),
                "term": sess.try_get::<String>("", "term").unwrap_or_default(),
                "academicYear": sess.try_get::<String>("", "academic_year").unwrap_or_default(),
            });
        }

        let ctx = json!({
            "id": entity_id,
            "seatingNumber": seat.try_get::<String>("", "seating_number").unwrap_or_default(),
            "sequenceNumber": seat.try_get::<i32>("", "sequence_number").unwrap_or(0),
            "level": seat.try_get::<String>("", "level").unwrap_or_default(),
            "division": seat.try_get::<Option<String>>("", "division").unwrap_or_default(),
            "studentNameAr": seat.try_get::<Option<String>>("", "student_name_ar").unwrap_or_default(),
            "session": session_json,
        });

        Ok(ctx)
    }

    fn schema(&self) -> Value {
        json!({
            "seatingNumber": { "type": "string", "label": "رقم الجلوس / Seating Number" },
            "sequenceNumber": { "type": "number", "label": "رقم التسلسل / Sequence Number" },
            "level": { "type": "string", "label": "المستوى الدراسي / Level" },
            "division": { "type": "string", "label": "الشعبة الدراسية / Division" },
            "studentNameAr": { "type": "string", "label": "اسم الطالب (عربي) / Student Name (AR)" },
            "session": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "label": "اسم دورة الامتحانات / Exam Session Name" },
                    "term": { "type": "string", "label": "الفصل الدراسي / Academic Term" },
                    "academicYear": { "type": "string", "label": "العام الدراسي / Academic Year" }
                }
            }
        })
    }
}

// ── User/Staff DataSource ─────────────────────────────────────

pub struct UserDataSource;

#[async_trait]
impl DataSource for UserDataSource {
    fn entity_type(&self) -> &str {
        "user"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        let stmt = select_by_id(db, "users", entity_id);
        let opt = db.query_one(stmt).await.map_err(|e| e.to_string())?;
        let user = opt.ok_or_else(|| format!("User not found: {}", entity_id))?;

        let ctx = json!({
            "id": entity_id,
            "fullName": user.try_get::<String>("", "full_name").unwrap_or_default(),
            "nameAr": user.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
            "nameEn": user.try_get::<Option<String>>("", "name_en").unwrap_or_default(),
            "email": user.try_get::<Option<String>>("", "email").unwrap_or_default(),
            "phone": user.try_get::<Option<String>>("", "phone").unwrap_or_default(),
            "sex": user.try_get::<Option<String>>("", "sex").unwrap_or_default(),
            "dateOfBirth": user.try_get::<Option<String>>("", "date_of_birth").unwrap_or_default(),
            "nationality": user.try_get::<Option<String>>("", "nationality").unwrap_or_default(),
            "nationalId": user.try_get::<Option<String>>("", "national_id").unwrap_or_default(),
            "primaryRole": user.try_get::<String>("", "primary_role").unwrap_or_default(),
        });

        Ok(ctx)
    }

    fn schema(&self) -> Value {
        json!({
            "fullName": { "type": "string", "label": "الاسم بالكامل / Full Name" },
            "nameAr": { "type": "string", "label": "الاسم بالعربية / Name (AR)" },
            "nameEn": { "type": "string", "label": "الاسم بالإنجليزية / Name (EN)" },
            "email": { "type": "string", "label": "البريد الإلكتروني / Email" },
            "phone": { "type": "string", "label": "رقم الهاتف / Phone Number" },
            "sex": { "type": "string", "label": "الجنس / Sex" },
            "dateOfBirth": { "type": "string", "label": "تاريخ الميلاد / Date of Birth" },
            "nationality": { "type": "string", "label": "الجنسية / Nationality" },
            "nationalId": { "type": "string", "label": "رقم الهوية الوطنية / National ID" },
            "primaryRole": { "type": "string", "label": "الدور الوظيفي الأساسي / Primary Role" }
        })
    }
}

// ── DataSource Registry ───────────────────────────────────────

pub struct DataSourceRegistry {
    sources: HashMap<String, Arc<dyn DataSource>>,
}

impl Default for DataSourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DataSourceRegistry {
    pub fn new() -> Self {
        let mut sources = HashMap::new();
        sources.insert("enrollment".to_string(), Arc::new(EnrollmentDataSource) as Arc<dyn DataSource>);
        sources.insert("payment".to_string(), Arc::new(PaymentDataSource) as Arc<dyn DataSource>);
        sources.insert("exam".to_string(), Arc::new(ExamDataSource) as Arc<dyn DataSource>);
        sources.insert("user".to_string(), Arc::new(UserDataSource) as Arc<dyn DataSource>);
        sources.insert("assessment_snapshot".to_string(), Arc::new(AssessmentSnapshotDataSource) as Arc<dyn DataSource>);
        sources.insert("report_card".to_string(), Arc::new(ReportCardDataSource) as Arc<dyn DataSource>);
        sources.insert("term_certificate".to_string(), Arc::new(TermCertificateDataSource) as Arc<dyn DataSource>);
        sources.insert("annual_certificate".to_string(), Arc::new(AnnualCertificateDataSource) as Arc<dyn DataSource>);
        sources.insert("exam_session".to_string(), Arc::new(ExamSessionDataSource) as Arc<dyn DataSource>);
        
        DataSourceRegistry { sources }
    }

    pub fn get(&self, entity_type: &str) -> Option<Arc<dyn DataSource>> {
        if let Some(source) = self.sources.get(entity_type).cloned() {
            Some(source)
        } else {
            // Fallback to dynamic ProjectionDataSource if not hardcoded
            Some(Arc::new(ProjectionDataSource {
                entity_type: entity_type.to_string(),
            }))
        }
    }

    pub fn list_schemas(&self) -> HashMap<String, Value> {
        let mut schemas = HashMap::new();
        for (name, src) in &self.sources {
            schemas.insert(name.clone(), src.schema());
        }
        schemas
    }
}

// ── Projection DataSource (CQRS) ──────────────────────────────

pub struct ProjectionDataSource {
    pub entity_type: String,
}

#[async_trait]
impl DataSource for ProjectionDataSource {
    fn entity_type(&self) -> &str {
        &self.entity_type
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        let stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT data FROM docforge_projections WHERE entity_type = $1 AND entity_id = $2",
            vec![self.entity_type.clone().into(), entity_id.into()],
        );
        let opt = db.query_one(stmt).await.map_err(|e| e.to_string())?;
        let record = opt.ok_or_else(|| format!("Projection not found for {} - {}", self.entity_type, entity_id))?;
        
        let data_str = record.try_get::<String>("", "data").unwrap_or_else(|_| "{}".to_string());
        let data: Value = serde_json::from_str(&data_str).unwrap_or_else(|_| json!({}));
        
        Ok(data)
    }

    fn schema(&self) -> Value {
        // Ideally this schema comes from `docforge_schema_registry`
        json!({
            "type": "object",
            "description": format!("CQRS Projection for {}", self.entity_type)
        })
    }
}

// ── Assessment Snapshot DataSource ─────────────────────────────
//
// Reads from `student_assessment_snapshots` for a single class_subject.
// Used in certificates and transcripts that show per-subject results.

pub struct AssessmentSnapshotDataSource;

#[async_trait]
impl DataSource for AssessmentSnapshotDataSource {
    fn entity_type(&self) -> &str {
        "assessment_snapshot"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        let stmt = select_by_id(db, "student_assessment_snapshots", entity_id);
        let opt = db.query_one(stmt).await.map_err(|e| e.to_string())?;
        let snap = opt.ok_or_else(|| format!("Assessment snapshot not found: {}", entity_id))?;

        let student_id: String = snap.try_get("", "student_id").map_err(|e| e.to_string())?;
        let period_id: String = snap.try_get("", "period_id").map_err(|e| e.to_string())?;
        let class_subject_id: String = snap.try_get("", "class_subject_id").map_err(|e| e.to_string())?;

        // Fetch student info
        let user_stmt = select_by_id(db, "users", &student_id);
        let user_opt = db.query_one(user_stmt).await.map_err(|e| e.to_string())?;

        let mut student_json = json!({});
        if let Some(user) = user_opt {
            student_json = json!({
                "fullName": user.try_get::<String>("", "full_name").unwrap_or_default(),
                "nameAr": user.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
                "nationalId": user.try_get::<Option<String>>("", "national_id").unwrap_or_default(),
            });
        }

        // Fetch period info
        let period_stmt = select_by_id(db, "academic_periods", &period_id);
        let period_opt = db.query_one(period_stmt).await.map_err(|e| e.to_string())?;
        let period_name = period_opt.as_ref()
            .and_then(|p| p.try_get::<String>("", "name").ok())
            .unwrap_or_default();

        // Fetch class_subject info
        let cs_stmt = select_by_id(db, "class_subjects", &class_subject_id);
        let cs_opt = db.query_one(cs_stmt).await.map_err(|e| e.to_string())?;
        let subject_name = cs_opt.as_ref()
            .and_then(|cs| cs.try_get::<Option<String>>("", "subject_name").ok().flatten())
            .unwrap_or_default();

        Ok(json!({
            "id": entity_id,
            "studentId": student_id,
            "periodId": period_id,
            "classSubjectId": class_subject_id,
            "totalScore": snap.try_get::<Option<f64>>("", "total_score").unwrap_or_default(),
            "gradeLabel": snap.try_get::<Option<String>>("", "grade_label").unwrap_or_default(),
            "gpaPoints": snap.try_get::<Option<f64>>("", "gpa_points").unwrap_or_default(),
            "isPassing": snap.try_get::<Option<i32>>("", "is_passing").unwrap_or_default().map(|v| v == 1),
            "rankInClass": snap.try_get::<Option<i32>>("", "rank_in_class").unwrap_or_default(),
            "rankInLevel": snap.try_get::<Option<i32>>("", "rank_in_level").unwrap_or_default(),
            "status": snap.try_get::<String>("", "status").unwrap_or_default(),
            "student": student_json,
            "periodName": period_name,
            "subjectName": subject_name,
        }))
    }

    fn schema(&self) -> Value {
        json!({
            "totalScore": { "type": "number", "label": "الدرجة الكلية / Total Score" },
            "gradeLabel": { "type": "string", "label": "التقدير / Grade Label" },
            "gpaPoints": { "type": "number", "label": "نقاط المعدل / GPA Points" },
            "isPassing": { "type": "boolean", "label": "ناجح / Is Passing" },
            "rankInClass": { "type": "number", "label": "الترتيب في الفصل / Rank in Class" },
            "rankInLevel": { "type": "number", "label": "الترتيب في المستوى / Rank in Level" },
            "status": { "type": "string", "label": "الحالة / Status" },
            "student": {
                "type": "object",
                "properties": {
                    "fullName": { "type": "string", "label": "الاسم الكامل / Full Name" },
                    "nameAr": { "type": "string", "label": "الاسم بالعربية / Name (AR)" },
                    "nationalId": { "type": "string", "label": "رقم الهوية / National ID" }
                }
            },
            "periodName": { "type": "string", "label": "الفترة الأكاديمية / Period Name" },
            "subjectName": { "type": "string", "label": "المادة / Subject Name" }
        })
    }
}

// ── Report Card DataSource ────────────────────────────────────
//
// Aggregates ALL snapshots for a student in a given period into a
// full report card. The entity_id is `{enrollment_id}:{period_id}`.
// Used for generating complete report cards via DocForge templates.

pub struct ReportCardDataSource;

#[async_trait]
impl DataSource for ReportCardDataSource {
    fn entity_type(&self) -> &str {
        "report_card"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        // entity_id format: "{enrollment_id}:{period_id}"
        let parts: Vec<&str> = entity_id.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("ReportCard entity_id must be in format 'enrollment_id:period_id'".to_string());
        }
        let enrollment_id = parts[0];
        let period_id = parts[1];

        // Fetch enrollment → student_id
        let enroll_stmt = select_by_id(db, "enrollments", enrollment_id);
        let enroll_opt = db.query_one(enroll_stmt).await.map_err(|e| e.to_string())?;
        let enroll = enroll_opt.ok_or_else(|| format!("Enrollment not found: {}", enrollment_id))?;
        let student_id: String = enroll.try_get("", "student_id").map_err(|e| e.to_string())?;

        // Fetch student
        let user_stmt = select_by_id(db, "users", &student_id);
        let user_opt = db.query_one(user_stmt).await.map_err(|e| e.to_string())?;
        let mut student_json = json!({});
        if let Some(user) = user_opt {
            student_json = json!({
                "fullName": user.try_get::<String>("", "full_name").unwrap_or_default(),
                "nameAr": user.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
                "nationalId": user.try_get::<Option<String>>("", "national_id").unwrap_or_default(),
            });
        }

        // Fetch period
        let period_stmt = select_by_id(db, "academic_periods", period_id);
        let period_opt = db.query_one(period_stmt).await.map_err(|e| e.to_string())?;
        let period_name = period_opt.as_ref()
            .and_then(|p| p.try_get::<String>("", "name").ok())
            .unwrap_or_default();

        // Fetch all snapshots for this student in this period
        let snapshots_stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT * FROM student_assessment_snapshots WHERE student_id = $1 AND period_id = $2 ORDER BY class_subject_id",
            vec![student_id.clone().into(), period_id.into()],
        );
        let snapshot_rows = db.query_all(snapshots_stmt).await.map_err(|e| e.to_string())?;

        let mut subjects = Vec::new();
        let mut total_gpa = 0.0f64;
        let mut passed_count = 0usize;
        let mut failed_count = 0usize;

        for row in &snapshot_rows {
            let total_score = row.try_get::<Option<f64>>("", "total_score").unwrap_or_default().unwrap_or(0.0);
            let grade_label = row.try_get::<Option<String>>("", "grade_label").unwrap_or_default().unwrap_or_default();
            let gpa = row.try_get::<Option<f64>>("", "gpa_points").unwrap_or_default().unwrap_or(0.0);
            let is_passing = row.try_get::<Option<i32>>("", "is_passing").unwrap_or_default().unwrap_or(0) == 1;
            let rank_class = row.try_get::<Option<i32>>("", "rank_in_class").unwrap_or_default();
            let cs_id = row.try_get::<String>("", "class_subject_id").unwrap_or_default();

            if is_passing { passed_count += 1; } else { failed_count += 1; }
            total_gpa += gpa;

            // Resolve subject name from class_subjects
            let cs_stmt = select_by_id(db, "class_subjects", &cs_id);
            let cs_opt = db.query_one(cs_stmt).await.ok().flatten();
            let subject_name = cs_opt.as_ref()
                .and_then(|cs| cs.try_get::<Option<String>>("", "subject_name").ok().flatten())
                .unwrap_or_else(|| cs_id.clone());

            subjects.push(json!({
                "subjectName": subject_name,
                "totalScore": total_score,
                "gradeLabel": grade_label,
                "gpaPoints": gpa,
                "isPassing": is_passing,
                "rankInClass": rank_class,
            }));
        }

        let subject_count = subjects.len().max(1) as f64;
        let overall_gpa = total_gpa / subject_count;

        Ok(json!({
            "enrollmentId": enrollment_id,
            "periodId": period_id,
            "periodName": period_name,
            "student": student_json,
            "subjects": subjects,
            "overallGpa": (overall_gpa * 100.0).round() / 100.0,
            "totalSubjects": subjects.len(),
            "passedSubjects": passed_count,
            "failedSubjects": failed_count,
        }))
    }

    fn schema(&self) -> Value {
        json!({
            "enrollmentId": { "type": "string", "label": "رقم القيد / Enrollment ID" },
            "periodId": { "type": "string", "label": "الفترة / Period ID" },
            "periodName": { "type": "string", "label": "اسم الفترة / Period Name" },
            "student": {
                "type": "object",
                "properties": {
                    "fullName": { "type": "string", "label": "الاسم الكامل / Full Name" },
                    "nameAr": { "type": "string", "label": "الاسم بالعربية / Name (AR)" },
                    "nationalId": { "type": "string", "label": "رقم الهوية / National ID" }
                }
            },
            "subjects": {
                "type": "array",
                "label": "المواد / Subjects",
                "items": {
                    "subjectName": { "type": "string", "label": "اسم المادة / Subject" },
                    "totalScore": { "type": "number", "label": "الدرجة / Score" },
                    "gradeLabel": { "type": "string", "label": "التقدير / Grade" },
                    "gpaPoints": { "type": "number", "label": "نقاط المعدل / GPA" },
                    "isPassing": { "type": "boolean", "label": "ناجح / Passing" },
                    "rankInClass": { "type": "number", "label": "الترتيب / Rank" }
                }
            },
            "overallGpa": { "type": "number", "label": "المعدل التراكمي / Overall GPA" },
            "totalSubjects": { "type": "number", "label": "عدد المواد / Total Subjects" },
            "passedSubjects": { "type": "number", "label": "المواد الناجحة / Passed" },
            "failedSubjects": { "type": "number", "label": "المواد الراسبة / Failed" }
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// Term Certificate DataSource
//
// Entity ID: "{enrollment_id}:{term_period_id}"
// Resolves monthly period breakdowns + exam for term certificates.
// ═══════════════════════════════════════════════════════════════

pub struct TermCertificateDataSource;

#[async_trait]
impl DataSource for TermCertificateDataSource {
    fn entity_type(&self) -> &str {
        "term_certificate"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        let parts: Vec<&str> = entity_id.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("TermCertificate entity_id must be 'enrollment_id:term_period_id'".into());
        }
        let enrollment_id = parts[0];
        let term_period_id = parts[1];

        // Fetch enrollment → student
        let enroll_stmt = select_by_id(db, "enrollments", enrollment_id);
        let enroll = db.query_one(enroll_stmt).await.map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Enrollment not found: {}", enrollment_id))?;
        let student_id: String = enroll.try_get("", "student_id").map_err(|e| e.to_string())?;
        let class_id: String = enroll.try_get("", "class_id").map_err(|e| e.to_string())?;

        // Student info
        let user_stmt = select_by_id(db, "users", &student_id);
        let user_opt = db.query_one(user_stmt).await.map_err(|e| e.to_string())?;
        let mut student_json = json!({});
        if let Some(user) = user_opt {
            student_json = json!({
                "fullName": user.try_get::<String>("", "full_name").unwrap_or_default(),
                "nameAr": user.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
                "nameEn": user.try_get::<Option<String>>("", "name_en").unwrap_or_default(),
                "nationalId": user.try_get::<Option<String>>("", "national_id").unwrap_or_default(),
            });
        }

        // Class info
        let class_stmt = select_by_id(db, "classes", &class_id);
        let class_opt = db.query_one(class_stmt).await.map_err(|e| e.to_string())?;
        let mut class_json = json!({});
        if let Some(class) = class_opt {
            class_json = json!({
                "name": class.try_get::<String>("", "name").unwrap_or_default(),
                "level": class.try_get::<String>("", "level").unwrap_or_default(),
            });
        }

        // Term period info
        let period_stmt = select_by_id(db, "academic_periods", term_period_id);
        let period_opt = db.query_one(period_stmt).await.map_err(|e| e.to_string())?;
        let term_name = period_opt.as_ref()
            .and_then(|p| p.try_get::<String>("", "name").ok())
            .unwrap_or_default();

        // Fetch all term_results for this student in this term
        let tr_stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT * FROM term_results WHERE student_id = $1 AND term_period_id = $2 ORDER BY class_subject_id",
            vec![student_id.clone().into(), term_period_id.into()],
        );
        let tr_rows = db.query_all(tr_stmt).await.map_err(|e| e.to_string())?;

        let mut subjects = Vec::new();
        let mut total_coursework = 0.0f64;
        let mut total_exam = 0.0f64;
        let mut total_score_sum = 0.0f64;

        for row in &tr_rows {
            let cs_id = row.try_get::<String>("", "class_subject_id").unwrap_or_default();
            let cs_stmt = select_by_id(db, "class_subjects", &cs_id);
            let cs_opt = db.query_one(cs_stmt).await.ok().flatten();
            let subject_name = cs_opt.as_ref()
                .and_then(|cs| cs.try_get::<Option<String>>("", "subject_name").ok().flatten())
                .unwrap_or_else(|| cs_id.clone());

            let cw = row.try_get::<f64>("", "coursework_score").unwrap_or(0.0);
            let ex = row.try_get::<f64>("", "exam_score").unwrap_or(0.0);
            let tot = row.try_get::<f64>("", "total_score").unwrap_or(0.0);
            let tot_max = row.try_get::<f64>("", "total_max").unwrap_or(50.0);
            let grade = row.try_get::<Option<String>>("", "grade_label").unwrap_or_default();
            let rank = row.try_get::<Option<i32>>("", "rank").unwrap_or_default();
            let passing = row.try_get::<i32>("", "is_passing").unwrap_or(0) == 1;

            total_coursework += cw;
            total_exam += ex;
            total_score_sum += tot;

            subjects.push(json!({
                "subjectName": subject_name,
                "courseworkScore": cw,
                "examScore": ex,
                "totalScore": tot,
                "totalMax": tot_max,
                "gradeLabel": grade,
                "rank": rank,
                "isPassing": passing,
            }));
        }

        let subject_count = subjects.len().max(1) as f64;

        Ok(json!({
            "enrollmentId": enrollment_id,
            "termPeriodId": term_period_id,
            "termName": term_name,
            "student": student_json,
            "class": class_json,
            "subjects": subjects,
            "totalCoursework": total_coursework,
            "totalExam": total_exam,
            "termTotal": total_score_sum,
            "subjectCount": subjects.len(),
        }))
    }

    fn schema(&self) -> Value {
        json!({
            "termName": { "type": "string", "label": "اسم الفصل / Term Name" },
            "student": { "type": "object", "properties": {
                "fullName": { "type": "string", "label": "الاسم / Full Name" },
                "nameAr": { "type": "string", "label": "الاسم بالعربية / Name (AR)" },
                "nationalId": { "type": "string", "label": "رقم الهوية / National ID" }
            }},
            "class": { "type": "object", "properties": {
                "name": { "type": "string", "label": "الفصل / Class" },
                "level": { "type": "string", "label": "المستوى / Level" }
            }},
            "subjects": { "type": "array", "label": "المواد / Subjects", "items": {
                "subjectName": { "type": "string", "label": "المادة / Subject" },
                "courseworkScore": { "type": "number", "label": "أعمال السنة / Coursework" },
                "examScore": { "type": "number", "label": "الاختبار / Exam" },
                "totalScore": { "type": "number", "label": "المجموع / Total" },
                "gradeLabel": { "type": "string", "label": "التقدير / Grade" },
                "rank": { "type": "number", "label": "الترتيب / Rank" },
                "isPassing": { "type": "boolean", "label": "ناجح / Passing" }
            }},
            "totalCoursework": { "type": "number", "label": "مجموع أعمال السنة / Total Coursework" },
            "totalExam": { "type": "number", "label": "مجموع الاختبارات / Total Exam" },
            "termTotal": { "type": "number", "label": "مجموع الفصل / Term Total" }
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// Annual Certificate DataSource
//
// Entity ID: "{enrollment_id}:{year_period_id}"
// Resolves multi-term data for annual certificates.
// ═══════════════════════════════════════════════════════════════

pub struct AnnualCertificateDataSource;

#[async_trait]
impl DataSource for AnnualCertificateDataSource {
    fn entity_type(&self) -> &str {
        "annual_certificate"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        let parts: Vec<&str> = entity_id.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("AnnualCertificate entity_id must be 'enrollment_id:year_period_id'".into());
        }
        let enrollment_id = parts[0];
        let year_period_id = parts[1];

        // Enrollment + student
        let enroll = db.query_one(select_by_id(db, "enrollments", enrollment_id))
            .await.map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Enrollment not found: {}", enrollment_id))?;
        let student_id: String = enroll.try_get("", "student_id").map_err(|e| e.to_string())?;

        let user_opt = db.query_one(select_by_id(db, "users", &student_id))
            .await.map_err(|e| e.to_string())?;
        let mut student_json = json!({});
        if let Some(user) = user_opt {
            student_json = json!({
                "fullName": user.try_get::<String>("", "full_name").unwrap_or_default(),
                "nameAr": user.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
                "nationalId": user.try_get::<Option<String>>("", "national_id").unwrap_or_default(),
            });
        }

        // Annual results
        let ar_stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT * FROM annual_results WHERE student_id = $1 AND year_period_id = $2 ORDER BY class_subject_id",
            vec![student_id.clone().into(), year_period_id.into()],
        );
        let ar_rows = db.query_all(ar_stmt).await.map_err(|e| e.to_string())?;

        let mut subjects = Vec::new();
        let mut grand_total = 0.0f64;
        let mut grand_max = 0.0f64;

        for row in &ar_rows {
            let cs_id = row.try_get::<String>("", "class_subject_id").unwrap_or_default();
            let cs_opt = db.query_one(select_by_id(db, "class_subjects", &cs_id)).await.ok().flatten();
            let subject_name = cs_opt.as_ref()
                .and_then(|cs| cs.try_get::<Option<String>>("", "subject_name").ok().flatten())
                .unwrap_or_else(|| cs_id.clone());

            let annual_total = row.try_get::<f64>("", "annual_total").unwrap_or(0.0);
            let annual_max = row.try_get::<f64>("", "annual_max").unwrap_or(100.0);
            let pct = row.try_get::<f64>("", "percentage").unwrap_or(0.0);
            let grade = row.try_get::<Option<String>>("", "grade_label").unwrap_or_default();
            let rank = row.try_get::<Option<i32>>("", "rank").unwrap_or_default();
            let passing = row.try_get::<i32>("", "is_passing").unwrap_or(0) == 1;
            let terms_json = row.try_get::<String>("", "term_scores_json").unwrap_or_else(|_| "[]".into());

            grand_total += annual_total;
            grand_max += annual_max;

            subjects.push(json!({
                "subjectName": subject_name,
                "annualTotal": annual_total,
                "annualMax": annual_max,
                "percentage": pct,
                "gradeLabel": grade,
                "rank": rank,
                "isPassing": passing,
                "termBreakdown": serde_json::from_str::<Value>(&terms_json).unwrap_or(json!([])),
            }));
        }

        let year_stmt = select_by_id(db, "academic_periods", year_period_id);
        let year_opt = db.query_one(year_stmt).await.map_err(|e| e.to_string())?;
        let year_name = year_opt.as_ref()
            .and_then(|p| p.try_get::<String>("", "name").ok())
            .unwrap_or_default();

        Ok(json!({
            "enrollmentId": enrollment_id,
            "yearPeriodId": year_period_id,
            "yearName": year_name,
            "student": student_json,
            "subjects": subjects,
            "grandTotal": grand_total,
            "grandMax": grand_max,
            "overallPercentage": if grand_max > 0.0 { (grand_total / grand_max * 100.0 * 100.0).round() / 100.0 } else { 0.0 },
            "subjectCount": subjects.len(),
        }))
    }

    fn schema(&self) -> Value {
        json!({
            "yearName": { "type": "string", "label": "العام الدراسي / Academic Year" },
            "student": { "type": "object", "properties": {
                "fullName": { "type": "string", "label": "الاسم / Full Name" },
                "nameAr": { "type": "string", "label": "الاسم بالعربية / Name (AR)" },
                "nationalId": { "type": "string", "label": "رقم الهوية / National ID" }
            }},
            "subjects": { "type": "array", "label": "المواد / Subjects", "items": {
                "subjectName": { "type": "string", "label": "المادة / Subject" },
                "annualTotal": { "type": "number", "label": "المجموع السنوي / Annual Total" },
                "percentage": { "type": "number", "label": "النسبة / Percentage" },
                "gradeLabel": { "type": "string", "label": "التقدير / Grade" },
                "rank": { "type": "number", "label": "الترتيب / Rank" },
                "isPassing": { "type": "boolean", "label": "ناجح / Passing" }
            }},
            "grandTotal": { "type": "number", "label": "المجموع الكلي / Grand Total" },
            "overallPercentage": { "type": "number", "label": "النسبة الكلية / Overall %" }
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// Exam Session DataSource (Generic)
//
// Entity ID: exam_session_id
// Generic data source for all exam documents (attendance sheets,
// committee lists, seating plans, invigilator lists, etc.).
// Specialized projections are built on top via DocForge templates.
// ═══════════════════════════════════════════════════════════════

pub struct ExamSessionDataSource;

#[async_trait]
impl DataSource for ExamSessionDataSource {
    fn entity_type(&self) -> &str {
        "exam_session"
    }

    async fn resolve(&self, db: &DatabaseConnection, entity_id: &str) -> Result<Value, String> {
        // Fetch session
        let sess_stmt = select_by_id(db, "exam_sessions", entity_id);
        let sess = db.query_one(sess_stmt).await.map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Exam session not found: {}", entity_id))?;

        let session_json = json!({
            "id": entity_id,
            "name": sess.try_get::<String>("", "name").unwrap_or_default(),
            "term": sess.try_get::<Option<String>>("", "term").unwrap_or_default(),
            "academicYear": sess.try_get::<Option<String>>("", "academic_year").unwrap_or_default(),
            "startDate": sess.try_get::<Option<String>>("", "start_date").unwrap_or_default(),
            "endDate": sess.try_get::<Option<String>>("", "end_date").unwrap_or_default(),
            "status": sess.try_get::<String>("", "status").unwrap_or_default(),
        });

        // Fetch all halls for this session
        let halls_stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT * FROM exam_halls WHERE session_id = $1 ORDER BY name",
            vec![entity_id.into()],
        );
        let hall_rows = db.query_all(halls_stmt).await.map_err(|e| e.to_string())?;
        let halls: Vec<Value> = hall_rows.iter().map(|h| json!({
            "id": h.try_get::<String>("", "id").unwrap_or_default(),
            "name": h.try_get::<String>("", "name").unwrap_or_default(),
            "capacity": h.try_get::<Option<i32>>("", "capacity").unwrap_or_default(),
            "building": h.try_get::<Option<String>>("", "building").unwrap_or_default(),
            "floor": h.try_get::<Option<String>>("", "floor").unwrap_or_default(),
        })).collect();

        // Fetch all seating numbers for this session
        let seats_stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT * FROM seating_numbers WHERE exam_session_id = $1 ORDER BY sequence_number",
            vec![entity_id.into()],
        );
        let seat_rows = db.query_all(seats_stmt).await.map_err(|e| e.to_string())?;
        let students: Vec<Value> = seat_rows.iter().map(|s| json!({
            "seatingNumber": s.try_get::<String>("", "seating_number").unwrap_or_default(),
            "sequenceNumber": s.try_get::<i32>("", "sequence_number").unwrap_or(0),
            "studentNameAr": s.try_get::<Option<String>>("", "student_name_ar").unwrap_or_default(),
            "level": s.try_get::<String>("", "level").unwrap_or_default(),
            "division": s.try_get::<Option<String>>("", "division").unwrap_or_default(),
            "hallId": s.try_get::<Option<String>>("", "hall_id").unwrap_or_default(),
        })).collect();

        // Fetch proctors/invigilators
        let proctors_stmt = sea_orm::Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT p.*, u.full_name, u.name_ar FROM exam_proctors p LEFT JOIN users u ON p.user_id = u.id WHERE p.session_id = $1",
            vec![entity_id.into()],
        );
        let proctor_rows = db.query_all(proctors_stmt).await.unwrap_or_default();
        let proctors: Vec<Value> = proctor_rows.iter().map(|p| json!({
            "name": p.try_get::<Option<String>>("", "full_name").unwrap_or_default(),
            "nameAr": p.try_get::<Option<String>>("", "name_ar").unwrap_or_default(),
            "role": p.try_get::<Option<String>>("", "role").unwrap_or_default(),
            "hallId": p.try_get::<Option<String>>("", "hall_id").unwrap_or_default(),
        })).collect();

        Ok(json!({
            "session": session_json,
            "halls": halls,
            "students": students,
            "proctors": proctors,
            "totalStudents": students.len(),
            "totalHalls": halls.len(),
            "totalProctors": proctors.len(),
        }))
    }

    fn schema(&self) -> Value {
        json!({
            "session": { "type": "object", "properties": {
                "name": { "type": "string", "label": "اسم الدورة / Session Name" },
                "term": { "type": "string", "label": "الفصل / Term" },
                "academicYear": { "type": "string", "label": "العام / Year" },
                "startDate": { "type": "string", "label": "بداية / Start" },
                "endDate": { "type": "string", "label": "نهاية / End" },
                "status": { "type": "string", "label": "الحالة / Status" }
            }},
            "halls": { "type": "array", "label": "القاعات / Halls", "items": {
                "name": { "type": "string", "label": "اسم القاعة / Hall" },
                "capacity": { "type": "number", "label": "السعة / Capacity" },
                "building": { "type": "string", "label": "المبنى / Building" }
            }},
            "students": { "type": "array", "label": "الطلاب / Students", "items": {
                "seatingNumber": { "type": "string", "label": "رقم الجلوس / Seat" },
                "studentNameAr": { "type": "string", "label": "الاسم / Name" },
                "level": { "type": "string", "label": "المستوى / Level" }
            }},
            "proctors": { "type": "array", "label": "المراقبون / Proctors", "items": {
                "name": { "type": "string", "label": "الاسم / Name" },
                "role": { "type": "string", "label": "الدور / Role" }
            }},
            "totalStudents": { "type": "number", "label": "عدد الطلاب / Total Students" },
            "totalHalls": { "type": "number", "label": "عدد القاعات / Total Halls" },
            "totalProctors": { "type": "number", "label": "عدد المراقبين / Total Proctors" }
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// BRIDGE: docforge-orbis DataSource → docforge-engine DataSource
//
// This adapter wraps an Orbis DataSource (which requires &DatabaseConnection)
// into a docforge-engine DataSource (which is self-contained). By binding a
// DB connection at construction time, we allow the engine to call resolve_one
// without knowing about Sea-ORM.
// ═══════════════════════════════════════════════════════════════

use docforge_engine::plugin::data_source::{
    DataSource as EngineDataSource,
    DataContext,
    DataFilter as EngineDataFilter,
    DataSchema,
    DataFieldSchema,
};
use docforge_engine::error::DocforgeError;

/// Bridges an Orbis DataSource (DB-aware) to the engine DataSource trait.
pub struct OrbisDataSourceAdapter {
    inner: Arc<dyn DataSource>,
    db: Arc<DatabaseConnection>,
}

impl OrbisDataSourceAdapter {
    pub fn new(inner: Arc<dyn DataSource>, db: Arc<DatabaseConnection>) -> Self {
        Self { inner, db }
    }
}

#[async_trait]
impl EngineDataSource for OrbisDataSourceAdapter {
    fn entity_type(&self) -> &str {
        self.inner.entity_type()
    }

    fn schema(&self) -> &DataSchema {
        // We leak a static ref since the engine trait returns &DataSchema.
        // This is safe for long-lived adapters (registered once per request).
        let orbis_schema = self.inner.schema();
        let fields = schema_value_to_fields(&orbis_schema);
        let ds = DataSchema {
            entity_type: self.inner.entity_type().to_string(),
            label: self.inner.entity_type().to_string(),
            label_ar: None,
            fields,
        };
        Box::leak(Box::new(ds))
    }

    async fn resolve_one(&self, id: &str) -> Result<DataContext, DocforgeError> {
        self.inner.resolve(&self.db, id).await
            .map_err(|e| DocforgeError::DataSourceError(e))
    }

    async fn resolve_many(&self, _filter: &EngineDataFilter) -> Result<Vec<DataContext>, DocforgeError> {
        // For most Orbis data sources, batch queries are not yet implemented.
        // Return empty — the engine will fall back to resolve_one per item.
        Ok(Vec::new())
    }

    async fn count(&self, _filter: &EngineDataFilter) -> Result<u64, DocforgeError> {
        Ok(0)
    }

    fn sample_data(&self) -> DataContext {
        json!({})
    }
}

/// Convert a JSON schema value to engine DataFieldSchema vec.
fn schema_value_to_fields(val: &Value) -> Vec<DataFieldSchema> {
    let mut fields = Vec::new();
    if let Some(obj) = val.as_object() {
        for (key, v) in obj {
            let field_type = v.get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("string")
                .to_string();
            let label = v.get("label")
                .and_then(|l| l.as_str())
                .unwrap_or(key.as_str())
                .to_string();
            let children = v.get("properties").map(|p| schema_value_to_fields(p))
                .or_else(|| v.get("items").map(|i| schema_value_to_fields(i)));
            fields.push(DataFieldSchema {
                path: key.clone(),
                field_type,
                label,
                label_ar: None,
                children,
            });
        }
    }
    fields
}
