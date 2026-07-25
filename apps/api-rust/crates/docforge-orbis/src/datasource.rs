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

        let ctx = json!({
            "id": entity_id,
            "studentNumber": enroll.try_get::<String>("", "student_number").unwrap_or_default(),
            "status": enroll.try_get::<String>("", "status").unwrap_or_default(),
            "enrollmentDate": enroll.try_get::<String>("", "enrollment_date").unwrap_or_default(),
            "finalGrade": enroll.try_get::<Option<String>>("", "final_grade").unwrap_or_default(),
            "finalScore": enroll.try_get::<Option<i32>>("", "final_score").unwrap_or_default(),
            "finalRank": enroll.try_get::<Option<i32>>("", "final_rank").unwrap_or_default(),
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
