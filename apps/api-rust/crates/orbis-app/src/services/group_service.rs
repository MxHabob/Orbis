use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use orbis_domain::entities::group::{Group, GroupDetail, GroupMemberInfo, UserGroupMembership};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::GroupRepository;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroupInput {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkAssignGroupInput {
    pub group_id: Uuid,
    pub user_ids: Vec<Uuid>,
}

pub struct GroupService {
    pub group_repo: Arc<dyn GroupRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl GroupService {
    pub fn new(
        group_repo: Arc<dyn GroupRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            group_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Create a new User Group
    pub async fn create_group(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: CreateGroupInput,
    ) -> Result<GroupDetail, DomainError> {
        self.permission_checker.check(ctx, Permission::GroupWrite).await?;

        let trimmed_name = input.name.trim().to_string();
        if trimmed_name.is_empty() {
            return Err(DomainError::Validation("Group name cannot be empty".into()));
        }

        if self.group_repo.find_by_name(org_id, &trimmed_name).await?.is_some() {
            return Err(DomainError::Conflict(format!("Group '{}' already exists", trimmed_name)));
        }

        let group_id = Uuid::new_v4();
        let now = Utc::now();

        let group = Group {
            id: group_id,
            org_id,
            name: trimmed_name.clone(),
            name_ar: input.name_ar,
            description: input.description,
            is_system: false,
            status: "active".to_string(),
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.group_repo.save(&group).await?;

        if !input.permissions.is_empty() {
            self.group_repo.set_group_permissions(org_id, group_id, &input.permissions).await?;
        }

        let _ = self.event_bus.publish(ctx, &DomainEvent::GroupCreated {
            group_id,
            org_id,
            name: trimmed_name,
            timestamp: now,
        }).await;

        let detail = self.group_repo.find_detail_by_id(group_id).await?
            .ok_or_else(|| DomainError::Internal("Failed to load created group".into()))?;

        Ok(detail)
    }

    /// Update Group details and permissions
    pub async fn update_group(
        &self,
        ctx: &RequestContext,
        group_id: Uuid,
        input: UpdateGroupInput,
    ) -> Result<GroupDetail, DomainError> {
        self.permission_checker.check(ctx, Permission::GroupWrite).await?;

        let mut group = self.group_repo.find_by_id(group_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Group {} not found", group_id)))?;

        let now = Utc::now();

        if let Some(name) = input.name {
            let trimmed = name.trim().to_string();
            if !trimmed.is_empty() {
                group.name = trimmed;
            }
        }
        if let Some(name_ar) = input.name_ar {
            group.name_ar = Some(name_ar);
        }
        if let Some(desc) = input.description {
            group.description = Some(desc);
        }
        if let Some(status) = input.status {
            group.status = status;
        }
        group.updated_at = now;

        self.group_repo.save(&group).await?;

        if let Some(perms) = input.permissions {
            self.group_repo.set_group_permissions(group.org_id, group_id, &perms).await?;
        }

        let _ = self.event_bus.publish(ctx, &DomainEvent::GroupUpdated {
            group_id,
            org_id: group.org_id,
            timestamp: now,
        }).await;

        let detail = self.group_repo.find_detail_by_id(group_id).await?
            .ok_or_else(|| DomainError::Internal("Failed to load updated group".into()))?;

        Ok(detail)
    }

    /// Delete a User Group
    pub async fn delete_group(
        &self,
        ctx: &RequestContext,
        group_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::GroupDelete).await?;

        let group = self.group_repo.find_by_id(group_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Group {} not found", group_id)))?;

        if group.is_system {
            return Err(DomainError::Forbidden("Cannot delete system default group".into()));
        }

        self.group_repo.delete(group_id).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::GroupDeleted {
            group_id,
            org_id: group.org_id,
            timestamp: Utc::now(),
        }).await;

        Ok(())
    }

    /// List all Groups for an organization
    pub async fn list_groups(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Vec<GroupDetail>, DomainError> {
        self.permission_checker.check(ctx, Permission::GroupRead).await?;
        self.group_repo.list_by_org(org_id).await
    }

    /// Seed standard system default user groups for an organization
    pub async fn seed_default_groups(&self, org_id: Uuid) -> Result<Vec<GroupDetail>, DomainError> {
        let defaults = vec![
            (
                "School Administration",
                Some("الإدارة المدرسية".to_string()),
                Some("صلاحيات الإشراف الإداري والأكاديمي الشامل، إدارة الوحدات والسياسات العامة.".to_string()),
                vec![
                    "organization:read", "organization:write", "org_unit:read", "org_unit:write",
                    "user:read", "user:write", "employee:read", "employee:write",
                    "group:read", "group:assign", "student:read", "student:write",
                    "enrollment:read", "period:read", "class:read", "subject:read",
                    "schedule:read", "timetable:read", "grade:read", "grade:analytics",
                    "grade:report", "attendance:read", "attendance:analytics", "attendance:report",
                    "behavior:read", "finance:read", "dashboard:read", "audit:read",
                    "report:read", "report:generate",
                ],
            ),
            (
                "Teachers & Faculty",
                Some("هيئة التدريس والمعلمون".to_string()),
                Some("إدخال ورصد الدرجات والتقييمات، تسجيل الحضور والغياب، ومتابعة الحصص والجداول.".to_string()),
                vec![
                    "class:read", "subject:read", "schedule:read", "timetable:read",
                    "lesson:read", "lesson:write", "student:read", "attendance:read",
                    "attendance:write", "attendance:report", "grade:read", "grade:write",
                    "grade:submit", "grade:report", "assessment:read", "behavior:read",
                    "behavior:write", "document:read",
                ],
            ),
            (
                "Student Affairs & Admissions",
                Some("شؤون الطلاب والقبول".to_string()),
                Some("إدارة ملفات الطلاب وأولياء الأمور، التسجيل، السجلات الأكاديمية والوثائق الرسمية.".to_string()),
                vec![
                    "student:read", "student:write", "student:create", "student:update", "student:delete",
                    "enrollment:read", "enrollment:write", "enrollment:approve", "enrollment:transfer", "enrollment:promote",
                    "guardian:read", "guardian:write", "guardian:delete",
                    "document:read", "document:write", "document:verify",
                    "attendance:read", "attendance:write", "behavior:read", "behavior:write",
                    "class:read", "period:read", "report:read", "report:generate",
                ],
            ),
            (
                "Human Resources",
                Some("الموارد البشرية وشؤون الموظفين".to_string()),
                Some("إدارة السجلات الوظيفية للكادر التعليمي والإداري، العقود، والصلاحيات والمجموعات.".to_string()),
                vec![
                    "employee:read", "employee:write", "employee:create", "employee:update", "employee:delete",
                    "user:read", "user:password_write", "group:read", "group:assign",
                    "org_unit:read", "document:read", "document:write", "audit:read",
                    "report:read", "report:generate",
                ],
            ),
            (
                "Finance & Accounting",
                Some("الشؤون المالية والحسابات".to_string()),
                Some("إدارة الرسوم الدراسية، الفواتير، سندات القبض والصرف، الخصومات والتقارير المالية.".to_string()),
                vec![
                    "finance:read", "finance:write", "finance:manage", "account:read", "account:write",
                    "invoice:read", "invoice:write", "payment:read", "payment:write", "payment:refund",
                    "fee:read", "fee:write", "fee:split", "discount:read", "discount:write",
                    "student:read", "enrollment:read", "report:read", "report:generate", "audit:read",
                ],
            ),
            (
                "Exam Control Board",
                Some("الكنترول ولجنة الامتحانات".to_string()),
                Some("إدارة الاختبارات والتقييمات الفصلية، توزيع اللجان، مراجعة واعتماد وتثبيت النتائج النهائية.".to_string()),
                vec![
                    "exam:read", "exam:write", "exam:manage", "exam_session:read", "exam_session:write",
                    "assessment:read", "assessment:write", "assessment:publish",
                    "grade:read", "grade:write", "grade:submit", "grade:approve", "grade:lock", "grade:report",
                    "period:read", "class:read", "subject:read", "student:read", "report:generate",
                ],
            ),
            (
                "Students",
                Some("مجموعة الطلاب".to_string()),
                Some("بوابة الطالب لاستعراض الجداول الدراسية، الحضور، التقييمات، والنتائج الأكاديمية.".to_string()),
                vec![
                    "student:read_self", "grade:read_self", "attendance:read_self",
                    "schedule:read_self", "timetable:read_self", "lesson:read_self",
                    "invoice:read_self", "document:read_self", "assessment:read_self",
                ],
            ),
            (
                "Parents & Guardians",
                Some("مجموعة أولياء الأمور".to_string()),
                Some("بوابة ولي الأمر لمتابعة التحصيل الدراسي للأبناء، الحضور، الرسوم، والتقارير الدورية.".to_string()),
                vec![
                    "student:read_children", "grade:read_children", "attendance:read_children",
                    "schedule:read_children", "timetable:read_children", "invoice:read_children",
                    "payment:read_children", "document:read_children", "behavior:read_children",
                ],
            ),
            (
                "Student Guidance & Counseling",
                Some("التوجيه والإرشاد الطلابي".to_string()),
                Some("متابعة السلوك والانضباط المدرسي، الرعاية الطلابية، والتواصل مع أولياء الأمور.".to_string()),
                vec![
                    "student:read", "attendance:read", "behavior:read", "behavior:write",
                    "behavior:delete", "grade:read", "guardian:read", "document:read",
                    "report:read", "report:generate",
                ],
            ),
        ];

        let now = Utc::now();
        for (name, name_ar, desc, perms) in defaults {
            if self.group_repo.find_by_name(org_id, name).await?.is_none() {
                let group_id = Uuid::new_v4();
                let grp = Group {
                    id: group_id,
                    org_id,
                    name: name.to_string(),
                    name_ar,
                    description: desc,
                    is_system: true,
                    status: "active".to_string(),
                    created_by: None,
                    updated_by: None,
                    created_at: now,
                    updated_at: now,
                };
                let _ = self.group_repo.save(&grp).await;
                let perm_strings: Vec<String> = perms.into_iter().map(|s| s.to_string()).collect();
                let _ = self.group_repo.set_group_permissions(org_id, group_id, &perm_strings).await;
            }
        }

        self.group_repo.list_by_org(org_id).await
    }

    /// Get Group detail by ID
    pub async fn get_group(
        &self,
        ctx: &RequestContext,
        group_id: Uuid,
    ) -> Result<Option<GroupDetail>, DomainError> {
        self.permission_checker.check(ctx, Permission::GroupRead).await?;
        self.group_repo.find_detail_by_id(group_id).await
    }

    /// Get members of a Group
    pub async fn get_group_members(
        &self,
        ctx: &RequestContext,
        group_id: Uuid,
    ) -> Result<Vec<GroupMemberInfo>, DomainError> {
        self.permission_checker.check(ctx, Permission::GroupRead).await?;
        self.group_repo.get_members(group_id).await
    }

    /// Add a user to a Group
    pub async fn add_member(
        &self,
        ctx: &RequestContext,
        group_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::GroupAssign).await?;

        let group = self.group_repo.find_by_id(group_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Group {} not found", group_id)))?;

        let now = Utc::now();
        let membership = UserGroupMembership {
            id: Uuid::new_v4(),
            org_id: group.org_id,
            user_id,
            group_id,
            created_at: now,
        };

        self.group_repo.add_member(&membership).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::UserGroupAssigned {
            user_id,
            group_id,
            org_id: group.org_id,
            timestamp: now,
        }).await;

        Ok(())
    }

    /// Remove a user from a Group
    pub async fn remove_member(
        &self,
        ctx: &RequestContext,
        group_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::GroupAssign).await?;

        let group = self.group_repo.find_by_id(group_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Group {} not found", group_id)))?;

        self.group_repo.remove_member(user_id, group_id).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::UserGroupRemoved {
            user_id,
            group_id,
            org_id: group.org_id,
            timestamp: Utc::now(),
        }).await;

        Ok(())
    }

    /// Bulk assign multiple users to a Group (10, 50, 500+ users)
    pub async fn bulk_assign_group(
        &self,
        ctx: &RequestContext,
        input: BulkAssignGroupInput,
    ) -> Result<u64, DomainError> {
        self.permission_checker.check(ctx, Permission::GroupAssign).await?;

        let group = self.group_repo.find_by_id(input.group_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Group {} not found", input.group_id)))?;

        let now = Utc::now();
        let memberships: Vec<UserGroupMembership> = input.user_ids.into_iter().map(|uid| {
            UserGroupMembership {
                id: Uuid::new_v4(),
                org_id: group.org_id,
                user_id: uid,
                group_id: group.id,
                created_at: now,
            }
        }).collect();

        let assigned_count = self.group_repo.bulk_add_members(&memberships).await?;
        Ok(assigned_count)
    }

    /// Set permissions for a Group
    pub async fn set_permissions(
        &self,
        ctx: &RequestContext,
        group_id: Uuid,
        permissions: Vec<String>,
    ) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::GroupWrite).await?;

        let group = self.group_repo.find_by_id(group_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Group {} not found", group_id)))?;

        self.group_repo.set_group_permissions(group.org_id, group_id, &permissions).await?;
        Ok(())
    }
}
