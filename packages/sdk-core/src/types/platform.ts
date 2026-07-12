/**
 * Platform command interfaces — Desktop-specific operations.
 * Year management, promotion, periods, overrides, AI, document templates.
 *
 * Migrated: periods → periods, AcademicPeriodContext → AcademicPeriod.
 */
import type { AcademicPeriod } from "./academic";

export interface AcademicYear {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  status: string; // planning | active | closing | archived
  start_date: string;
  end_date: string;
  is_locked: number;
  locked_by?: string | null;
  lock_reason?: string | null;
  archived_at?: string | null;
  archived_by?: string | null;
  created_at: string;
  updated_at: string;
}

export interface AcademicYearInfo {
  id: string;
  name: string;
  name_ar?: string | null;
  status: string;
}

export interface AcademicContextResponse {
  year_id: string;
  current_year: AcademicYear | null;
  all_years: AcademicYearInfo[];
  periods: AcademicPeriod[];
  year_status: "planning" | "active" | "closing" | "archived";
  is_read_only: boolean;
}

/** @deprecated Use AcademicContextResponse instead */
export type AcademicPeriodContext = AcademicPeriod;
/** @deprecated Use AcademicPeriod instead */
export type AcademicPeriodSummary = AcademicPeriod;
/** @deprecated Use period status instead */
export type PeriodPhase = "planned" | "active" | "grading" | "exam" | "locked" | "closed";

export interface PromotionJob {
  id: string;
  org_id: string;
  source_year_id: string;
  target_year_id?: string | null;
  status: string;
  initiated_by: string;
  config: string;
  dry_run_result?: string | null;
  total_students?: number | null;
  promoted_count?: number | null;
  retained_count?: number | null;
  graduated_count?: number | null;
  created_at: string;
  updated_at: string;
}

export interface PromotionDetail {
  id: string;
  job_id: string;
  enrollment_id: string;
  student_name: string;
  decision: string;
  reason?: string | null;
  override_by?: string | null;
}

export interface PromotionConfig {
  passing_score: number;
  min_attendance_rate: number;
  carry_over_fees: boolean;
  auto_create_target_year: boolean;
}

export interface OverrideRequest {
  id: string;
  org_id: string;
  entity_type: string;
  entity_id: string;
  field_name: string;
  old_value?: string | null;
  new_value: string;
  reason: string;
  requested_by: string;
  status: string;
  approved_by?: string | null;
  created_at: string;
  updated_at: string;
  override_type?: string;
  resolution_notes?: string | null;
  requested_by_name?: string | null;
}

/** Unified academic policy (replaces CalendarEvent + OrgPolicy) */
export interface AcademicPolicy {
  id: string;
  org_id: string;
  // Grading defaults
  default_grading_system: string;
  default_passing_grade: number;
  default_max_grade: number;
  // Enrollment
  enrollment_auto_close: number;
  allow_late_enrollment: number;
  late_enrollment_approval: number;
  // Grade entry
  grade_entry_auto_close: number;
  allow_grade_edit_after_close: number;
  grade_edit_approval: number;
  // Promotion
  passing_score: number;
  min_attendance_rate: number;
  auto_graduate_final_level: number;
  final_level?: string | null;
  exclude_withdrawn: number;
  // Autopilot
  autopilot_mode: string;
  autopilot_enabled: number;
  auto_open_enrollment: number;
  auto_close_enrollment: number;
  auto_open_grade_entry: number;
  auto_close_grade_entry: number;
  auto_lock_periods: number;
  auto_advance_periods: number;
  auto_archive_years: number;
  grace_days_after_deadline: number;
  notify_before_deadline_days: number;
  created_at: string;
  updated_at: string;
}

/** @deprecated Use AcademicPolicy instead */
export interface CalendarEvent {
  id: string;
  org_id: string;
  period_id: string;
  title: string;
  title_ar?: string | null;
  event_type: string;
  event_date: string;
  end_date?: string | null;
  status: string;
  created_at: string;
}

/** @deprecated Use AcademicPolicy instead */
export interface OrgPolicy {
  id: string;
  org_id: string;
  automation_level: string;
  enrollment_policy: string;
  grade_policy: string;
  settings: string;
  created_at: string;
  updated_at: string;
}

export interface DocumentTemplate {
  id: string;
  org_id: string;
  name: string;
  entity_type: string;
  status: string;
  document_json: string;
  created_at: string;
  updated_at: string;
}

export interface AutopilotStatus {
  org_id: string;
  is_enabled: boolean;
  is_paused: boolean;
  mode: string;
  last_tick_at?: string | null;
}

export interface AutopilotTask {
  id: string;
  org_id: string;
  task_type: string;
  status: string;
  result?: string | null;
  created_at: string;
}

export interface AgentSession {
  id: string;
  org_id: string;
  agent_type: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface AgentActionLog {
  id: string;
  session_id: string;
  action_type: string;
  status: string;
  description: string;
  approved_by?: string | null;
  rejected_by?: string | null;
  created_at: string;
}

export type AcademicOverrideRequest = OverrideRequest;

export interface ApprovalAction {
  action: "approve" | "reject" | "escalate";
  user_id: string;
  user_name?: string | null;
  user_role: string;
  notes?: string | null;
}
