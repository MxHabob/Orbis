/**
 * @module @orbis/desktop-api
 *
 * Strongly-typed API client interfaces for the Orbis Desktop application.
 * Both LocalClient (Tauri IPC) and RemoteClient (HTTP SDK) implement
 * this interface, ensuring transport-agnostic UI code.
 *
 * Canonical DTOs and API models are derived directly from the OpenAPI contract
 * exported via `@orbis/sdk`.
 */

export * from "@orbis/sdk";

import type {
  LoginRequest,
  LoginResponse,
  SessionResponse,
  MfaEnrollRequest,
  MfaEnrollResponse,
  MfaVerifyEnrollmentRequest,
  MfaChallengeRequest,
  GoogleAuthRequest,
  UserDeviceDto,
  UserIdentityDto,
  RegisterUserRequest,
  UpdateUserRequest,
  ProvisionUserRequest,
  UserDto,
  RoleDto,
  SaveRoleRequest,
  PermissionRuleDto,
  OrgDto,
  UpdateOrgRequest,
  OrgUnitDto,
  CreateOrgUnitRequest,
  UpdateOrgUnitRequest,
  OrganizationSettingsDto,
  DashboardCountsDto,
  AcademicYearDto,
  AcademicPeriodDto,
  AcademicContextResponse,
  PeriodTemplateDto,
  CreateYearRequest,
  UpdateYearRequest,
  CreatePeriodRequest,
  UpdatePeriodRequest,
  SubjectDto,
  ClassDto,
  ClassSubjectDto,
  CreateSubjectRequest,
  CreateClassRequest,
  AssignClassSubjectRequest,
  EnrollmentDto,
  StudentDto,
  EnrollStudentRequest,
  GuardianDto,
  AssignGuardianRequest,
  EmployeeDetailDto,
  RegisterEmployeeRequest,
  RegisterEmployeeResponseDto,
  BulkRegisterEmployeeItemRequest,
  BulkRegisterEmployeesRequest,
  BulkRegisterEmployeeItemResultDto,
  BulkRegisterEmployeesResponseDto,
  PaginatedEmployeesDto,
  UpdateEmployeeRequest,
  UpdateEmployeeStatusRequest,
  ProvisionEmployeeAccountRequest,
  AssignEmployeeGroupsRequest,
  AccountProvisioningResultDto,
  GroupDetailDto,
  GroupMemberInfoDto,
  CreateGroupRequest,
  UpdateGroupRequest,
  AddGroupMemberRequest,
  BulkAssignGroupRequest,
  SetGroupPermissionsRequest,
  OverrideRequestDto,
  SubmitOverrideRequest,
  ResolveOverrideRequest,
  ApprovalStepDto,
  StudentProgressionItemInput,
  PromotionJobDto,
  PromoteCohortRequest,
  EvaluateCohortRequest,
  RoomDto,
  TimeSlotDto,
  ScheduledLessonDto,
  CreateRoomRequest,
  UpdateRoomRequest,
  CreateTimeSlotRequest,
  CreateTimeSlotItemRequest,
  BatchCreateTimeSlotsRequest,
  UpdateTimeSlotRequest,
  ScheduleLessonRequest,
  BulkScheduleLessonsRequest,
  AssessmentSchemeDto,
  GradeDto,
  AttendanceDto,
  BehaviorRecordDto,
  CumulativeGpaRecordDto,
  CalculationResultDto,
  CreateSchemeRequest,
  UpdateSchemeRequest,
  RecordGradeRequest,
  RecordGradeItemRequest,
  BatchRecordGradesRequest,
  BatchRecordGradesResponseDto,
  MarkAttendanceRequest,
  MarkAttendanceItemRequest,
  BatchMarkAttendanceRequest,
  BatchMarkAttendanceResponseDto,
  RecordBehaviorRequest,
  CalculateGradesRequest,
  CalculateGpaRequest,
  CalculateRankingsRequest,
  ScoreItemRequest,
  RankingItemDto,
  ExamSessionDto,
  ExamScheduleDto,
  SeatingDto,
  SecretDto,
  ProctorDto,
  GenerateSeatingRequest,
  GenerateSecretsRequest,
  CreateExamSessionRequest,
  UpdateExamSessionRequest,
  ScheduleExamRequest,
  AssignSeatingRequest,
  AssignSecretRequest,
  AssignProctorRequest,
  AccountDto,
  CreateAccountRequest,
  UpdateAccountRequest,
  FeeTemplateDto,
  CreateFeeTemplateRequest,
  UpdateFeeTemplateRequest,
  DiscountTemplateDto,
  CreateDiscountTemplateRequest,
  StudentFeeDto,
  GenerateFeeRequest,
  PaymentDto,
  ProcessPaymentRequest,
  CurrencyDto,
  FolderDto,
  FolderContentsDto,
  CreateFolderRequest,
  UpdateFolderRequest,
  CreateCurriculumFolderRequest,
  DocumentDto,
  UploadDocumentRequest,
  UpdateDocumentRequest,
  DocumentRequirementDto,
  CreateDocumentRequirementRequest,
  UpdateDocumentRequirementRequest,
  StudentDocumentFulfillmentDto,
  PresignedUrlResponseDto,
  PresignRequest,
  ArchiveRequest,
  IntegrationDto,
  ConnectAppRequest,
  TemplateDto,
  CreateTemplateRequest,
  UpdateTemplateRequest,
  TemplateVersionDto,
  TemplateBindingDto,
  ValidationResultDto,
  ValidationIssueDto,
  ReportDefinitionDto,
  JournalEntryDto,
  CreateJournalEntryRequest,
  ReverseJournalRequest,
  SplitFeeRequest,
  RefundStudentFeeRequest,
  DuplicateCheckRequest,
  DuplicateCheckResponse,
  AiChatRequest,
  AiChatResponse,
  AiToolDefinition,
  AiStoredSession,
  AutopilotTaskDto,
  AutopilotDiagnosticsDto,
  ListTasksResponse,
} from "@orbis/sdk";

// ═══════════════════════════════════════════════════════════════
// SHARED ENVELOPES & ALIASES
// ═══════════════════════════════════════════════════════════════

export interface ApiResponse<T> {
  data: T;
  requestId?: string;
}


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
  enrollment?: EnrollmentDto | null;
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
  student_number?: string;
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

export interface StudentFeeFilterParams {
  period_id?: string;
  student_id?: string;
  enrollment_id?: string;
  feeTemplate_id?: string;
  status?: string;
  is_overdue?: boolean;
  page?: number;
  page_size?: number;
  sort_by?: string;
  sort_dir?: string;
}

export interface PaymentFilterParams {
  fee_id?: string;
  student_id?: string;
  enrollment_id?: string;
  method?: string;
  from_date?: string;
  to_date?: string;
  page?: number;
  page_size?: number;
  sort_by?: string;
  sort_dir?: string;
}

export interface StatementTransactionItemDto {
  id: string;
  date: string;
  reference_number: string;
  source_type: string;
  source_id?: string | null;
  description: string;
  description_ar?: string | null;
  debit: number | string;
  credit: number | string;
  running_balance: number | string;
  contra_account_name?: string | null;
}

export interface StatementResponseDto {
  account_id?: string | null;
  account_code?: string | null;
  account_name?: string | null;
  account_name_ar?: string | null;
  entity_type?: string | null;
  entity_id?: string | null;
  currency: string;
  opening_balance: number | string;
  total_debit: number | string;
  total_credit: number | string;
  closing_balance: number | string;
  transactions: StatementTransactionItemDto[];
}

export interface StatementFilterParams {
  account_id?: string;
  entity_type?: string;
  entity_id?: string;
  start_date?: string;
  end_date?: string;
  currency_code?: string;
  currency?: string;
}

export interface ProcessDisbursementRequest {
  treasury_account_id: string;
  beneficiary_account_id: string;
  amount: number | string;
  currency_code?: string;
  payment_method?: string;
  reference?: string;
  beneficiary_name?: string;
  beneficiary_type?: string;
  beneficiary_id?: string;
  notes?: string;
  voucher_date?: string;
}

export interface CreateSubAccountRequest {
  parent_id?: string;
  parent_code?: string;
  code?: string;
  name: string;
  name_ar?: string;
  entity_type?: string;
  entity_id?: string;
  currency_code?: string;
}

export interface SettleGatewayPayoutRequest {
  clearing_account_id?: string;
  bank_account_id: string;
  gross_amount: number | string;
  fee_amount: number | string;
  net_amount: number | string;
  payout_reference: string;
  accounting_date?: string;
}

export interface PayrollItemInputDto {
  employee_id: string;
  employee_name: string;
  is_academic: boolean;
  base_salary: number | string;
  allowances: number | string;
  deductions: number | string;
  gosi_deduction: number | string;
  net_salary: number | string;
}

export interface PostPayrollRequest {
  payroll_period: string;
  accounting_date: string;
  items: PayrollItemInputDto[];
}

export interface DisbursePayrollRequest {
  bank_account_id: string;
  payroll_period: string;
  amount: number | string;
  notes?: string;
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

export interface PromoteCohortInput {
  source_period_id: string;
  target_period_id: string;
  job_name: string;
  items: StudentProgressionItemInput[];
  sourcePeriodId?: string;
  targetPeriodId?: string;
  jobName?: string;
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
  details?: unknown;
  created_at: string;
}

export interface IntegrationStatusResponse {
  connected: boolean;
  is_enabled: boolean;
  is_healthy?: boolean | null;
  health_error?: string | null;
  config?: IntegrationDto | null;
}

// ── Document & Storage Types ──────────────────────────────────────────────

export interface StorageUploadRequest {
  file: File | Blob;
  name?: string;
  category?: string;
  entity_type?: string;
  entity_id?: string;
  org_unit_id?: string;
  folder_id?: string;
  asset_type?: string;
  description?: string;
  academic_year_id?: string;
  term_id?: string;
  tags?: string;
  visibility?: string;
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

export type OrgLifecycleState = "empty" | "core_ready" | "ready_with_setup" | "operational";

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

export interface ReleaseChangelogItem {
  version: string;
  title: string;
  published_at?: string;
  body?: string;
  is_current: boolean;
  is_latest: boolean;
  html_url: string;
  images: string[];
  resources: ReleaseResource[];
}

export interface ReleaseResource {
  name: string;
  url: string;
  content_type: string;
  size: number;
}

export interface AppInfo {
  name: string;
  version: string;
  build_profile: string;
  target_os: string;
  target_arch?: string;
}

export interface UpdateCheckResult {
  current_version: string;
  latest_version: string;
  has_update: boolean;
  release_notes?: string;
  release_date?: string;
  release_url?: string;
  channel: string;
  status: "up_to_date" | "update_available" | "checking_failed";
  error_message?: string;
}

export interface EnvironmentDetectionResult {
  detected_locale: string;
  detected_language: string;
  country_code: string;
  country_name_en: string;
  country_name_ar: string;
  flag_emoji: string;
  timezone: string;
  timezone_offset: string;
  timezone_offset_hours: number;
  currency_code: string;
  currency_symbol_en: string;
  currency_symbol_ar: string;
  currency_name_en: string;
  currency_name_ar: string;
  is_hijri_recommended: boolean;
  date_format: string;
  suggested_academic_year: string;
  suggested_academic_year_ar: string;
}

export interface SlugGenerationResult {
  slug: string;
  is_available: boolean;
  suggestions: string[];
}

export interface BootstrapRequest {
  institution_type: string;
  organization_name: string;
  organization_name_ar?: string;
  organization_slug?: string;
  organization_code?: string;
  educational_track?: string;
  organization_size?: string;
  country_code?: string;
  city?: string;
  phone?: string;
  website?: string;
  tax_id?: string;
  license_number?: string;
  mulverse_node_id?: string;
  timezone: string;
  locale: string;
  date_format?: string;
  format_locale?: string;
  time_format?: "locale" | "12-hour" | "24-hour";
  is_hijri_enabled?: boolean;
  base_currency_code?: string;
  primary_color?: string;
  admin_first_name: string;
  admin_last_name: string;
  admin_first_name_ar?: string;
  admin_last_name_ar?: string;
  admin_email: string;
  admin_password: string;
  admin_phone?: string;
}

export interface BootstrapResult {
  success: boolean;
  organization_id: string;
  organization_name: string;
  organization_slug: string;
  institution_type: string;
  admin_user_id: string;
  admin_email: string;
  setup_required: boolean;
  warnings: string[];
}

export interface BootstrapStatusResponse {
  status: "required" | "in_progress" | "completed";
  organization_id?: string;
  organization_name?: string;
  institution_type?: string;
  has_admin: boolean;
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
      id?: string;
      parentId?: string;
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
      nameAr?: string;
      roomType: string;
      capacity: number;
      floor?: number;
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
      applicableLevels?: string[];
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

export interface EmployeeFilterInput {
  query?: string;
  employee_type?: string;
  department?: string;
  status?: string;
  org_unit_id?: string;
  page?: number;
  page_size?: number;
}

export interface BulkRegisterEmployeeCommandItem {
  row_index: number;
  registration: RegisterEmployeeRequest;
}

export interface BulkRegisterEmployeeItemCommandResultDto {
  row_index: number;
  success: boolean;
  employee_id?: string;
  employee_number?: string;
  user_id?: string;
  error?: string;
}

export interface BulkRegisterEmployeeCommandResultDto {
  total: number;
  succeeded: number;
  failed: number;
  items: BulkRegisterEmployeeItemCommandResultDto[];
}

// ═══════════════════════════════════════════════════════════════
// ONBOARDING & DAG READINESS
// ═══════════════════════════════════════════════════════════════

export type OnboardingTaskCategory =
  | "organization"
  | "calendar"
  | "academics"
  | "assessment"
  | "finance"
  | "students"
  | "templates"
  | "integrations";

export type OnboardingTaskStatus =
  | "not_started"
  | "in_progress"
  | "completed"
  | "blocked"
  | "optional"
  | "not_applicable";

export interface SetupTaskDto {
  id: string;
  category: OnboardingTaskCategory;
  title: string;
  title_ar: string;
  description: string;
  description_ar: string;
  is_required: boolean;
  status: OnboardingTaskStatus;
  dependencies: string[];
  progress: number;
  route: string;
  action_type: string;
  institution_types: string[];
  required_permissions: string[];
  completion_condition: string;
  blocking_reason?: string;
  blocking_reason_ar?: string;
  metadata?: Record<string, unknown>;
}

export interface CategoryProgressDto {
  category: OnboardingTaskCategory;
  label: string;
  label_ar: string;
  total_tasks: number;
  completed_tasks: number;
  progress_percentage: number;
}

export interface OnboardingStateDto {
  org_id: string;
  org_name: string;
  institution_type: string;
  total_tasks: number;
  completed_tasks: number;
  required_tasks: number;
  required_completed: number;
  optional_tasks: number;
  optional_completed: number;
  blocked_tasks: number;
  progress_percentage: number;
  required_progress_percentage: number;
  is_institution_ready: boolean;
  tasks: SetupTaskDto[];
  categories: CategoryProgressDto[];
  recommended_next_task?: SetupTaskDto;
  warnings: string[];
}

export interface ProvisionDefaultsReqDto {
  academic_year_id?: string;
  academic_year_name?: string;
  academic_year_name_ar?: string;
  start_date?: string;
  end_date?: string;
  provision_stages_and_grades?: boolean;
  provision_subjects?: boolean;
  provision_assessment_scheme?: boolean;
  provision_tuition_fee_template?: boolean;
  provision_discount_templates?: boolean;
  provision_hijri_school_calendar?: boolean;
  hijri_start_year?: number;
  hijri_month_lengths?: number[];
  provision_time_slots?: boolean;
  enable_period_autopilot?: boolean;
}

export interface ProvisionDefaultsResultDto {
  success: boolean;
  created_items: string[];
  state: OnboardingStateDto;
}

export type RoundingMode = 'round_half_up' | 'round_up' | 'round_down' | 'truncate';
export type RankingScope = 'class' | 'grade_level' | 'org_unit' | 'cohort';
export type TiePolicy = 'standard_competition' | 'dense' | 'fractional';
export type AcademicStatus =
  | 'passed'
  | 'passed_with_honors'
  | 'conditional_pass'
  | 'failed'
  | 'incomplete'
  | 'absent'
  | 'probation'
  | 'withdrawn';

export interface ComponentScore {
  code: string;
  name: string;
  name_ar?: string | null;
  score: number;
  max_score: number;
}

export interface PeriodComponentScores {
  components: ComponentScore[];
  total: number;
  total_max: number;
}

export interface PeriodAssessmentResult {
  period_id: string;
  period_code: string;
  period_name: string;
  period_name_ar?: string | null;
  breakdown: PeriodComponentScores;
}

export interface SubjectTermResult {
  subject_id: string;
  subject_code: string;
  subject_name: string;
  subject_name_ar?: string | null;
  credit_hours: number;
  monthly_results: PeriodAssessmentResult[];
  monthly_average: number;
  monthly_contribution: number;
  monthly_contribution_max: number;
  term_final_exam?: number | null;
  term_final_exam_max: number;
  term_total: number;
  term_max: number;
  percentage: number;
  letter_grade?: string | null;
  status: AcademicStatus;
}

export interface SubjectAnnualResult {
  subject_id: string;
  subject_code: string;
  subject_name: string;
  subject_name_ar?: string | null;
  credit_hours: number;
  term_1_result?: SubjectTermResult | null;
  term_2_result?: SubjectTermResult | null;
  annual_total: number;
  annual_max: number;
  percentage: number;
  letter_grade?: string | null;
  status: AcademicStatus;
}

export interface StudentTermReportCard {
  student_id: string;
  student_name: string;
  student_number?: string | null;
  class_id: string;
  class_name: string;
  period_id: string;
  period_name: string;
  subjects: SubjectTermResult[];
  total_score: number;
  max_possible_score: number;
  overall_percentage: number;
  gpa?: number | null;
  rank_in_class?: number | null;
  rank_in_grade?: number | null;
  total_students_in_class: number;
  total_students_in_grade: number;
  status: AcademicStatus;
  calculated_at: string;
}

export interface StudentAnnualReportCard {
  student_id: string;
  student_name: string;
  student_number?: string | null;
  class_id: string;
  class_name: string;
  academic_year_id: string;
  academic_year_name: string;
  subjects: SubjectAnnualResult[];
  total_score: number;
  max_possible_score: number;
  overall_percentage: number;
  cumulative_gpa?: number | null;
  rank_in_class?: number | null;
  rank_in_grade?: number | null;
  total_students_in_class: number;
  total_students_in_grade: number;
  status: AcademicStatus;
  calculated_at: string;
}

export interface AcademicPolicy {
  org_id: string;
  passing_threshold_percentage: number;
  honors_threshold_percentage: number;
  minimum_final_exam_score?: number | null;
  max_failed_subjects_for_conditional_pass: number;
  rounding_mode: RoundingMode;
  decimal_places: number;
  ranking_scope: RankingScope;
  tie_policy: TiePolicy;
  ranking_enabled: boolean;
  max_displayed_rank?: number | null;
}

// ═══════════════════════════════════════════════════════════════
// LICENSE & ACTIVATION TYPES (Mulverse Platform)
// ═══════════════════════════════════════════════════════════════

export type LicenseState =
  | "unavailable"
  | "trial"
  | "active"
  | "expiring"
  | "offline_grace"
  | "expired"
  | "revoked"
  | "invalid";

export interface LimitEntry {
  key: string;
  max_value: number;
}

export interface LicenseStateDto {
  state: LicenseState;
  product: string;
  edition: string | null;
  plan: string | null;
  organization_id: string | null;
  valid_until: string | null;
  days_until_expiry: number | null;
  trial_days_remaining: number;
  is_operational: boolean;
  is_restricted: boolean;
  installation_id: string;
  last_sync_at: string | null;
  enabled_features: string[];
  limits: LimitEntry[];
  app_version: string;
}

export interface ActivateKeyRequest {
  product_key: string;
}

export interface ActivationResultDto {
  success: boolean;
  license_state: LicenseStateDto;
  message: string;
}

// ═══════════════════════════════════════════════════════════════
// UNIFIED CLIENT INTERFACE
// ═══════════════════════════════════════════════════════════════

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
  provisionUser(req: ProvisionUserRequest): Promise<AccountProvisioningResultDto>;
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
  updateOrgUnit(id: string, req: UpdateOrgUnitRequest): Promise<OrgUnitDto>;
  listOrgUnits(): Promise<OrgUnitDto[]>;
  getOrgUnit(id: string): Promise<OrgUnitDto>;
  deleteOrgUnit(id: string): Promise<void>;
  selectOrgUnit(id?: string): Promise<void>;

  // ── Calendar ────────────────────────────────────────────────
  createAcademicYear(req: CreateYearRequest): Promise<AcademicYearDto>;
  updateAcademicYear(id: string, req: UpdateYearRequest): Promise<AcademicYearDto>;
  listAcademicYears(): Promise<AcademicYearDto[]>;
  getAcademicContext(): Promise<AcademicContextResponse>;
  getAcademicYear(id: string): Promise<AcademicYearDto>;
  activateAcademicYear(id: string): Promise<AcademicYearDto>;
  closeAcademicYear(id: string): Promise<AcademicYearDto>;
  archiveAcademicYear(id: string): Promise<AcademicYearDto>;
  createAcademicPeriod(req: CreatePeriodRequest): Promise<AcademicPeriodDto>;
  updateAcademicPeriod(id: string, req: UpdatePeriodRequest): Promise<AcademicPeriodDto>;
  deleteAcademicPeriod(id: string): Promise<boolean>;
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
  checkDuplicate(req: DuplicateCheckRequest): Promise<DuplicateCheckResponse>;
  listStudents(filter?: StudentFilterParams): Promise<PaginatedList<StudentDetailDto>>;
  getStudent(id: string): Promise<StudentDetailDto>;
  updateStudentStatus(req: UpdateStudentStatusInput): Promise<StudentDto>;
  updateStudentProfile(req: UpdateStudentProfileInput): Promise<StudentDetailDto>;
  deleteStudent(id: string): Promise<boolean>;
  restoreStudent(id: string): Promise<StudentDto>;
  changeStudyMode(req: ChangeStudyModeInput): Promise<EnrollmentDto>;
  transferClass(req: TransferClassInput): Promise<EnrollmentDto>;
  enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto>;
  listEnrollments(filter?: EnrollmentFilterParams): Promise<EnrollmentDto[]>;
  getEnrollment(id: string): Promise<EnrollmentDto>;
  assignGuardian(req: AssignGuardianRequest): Promise<GuardianDto>;
  listGuardians(studentId: string): Promise<GuardianDto[]>;
  removeGuardian(studentId: string, guardianId: string): Promise<boolean>;
  bulkRegisterStudents(items: BulkRegisterStudentCommandItem[], options?: { atomic?: boolean; dryRun?: boolean }): Promise<BulkRegisterStudentCommandResultDto>;

  // ── Employee Affairs ────────────────────────────────────────
  registerEmployee(req: RegisterEmployeeRequest): Promise<RegisterEmployeeResponseDto>;
  listEmployees(filter?: EmployeeFilterInput): Promise<PaginatedEmployeesDto>;
  getEmployee(id: string): Promise<EmployeeDetailDto>;
  getEmployeeStats(): Promise<Record<string, number>>;
  updateEmployee(id: string, req: UpdateEmployeeRequest): Promise<EmployeeDetailDto>;
  updateEmployeeStatus(id: string, status: string): Promise<EmployeeDetailDto>;
  deleteEmployee(id: string): Promise<boolean>;
  restoreEmployee(id: string): Promise<EmployeeDetailDto>;
  provisionEmployeeAccount(id: string, req: ProvisionEmployeeAccountRequest): Promise<AccountProvisioningResultDto>;
  assignEmployeeGroups(id: string, groupIds: string[]): Promise<boolean>;
  bulkRegisterEmployees(items: BulkRegisterEmployeeCommandItem[], options?: { atomic?: boolean; dryRun?: boolean }): Promise<BulkRegisterEmployeeCommandResultDto>;

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
  updateRoom(id: string, req: UpdateRoomRequest): Promise<RoomDto>;
  deleteRoom(id: string): Promise<void>;
  createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto>;
  batchCreateTimeSlots(req: BatchCreateTimeSlotsRequest): Promise<TimeSlotDto[]>;
  listTimeSlots(periodId?: string): Promise<TimeSlotDto[]>;
  getTimeSlot(id: string): Promise<TimeSlotDto>;
  updateTimeSlot(id: string, req: UpdateTimeSlotRequest): Promise<TimeSlotDto>;
  deleteTimeSlot(id: string): Promise<void>;
  scheduleLesson(req: ScheduleLessonRequest): Promise<ScheduledLessonDto>;
  bulkScheduleLessons(req: BulkScheduleLessonsRequest): Promise<ScheduledLessonDto[]>;
  deleteScheduledLesson(id: string): Promise<void>;
  listScheduledLessons(
    filterOrPeriodId?:
      | string
      | {
          classSubjectId?: string;
          periodId?: string;
          classId?: string;
          teacherId?: string;
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
  batchRecordGrades(
    req: BatchRecordGradesRequest,
  ): Promise<BatchRecordGradesResponseDto>;
  deleteGrade(id: string): Promise<void>;
  listGrades(filter?: {
    studentId?: string;
    classSubjectId?: string;
  }): Promise<GradeDto[]>;
  calculateGrades(req: CalculateGradesRequest): Promise<CalculationResultDto>;
  calculateGpa(req: CalculateGpaRequest): Promise<CumulativeGpaRecordDto>;
  calculateRankings(req: CalculateRankingsRequest): Promise<RankingItemDto[]>;
  getStudentTermResults(
    studentId: string,
    periodId: string,
    classId?: string,
  ): Promise<StudentTermReportCard>;
  getStudentAnnualResults(
    studentId: string,
    yearId: string,
    classId?: string,
  ): Promise<StudentAnnualReportCard>;
  getClassTermResults(
    classId: string,
    periodId: string,
  ): Promise<StudentTermReportCard[]>;
  getClassAnnualResults(
    classId: string,
    yearId: string,
  ): Promise<StudentAnnualReportCard[]>;
  markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto>;
  batchMarkAttendance(
    req: BatchMarkAttendanceRequest,
  ): Promise<BatchMarkAttendanceResponseDto>;
  deleteAttendance(id: string): Promise<void>;
  listAttendance(classId: string, date: string): Promise<AttendanceDto[]>;
  listStudentAttendance(studentId: string): Promise<AttendanceDto[]>;
  recordBehavior(req: RecordBehaviorRequest): Promise<BehaviorRecordDto>;
  deleteBehavior(id: string): Promise<void>;
  listStudentBehaviorRecords(studentId: string): Promise<BehaviorRecordDto[]>;
  listAllBehavior(): Promise<BehaviorRecordDto[]>;

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
  updateExamSession(id: string, req: UpdateExamSessionRequest): Promise<ExamSessionDto>;
  deleteExamSession(id: string): Promise<void>;
  scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto>;
  listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]>;
  deleteExamSchedule(id: string): Promise<void>;
  assignSeating(req: AssignSeatingRequest): Promise<SeatingDto>;
  listSeating(sessionId: string): Promise<SeatingDto[]>;
  generateSeating(req: GenerateSeatingRequest): Promise<SeatingDto[]>;
  deleteSeating(id: string): Promise<void>;
  assignSecret(req: AssignSecretRequest): Promise<SecretDto>;
  listSecrets(scheduleId: string): Promise<SecretDto[]>;
  generateSecrets(req: GenerateSecretsRequest): Promise<SecretDto[]>;
  deleteSecret(id: string): Promise<void>;
  assignProctor(req: AssignProctorRequest): Promise<ProctorDto>;
  listProctors(scheduleId: string): Promise<ProctorDto[]>;
  deleteProctor(id: string): Promise<void>;

  // ── Finance ─────────────────────────────────────────────────
  createAccount(req: CreateAccountRequest): Promise<AccountDto>;
  listAccounts(): Promise<AccountDto[]>;
  getAccount(id: string): Promise<AccountDto>;
  updateAccount(id: string, req: UpdateAccountRequest): Promise<AccountDto>;
  deleteAccount(id: string): Promise<boolean>;
  createFeeTemplate(req: CreateFeeTemplateRequest): Promise<FeeTemplateDto>;
  getFeeTemplate(id: string): Promise<FeeTemplateDto>;
  updateFeeTemplate(id: string, req: UpdateFeeTemplateRequest): Promise<FeeTemplateDto>;
  deleteFeeTemplate(id: string): Promise<boolean>;
  listFeeTemplates(): Promise<FeeTemplateDto[]>;
  generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto>;
  listStudentFees(filter?: StudentFeeFilterParams): Promise<StudentFeeDto[]>;
  deleteStudentFee(id: string): Promise<boolean>;
  processPayment(req: ProcessPaymentRequest): Promise<PaymentDto>;
  listPayments(filter?: PaymentFilterParams): Promise<PaymentDto[]>;
  deletePayment(id: string): Promise<boolean>;
  listCurrencies(): Promise<CurrencyDto[]>;
  listJournalEntries(): Promise<JournalEntryDto[]>;
  getJournalEntry(id: string): Promise<JournalEntryDto>;
  createJournalEntry(req: CreateJournalEntryRequest): Promise<JournalEntryDto>;
  reverseJournalEntry(
    id: string,
    req: ReverseJournalRequest,
  ): Promise<JournalEntryDto>;
  deleteJournalEntry(id: string): Promise<boolean>;
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
  deleteDiscountTemplate(id: string): Promise<boolean>;
  getAccountStatement(filter: StatementFilterParams): Promise<StatementResponseDto>;
  processDisbursement(req: ProcessDisbursementRequest): Promise<JournalEntryDto>;
  createSubAccount(req: CreateSubAccountRequest): Promise<AccountDto>;
  settleGatewayPayout(req: SettleGatewayPayoutRequest): Promise<JournalEntryDto>;
  postPayrollAccrual(req: PostPayrollRequest): Promise<JournalEntryDto>;
  disbursePayroll(req: DisbursePayrollRequest): Promise<JournalEntryDto>;

  // ── Document & Storage Management ───────────────────────────
  uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto>;
  uploadFile(req: StorageUploadRequest): Promise<DocumentDto>;
  listDocuments(params?: { entity_type?: string; entity_id?: string; folder_id?: string }): Promise<DocumentDto[]>;
  listDocumentsByEntity(entityType: string, entityId: string): Promise<DocumentDto[]>;
  getDocument(id: string): Promise<DocumentDto>;
  updateDocument(id: string, req: UpdateDocumentRequest): Promise<DocumentDto>;
  deleteDocument(id: string): Promise<void>;
  archiveDocument(id: string, reason?: string): Promise<DocumentDto>;
  restoreDocument(id: string): Promise<DocumentDto>;
  trashDocument(id: string): Promise<void>;
  purgeDocument(id: string): Promise<void>;
  listArchivedDocuments(): Promise<DocumentDto[]>;
  listTrashDocuments(): Promise<DocumentDto[]>;
  getDocumentAccessUrl(id: string, expiresInSeconds?: number): Promise<string>;
  downloadDocumentFile(id: string): Promise<Blob | Uint8Array>;
  createDocumentRequirement(req: CreateDocumentRequirementRequest): Promise<DocumentRequirementDto>;
  getDocumentRequirement(id: string): Promise<DocumentRequirementDto>;
  updateDocumentRequirement(id: string, req: UpdateDocumentRequirementRequest): Promise<DocumentRequirementDto>;
  listDocumentRequirements(): Promise<DocumentRequirementDto[]>;
  deleteDocumentRequirement(id: string): Promise<void>;
  evaluateStudentDocumentRequirements(
    studentId: string,
    params?: {
      grade_level_id?: string;
      is_national?: boolean;
      is_special_needs?: boolean;
      is_transfer?: boolean;
    },
  ): Promise<StudentDocumentFulfillmentDto[]>;

  // ── Folders & Curriculum Knowledge Base ─────────────────────
  createFolder(req: CreateFolderRequest): Promise<FolderDto>;
  listFolders(parentId?: string): Promise<FolderDto[]>;
  getFolder(id: string): Promise<FolderDto>;
  getFolderContents(id: string): Promise<FolderContentsDto>;
  getOrCreateStudentFolder(studentId: string, studentName?: string): Promise<FolderDto>;
  listCurriculumFolders(): Promise<FolderDto[]>;
  getOrCreateCurriculumFolder(req: CreateCurriculumFolderRequest): Promise<FolderDto>;
  updateFolder(id: string, req: UpdateFolderRequest): Promise<FolderDto>;
  deleteFolder(id: string): Promise<void>;

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
  validateTemplate(documentJson: string): Promise<ValidationResultDto>;

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

  // ── System ──────────────────────────────────────────────────
  systemHealth(): Promise<HealthResponse>;
  systemHasOrganization(): Promise<boolean>;
  systemGetReadiness(): Promise<OrganizationReadiness>;
  systemGetEntitlement(): Promise<EntitlementResult>;
  systemGetAppInfo(): Promise<AppInfo>;
  systemGetChangelog(): Promise<ReleaseChangelogItem[]>;
  systemCheckForUpdates(): Promise<UpdateCheckResult>;
  systemInstallUpdate(): Promise<void>;
  openChangelogWindow(): Promise<void>;
  openAgentWindow(): Promise<void>;

  // ── License & Activation ────────────────────────────────
  licenseGetState(): Promise<LicenseStateDto>;
  licenseActivateKey(req: ActivateKeyRequest): Promise<ActivationResultDto>;
  licenseStartTrial(): Promise<LicenseStateDto>;
  licenseDeactivate(): Promise<LicenseStateDto>;
  licenseRefresh(): Promise<LicenseStateDto>;
  licenseCheckFeature(featureKey: string): Promise<boolean>;
  licenseGetInstallationId(): Promise<string>;

  // ── Bootstrap & Regional Initialization ─────────────────────
  bootstrapDetectEnvironment(): Promise<EnvironmentDetectionResult>;
  bootstrapGenerateSlug(name: string, nameAr?: string): Promise<SlugGenerationResult>;
  bootstrapGetStatus(): Promise<BootstrapStatusResponse>;
  bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult>;
  completeOnboarding(req: CompleteOrganizationOnboardingRequest): Promise<OnboardingResult>;

  // ── Onboarding & DAG Readiness ──────────────────────────────
  onboardingGetState(orgId?: string): Promise<OnboardingStateDto>;
  onboardingProvisionDefaults(
    req: ProvisionDefaultsReqDto,
    orgId?: string,
  ): Promise<ProvisionDefaultsResultDto>;

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
