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
