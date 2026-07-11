/**
 * Platform command interfaces — Desktop-specific operations.
 * Year management, promotion, calendar, overrides, AI, document templates.
 */

export interface AcademicYear {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  status: string;
  start_date: string;
  end_date: string;
  is_current: number;
  created_at: string;
  updated_at: string;
}

export interface AcademicYearInfo {
  id: string;
  name: string;
  name_ar?: string | null;
  status: string;
  is_current: number;
}

export interface AcademicCycleContext {
  id: string;
  name: string;
  name_ar?: string | null;
  is_enrollment_open: boolean;
  is_grade_entry_open: boolean;
  is_locked: boolean;
  phase: CyclePhase;
}

export interface AcademicCycleSummary {
  id: string;
  name: string;
  name_ar?: string | null;
  cycle_type: string;
  start_date: string;
  end_date: string;
  is_current: number;
}

export interface AcademicContextResponse {
  year_id: string;
  active_cycle: AcademicCycleContext | null;
  all_cycles: AcademicCycleSummary[];
  year_status: "active" | "archived" | "promoting";
  is_read_only: boolean;
}

export type CyclePhase = "active" | "enrollment" | "grading" | "exam" | "locked" | "archived";

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

export interface CalendarEvent {
  id: string;
  org_id: string;
  cycle_id: string;
  title: string;
  title_ar?: string | null;
  event_type: string;
  event_date: string;
  end_date?: string | null;
  status: string;
  created_at: string;
}

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
