//! Action Registry — In-memory catalog of all callable AI actions (ADR-005).
//!
//! Built once at startup from hardcoded registration functions.
//! No DB queries needed — O(1) lookup by action code.
//!
//! Memory footprint: ~50KB for ~100 actions × 500B metadata.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use shared_kernel::errors::AppError;
use shared_kernel::types::RiskLevel;

/// Result type for action handlers.
pub type ActionResult = Result<Value, AppError>;

/// Async action handler function signature.
pub type ActionHandlerFn = Arc<
    dyn Fn(Value) -> Pin<Box<dyn Future<Output = ActionResult> + Send>> + Send + Sync,
>;

/// Definition of a callable action — metadata used by AI for discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDefinition {
    /// Unique action code: "domain.operation" (e.g., "enrollment.create")
    pub code: String,
    /// Human-readable name.
    pub name: String,
    /// Name in Arabic.
    pub name_ar: Option<String>,
    /// Description of what this action does.
    pub description: String,
    /// Description in Arabic.
    pub description_ar: Option<String>,
    /// Domain this action belongs to.
    pub domain: String,
    /// Required permissions to execute.
    pub permissions: Vec<String>,
    /// Risk level of this action.
    pub risk_level: RiskLevel,
    /// Whether this action requires explicit human approval.
    pub requires_approval: bool,
    /// Whether this action can be reversed.
    pub is_reversible: bool,
    /// JSON Schema for expected input parameters.
    pub input_schema: Option<Value>,
    /// JSON Schema for expected output.
    pub output_schema: Option<Value>,
    /// Tags for categorization and discovery.
    pub tags: Vec<String>,
    /// Example input for documentation / AI prompts.
    pub example_input: Option<Value>,
}

/// A registered action — definition + handler.
pub struct RegisteredAction {
    pub definition: ActionDefinition,
    pub handler: ActionHandlerFn,
}

/// In-memory registry of all callable actions.
pub struct ActionRegistry {
    /// code → RegisteredAction (O(1) lookup)
    actions: HashMap<String, RegisteredAction>,
    /// domain → list of action codes (for discovery filtering)
    by_domain: HashMap<String, Vec<String>>,
    /// tag → list of action codes
    by_tag: HashMap<String, Vec<String>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            by_domain: HashMap::new(),
            by_tag: HashMap::new(),
        }
    }

    /// Register a single action.
    pub fn register(&mut self, definition: ActionDefinition, handler: ActionHandlerFn) {
        let code = definition.code.clone();
        let domain = definition.domain.clone();
        let tags = definition.tags.clone();

        // Index by domain
        self.by_domain
            .entry(domain)
            .or_default()
            .push(code.clone());

        // Index by tags
        for tag in &tags {
            self.by_tag
                .entry(tag.clone())
                .or_default()
                .push(code.clone());
        }

        self.actions.insert(
            code.clone(),
            RegisteredAction {
                definition,
                handler,
            },
        );
    }

    /// Look up an action by code. O(1).
    pub fn find(&self, code: &str) -> Option<&RegisteredAction> {
        self.actions.get(code)
    }

    /// Discover actions with optional filters.
    pub fn discover(
        &self,
        domain: Option<&str>,
        tags: Option<&[&str]>,
    ) -> Vec<&ActionDefinition> {
        if domain.is_none() && tags.is_none() {
            // Return all
            return self.actions.values().map(|a| &a.definition).collect();
        }

        let mut candidates: Option<Vec<&str>> = None;

        // Filter by domain
        if let Some(d) = domain {
            if let Some(codes) = self.by_domain.get(d) {
                candidates = Some(codes.iter().map(|s| s.as_str()).collect());
            } else {
                return vec![];
            }
        }

        // Filter by tags (intersection)
        if let Some(tag_list) = tags {
            for tag in tag_list {
                if let Some(codes) = self.by_tag.get(*tag) {
                    let tag_set: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
                    candidates = Some(match candidates {
                        Some(existing) => existing
                            .into_iter()
                            .filter(|c| tag_set.contains(c))
                            .collect(),
                        None => tag_set,
                    });
                } else {
                    return vec![];
                }
            }
        }

        match candidates {
            Some(codes) => codes
                .iter()
                .filter_map(|c| self.actions.get(*c).map(|a| &a.definition))
                .collect(),
            None => vec![],
        }
    }

    /// Total number of registered actions.
    pub fn count(&self) -> usize {
        self.actions.len()
    }

    /// List all domain names that have registered actions.
    pub fn domains(&self) -> Vec<&str> {
        self.by_domain.keys().map(|s| s.as_str()).collect()
    }

    /// Build the full registry from all domain registration functions.
    ///
    /// Called once at application startup.
    pub fn build() -> Self {
        let mut registry = Self::new();

        register_iam_actions(&mut registry);
        register_academic_actions(&mut registry);
        register_assessment_actions(&mut registry);
        register_lifecycle_actions(&mut registry);
        register_finance_actions(&mut registry);
        register_scheduling_actions(&mut registry);
        register_exam_actions(&mut registry);

        info!(
            count = registry.count(),
            domains = ?registry.domains(),
            "Action registry built"
        );

        registry
    }
}

// ═══════════════════════════════════════════════════════════════
// Domain Action Registrations
//
// Each function registers actions for one domain.
// The handler is a placeholder that returns a stub — actual domain
// service wiring happens when the ActionExecutor receives a
// ServiceRegistry reference.
// ═══════════════════════════════════════════════════════════════

fn register_iam_actions(reg: &mut ActionRegistry) {
    reg.register(
        ActionDefinition {
            code: "iam.user.list".into(),
            name: "List Users".into(),
            name_ar: Some("عرض المستخدمين".into()),
            description: "List all users in the organization with optional filters".into(),
            description_ar: Some("عرض جميع المستخدمين في المنظمة مع فلاتر اختيارية".into()),
            domain: "iam".into(),
            permissions: vec!["users.read".into()],
            risk_level: RiskLevel::Low,
            requires_approval: false,
            is_reversible: false,
            input_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "role": { "type": "string", "enum": ["admin", "teacher", "student", "parent"] },
                    "search": { "type": "string" },
                    "limit": { "type": "integer", "default": 50 }
                }
            })),
            output_schema: None,
            tags: vec!["read".into(), "users".into()],
            example_input: Some(serde_json::json!({ "role": "teacher", "limit": 20 })),
        },
        Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
    );

    reg.register(
        ActionDefinition {
            code: "iam.user.get".into(),
            name: "Get User".into(),
            name_ar: Some("جلب مستخدم".into()),
            description: "Get a single user by ID with full profile".into(),
            description_ar: Some("جلب مستخدم واحد بالمعرف مع الملف الشخصي الكامل".into()),
            domain: "iam".into(),
            permissions: vec!["users.read".into()],
            risk_level: RiskLevel::Low,
            requires_approval: false,
            is_reversible: false,
            input_schema: Some(serde_json::json!({
                "type": "object",
                "properties": { "user_id": { "type": "string" } },
                "required": ["user_id"]
            })),
            output_schema: None,
            tags: vec!["read".into(), "users".into()],
            example_input: Some(serde_json::json!({ "user_id": "usr_abc123" })),
        },
        Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
    );

    reg.register(
        ActionDefinition {
            code: "iam.user.update_role".into(),
            name: "Update User Role".into(),
            name_ar: Some("تحديث دور المستخدم".into()),
            description: "Change a user's role (requires admin permission)".into(),
            description_ar: Some("تغيير دور المستخدم (يتطلب صلاحية مدير)".into()),
            domain: "iam".into(),
            permissions: vec!["users.write".into(), "roles.assign".into()],
            risk_level: RiskLevel::High,
            requires_approval: true,
            is_reversible: true,
            input_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "user_id": { "type": "string" },
                    "new_role": { "type": "string" }
                },
                "required": ["user_id", "new_role"]
            })),
            output_schema: None,
            tags: vec!["write".into(), "users".into(), "roles".into()],
            example_input: Some(serde_json::json!({ "user_id": "usr_abc123", "new_role": "teacher" })),
        },
        Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
    );
}

fn register_academic_actions(reg: &mut ActionRegistry) {
    for (code, name, name_ar, desc, risk, tags) in [
        ("academic.enrollment.list", "List Enrollments", "عرض التسجيلات", "List student enrollments with filters", RiskLevel::Low, vec!["read", "enrollments"]),
        ("academic.enrollment.create", "Create Enrollment", "إنشاء تسجيل", "Enroll a student in a class", RiskLevel::Medium, vec!["write", "enrollments"]),
        ("academic.enrollment.transfer", "Transfer Student", "نقل طالب", "Transfer student between classes", RiskLevel::Medium, vec!["write", "enrollments"]),
        ("academic.enrollment.withdraw", "Withdraw Student", "سحب طالب", "Withdraw a student from enrollment", RiskLevel::High, vec!["write", "enrollments"]),
        ("academic.class.list", "List Classes", "عرض الفصول", "List all classes with metadata", RiskLevel::Low, vec!["read", "classes"]),
        ("academic.subject.list", "List Subjects", "عرض المواد", "List all subjects", RiskLevel::Low, vec!["read", "subjects"]),
    ] {
        reg.register(
            ActionDefinition {
                code: code.into(),
                name: name.into(),
                name_ar: Some(name_ar.into()),
                description: desc.into(),
                description_ar: None,
                domain: "academic".into(),
                permissions: vec![],
                risk_level: risk,
                requires_approval: risk >= RiskLevel::High,
                is_reversible: risk >= RiskLevel::Medium,
                input_schema: None,
                output_schema: None,
                tags: tags.into_iter().map(String::from).collect(),
                example_input: None,
            },
            Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
        );
    }
}

fn register_assessment_actions(reg: &mut ActionRegistry) {
    for (code, name, name_ar, desc, risk, tags) in [
        ("assessment.grade.list", "List Grades", "عرض الدرجات", "List grades for a class/subject", RiskLevel::Low, vec!["read", "grades"]),
        ("assessment.grade.enter", "Enter Grades", "إدخال درجات", "Enter grades for students in bulk", RiskLevel::Medium, vec!["write", "grades"]),
        ("assessment.attendance.list", "List Attendance", "عرض الحضور", "List attendance records", RiskLevel::Low, vec!["read", "attendance"]),
        ("assessment.attendance.record", "Record Attendance", "تسجيل حضور", "Record daily attendance", RiskLevel::Low, vec!["write", "attendance"]),
        ("assessment.behavior.list", "List Behavior Records", "عرض السلوك", "List behavior records", RiskLevel::Low, vec!["read", "behavior"]),
        ("assessment.behavior.create", "Create Behavior Record", "إنشاء سجل سلوك", "Record a behavior incident", RiskLevel::Medium, vec!["write", "behavior"]),
    ] {
        reg.register(
            ActionDefinition {
                code: code.into(),
                name: name.into(),
                name_ar: Some(name_ar.into()),
                description: desc.into(),
                description_ar: None,
                domain: "assessment".into(),
                permissions: vec![],
                risk_level: risk,
                requires_approval: false,
                is_reversible: risk >= RiskLevel::Medium,
                input_schema: None,
                output_schema: None,
                tags: tags.into_iter().map(String::from).collect(),
                example_input: None,
            },
            Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
        );
    }
}

fn register_lifecycle_actions(reg: &mut ActionRegistry) {
    for (code, name, name_ar, desc, risk, tags) in [
        ("lifecycle.period.list", "List Periods", "عرض الدورات", "List academic periods", RiskLevel::Low, vec!["read", "periods"]),
        ("lifecycle.period.lock", "Lock Period", "قفل الدورة", "Lock an academic period (irreversible)", RiskLevel::Critical, vec!["write", "periods"]),
        ("lifecycle.calendar.list", "List Calendar Events", "عرض التقويم", "List calendar events", RiskLevel::Low, vec!["read", "calendar"]),
        ("lifecycle.promotion.run", "Run Promotion", "تنفيذ الترفيع", "Execute student promotion to next year", RiskLevel::Critical, vec!["write", "promotion"]),
    ] {
        reg.register(
            ActionDefinition {
                code: code.into(),
                name: name.into(),
                name_ar: Some(name_ar.into()),
                description: desc.into(),
                description_ar: None,
                domain: "lifecycle".into(),
                permissions: vec![],
                risk_level: risk,
                requires_approval: risk >= RiskLevel::High,
                is_reversible: false,
                input_schema: None,
                output_schema: None,
                tags: tags.into_iter().map(String::from).collect(),
                example_input: None,
            },
            Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
        );
    }
}

fn register_finance_actions(reg: &mut ActionRegistry) {
    for (code, name, name_ar, desc, risk, tags) in [
        ("finance.fee.list", "List Fees", "عرض الرسوم", "List fee templates and student fees", RiskLevel::Low, vec!["read", "fees"]),
        ("finance.fee.assign", "Assign Fee", "تعيين رسوم", "Assign a fee template to students", RiskLevel::Medium, vec!["write", "fees"]),
        ("finance.payment.list", "List Payments", "عرض المدفوعات", "List payment records", RiskLevel::Low, vec!["read", "payments"]),
        ("finance.payment.record", "Record Payment", "تسجيل دفعة", "Record a payment from a student", RiskLevel::Medium, vec!["write", "payments"]),
        ("finance.treasury.balance", "Treasury Balance", "رصيد الخزينة", "Get current treasury balance", RiskLevel::Low, vec!["read", "treasury"]),
        ("finance.report.generate", "Generate Financial Report", "تقرير مالي", "Generate a financial summary report", RiskLevel::Low, vec!["read", "reports"]),
    ] {
        reg.register(
            ActionDefinition {
                code: code.into(),
                name: name.into(),
                name_ar: Some(name_ar.into()),
                description: desc.into(),
                description_ar: None,
                domain: "finance".into(),
                permissions: vec![],
                risk_level: risk,
                requires_approval: false,
                is_reversible: risk >= RiskLevel::Medium,
                input_schema: None,
                output_schema: None,
                tags: tags.into_iter().map(String::from).collect(),
                example_input: None,
            },
            Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
        );
    }
}

fn register_scheduling_actions(reg: &mut ActionRegistry) {
    for (code, name, name_ar, desc, risk, tags) in [
        ("scheduling.timetable.generate", "Generate Timetable", "إنشاء جدول حصص", "Auto-generate optimal timetable", RiskLevel::High, vec!["write", "timetable"]),
        ("scheduling.timetable.view", "View Timetable", "عرض الجدول", "View current timetable", RiskLevel::Low, vec!["read", "timetable"]),
    ] {
        reg.register(
            ActionDefinition {
                code: code.into(),
                name: name.into(),
                name_ar: Some(name_ar.into()),
                description: desc.into(),
                description_ar: None,
                domain: "scheduling".into(),
                permissions: vec![],
                risk_level: risk,
                requires_approval: risk >= RiskLevel::High,
                is_reversible: false,
                input_schema: None,
                output_schema: None,
                tags: tags.into_iter().map(String::from).collect(),
                example_input: None,
            },
            Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
        );
    }
}

fn register_exam_actions(reg: &mut ActionRegistry) {
    for (code, name, name_ar, desc, risk, tags) in [
        ("exam.session.list", "List Exam Sessions", "عرض جلسات الاختبار", "List exam sessions", RiskLevel::Low, vec!["read", "exams"]),
        ("exam.session.create", "Create Exam Session", "إنشاء جلسة اختبار", "Create a new exam session", RiskLevel::Medium, vec!["write", "exams"]),
        ("exam.numbers.assign", "Assign Exam Numbers", "توزيع أرقام الجلوس", "Auto-assign seating/secret numbers", RiskLevel::High, vec!["write", "exams"]),
        ("exam.schedule.generate", "Auto-Schedule Exams", "جدولة تلقائية", "Auto-generate exam schedule", RiskLevel::High, vec!["write", "exams"]),
    ] {
        reg.register(
            ActionDefinition {
                code: code.into(),
                name: name.into(),
                name_ar: Some(name_ar.into()),
                description: desc.into(),
                description_ar: None,
                domain: "exam".into(),
                permissions: vec![],
                risk_level: risk,
                requires_approval: risk >= RiskLevel::High,
                is_reversible: false,
                input_schema: None,
                output_schema: None,
                tags: tags.into_iter().map(String::from).collect(),
                example_input: None,
            },
            Arc::new(|_params| Box::pin(async { Ok(serde_json::json!({ "stub": true })) })),
        );
    }
}
