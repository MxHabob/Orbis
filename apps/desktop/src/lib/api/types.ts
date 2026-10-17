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
  AcademicCalendarDto,
  AcademicPeriodDto,
  AcademicContextResponse,
  PeriodTypeDto,
  CreateYearRequest,
  UpdateYearRequest,
  CreatePeriodRequest,
  UpdatePeriodRequest,
  SubjectDto,
  SectionDto,
  SectionSubjectDto,
  CreateSectionRequest,
  UpdateSectionRequest,
  CreateSubjectRequest,
  UpdateSubjectRequest,
  AssignSectionSubjectRequest,
  UpdateSectionSubjectRequest,
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
  ProgressionJobDto,
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

export type ClassDto = SectionDto;
export type CreateClassRequest = CreateSectionRequest;
export type UpdateClassRequest = UpdateSectionRequest;
export type academicCalendarDto = AcademicCalendarDto;
export type progressionJobDto = ProgressionJobDto;

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
  address?: string;
  initial_period_id?: string;
  initial_section_id?: string;
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
  output_size_bytes?: number | null;
  error_message?: string | null;
}

export interface BatchPrintParams {
  templateId?: string;
  documentJson?: string;
  contexts?: Record<string, unknown>[];
  title?: string;
  reportCode?: string;
  parameters?: Record<string, unknown>;
  batchConfig: Record<string, unknown>;
  locale?: string;
}

export interface StudentFeeFilterParams {
  period_id?: string;
  student_id?: string;
  enrollment_id?: string;
  feeTemplate_id?: string;
  status?: string;
  is_overdue?: boolean;
  search?: string;
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
  search?: string;
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

export interface ProgressionDetailDto {
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

export interface MulverseConnectionDto {
  connected: boolean;
  organization_id: string | null;
  product_organization_id: string | null;
}

export interface MulverseDeploymentDto {
  id: string;
  name: string;
  environment: string;
  status: string;
  verification_status: string;
  created_at: string;
  updated_at: string;
}

export interface MulverseEndpointDto {
  id: string;
  deployment_id: string;
  url: string;
  network_scope: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface MulverseDeploymentRequest {
  name: string;
  environment: string;
}

export interface MulverseEndpointRequest {
  url: string;
  network_scope: "public" | "local_network";
}

export interface MulverseEndpointUpdateRequest {
  url?: string;
  status?: "active" | "pending" | "disabled";
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
  calendar_id?: string;
  term_id?: string;
  tags?: string;
  visibility?: string;
}

// ── Connection & Database Management Types (Desktop / Hybrid) ─────────────

export type ConnectionMode = "local" | "direct" | "api";

export interface LocalConfig {
  db_filename: string;
  custom_path?: string;
  wal_mode?: boolean;
  busy_timeout_ms?: number;
}

export interface ApiConfig {
  server_url: string;
}

export interface DirectConfig {
  engine: "postgres";
  host: string;
  port: number;
  database_name: string;
  username: string;
  ssl_mode?: string;
  credential_id: string;
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
  api?: ApiConfig;
  direct?: DirectConfig;
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
  mode: "local" | "direct" | "api" | "remote";
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
  has_academic_calendar: boolean;
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
  suggested_academic_calendar: string;
  suggested_academic_calendar_ar: string;
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
    academicCalendarName: string;
    academicCalendarNameAr?: string;
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
  calendar_id?: string;
  academic_calendar_name?: string;
  academic_calendar_name_ar?: string;
  start_date?: string;
  end_date?: string;
  provision_academic_calendar?: boolean;
  provision_default_academic_periods?: boolean;
  provision_stages_and_grades?: boolean;
  provision_subjects?: boolean;
  provision_assessment_scheme?: boolean;
  provision_tuition_fee_template?: boolean;
  provision_discount_templates?: boolean;
  provision_hijri_school_calendar?: boolean;
  hijri_start_year?: number;
  hijri_start_month?: number;
  hijri_start_day?: number;
  hijri_month_lengths?: number[];
  provision_time_slots?: boolean;
  enable_period_autopilot?: boolean;
}

export interface ProvisionDefaultsResultDto {
  success: boolean;
  created_items: string[];
  state: OnboardingStateDto;
}

export interface OfficialCalendarTemplateDto {
  code: string;
  name: string;
  name_ar: string;
  country: string;
  country_name: string;
  country_name_ar: string;
  hijri_year: number;
  gregorian_years: string;
  decision_number?: string;
  decision_date?: string;
  authority: string;
  authority_ar: string;
}

export interface ApplyCalendarTemplateRequest {
  template_code: string;
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
  calendar_id: string;
  academic_calendar_name: string;
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

export interface StudentCertificateReportCard {
  student_id: string;
  student_name: string;
  student_number?: string | null;
  section_id: string;
  class_name: string;
  calendar_id: string;
  academic_calendar_name: string;
  /** "basic_grade9" or "secondary_thanawiya" */
  certificate_type: string;
  subjects: SubjectTermResult[];
  total_score: number;
  max_possible_score: number;
  overall_percentage: number;
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

export type DecimalValue = number | string;

export type CalculationUnit =
  | { type: "dimensionless" }
  | { type: "score"; scale: string }
  | { type: "credit" }
  | { type: "count" }
  | { type: "custom"; scale: string };

export interface CalculationWeightedItem {
  node: CalculationExpression;
  weight: DecimalValue;
}

export type CalculationExpression =
  | { type: "literal"; value: DecimalValue }
  | { type: "measure_literal"; value: DecimalValue; unit: CalculationUnit }
  | { type: "variable"; name: string }
  | {
      type: "add" | "multiply";
      operands: CalculationExpression[];
      left?: CalculationExpression | null;
      right?: CalculationExpression | null;
    }
  | { type: "subtract" | "divide"; left: CalculationExpression; right: CalculationExpression }
  | { type: "max" | "min" | "average"; operands: CalculationExpression[] }
  | {
      type: "weighted_sum";
      items: CalculationWeightedItem[];
      normalize: boolean;
    }
  | {
      type: "convert";
      value: CalculationExpression;
      from: CalculationUnit;
      to: CalculationUnit;
      factor: DecimalValue;
      offset: DecimalValue;
    };

export type CalculationRoundingMode =
  | "round_half_up"
  | "round_up"
  | "round_down"
  | "truncate";

export interface CalculationRoundingRule {
  mode: CalculationRoundingMode;
  decimal_places: number;
}

export interface CalculationInput {
  name: string;
  unit: CalculationUnit;
  minimum?: DecimalValue | null;
  maximum?: DecimalValue | null;
  is_required: boolean;
}

export interface CalculationOutput {
  name: string;
  unit: CalculationUnit;
  expression: CalculationExpression;
  rounding?: CalculationRoundingRule | null;
}

export interface CalculationGradeBand {
  code: string;
  label: string;
  minimum_score: DecimalValue;
}

export interface CalculationOutcomePolicy {
  unit: CalculationUnit;
  pass_at_or_above?: DecimalValue | null;
  honors_at_or_above?: DecimalValue | null;
  grade_bands: CalculationGradeBand[];
}

export interface CalculationPlan {
  version: number;
  inputs: CalculationInput[];
  outputs: CalculationOutput[];
}

export interface CalculationEngineSettings {
  calculation_plan: CalculationPlan | null;
  outcome_policy: CalculationOutcomePolicy | null;
  ranking_enabled: boolean;
  ranking_scope: RankingScope;
  tie_policy: TiePolicy;
  max_displayed_rank: number | null;
}

export interface CalculationResult {
  policy_version: number;
  outputs: Array<{
    name: string;
    unit: CalculationUnit;
    value: DecimalValue;
  }>;
}

export interface CalculationOutcome {
  is_passed?: boolean | null;
  is_honors?: boolean | null;
  grade_code?: string | null;
}

export interface CalculationPreview {
  result: CalculationResult;
  outcomes: Array<{
    output_name: string;
    outcome: CalculationOutcome;
  }>;
}

// ═══════════════════════════════════════════════════════════════
// LICENSE & ACTIVATION TYPES (Mulverse Platform)
// ═══════════════════════════════════════════════════════════════

export type LicenseState =
  | "unavailable"
  | "pending"
  | "trial"
  | "active"
  | "expiring"
  | "offline_grace"
  | "expired"
  | "suspended"
  | "deactivation_pending"
  | "revoked"
  | "invalid";

export type EntitlementKey =
  | "students"
  | "ai_grading"
  | "multi_branch"
  | "offline_sync"
  | "parent_portal"
  | "campus_analytics"
  | "sso"
  | "exam_tools";

export type LimitKey =
  | "students.max"
  | "staff.max"
  | "devices.max"
  | "branches.max"
  | "classes.max"
  | "storage_gb.max";

export interface LimitEntry {
  key: string;
  /** `null` means the quota is unlimited. */
  max_value: number | null;
  unit?: string;
}

export interface LicenseStateDto {
  state: LicenseState;
  product: string;
  edition: string | null;
  plan: string | null;
  plan_name: string | null;
  organization_id: string | null;
  valid_until: string | null;
  days_until_expiry: number | null;
  trial_days_remaining: number;
  is_operational: boolean;
  is_restricted: boolean;
  installation_id: string;
  last_sync_at: string | null;
  enabled_features: string[];
  trial_features: string[];
  limits: LimitEntry[];
  app_version: string;
}

export interface ActivateKeyRequest {
  product_key: string;
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
  createacademicCalendar(req: CreateYearRequest): Promise<AcademicCalendarDto>;
  updateacademicCalendar(id: string, req: UpdateYearRequest): Promise<AcademicCalendarDto>;
  listacademicCalendars(): Promise<AcademicCalendarDto[]>;
  getAcademicContext(): Promise<AcademicContextResponse>;
  getacademicCalendar(id: string): Promise<AcademicCalendarDto>;
  activateacademicCalendar(id: string): Promise<AcademicCalendarDto>;
  closeacademicCalendar(id: string): Promise<AcademicCalendarDto>;
  archiveacademicCalendar(id: string): Promise<AcademicCalendarDto>;
  deleteacademicCalendar(id: string): Promise<boolean>;
  createAcademicPeriod(req: CreatePeriodRequest): Promise<AcademicPeriodDto>;
  updateAcademicPeriod(id: string, req: UpdatePeriodRequest): Promise<AcademicPeriodDto>;
  deleteAcademicPeriod(id: string): Promise<boolean>;
  listAcademicPeriods(): Promise<AcademicPeriodDto[]>;
  getAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  lockAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  unlockAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  closeAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  listPeriodTypes(): Promise<PeriodTypeDto[]>;
  listOfficialCalendarTemplates(): Promise<OfficialCalendarTemplateDto[]>;
  applyCalendarTemplate(yearId: string, req: ApplyCalendarTemplateRequest): Promise<AcademicPeriodDto[]>;

  // ── Academic ────────────────────────────────────────────────
  createSubject(req: CreateSubjectRequest): Promise<SubjectDto>;
  updateSubject(id: string, req: UpdateSubjectRequest): Promise<SubjectDto>;
  listSubjects(): Promise<SubjectDto[]>;
  getSubject(id: string): Promise<SubjectDto>;
  createSection(req: CreateSectionRequest): Promise<SectionDto>;
  updateSection(id: string, req: UpdateSectionRequest): Promise<SectionDto>;
  listSections(): Promise<SectionDto[]>;
  getSection(id: string): Promise<SectionDto>;
  assignSectionSubject(req: AssignSectionSubjectRequest): Promise<SectionSubjectDto>;
  getSectionSubject(id: string): Promise<SectionSubjectDto>;
  updateSectionSubject(
    id: string,
    req: UpdateSectionSubjectRequest,
  ): Promise<SectionSubjectDto>;
  listSectionSubjects(
    classId: string,
    periodId: string,
  ): Promise<SectionSubjectDto[]>;
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
          SectionSubjectId?: string;
          periodId?: string;
          classId?: string;
          teacherId?: string;
        },
    SectionSubjectId?: string,
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
    SectionSubjectId?: string;
  }): Promise<GradeDto[]>;
  calculateGrades(req: CalculateGradesRequest): Promise<CalculationResultDto>;
  calculateGpa(req: CalculateGpaRequest): Promise<CumulativeGpaRecordDto>;
  calculateRankings(req: CalculateRankingsRequest): Promise<RankingItemDto[]>;
  getCalculationEngineSettings(): Promise<CalculationEngineSettings>;
  updateCalculationEngineSettings(
    settings: CalculationEngineSettings,
  ): Promise<CalculationEngineSettings>;
  previewCalculation(values: Record<string, DecimalValue>): Promise<CalculationPreview>;
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
  getStudentCertificateResults(
    studentId: string,
    calendarId: string,
    certificateType: string,
    classId?: string,
  ): Promise<StudentCertificateReportCard>;
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
  promoteCohort(req: PromoteCohortInput): Promise<ProgressionJobDto>;
  listprogressionJobs(): Promise<ProgressionJobDto[]>;
  getprogressionJob(id: string): Promise<ProgressionJobDto>;

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
  listStudentFeesPaginated(
    filter?: StudentFeeFilterParams,
  ): Promise<PaginatedList<StudentFeeDto>>;
  deleteStudentFee(id: string): Promise<boolean>;
  processPayment(req: ProcessPaymentRequest): Promise<PaymentDto>;
  listPayments(filter?: PaymentFilterParams): Promise<PaymentDto[]>;
  listPaymentsPaginated(
    filter?: PaymentFilterParams,
  ): Promise<PaginatedList<PaymentDto>>;
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
  getMulverseConnection(): Promise<MulverseConnectionDto>;
  connectMulverse(apiKey: string): Promise<MulverseConnectionDto>;
  disconnectMulverse(): Promise<boolean>;
  listMulverseDeployments(): Promise<MulverseDeploymentDto[]>;
  createMulverseDeployment(req: MulverseDeploymentRequest): Promise<MulverseDeploymentDto>;
  updateMulverseDeployment(
    id: string,
    req: Partial<MulverseDeploymentRequest>,
  ): Promise<MulverseDeploymentDto>;
  listMulverseEndpoints(deploymentId: string): Promise<MulverseEndpointDto[]>;
  createMulverseEndpoint(
    deploymentId: string,
    req: MulverseEndpointRequest,
  ): Promise<MulverseEndpointDto>;
  updateMulverseEndpoint(
    id: string,
    req: MulverseEndpointUpdateRequest,
  ): Promise<MulverseEndpointDto>;

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
  batchPrint(req: BatchPrintParams): Promise<Uint8Array>;
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
  openStandaloneWindow(path: string): Promise<void>;

  // ── License & Activation ────────────────────────────────
  licenseGetState(): Promise<LicenseStateDto>;
  licenseActivateKey(req: ActivateKeyRequest): Promise<LicenseStateDto>;
  licenseRedeemKey(req: ActivateKeyRequest): Promise<LicenseStateDto>;
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
    databaseEngine?: "postgres";
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
