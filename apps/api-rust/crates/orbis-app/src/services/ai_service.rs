//! AI Orchestration Service — Stateless ReAct Engine & Direct Domain Integration.
//! Integrates directly with existing Domain Services (Student, Academic, Assessment, Finance, Scheduling, Reports, Org)
//! with strict permission enforcement, dynamic identity resolution, and academic scoping.

use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::gateways::LlmGatewayPort;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::EmployeeRepository;
use crate::services::{
    academic_service::AcademicService,
    assessment_service::AssessmentService,
    autopilot_service::AutopilotService,
    finance_service::FinanceService,
    organization_service::OrganizationService,
    report_service::{GenerateReportInput, ReportService},
    scheduling_service::SchedulingService,
    student_service::StudentService,
};
use orbis_domain::entities::ai::{
    AcademicAiContext, AiChatRequest, AiChatResponse, AiMessage, AiProposedAction,
    AiToolDefinition, AiToolResult, ToolCategory, UiContext, UserAiContext,
};
use orbis_domain::entities::report::{OutputFormat, ParamValue, ReportLocale};
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{
    AttendanceFilter, ClassSubjectFilter, GradeFilter, PaginationParams, StudentFeeFilter,
    StudentFilter,
};

fn take_private_ui_result(result: &mut serde_json::Value) -> Option<serde_json::Value> {
    result
        .as_object_mut()
        .and_then(|object| object.remove("_ui_private_result"))
}

fn required_tool_permission(tool: &AiToolDefinition) -> Result<Permission, DomainError> {
    tool.required_permission
        .as_deref()
        .and_then(Permission::from_str)
        .ok_or_else(|| {
            DomainError::Forbidden(format!(
                "AI tool '{}' has no recognized required permission",
                tool.name
            ))
        })
}

async fn check_tool_permission(
    checker: &dyn PermissionCheckerPort,
    ctx: &RequestContext,
    tool: &AiToolDefinition,
) -> Result<(), DomainError> {
    checker.check(ctx, required_tool_permission(tool)?).await
}

fn ensure_tenant_matches(record_org_id: Uuid, tenant_id: Uuid) -> Result<(), DomainError> {
    if record_org_id == tenant_id {
        Ok(())
    } else {
        Err(DomainError::NotFound("Record not found".into()))
    }
}

fn teacher_assigned_to_class(
    teacher_id: Uuid,
    class_id: Uuid,
    homeroom_teacher_id: Option<Uuid>,
    assignments: &[orbis_domain::entities::class_subject::ClassSubject],
) -> bool {
    homeroom_teacher_id == Some(teacher_id)
        || assignments.iter().any(|assignment| {
            assignment.class_id == class_id
                && (assignment.primary_teacher_id == teacher_id
                    || assignment.assistant_teacher_id == Some(teacher_id)
                    || assignment.substitute_teacher_id == Some(teacher_id))
        })
}

fn unit_in_scope(unit_id: Option<Uuid>, allowed_units: Option<&[Uuid]>) -> bool {
    match allowed_units {
        None => true,
        Some(allowed) => unit_id.is_some_and(|id| allowed.contains(&id)),
    }
}

fn tool_supports_unit_scoping(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "search_students"
            | "get_student_profile"
            | "get_student_grades"
            | "get_student_attendance"
            | "get_student_fees"
            | "analyze_at_risk_students"
            | "analyze_class_performance"
            | "draft_parent_notice"
    )
}

pub struct AiOrchestrationService {
    pub llm_gateway: Arc<dyn LlmGatewayPort>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
    pub student_service: Arc<StudentService>,
    pub employee_repo: Arc<dyn EmployeeRepository>,
    pub academic_service: Arc<AcademicService>,
    pub assessment_service: Arc<AssessmentService>,
    pub finance_service: Arc<FinanceService>,
    pub scheduling_service: Arc<SchedulingService>,
    pub report_service: Arc<ReportService>,
    pub org_service: Arc<OrganizationService>,
    pub autopilot_service: Option<Arc<AutopilotService>>,
}

impl AiOrchestrationService {
    async fn ensure_student_tenant(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<(), DomainError> {
        let student = self
            .student_service
            .get_student(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
        ensure_tenant_matches(student.student.org_id, ctx.tenant_id)?;
        let allowed_units = self.user_org_unit_allowlist(ctx).await?;
        if !unit_in_scope(student.student.org_unit_id, allowed_units.as_deref()) {
            return Err(DomainError::NotFound("Student not found".into()));
        }

        let Some(employee) = self.teacher_employee(ctx).await? else {
            return Ok(());
        };

        let enrollments = self
            .student_service
            .enrollment_repo
            .find_by_student(student.student.id)
            .await?;
        for enrollment in enrollments
            .into_iter()
            .filter(|enrollment| enrollment.org_id == ctx.tenant_id)
        {
            let Some(class_id) = enrollment.class_id else {
                continue;
            };
            let class = self
                .academic_service
                .class_repo
                .find_by_id(class_id)
                .await?;
            let Some(class) = class.filter(|class| class.org_id == ctx.tenant_id) else {
                continue;
            };
            let assignments = self
                .class_subjects_for_teacher(employee.id, enrollment.period_id)
                .await?;
            if teacher_assigned_to_class(
                employee.id,
                class_id,
                class.homeroom_teacher_id,
                &assignments,
            ) {
                return Ok(());
            }
        }
        Err(DomainError::NotFound("Student not found".into()))
    }

    async fn teacher_employee(
        &self,
        ctx: &RequestContext,
    ) -> Result<Option<orbis_domain::entities::employee::Employee>, DomainError> {
        let Some(user_id) = ctx.user_id() else {
            return Ok(None);
        };
        let Some(employee) = self
            .employee_repo
            .find_by_user_id(ctx.tenant_id, user_id)
            .await?
        else {
            return Ok(None);
        };
        Ok(employee.is_teacher().then_some(employee))
    }

    async fn class_subjects_for_teacher(
        &self,
        teacher_id: Uuid,
        period_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::class_subject::ClassSubject>, DomainError> {
        self.academic_service
            .class_subject_repo
            .find_by_teacher(teacher_id, period_id)
            .await
    }

    async fn teaching_assignments(
        &self,
        ctx: &RequestContext,
        teacher_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::ai::TeachingAssignmentInfo>, DomainError> {
        let assignments = self
            .academic_service
            .class_subject_repo
            .list(
                ctx.tenant_id,
                ClassSubjectFilter {
                    teacher_id: Some(teacher_id),
                    pagination: Some(PaginationParams {
                        page: Some(1),
                        page_size: Some(PaginationParams::MAX_PAGE_SIZE),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;
        let mut result = Vec::new();
        for assignment in assignments.items {
            let Some(class) = self
                .academic_service
                .class_repo
                .find_by_id(assignment.class_id)
                .await?
            else {
                continue;
            };
            if class.org_id != ctx.tenant_id {
                continue;
            }
            let Some(subject) = self
                .academic_service
                .subject_repo
                .find_by_id(assignment.subject_id)
                .await?
            else {
                continue;
            };
            if subject.org_id != ctx.tenant_id {
                continue;
            }
            result.push(orbis_domain::entities::ai::TeachingAssignmentInfo {
                class_subject_id: assignment.id,
                class_id: assignment.class_id,
                class_name: class.name,
                subject_id: assignment.subject_id,
                subject_name: subject.name,
                period_id: Some(assignment.period_id),
            });
        }
        Ok(result)
    }

    async fn ensure_class_tenant(
        &self,
        ctx: &RequestContext,
        class_id: Uuid,
    ) -> Result<(), DomainError> {
        let class = self
            .academic_service
            .class_repo
            .find_by_id(class_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Class not found".into()))?;
        ensure_tenant_matches(class.org_id, ctx.tenant_id)?;
        let allowed_units = self.user_org_unit_allowlist(ctx).await?;
        if !unit_in_scope(class.unit_id.into(), allowed_units.as_deref()) {
            return Err(DomainError::NotFound("Class not found".into()));
        }
        Ok(())
    }

    async fn user_org_unit_allowlist(
        &self,
        ctx: &RequestContext,
    ) -> Result<Option<Vec<Uuid>>, DomainError> {
        let Some(user_id) = ctx.user_id() else {
            return Ok(None);
        };
        let Some(scoped_units) = self
            .permission_checker
            .get_org_unit_scope(user_id, ctx.tenant_id)
            .await?
        else {
            return Ok(None);
        };
        let mut allowed = Vec::new();
        for unit_id in scoped_units {
            let Some(unit) = self.org_service.org_unit_repo.find_by_id(unit_id).await? else {
                continue;
            };
            if unit.org_id != ctx.tenant_id {
                continue;
            }
            allowed.push(unit.id);
            let descendants = self
                .org_service
                .org_unit_repo
                .find_descendants(ctx.tenant_id, &unit.path)
                .await?;
            allowed.extend(
                descendants
                    .into_iter()
                    .filter(|descendant| descendant.org_id == ctx.tenant_id)
                    .map(|descendant| descendant.id),
            );
        }
        allowed.sort();
        allowed.dedup();
        Ok(Some(allowed))
    }

    async fn ensure_class_access(
        &self,
        ctx: &RequestContext,
        class_id: Uuid,
        period_id: Option<Uuid>,
    ) -> Result<(), DomainError> {
        self.ensure_class_tenant(ctx, class_id).await?;
        let Some(employee) = self.teacher_employee(ctx).await? else {
            return Ok(());
        };

        let assignments = if let Some(period_id) = period_id {
            self.class_subjects_for_teacher(employee.id, period_id)
                .await?
        } else {
            self.academic_service
                .class_subject_repo
                .find_by_class(class_id)
                .await?
                .into_iter()
                .filter(|assignment| {
                    assignment.primary_teacher_id == employee.id
                        || assignment.assistant_teacher_id == Some(employee.id)
                        || assignment.substitute_teacher_id == Some(employee.id)
                })
                .collect()
        };
        let class = self
            .academic_service
            .class_repo
            .find_by_id(class_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Class not found".into()))?;
        if teacher_assigned_to_class(
            employee.id,
            class_id,
            class.homeroom_teacher_id,
            &assignments,
        ) {
            Ok(())
        } else {
            Err(DomainError::NotFound("Class not found".into()))
        }
    }

    async fn ensure_class_subject_access(
        &self,
        ctx: &RequestContext,
        class_subject_id: Uuid,
    ) -> Result<orbis_domain::entities::class_subject::ClassSubject, DomainError> {
        let assignment = self
            .academic_service
            .class_subject_repo
            .find_by_id(class_subject_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Class subject not found".into()))?;
        self.ensure_class_access(ctx, assignment.class_id, Some(assignment.period_id))
            .await?;
        let subject = self
            .academic_service
            .subject_repo
            .find_by_id(assignment.subject_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Subject not found".into()))?;
        ensure_tenant_matches(subject.org_id, ctx.tenant_id)?;
        Ok(assignment)
    }

    pub fn new(
        llm_gateway: Arc<dyn LlmGatewayPort>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
        student_service: Arc<StudentService>,
        employee_repo: Arc<dyn EmployeeRepository>,
        academic_service: Arc<AcademicService>,
        assessment_service: Arc<AssessmentService>,
        finance_service: Arc<FinanceService>,
        scheduling_service: Arc<SchedulingService>,
        report_service: Arc<ReportService>,
        org_service: Arc<OrganizationService>,
    ) -> Self {
        Self {
            llm_gateway,
            permission_checker,
            student_service,
            employee_repo,
            academic_service,
            assessment_service,
            finance_service,
            scheduling_service,
            report_service,
            org_service,
            autopilot_service: None,
        }
    }

    pub fn with_autopilot_service(mut self, autopilot_service: Arc<AutopilotService>) -> Self {
        self.autopilot_service = Some(autopilot_service);
        self
    }

    pub async fn get_authorized_tools(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<AiToolDefinition>, DomainError> {
        let has_unit_scope = self.user_org_unit_allowlist(ctx).await?.is_some();
        let mut authorized = Vec::new();
        for tool in Self::registered_tools() {
            if has_unit_scope && !tool_supports_unit_scoping(&tool.name) {
                continue;
            }
            match check_tool_permission(self.permission_checker.as_ref(), ctx, &tool).await {
                Ok(()) => {
                    if tool.name == "generate_report" {
                        match self
                            .permission_checker
                            .check(ctx, Permission::ExportData)
                            .await
                        {
                            Ok(()) => {}
                            Err(DomainError::Unauthorized(_)) => continue,
                            Err(error) => return Err(error),
                        }
                    }
                    authorized.push(tool)
                }
                Err(DomainError::Unauthorized(_)) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(authorized)
    }

    async fn authorize_tool(
        &self,
        ctx: &RequestContext,
        tool_name: &str,
    ) -> Result<(), DomainError> {
        let tool = Self::registered_tools()
            .into_iter()
            .find(|tool| tool.name == tool_name)
            .ok_or_else(|| DomainError::Forbidden("Unknown AI tool".into()))?;
        if self.user_org_unit_allowlist(ctx).await?.is_some()
            && !tool_supports_unit_scoping(tool_name)
        {
            return Err(DomainError::Forbidden(
                "AI tool is unavailable within the user's organizational-unit scope".into(),
            ));
        }
        check_tool_permission(self.permission_checker.as_ref(), ctx, &tool).await?;
        if tool_name == "generate_report" {
            self.permission_checker
                .check(ctx, Permission::ExportData)
                .await?;
        }
        Ok(())
    }

    fn registered_tools() -> Vec<AiToolDefinition> {
        vec![
            AiToolDefinition {
                name: "search_students".into(),
                description: "Search students by name, national ID, student number, or filter by class/status in the organization.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "search_term": {"type": "string", "description": "Search query for name, student number, or national ID"},
                        "class_id": {"type": "string", "description": "Optional UUID of the class to scope search"},
                        "period_id": {"type": "string", "description": "Optional UUID of the academic period"}
                    }
                }),
                required_permission: Some("student:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "get_student_profile".into(),
                description: "Get full profile details for a specific student including academic standing and enrollments.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"}
                    },
                    "required": ["student_id"]
                }),
                required_permission: Some("student:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "get_student_grades".into(),
                description: "Get grades and assessment scores recorded for a student, optionally filtered by academic period.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"},
                        "period_id": {"type": "string", "description": "Optional UUID of the academic period to filter grades"}
                    },
                    "required": ["student_id"]
                }),
                required_permission: Some("grade:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "get_student_attendance".into(),
                description: "Get attendance history and absence records for a student, optionally filtered by date or status.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"},
                        "class_id": {"type": "string", "description": "Optional UUID of the class to filter attendance"},
                        "status": {"type": "string", "description": "Optional status filter (e.g. absent, late, excused)"}
                    },
                    "required": ["student_id"]
                }),
                required_permission: Some("attendance:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "get_financial_summary".into(),
                description: "Get organizational financial summary including fee collection, accounts, and payments.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
                required_permission: Some("finance:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "get_student_fees".into(),
                description: "Get fee obligations, balances, and payment records for a specific student.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"}
                    },
                    "required": ["student_id"]
                }),
                required_permission: Some("finance:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "get_unpaid_fees".into(),
                description: "Find students with unpaid fee balances. Results are shown securely in the application; large result sets are exported as a background XLSX report.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "period_id": {"type": "string", "description": "Optional academic period UUID"}
                    }
                }),
                required_permission: Some("finance:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "list_report_templates".into(),
                description: "List available report and document templates in the system.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
                required_permission: Some("template:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "generate_report".into(),
                description: "Generate a formal PDF or Excel report using DocForge reporting engine.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "report_code": {"type": "string", "description": "Code of the report definition (e.g. STUDENT_TRANSCRIPT, FEE_COLLECTION)"},
                        "output_format": {"type": "string", "enum": ["pdf", "xlsx", "csv", "json", "html"], "description": "Format for generated output"},
                        "parameters": {"type": "object", "description": "Parameters required by the report definition"}
                    },
                    "required": ["report_code", "output_format"]
                }),
                required_permission: Some("template:generate_single".into()),
                is_consequential: false,
                category: ToolCategory::Generation,
            },
            AiToolDefinition {
                name: "get_org_overview".into(),
                description: "Get organization metadata, branch list, and high-level platform statistics.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
                required_permission: Some("organization:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "update_student_status".into(),
                description: "Propose changing the status of a student (e.g. Active, Suspended, Graduated, Withdrawn). Requires user confirmation.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"},
                        "status": {"type": "string", "enum": ["active", "suspended", "graduated", "withdrawn", "expelled", "transferred_out"], "description": "Target status"},
                        "reason": {"type": "string", "description": "Reason for status change"}
                    },
                    "required": ["student_id", "status"]
                }),
                required_permission: Some("student:update".into()),
                is_consequential: true,
                category: ToolCategory::Action,
            },
            AiToolDefinition {
                name: "get_autopilot_status".into(),
                description: "Get the current operating mode, status, and health diagnostics of the Orbis Autopilot engine.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
                required_permission: Some("organization:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "list_autopilot_tasks".into(),
                description: "List scheduled, executing, completed, failed, and suggested Autopilot tasks with their execution reasons and error logs.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "status": {"type": "string", "enum": ["pending", "executing", "completed", "failed", "suggested", "cancelled"], "description": "Filter by task status"}
                    }
                }),
                required_permission: Some("organization:read".into()),
                is_consequential: false,
                category: ToolCategory::Read,
            },
            AiToolDefinition {
                name: "trigger_autopilot_tick".into(),
                description: "Trigger an authorized Autopilot policy evaluation tick (dry-run or live execution). Requires user confirmation.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "dry_run": {"type": "boolean", "description": "If true, simulates execution without making state changes"}
                    }
                }),
                required_permission: Some("organization:write".into()),
                is_consequential: true,
                category: ToolCategory::Action,
            },
            // ── Advanced Real-World Educational AI Tools ──────────
            AiToolDefinition {
                name: "analyze_at_risk_students".into(),
                description: "Execute Early Warning Analytics across attendance, grades, and financial records to identify at-risk students and recommend pedagogical interventions.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "class_id": {"type": "string", "description": "Optional UUID of the class to scope evaluation"},
                        "period_id": {"type": "string", "description": "Optional UUID of the academic period"}
                    }
                }),
                required_permission: Some("student:read".into()),
                is_consequential: false,
                category: ToolCategory::Analysis,
            },
            AiToolDefinition {
                name: "generate_exam_paper".into(),
                description: "Generate a complete, pedagogical, multi-section exam paper with questions, Bloom taxonomy levels, points, and model answer keys.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "subject_name": {"type": "string", "description": "Name of the subject (e.g. Mathematics, Science, Arabic)"},
                        "grade_level": {"type": "string", "description": "Target educational grade level (e.g. Grade 8, Middle 2)"},
                        "topics": {"type": "string", "description": "Curriculum topics or unit description to cover"},
                        "language": {"type": "string", "enum": ["ar", "en", "bilingual"], "description": "Output language"},
                        "difficulty": {"type": "string", "enum": ["easy", "medium", "hard", "balanced"], "description": "Difficulty balance"},
                        "total_marks": {"type": "integer", "description": "Total marks for the exam (e.g. 100)"},
                        "duration_minutes": {"type": "integer", "description": "Duration in minutes (e.g. 60, 90)"},
                        "question_count": {"type": "integer", "description": "Number of questions to generate (default 8)"}
                    },
                    "required": ["subject_name", "topics"]
                }),
                required_permission: Some("assessment:write".into()),
                is_consequential: false,
                category: ToolCategory::Generation,
            },
            AiToolDefinition {
                name: "analyze_class_performance".into(),
                description: "Perform comprehensive academic distribution analytics for a class/subject, calculating GPA trends, pass rates, and grade distribution curves.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "class_id": {"type": "string", "description": "Optional UUID of the class"},
                        "class_subject_id": {"type": "string", "description": "Optional UUID of the class-subject"},
                        "period_id": {"type": "string", "description": "Optional UUID of the academic period"}
                    }
                }),
                required_permission: Some("grade:analytics".into()),
                is_consequential: false,
                category: ToolCategory::Analysis,
            },
            AiToolDefinition {
                name: "analyze_financial_health".into(),
                description: "Calculate institutional revenue collection efficiency, fee aging analysis, and overdue tuition breakdown.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "period_id": {"type": "string", "description": "Optional UUID of the academic period"}
                    }
                }),
                required_permission: Some("finance:read".into()),
                is_consequential: false,
                category: ToolCategory::Analysis,
            },
            AiToolDefinition {
                name: "draft_parent_notice".into(),
                description: "Draft a personalized, professional official school communication notice for a student's parent (academic alert, attendance warning, praise, or fee reminder).".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"},
                        "notice_type": {"type": "string", "enum": ["academic_warning", "praise_excellence", "attendance_alert", "fee_reminder", "general_update"], "description": "Type of notice"},
                        "language": {"type": "string", "enum": ["ar", "en"], "description": "Notice language (default ar)"},
                        "custom_notes": {"type": "string", "description": "Specific teacher notes or context to include"}
                    },
                    "required": ["student_id", "notice_type"]
                }),
                required_permission: Some("notification:write".into()),
                is_consequential: false,
                category: ToolCategory::Generation,
            },
        ]
    }

    /// Resolves rich identity, roles, and permissions of the active user.
    pub async fn resolve_user_ai_context(
        &self,
        ctx: &RequestContext,
    ) -> Result<UserAiContext, DomainError> {
        let mut uctx = UserAiContext {
            org_id: ctx.tenant_id,
            org_unit_path: ctx.org_unit_path.clone(),
            ..Default::default()
        };

        if let Some(org) = self.org_service.get_organization(ctx.tenant_id).await? {
            uctx.org_name = Some(org.name);
        }

        match &ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => {
                uctx.user_id = Some(*uid);
                let roles = self
                    .permission_checker
                    .get_user_roles(*uid, ctx.tenant_id)
                    .await?;
                let perms = self
                    .permission_checker
                    .get_user_permissions(*uid, ctx.tenant_id)
                    .await?;
                uctx.roles = roles.clone();
                uctx.permissions = perms;
                if let Some(allowed_units) = self.user_org_unit_allowlist(ctx).await? {
                    let mut paths = Vec::new();
                    for unit_id in allowed_units {
                        if let Some(unit) = self
                            .org_service
                            .org_unit_repo
                            .find_by_id(unit_id)
                            .await?
                            .filter(|unit| unit.org_id == ctx.tenant_id)
                        {
                            paths.push(unit.path);
                        }
                    }
                    uctx.org_unit_path = Some(paths.join(", "));
                }
                if let Some(employee) = self
                    .employee_repo
                    .find_by_user_id(ctx.tenant_id, *uid)
                    .await?
                {
                    if employee.is_teacher() {
                        uctx.employee_id = Some(employee.id);
                        uctx.is_teacher = true;
                        if !roles.iter().any(|role| {
                            role.eq_ignore_ascii_case("admin")
                                || role.eq_ignore_ascii_case("super_admin")
                        }) {
                            uctx.effective_role = Some("teacher".into());
                        }
                        uctx.teaching_assignments =
                            self.teaching_assignments(ctx, employee.id).await?;
                    }
                }

                if roles.iter().any(|r| {
                    r.eq_ignore_ascii_case("super_admin") || r.eq_ignore_ascii_case("admin")
                }) {
                    uctx.effective_role = Some("admin".to_string());
                } else if roles.iter().any(|r| r.eq_ignore_ascii_case("teacher")) {
                    uctx.effective_role = Some("teacher".to_string());
                    uctx.is_teacher = true;
                } else if roles.iter().any(|r| {
                    r.eq_ignore_ascii_case("principal") || r.eq_ignore_ascii_case("headmaster")
                }) {
                    uctx.effective_role = Some("principal".to_string());
                } else if roles.iter().any(|r| r.eq_ignore_ascii_case("counselor")) {
                    uctx.effective_role = Some("counselor".to_string());
                } else if roles.iter().any(|r| r.eq_ignore_ascii_case("parent")) {
                    uctx.effective_role = Some("parent".to_string());
                    uctx.is_parent = true;
                } else {
                    uctx.effective_role =
                        roles.first().cloned().or_else(|| Some("user".to_string()));
                }
            }
            orbis_domain::iam::actor::Actor::Agent(aid) => {
                uctx.effective_role = Some(format!("agent:{}", aid));
            }
            orbis_domain::iam::actor::Actor::Autopilot => {
                uctx.effective_role = Some("autopilot".to_string());
            }
            orbis_domain::iam::actor::Actor::System => {
                uctx.effective_role = Some("system".to_string());
            }
        }

        Ok(uctx)
    }

    /// Resolves academic year, period, class, and subject context from request and domain.
    pub async fn resolve_academic_ai_context(
        &self,
        ctx: &RequestContext,
        request: &AiChatRequest,
    ) -> Result<AcademicAiContext, DomainError> {
        let mut actx = request.academic_context.clone().unwrap_or_default();

        if let Some(ref ui) = request.ui_context {
            if actx.period_id.is_none() {
                actx.period_id = ui.period_id;
            }
            if actx.class_id.is_none() {
                actx.class_id = ui.class_id;
            }
            if actx.subject_id.is_none() {
                actx.subject_id = ui.subject_id;
            }
            if actx.class_subject_id.is_none() {
                actx.class_subject_id = ui.class_subject_id;
            }
        }

        if actx.class_id.is_none() && actx.class_subject_id.is_none() {
            if let Some(employee) = self.teacher_employee(ctx).await? {
                let assignments = self.teaching_assignments(ctx, employee.id).await?;
                if assignments.len() == 1 {
                    let assignment = &assignments[0];
                    actx.class_id = Some(assignment.class_id);
                    actx.class_name = Some(assignment.class_name.clone());
                    actx.subject_id = Some(assignment.subject_id);
                    actx.subject_name = Some(assignment.subject_name.clone());
                    actx.class_subject_id = Some(assignment.class_subject_id);
                    actx.period_id = assignment.period_id;
                }
            }
        }

        if let Some(cid) = actx.class_id {
            let class = self
                .academic_service
                .class_repo
                .find_by_id(cid)
                .await?
                .ok_or_else(|| DomainError::NotFound("Class not found".into()))?;
            ensure_tenant_matches(class.org_id, ctx.tenant_id)?;
            self.ensure_class_access(ctx, cid, actx.period_id).await?;
            actx.class_name = Some(class.name);
        }

        if let Some(sid) = actx.subject_id {
            let subject = self
                .academic_service
                .subject_repo
                .find_by_id(sid)
                .await?
                .ok_or_else(|| DomainError::NotFound("Subject not found".into()))?;
            ensure_tenant_matches(subject.org_id, ctx.tenant_id)?;
            actx.subject_name = Some(subject.name);
        }

        if let Some(class_subject_id) = actx.class_subject_id {
            let assignment = self
                .ensure_class_subject_access(ctx, class_subject_id)
                .await?;
            if actx.class_id.is_some_and(|id| id != assignment.class_id)
                || actx
                    .subject_id
                    .is_some_and(|id| id != assignment.subject_id)
                || actx.period_id.is_some_and(|id| id != assignment.period_id)
            {
                return Err(DomainError::NotFound("Class subject not found".into()));
            }
            actx.class_id = Some(assignment.class_id);
            actx.subject_id = Some(assignment.subject_id);
            actx.period_id = Some(assignment.period_id);
        }

        Ok(actx)
    }

    /// Primary Stateless ReAct Multi-turn Execution Loop with Strict Permission Enforcement.
    pub async fn chat(
        &self,
        ctx: &RequestContext,
        request: AiChatRequest,
    ) -> Result<AiChatResponse, DomainError> {
        let tools = self.get_authorized_tools(ctx).await?;
        let user_ctx = self.resolve_user_ai_context(ctx).await?;
        let academic_ctx = self.resolve_academic_ai_context(ctx, &request).await?;

        let system_prompt = self
            .build_system_prompt(
                ctx,
                &user_ctx,
                &academic_ctx,
                request.ui_context.as_ref(),
                request.screen_context.as_ref(),
            )
            .await;

        let mut conversation_history = request.messages;
        let mut executed_tools_all = Vec::new();
        let mut proposed_actions_all = Vec::new();
        let mut ui_data_all = Vec::new();

        const MAX_REACT_TURNS: usize = 5;
        let mut final_response = String::new();

        let target_model = request.model.as_deref().unwrap_or("");

        for _turn in 0..MAX_REACT_TURNS {
            let llm_output = self
                .llm_gateway
                .chat_with_tools(
                    ctx,
                    target_model,
                    &system_prompt,
                    &conversation_history,
                    &tools,
                    0.2,
                )
                .await?;

            if llm_output.tool_calls.is_empty() {
                final_response = llm_output
                    .content
                    .unwrap_or_else(|| "No response generated.".to_string());
                break;
            }

            // Record assistant message with tool calls in history
            conversation_history.push(AiMessage {
                role: "assistant".into(),
                content: llm_output.content.clone().unwrap_or_default(),
                tool_calls: Some(llm_output.tool_calls.clone()),
                tool_call_id: None,
                created_at: Some(Utc::now()),
            });

            for tool_call in &llm_output.tool_calls {
                let tool_def = tools.iter().find(|t| t.name == tool_call.tool_name);

                if let Some(def) = tool_def {
                    // Strict Authorization Enforcement before any execution or action staging
                    if let Err(error) = self.authorize_tool(ctx, &def.name).await {
                        let err_msg = format!(
                            "Permission denied for tool '{}': {}",
                            tool_call.tool_name, error
                        );
                        conversation_history.push(AiMessage {
                            role: "tool".into(),
                            content: err_msg.clone(),
                            tool_calls: None,
                            tool_call_id: Some(tool_call.id.clone()),
                            created_at: Some(Utc::now()),
                        });
                        executed_tools_all.push(AiToolResult {
                            tool_call_id: tool_call.id.clone(),
                            tool_name: tool_call.tool_name.clone(),
                            content: err_msg,
                            is_error: true,
                        });
                        continue;
                    }

                    if def.is_consequential {
                        // Consequential tool: create a proposed action for Cockpit confirmation
                        let action = AiProposedAction {
                            tool_name: tool_call.tool_name.clone(),
                            title: format!("Proposed Action: {}", tool_call.tool_name),
                            description: format!(
                                "AI requested to execute '{}' with parameters: {}",
                                tool_call.tool_name, tool_call.arguments
                            ),
                            payload: tool_call.arguments.clone(),
                            requires_confirmation: true,
                        };
                        proposed_actions_all.push(action);

                        let tool_result_text = format!(
                            "[Action Pending Confirmation]: The action '{}' requires explicit confirmation from the user in the Cockpit. It has been queued.",
                            tool_call.tool_name
                        );

                        conversation_history.push(AiMessage {
                            role: "tool".into(),
                            content: tool_result_text.clone(),
                            tool_calls: None,
                            tool_call_id: Some(tool_call.id.clone()),
                            created_at: Some(Utc::now()),
                        });

                        executed_tools_all.push(AiToolResult {
                            tool_call_id: tool_call.id.clone(),
                            tool_name: tool_call.tool_name.clone(),
                            content: tool_result_text,
                            is_error: false,
                        });
                        continue;
                    }

                    // Execute non-consequential tool directly
                    let execution_result = self
                        .execute_tool(ctx, &tool_call.tool_name, &tool_call.arguments)
                        .await;

                    let (content, is_error) = match execution_result {
                        Ok(mut data) => {
                            if let Some(private_data) = take_private_ui_result(&mut data) {
                                ui_data_all.push(private_data);
                            }
                            (data.to_string(), false)
                        }
                        Err(e) => (
                            format!("Error executing {}: {}", tool_call.tool_name, e),
                            true,
                        ),
                    };

                    conversation_history.push(AiMessage {
                        role: "tool".into(),
                        content: content.clone(),
                        tool_calls: None,
                        tool_call_id: Some(tool_call.id.clone()),
                        created_at: Some(Utc::now()),
                    });

                    executed_tools_all.push(AiToolResult {
                        tool_call_id: tool_call.id.clone(),
                        tool_name: tool_call.tool_name.clone(),
                        content,
                        is_error,
                    });
                } else {
                    let err_msg = format!("Unknown tool: {}", tool_call.tool_name);
                    conversation_history.push(AiMessage {
                        role: "tool".into(),
                        content: err_msg.clone(),
                        tool_calls: None,
                        tool_call_id: Some(tool_call.id.clone()),
                        created_at: Some(Utc::now()),
                    });
                }
            }
        }

        if final_response.is_empty() {
            final_response = "I have completed processing the requested operations.".to_string();
        }

        Ok(AiChatResponse {
            response: final_response,
            executed_tools: executed_tools_all,
            proposed_actions: proposed_actions_all,
            ui_data: ui_data_all,
            resolved_user_context: Some(user_ctx),
            resolved_academic_context: Some(academic_ctx),
        })
    }

    /// Execute a domain tool by dispatching to the appropriate domain service.
    /// Includes defense-in-depth permission verification.
    pub async fn execute_tool(
        &self,
        ctx: &RequestContext,
        tool_name: &str,
        arguments: &serde_json::Value,
    ) -> Result<serde_json::Value, DomainError> {
        self.authorize_tool(ctx, tool_name).await?;

        match tool_name {
            "search_students" => {
                let term = arguments
                    .get("search_term")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let class_id = arguments
                    .get("class_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                let period_id = arguments
                    .get("period_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());

                if let Some(class_id) = class_id {
                    self.ensure_class_access(ctx, class_id, period_id).await?;
                } else if self.teacher_employee(ctx).await?.is_some()
                    || self.user_org_unit_allowlist(ctx).await?.is_some()
                {
                    return Err(DomainError::Validation(
                        "Select a class within your authorized scope for student search".into(),
                    ));
                }
                let mut filter = StudentFilter::default();
                if !term.is_empty() {
                    filter.search = Some(term.to_string());
                }
                filter.class_id = class_id;
                filter.period_id = period_id;

                let result = self
                    .student_service
                    .list_students(ctx.tenant_id, filter)
                    .await?;
                Ok(json!({
                    "total_count": result.total_count,
                    "students": result.items.into_iter().map(|s| json!({
                        "id": s.student.id,
                        "student_number": s.student.student_number,
                        "name": format!("{} {}", s.person.first_name, s.person.last_name),
                        "status": s.student.status
                    })).collect::<Vec<_>>()
                }))
            }
            "get_student_profile" => {
                let sid_str = arguments
                    .get("student_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DomainError::Validation("student_id is required".into()))?;
                let sid = Uuid::parse_str(sid_str)
                    .map_err(|_| DomainError::Validation("Invalid student UUID".into()))?;
                let profile_opt = self.student_service.get_student(sid).await?;
                let profile =
                    profile_opt.ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
                ensure_tenant_matches(profile.student.org_id, ctx.tenant_id)?;
                Ok(json!({
                    "id": profile.student.id,
                    "student_number": profile.student.student_number,
                    "name": format!("{} {}", profile.person.first_name, profile.person.last_name),
                    "status": profile.student.status
                }))
            }
            "get_student_grades" => {
                let sid_str = arguments
                    .get("student_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DomainError::Validation("student_id is required".into()))?;
                let sid = Uuid::parse_str(sid_str)
                    .map_err(|_| DomainError::Validation("Invalid student UUID".into()))?;
                self.ensure_student_tenant(ctx, sid).await?;
                let period_id = arguments
                    .get("period_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());

                let mut filter = GradeFilter::default();
                filter.student_id = Some(sid);
                filter.period_id = period_id;

                let grades = self
                    .assessment_service
                    .list_grades(ctx.tenant_id, filter)
                    .await?;
                Ok(json!({
                    "total_count": grades.total_count,
                    "grades": grades.items.into_iter().map(|g| json!({
                        "id": g.id,
                        "class_subject_id": g.class_subject_id,
                        "score": g.score,
                        "max_score": g.max_score,
                        "is_final": g.is_final
                    })).collect::<Vec<_>>()
                }))
            }
            "get_student_attendance" => {
                let sid_str = arguments
                    .get("student_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DomainError::Validation("student_id is required".into()))?;
                let sid = Uuid::parse_str(sid_str)
                    .map_err(|_| DomainError::Validation("Invalid student UUID".into()))?;
                self.ensure_student_tenant(ctx, sid).await?;
                let class_id = arguments
                    .get("class_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                if let Some(class_id) = class_id {
                    self.ensure_class_access(ctx, class_id, None).await?;
                }
                let status = arguments
                    .get("status")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let mut filter = AttendanceFilter::default();
                filter.student_id = Some(sid);
                filter.class_id = class_id;
                filter.status = status;

                let attendance = self
                    .assessment_service
                    .list_attendance(ctx.tenant_id, filter)
                    .await?;
                Ok(json!({
                    "total_count": attendance.total_count,
                    "records": attendance.items.into_iter().map(|a| json!({
                        "date": a.attendance_date,
                        "status": a.status,
                        "notes": a.notes
                    })).collect::<Vec<_>>()
                }))
            }
            "get_financial_summary" => {
                let accounts = self.finance_service.list_accounts(ctx.tenant_id).await?;
                let templates = self
                    .finance_service
                    .list_fee_templates(ctx.tenant_id)
                    .await?;
                Ok(json!({
                    "accounts_count": accounts.len(),
                    "fee_templates_count": templates.len(),
                    "accounts": accounts.into_iter().take(10).map(|a| json!({
                        "code": a.code,
                        "name": a.name,
                        "type": a.account_type,
                        "currency": a.currency_code
                    })).collect::<Vec<_>>()
                }))
            }
            "get_student_fees" => {
                let sid_str = arguments
                    .get("student_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DomainError::Validation("student_id is required".into()))?;
                let sid = Uuid::parse_str(sid_str)
                    .map_err(|_| DomainError::Validation("Invalid student UUID".into()))?;
                self.ensure_student_tenant(ctx, sid).await?;
                let mut filter = StudentFeeFilter::default();
                filter.student_id = Some(sid);
                let fees = self
                    .finance_service
                    .list_student_fees(ctx.tenant_id, filter)
                    .await?;
                Ok(json!({
                    "total_count": fees.total_count,
                    "fees": fees.items.into_iter().map(|f| json!({
                        "id": f.id,
                        "original_amount": f.original_amount,
                        "paid_amount": f.paid_amount,
                        "status": f.status,
                        "due_date": f.due_date
                    })).collect::<Vec<_>>()
                }))
            }
            "get_unpaid_fees" => {
                let period_id = arguments
                    .get("period_id")
                    .and_then(|value| value.as_str())
                    .map(Uuid::parse_str)
                    .transpose()
                    .map_err(|_| DomainError::Validation("Invalid period UUID".into()))?;

                let mut filter = StudentFeeFilter {
                    period_id,
                    only_unpaid: Some(true),
                    pagination: Some(PaginationParams {
                        page: Some(1),
                        page_size: Some(50),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                filter.is_overdue = Some(false);
                let fees = self
                    .finance_service
                    .list_student_fees(ctx.tenant_id, filter)
                    .await?;

                let has_more = fees.total_count > fees.items.len() as u64;
                let mut rows = Vec::with_capacity(fees.items.len());
                let mut total_balance = rust_decimal::Decimal::ZERO;
                for fee in fees.items {
                    self.ensure_student_tenant(ctx, fee.student_id).await?;
                    let student = self
                        .student_service
                        .get_student(fee.student_id)
                        .await?
                        .ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
                    total_balance += fee.balance_due;
                    rows.push(json!({
                        "student_name": student.person.full_name(),
                        "balance_due": fee.balance_due.to_string(),
                        "status": fee.status,
                        "due_date": fee.due_date
                    }));
                }

                Ok(json!({
                    "total_count": fees.total_count,
                    "has_more": has_more,
                    "total_balance": total_balance.to_string(),
                    "_ui_private_result": {
                        "type": "unpaid_fees",
                        "items": rows
                    }
                }))
            }
            "list_report_templates" => {
                let templates = self.report_service.list_templates(ctx.tenant_id).await?;
                Ok(json!({
                    "count": templates.len(),
                    "templates": templates.into_iter().map(|t| json!({
                        "id": t.id,
                        "name": t.name,
                        "code": t.code,
                        "document_type": t.document_type
                    })).collect::<Vec<_>>()
                }))
            }
            "generate_report" => {
                let report_code = arguments
                    .get("report_code")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DomainError::Validation("report_code is required".into()))?;
                let definition = self
                    .report_service
                    .get_report_by_code(ctx.tenant_id, report_code)
                    .await?
                    .ok_or_else(|| DomainError::NotFound("Report definition not found".into()))?;
                if matches!(
                    definition.domain,
                    orbis_domain::entities::report::ReportDomain::Finance
                ) {
                    self.permission_checker
                        .check(ctx, Permission::FinanceRead)
                        .await?;
                }
                let output_format_str = arguments
                    .get("output_format")
                    .and_then(|v| v.as_str())
                    .unwrap_or("pdf");
                let output_format = OutputFormat::from_str(output_format_str);

                let mut params = std::collections::HashMap::new();
                if let Some(obj) = arguments.get("parameters").and_then(|v| v.as_object()) {
                    for (k, v) in obj {
                        if let Some(s) = v.as_str() {
                            params.insert(k.clone(), ParamValue::Text(s.to_string()));
                        } else if let Some(n) = v.as_i64() {
                            params.insert(k.clone(), ParamValue::Integer(n));
                        } else if let Some(b) = v.as_bool() {
                            params.insert(k.clone(), ParamValue::Boolean(b));
                        }
                    }
                }

                let input = GenerateReportInput {
                    report_code: report_code.to_string(),
                    output_format,
                    parameters: params,
                    locale: ReportLocale::Ar,
                };

                let job = self.report_service.enqueue_report(ctx, input).await?;
                Ok(json!({
                    "job_id": job.id,
                    "status": job.status.to_string(),
                    "report_code": job.report_code
                }))
            }
            "get_org_overview" => {
                let org = self.org_service.get_organization(ctx.tenant_id).await?;
                let units = self.org_service.list_org_units(ctx.tenant_id).await?;
                let counts = self
                    .org_service
                    .get_dashboard_counts(ctx.tenant_id)
                    .await
                    .ok();

                Ok(json!({
                    "organization": org.map(|o| json!({"name": o.name, "slug": o.slug, "timezone": o.timezone})),
                    "branches_count": units.len(),
                    "dashboard_stats": counts.map(|c| json!({
                        "total_students": c.total_students,
                        "total_teachers": c.total_teachers,
                        "total_classes": c.total_classes
                    }))
                }))
            }
            "get_autopilot_status" => {
                let ap = self.autopilot_service.as_ref().ok_or_else(|| {
                    DomainError::Internal("Autopilot service not available".into())
                })?;
                let diag = ap.get_diagnostics(ctx, ctx.tenant_id).await?;
                Ok(json!({
                    "status": diag.status,
                    "mode": diag.mode,
                    "is_healthy": diag.is_healthy,
                    "total_tasks": diag.total_tasks_count,
                    "pending_tasks": diag.pending_tasks_count,
                    "completed_tasks": diag.completed_tasks_count,
                    "failed_tasks": diag.failed_tasks_count,
                    "stuck_tasks_count": diag.stuck_tasks.len(),
                    "overdue_tasks_count": diag.overdue_tasks.len()
                }))
            }
            "list_autopilot_tasks" => {
                let ap = self.autopilot_service.as_ref().ok_or_else(|| {
                    DomainError::Internal("Autopilot service not available".into())
                })?;
                let status_filter = arguments.get("status").and_then(|v| v.as_str());
                let (tasks, total) = ap
                    .list_tasks(ctx, ctx.tenant_id, status_filter, None, 20, 0)
                    .await?;
                Ok(json!({
                    "total": total,
                    "tasks": tasks.into_iter().map(|t| json!({
                        "id": t.id,
                        "task_type": t.task_type.as_str(),
                        "status": t.status.as_str(),
                        "risk_level": t.risk_level.as_str(),
                        "reason": t.reason,
                        "scheduled_at": t.scheduled_at,
                        "attempts": t.attempts,
                        "error_log": t.error_log
                    })).collect::<Vec<_>>()
                }))
            }
            "trigger_autopilot_tick" => {
                let ap = self.autopilot_service.as_ref().ok_or_else(|| {
                    DomainError::Internal("Autopilot service not available".into())
                })?;
                let dry_run = arguments
                    .get("dry_run")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let tasks = ap.run_tick(ctx, ctx.tenant_id, dry_run).await?;
                Ok(json!({
                    "dry_run": dry_run,
                    "evaluated_tasks_count": tasks.len(),
                    "tasks": tasks.into_iter().map(|t| json!({
                        "id": t.id,
                        "task_type": t.task_type.as_str(),
                        "status": t.status.as_str(),
                        "reason": t.reason
                    })).collect::<Vec<_>>()
                }))
            }

            // ── Advanced Real-World Educational AI Tools ──────────
            "analyze_at_risk_students" => {
                let class_id = arguments
                    .get("class_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                let period_id = arguments
                    .get("period_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                if let Some(class_id) = class_id {
                    self.ensure_class_access(ctx, class_id, period_id).await?;
                } else if self.teacher_employee(ctx).await?.is_some()
                    || self.user_org_unit_allowlist(ctx).await?.is_some()
                {
                    return Err(DomainError::Validation(
                        "Select a class within your authorized scope for at-risk analysis".into(),
                    ));
                }

                let mut filter = StudentFilter::default();
                filter.class_id = class_id;
                filter.period_id = period_id;

                let students_list = self
                    .student_service
                    .list_students(ctx.tenant_id, filter)
                    .await?;
                let mut at_risk_items = Vec::new();

                for s in students_list.items.iter().take(30) {
                    let sid = s.student.id;

                    // 1. Attendance Analysis
                    let mut att_filter = AttendanceFilter::default();
                    att_filter.student_id = Some(sid);
                    let att_records = self
                        .assessment_service
                        .list_attendance(ctx.tenant_id, att_filter)
                        .await
                        .unwrap_or_else(|_| {
                            orbis_domain::queries::filters::PaginatedList::empty(20)
                        });
                    let total_days = att_records.total_count;
                    let absent_count = att_records
                        .items
                        .iter()
                        .filter(|a| {
                            a.status.eq_ignore_ascii_case("absent") || a.status.contains("غائب")
                        })
                        .count();
                    let absence_rate = if total_days > 0 {
                        (absent_count as f64 / total_days as f64) * 100.0
                    } else {
                        0.0
                    };

                    // 2. Academic Performance Analysis
                    let mut gr_filter = GradeFilter::default();
                    gr_filter.student_id = Some(sid);
                    gr_filter.period_id = period_id;
                    let grade_records = self
                        .assessment_service
                        .list_grades(ctx.tenant_id, gr_filter)
                        .await
                        .unwrap_or_else(|_| {
                            orbis_domain::queries::filters::PaginatedList::empty(20)
                        });
                    let mut total_score = rust_decimal::Decimal::ZERO;
                    let mut total_max = rust_decimal::Decimal::ZERO;
                    let mut failing_count = 0;

                    for g in &grade_records.items {
                        total_score += g.score;
                        total_max += g.max_score;
                        if g.max_score > rust_decimal::Decimal::ZERO {
                            let ratio = g.score / g.max_score;
                            if ratio
                                < rust_decimal::Decimal::from_str_exact("0.50").unwrap_or_default()
                            {
                                failing_count += 1;
                            }
                        }
                    }
                    let avg_grade_pct = if total_max > rust_decimal::Decimal::ZERO {
                        (total_score / total_max * rust_decimal::Decimal::from(100)).to_string()
                    } else {
                        "N/A".to_string()
                    };

                    // 3. Fee Overdue Analysis
                    let mut fee_filter = StudentFeeFilter::default();
                    fee_filter.student_id = Some(sid);
                    let fee_records = self
                        .finance_service
                        .list_student_fees(ctx.tenant_id, fee_filter)
                        .await
                        .unwrap_or_else(|_| {
                            orbis_domain::queries::filters::PaginatedList::empty(20)
                        });
                    let mut overdue_balance = rust_decimal::Decimal::ZERO;
                    for f in &fee_records.items {
                        if f.status == orbis_domain::entities::student_fee::FeeStatus::Overdue
                            || f.balance_due > rust_decimal::Decimal::ZERO
                        {
                            overdue_balance += f.balance_due;
                        }
                    }

                    // Composite risk score calculation
                    let mut risk_score: u32 = 0;
                    let mut risk_factors = Vec::new();

                    if absence_rate >= 15.0 {
                        risk_score += 35;
                        risk_factors.push(format!(
                            "High absence rate ({:.1}%, {} days)",
                            absence_rate, absent_count
                        ));
                    } else if absence_rate >= 10.0 {
                        risk_score += 20;
                        risk_factors.push(format!(
                            "Moderate absence rate ({:.1}%, {} days)",
                            absence_rate, absent_count
                        ));
                    }

                    if failing_count > 0 {
                        risk_score += (failing_count as u32 * 15).min(40);
                        risk_factors.push(format!(
                            "Failing in {} subjects (Avg: {}%)",
                            failing_count, avg_grade_pct
                        ));
                    }

                    if overdue_balance > rust_decimal::Decimal::ZERO {
                        risk_score += 15;
                        risk_factors.push(format!("Overdue tuition balance: {}", overdue_balance));
                    }

                    let risk_level = if risk_score >= 50 {
                        "high"
                    } else if risk_score >= 25 {
                        "medium"
                    } else {
                        "low"
                    };

                    if risk_level == "high" || risk_level == "medium" {
                        let mut interventions = Vec::new();
                        if absence_rate >= 10.0 {
                            interventions.push(
                                "Issue attendance warning and schedule parent counseling"
                                    .to_string(),
                            );
                        }
                        if failing_count > 0 {
                            interventions
                                .push("Enroll in remedial academic support sessions".to_string());
                        }
                        if overdue_balance > rust_decimal::Decimal::ZERO {
                            interventions.push(
                                "Review financial assistance or installment schedule".to_string(),
                            );
                        }

                        at_risk_items.push(json!({
                            "student_id": s.student.id,
                            "student_number": s.student.student_number,
                            "name": format!("{} {}", s.person.first_name, s.person.last_name),
                            "risk_level": risk_level,
                            "risk_score": risk_score,
                            "risk_factors": risk_factors,
                            "absence_rate_pct": format!("{:.1}%", absence_rate),
                            "absent_days": absent_count,
                            "failing_subjects_count": failing_count,
                            "average_grade_pct": avg_grade_pct,
                            "overdue_fee_balance": overdue_balance.to_string(),
                            "recommended_interventions": interventions
                        }));
                    }
                }

                Ok(json!({
                    "analyzed_students_count": students_list.items.len(),
                    "at_risk_students_count": at_risk_items.len(),
                    "students": at_risk_items
                }))
            }

            "generate_exam_paper" => {
                let subject_name = arguments
                    .get("subject_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Mathematics");
                let grade_level = arguments
                    .get("grade_level")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Grade 8");
                let topics = arguments
                    .get("topics")
                    .and_then(|v| v.as_str())
                    .unwrap_or("General Curriculum Review");
                let language = arguments
                    .get("language")
                    .and_then(|v| v.as_str())
                    .unwrap_or("ar");
                let difficulty = arguments
                    .get("difficulty")
                    .and_then(|v| v.as_str())
                    .unwrap_or("balanced");
                let total_marks = arguments
                    .get("total_marks")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(100);
                let duration_minutes = arguments
                    .get("duration_minutes")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(60);
                let question_count = arguments
                    .get("question_count")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(8);

                let system_instructions = format!(
                    "You are an expert curriculum and assessment designer for the Orbis Educational Operating System.\n\
                     Generate a formal, balanced, high-quality examination paper in JSON format.\n\
                     Subject: {}\nGrade Level: {}\nTopics: {}\nTarget Language: {}\nDifficulty: {}\nTotal Marks: {}\nDuration: {} mins\nQuestion Count: {}\n\n\
                     Requirements:\n\
                     1. Apply Bloom's Taxonomy (Remember, Understand, Apply, Analyze, Evaluate).\n\
                     2. Include a balanced mix: Multiple Choice (MCQ), Short Answer, and Comprehensive / Problem Solving.\n\
                     3. Provide clear model answers / answer keys and allocated marks for every question.\n\
                     4. Respond with valid JSON only conforming to:\n\
                     {{\n\
                       \"exam_title\": \"...\",\n\
                       \"subject\": \"...\",\n\
                       \"grade_level\": \"...\",\n\
                       \"total_marks\": {},\n\
                       \"duration_minutes\": {},\n\
                       \"instructions\": [\"...\"],\n\
                       \"sections\": [\n\
                         {{\n\
                           \"section_title\": \"...\",\n\
                           \"section_marks\": 30,\n\
                           \"questions\": [\n\
                             {{\n\
                               \"question_number\": 1,\n\
                               \"question_type\": \"mcq|true_false|short_answer|essay|numeric\",\n\
                               \"bloom_level\": \"remember|understand|apply|analyze|evaluate\",\n\
                               \"marks\": 5,\n\
                               \"prompt\": \"...\",\n\
                               \"options\": [\"A) ...\", \"B) ...\", \"C) ...\", \"D) ...\"],\n\
                               \"correct_answer\": \"...\",\n\
                               \"explanation\": \"...\"\n\
                             }}\n\
                           ]\n\
                         }}\n\
                       ]\n\
                     }}",
                    subject_name,
                    grade_level,
                    topics,
                    language,
                    difficulty,
                    total_marks,
                    duration_minutes,
                    question_count,
                    total_marks,
                    duration_minutes
                );

                let user_prompt = format!(
                    "Generate the complete examination paper for {} ({}) covering: {}. Total marks: {}, Duration: {} minutes.",
                    subject_name, grade_level, topics, total_marks, duration_minutes
                );

                let generated_json_str = self
                    .llm_gateway
                    .generate_text(ctx, "", &system_instructions, &user_prompt, 0.2)
                    .await?;
                let clean_str = generated_json_str
                    .trim()
                    .trim_start_matches("```json")
                    .trim_start_matches("```")
                    .trim_end_matches("```")
                    .trim();
                let exam_json: serde_json::Value =
                    serde_json::from_str(clean_str).unwrap_or_else(|_| {
                        json!({
                            "raw_generated_exam": generated_json_str,
                            "subject": subject_name,
                            "grade_level": grade_level,
                            "topics": topics
                        })
                    });

                Ok(json!({
                    "status": "success",
                    "exam": exam_json
                }))
            }

            "analyze_class_performance" => {
                let class_id = arguments
                    .get("class_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                let class_subject_id = arguments
                    .get("class_subject_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                let period_id = arguments
                    .get("period_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                if let Some(class_id) = class_id {
                    self.ensure_class_access(ctx, class_id, period_id).await?;
                }
                if let Some(class_subject_id) = class_subject_id {
                    let class_subject = self
                        .ensure_class_subject_access(ctx, class_subject_id)
                        .await?;
                    if class_id.is_some_and(|id| id != class_subject.class_id) {
                        return Err(DomainError::NotFound("Class subject not found".into()));
                    }
                } else if self.teacher_employee(ctx).await?.is_some()
                    || self.user_org_unit_allowlist(ctx).await?.is_some()
                {
                    return Err(DomainError::Validation(
                        "Select a class subject within your authorized scope for performance analysis"
                            .into(),
                    ));
                }

                let mut gr_filter = GradeFilter::default();
                gr_filter.class_subject_id = class_subject_id;
                gr_filter.period_id = period_id;

                let grades_list = self
                    .assessment_service
                    .list_grades(ctx.tenant_id, gr_filter)
                    .await?;
                let mut total_score = rust_decimal::Decimal::ZERO;
                let mut total_max = rust_decimal::Decimal::ZERO;
                let mut count = 0;
                let mut a_count = 0;
                let mut b_count = 0;
                let mut c_count = 0;
                let mut d_count = 0;
                let mut f_count = 0;

                for g in &grades_list.items {
                    if g.max_score > rust_decimal::Decimal::ZERO {
                        total_score += g.score;
                        total_max += g.max_score;
                        count += 1;
                        let pct = (g.score / g.max_score * rust_decimal::Decimal::from(100))
                            .to_string()
                            .parse::<f64>()
                            .unwrap_or(0.0);
                        if pct >= 90.0 {
                            a_count += 1;
                        } else if pct >= 80.0 {
                            b_count += 1;
                        } else if pct >= 70.0 {
                            c_count += 1;
                        } else if pct >= 60.0 {
                            d_count += 1;
                        } else {
                            f_count += 1;
                        }
                    }
                }

                let avg_pct = if total_max > rust_decimal::Decimal::ZERO {
                    (total_score / total_max * rust_decimal::Decimal::from(100)).to_string()
                } else {
                    "0.0".to_string()
                };
                let pass_rate = if count > 0 {
                    format!("{:.1}%", ((count - f_count) as f64 / count as f64) * 100.0)
                } else {
                    "100%".to_string()
                };

                Ok(json!({
                    "evaluated_grades_count": count,
                    "class_id": class_id,
                    "class_subject_id": class_subject_id,
                    "period_id": period_id,
                    "average_score_pct": format!("{}%", avg_pct),
                    "pass_rate": pass_rate,
                    "distribution": {
                        "A_excellent_90_100": a_count,
                        "B_very_good_80_89": b_count,
                        "C_good_70_79": c_count,
                        "D_pass_60_69": d_count,
                        "F_fail_below_60": f_count
                    }
                }))
            }

            "analyze_financial_health" => {
                let period_id = arguments
                    .get("period_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());

                let fee_filter = StudentFeeFilter::default();
                let fees_list = self
                    .finance_service
                    .list_student_fees(ctx.tenant_id, fee_filter)
                    .await?;
                let mut total_billed = rust_decimal::Decimal::ZERO;
                let mut total_collected = rust_decimal::Decimal::ZERO;
                let mut total_outstanding = rust_decimal::Decimal::ZERO;
                let mut paid_count = 0;
                let mut partial_count = 0;
                let mut overdue_count = 0;
                let mut pending_count = 0;

                for f in &fees_list.items {
                    total_billed += f.original_amount;
                    total_collected += f.paid_amount;
                    total_outstanding += f.balance_due;

                    match f.status {
                        orbis_domain::entities::student_fee::FeeStatus::Paid => paid_count += 1,
                        orbis_domain::entities::student_fee::FeeStatus::PartiallyPaid => {
                            partial_count += 1
                        }
                        orbis_domain::entities::student_fee::FeeStatus::Overdue => {
                            overdue_count += 1
                        }
                        _ => pending_count += 1,
                    }
                }

                let collection_efficiency = if total_billed > rust_decimal::Decimal::ZERO {
                    format!(
                        "{:.1}%",
                        (total_collected / total_billed * rust_decimal::Decimal::from(100))
                            .to_string()
                            .parse::<f64>()
                            .unwrap_or(0.0)
                    )
                } else {
                    "100%".to_string()
                };

                Ok(json!({
                    "total_fee_records": fees_list.total_count,
                    "period_id": period_id,
                    "total_billed_amount": total_billed.to_string(),
                    "total_collected_revenue": total_collected.to_string(),
                    "total_outstanding_balance": total_outstanding.to_string(),
                    "collection_efficiency_rate": collection_efficiency,
                    "status_breakdown": {
                        "fully_paid_count": paid_count,
                        "partially_paid_count": partial_count,
                        "overdue_count": overdue_count,
                        "pending_count": pending_count
                    }
                }))
            }

            "draft_parent_notice" => {
                let sid_str = arguments
                    .get("student_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DomainError::Validation("student_id is required".into()))?;
                let sid = Uuid::parse_str(sid_str)
                    .map_err(|_| DomainError::Validation("Invalid student UUID".into()))?;
                self.ensure_student_tenant(ctx, sid).await?;
                let notice_type = arguments
                    .get("notice_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("academic_warning");
                let language = arguments
                    .get("language")
                    .and_then(|v| v.as_str())
                    .unwrap_or("ar");
                let custom_notes = arguments
                    .get("custom_notes")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                let student_opt = self.student_service.get_student(sid).await?;
                let student =
                    student_opt.ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
                ensure_tenant_matches(student.student.org_id, ctx.tenant_id)?;

                let student_name =
                    format!("{} {}", student.person.first_name, student.person.last_name);

                let system_instructions = format!(
                    "You are an official school administrative assistant drafting a formal parent communication letter for the Orbis Education Platform.\n\
                     Language: {}\n\
                     Tone: Respectful, professional, clear, constructive, and empathetic.\n\
                     Notice Type: {}\n\
                     Student Name: {}\n\
                     Student Number: {}\n\
                     Additional Notes: {}\n\n\
                     Return a JSON object with fields: 'title', 'greeting', 'body', 'action_required', 'closing', 'contact_signoff'.",
                    language,
                    notice_type,
                    student_name,
                    student.student.student_number,
                    custom_notes
                );

                let user_prompt = format!(
                    "Draft the parent notice for {} regarding: {}.",
                    student_name, notice_type
                );

                let generated_text = self
                    .llm_gateway
                    .generate_text(ctx, "", &system_instructions, &user_prompt, 0.3)
                    .await?;
                let clean_str = generated_text
                    .trim()
                    .trim_start_matches("```json")
                    .trim_start_matches("```")
                    .trim_end_matches("```")
                    .trim();
                let notice_json: serde_json::Value = serde_json::from_str(clean_str)
                    .unwrap_or_else(|_| {
                        json!({
                            "raw_notice": generated_text,
                            "student_name": student_name,
                            "notice_type": notice_type
                        })
                    });

                Ok(json!({
                    "status": "success",
                    "notice": notice_json
                }))
            }

            _ => Err(DomainError::Validation(format!(
                "Tool '{}' not supported",
                tool_name
            ))),
        }
    }

    /// Build dynamic system prompt with tenant, user, academic, and UI screen context.
    pub async fn build_system_prompt(
        &self,
        _ctx: &RequestContext,
        user_context: &UserAiContext,
        academic_context: &AcademicAiContext,
        ui_context: Option<&UiContext>,
        screen_context: Option<&serde_json::Value>,
    ) -> String {
        let mut prompt = format!(
            "You are Orbis AI, the intelligent, authoritative enterprise assistant for the Orbis Educational Operating System.\n\n\
             [CONTEXT & IDENTITY]\n\
             - Tenant: {} (Org ID: {})\n\
             - User Role: {}\n\
             - Assigned Roles: [{}]\n\
             - Permissions Count: {}\n\
             - Branch / Scope Path: {}\n",
            user_context.org_name.as_deref().unwrap_or("Orbis School"),
            user_context.org_id,
            user_context
                .effective_role
                .as_deref()
                .unwrap_or("Authorized User"),
            user_context.roles.join(", "),
            user_context.permissions.len(),
            user_context
                .org_unit_path
                .as_deref()
                .unwrap_or("Root / Organization-wide")
        );

        if academic_context.academic_year_name.is_some()
            || academic_context.period_name.is_some()
            || academic_context.class_name.is_some()
            || academic_context.subject_name.is_some()
        {
            prompt.push_str("\n[ACADEMIC CONTEXT]\n");
            if let Some(ref y) = academic_context.academic_year_name {
                prompt.push_str(&format!("- Academic Year: {}\n", y));
            }
            if let Some(ref p) = academic_context.period_name {
                prompt.push_str(&format!("- Active Period: {}\n", p));
            }
            if let Some(ref c) = academic_context.class_name {
                prompt.push_str(&format!("- Selected Class: {}\n", c));
            }
            if let Some(ref s) = academic_context.subject_name {
                prompt.push_str(&format!("- Selected Subject: {}\n", s));
            }
        }

        if !user_context.teaching_assignments.is_empty() {
            prompt.push_str("\n[VERIFIED TEACHING ASSIGNMENTS]\n");
            for assignment in &user_context.teaching_assignments {
                prompt.push_str(&format!(
                    "- Class: {} ({}), Subject: {} ({}), Period: {}, Assignment: {}\n",
                    assignment.class_name,
                    assignment.class_id,
                    assignment.subject_name,
                    assignment.subject_id,
                    assignment
                        .period_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "unspecified".into()),
                    assignment.class_subject_id
                ));
            }
            prompt.push_str(
                "Use only these assignments for teacher-specific academic requests. If more than one applies and the request does not distinguish them, ask the user to choose.\n",
            );
        }

        if let Some(ui) = ui_context {
            prompt.push_str("\n[ACTIVE UI SCREEN CONTEXT]\n");
            if let Some(ref route) = ui.route {
                prompt.push_str(&format!("- Route: {}\n", route));
            }
            if let Some(ref screen) = ui.screen {
                prompt.push_str(&format!("- Screen: {}\n", screen));
            }
            if let Some(ref domain) = ui.domain {
                prompt.push_str(&format!("- Domain: {}\n", domain));
            }
            if let Some(ref etype) = ui.entity_type {
                prompt.push_str(&format!("- Viewing Entity Type: {}\n", etype));
            }
            if let Some(eid) = ui.entity_id {
                prompt.push_str(&format!("- Primary Entity ID: {}\n", eid));
            }
            if !ui.selected_entity_ids.is_empty() {
                prompt.push_str(&format!(
                    "- Selected Entity IDs: {:?}\n",
                    ui.selected_entity_ids
                ));
            }
            if let Some(ref filters) = ui.active_filters {
                prompt.push_str(&format!("- Active Filters: {}\n", filters));
            }
        } else if let Some(screen) = screen_context {
            prompt.push_str(&format!(
                "\n[ACTIVE SCREEN DATA]\n```json\n{}\n```\n",
                screen
            ));
        }

        prompt.push_str(
            "\n[OPERATIONAL & SAFETY CONTRACT]\n\
             1. Always query and execute authoritative domain tools to produce real-time, authentic institutional data. Never fabricate student names, IDs, grades, or finances.\n\
             2. Respect user permissions and role boundaries at all times.\n\
             3. Consequential actions (e.g. changing student status, triggering autopilot tasks) must be proposed for user review and confirmed through Cockpit.\n\
             4. For educational assessments, generate structured blueprints and draft questions adhering to Bloom's taxonomy; final publication and subjective grading suggestions always require teacher review and sign-off.\n\
             5. Respond in Arabic or English matching the user's conversation language."
        );

        prompt
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiOrchestrationService, check_tool_permission, ensure_tenant_matches,
        required_tool_permission, take_private_ui_result,
        teacher_assigned_to_class, tool_supports_unit_scoping, unit_in_scope,
    };
    use crate::context::{ActionSource, RequestContext};
    use crate::ports::iam::PermissionCheckerPort;
    use async_trait::async_trait;
    use orbis_domain::entities::ai::{AiToolDefinition, ToolCategory};
    use orbis_domain::errors::DomainError;
    use orbis_domain::iam::actor::Actor;
    use orbis_domain::iam::permission::Permission;
    use serde_json::json;
    use std::collections::HashSet;
    use uuid::Uuid;

    struct DenyPermissionChecker;

    #[async_trait]
    impl PermissionCheckerPort for DenyPermissionChecker {
        async fn check(
            &self,
            _ctx: &RequestContext,
            permission: Permission,
        ) -> Result<(), DomainError> {
            Err(DomainError::Unauthorized(format!(
                "Missing permission: {}",
                permission.as_str()
            )))
        }
    }

    #[test]
    fn every_registered_tool_has_a_known_permission() {
        let tools = AiOrchestrationService::registered_tools();
        let mut names = HashSet::new();

        assert!(!tools.is_empty());
        for tool in tools {
            assert!(
                names.insert(tool.name.clone()),
                "duplicate tool: {}",
                tool.name
            );
            let permission = tool
                .required_permission
                .as_deref()
                .unwrap_or_else(|| panic!("tool {} has no required permission", tool.name));
            let parsed = Permission::from_str(permission).unwrap_or_else(|| {
                panic!("tool {} has unknown permission {permission}", tool.name)
            });
            assert_eq!(parsed.as_str(), permission);
        }
    }

    #[test]
    fn unpaid_fees_requires_finance_read() {
        let tool = AiOrchestrationService::registered_tools()
            .into_iter()
            .find(|tool| tool.name == "get_unpaid_fees")
            .expect("unpaid-fee query tool is registered");

        assert_eq!(
            required_tool_permission(&tool).expect("tool permission is required"),
            Permission::FinanceRead
        );
    }

    #[test]
    fn missing_or_unknown_tool_permissions_are_denied() {
        for required_permission in [None, Some("not:a:permission".into())] {
            let tool = AiToolDefinition {
                name: "invalid_tool".into(),
                description: String::new(),
                parameters_schema: json!({"type": "object"}),
                required_permission,
                is_consequential: false,
                category: ToolCategory::Read,
            };
            assert!(required_tool_permission(&tool).is_err());
        }
    }

    #[tokio::test]
    async fn unauthorized_tool_is_rejected_before_dispatch() {
        let ctx = RequestContext {
            tenant_id: Uuid::new_v4(),
            actor: Actor::User(Uuid::new_v4()),
            request_id: "test".into(),
            source: ActionSource::Api,
            org_unit_path: None,
        };
        let tool = AiOrchestrationService::registered_tools()
            .into_iter()
            .find(|tool| tool.name == "get_unpaid_fees")
            .expect("unpaid-fee query tool is registered");

        let result = check_tool_permission(&DenyPermissionChecker, &ctx, &tool).await;
        assert!(matches!(result, Err(DomainError::Unauthorized(_))));
    }

    #[test]
    fn cross_tenant_ai_records_are_hidden_as_not_found() {
        let tenant_id = Uuid::new_v4();
        assert!(ensure_tenant_matches(tenant_id, tenant_id).is_ok());
        assert!(matches!(
            ensure_tenant_matches(Uuid::new_v4(), tenant_id),
            Err(DomainError::NotFound(_))
        ));
    }

    #[test]
    fn teacher_class_scope_requires_a_matching_assignment() {
        let teacher_id = Uuid::new_v4();
        let class_id = Uuid::new_v4();
        let other_class_id = Uuid::new_v4();
        let assignment = orbis_domain::entities::class_subject::ClassSubject {
            id: Uuid::new_v4(),
            class_id,
            subject_id: Uuid::new_v4(),
            period_id: Uuid::new_v4(),
            primary_teacher_id: teacher_id,
            assistant_teacher_id: None,
            substitute_teacher_id: None,
            grade_groups: None,
            weekly_hours: None,
            theory_hours: None,
            practical_hours: None,
            assessment_config: None,
            scheme_id: None,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert!(teacher_assigned_to_class(
            teacher_id,
            class_id,
            None,
            std::slice::from_ref(&assignment)
        ));
        assert!(!teacher_assigned_to_class(
            teacher_id,
            other_class_id,
            None,
            std::slice::from_ref(&assignment)
        ));
    }

    #[test]
    fn unit_scoped_tools_require_records_inside_the_allowlist() {
        let allowed_unit = Uuid::new_v4();
        let other_unit = Uuid::new_v4();
        assert!(unit_in_scope(Some(allowed_unit), Some(&[allowed_unit])));
        assert!(!unit_in_scope(Some(other_unit), Some(&[allowed_unit])));
        assert!(!unit_in_scope(None, Some(&[allowed_unit])));
        assert!(unit_in_scope(None, None));
        assert!(tool_supports_unit_scoping("get_student_profile"));
        assert!(!tool_supports_unit_scoping("get_financial_summary"));
        assert!(!tool_supports_unit_scoping("get_unpaid_fees"));
    }

    #[test]
    fn private_ui_rows_are_removed_from_the_tool_result_before_model_history() {
        let mut result = json!({
            "total_count": 1,
            "_ui_private_result": {
                "type": "unpaid_fees",
                "items": [{"student_name": "Private Student"}]
            }
        });

        let ui_data = take_private_ui_result(&mut result);

        assert_eq!(result, json!({"total_count": 1}));
        assert_eq!(
            ui_data.expect("UI should receive the separate private result")["items"][0]["student_name"],
            "Private Student"
        );
    }
}
