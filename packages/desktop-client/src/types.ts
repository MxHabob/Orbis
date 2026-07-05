/** Shared entity types — mirror core-domain SeaORM models (snake_case from Rust IPC). */

export interface CommandError {
  message: string;
  code?: string | null;
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

export type AcademicCycle = JsonRecord;
export type Class = JsonRecord;
export type Enrollment = JsonRecord;
export type Grade = JsonRecord;
export type Attendance = JsonRecord;
export type Document = JsonRecord;
export type DocumentTemplate = JsonRecord;
export type Subject = JsonRecord;
export type Assessment = JsonRecord;
export type Notification = JsonRecord;
export type Organization = JsonRecord;
export type DashboardStats = JsonRecord;
export type GradeAnalytics = JsonRecord;

// ── Schedule ─────────────────────────────────────────────────────────────────
export type TimeSlot = JsonRecord;
export type ClassSubject = JsonRecord;
export type ScheduledLesson = JsonRecord;
export type ScheduleConflict = JsonRecord;
export type SolveJob = JsonRecord;

// ── Guardians ────────────────────────────────────────────────────────────────
export type GuardianLink = JsonRecord;

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

export type CyclePhase = "pre_start" | "active" | "exams" | "grade_entry" | "closed";

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
  | "enrollment_open" | "enrollment_close"
  | "add_drop_open" | "add_drop_close"
  | "classes_start" | "classes_end"
  | "exam_prep_start" | "exam_start" | "exam_end"
  | "grade_entry_open" | "grade_entry_close"
  | "results_announcement"
  | "holiday" | "break" | "teacher_training"
  | "promotion_window_open" | "promotion_window_close"
  | "cycle_start" | "cycle_end" | "custom";

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

export type FeeCarryOverMode = "ignore" | "carry_all" | "carry_selected" | "require_approval";

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
