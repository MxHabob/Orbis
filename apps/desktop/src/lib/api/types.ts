/**
 * @module @orbis/desktop-api
 *
 * Strongly-typed API client interfaces for the Orbis Desktop application.
 * Both LocalClient (Tauri IPC) and RemoteClient (HTTP SDK) implement
 * this interface, ensuring transport-agnostic UI code.
 */

export * from "@orbis/sdk";

import type {
  LoginRequest,
  LoginResponse,
  SessionResponse,
  RegisterUserRequest,
  UpdateUserRequest,
  UserDto,
  OrgDto,
  UpdateOrgRequest,
  OrgUnitDto,
  CreateOrgUnitRequest,
  OrganizationSettingsDto,
  DashboardCountsDto,
  AcademicYearDto,
  AcademicPeriodDto,
  AcademicContextResponse,
  PeriodTemplateDto,
  CreateYearRequest,
  CreatePeriodRequest,
  SubjectDto,
  ClassDto,
  ClassSubjectDto,
  CreateSubjectRequest,
  CreateClassRequest,
  AssignClassSubjectRequest,
  EnrollmentDto,
  GuardianDto,
  StudentDto,
  EnrollStudentRequest,
  AssignGuardianRequest,
  RoomDto,
  TimeSlotDto,
  ScheduledLessonDto,
  CreateRoomRequest,
  CreateTimeSlotRequest,
  ScheduleLessonRequest,
  AssessmentSchemeDto,
  GradeDto,
  AttendanceDto,
  CalculationResultDto,
  CreateSchemeRequest,
  RecordGradeRequest,
  MarkAttendanceRequest,
  CalculateGradesRequest,
  CalculateRankingsRequest,
  RankingItemDto,
  ExamSessionDto,
  ExamScheduleDto,
  SeatingDto,
  SecretDto,
  ProctorDto,
  GenerateSeatingRequest,
  GenerateSecretsRequest,
  CreateExamSessionRequest,
  ScheduleExamRequest,
  AssignSeatingRequest,
  AssignSecretRequest,
  AssignProctorRequest,
  AccountDto,
  FeeTemplateDto,
  StudentFeeDto,
  PaymentDto,
  CurrencyDto,
  CreateAccountRequest,
  CreateFeeTemplateRequest,
  GenerateFeeRequest,
  ProcessPaymentRequest,
  DocumentDto,
  UploadDocumentRequest,
  IntegrationDto,
  ConnectAppRequest,
  RoleDto,
  SaveRoleRequest,
  PermissionRuleDto,
} from "@orbis/sdk";

// ═══════════════════════════════════════════════════════════════
// SHARED ENVELOPES
// ═══════════════════════════════════════════════════════════════

export interface ApiResponse<T> {
  data: T;
  requestId?: string;
}

export interface MfaEnrollRequest {
  factor_name?: string;
}

export interface MfaEnrollResponse {
  factor_id: string;
  secret: string;
  qr_code_uri: string;
  recovery_codes: string[];
}

export interface MfaVerifyEnrollmentRequest {
  factor_id: string;
  code: string;
}

export interface MfaChallengeRequest {
  user_id: string;
  code: string;
  device_id?: string;
  remember_device?: boolean;
}

export interface GoogleAuthRequest {
  id_token: string;
}

export interface UserDeviceDto {
  id: string;
  device_id: string;
  device_name: string;
  device_type: string;
  os?: string;
  browser?: string;
  is_trusted: boolean;
  last_active_at: string;
}

export interface UserIdentityDto {
  id: string;
  provider: string;
  provider_user_id: string;
  email?: string;
  connected_at: string;
}

export type UpdateSchemeRequest = Partial<CreateSchemeRequest>;

export interface PaginatedResponse<T> {
  items: T[];
  count: number;
  nextCursor?: string | null;
  hasMore: boolean;
}

export interface PaginatedList<T> {
  items: T[];
  total_count: number;
  page: number;
  page_size: number;
  total_pages: number;
}

export interface PersonDto {
  id: string;
  org_id?: string | null;
  first_name: string;
  middle_name?: string | null;
  last_name: string;
  first_name_ar?: string | null;
  middle_name_ar?: string | null;
  last_name_ar?: string | null;
  display_name?: string | null;
  full_name: string;
  full_name_ar?: string | null;
  national_id?: string | null;
  passport_number?: string | null;
  gender?: string | null;
  birth_date?: string | null;
  place_of_birth?: string | null;
  nationality?: string | null;
  blood_type?: string | null;
  medical_notes?: string | null;
  special_needs_notes?: string | null;
  phone?: string | null;
  email?: string | null;
  address?: string | null;
  created_at: string;
  updated_at: string;
}

export interface StudentDetailDto {
  student: StudentDto;
  person: PersonDto;
}

export interface StudentFilterParams {
  period_id?: string;
  class_id?: string;
  status?: string;
  standing?: string;
  search?: string;
  page?: number;
  page_size?: number;
  sort_by?: string;
  sort_dir?: string;
  periodId?: string;
  classId?: string;
  pageSize?: number;
  sortBy?: string;
  sortDir?: string;
}

export interface EnrollmentFilterParams {
  period_id?: string;
  class_id?: string;
  student_id?: string;
  study_mode?: string;
  status?: string;
  page?: number;
  page_size?: number;
  sort_by?: string;
  sort_dir?: string;
  periodId?: string;
  classId?: string;
  studentId?: string;
  studyMode?: string;
  pageSize?: number;
  sortBy?: string;
  sortDir?: string;
}

export interface RegisterStudentInput {
  person_id?: string;
  user_id?: string;
  student_number: string;
  first_name: string;
  middle_name?: string;
  last_name: string;
  first_name_ar?: string;
  middle_name_ar?: string;
  last_name_ar?: string;
  display_name?: string;
  national_id?: string;
  passport_number?: string;
  gender?: string;
  birth_date?: string;
  place_of_birth?: string;
  nationality?: string;
  blood_type?: string;
  medical_notes?: string;
  special_needs_notes?: string;
  phone?: string;
  email?: string;
  initial_period_id?: string;
  initial_class_id?: string;
  study_mode?: string;
  // Aliases for compatibility
  personId?: string;
  userId?: string;
  studentNumber?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;
  firstNameAr?: string;
  middleNameAr?: string;
  lastNameAr?: string;
  displayName?: string;
  nationalId?: string;
  passportNumber?: string;
  birthDate?: string;
  placeOfBirth?: string;
  bloodType?: string;
  medicalNotes?: string;
  specialNeedsNotes?: string;
  initialPeriodId?: string;
  initialClassId?: string;
  studyMode?: string;
}

export interface BulkRegisterStudentCommandItem {
  row_index: number;
  registration: RegisterStudentInput;
  guardian_name?: string;
  guardian_phone?: string;
  guardian_relationship?: string;
}

export interface BulkRegisterItemCommandResultDto {
  row_index: number;
  success: boolean;
  student_id?: string;
  student_number?: string;
  error?: string;
}

export interface BulkRegisterStudentCommandResultDto {
  total: number;
  succeeded: number;
  failed: number;
  items: BulkRegisterItemCommandResultDto[];
}

export interface UpdateStudentStatusInput {
  student_id: string;
  status: string;
  reason?: string;
  studentId?: string;
}

export interface UpdateStudentProfileInput {
  student_id: string;
  first_name?: string;
  middle_name?: string;
  last_name?: string;
  first_name_ar?: string;
  middle_name_ar?: string;
  last_name_ar?: string;
  display_name?: string;
  national_id?: string;
  passport_number?: string;
  gender?: string;
  birth_date?: string;
  place_of_birth?: string;
  nationality?: string;
  blood_type?: string;
  medical_notes?: string;
  special_needs_notes?: string;
  phone?: string;
  email?: string;
  address?: string;
  avatar_url?: string;
  // Aliases
  studentId?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;
  firstNameAr?: string;
  middleNameAr?: string;
  lastNameAr?: string;
  displayName?: string;
  nationalId?: string;
  passportNumber?: string;
  birthDate?: string;
  placeOfBirth?: string;
  bloodType?: string;
  medicalNotes?: string;
  specialNeedsNotes?: string;
  avatarUrl?: string;
}

export interface ChangeStudyModeInput {
  enrollment_id: string;
  study_mode: string;
  reason?: string;
  enrollmentId?: string;
  studyMode?: string;
}

export interface TransferClassInput {
  enrollment_id: string;
  target_class_id: string;
  reason?: string;
  enrollmentId?: string;
  targetClassId?: string;
}

export interface BehaviorRecordDto {
  id: string;
  student_id: string;
  org_id: string;
  reporter_id: string;
  record_type: string;
  description: string;
  points: number;
  created_at: string;
  updated_at: string;
  studentId?: string;
  orgId?: string;
  reporterId?: string;
  recordType?: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface RecordBehaviorInput {
  student_id: string;
  record_type: string;
  description: string;
  points: number;
  studentId?: string;
  recordType?: string;
}

export interface ReportJobStatusDto {
  job_id: string;
  status: string;
}

export interface BatchPrintParams {
  reportCode: string;
  parameters: Record<string, unknown>;
  gridColumns?: number;
  gridRows?: number;
  marginMm?: number;
  cutMarks?: boolean;
}

export interface JournalEntryLineDto {
  id: string;
  account_id: string;
  debit_amount: string | number;
  credit_amount: string | number;
  description?: string | null;
  entity_type?: string | null;
  entity_id?: string | null;
}

export interface JournalEntryDto {
  id: string;
  org_id: string;
  reference_number: string;
  accounting_date: string;
  amount: string | number;
  currency: string;
  exchange_rate: string | number;
  base_amount: string | number;
  debit_account_id: string;
  credit_account_id: string;
  description: string;
  description_ar?: string | null;
  source_type: string;
  source_id?: string | null;
  status: string;
  lines: JournalEntryLineDto[];
  is_reversed: boolean;
  reversed_by?: string | null;
  reversal_reason?: string | null;
  created_by?: string | null;
  created_at: string;
}

export interface CreateJournalEntryLineRequest {
  account_id: string;
  debit_amount: number | string;
  credit_amount: number | string;
  description?: string | null;
  entity_type?: string | null;
  entity_id?: string | null;
}

export interface CreateJournalEntryRequest {
  reference_number?: string;
  accounting_date?: string;
  amount: number | string;
  currency_code?: string;
  exchange_rate?: number | string;
  debit_account_id: string;
  credit_account_id: string;
  description: string;
  description_ar?: string;
  lines?: CreateJournalEntryLineRequest[];
}

export interface ReverseJournalRequest {
  reason: string;
}

export interface SplitFeeRequest {
  num_installments: number;
  due_dates: string[];
}

export interface RefundStudentFeeRequest {
  refund_amount: number | string;
  notes?: string;
}

export interface DiscountTemplateDto {
  id: string;
  org_id: string;
  code: string;
  name: string;
  name_ar?: string | null;
  discount_type: string;
  value: string | number;
  eligibility_rules?: Record<string, unknown> | null;
  is_approval_required: boolean;
  is_auto_apply: boolean;
  is_stackable: boolean;
  priority: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateDiscountTemplateRequest {
  code?: string;
  name: string;
  nameAr?: string;
  discountType: string;
  value: number | string;
  eligibilityRules?: Record<string, unknown>;
  isApprovalRequired?: boolean;
  isAutoApply?: boolean;
  isStackable?: boolean;
  priority?: number;
}

export interface StudentFeeFilterParams {
  periodId?: string;
  studentId?: string;
  enrollmentId?: string;
  feeTemplateId?: string;
  status?: string;
  isOverdue?: boolean;
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortDir?: string;
}

export interface PaymentFilterParams {
  feeId?: string;
  studentId?: string;
  enrollmentId?: string;
  method?: string;
  fromDate?: string;
  toDate?: string;
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortDir?: string;
}

export interface PrerequisiteCheckResultDto {
  subject_id: string;
  prerequisite_subject_id: string;
  requirement_type: string;
  is_satisfied: boolean;
  min_score_required?: number | null;
  current_score?: number | null;
  failure_reason?: string | null;
}

export interface ApprovalStepDto {
  step_order: number;
  approver_role: string;
  status: string;
  approver_id?: string | null;
  approver_name?: string | null;
  decision_at?: string | null;
  notes?: string | null;
}

export interface OverrideRequestDto {
  id: string;
  org_id: string;
  override_type: string;
  risk_level: string;
  entity_type: string;
  entity_id: string;
  field_name?: string | null;
  old_value?: string | null;
  new_value?: string | null;
  reason: string;
  requested_by: string;
  requested_by_name?: string | null;
  status: string;
  current_approver_role?: string | null;
  approval_chain: ApprovalStepDto[];
  resolved_by?: string | null;
  resolved_by_name?: string | null;
  resolved_at?: string | null;
  resolution_notes?: string | null;
  expires_at?: string | null;
  metadata: Record<string, unknown>;
  created_at: string;
  updated_at: string;
}

export interface SubmitOverrideRequestInput {
  override_type: string;
  risk_level?: string;
  entity_type: string;
  entity_id: string;
  field_name?: string;
  old_value?: string;
  new_value?: string;
  reason: string;
  approval_chain?: ApprovalStepDto[];
  metadata?: Record<string, unknown>;
  overrideType?: string;
  riskLevel?: string;
  entityType?: string;
  entityId?: string;
  fieldName?: string;
  oldValue?: string;
  newValue?: string;
  approvalChain?: ApprovalStepDto[];
}

export interface ApproveOverrideInput {
  request_id: string;
  notes?: string;
  requestId?: string;
}

export interface RejectOverrideInput {
  request_id: string;
  reason: string;
  requestId?: string;
}

export interface PromotionJobDto {
  id: string;
  org_id: string;
  source_period_id: string;
  target_period_id?: string | null;
  name: string;
  status: string;
  total_students: number;
  promoted_count: number;
  retained_count: number;
  graduated_count: number;
  excluded_count: number;
  executed_by?: string | null;
  executed_at?: string | null;
  created_at: string;
  updated_at: string;
}

export interface PromotionDetailDto {
  id: string;
  job_id: string;
  student_id: string;
  enrollment_id: string;
  source_class_id?: string | null;
  target_class_id?: string | null;
  decision: string;
  reason: string;
  created_at: string;
}

export interface StudentProgressionItemInput {
  student_id: string;
  enrollment_id: string;
  source_class_id?: string;
  target_class_id?: string;
  decision: string;
  reason?: string;
  studentId?: string;
  enrollmentId?: string;
  sourceClassId?: string;
  targetClassId?: string;
}

export interface PromoteCohortInput {
  source_period_id: string;
  target_period_id: string;
  job_name: string;
  items: StudentProgressionItemInput[];
  sourcePeriodId?: string;
  targetPeriodId?: string;
  jobName?: string;
}

export interface CumulativeGpaRecordDto {
  id: string;
  org_id: string;
  student_id: string;
  period_id: string;
  term_attempted_credits: number | string;
  term_earned_credits: number | string;
  term_points: number | string;
  term_gpa: number | string;
  cumulative_attempted_credits: number | string;
  cumulative_earned_credits: number | string;
  cumulative_points: number | string;
  cumulative_gpa: number | string;
  standing: string;
  calculated_at: string;
}

export interface CalculateGpaInput {
  student_id: string;
  period_id: string;
  studentId?: string;
  periodId?: string;
}

export class OrbisError extends Error {
  code: string;
  details?: unknown;
  requestId?: string;

  constructor(
    message: string,
    code = "UNKNOWN_ERROR",
    details?: unknown,
    requestId?: string,
  ) {
    super(message);
    this.name = "OrbisError";
    this.code = code;
    this.details = details;
    this.requestId = requestId;
  }

  static fromUnknown(err: unknown): OrbisError {
    if (err instanceof OrbisError) return err;
    if (err instanceof Error) {
      return new OrbisError(err.message, "INTERNAL_ERROR", err);
    }
    if (typeof err === "object" && err !== null) {
      const record = err as Record<string, unknown>;
      return new OrbisError(
        typeof record.message === "string" ? record.message : "Unknown error",
        typeof record.code === "string" ? record.code : "UNKNOWN_ERROR",
        record.details ?? record,
        typeof record.requestId === "string" ? record.requestId : undefined,
      );
    }
    return new OrbisError(String(err));
  }
}

// ═══════════════════════════════════════════════════════════════
// DOCFORGE ENGINE & TEMPLATE DTOs
// ═══════════════════════════════════════════════════════════════

export interface TemplateDto {
  id: string;
  name: string;
  name_ar?: string | null;
  description?: string | null;
  description_ar?: string | null;
  document_type: string;
  entity_type: string;
  status: string;
  version: number;
  published_version?: number | null;
  page_size: string;
  page_width_mm?: number | null;
  page_height_mm?: number | null;
  orientation: string;
  margins_json: string;
  document_json: string;
  locale: string;
  master_template_id?: string | null;
  code?: string | null;
  tags?: string | null;
  is_system: boolean;
  is_default: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateTemplateRequest {
  name: string;
  name_ar?: string | null;
  description?: string | null;
  description_ar?: string | null;
  document_type: string;
  entity_type: string;
  page_size?: string | null;
  page_width_mm?: number | null;
  page_height_mm?: number | null;
  orientation?: string | null;
  margins_json?: string | null;
  document_json?: string | null;
  locale?: string | null;
  code?: string | null;
  tags?: string | null;
  master_template_id?: string | null;
}

export interface UpdateTemplateRequest {
  name?: string | null;
  name_ar?: string | null;
  description?: string | null;
  description_ar?: string | null;
  page_size?: string | null;
  orientation?: string | null;
  margins_json?: string | null;
  document_json?: string | null;
}

export interface TemplateVersionDto {
  id: string;
  template_id: string;
  version: number;
  change_summary?: string | null;
  created_by: string;
  created_at: string;
}

export interface TemplateBindingDto {
  id: string;
  report_definition_id: string;
  template_id: string;
  locale: string;
  format: string;
  is_default: boolean;
  created_at: string;
}

export interface ValidationResult {
  valid: boolean;
  errors: ValidationIssue[];
  warnings: ValidationIssue[];
}

export interface ValidationIssue {
  nodeId?: string | null;
  path: string;
  message: string;
  severity: string;
}

export interface ReportDefinitionDto {
  id: string;
  code: string;
  name: string;
  name_ar: string;
  domain: string;
  category: string;
  description?: string | null;
  description_ar?: string | null;
  supported_formats: string[];
  parameters: any[];
  is_system: boolean;
  is_active: boolean;
}

export interface AuditLogDto {
  id: string;
  org_id?: string;
  user_id?: string | null;
  action: string;
  entity_type: string;
  entity_id?: string | null;
  ip_address?: string | null;
  metadata?: string | null;
  details?: Record<string, unknown> | null;
  created_at: string;
}

export interface IntegrationStatusResponse {
  connected: boolean;
  is_enabled: boolean;
  config?: IntegrationDto | null;
}

// ── Connection & Database Management Types (Desktop / Hybrid) ─────────────

export type ConnectionMode = "local" | "remote";

export interface LocalConfig {
  db_filename: string;
  custom_path?: string;
  wal_mode?: boolean;
  busy_timeout_ms?: number;
}

export interface RemoteConfig {
  server_url: string;
  host?: string;
  port?: number;
  database_name?: string;
  username?: string;
  password?: string;
  ssl_mode?: string;
  api_key?: string;
}

export interface DatabaseSettingsDto {
  auto_backup_enabled: boolean;
  auto_backup_interval_hours: number;
  backup_dir?: string;
  max_backup_files: number;
  last_backup_timestamp?: string;
  last_backup_path?: string;
  integrity_check_on_startup: boolean;
}

export interface ConnectionConfig {
  mode: ConnectionMode;
  local?: LocalConfig;
  remote?: RemoteConfig;
  settings?: DatabaseSettingsDto;
}

export interface ConnectionValidation {
  is_valid: boolean;
  mode: string;
  message: string;
  resolved_url?: string;
  resolved_path?: string;
}

export interface RemoteHealthResult {
  reachable: boolean;
  status?: string;
  version?: string;
  api_version?: string;
  latency_ms: number;
  error?: string;
}

export interface DatabaseStatusDto {
  mode: "local" | "remote";
  state: "connected" | "disconnected" | "connecting" | "migration_required" | "warning" | "error" | "not_found";
  database_name: string;
  connection_target: string;
  file_size_bytes?: number;
  file_size_formatted?: string;
  schema_version: string;
  applied_migrations_count: number;
  total_migrations_count: number;
  pending_migrations: string[];
  is_compatible: boolean;
  journal_mode?: string;
  integrity_status?: string;
  last_backup_timestamp?: string;
  last_backup_path?: string;
  last_connected_at?: string;
  error_message?: string;
}

export interface BackupResultDto {
  success: boolean;
  backup_path: string;
  file_size_bytes: number;
  file_size_formatted: string;
  duration_ms: number;
  timestamp: string;
  integrity_verified: boolean;
  message: string;
}

export interface BackupInspectionDto {
  is_valid: boolean;
  file_path: string;
  file_size_bytes: number;
  file_size_formatted: string;
  schema_version: string;
  applied_migrations_count: number;
  is_compatible: boolean;
  error?: string;
}

export interface RestoreResultDto {
  success: boolean;
  message: string;
  pre_restore_backup_path?: string;
  applied_migrations: string[];
  duration_ms: number;
}

export interface IntegrityResultDto {
  is_healthy: boolean;
  status: string;
  details: string[];
  checked_at: string;
}

export interface OptimizeResultDto {
  success: boolean;
  message: string;
  size_before_bytes: number;
  size_after_bytes: number;
  space_freed_bytes: number;
  space_freed_formatted: string;
}

export interface MigrationResultDto {
  success: boolean;
  applied_migrations: string[];
  total_applied: number;
  message: string;
}

export interface ConnectResult {
  connected: boolean;
  mode: string;
  server_url?: string;
  server_version?: string;
  message: string;
}

// ── System & Bootstrap Types ──────────────────────────────────

export interface HealthResponse {
  status: string;
  version: string;
  api_version: string;
  database_connected: boolean;
  has_organization: boolean;
  schema_version: string;
}

export type OrgLifecycleState = "empty" | "core_ready" | "operational";

export interface SetupItem {
  key: string;
  label: string;
  label_ar: string;
  is_required: boolean;
  is_completed: boolean;
}

export interface OrganizationReadiness {
  state: OrgLifecycleState;
  org_id?: string;
  org_name?: string;
  has_organization: boolean;
  has_admin: boolean;
  has_academic_year: boolean;
  has_classes: boolean;
  has_subjects: boolean;
  pending_setup_items: SetupItem[];
}

export interface BootstrapRequest {
  orgName: string;
  orgNameAr?: string;
  orgType: string;
  timezone: string;
  locale: string;
  currency: string;
  adminFullName: string;
  adminEmail: string;
  adminPassword: string;
}

export interface BootstrapResult {
  org_id: string;
  admin_user_id: string;
  token: string;
  readiness: OrganizationReadiness;
}

export interface InitialSetupRequest {
  orgId: string;
  academicYearName: string;
  academicYearNameAr?: string;
  academicYearStart: string;
  academicYearEnd: string;
  periods: Array<{ name: string; name_ar?: string; period_type: string; start_date: string; end_date: string }>;
  classes: Array<{ name: string; name_ar?: string; level: string; division?: string }>;
  subjects: Array<{ code: string; name: string; name_ar?: string; is_core: boolean }>;
}

export interface CompleteOrganizationOnboardingRequest {
  orgId?: string;
  profile: {
    institutionType: string;
    name: string;
    nameAr?: string;
    baseCurrencyCode: string;
    timezone: string;
    locale: string;
    studentNumberPattern?: string;
    requiredFields?: Record<string, unknown>;
    activeFields?: Record<string, unknown>;
    preventDuplicates?: Record<string, unknown>;
    requiredDocuments?: string[];
  };
  calendar: {
    academicYearName: string;
    academicYearNameAr?: string;
    startDate: string;
    endDate: string;
    periods: Array<{
      name: string;
      nameAr?: string;
      periodType: string;
      startDate: string;
      endDate: string;
    }>;
  };
  academics: {
    classes: Array<{
      name: string;
      nameAr?: string;
      level: string;
      division?: string;
    }>;
    subjects: Array<{
      code: string;
      name: string;
      nameAr?: string;
      isCore: boolean;
    }>;
    timeSlots?: Array<{
      name: string;
      nameAr?: string;
      startTime: string;
      endTime: string;
      dayOfWeek?: number;
      isBreak?: boolean;
      breakType?: string;
    }>;
    workingDays?: number[];
  };
  facilities?: {
    buildings: string[];
    rooms: Array<{
      name: string;
      roomType: string;
      capacity: number;
      building?: string;
    }>;
  };
  finance?: {
    initializeChartOfAccounts?: boolean;
    taxRate?: number;
    feeTemplates?: Array<{
      code: string;
      name: string;
      nameAr?: string;
      category: string;
      defaultAmount: number;
      frequency: string;
      isMandatory?: boolean;
    }>;
    discountTemplates?: Array<{
      code: string;
      name: string;
      nameAr?: string;
      discountType: string;
      value: number;
      isApprovalRequired?: boolean;
      isAutoApply?: boolean;
      isStackable?: boolean;
      priority?: number;
    }>;
  };
}

export interface OnboardingResult {
  success: boolean;
  orgId: string;
  readiness: OrganizationReadiness;
}

export interface EntitlementResult {
  status: string;
  plan?: string;
  max_users?: number;
  max_students?: number;
  features: string[];
  expires_at?: string;
  grace_period_until?: string;
}

export type StartupPhase =
  | "loading"
  | "connection_setup"
  | "connecting"
  | "bootstrap"
  | "login"
  | "ready"
  | "error";

export interface OrbisApiClient {
  // ── Auth & MFA & Devices ────────────────────────────────────
  login(req: LoginRequest): Promise<LoginResponse>;
  getSession(): Promise<SessionResponse>;
  logout(): Promise<void>;
  mfaEnroll(req?: MfaEnrollRequest): Promise<MfaEnrollResponse>;
  mfaVerifyEnrollment(req: MfaVerifyEnrollmentRequest): Promise<boolean>;
  mfaChallenge(req: MfaChallengeRequest): Promise<LoginResponse>;
  linkGoogle(req: GoogleAuthRequest): Promise<UserIdentityDto>;
  loginWithGoogle(req: GoogleAuthRequest): Promise<LoginResponse>;
  listDevices(): Promise<UserDeviceDto[]>;
  revokeDevice(id: string): Promise<boolean>;

  // ── IAM ─────────────────────────────────────────────────────
  registerUser(req: RegisterUserRequest): Promise<UserDto>;
  listUsers(): Promise<UserDto[]>;
  getUser(id: string): Promise<UserDto>;
  updateUser(id: string, req: UpdateUserRequest): Promise<UserDto>;
  resetUserPassword(userId: string): Promise<AccountProvisioningResultDto>;
  listRoles(): Promise<RoleDto[]>;
  saveRole(req: SaveRoleRequest): Promise<RoleDto>;
  deleteRole(id: string): Promise<boolean>;
  listPermissions(): Promise<PermissionRuleDto[]>;
  savePermissions(req: PermissionRuleDto[]): Promise<boolean>;

  // ── Organization ────────────────────────────────────────────
  getCurrentOrg(): Promise<OrgDto>;
  updateCurrentOrg(req: UpdateOrgRequest): Promise<OrgDto>;
  getSettings(): Promise<OrganizationSettingsDto>;
  updateSettings(
    settings: OrganizationSettingsDto,
  ): Promise<OrganizationSettingsDto>;
  getDashboard(): Promise<DashboardCountsDto>;

  // ── OrgUnits ────────────────────────────────────────────────
  createOrgUnit(req: CreateOrgUnitRequest): Promise<OrgUnitDto>;
  listOrgUnits(): Promise<OrgUnitDto[]>;
  getOrgUnit(id: string): Promise<OrgUnitDto>;
  deleteOrgUnit(id: string): Promise<void>;
  selectOrgUnit(id?: string): Promise<void>;

  // ── Calendar ────────────────────────────────────────────────
  createAcademicYear(req: CreateYearRequest): Promise<AcademicYearDto>;
  listAcademicYears(): Promise<AcademicYearDto[]>;
  getAcademicContext(): Promise<AcademicContextResponse>;
  getAcademicYear(id: string): Promise<AcademicYearDto>;
  createAcademicPeriod(req: CreatePeriodRequest): Promise<AcademicPeriodDto>;
  listAcademicPeriods(): Promise<AcademicPeriodDto[]>;
  getAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  lockAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  unlockAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  closeAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  listPeriodTemplates(): Promise<PeriodTemplateDto[]>;

  // ── Academic ────────────────────────────────────────────────
  createSubject(req: CreateSubjectRequest): Promise<SubjectDto>;
  listSubjects(): Promise<SubjectDto[]>;
  getSubject(id: string): Promise<SubjectDto>;
  createClass(req: CreateClassRequest): Promise<ClassDto>;
  listClasses(): Promise<ClassDto[]>;
  getClass(id: string): Promise<ClassDto>;
  assignClassSubject(req: AssignClassSubjectRequest): Promise<ClassSubjectDto>;
  listClassSubjects(
    classId: string,
    periodId: string,
  ): Promise<ClassSubjectDto[]>;
  checkPrerequisites(
    studentId: string,
    subjectId: string,
  ): Promise<PrerequisiteCheckResultDto[]>;

  // ── Student ─────────────────────────────────────────────────
  registerStudent(req: RegisterStudentInput): Promise<StudentDetailDto>;
  checkDuplicate(req: CheckDuplicateInput): Promise<DuplicateCheckResultDto>;
  listStudents(filter?: StudentFilterParams): Promise<PaginatedList<StudentDetailDto>>;
  getStudent(id: string): Promise<StudentDetailDto>;
  updateStudentStatus(req: UpdateStudentStatusInput): Promise<StudentDto>;
  updateStudentProfile(req: UpdateStudentProfileInput): Promise<StudentDetailDto>;
  changeStudyMode(req: ChangeStudyModeInput): Promise<EnrollmentDto>;
  transferClass(req: TransferClassInput): Promise<EnrollmentDto>;
  enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto>;
  listEnrollments(filter?: EnrollmentFilterParams): Promise<EnrollmentDto[]>;
  getEnrollment(id: string): Promise<EnrollmentDto>;
  assignGuardian(req: AssignGuardianRequest): Promise<GuardianDto>;
  bulkRegisterStudents(items: BulkRegisterStudentCommandItem[], options?: { atomic?: boolean; dryRun?: boolean }): Promise<BulkRegisterStudentCommandResultDto>;

  // ── Employee Affairs ────────────────────────────────────────
  registerEmployee(req: RegisterEmployeeRequest): Promise<RegisterEmployeeResponseDto>;
  listEmployees(filter?: EmployeeFilterInput): Promise<PaginatedEmployeesDto>;
  getEmployee(id: string): Promise<EmployeeDetailDto>;
  getEmployeeStats(): Promise<Record<string, number>>;
  updateEmployee(id: string, req: UpdateEmployeeRequest): Promise<EmployeeDetailDto>;
  updateEmployeeStatus(id: string, status: string): Promise<EmployeeDetailDto>;
  provisionEmployeeAccount(id: string, req: ProvisionEmployeeAccountInput): Promise<AccountProvisioningResultDto>;
  assignEmployeeGroups(id: string, groupIds: string[]): Promise<boolean>;
  resetEmployeePassword(id: string): Promise<AccountProvisioningResultDto>;

  // ── User Groups ─────────────────────────────────────────────
  createGroup(req: CreateGroupRequest): Promise<GroupDetailDto>;
  listGroups(): Promise<GroupDetailDto[]>;
  getGroup(id: string): Promise<GroupDetailDto>;
  updateGroup(id: string, req: UpdateGroupRequest): Promise<GroupDetailDto>;
  deleteGroup(id: string): Promise<boolean>;
  getGroupMembers(id: string): Promise<GroupMemberInfoDto[]>;
  addGroupMember(id: string, userId: string): Promise<boolean>;
  removeGroupMember(id: string, userId: string): Promise<boolean>;
  bulkAssignGroup(req: BulkAssignGroupRequest): Promise<number>;
  setGroupPermissions(id: string, permissions: string[]): Promise<boolean>;

  // ── Scheduling ──────────────────────────────────────────────
  createRoom(req: CreateRoomRequest): Promise<RoomDto>;
  listRooms(): Promise<RoomDto[]>;
  getRoom(id: string): Promise<RoomDto>;
  createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto>;
  listTimeSlots(periodId?: string): Promise<TimeSlotDto[]>;
  scheduleLesson(req: ScheduleLessonRequest): Promise<ScheduledLessonDto>;
  listScheduledLessons(
    filterOrPeriodId?:
      | string
      | {
          classSubjectId?: string;
          periodId?: string;
        },
    classSubjectId?: string,
  ): Promise<ScheduledLessonDto[]>;

  // ── Assessment ──────────────────────────────────────────────
  createAssessmentScheme(
    req: CreateSchemeRequest,
  ): Promise<AssessmentSchemeDto>;
  updateAssessmentScheme(
    id: string,
    req: UpdateSchemeRequest,
  ): Promise<AssessmentSchemeDto>;
  deleteAssessmentScheme(id: string): Promise<void>;
  listAssessmentSchemes(): Promise<AssessmentSchemeDto[]>;
  getAssessmentScheme(id: string): Promise<AssessmentSchemeDto>;
  recordGrade(req: RecordGradeRequest): Promise<GradeDto>;
  listGrades(filter?: {
    studentId?: string;
    classSubjectId?: string;
  }): Promise<GradeDto[]>;
  calculateGrades(req: CalculateGradesRequest): Promise<CalculationResultDto>;
  calculateGpa(req: CalculateGpaInput): Promise<CumulativeGpaRecordDto>;
  calculateRankings(req: CalculateRankingsRequest): Promise<RankingItemDto[]>;
  markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto>;
  listAttendance(classId: string, date: string): Promise<AttendanceDto[]>;
  recordBehavior(req: RecordBehaviorInput): Promise<BehaviorRecordDto>;
  listStudentBehaviorRecords(studentId: string): Promise<BehaviorRecordDto[]>;

  // ── Student Cohort Progression ──────────────────────────────
  evaluateCohort(sourcePeriodId: string): Promise<StudentProgressionItemInput[]>;
  promoteCohort(req: PromoteCohortInput): Promise<PromotionJobDto>;
  listPromotionJobs(): Promise<PromotionJobDto[]>;
  getPromotionJob(id: string): Promise<PromotionJobDto>;

  // ── Workflow & Academic Overrides ───────────────────────────
  submitOverride(req: SubmitOverrideRequestInput): Promise<OverrideRequestDto>;
  approveOverride(req: ApproveOverrideInput): Promise<OverrideRequestDto>;
  rejectOverride(req: RejectOverrideInput): Promise<OverrideRequestDto>;
  listOverrides(status?: string): Promise<OverrideRequestDto[]>;
  getOverride(id: string): Promise<OverrideRequestDto>;

  // ── Exam ────────────────────────────────────────────────────
  createExamSession(req: CreateExamSessionRequest): Promise<ExamSessionDto>;
  listExamSessions(periodId: string): Promise<ExamSessionDto[]>;
  getExamSession(id: string): Promise<ExamSessionDto>;
  scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto>;
  listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]>;
  assignSeating(req: AssignSeatingRequest): Promise<SeatingDto>;
  listSeating(sessionId: string): Promise<SeatingDto[]>;
  generateSeating(req: GenerateSeatingRequest): Promise<SeatingDto[]>;
  assignSecret(req: AssignSecretRequest): Promise<SecretDto>;
  listSecrets(scheduleId: string): Promise<SecretDto[]>;
  generateSecrets(req: GenerateSecretsRequest): Promise<SecretDto[]>;
  assignProctor(req: AssignProctorRequest): Promise<ProctorDto>;

  // ── Finance ─────────────────────────────────────────────────
  createAccount(req: CreateAccountRequest): Promise<AccountDto>;
  listAccounts(): Promise<AccountDto[]>;
  createFeeTemplate(req: CreateFeeTemplateRequest): Promise<FeeTemplateDto>;
  updateFeeTemplate(id: string, req: Partial<CreateFeeTemplateRequest>): Promise<FeeTemplateDto>;
  deleteFeeTemplate(id: string): Promise<boolean>;
  listFeeTemplates(): Promise<FeeTemplateDto[]>;
  generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto>;
  listStudentFees(filter?: StudentFeeFilterParams): Promise<StudentFeeDto[]>;
  processPayment(req: ProcessPaymentRequest): Promise<PaymentDto>;
  listPayments(filter?: PaymentFilterParams): Promise<PaymentDto[]>;
  listCurrencies(): Promise<CurrencyDto[]>;
  listJournalEntries(): Promise<JournalEntryDto[]>;
  getJournalEntry(id: string): Promise<JournalEntryDto>;
  createJournalEntry(req: CreateJournalEntryRequest): Promise<JournalEntryDto>;
  reverseJournalEntry(
    id: string,
    req: ReverseJournalRequest,
  ): Promise<JournalEntryDto>;
  splitStudentFee(
    id: string,
    req: SplitFeeRequest,
  ): Promise<StudentFeeDto[]>;
  refundStudentFee(
    id: string,
    req: RefundStudentFeeRequest,
  ): Promise<StudentFeeDto>;
  listDiscountTemplates(): Promise<DiscountTemplateDto[]>;
  createDiscountTemplate(
    req: CreateDiscountTemplateRequest,
  ): Promise<DiscountTemplateDto>;

  // ── Document ────────────────────────────────────────────────
  uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto>;
  listDocuments(): Promise<DocumentDto[]>;
  getDocument(id: string): Promise<DocumentDto>;
  deleteDocument(id: string): Promise<void>;
  createDocumentRequirement(req: CreateDocumentRequirementRequest): Promise<DocumentRequirementDto>;
  listDocumentRequirements(): Promise<DocumentRequirementDto[]>;
  deleteDocumentRequirement(id: string): Promise<void>;
  evaluateStudentDocumentRequirements(
    studentId: string,
    params?: {
      grade_level_id?: string;
      is_national?: boolean;
      is_transfer?: boolean;
    },
  ): Promise<StudentDocumentFulfillmentDto[]>;

  // ── Integration ─────────────────────────────────────────────
  connectApp(req: ConnectAppRequest): Promise<IntegrationDto>;
  listIntegrations(): Promise<IntegrationDto[]>;
  getIntegrationStatus(
    providerName: string,
  ): Promise<IntegrationStatusResponse>;
  disconnectApp(providerName: string): Promise<boolean>;
  toggleIntegration(providerName: string, isEnabled: boolean): Promise<boolean>;

  // ── Templates ───────────────────────────────────────────────
  createTemplate(req: CreateTemplateRequest): Promise<TemplateDto>;
  getTemplate(id: string): Promise<TemplateDto>;
  listTemplates(filter?: {
    documentType?: string;
    entityType?: string;
    status?: string;
  }): Promise<TemplateDto[]>;
  updateTemplate(id: string, req: UpdateTemplateRequest): Promise<TemplateDto>;
  deleteTemplate(id: string): Promise<void>;
  publishTemplate(id: string): Promise<TemplateDto>;
  archiveTemplate(id: string): Promise<void>;
  getTemplateVersions(id: string): Promise<TemplateVersionDto[]>;
  revertTemplateVersion(
    id: string,
    targetVersion: number,
  ): Promise<TemplateDto>;
  previewTemplate(
    documentJson: string,
    contextData: any,
    locale?: string,
  ): Promise<Uint8Array>;
  validateTemplate(documentJson: string): Promise<ValidationResult>;

  // ── Template Bindings ───────────────────────────────────────
  listTemplateBindings(definitionId: string): Promise<TemplateBindingDto[]>;
  createTemplateBinding(
    definitionId: string,
    templateId: string,
    locale: string,
    format: string,
  ): Promise<TemplateBindingDto>;

  // ── Report Definitions ──────────────────────────────────────
  listReportDefinitions(
    domain?: string,
    category?: string,
  ): Promise<ReportDefinitionDto[]>;
  getReportDefinition(code: string): Promise<ReportDefinitionDto>;

  // ── Report Execution & Printing & Audit Logs ───────────────
  generateReport(
    templateId: string,
    format: string,
    parameters: Record<string, unknown>,
  ): Promise<string>;
  getReportJobStatus(jobId: string): Promise<ReportJobStatusDto>;
  downloadReportFile(jobId: string): Promise<Uint8Array>;
  batchPrint(req: BatchPrintParams): Promise<string>;
  listAuditLogs(limit?: number, offset?: number): Promise<AuditLogDto[]>;
  getEntityAudit(
    entityType: string,
    entityId: string,
    limit?: number,
    offset?: number,
  ): Promise<AuditLogDto[]>;

  // ── System & Bootstrap ──────────────────────────────────────
  systemHealth(): Promise<HealthResponse>;
  systemHasOrganization(): Promise<boolean>;
  systemGetReadiness(): Promise<OrganizationReadiness>;
  systemGetEntitlement(): Promise<EntitlementResult>;
  bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult>;
  bootstrapInitialSetup(req: InitialSetupRequest): Promise<OrganizationReadiness>;
  completeOnboarding(req: CompleteOrganizationOnboardingRequest): Promise<OnboardingResult>;

  // ── Connection & Database Management ─────────────────────────
  connectionGetConfig(): Promise<ConnectionConfig>;
  connectionSetConfig(params: {
    mode: ConnectionMode;
    serverUrl?: string;
    dbFilename?: string;
    customPath?: string;
    host?: string;
    port?: number;
    databaseName?: string;
    username?: string;
    password?: string;
    sslMode?: string;
    walMode?: boolean;
    busyTimeoutMs?: number;
  }): Promise<ConnectionConfig>;
  connectionReset(): Promise<ConnectionConfig>;
  connectionValidate(): Promise<ConnectionValidation>;
  connectionCheckRemoteHealth(serverUrl: string): Promise<RemoteHealthResult>;
  connectionConnect(): Promise<ConnectResult>;
  connectionOpenWindow(): Promise<void>;

  databaseGetStatus(): Promise<DatabaseStatusDto>;
  databaseCreateLocal(params: { customDir?: string; filename: string }): Promise<DatabaseStatusDto>;
  databaseOpenExisting(filePath: string): Promise<DatabaseStatusDto>;
  databaseBackup(destinationPath?: string): Promise<BackupResultDto>;
  databaseValidateBackupFile(filePath: string): Promise<BackupInspectionDto>;
  databaseRestore(backupFilePath: string): Promise<RestoreResultDto>;
  databaseRunMigrations(): Promise<MigrationResultDto>;
  databaseCheckIntegrity(): Promise<IntegrityResultDto>;
  databaseOptimize(): Promise<OptimizeResultDto>;
  databaseGetSettings(): Promise<DatabaseSettingsDto>;
  databaseUpdateSettings(settings: DatabaseSettingsDto): Promise<DatabaseSettingsDto>;
  databasePickFile(params?: { filterName?: string; extensions?: string[] }): Promise<string | null>;
  databasePickFolder(): Promise<string | null>;
  databasePickSavePath(params?: { defaultFilename?: string; filterName?: string; extensions?: string[] }): Promise<string | null>;

  // ── AI Platform ─────────────────────────────────────────────
  aiChat(req: AiChatRequest): Promise<AiChatResponse>;
  aiListTools(): Promise<AiToolDefinition[]>;
  aiSaveSession(session: AiStoredSession): Promise<boolean>;
  aiLoadSessions(orgId: string): Promise<AiStoredSession[]>;
  aiDeleteSession(orgId: string, sessionId: string): Promise<boolean>;
  aiSaveContext(orgId: string, context: Record<string, unknown>): Promise<boolean>;
  aiLoadContext(orgId: string): Promise<Record<string, unknown> | null>;

  // ── Autopilot System ────────────────────────────────────────
  autopilotListTasks(params?: {
    status?: string;
    task_type?: string;
    limit?: number;
    offset?: number;
  }): Promise<ListTasksResponse>;
  autopilotRunTick(dryRun?: boolean): Promise<AutopilotTaskDto[]>;
  autopilotGetTask(id: string): Promise<AutopilotTaskDto>;
  autopilotExecuteTask(id: string): Promise<AutopilotTaskDto>;
  autopilotRetryTask(id: string): Promise<AutopilotTaskDto>;
  autopilotDismissTask(id: string): Promise<AutopilotTaskDto>;
  autopilotGetDiagnostics(): Promise<AutopilotDiagnosticsDto>;
}

// ── AI Platform Interfaces ───────────────────────────────────

export interface AiToolCall {
  id: string;
  tool_name: string;
  arguments: Record<string, unknown>;
}

export interface AiToolResult {
  tool_call_id: string;
  tool_name: string;
  content: string;
  is_error: boolean;
}

export interface AiMessage {
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  tool_calls?: AiToolCall[];
  tool_call_id?: string;
  created_at?: string;
}

export interface AiToolDefinition {
  name: string;
  description: string;
  parameters_schema: Record<string, unknown>;
  required_permission?: string | null;
  is_consequential: boolean;
}

export interface AiProposedAction {
  tool_name: string;
  title: string;
  description: string;
  payload: Record<string, unknown>;
  requires_confirmation: boolean;
}

export interface AiChatRequest {
  messages: AiMessage[];
  screen_context?: Record<string, unknown> | null;
  agent_type?: string | null;
  model?: string | null;
}

export interface AiChatResponse {
  response: string;
  executed_tools: AiToolResult[];
  proposed_actions: AiProposedAction[];
}

export interface AiStoredMessage {
  id: string;
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  timestamp: number;
  executed_tools?: AiToolResult[];
  proposed_actions?: Record<string, unknown>[];
}

export interface AiStoredSession {
  id: string;
  title: string;
  org_id: string;
  created_at: number;
  updated_at: number;
  context?: Record<string, unknown> | null;
  messages: AiStoredMessage[];
}

// ── Autopilot System Interfaces ───────────────────────────────

export interface AutopilotTaskDto {
  id: string;
  org_id: string;
  task_type: string;
  status: string;
  target_id?: string | null;
  target_type?: string | null;
  risk_level: string;
  reason?: string | null;
  parameters?: Record<string, unknown> | null;
  scheduled_at: string;
  executed_at?: string | null;
  attempts: number;
  max_retries: number;
  error_log?: string | null;
  idempotency_key?: string | null;
  triggered_by: string;
  created_at: string;
  updated_at: string;
}

export interface AutopilotDiagnosticsDto {
  is_healthy: boolean;
  status: string;
  mode: string;
  stuck_tasks: AutopilotTaskDto[];
  overdue_tasks: AutopilotTaskDto[];
  total_tasks_count: number;
  pending_tasks_count: number;
  completed_tasks_count: number;
  failed_tasks_count: number;
}

export interface ListTasksResponse {
  total_count: number;
  items: AutopilotTaskDto[];
}

// ── Employee Affairs Interfaces ───────────────────────────────

export interface EmployeeDetailDto {
  id: string;
  org_id: string;
  person_id: string;
  user_id?: string | null;
  employee_number: string;
  employee_type: string;
  job_title?: string | null;
  department?: string | null;
  org_unit_id?: string | null;
  org_unit_name?: string | null;
  hire_date: string;
  status: string;
  employment_type: string;
  work_shift?: string | null;
  notes?: string | null;

  // Person demographic fields
  national_id?: string | null;
  passport_number?: string | null;
  first_name: string;
  middle_name?: string | null;
  last_name: string;
  first_name_ar?: string | null;
  middle_name_ar?: string | null;
  last_name_ar?: string | null;
  display_name?: string | null;
  full_name: string;
  full_name_ar?: string | null;
  gender?: string | null;
  birth_date?: string | null;
  place_of_birth?: string | null;
  nationality?: string | null;
  blood_type?: string | null;
  medical_notes?: string | null;
  special_needs_notes?: string | null;
  phone?: string | null;
  email?: string | null;
  address?: string | null;
  avatar_url?: string | null;

  // Linked user account
  username?: string | null;
  user_status?: string | null;
  roles: string[];
  groups: string[];
  effective_permissions: string[];

  created_at: string;
  updated_at: string;
}

export interface AccountProvisioningResultDto {
  user_id: string;
  username: string;
  temporary_password?: string | null;
  must_change_password: boolean;
  delivery_channel: string;
  delivery_status: string;
}

export interface RegisterEmployeeResponseDto {
  employee: EmployeeDetailDto;
  provisioning?: AccountProvisioningResultDto | null;
}

export interface PaginatedEmployeesDto {
  items: EmployeeDetailDto[];
  total_count: number;
  page: number;
  page_size: number;
  total_pages: number;
}

export interface AccountProvisioningConfigInput {
  enabled: boolean;
  preferred_username?: string | null;
  role?: string | null;
  group_ids?: string[];
  delivery_channel?: "email" | "sms" | "whatsapp" | "manual" | "auto" | null;
  custom_password?: string | null;
}

export interface RegisterEmployeeRequest {
  person_id?: string | null;
  employee_number?: string | null;
  employee_type: string;
  job_title?: string | null;
  department?: string | null;
  org_unit_id?: string | null;
  hire_date?: string | null;
  status?: string | null;
  employment_type?: string | null;
  work_shift?: string | null;
  notes?: string | null;

  first_name: string;
  middle_name?: string | null;
  last_name: string;
  first_name_ar?: string | null;
  middle_name_ar?: string | null;
  last_name_ar?: string | null;
  display_name?: string | null;
  national_id?: string | null;
  passport_number?: string | null;
  gender?: string | null;
  birth_date?: string | null;
  place_of_birth?: string | null;
  nationality?: string | null;
  blood_type?: string | null;
  medical_notes?: string | null;
  special_needs_notes?: string | null;
  phone?: string | null;
  email?: string | null;
  address?: string | null;
  avatar_url?: string | null;

  account?: AccountProvisioningConfigInput | null;
}

export interface UpdateEmployeeRequest {
  employee_type?: string | null;
  job_title?: string | null;
  department?: string | null;
  org_unit_id?: string | null;
  hire_date?: string | null;
  status?: string | null;
  employment_type?: string | null;
  work_shift?: string | null;
  notes?: string | null;

  first_name?: string | null;
  middle_name?: string | null;
  last_name?: string | null;
  first_name_ar?: string | null;
  middle_name_ar?: string | null;
  last_name_ar?: string | null;
  display_name?: string | null;
  national_id?: string | null;
  passport_number?: string | null;
  gender?: string | null;
  birth_date?: string | null;
  place_of_birth?: string | null;
  nationality?: string | null;
  blood_type?: string | null;
  medical_notes?: string | null;
  special_needs_notes?: string | null;
  phone?: string | null;
  email?: string | null;
  address?: string | null;
  avatar_url?: string | null;
}

export interface EmployeeFilterInput {
  query?: string;
  employee_type?: string;
  department?: string;
  status?: string;
  org_unit_id?: string;
  page?: number;
  page_size?: number;
}

export interface ProvisionEmployeeAccountInput {
  preferred_username?: string | null;
  role?: string | null;
  group_ids?: string[];
  delivery_channel?: string | null;
}

// ── User Groups Interfaces ────────────────────────────────────

export interface GroupDetailDto {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  description?: string | null;
  is_system: boolean;
  status: string;
  member_count: number;
  permissions: string[];
  created_at: string;
  updated_at: string;
}

export interface GroupMemberInfoDto {
  user_id: string;
  person_id: string;
  full_name: string;
  full_name_ar?: string | null;
  email?: string | null;
  phone?: string | null;
  avatar_url?: string | null;
  user_status: string;
  joined_at: string;
}

export interface CreateGroupRequest {
  name: string;
  name_ar?: string | null;
  description?: string | null;
  permissions?: string[];
}

export interface UpdateGroupRequest {
  name?: string | null;
  name_ar?: string | null;
  description?: string | null;
  status?: string | null;
  permissions?: string[];
}

export interface BulkAssignGroupRequest {
  group_id: string;
  user_ids: string[];
}

// ── Student Duplicate Check Interfaces ────────────────────────

export interface CheckDuplicateInput {
  national_id?: string;
  phone?: string;
  email?: string;
  exclude_student_id?: string;
}

export interface DuplicateMatchDto {
  field: string;
  value: string;
  matched_person_id: string;
  matched_person_name: string;
  student_id?: string;
  student_number?: string;
}

export interface DuplicateCheckResultDto {
  has_duplicate: boolean;
  matches: DuplicateMatchDto[];
}

// ── Document Requirement Interfaces ───────────────────────────

export interface DocumentRequirementDto {
  id: string;
  org_id: string;
  target_type: string;
  target_id?: string;
  code: string;
  name_ar: string;
  name_en: string;
  is_mandatory: boolean;
  blocks_enrollment: boolean;
  condition: string;
  created_at: string;
  updated_at: string;
}

export interface CreateDocumentRequirementRequest {
  target_type: string;
  target_id?: string;
  code: string;
  name_ar: string;
  name_en?: string;
  is_mandatory: boolean;
  blocks_enrollment: boolean;
  condition?: string;
}

export interface StudentDocumentFulfillmentDto {
  requirement: DocumentRequirementDto;
  is_fulfilled: boolean;
  is_applicable: boolean;
  document_id?: string;
  document_name?: string;
  file_path?: string;
  uploaded_at?: string;
}



