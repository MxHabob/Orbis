/** Shared entity types — mirror core-domain SeaORM models (snake_case from Rust IPC). */

export interface CommandError {
  message: string;
  code: string;
}

export interface User {
  id: string;
  org_id: string;
  email?: string | null;
  full_name: string;
  name_ar?: string | null;
  name_en?: string | null;
  avatar?: string | null;
  role?: string | null;
  [key: string]: unknown;
}

export interface LoginResponse {
  user: User;
  access_token: string;
  refresh_token: string;
}

export type JsonRecord = Record<string, unknown>;

export interface AcademicCycle {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  cycle_type: string;
  parent_id?: string | null;
  start_date: string;
  end_date: string;
  hijri_start_date?: string | null;
  hijri_end_date?: string | null;
  exam_start_date?: string | null;
  exam_end_date?: string | null;
  weight?: number | null;
  grading_system: string;
  passing_grade?: number | null;
  max_grade?: number | null;
  is_current: number;
  is_locked: number;
  is_enrollment_open: number;
  is_grade_entry_open: number;
  grade_entry_start?: string | null;
  grade_entry_end?: string | null;
  locked_at?: string | null;
  locked_by?: string | null;
  settings?: string | null;
  sort_order?: number | null;
  created_at: string;
  updated_at: string;
}

export interface Class {
  id: string;
  org_id: string;
  unit_id: string;
  cycle_id: string;
  name: string;
  name_ar?: string | null;
  level: string;
  division?: string | null;
  homeroom_teacher_id?: string | null;
  capacity?: number | null;
  current_students?: number | null;
  room_id?: string | null;
  color?: string | null;
  is_active: number;
  is_deleted: number;
  deleted_at?: string | null;
  created_at: string;
  updated_at: string;
}

export interface Enrollment {
  id: string;
  org_id: string;
  cycle_id: string;
  student_id: string;
  class_id: string;
  student_number: string;
  status: string;
  enrollment_date: string;
  enrollment_type: string;
  withdrawal_date?: string | null;
  withdrawal_reason?: string | null;
  final_grade?: string | null;
  final_score?: number | null;
  final_rank?: number | null;
  is_promoted?: number | null;
  promotion_decision?: string | null;
  promotion_notes?: string | null;
  meta?: string | null;
  created_at: string;
  updated_at: string;

  // Merged student/user fields accessed in detail/edit views
  full_name?: string | null;
  name_ar?: string | null;
  name_en?: string | null;
  birth_date?: string | null;
  sex?: string | null;
  email?: string | null;
  phone_number?: string | null;
  phone?: string | null;
  address?: string | null;
  student_name?: string | null;
  student_email?: string | null;
  class_name?: string | null;
  class_name_ar?: string | null;
  avatar?: string | null;
  guardian_name?: string | null;
  guardian_phone?: string | null;
}

export interface Grade {
  id: string;
  org_id: string;
  cycle_id: string;
  enrollment_id: string;
  assessment_id?: string | null;
  class_subject_id?: string | null;
  subject_name: string;
  subject_name_ar?: string | null;
  score: number;
  max_score: number;
  weight: number;
  grade_letter?: string | null;
  grade_points?: number | null;
  exam_type: string;
  semester: string;
  notes?: string | null;
  graded_by: string;
  graded_at: string;
  is_final: number;
  created_at: string;
  updated_at: string;
}

export interface Attendance {
  id: string;
  org_id: string;
  cycle_id: string;
  enrollment_id: string;
  class_id: string;
  date: string;
  status: string;
  check_in_time?: string | null;
  check_out_time?: string | null;
  period?: string | null;
  subject_name?: string | null;
  notes?: string | null;
  excuse_type?: string | null;
  is_excused: number;
  recorded_by: string;
  created_at: string;
  updated_at: string;
}

export interface Document {
  id: string;
  org_id: string;
  name: string;
  file_path: string;
  file_size: number;
  mime_type: string;
  entity_type: string;
  entity_id: string;
  created_at: string;
  updated_at: string;
}

export interface DocumentTemplate {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  template_type: string;
  content: string;
  engine_type: string;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface Subject {
  id: string;
  org_id: string;
  code: string;
  name: string;
  name_ar?: string | null;
  department?: string | null;
  description?: string | null;
  credit_hours: number;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface Assessment {
  id: string;
  org_id: string;
  cycle_id: string;
  class_subject_id: string;
  title: string;
  title_ar?: string | null;
  description?: string | null;
  item_type: string;
  max_score: number;
  weight: number;
  grade_group_name: string;
  assessment_date?: string | null;
  due_date?: string | null;
  is_published: number;
  is_locked: number;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface Notification {
  id: string;
  org_id: string;
  user_id: string;
  title: string;
  content: string;
  is_read: number;
  created_at: string;
}

export interface Organization {
  id: string;
  name: string;
  name_ar?: string | null;
  primary_color?: string | null;
  logo_url?: string | null;
  created_at: string;
  updated_at: string;
}
export interface AttendanceOverview {
  present_rate: number;
  late_rate: number;
  absent_rate: number;
}

export interface BehaviorSummary {
  net_points: number;
  critical_count: number;
  major_count: number;
  minor_count: number;
  total_records: number;
}

export interface DashboardStats {
  student_count: number;
  teacher_count: number;
  class_count: number;
  attendance_today: number;

  total_students: number;
  total_classes: number;
  total_teachers: number;
  total_staff: number;
  attendance_overview: AttendanceOverview;
  behavior_summary: BehaviorSummary;
  enrollment_rate: number;
  total_enrollments: number;
}

export interface GradeAnalytics {
  overall_average: number;
  pass_rate: number;
  highest_score: number;
  lowest_score: number;
  total_graded: number;
}

// ── Schedule ─────────────────────────────────────────────────────────────────
export interface TimeSlot {
  id: string;
  org_id: string;
  cycle_id: string;
  slot_number: number;
  name?: string | null;
  name_ar?: string | null;
  start_time: string;
  end_time: string;
  duration_minutes: number;
  is_break: number;
  break_type?: string | null;
  applicable_days?: string | null;
  sort_order?: number | null;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface ClassSubject {
  id: string;
  class_id: string;
  subject_id: string;
  cycle_id: string;
  primary_teacher_id: string;
  assistant_teacher_id?: string | null;
  substitute_teacher_id?: string | null;
  grade_groups?: string | null;
  weekly_hours?: number | null;
  theory_hours?: number | null;
  practical_hours?: number | null;
  assessment_config?: string | null;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface ScheduledLesson {
  id: string;
  org_id: string;
  cycle_id: string;
  class_id: string;
  class_subject_id: string;
  subject_id: string;
  teacher_id: string;
  substitute_teacher_id?: string | null;
  time_slot_id: string;
  day_of_week: number;
  room_id?: string | null;
  room_name?: string | null;
  status: string;
  is_substituted: number;
  substitution_reason?: string | null;
  topic?: string | null;
  topic_ar?: string | null;
  objectives?: string | null;
  materials?: string | null;
  homework?: string | null;
  homework_due_date?: string | null;
  attendance_taken: number;
  attendance_taken_at?: string | null;
  attendance_taken_by?: string | null;
  attendance_method?: string | null;
  notes?: string | null;
  cancellation_reason?: string | null;
  created_at: string;
  updated_at: string;
}

export interface ScheduleConflict {
  lesson_id: string;
  conflict_type: string;
  message: string;
}

export interface SolveJob {
  id: string;
  org_id: string;
  status: string;
  progress: number;
  error_message?: string | null;
  created_at: string;
  updated_at: string;
}

// ── Guardians ────────────────────────────────────────────────────────────────
export interface GuardianLink {
  id: string;
  org_id: string;
  student_id: string;
  guardian_id: string;
  relationship: string;
  is_primary: number | boolean;
  emergency_contact: number;
  created_at: string;
  updated_at: string;

  // Extensively used optional properties in svelte templates
  relation?: string | null;
  occupation?: string | null;
  is_emergency_contact?: number | boolean | null;
  is_payer?: number | boolean | null;
  payment_share_percent?: number | null;
}

export interface UserSession {
  id: string;
  user_id: string;
  token: string;
  refresh_token?: string | null;
  device_id?: string | null;
  device_type?: string | null;
  device_name?: string | null;
  os?: string | null;
  browser?: string | null;
  ip_address?: string | null;
  user_agent?: string | null;
  is_current: number;
  is_trusted: number;
  mfa_verified: number;
  expires_at: string;
  refresh_expires_at?: string | null;
  last_active_at?: string | null;
  created_at: string;
}

export interface BatchJobStatus {
  jobId: string;
  totalRecords: number;
  processedRecords: number;
  status: "running" | "paused" | "cancelled" | "completed" | "failed";
  errorMessage?: string | null;
  outputPath: string;
}

// ── Academic Context ─────────────────────────────────────────────────────────

export type CyclePhase =
  | "pre_start"
  | "active"
  | "exams"
  | "grade_entry"
  | "closed";

export interface AcademicCycleSummary {
  id: string;
  name: string;
  name_ar?: string | null;
  cycle_type: string;
  start_date: string;
  end_date: string;
  is_current: boolean;
  is_locked: boolean;
  phase: CyclePhase;
}

export interface AcademicCycleContext {
  id: string;
  name: string;
  name_ar?: string | null;
  cycle_type: string;
  start_date: string;
  end_date: string;
  is_enrollment_open: boolean;
  is_grade_entry_open: boolean;
  is_locked: boolean;
  phase: CyclePhase;
}

export interface AcademicContextResponse {
  year_id: string;
  year_name: string;
  year_name_ar?: string | null;
  year_status: "active" | "archived" | "promoting";
  is_read_only: boolean;
  active_cycle: AcademicCycleContext | null;
  all_cycles: AcademicCycleSummary[];
}

// ── Academic Calendar ────────────────────────────────────────────────────────

export type CalendarEventType =
  | "enrollment_open"
  | "enrollment_close"
  | "add_drop_open"
  | "add_drop_close"
  | "classes_start"
  | "classes_end"
  | "exam_prep_start"
  | "exam_start"
  | "exam_end"
  | "grade_entry_open"
  | "grade_entry_close"
  | "results_announcement"
  | "holiday"
  | "break"
  | "teacher_training"
  | "promotion_window_open"
  | "promotion_window_close"
  | "cycle_start"
  | "cycle_end"
  | "custom";

export type CalendarExecutionMode = "manual" | "suggested" | "automatic";

export interface CalendarEvent {
  id: string;
  org_id: string;
  cycle_id: string;
  event_type: CalendarEventType;
  title: string;
  title_ar?: string | null;
  description?: string | null;
  start_date: string;
  end_date: string;
  start_time?: string | null;
  end_time?: string | null;
  execution_mode: CalendarExecutionMode;
  auto_action?: string | null;
  prerequisites?: string | null;
  executed_at?: string | null;
  executed_by?: string | null;
  execution_result?: string | null;
  execution_notes?: string | null;
  is_recurring: number;
  recurrence_rule?: string | null;
  color?: string | null;
  is_active: number;
  sort_order?: number | null;
  created_by?: string | null;
  created_at: string;
  updated_at: string;
}

export interface EventProcessingResult {
  event_id: string;
  event_type: string;
  title: string;
  execution_mode: string;
  outcome: "executed" | "suggested" | "skipped" | "blocked" | "failed";
  message?: string | null;
}

export interface CalendarProcessingReport {
  executed: EventProcessingResult[];
  suggested: EventProcessingResult[];
  skipped: EventProcessingResult[];
  blocked: EventProcessingResult[];
}

export interface OrgAcademicPolicy {
  id: string;
  org_id: string;
  default_grading_system: string;
  default_passing_grade: number;
  default_max_grade: number;
  enrollment_auto_close_on_capacity: number;
  allow_late_enrollment: number;
  late_enrollment_requires_approval: number;
  grade_entry_auto_close_on_deadline: number;
  allow_grade_modification_after_close: number;
  grade_modification_requires_approval: number;
  default_event_execution_mode: CalendarExecutionMode;
  settings?: string | null;
  created_at: string;
  updated_at: string;
}

// ── Academic Year Management ─────────────────────────────────────────────────

export interface AcademicYear {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  start_date: string;
  end_date: string;
  hijri_start_date?: string | null;
  hijri_end_date?: string | null;
  db_filename: string;
  status: "active" | "archived" | "promoting";
  is_current: number;
  archived_at?: string | null;
  archived_by?: string | null;
  promotion_source_year_id?: string | null;
  db_size_bytes?: number | null;
  student_count?: number | null;
  teacher_count?: number | null;
  class_count?: number | null;
  notes?: string | null;
  created_at: string;
  updated_at: string;
}

export interface AcademicYearInfo {
  year: AcademicYear;
  is_active_in_app: boolean;
}

export interface CreateAcademicYearRequest {
  name: string;
  name_ar?: string | null;
  start_date: string;
  end_date: string;
  hijri_start_date?: string | null;
  hijri_end_date?: string | null;
  notes?: string | null;
}

// ── Promotion ────────────────────────────────────────────────────────────────

export interface PromotionJob {
  id: string;
  org_id: string;
  source_year_id: string;
  target_year_id?: string | null;
  status: string;
  initiated_by: string;
  initiated_by_name?: string | null;
  config: string;
  dry_run_result?: string | null;
  dry_run_at?: string | null;
  total_students?: number | null;
  promoted_count?: number | null;
  retained_count?: number | null;
  graduated_count?: number | null;
  withdrawn_count?: number | null;
  excluded_count?: number | null;
  backup_path?: string | null;
  rollback_available: number;
  started_at?: string | null;
  completed_at?: string | null;
  rolled_back_at?: string | null;
  created_at: string;
  updated_at: string;
}

export interface PromotionDetail {
  id: string;
  job_id: string;
  student_id: string;
  student_name: string;
  student_name_ar?: string | null;
  student_number?: string | null;
  source_enrollment_id: string;
  source_class_id: string;
  source_class_name?: string | null;
  source_level: string;
  source_division?: string | null;
  target_enrollment_id?: string | null;
  target_class_id?: string | null;
  target_class_name?: string | null;
  target_level?: string | null;
  decision: "promoted" | "retained" | "graduated" | "excluded";
  reason: string;
  final_score?: number | null;
  attendance_rate?: number | null;
  manual_override: number;
  override_by?: string | null;
  override_reason?: string | null;
  original_decision?: string | null;
  created_at: string;
}

export interface PromotionConfig {
  source_year_id: string;
  target_year_name: string;
  target_year_name_ar?: string | null;
  target_start_date: string;
  target_end_date: string;
  passing_score?: number | null;
  min_attendance_rate?: number | null;
  auto_graduate_final_level: boolean;
  final_level?: string | null;
  exclude_withdrawn: boolean;
  carry_over_outstanding_fees: boolean;
  carry_over_guardians: boolean;
  class_mapping: ClassMapping[];
}

export interface ClassMapping {
  source_level: string;
  target_level: string;
  target_class_names: string[];
  distribution_strategy: string;
}

export interface PromotionPreview {
  job_id: string;
  total: number;
  promoted: number;
  retained: number;
  graduated: number;
  excluded: number;
}

export interface OverrideRequest {
  detail_id: string;
  new_decision: string;
  reason: string;
  user_id: string;
}

// ── Promotion Policy ─────────────────────────────────────────────────────────

export type FeeCarryOverMode =
  | "ignore"
  | "carry_all"
  | "carry_selected"
  | "require_approval";

export interface PromotionPolicy {
  id: string;
  org_id: string;
  passing_score: number;
  min_attendance_rate: number;
  auto_graduate_final_level: number;
  final_level?: string | null;
  exclude_withdrawn: number;
  fee_carry_over_mode: FeeCarryOverMode;
  fee_carry_over_categories?: string | null;
  carry_over_guardians: number;
  carry_over_time_slots: number;
  carry_over_class_subjects: number;
  auto_create_cycles: number;
  cycle_template?: string | null;
  created_at: string;
  updated_at: string;
}

// ── Override & Approval ──────────────────────────────────────────────────────

export type OverrideType =
  | "grade_modification"
  | "attendance_correction"
  | "enrollment_exception"
  | "late_enrollment"
  | "fee_waiver"
  | "promotion_override"
  | "cycle_unlock"
  | "deadline_extension"
  | "custom";

export type RiskLevel = "low" | "medium" | "high" | "critical";

export type OverrideStatus =
  | "pending"
  | "approved"
  | "rejected"
  | "escalated"
  | "expired"
  | "cancelled";

export interface ApprovalStep {
  role: string;
  status: string;
  by: string;
  by_name?: string | null;
  at: string;
  notes?: string | null;
}

export interface AcademicOverrideRequest {
  id: string;
  org_id: string;
  override_type: OverrideType;
  risk_level: RiskLevel;
  entity_type: string;
  entity_id: string;
  field_name?: string | null;
  old_value?: string | null;
  new_value?: string | null;
  reason: string;
  requested_by: string;
  requested_by_name?: string | null;
  requested_at: string;
  status: OverrideStatus;
  current_approver_role?: string | null;
  approval_chain: string; // JSON array of ApprovalStep[]
  resolved_by?: string | null;
  resolved_by_name?: string | null;
  resolved_at?: string | null;
  resolution_notes?: string | null;
  expires_at?: string | null;
  metadata?: string | null;
  created_at: string;
  updated_at: string;
}

export interface ApprovalAction {
  action: "approve" | "reject" | "escalate";
  user_id: string;
  user_name?: string | null;
  user_role: string;
  notes?: string | null;
}

// ── AI & Autopilot ───────────────────────────────────────────────────────────

export interface AutopilotConfig {
  id: string;
  org_id: string;
  mode: "automatic" | "suggested" | "manual";
  check_interval_minutes: number;
  timezone: string;
  require_backup_before_critical: number;
  max_auto_retries: number;
  pause_on_error: number;
  is_enabled: number;
  last_tick_at?: string | null;
  last_tick_result?: string | null;
  paused_at?: string | null;
  pause_reason?: string | null;
  created_at: string;
  updated_at: string;
}

export interface AutopilotTask {
  id: string;
  org_id: string;
  calendar_event_id?: string | null;
  task_type: string;
  scheduled_at: string;
  deadline_at?: string | null;
  target_entity_type?: string | null;
  target_entity_id?: string | null;
  params?: string | null;
  prerequisites?: string | null;
  status: "pending" | "ready" | "executing" | "completed" | "failed" | "skipped" | "suggested" | "approved" | "cancelled";
  execution_mode: string;
  attempts: number;
  last_attempt_at?: string | null;
  last_error?: string | null;
  suggested_at?: string | null;
  approved_by?: string | null;
  approved_at?: string | null;
  rejection_reason?: string | null;
  completed_at?: string | null;
  result?: string | null;
  idempotency_key: string;
  created_at: string;
  updated_at: string;
}

export interface AutopilotTickResult {
  org_id: string;
  tick_at: string;
  snapshot_summary: {
    active_events: number;
    missed_events: number;
    upcoming_events: number;
    pending_tasks: number;
    failed_tasks: number;
  };
  plan: {
    tasks_created: number;
    already_planned: number;
    missed_event_count: number;
  };
  execution: {
    executed: number;
    succeeded: number;
    failed: number;
    skipped: number;
  };
  health: AutopilotHealthReport;
  status: "ok" | "warning" | "error" | "paused" | "disabled";
  error?: string | null;
}

export interface AutopilotHealthReport {
  org_id: string;
  checked_at: string;
  stuck_tasks: string[];
  overdue_tasks: string[];
  status: "healthy" | "warning" | "critical";
}

export interface AgentSession {
  id: string;
  org_id: string;
  agent_type: "assistant" | "autopilot" | "analyst" | "guardian" | "external";
  agent_version?: string | null;
  acting_as_user_id: string;
  acting_as_role: string;
  conversation_id?: string | null;
  status: "active" | "completed" | "suspended" | "revoked";
  max_risk_level: string;
  requires_approval_above: string;
  actions_executed: number;
  actions_suggested: number;
  started_at: string;
  last_active_at: string;
  ended_at?: string | null;
  created_at: string;
}

export interface AgentActionLog {
  id: string;
  org_id: string;
  session_id: string;
  correlation_id?: string | null;
  action_code: string;
  input_data?: string | null;
  output_data?: string | null;
  reasoning: string;
  confidence?: number | null;
  status: "pending" | "approved" | "executed" | "rejected" | "failed" | "rolled_back";
  risk_level: string;
  approval_required: number;
  approved_by?: string | null;
  approved_at?: string | null;
  rejection_reason?: string | null;
  affected_entities?: string | null;
  idempotency_key?: string | null;
  duration_ms?: number | null;
  error?: string | null;
  is_reversible: number;
  rollback_data?: string | null;
  reversed_at?: string | null;
  reversed_by?: string | null;
  executed_at: string;
  created_at: string;
}
