use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::entities::enrollment;
use crate::repository::enrollment::EnrollmentUpdate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateEnrollmentRequest {
    pub student_id: String,
    pub class_id: String,
    pub cycle_id: Option<String>,
    pub enrollment_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct WithdrawRequest {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TransferRequest {
    pub new_class_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PromoteRequest {
    pub class_id: String,
    pub passing_score: Option<i32>,
    pub min_attendance_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PromotionResult {
    pub total_students: usize,
    pub promoted: usize,
    pub retained: usize,
    pub details: Vec<PromotionDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PromotionDetail {
    pub enrollment_id: String,
    pub student_id: String,
    pub student_name: String,
    pub final_score: Option<i32>,
    pub attendance_rate: f64,
    pub decision: String,
    pub notes: String,
}

impl super::Service {
    pub async fn get_enrollments(
        &self,
        org_id: &str,
        params: crate::params::EnrollmentParams,
    ) -> Result<Vec<serde_json::Value>, String> {
        use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
        use crate::entities::user;
        use std::collections::HashMap;

        // 1. Fetch raw enrollments
        let enrollments = self.repo.get_enrollments(org_id, params).await
            .map_err(|e| e.to_string())?;

        // 2. Fetch user details for each unique student_id
        let student_ids: Vec<String> = enrollments
            .iter()
            .map(|e| e.student_id.clone())
            .collect();

        let mut users_map = HashMap::new();
        if !student_ids.is_empty() {
            let users = user::Entity::find()
                .filter(user::Column::Id.is_in(student_ids))
                .filter(user::Column::IsDeleted.eq(0))
                .all(&self.repo.db)
                .await
                .map_err(|e| e.to_string())?;

            for u in users {
                users_map.insert(u.id.clone(), u);
            }
        }

        // 3. Merge user details into enrollments as JSON values
        let mut merged = Vec::new();
        for e in enrollments {
            let mut val = serde_json::to_value(&e).map_err(|e| e.to_string())?;
            if let Some(obj) = val.as_object_mut() {
                if let Some(u) = users_map.get(&e.student_id) {
                    obj.insert("full_name".to_string(), serde_json::Value::String(u.full_name.clone()));
                    obj.insert("name_ar".to_string(), serde_json::to_value(&u.name_ar).unwrap_or(serde_json::Value::Null));
                    obj.insert("name_en".to_string(), serde_json::to_value(&u.name_en).unwrap_or(serde_json::Value::Null));
                    obj.insert("email".to_string(), serde_json::to_value(&u.email).unwrap_or(serde_json::Value::Null));
                    obj.insert("phone_number".to_string(), serde_json::to_value(&u.phone).unwrap_or(serde_json::Value::Null));
                    obj.insert("avatar".to_string(), serde_json::to_value(&u.avatar).unwrap_or(serde_json::Value::Null));
                    obj.insert("birth_date".to_string(), serde_json::to_value(&u.date_of_birth).unwrap_or(serde_json::Value::Null));
                    obj.insert("address".to_string(), serde_json::to_value(&u.address).unwrap_or(serde_json::Value::Null));
                    obj.insert("sex".to_string(), serde_json::to_value(&u.sex).unwrap_or(serde_json::Value::Null));
                }
            }
            merged.push(val);
        }

        Ok(merged)
    }

    pub async fn get_enrollment(&self, enrollment_id: &str) -> Result<serde_json::Value, String> {

        let enrollment = self.repo.get_enrollment_by_id(enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "enrollment not found".to_string())?;

        let mut val = serde_json::to_value(&enrollment).map_err(|e| e.to_string())?;
        if let Some(obj) = val.as_object_mut() {
            if let Some(u) = self.repo.get_user_by_id(&enrollment.student_id).await.map_err(|e| e.to_string())? {
                obj.insert("full_name".to_string(), serde_json::Value::String(u.full_name.clone()));
                obj.insert("name_ar".to_string(), serde_json::to_value(&u.name_ar).unwrap_or(serde_json::Value::Null));
                obj.insert("name_en".to_string(), serde_json::to_value(&u.name_en).unwrap_or(serde_json::Value::Null));
                obj.insert("email".to_string(), serde_json::to_value(&u.email).unwrap_or(serde_json::Value::Null));
                obj.insert("phone_number".to_string(), serde_json::to_value(&u.phone).unwrap_or(serde_json::Value::Null));
                obj.insert("avatar".to_string(), serde_json::to_value(&u.avatar).unwrap_or(serde_json::Value::Null));
                obj.insert("birth_date".to_string(), serde_json::to_value(&u.date_of_birth).unwrap_or(serde_json::Value::Null));
                obj.insert("address".to_string(), serde_json::to_value(&u.address).unwrap_or(serde_json::Value::Null));
                obj.insert("sex".to_string(), serde_json::to_value(&u.sex).unwrap_or(serde_json::Value::Null));
            }
        }

        Ok(val)
    }

    pub async fn create_enrollment(&self, org_id: &str, req: CreateEnrollmentRequest) -> Result<enrollment::Model, String> {
        // Find the current cycle or use specified cycle
        let cycle_id = match req.cycle_id {
            Some(cid) => cid,
            None => {
                let cycle = self.repo.get_current_cycle(org_id).await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| "no active academic cycle found".to_string())?;
                cycle.id
            }
        };

        // Check if student exists
        let student = self.repo.get_user_by_id(&req.student_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "student not found".to_string())?;

        // Check if already enrolled in this cycle
        let existing = self.repo.get_enrollment_by_student_id(&req.student_id, &cycle_id).await
            .map_err(|e| e.to_string())?;
        if existing.is_some() {
            return Err("student is already enrolled in this cycle".to_string());
        }

        // Generate student number
        let count = self.repo.get_enrollment_count(org_id).await
            .map_err(|e| e.to_string())?;
        let student_number = format!("STU-{:06}", count + 1);

        let new_enrollment = enrollment::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: cycle_id.clone(),
            student_id: req.student_id.clone(),
            class_id: req.class_id.clone(),
            student_number: student_number.clone(),
            status: "active".to_string(),
            enrollment_date: Utc::now().format("%Y-%m-%d").to_string(),
            enrollment_type: req.enrollment_type.unwrap_or_else(|| "regular".to_string()),
            withdrawal_date: None,
            withdrawal_reason: None,
            final_grade: None,
            final_score: None,
            final_rank: None,
            is_promoted: None,
            promotion_decision: None,
            promotion_notes: None,
            meta: Some("{}".to_string()),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        // Save enrollment first
        let created_enrollment = self.repo.create_enrollment(new_enrollment).await
            .map_err(|e| e.to_string())?;

        // Auto-create student sub-ledger account in Chart of Accounts
        let student_ledger_account = self.get_or_create_sub_account(
            org_id,
            "student",
            &student.id,
            Some(&student.full_name),
            student.name_ar.as_deref(),
        ).await?;

        // 1. Get class details to determine level/grade
        let class_info = self.repo.get_class_by_id(&req.class_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "class not found".to_string())?;

        // 2. Fetch and apply active fee templates
        if let Ok(fee_templates) = self.repo.get_active_fee_templates(org_id).await {
            for tpl in fee_templates {
                // Check applicability rules
                let is_applicable = if let Some(rules_str) = &tpl.applicable_levels {
                    if let Ok(rules) = serde_json::from_str::<serde_json::Value>(rules_str) {
                        if let Some(levels) = rules.get("levels").and_then(|l| l.as_array()) {
                            levels.iter().any(|l| l.as_str() == Some(&class_info.level))
                        } else if let Some(level) = rules.get("level").and_then(|l| l.as_str()) {
                            level == class_info.level
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    true // Applies to all if no rules are specified
                };

                if is_applicable {
                    let original_amount = tpl.default_amount;
                    let total_amount = original_amount;

                    let sf = crate::entities::student_fee::Model {
                        id: Uuid::new_v4().to_string(),
                        org_id: org_id.to_string(),
                        cycle_id: cycle_id.clone(),
                        enrollment_id: created_enrollment.id.clone(),
                        fee_template_id: tpl.id.clone(),
                        original_amount,
                        discount_amount: 0.0,
                        net_amount: total_amount,
                        status: "unpaid".to_string(),
                        due_date: Utc::now().checked_add_signed(chrono::Duration::days(30))
                            .unwrap_or_else(Utc::now)
                            .format("%Y-%m-%d")
                            .to_string(),
                        discount_details: None,
                        parent_fee_id: None,
                        installment_number: None,
                        notes: Some(format!("Auto-generated fee: {}", tpl.name)),
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                    };

                    if let Ok(created_fee) = self.repo.create_student_fee(sf).await {
                        // Create double entry bookkeeping journal entry if amount > 0
                        if original_amount > 0.0 {
                            let je = crate::entities::journal_entry::Model {
                                id: Uuid::new_v4().to_string(),
                                org_id: org_id.to_string(),
                                transaction_ref: format!("ENR-{}", created_enrollment.student_number),
                                accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
                                fiscal_period_id: cycle_id.clone(),
                                amount: original_amount,
                                debit_account_id: student_ledger_account.id.clone(),
                                credit_account_id: tpl.revenue_account_id.clone(),
                                currency: student_ledger_account.currency.clone(),
                                exchange_rate: Some(1.0),
                                base_currency_amount: Some(original_amount),
                                source_type: "student_fee".to_string(),
                                source_id: Some(created_fee.id),
                                source_module: Some("finance".to_string()),
                                description: format!("Auto-tuition fee assignment for student {}", student.full_name),
                                description_ar: Some(format!("قيد إثبات رسوم دراسية تلقائي للطالب {}", student.full_name)),
                                metadata: None,
                                is_reversed: 0,
                                reversed_by: None,
                                reversal_reason: None,
                                is_reconciled: 0,
                                reconciled_at: None,
                                reconciliation_ref: None,
                                status: "posted".to_string(),
                                posted_by: Some("system".to_string()),
                                posted_at: Some(Utc::now().to_rfc3339()),
                                approved_by: Some("system".to_string()),
                                approved_at: Some(Utc::now().to_rfc3339()),
                                created_by: Some("system".to_string()),
                                created_at: Utc::now().to_rfc3339(),
                            };
                            let _ = self.repo.create_journal_entry(je).await;

                            // Update cached ledger balances
                            let _ = self.repo.update_ledger_account_balance(&student_ledger_account.id, original_amount).await;
                            let _ = self.repo.update_ledger_account_balance(&tpl.revenue_account_id, original_amount).await;
                        }
                    }
                }
            }
        }

        // 3. Create document placeholders for enrollment requirements
        let default_docs = vec![
            ("Birth Certificate", "شهادة الميلاد"),
            ("National ID or Passport", "الهوية الوطنية أو جواز السفر"),
            ("Previous Academic Record", "الشهادة الدراسية السابقة"),
            ("Medical Record", "الملف الصحي"),
        ];

        for (en_name, ar_name) in default_docs {
            let req_doc = crate::service::document::CreateDocumentRequest {
                name: en_name.to_string(),
                title: Some(ar_name.to_string()),
                description: Some(format!("Required document placeholder for student: {}", student.full_name)),
                item_type: Some("placeholder".to_string()),
                visibility: Some("private".to_string()),
                storage_provider: Some("local".to_string()),
                key: Some(String::new()),
                url: None,
                mime_type: None,
                extension: None,
                size: None,
                parent_id: None,
                is_template: Some(0),
                tags: Some("enrollment_requirement".to_string()),
            };
            let _ = self.create_document(org_id, &student.id, Some("system"), req_doc).await;
        }

        // Send enrollment notification
        let _ = self.send_notification(
            &req.student_id,
            org_id,
            "Enrollment Confirmed",
            "تأكيد التسجيل",
            &format!("Student {} has been enrolled successfully.", student.full_name),
            &format!("تم تسجيل الطالب {} بنجاح.", student.full_name),
            "low",
            "enrollment",
        ).await;

        Ok(created_enrollment)
    }

    pub async fn withdraw_enrollment(&self, enrollment_id: &str, reason: &str) -> Result<enrollment::Model, String> {
        let updates = EnrollmentUpdate {
            status: Some("withdrawn".to_string()),
            withdrawal_date: Some(Utc::now().format("%Y-%m-%d").to_string()),
            withdrawal_reason: Some(reason.to_string()),
            ..Default::default()
        };

        self.repo.update_enrollment(enrollment_id, updates).await
            .map_err(|e| e.to_string())
    }

    pub async fn transfer_enrollment(&self, enrollment_id: &str, new_class_id: &str) -> Result<enrollment::Model, String> {
        let updates = EnrollmentUpdate {
            class_id: Some(new_class_id.to_string()),
            ..Default::default()
        };

        self.repo.update_enrollment(enrollment_id, updates).await
            .map_err(|e| e.to_string())
    }

    pub async fn promote_students(&self, org_id: &str, req: PromoteRequest) -> Result<PromotionResult, String> {
        let passing_score = req.passing_score.unwrap_or(50);
        let min_attendance = req.min_attendance_rate.unwrap_or(75.0);

        // Get all active enrollments for the class
        let enrollments = self.repo.get_enrollments_by_class(&req.class_id).await
            .map_err(|e| e.to_string())?;

        let mut promoted = 0usize;
        let mut retained = 0usize;
        let mut details = Vec::new();

        for enrollment in &enrollments {
            // Get student info
            let student = self.repo.get_user_by_id(&enrollment.student_id).await
                .map_err(|e| e.to_string())?
                .unwrap_or_else(|| crate::entities::user::Model {
                    id: enrollment.student_id.clone(),
                    org_id: org_id.to_string(),
                    full_name: "Unknown".to_string(),
                    email: None, email_verified: 0, email_verified_at: None,
                    phone: None, phone_verified: 0, phone_verified_at: None,
                    password_hash: None, password_changed_at: None, password_expires_at: None,
                    mfa_enabled: 0, mfa_secret: None, mfa_backup_codes: None,
                    name_ar: None, name_en: None, avatar: None, sex: None,
                    date_of_birth: None, place_of_birth: None, nationality: None,
                    national_id: None, passport_number: None, address: None,
                    city: None, country: None, postal_code: None, emergency_contact: None,
                    blood_type: None, allergies: None, medical_conditions: None,
                    medical_notes: None, social_links: None, bio: None,
                    primary_role: "student".to_string(), status: "active".to_string(),
                    last_login_at: None, last_login_ip: None, login_count: None,
                    failed_logins: None, locked_until: None, prefs: "{}".to_string(),
                    meta: "{}".to_string(), is_deleted: 0, deleted_at: None,
                    deleted_by: None, created_at: String::new(), updated_at: String::new(),
                    created_by: None,
                });

            // Calculate final score from grades
            let grades = self.repo.get_grades_by_enrollment_final(&enrollment.id).await
                .map_err(|e| e.to_string())?;

            let final_score: Option<i32> = if grades.is_empty() {
                enrollment.final_score
            } else {
                let total_weight: f64 = grades.iter().map(|g| g.weight).sum();
                if total_weight > 0.0 {
                    let weighted_sum: f64 = grades.iter()
                        .map(|g| (g.score / g.max_score) * g.weight)
                        .sum();
                    Some(((weighted_sum / total_weight) * 100.0).round() as i32)
                } else {
                    None
                }
            };

            // Calculate attendance rate
            let total_days = self.repo.get_attendance_total(&enrollment.id).await
                .map_err(|e| e.to_string())?;
            let present_days = self.repo.get_attendance_count_by_status(&enrollment.id, "present").await
                .map_err(|e| e.to_string())?;
            let late_days = self.repo.get_attendance_count_by_status(&enrollment.id, "late").await
                .map_err(|e| e.to_string())?;

            let attendance_rate = if total_days > 0 {
                ((present_days + late_days) as f64 / total_days as f64) * 100.0
            } else {
                100.0 // No attendance data = assume full attendance
            };

            // Promotion decision: score >= passing AND attendance >= min
            let score_pass = final_score.map(|s| s >= passing_score).unwrap_or(false);
            let attendance_pass = attendance_rate >= min_attendance;
            let is_promoted = score_pass && attendance_pass;

            let (decision, notes) = if is_promoted {
                promoted += 1;
                ("promoted".to_string(), format!(
                    "Score: {}/100 (pass: {}), Attendance: {:.1}% (min: {:.1}%)",
                    final_score.unwrap_or(0), passing_score, attendance_rate, min_attendance
                ))
            } else {
                retained += 1;
                let mut reasons = Vec::new();
                if !score_pass {
                    reasons.push(format!("Score {} < {}", final_score.unwrap_or(0), passing_score));
                }
                if !attendance_pass {
                    reasons.push(format!("Attendance {:.1}% < {:.1}%", attendance_rate, min_attendance));
                }
                ("retained".to_string(), format!("Retained: {}", reasons.join(", ")))
            };

            // Update enrollment with promotion result
            let _ = self.repo.update_enrollment(&enrollment.id, EnrollmentUpdate {
                final_score,
                is_promoted: Some(if is_promoted { 1 } else { 0 }),
                promotion_decision: Some(decision.clone()),
                promotion_notes: Some(notes.clone()),
                ..Default::default()
            }).await;

            details.push(PromotionDetail {
                enrollment_id: enrollment.id.clone(),
                student_id: enrollment.student_id.clone(),
                student_name: student.full_name,
                final_score,
                attendance_rate,
                decision,
                notes,
            });
        }

        Ok(PromotionResult {
            total_students: enrollments.len(),
            promoted,
            retained,
            details,
        })
    }
}

