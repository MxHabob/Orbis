import * as fs from "fs";
import * as path from "path";

// Define the absolute path for target Rust entities folder
const targetDir = path.resolve(__dirname, "../apps/api-rust/src/entities");

interface Column {
  name: string;
  type: string;
  isNullable: boolean;
  isPrimaryKey?: boolean;
}

interface Table {
  name: string;
  structName: string; // e.g. UserAccount
  columns: Column[];
}

const tables: Table[] = [
  // ─────────────────────────────────────────────────────────────────────────────
  // CORE SCHEMA TABLES
  // ─────────────────────────────────────────────────────────────────────────────
  {
    name: "user_accounts",
    structName: "UserAccount",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "user_id", type: "uuid", isNullable: false },
      { name: "provider", type: "varchar", isNullable: false },
      { name: "provider_account_id", type: "varchar", isNullable: false },
      { name: "provider_email", type: "varchar", isNullable: true },
      { name: "provider_name", type: "varchar", isNullable: true },
      { name: "provider_picture", type: "varchar", isNullable: true },
      { name: "access_token", type: "text", isNullable: true },
      { name: "refresh_token", type: "text", isNullable: true },
      { name: "id_token", type: "text", isNullable: true },
      { name: "expires_at", type: "timestamp", isNullable: true },
      { name: "scope", type: "text", isNullable: true },
      { name: "token_type", type: "varchar", isNullable: true },
      { name: "meta", type: "jsonb", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "user_roles",
    structName: "UserRole",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "user_id", type: "uuid", isNullable: false },
      { name: "role", type: "varchar", isNullable: false },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "scope_type", type: "varchar", isNullable: false },
      { name: "scope_id", type: "uuid", isNullable: true },
      { name: "valid_from", type: "date", isNullable: true },
      { name: "valid_until", type: "date", isNullable: true },
      { name: "is_primary", type: "boolean", isNullable: false },
      { name: "assigned_by", type: "uuid", isNullable: true },
      { name: "assigned_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "permissions",
    structName: "Permission",
    columns: [
      { name: "id", type: "integer", isNullable: false, isPrimaryKey: true },
      { name: "code", type: "varchar", isNullable: false },
      { name: "name", type: "varchar", isNullable: false },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "description", type: "text", isNullable: true },
      { name: "module", type: "varchar", isNullable: false },
      { name: "action", type: "varchar", isNullable: false },
      { name: "resource", type: "varchar", isNullable: false },
      { name: "requires_scope", type: "boolean", isNullable: false },
      { name: "scope_types", type: "jsonb", isNullable: true },
      { name: "is_system", type: "boolean", isNullable: false },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "role_permissions",
    structName: "RolePermission",
    columns: [
      { name: "role", type: "varchar", isNullable: false, isPrimaryKey: true },
      { name: "permission_id", type: "integer", isNullable: false, isPrimaryKey: true },
      { name: "conditions", type: "jsonb", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "user_permissions",
    structName: "UserPermission",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "user_id", type: "uuid", isNullable: false },
      { name: "permission_id", type: "integer", isNullable: false },
      { name: "grant_type", type: "varchar", isNullable: false },
      { name: "conditions", type: "jsonb", isNullable: true },
      { name: "valid_from", type: "date", isNullable: true },
      { name: "valid_until", type: "date", isNullable: true },
      { name: "reason", type: "text", isNullable: true },
      { name: "granted_by", type: "uuid", isNullable: true },
      { name: "granted_at", type: "timestamp", isNullable: false },
      { name: "is_active", type: "boolean", isNullable: false },
    ]
  },
  {
    name: "verifications",
    structName: "Verification",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "user_id", type: "uuid", isNullable: true },
      { name: "identifier", type: "varchar", isNullable: false },
      { name: "value", type: "varchar", isNullable: false },
      { name: "type", type: "varchar", isNullable: false },
      { name: "attempts", type: "integer", isNullable: true },
      { name: "max_attempts", type: "integer", isNullable: true },
      { name: "expires_at", type: "timestamp", isNullable: false },
      { name: "used_at", type: "timestamp", isNullable: true },
      { name: "ip_address", type: "varchar", isNullable: true },
      { name: "user_agent", type: "text", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "notification_preferences",
    structName: "NotificationPreference",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "user_id", type: "uuid", isNullable: false },
      { name: "notification_type", type: "varchar", isNullable: false },
      { name: "in_app_enabled", type: "boolean", isNullable: false },
      { name: "email_enabled", type: "boolean", isNullable: false },
      { name: "sms_enabled", type: "boolean", isNullable: false },
      { name: "push_enabled", type: "boolean", isNullable: false },
      { name: "whatsapp_enabled", type: "boolean", isNullable: false },
      { name: "quiet_hours_enabled", type: "boolean", isNullable: false },
      { name: "quiet_hours_start", type: "varchar", isNullable: true },
      { name: "quiet_hours_end", type: "varchar", isNullable: true },
      { name: "digest_mode", type: "varchar", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "documents",
    structName: "Document",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "owner_id", type: "uuid", isNullable: false },
      { name: "uploaded_by", type: "uuid", isNullable: true },
      { name: "name", type: "varchar", isNullable: false },
      { name: "title", type: "varchar", isNullable: true },
      { name: "description", type: "text", isNullable: true },
      { name: "type", type: "varchar", isNullable: false },
      { name: "visibility", type: "varchar", isNullable: false },
      { name: "size", type: "integer", isNullable: true },
      { name: "mime_type", type: "varchar", isNullable: true },
      { name: "extension", type: "varchar", isNullable: true },
      { name: "storage_provider", type: "varchar", isNullable: false },
      { name: "bucket", type: "varchar", isNullable: true },
      { name: "key", type: "text", isNullable: false },
      { name: "url", type: "text", isNullable: true },
      { name: "thumbnail_url", type: "text", isNullable: true },
      { name: "parent_id", type: "uuid", isNullable: true },
      { name: "path", type: "text", isNullable: true },
      { name: "depth", type: "integer", isNullable: true },
      { name: "shared_with", type: "jsonb", isNullable: true },
      { name: "share_link", type: "text", isNullable: true },
      { name: "share_link_expires_at", type: "timestamp", isNullable: true },
      { name: "share_password", type: "varchar", isNullable: true },
      { name: "version", type: "integer", isNullable: true },
      { name: "previous_version_id", type: "uuid", isNullable: true },
      { name: "is_latest_version", type: "boolean", isNullable: false },
      { name: "processing_status", type: "varchar", isNullable: true },
      { name: "ocr_text", type: "text", isNullable: true },
      { name: "extracted_data", type: "jsonb", isNullable: true },
      { name: "is_pinned", type: "boolean", isNullable: false },
      { name: "is_starred", type: "boolean", isNullable: false },
      { name: "is_template", type: "boolean", isNullable: false },
      { name: "tags", type: "jsonb", isNullable: true },
      { name: "meta", type: "jsonb", isNullable: true },
      { name: "deleted_at", type: "timestamp", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  // ─────────────────────────────────────────────────────────────────────────────
  // ACADEMIC SCHEMA TABLES
  // ─────────────────────────────────────────────────────────────────────────────
  {
    name: "subjects",
    structName: "Subject",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "unit_id", type: "uuid", isNullable: true },
      { name: "code", type: "varchar", isNullable: false },
      { name: "name", type: "varchar", isNullable: false },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "name_en", type: "varchar", isNullable: true },
      { name: "description", type: "text", isNullable: true },
      { name: "short_description", type: "varchar", isNullable: true },
      { name: "is_core", type: "boolean", isNullable: false },
      { name: "is_elective", type: "boolean", isNullable: false },
      { name: "credit_hours", type: "integer", isNullable: true },
      { name: "requires_lab", type: "boolean", isNullable: false },
      { name: "requires_practical", type: "boolean", isNullable: false },
      { name: "max_students_per_class", type: "integer", isNullable: true },
      { name: "default_grade_groups", type: "jsonb", isNullable: true },
      { name: "prerequisites", type: "jsonb", isNullable: true },
      { name: "learning_outcomes", type: "jsonb", isNullable: true },
      { name: "color", type: "varchar", isNullable: true },
      { name: "icon", type: "varchar", isNullable: true },
      { name: "cover_image", type: "varchar", isNullable: true },
      { name: "sort_order", type: "integer", isNullable: true },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "class_subjects",
    structName: "ClassSubject",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "class_id", type: "uuid", isNullable: false },
      { name: "subject_id", type: "uuid", isNullable: false },
      { name: "cycle_id", type: "uuid", isNullable: false },
      { name: "primary_teacher_id", type: "uuid", isNullable: false },
      { name: "assistant_teacher_id", type: "uuid", isNullable: true },
      { name: "substitute_teacher_id", type: "uuid", isNullable: true },
      { name: "grade_groups", type: "jsonb", isNullable: true },
      { name: "weekly_hours", type: "integer", isNullable: true },
      { name: "theory_hours", type: "integer", isNullable: true },
      { name: "practical_hours", type: "integer", isNullable: true },
      { name: "assessment_config", type: "jsonb", isNullable: true },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "student_guardians",
    structName: "StudentGuardian",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "student_id", type: "uuid", isNullable: false },
      { name: "guardian_id", type: "uuid", isNullable: false },
      { name: "relationship", type: "varchar", isNullable: false },
      { name: "relationship_ar", type: "varchar", isNullable: true },
      { name: "is_primary", type: "boolean", isNullable: false },
      { name: "is_emergency_contact", type: "boolean", isNullable: false },
      { name: "is_authorized_pickup", type: "boolean", isNullable: false },
      { name: "can_view_grades", type: "boolean", isNullable: false },
      { name: "can_view_attendance", type: "boolean", isNullable: false },
      { name: "can_view_behavior", type: "boolean", isNullable: false },
      { name: "can_view_finance", type: "boolean", isNullable: false },
      { name: "can_view_schedule", type: "boolean", isNullable: false },
      { name: "can_receive_notifications", type: "boolean", isNullable: false },
      { name: "is_payer", type: "boolean", isNullable: false },
      { name: "payment_share_percent", type: "double", isNullable: true },
      { name: "occupation", type: "varchar", isNullable: true },
      { name: "employer", type: "varchar", isNullable: true },
      { name: "work_phone", type: "varchar", isNullable: true },
      { name: "notes", type: "text", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "time_slots",
    structName: "TimeSlot",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "cycle_id", type: "uuid", isNullable: false },
      { name: "slot_number", type: "integer", isNullable: false },
      { name: "name", type: "varchar", isNullable: true },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "start_time", type: "varchar", isNullable: false },
      { name: "end_time", type: "varchar", isNullable: false },
      { name: "duration_minutes", type: "integer", isNullable: false },
      { name: "is_break", type: "boolean", isNullable: false },
      { name: "break_type", type: "varchar", isNullable: true },
      { name: "applicable_days", type: "jsonb", isNullable: true },
      { name: "sort_order", type: "integer", isNullable: true },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "scheduled_lessons",
    structName: "ScheduledLesson",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "cycle_id", type: "uuid", isNullable: false },
      { name: "class_id", type: "uuid", isNullable: false },
      { name: "class_subject_id", type: "uuid", isNullable: false },
      { name: "subject_id", type: "uuid", isNullable: false },
      { name: "teacher_id", type: "uuid", isNullable: false },
      { name: "substitute_teacher_id", type: "uuid", isNullable: true },
      { name: "time_slot_id", type: "uuid", isNullable: false },
      { name: "day_of_week", type: "integer", isNullable: false },
      { name: "room_id", type: "uuid", isNullable: true },
      { name: "room_name", type: "varchar", isNullable: true },
      { name: "status", type: "varchar", isNullable: false },
      { name: "is_substituted", type: "boolean", isNullable: false },
      { name: "substitution_reason", type: "varchar", isNullable: true },
      { name: "topic", type: "varchar", isNullable: true },
      { name: "topic_ar", type: "varchar", isNullable: true },
      { name: "objectives", type: "text", isNullable: true },
      { name: "materials", type: "jsonb", isNullable: true },
      { name: "homework", type: "text", isNullable: true },
      { name: "homework_due_date", type: "date", isNullable: true },
      { name: "attendance_taken", type: "boolean", isNullable: false },
      { name: "attendance_taken_at", type: "timestamp", isNullable: true },
      { name: "attendance_taken_by", type: "uuid", isNullable: true },
      { name: "attendance_method", type: "varchar", isNullable: true },
      { name: "notes", type: "text", isNullable: true },
      { name: "cancellation_reason", type: "text", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "assessments",
    structName: "Assessment",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "cycle_id", type: "uuid", isNullable: false },
      { name: "class_subject_id", type: "uuid", isNullable: false },
      { name: "title", type: "varchar", isNullable: false },
      { name: "title_ar", type: "varchar", isNullable: true },
      { name: "description", type: "text", isNullable: true },
      { name: "type", type: "varchar", isNullable: false },
      { name: "max_score", type: "double", isNullable: false },
      { name: "passing_score", type: "double", isNullable: true },
      { name: "weight", type: "double", isNullable: false },
      { name: "grade_group_name", type: "varchar", isNullable: false },
      { name: "assessment_date", type: "date", isNullable: true },
      { name: "due_date", type: "date", isNullable: true },
      { name: "duration_minutes", type: "integer", isNullable: true },
      { name: "is_published", type: "boolean", isNullable: false },
      { name: "published_at", type: "timestamp", isNullable: true },
      { name: "published_by", type: "uuid", isNullable: true },
      { name: "is_locked", type: "boolean", isNullable: false },
      { name: "locked_at", type: "timestamp", isNullable: true },
      { name: "locked_by", type: "uuid", isNullable: true },
      { name: "grading_method", type: "varchar", isNullable: true },
      { name: "rubric_id", type: "uuid", isNullable: true },
      { name: "instructions", type: "text", isNullable: true },
      { name: "instructions_ar", type: "text", isNullable: true },
      { name: "attachments", type: "jsonb", isNullable: true },
      { name: "created_by", type: "uuid", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "grade_history",
    structName: "GradeHistory",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "grade_id", type: "uuid", isNullable: false },
      { name: "score", type: "double", isNullable: true },
      { name: "grade_letter", type: "varchar", isNullable: true },
      { name: "is_absent", type: "boolean", isNullable: true },
      { name: "is_excused", type: "boolean", isNullable: true },
      { name: "comment", type: "varchar", isNullable: true },
      { name: "changed_by", type: "uuid", isNullable: true },
      { name: "changed_at", type: "timestamp", isNullable: false },
      { name: "change_reason", type: "varchar", isNullable: true },
      { name: "change_type", type: "varchar", isNullable: false },
      { name: "snapshot", type: "jsonb", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "attendance_daily",
    structName: "AttendanceDaily",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "cycle_id", type: "uuid", isNullable: false },
      { name: "enrollment_id", type: "uuid", isNullable: false },
      { name: "date", type: "date", isNullable: false },
      { name: "status", type: "varchar", isNullable: false },
      { name: "lessons", type: "jsonb", isNullable: true },
      { name: "total_lessons", type: "integer", isNullable: true },
      { name: "present_lessons", type: "integer", isNullable: true },
      { name: "absent_lessons", type: "integer", isNullable: true },
      { name: "late_lessons", type: "integer", isNullable: true },
      { name: "excused_lessons", type: "integer", isNullable: true },
      { name: "late_minutes", type: "integer", isNullable: true },
      { name: "excuse_type", type: "varchar", isNullable: true },
      { name: "excuse_document", type: "varchar", isNullable: true },
      { name: "excuse_notes", type: "varchar", isNullable: true },
      { name: "is_verified", type: "boolean", isNullable: false },
      { name: "verified_by", type: "uuid", isNullable: true },
      { name: "verified_at", type: "timestamp", isNullable: true },
      { name: "marked_by", type: "uuid", isNullable: false },
      { name: "marked_at", type: "timestamp", isNullable: false },
      { name: "mark_method", type: "varchar", isNullable: true },
      { name: "device_id", type: "varchar", isNullable: true },
      { name: "location", type: "jsonb", isNullable: true },
      { name: "notes", type: "text", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "attendance_summary",
    structName: "AttendanceSummary",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "enrollment_id", type: "uuid", isNullable: false },
      { name: "cycle_id", type: "uuid", isNullable: false },
      { name: "total_lessons", type: "integer", isNullable: true },
      { name: "present_lessons", type: "integer", isNullable: true },
      { name: "absent_lessons", type: "integer", isNullable: true },
      { name: "late_lessons", type: "integer", isNullable: true },
      { name: "excused_lessons", type: "integer", isNullable: true },
      { name: "late_minutes", type: "integer", isNullable: true },
      { name: "attendance_rate", type: "double", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  // ─────────────────────────────────────────────────────────────────────────────
  // FINANCE SCHEMA TABLES
  // ─────────────────────────────────────────────────────────────────────────────
  {
    name: "ledger_accounts",
    structName: "LedgerAccount",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "code", type: "varchar", isNullable: false },
      { name: "name", type: "varchar", isNullable: false },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "type", type: "varchar", isNullable: false },
      { name: "sub_type", type: "varchar", isNullable: true },
      { name: "parent_id", type: "uuid", isNullable: true },
      { name: "path", type: "varchar", isNullable: false },
      { name: "depth", type: "integer", isNullable: false },
      { name: "currency", type: "varchar", isNullable: false },
      { name: "opening_balance", type: "double", isNullable: true },
      { name: "current_balance", type: "double", isNullable: true },
      { name: "is_bank_account", type: "boolean", isNullable: false },
      { name: "is_cash_account", type: "boolean", isNullable: false },
      { name: "is_control_account", type: "boolean", isNullable: false },
      { name: "external_ref", type: "jsonb", isNullable: true },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "journal_entries",
    structName: "JournalEntry",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "transaction_ref", type: "varchar", isNullable: false },
      { name: "accounting_date", type: "date", isNullable: false },
      { name: "fiscal_period_id", type: "uuid", isNullable: false },
      { name: "amount", type: "double", isNullable: false },
      { name: "debit_account_id", type: "uuid", isNullable: false },
      { name: "credit_account_id", type: "uuid", isNullable: false },
      { name: "currency", type: "varchar", isNullable: false },
      { name: "exchange_rate", type: "double", isNullable: true },
      { name: "base_currency_amount", type: "double", isNullable: true },
      { name: "source_type", type: "varchar", isNullable: false },
      { name: "source_id", type: "uuid", isNullable: true },
      { name: "source_module", type: "varchar", isNullable: true },
      { name: "description", type: "text", isNullable: false },
      { name: "description_ar", type: "text", isNullable: true },
      { name: "metadata", type: "jsonb", isNullable: true },
      { name: "is_reversed", type: "boolean", isNullable: false },
      { name: "reversed_by", type: "uuid", isNullable: true },
      { name: "reversal_reason", type: "text", isNullable: true },
      { name: "is_reconciled", type: "boolean", isNullable: false },
      { name: "reconciled_at", type: "timestamp", isNullable: true },
      { name: "reconciliation_ref", type: "varchar", isNullable: true },
      { name: "status", type: "varchar", isNullable: false },
      { name: "posted_by", type: "uuid", isNullable: true },
      { name: "posted_at", type: "timestamp", isNullable: true },
      { name: "approved_by", type: "uuid", isNullable: true },
      { name: "approved_at", type: "timestamp", isNullable: true },
      { name: "created_by", type: "uuid", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "fee_templates",
    structName: "FeeTemplate",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "name", type: "varchar", isNullable: false },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "code", type: "varchar", isNullable: true },
      { name: "category", type: "varchar", isNullable: false },
      { name: "revenue_account_id", type: "uuid", isNullable: false },
      { name: "receivable_account_id", type: "uuid", isNullable: false },
      { name: "pricing_type", type: "varchar", isNullable: false },
      { name: "pricing_config", type: "jsonb", isNullable: true },
      { name: "applicability_rules", type: "jsonb", isNullable: true },
      { name: "frequency", type: "varchar", isNullable: false },
      { name: "frequency_config", type: "jsonb", isNullable: true },
      { name: "late_fee_rules", type: "jsonb", isNullable: true },
      { name: "allowed_discounts", type: "jsonb", isNullable: true },
      { name: "is_mandatory", type: "boolean", isNullable: false },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "sort_order", type: "integer", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "discount_templates",
    structName: "DiscountTemplate",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "name", type: "varchar", isNullable: false },
      { name: "code", type: "varchar", isNullable: true },
      { name: "type", type: "varchar", isNullable: false },
      { name: "calculation_method", type: "varchar", isNullable: false },
      { name: "value", type: "integer", isNullable: false },
      { name: "eligibility_rules", type: "jsonb", isNullable: true },
      { name: "limits", type: "jsonb", isNullable: true },
      { name: "requires_approval", type: "boolean", isNullable: false },
      { name: "approver_roles", type: "jsonb", isNullable: true },
      { name: "is_auto_apply", type: "boolean", isNullable: false },
      { name: "is_stackable", type: "boolean", isNullable: false },
      { name: "priority", type: "integer", isNullable: true },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "payment_plans",
    structName: "PaymentPlan",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "name", type: "varchar", isNullable: false },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "type", type: "varchar", isNullable: false },
      { name: "schedule", type: "jsonb", isNullable: false },
      { name: "eligibility_rules", type: "jsonb", isNullable: true },
      { name: "plan_fees", type: "jsonb", isNullable: true },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "student_fees",
    structName: "StudentFee",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "cycle_id", type: "uuid", isNullable: false },
      { name: "enrollment_id", type: "uuid", isNullable: false },
      { name: "fee_template_id", type: "uuid", isNullable: false },
      { name: "original_amount", type: "double", isNullable: false },
      { name: "discount_amount", type: "double", isNullable: false },
      { name: "late_fee_amount", type: "double", isNullable: false },
      { name: "total_amount", type: "double", isNullable: false },
      { name: "paid_amount", type: "double", isNullable: false },
      { name: "balance", type: "double", isNullable: false },
      { name: "applied_discounts", type: "jsonb", isNullable: true },
      { name: "payment_plan_id", type: "uuid", isNullable: true },
      { name: "status", type: "varchar", isNullable: false },
      { name: "due_date", type: "date", isNullable: false },
      { name: "is_overdue", type: "boolean", isNullable: false },
      { name: "overdue_since", type: "date", isNullable: true },
      { name: "overdue_days", type: "integer", isNullable: true },
      { name: "is_installment", type: "boolean", isNullable: false },
      { name: "parent_fee_id", type: "uuid", isNullable: true },
      { name: "installment_number", type: "integer", isNullable: true },
      { name: "total_installments", type: "integer", isNullable: true },
      { name: "reminder_sent", type: "boolean", isNullable: true },
      { name: "reminder_sent_at", type: "timestamp", isNullable: true },
      { name: "final_notice_sent", type: "boolean", isNullable: true },
      { name: "final_notice_sent_at", type: "timestamp", isNullable: true },
      { name: "is_locked", type: "boolean", isNullable: false },
      { name: "locked_at", type: "timestamp", isNullable: true },
      { name: "locked_by", type: "uuid", isNullable: true },
      { name: "notes", type: "text", isNullable: true },
      { name: "meta", type: "jsonb", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "student_discounts",
    structName: "StudentDiscount",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "enrollment_id", type: "uuid", isNullable: false },
      { name: "discount_template_id", type: "uuid", isNullable: false },
      { name: "applied_amount", type: "double", isNullable: false },
      { name: "applied_to_fee_id", type: "uuid", isNullable: true },
      { name: "status", type: "varchar", isNullable: false },
      { name: "reason", type: "text", isNullable: true },
      { name: "evidence_urls", type: "jsonb", isNullable: true },
      { name: "applied_by", type: "uuid", isNullable: true },
      { name: "applied_at", type: "timestamp", isNullable: true },
      { name: "approved_by", type: "uuid", isNullable: true },
      { name: "approved_at", type: "timestamp", isNullable: true },
      { name: "approval_notes", type: "text", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "payments",
    structName: "Payment",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "fee_id", type: "uuid", isNullable: false },
      { name: "enrollment_id", type: "uuid", isNullable: false },
      { name: "amount", type: "double", isNullable: false },
      { name: "currency", type: "varchar", isNullable: false },
      { name: "exchange_rate", type: "double", isNullable: true },
      { name: "method", type: "varchar", isNullable: false },
      { name: "reference", type: "varchar", isNullable: true },
      { name: "transaction_id", type: "varchar", isNullable: true },
      { name: "receipt_number", type: "varchar", isNullable: true },
      { name: "journal_entry_id", type: "uuid", isNullable: true },
      { name: "treasury_id", type: "uuid", isNullable: true },
      { name: "payer_name", type: "varchar", isNullable: true },
      { name: "payer_phone", type: "varchar", isNullable: true },
      { name: "payer_relation", type: "varchar", isNullable: true },
      { name: "payer_id", type: "uuid", isNullable: true },
      { name: "is_verified", type: "boolean", isNullable: false },
      { name: "verified_by", type: "uuid", isNullable: true },
      { name: "verified_at", type: "timestamp", isNullable: true },
      { name: "is_refund", type: "boolean", isNullable: false },
      { name: "refund_amount", type: "double", isNullable: true },
      { name: "refund_reason", type: "varchar", isNullable: true },
      { name: "refund_approved_by", type: "uuid", isNullable: true },
      { name: "receipt_url", type: "varchar", isNullable: true },
      { name: "proof_url", type: "varchar", isNullable: true },
      { name: "reconciled", type: "boolean", isNullable: true },
      { name: "reconciled_at", type: "timestamp", isNullable: true },
      { name: "reconciled_by", type: "uuid", isNullable: true },
      { name: "notes", type: "text", isNullable: true },
      { name: "meta", type: "jsonb", isNullable: true },
      { name: "paid_at", type: "timestamp", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "created_by", type: "uuid", isNullable: true },
    ]
  },
  {
    name: "treasuries",
    structName: "Treasury",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "unit_id", type: "uuid", isNullable: true },
      { name: "name", type: "varchar", isNullable: false },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "code", type: "varchar", isNullable: true },
      { name: "type", type: "varchar", isNullable: false },
      { name: "ledger_account_id", type: "uuid", isNullable: false },
      { name: "bank_name", type: "varchar", isNullable: true },
      { name: "bank_branch", type: "varchar", isNullable: true },
      { name: "account_number", type: "varchar", isNullable: true },
      { name: "account_name", type: "varchar", isNullable: true },
      { name: "iban", type: "varchar", isNullable: true },
      { name: "swift_code", type: "varchar", isNullable: true },
      { name: "opening_balance", type: "double", isNullable: true },
      { name: "current_balance", type: "double", isNullable: true },
      { name: "currency", type: "varchar", isNullable: false },
      { name: "authorized_users", type: "jsonb", isNullable: true },
      { name: "is_default", type: "boolean", isNullable: false },
      { name: "is_active", type: "boolean", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "treasury_transactions",
    structName: "TreasuryTransaction",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "treasury_id", type: "uuid", isNullable: false },
      { name: "journal_entry_id", type: "uuid", isNullable: false },
      { name: "type", type: "varchar", isNullable: false },
      { name: "category", type: "varchar", isNullable: false },
      { name: "amount", type: "double", isNullable: false },
      { name: "currency", type: "varchar", isNullable: false },
      { name: "exchange_rate", type: "double", isNullable: true },
      { name: "reference_type", type: "varchar", isNullable: true },
      { name: "reference_id", type: "uuid", isNullable: true },
      { name: "description", type: "text", isNullable: false },
      { name: "notes", type: "text", isNullable: true },
      { name: "attachments", type: "jsonb", isNullable: true },
      { name: "created_by", type: "uuid", isNullable: true },
      { name: "approved_by", type: "uuid", isNullable: true },
      { name: "approved_at", type: "timestamp", isNullable: true },
      { name: "transaction_date", type: "date", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "financial_events",
    structName: "FinancialEvent",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "event_type", type: "varchar", isNullable: false },
      { name: "entity_type", type: "varchar", isNullable: false },
      { name: "entity_id", type: "uuid", isNullable: false },
      { name: "payload", type: "jsonb", isNullable: false },
      { name: "sequence_number", type: "integer", isNullable: true },
      { name: "aggregate_id", type: "uuid", isNullable: true },
      { name: "status", type: "varchar", isNullable: false },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "financial_reports",
    structName: "FinancialReport",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "name", type: "varchar", isNullable: false },
      { name: "name_ar", type: "varchar", isNullable: true },
      { name: "type", type: "varchar", isNullable: false },
      { name: "start_date", type: "date", isNullable: false },
      { name: "end_date", type: "date", isNullable: false },
      { name: "summary", type: "jsonb", isNullable: true },
      { name: "details", type: "jsonb", isNullable: true },
      { name: "status", type: "varchar", isNullable: false },
      { name: "generated_by", type: "uuid", isNullable: true },
      { name: "generated_at", type: "timestamp", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
    ]
  },
  // ─────────────────────────────────────────────────────────────────────────────
  // AUDIT SCHEMA TABLES
  // ─────────────────────────────────────────────────────────────────────────────
  {
    name: "system_log",
    structName: "SystemLog",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: true },
      { name: "level", type: "varchar", isNullable: false },
      { name: "category", type: "varchar", isNullable: false },
      { name: "message", type: "text", isNullable: false },
      { name: "details", type: "jsonb", isNullable: true },
      { name: "stack_trace", type: "text", isNullable: true },
      { name: "user_id", type: "uuid", isNullable: true },
      { name: "request_id", type: "varchar", isNullable: true },
      { name: "endpoint", type: "varchar", isNullable: true },
      { name: "method", type: "varchar", isNullable: true },
      { name: "duration_ms", type: "integer", isNullable: true },
      { name: "memory_usage_mb", type: "integer", isNullable: true },
      { name: "cpu_usage_percent", type: "integer", isNullable: true },
      { name: "timestamp", type: "timestamp", isNullable: false },
    ]
  },
  {
    name: "data_transfers",
    structName: "DataTransfer",
    columns: [
      { name: "id", type: "uuid", isNullable: false, isPrimaryKey: true },
      { name: "org_id", type: "uuid", isNullable: false },
      { name: "type", type: "varchar", isNullable: false },
      { name: "entity_type", type: "varchar", isNullable: false },
      { name: "file_name", type: "varchar", isNullable: true },
      { name: "file_url", type: "text", isNullable: true },
      { name: "file_size", type: "integer", isNullable: true },
      { name: "file_format", type: "varchar", isNullable: true },
      { name: "total_records", type: "integer", isNullable: true },
      { name: "success_records", type: "integer", isNullable: true },
      { name: "failed_records", type: "integer", isNullable: true },
      { name: "skipped_records", type: "integer", isNullable: true },
      { name: "error_log", type: "jsonb", isNullable: true },
      { name: "warning_log", type: "jsonb", isNullable: true },
      { name: "status", type: "varchar", isNullable: false },
      { name: "started_at", type: "timestamp", isNullable: true },
      { name: "completed_at", type: "timestamp", isNullable: true },
      { name: "cancelled_at", type: "timestamp", isNullable: true },
      { name: "cancel_reason", type: "varchar", isNullable: true },
      { name: "initiated_by", type: "uuid", isNullable: true },
      { name: "config", type: "jsonb", isNullable: true },
      { name: "mapping", type: "jsonb", isNullable: true },
      { name: "created_at", type: "timestamp", isNullable: false },
      { name: "updated_at", type: "timestamp", isNullable: false },
    ]
  }
];

function mapRustType(col: Column): string {
  // If nullable, wrap in Option
  let rustType = "";
  switch (col.type) {
    case "uuid":
    case "varchar":
    case "text":
    case "date":
    case "timestamp":
    case "time":
      rustType = "String";
      break;
    case "integer":
      rustType = "i32";
      break;
    case "boolean":
      // SQLite booleans are typically stored as i32
      rustType = "i32";
      break;
    case "double":
    case "numeric":
      rustType = "f64";
      break;
    case "jsonb":
      // Store JSON structures as raw option-string for SQLite storage
      rustType = "String";
      break;
    default:
      rustType = "String";
  }

  if (col.isNullable && !col.isPrimaryKey) {
    return `Option<${rustType}>`;
  }
  return rustType;
}

function generateRustFile(table: Table): string {
  let fields = "";
  for (const col of table.columns) {
    const rustType = mapRustType(col);
    const primaryKeyAttr = col.isPrimaryKey
      ? "    #[sea_orm(primary_key, auto_increment = false)]\n"
      : "";
    // Check if column name needs sea_orm column_name overrides (e.g. if it matches Rust keywords or special naming)
    let extraAttr = "";
    if (col.name === "type") {
      extraAttr = `    #[sea_orm(column_name = "type")]\n`;
    }
    
    // Rust naming format: snake_case matches JavaScript camelCase in SeaORM, but we write it direct snake_case
    const rustFieldName = col.name === "type" ? "item_type" : col.name;
    
    fields += `${primaryKeyAttr}${extraAttr}    pub ${rustFieldName}: ${rustType},\n`;
  }

  const moduleName = table.name === "data_transfers" ? "data_transfer" : table.name.replace(/s$/, "");

  return `use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "${table.name}")]
pub struct Model {
${fields}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
`;
}

// Generate the files
let modExports = "";
for (const table of tables) {
  // Translate table names to Rust module naming: singular snake_case
  let moduleName = table.name;
  if (moduleName === "data_transfers") {
    moduleName = "data_transfer";
  } else if (moduleName.endsWith("_entries") || moduleName === "journal_entries") {
    moduleName = moduleName.replace(/_entries$/, "_entry");
  } else if (moduleName.endsWith("treasuries") || moduleName === "treasuries") {
    moduleName = moduleName.replace(/treasuries$/, "treasury");
  } else if (moduleName.endsWith("s")) {
    moduleName = moduleName.substring(0, moduleName.length - 1);
  }
  const code = generateRustFile(table);
  const filePath = path.join(targetDir, `${moduleName}.rs`);
  fs.writeFileSync(filePath, code, "utf-8");
  console.log(`✅ Generated: ${filePath}`);
  
  modExports += `pub mod ${moduleName};\n`;
}

console.log("\nCopy and paste this into apps/api-rust/src/entities/mod.rs:\n");
console.log(modExports);
