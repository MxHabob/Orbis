/**
 * Platform command interfaces — Desktop-specific operations.
 * Year management, promotion, periods, overrides, AI, document templates.
 *
 * Migrated: periods → periods, AcademicPeriodContext → AcademicPeriod.
 */
import type { AcademicPeriod } from "./academic.js";

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
  class_count?: number;
  subject_count?: number;
  teacher_count?: number;
  student_count?: number;
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
  period_id: string;
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
export type PeriodPhase =
  | "planned"
  | "active"
  | "grading"
  | "exam"
  | "locked"
  | "closed";

export interface PromotionJob {
  id: string;
  org_id: string;
  source_period_id: string;
  target_period_id?: string | null;
  status: string;
  initiated_by: string;
  config: string;
  dry_run_result?: string | null;
  total_students?: number | null;
  promoted_count?: number | null;
  retained_count?: number | null;
  graduated_count?: number | null;
  withdrawn_count?: number | null;
  excluded_count?: number | null;
  error_log?: string | null;
  rollback_available: number;
  started_at?: string | null;
  completed_at?: string | null;
  rolled_back_at?: string | null;
  rolled_back_by?: string | null;
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

export interface ClassMapping {
  source_level: string;
  target_level: string;
  target_class_names: string[];
  distribution_strategy: string;
}

export interface PromotionConfig {
  source_period_id: string;
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
  rejection_reason?: string | null;
  reasoning?: string;
  risk_level?: string;
  action_code?: string;
  input_data?: any;
  created_at: string;
  updated_at: string;
  override_type?: string;
  resolution_notes?: string | null;
  requested_by_name?: string | null;
  approval_chain?: string | null;
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
  executed_at?: string;
  start_date?: string;
  execution_mode?: string;
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
  layout_type: string;
  content: string;
  page_size: string;
  orientation: string;
  margins: Record<string, any>;
  is_default: boolean;
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

// ── Period Gate Types ─────────────────────────────────────────────

export type GateType =
  | "enrollment"
  | "grade_entry"
  | "attendance"
  | "exam_entry"
  | "assessment";

export type GateDecision = "allowed" | "late" | "requires_override" | "blocked";

export type GateAction = "direct_action" | "save_draft" | "request_override";

export interface CapacityInfo {
  current: number;
  max: number;
  available: number;
  utilization_pct: number;
}

export interface GateContext {
  period_status: string;
  window_start?: string | null;
  window_end?: string | null;
  is_within_grace: boolean;
  grace_days: number;
  capacity?: CapacityInfo | null;
}

export interface GateVerdict {
  gate_type: GateType;
  decision: GateDecision;
  reason?: string | null;
  override_type?: string | null;
  allowed_actions: GateAction[];
  context: GateContext;
}

// ── Cursor Pagination Types ──────────────────────────────────────

export interface CursorRequest {
  cursor?: string | null;
  limit?: number;
}

export interface CursorPage<T> {
  items: T[];
  count: number;
  next_cursor: string | null;
  has_more: boolean;
}

export interface Party {
  id: string;
  party_type: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface ProcessLog {
  id: string;
  saga_id: string;
  status: string;
  step_name: string;
  payload?: Record<string, any>;
  created_at: string;
}

export interface UserAccount {
  id: string;
  user_id: string;
  provider: string;
  provider_account_id: string;
  provider_email?: string;
  provider_name?: string;
  provider_picture?: string;
  access_token?: string;
  refresh_token?: string;
  id_token?: string;
  expires_at?: string;
  scope?: string;
  token_type?: string;
  meta?: Record<string, any>;
  created_at: string;
  updated_at: string;
}

export interface RolePermission {
  role: string;
  permission_id: number;
  conditions?: Record<string, any>;
  created_at: string;
}

export interface UserPermission {
  id: string;
  user_id: string;
  permission_id: number;
  grant_type: string;
  conditions?: Record<string, any>;
  valid_from?: string;
  valid_until?: string;
  reason?: string;
  granted_by?: string;
  granted_at: string;
  is_active: boolean;
}

export interface Verification {
  id: string;
  user_id?: string;
  identifier: string;
  value: string;
  type: string;
  attempts?: number;
  max_attempts?: number;
  expires_at: string;
  used_at?: string;
  ip_address?: string;
  user_agent?: string;
  created_at: string;
}

export interface NotificationPreference {
  id: string;
  user_id: string;
  notification_type: string;
  in_app_enabled: boolean;
  email_enabled: boolean;
  sms_enabled: boolean;
  push_enabled: boolean;
  whatsapp_enabled: boolean;
  quiet_hours_enabled: boolean;
  quiet_hours_start?: string;
  quiet_hours_end?: string;
  digest_mode: string;
  created_at: string;
  updated_at: string;
}

export interface GradeHistory {
  id: string;
  grade_id: string;
  score?: number;
  grade_letter?: string;
  is_absent?: boolean;
  is_excused?: boolean;
  comment?: string;
  changed_by?: string;
  changed_at: string;
  change_reason?: string;
  change_type: string;
  snapshot?: Record<string, any>;
  created_at: string;
}

export interface AttendanceDaily {
  id: string;
  org_id: string;
  cycle_id: string;
  enrollment_id: string;
  date: string;
  status: string;
  lessons?: Record<string, any>;
  total_lessons?: number;
  present_lessons?: number;
  absent_lessons?: number;
  late_lessons?: number;
  excused_lessons?: number;
  late_minutes?: number;
  excuse_type?: string;
  excuse_document?: string;
  excuse_notes?: string;
  is_verified: boolean;
  verified_by?: string;
  verified_at?: string;
  marked_by: string;
  marked_at: string;
  mark_method?: string;
  device_id?: string;
  location?: Record<string, any>;
  notes?: string;
  created_at: string;
  updated_at: string;
}

export interface AttendanceSummary {
  id: string;
  enrollment_id: string;
  cycle_id: string;
  total_lessons?: number;
  present_lessons?: number;
  absent_lessons?: number;
  late_lessons?: number;
  excused_lessons?: number;
  late_minutes?: number;
  attendance_rate?: number;
  created_at: string;
  updated_at: string;
}

export interface LedgerAccount {
  id: string;
  org_id: string;
  code: string;
  name: string;
  name_ar?: string;
  type: string;
  sub_type?: string;
  parent_id?: string;
  path: string;
  depth: number;
  currency: string;
  opening_balance?: number;
  current_balance?: number;
  is_bank_account: boolean;
  is_cash_account: boolean;
  is_control_account: boolean;
  external_ref?: Record<string, any>;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface PaymentPlan {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string;
  type: string;
  schedule: Record<string, any>;
  eligibility_rules?: Record<string, any>;
  plan_fees?: Record<string, any>;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface StudentDiscount {
  id: string;
  enrollment_id: string;
  discount_template_id: string;
  applied_amount: number;
  applied_to_fee_id?: string;
  status: string;
  reason?: string;
  evidence_urls?: Record<string, any>;
  applied_by?: string;
  applied_at?: string;
  approved_by?: string;
  approved_at?: string;
  approval_notes?: string;
  created_at: string;
  updated_at: string;
}

export interface TreasuryTransaction {
  id: string;
  org_id: string;
  treasury_id: string;
  journal_entry_id: string;
  type: string;
  category: string;
  amount: number;
  currency: string;
  exchange_rate?: number;
  reference_type?: string;
  reference_id?: string;
  description: string;
  notes?: string;
  attachments?: Record<string, any>;
  created_by?: string;
  approved_by?: string;
  approved_at?: string;
  transaction_date: string;
  created_at: string;
}

export interface FinancialEvent {
  id: string;
  org_id: string;
  event_type: string;
  entity_type: string;
  entity_id: string;
  payload: Record<string, any>;
  sequence_number?: number;
  aggregate_id?: string;
  status: string;
  created_at: string;
}

export interface SystemLog {
  id: string;
  org_id?: string;
  level: string;
  category: string;
  message: string;
  details?: Record<string, any>;
  stack_trace?: string;
  user_id?: string;
  request_id?: string;
  endpoint?: string;
  method?: string;
  duration_ms?: number;
  memory_usage_mb?: number;
  cpu_usage_percent?: number;
  timestamp: string;
}

export interface DataTransfer {
  id: string;
  org_id: string;
  type: string;
  entity_type: string;
  file_name?: string;
  file_url?: string;
  file_size?: number;
  file_format?: string;
  total_records?: number;
  success_records?: number;
  failed_records?: number;
  skipped_records?: number;
  error_log?: Record<string, any>;
  warning_log?: Record<string, any>;
  status: string;
  started_at?: string;
  completed_at?: string;
  cancelled_at?: string;
  cancel_reason?: string;
  initiated_by?: string;
  config?: Record<string, any>;
  mapping?: Record<string, any>;
  created_at: string;
  updated_at: string;
}

export interface ReportDefinition {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string;
  description?: string;
  base_entity: string;
  columns: Record<string, any>;
  joins?: Record<string, any>;
  default_filters?: Record<string, any>;
  group_by?: string;
  sort_by?: string;
  is_system: boolean;
  created_at: string;
  updated_at: string;
}
