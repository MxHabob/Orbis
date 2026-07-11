use std::sync::Arc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use shared_kernel::errors::AppError;
use svc_iam::entities::user;
use svc_academic::entities::{enrollment, student_guardian};
mod local_finance {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "fee_templates")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub org_id: String,
        pub name: String,
        pub amount: f64,
        pub is_mandatory: i32,
        pub is_active: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

mod local_student_fee {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "student_fees")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub org_id: String,
        pub enrollment_id: String,
        pub fee_template_id: String,
        pub amount: f64,
        pub discount_amount: f64,
        pub net_amount: f64,
        pub paid_amount: f64,
        pub due_date: String,
        pub status: String,
        pub notes: Option<String>,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

use local_finance as fee_template;
use local_student_fee as student_fee;

use crate::services::settings::OrganizationSettingsService;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterStudentRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub nationality: Option<String>,
    pub national_id: Option<String>,
    pub address: Option<String>,
    pub class_id: Option<String>,
    pub guardian_name: Option<String>,
    pub guardian_phone: Option<String>,
    pub guardian_relationship: Option<String>,
}

pub struct StudentRegistrationService {
    pub settings_service: Arc<OrganizationSettingsService>,
}

impl StudentRegistrationService {
    pub fn new(settings_service: Arc<OrganizationSettingsService>) -> Self {
        Self { settings_service }
    }

    /// Register a student: inserts user, enrollment, links guardians, and assigns mandatory fees.
    pub async fn register_student<C: ConnectionTrait>(
        &self,
        db: &C,
        org_id: &str,
        req: RegisterStudentRequest,
        created_by: Option<&str>,
    ) -> Result<enrollment::Model, AppError> {
        let now = chrono::Utc::now().to_rfc3339();

        // 1. Validate required fields from Organization Settings
        let settings = self.settings_service.get_settings(org_id).await?;
        for field in &settings.required_registration_fields {
            match field.as_str() {
                "email" => {
                    if req.email.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                        return Err(AppError::validation("Email is required by organization policy"));
                    }
                }
                "phone" => {
                    if req.phone.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                        return Err(AppError::validation("Phone is required by organization policy"));
                    }
                }
                "national_id" => {
                    if req.national_id.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
                        return Err(AppError::validation("National ID is required by organization policy"));
                    }
                }
                _ => {}
            }
        }

        // 2. Check duplicate email / phone / national_id in IAM
        if let Some(ref email) = req.email {
            if !email.trim().is_empty() {
                let exists = user::Entity::find()
                    .filter(user::Column::OrgId.eq(org_id))
                    .filter(user::Column::Email.eq(email))
                    .one(db)
                    .await?
                    .is_some();
                if exists {
                    return Err(AppError::conflict(format!("Email '{}' is already registered", email)));
                }
            }
        }

        if let Some(ref nid) = req.national_id {
            if !nid.trim().is_empty() {
                let exists = user::Entity::find()
                    .filter(user::Column::OrgId.eq(org_id))
                    .filter(user::Column::NationalId.eq(nid))
                    .one(db)
                    .await?
                    .is_some();
                if exists {
                    return Err(AppError::conflict(format!("National ID '{}' is already registered", nid)));
                }
            }
        }

        // 3. Create student user in IAM
        let student_user_id = uuid::Uuid::new_v4().to_string();
        let cost = if cfg!(debug_assertions) { 4 } else { 10 };
        let password_hash = bcrypt::hash(req.password.as_deref().unwrap_or("123456"), cost)
            .map_err(|e| AppError::internal(format!("Bcrypt hashing failed: {}", e)))?;

        let new_user = user::Model {
            id: student_user_id.clone(),
            org_id: org_id.to_string(),
            email: req.email.clone(),
            email_verified: 1,
            email_verified_at: Some(now.clone()),
            phone: req.phone.clone(),
            phone_verified: 0,
            phone_verified_at: None,
            password_hash: Some(password_hash),
            password_changed_at: None,
            password_expires_at: None,
            mfa_enabled: 0,
            mfa_secret: None,
            mfa_backup_codes: None,
            full_name: req.name.clone(),
            name_ar: Some(req.name.clone()),
            name_en: None,
            avatar: None,
            sex: req.gender.clone(),
            date_of_birth: req.birth_date.clone(),
            place_of_birth: None,
            nationality: req.nationality.clone(),
            national_id: req.national_id.clone(),
            passport_number: None,
            address: req.address.clone(),
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
            primary_role: "student".to_string(),
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
            created_at: now.clone(),
            updated_at: now.clone(),
            created_by: created_by.map(String::from),
        };
        let active_user: user::ActiveModel = new_user.into();
        active_user.insert(db).await?;

        // 4. Determine Dynamic Student Number based on organization settings
        let sequence = enrollment::Entity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .count(db)
            .await? + 1;
        let student_number = self.settings_service.format_student_number(org_id, sequence).await?;

        // 5. Create Enrollment record
        let class_id = req.class_id.clone().unwrap_or_default();
        let enrollment_id = uuid::Uuid::new_v4().to_string();
        let new_enrollment = enrollment::Model {
            id: enrollment_id.clone(),
            org_id: org_id.to_string(),
            cycle_id: String::new(),
            student_id: student_user_id.clone(),
            class_id: class_id.clone(),
            student_number: student_number.clone(),
            status: "active".to_string(),
            enrollment_date: now.clone(),
            enrollment_type: "standard".to_string(),
            withdrawal_date: None,
            withdrawal_reason: None,
            final_grade: None,
            final_score: None,
            final_rank: None,
            is_promoted: None,
            promotion_decision: None,
            promotion_notes: None,
            meta: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        let active_enrollment: enrollment::ActiveModel = new_enrollment.clone().into();
        active_enrollment.insert(db).await?;

        // 6. Link Guardian if provided
        if let (Some(g_name), Some(g_phone)) = (req.guardian_name, req.guardian_phone) {
            if !g_name.trim().is_empty() && !g_phone.trim().is_empty() {
                let guardian_id = uuid::Uuid::new_v4().to_string();
                
                // Create guardian user first
                let g_user = user::Model {
                    id: guardian_id.clone(),
                    org_id: org_id.to_string(),
                    email: None,
                    email_verified: 0,
                    email_verified_at: None,
                    phone: Some(g_phone.clone()),
                    phone_verified: 0,
                    phone_verified_at: None,
                    password_hash: None,
                    password_changed_at: None,
                    password_expires_at: None,
                    mfa_enabled: 0,
                    mfa_secret: None,
                    mfa_backup_codes: None,
                    full_name: g_name.clone(),
                    name_ar: Some(g_name.clone()),
                    name_en: None,
                    avatar: None,
                    sex: None,
                    date_of_birth: None,
                    place_of_birth: None,
                    nationality: None,
                    national_id: None,
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
                    primary_role: "guardian".to_string(),
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
                    created_at: now.clone(),
                    updated_at: now.clone(),
                    created_by: created_by.map(String::from),
                };
                let active_g_user: user::ActiveModel = g_user.into();
                active_g_user.insert(db).await?;

                // Create student-guardian link
                let link = student_guardian::Model {
                    id: uuid::Uuid::new_v4().to_string(),
                    student_id: student_user_id.clone(),
                    guardian_id,
                    relationship: req.guardian_relationship.unwrap_or_else(|| "father".to_string()),
                    relationship_ar: None,
                    is_primary: 1,
                    is_emergency_contact: 1,
                    is_authorized_pickup: 1,
                    can_view_grades: 1,
                    can_view_attendance: 1,
                    can_view_behavior: 1,
                    can_view_finance: 1,
                    can_view_schedule: 1,
                    can_receive_notifications: 1,
                    is_payer: 1,
                    payment_share_percent: None,
                    occupation: None,
                    employer: None,
                    work_phone: None,
                    notes: None,
                    created_at: now.clone(),
                    updated_at: now.clone(),
                };
                let active_link: student_guardian::ActiveModel = link.into();
                active_link.insert(db).await?;
            }
        }

        // 7. Auto-assign mandatory active fee templates (Initial invoicing)
        let mandatory_templates = fee_template::Entity::find()
            .filter(fee_template::Column::OrgId.eq(org_id))
            .filter(fee_template::Column::IsActive.eq(1))
            .filter(fee_template::Column::IsMandatory.eq(1))
            .all(db)
            .await?;

        for template in mandatory_templates {
            let student_fee_id = uuid::Uuid::new_v4().to_string();
            let due_date = (chrono::Utc::now() + chrono::Duration::days(30)).naive_utc().date().to_string();
            let new_fee = student_fee::Model {
                id: student_fee_id,
                org_id: org_id.to_string(),
                enrollment_id: enrollment_id.clone(),
                fee_template_id: template.id.clone(),
                amount: template.amount,
                discount_amount: 0.0,
                net_amount: template.amount,
                paid_amount: 0.0,
                due_date,
                status: "unpaid".to_string(),
                notes: Some("Automatically assigned on registration".to_string()),
                created_at: now.clone(),
                updated_at: now.clone(),
            };
            let active_fee: student_fee::ActiveModel = new_fee.into();
            active_fee.insert(db).await?;
        }

        Ok(new_enrollment)
    }
}
