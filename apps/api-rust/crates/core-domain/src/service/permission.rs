use crate::entities::{permission, role_permission};
use std::collections::HashMap;

impl super::Service {
    pub async fn get_permissions(&self) -> Result<Vec<permission::Model>, String> {
        let mut db_perms = self.repo.get_all_permissions().await.map_err(|e| e.to_string())?;

        // Standard system permission codes we expect in the database
        let standard_codes = vec![
            "OrganizationRead", "OrganizationWrite",
            "UserRead", "UserWrite", "UserDelete", "UserRoleWrite", "UserPasswordWrite",
            "CycleRead", "CycleWrite", "CycleActivate", "CycleLock", "CycleUnlock", "GradeEntryManage",
            "OrgUnitRead", "OrgUnitWrite", "OrgUnitDelete",
            "ClassRead", "ClassWrite", "ClassDelete",
            "SubjectRead", "SubjectWrite", "SubjectDelete",
            "ScheduleRead", "ScheduleWrite", "ScheduleDelete",
            "AssessmentRead", "AssessmentWrite", "AssessmentDelete",
            "GuardianRead", "GuardianWrite", "GuardianDelete",
            "DocumentRead", "DocumentWrite", "DocumentDelete",
            "EnrollmentRead", "EnrollmentWrite", "EnrollmentPromote", "EnrollmentTransfer",
            "GradeRead", "GradeWrite", "GradeAnalytics", "GradeReport",
            "AttendanceRead", "AttendanceWrite", "AttendanceAnalytics", "AttendanceReport",
            "BehaviorRead", "BehaviorWrite", "BehaviorDelete", "BehaviorAI",
            "NotificationRead", "NotificationWrite",
            "DashboardRead",
            "AuditRead",
            "FinanceRead", "FinanceWrite",
            "ImportData", "ExportData",
            "ExamSessionRead", "ExamSessionWrite", "ExamSessionActivate", "ExamSessionDelete",
            "SeatingNumberGenerate", "SeatingNumberRead",
            "SecretNumberGenerate", "SecretNumberRead", "SecretNumberReveal",
            "ExamHallRead", "ExamHallWrite", "ExamHallAllocate",
            "ProctorRead", "ProctorAssign",
            "StudentOrderRead", "StudentOrderRebuild", "StudentOrderFreeze",
            "ExamScheduleRead", "ExamScheduleWrite"
        ];

        let mut next_id = db_perms.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        let mut missing_perms = Vec::new();
        let now = chrono::Utc::now().naive_utc().to_string();

        for code in standard_codes {
            if !db_perms.iter().any(|p| p.code == code) {
                // Determine module/action from code
                let module = match code {
                    c if c.contains("Organization") => "organization",
                    c if c.contains("User") => "users",
                    c if c.contains("Cycle") || c.contains("GradeEntry") => "academics",
                    c if c.contains("OrgUnit") => "organization_units",
                    c if c.contains("Class") => "classes",
                    c if c.contains("Subject") => "subjects",
                    c if c.contains("Schedule") => "schedule",
                    c if c.contains("Assessment") => "assessments",
                    c if c.contains("Guardian") => "guardians",
                    c if c.contains("Document") => "documents",
                    c if c.contains("Enrollment") => "enrollments",
                    c if c.contains("Grade") => "grades",
                    c if c.contains("Attendance") => "attendance",
                    c if c.contains("Behavior") => "behavior",
                    c if c.contains("Notification") => "notifications",
                    c if c.contains("Dashboard") => "dashboard",
                    c if c.contains("Audit") => "audit",
                    c if c.contains("Finance") => "finance",
                    c if c.contains("Exam") || c.contains("Seating") || c.contains("Secret") || c.contains("Proctor") || c.contains("StudentOrder") => "exams",
                    _ => "system"
                };

                let action = match code {
                    c if c.contains("Read") || c.contains("View") => "read",
                    c if c.contains("Write") || c.contains("Create") || c.contains("Manage") || c.contains("Submit") || c.contains("Record") || c.contains("Assign") => "write",
                    c if c.contains("Delete") || c.contains("Withdraw") => "delete",
                    _ => "execute"
                };

                let new_id = next_id;
                next_id += 1;

                missing_perms.push(sea_orm::ActiveModelTrait::insert(
                    crate::entities::permission::ActiveModel {
                        id: sea_orm::ActiveValue::Set(new_id),
                        code: sea_orm::ActiveValue::Set(code.to_string()),
                        name: sea_orm::ActiveValue::Set(code.to_string()),
                        name_ar: sea_orm::ActiveValue::Set(Some(code.to_string())),
                        description: sea_orm::ActiveValue::Set(Some(code.to_string())),
                        module: sea_orm::ActiveValue::Set(module.to_string()),
                        action: sea_orm::ActiveValue::Set(action.to_string()),
                        resource: sea_orm::ActiveValue::Set("feature".to_string()),
                        requires_scope: sea_orm::ActiveValue::Set(0),
                        scope_types: sea_orm::ActiveValue::Set(Some("[\"global\"]".to_string())),
                        is_system: sea_orm::ActiveValue::Set(1),
                        is_active: sea_orm::ActiveValue::Set(1),
                        created_at: sea_orm::ActiveValue::Set(now.clone()),
                    },
                    &self.repo.db
                ));
            }
        }

        if !missing_perms.is_empty() {
            for fut in missing_perms {
                let _ = fut.await;
            }
            // Re-fetch all permissions from the database
            db_perms = self.repo.get_all_permissions().await.map_err(|e| e.to_string())?;
        }

        Ok(db_perms)
    }

    pub async fn get_role_permissions(&self) -> Result<Vec<role_permission::Model>, String> {
        self.repo.get_role_permissions().await.map_err(|e| e.to_string())
    }

    pub async fn save_role_permissions(
        &self,
        role: &str,
        permissions_map: HashMap<String, String>,
    ) -> Result<(), String> {
        // Fetch all permissions to map code string to its numeric permission ID
        let all_perms = self.get_permissions().await?;
        
        let mut resolved_permissions = Vec::new();
        for (perm_code, scope) in permissions_map {
            if let Some(perm) = all_perms.iter().find(|p| p.code == perm_code) {
                resolved_permissions.push((perm.id, scope));
            }
        }

        self.repo
            .save_role_permissions(role, resolved_permissions)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_user_permissions(&self, user_id: &str) -> Result<HashMap<String, String>, String> {
        let perms = self.get_permissions().await?;
        let user_perms = self.repo.get_user_permissions(user_id).await.map_err(|e| e.to_string())?;

        let id_to_code: HashMap<i32, String> = perms.into_iter().map(|p| (p.id, p.code)).collect();
        let mut result = HashMap::new();
        for up in user_perms {
            if let Some(code) = id_to_code.get(&up.permission_id) {
                result.insert(code.clone(), up.grant_type);
            }
        }
        Ok(result)
    }

    pub async fn save_user_permissions(
        &self,
        user_id: &str,
        permissions_map: HashMap<String, String>,
    ) -> Result<(), String> {
        let all_perms = self.get_permissions().await?;
        let mut resolved_permissions = Vec::new();
        for (perm_code, grant_type) in permissions_map {
            if let Some(perm) = all_perms.iter().find(|p| p.code == perm_code) {
                resolved_permissions.push((perm.id, grant_type));
            }
        }

        self.repo
            .save_user_permissions(user_id, resolved_permissions)
            .await
            .map_err(|e| e.to_string())
    }
}
