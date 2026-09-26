//! AI Orchestration Service — Stateless ReAct Engine & Direct Domain Integration.
//! Integrates directly with existing Domain Services (Student, Academic, Assessment, Finance, Scheduling, Reports, Org)
//! without extra database tables or intermediate state bloat.

use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::gateways::LlmGatewayPort;
use crate::ports::iam::PermissionCheckerPort;
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
    AiChatRequest, AiChatResponse, AiMessage, AiProposedAction, AiToolDefinition,
    AiToolResult,
};
use orbis_domain::entities::report::{OutputFormat, ParamValue, ReportLocale};
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::{
    AttendanceFilter, GradeFilter, StudentFeeFilter, StudentFilter,
};

pub struct AiOrchestrationService {
    pub llm_gateway: Arc<dyn LlmGatewayPort>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
    pub student_service: Arc<StudentService>,
    pub academic_service: Arc<AcademicService>,
    pub assessment_service: Arc<AssessmentService>,
    pub finance_service: Arc<FinanceService>,
    pub scheduling_service: Arc<SchedulingService>,
    pub report_service: Arc<ReportService>,
    pub org_service: Arc<OrganizationService>,
    pub autopilot_service: Option<Arc<AutopilotService>>,
}

impl AiOrchestrationService {
    pub fn new(
        llm_gateway: Arc<dyn LlmGatewayPort>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
        student_service: Arc<StudentService>,
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

    /// Registry of available domain tools with JSON schemas and permission requirements.
    pub fn get_available_tools(&self) -> Vec<AiToolDefinition> {
        vec![
            AiToolDefinition {
                name: "search_students".into(),
                description: "Search students by name, national ID, or student number in the organization.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "search_term": {"type": "string", "description": "Search query for name, student number, or national ID"}
                    },
                    "required": ["search_term"]
                }),
                required_permission: Some("student:read".into()),
                is_consequential: false,
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
            },
            AiToolDefinition {
                name: "get_student_grades".into(),
                description: "Get grades and assessment scores recorded for a student.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"}
                    },
                    "required": ["student_id"]
                }),
                required_permission: Some("grade:read".into()),
                is_consequential: false,
            },
            AiToolDefinition {
                name: "get_student_attendance".into(),
                description: "Get attendance history and absence records for a student.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "student_id": {"type": "string", "description": "UUID of the student"}
                    },
                    "required": ["student_id"]
                }),
                required_permission: Some("attendance:read".into()),
                is_consequential: false,
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
            },
            AiToolDefinition {
                name: "list_report_templates".into(),
                description: "List available report and document templates in the system.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
                required_permission: Some("report:read".into()),
                is_consequential: false,
            },
            AiToolDefinition {
                name: "generate_report".into(),
                description: "Generate a formal PDF or Excel report using DocForge reporting engine.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {
                        "report_code": {"type": "string", "description": "Code of the report definition (e.g. STUDENT_TRANSCRIPT, FEE_COLLECTION)"},
                        "output_format": {"type": "string", "enum": ["pdf", "csv", "json", "html"], "description": "Format for generated output"},
                        "parameters": {"type": "object", "description": "Parameters required by the report definition"}
                    },
                    "required": ["report_code", "output_format"]
                }),
                required_permission: Some("report:generate".into()),
                is_consequential: false,
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
            },
            AiToolDefinition {
                name: "get_autopilot_status".into(),
                description: "Get the current operating mode, status, and health diagnostics of the Orbis Autopilot engine.".into(),
                parameters_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
                required_permission: Some("settings:read".into()),
                is_consequential: false,
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
                required_permission: Some("settings:read".into()),
                is_consequential: false,
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
                required_permission: Some("settings:write".into()),
                is_consequential: true,
            },
        ]
    }

    /// Primary Stateless ReAct Multi-turn Execution Loop.
    pub async fn chat(
        &self,
        ctx: &RequestContext,
        request: AiChatRequest,
    ) -> Result<AiChatResponse, DomainError> {
        let tools = self.get_available_tools();
        let system_prompt = self.build_system_prompt(ctx, request.screen_context.as_ref()).await;

        let mut conversation_history = request.messages;
        let mut executed_tools_all = Vec::new();
        let mut proposed_actions_all = Vec::new();

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
                        Ok(data) => (data.to_string(), false),
                        Err(e) => (format!("Error executing {}: {}", tool_call.tool_name, e), true),
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
        })
    }

    /// Execute a domain tool by dispatching to the appropriate domain service.
    async fn execute_tool(
        &self,
        ctx: &RequestContext,
        tool_name: &str,
        arguments: &serde_json::Value,
    ) -> Result<serde_json::Value, DomainError> {
        match tool_name {
            "search_students" => {
                let term = arguments
                    .get("search_term")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let mut filter = StudentFilter::default();
                filter.search = Some(term.to_string());
                let result = self.student_service.list_students(ctx.tenant_id, filter).await?;
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
                let profile = profile_opt
                    .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", sid)))?;
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
                let mut filter = GradeFilter::default();
                filter.student_id = Some(sid);
                let grades = self.assessment_service.list_grades(ctx.tenant_id, filter).await?;
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
                let mut filter = AttendanceFilter::default();
                filter.student_id = Some(sid);
                let attendance = self.assessment_service.list_attendance(ctx.tenant_id, filter).await?;
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
                let templates = self.finance_service.list_fee_templates(ctx.tenant_id).await?;
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
                let mut filter = StudentFeeFilter::default();
                filter.student_id = Some(sid);
                let fees = self.finance_service.list_student_fees(ctx.tenant_id, filter).await?;
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

                let (job, _) = self.report_service.generate_report(ctx, input).await?;
                Ok(json!({
                    "job_id": job.id,
                    "status": format!("{:?}", job.status),
                    "report_code": job.report_code
                }))
            }
            "get_org_overview" => {
                let org = self.org_service.get_organization(ctx.tenant_id).await?;
                let units = self.org_service.list_org_units(ctx.tenant_id).await?;
                let counts = self.org_service.get_dashboard_counts(ctx.tenant_id).await.ok();

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
                let ap = self.autopilot_service.as_ref()
                    .ok_or_else(|| DomainError::Internal("Autopilot service not available".into()))?;
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
                let ap = self.autopilot_service.as_ref()
                    .ok_or_else(|| DomainError::Internal("Autopilot service not available".into()))?;
                let status_filter = arguments.get("status").and_then(|v| v.as_str());
                let (tasks, total) = ap.list_tasks(ctx, ctx.tenant_id, status_filter, None, 20, 0).await?;
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
                let ap = self.autopilot_service.as_ref()
                    .ok_or_else(|| DomainError::Internal("Autopilot service not available".into()))?;
                let dry_run = arguments.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
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
            _ => Err(DomainError::Validation(format!("Tool '{}' not supported", tool_name))),
        }
    }

    /// Build dynamic system prompt with tenant and screen context.
    async fn build_system_prompt(
        &self,
        ctx: &RequestContext,
        screen_context: Option<&serde_json::Value>,
    ) -> String {
        let user_repr = match &ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => format!("User ({})", uid),
            orbis_domain::iam::actor::Actor::Agent(aid) => format!("Agent ({})", aid),
            orbis_domain::iam::actor::Actor::Autopilot => "Autopilot Engine".to_string(),
            orbis_domain::iam::actor::Actor::System => "System".to_string(),
        };

        let mut prompt = format!(
            "You are Orbis AI, the intelligent, authoritative assistant for the Orbis Education Platform.\n\
             Tenant Organization ID: {}\n\
             Active User: {}\n\
             Language: Respond in Arabic or English based on user query language.\n\
             Tone: Professional, helpful, precise, educational, and secure.\n\
             Capabilities: You can inspect student profiles, check grades, analyze attendance, query financial accounts and fees, trigger report generation, and view organization statistics.\n\
             Safety Rules:\n\
             1. Always verify and execute domain tools to provide authentic real-time data.\n\
             2. Never make up data or IDs.\n\
             3. If a tool proposes a consequential state modification, inform the user clearly that confirmation is required in their Cockpit.",
            ctx.tenant_id,
            user_repr
        );

        if let Some(screen) = screen_context {
            prompt.push_str(&format!(
                "\n\nActive Screen Context:\n```json\n{}\n```",
                screen
            ));
        }

        prompt
    }
}
