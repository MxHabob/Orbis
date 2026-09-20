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
  ExamSessionDto,
  ExamScheduleDto,
  SeatingDto,
  SecretDto,
  ProctorDto,
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
  periodId?: string;
  classId?: string;
  status?: string;
  standing?: string;
  search?: string;
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortDir?: string;
}

export interface EnrollmentFilterParams {
  periodId?: string;
  classId?: string;
  studentId?: string;
  studyMode?: string;
  status?: string;
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortDir?: string;
}

export interface RegisterStudentInput {
  personId?: string;
  userId?: string;
  studentNumber: string;
  firstName: string;
  middleName?: string;
  lastName: string;
  firstNameAr?: string;
  middleNameAr?: string;
  lastNameAr?: string;
  displayName?: string;
  nationalId?: string;
  passportNumber?: string;
  gender?: string;
  birthDate?: string;
  placeOfBirth?: string;
  nationality?: string;
  bloodType?: string;
  medicalNotes?: string;
  specialNeedsNotes?: string;
  phone?: string;
  email?: string;
  initialPeriodId?: string;
  initialClassId?: string;
  studyMode?: string;
}

export interface UpdateStudentStatusInput {
  studentId: string;
  status: string;
  reason?: string;
}

export interface ChangeStudyModeInput {
  enrollmentId: string;
  studyMode: string;
  reason?: string;
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
  overrideType: string;
  riskLevel?: string;
  entityType: string;
  entityId: string;
  fieldName?: string;
  oldValue?: string;
  newValue?: string;
  reason: string;
  approvalChain?: ApprovalStepDto[];
  metadata?: Record<string, unknown>;
}

export interface ApproveOverrideInput {
  requestId: string;
  notes?: string;
}

export interface RejectOverrideInput {
  requestId: string;
  reason: string;
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
  studentId: string;
  enrollmentId: string;
  sourceClassId?: string;
  targetClassId?: string;
  decision: string;
  reason?: string;
}

export interface PromoteCohortInput {
  sourcePeriodId: string;
  targetPeriodId: string;
  jobName: string;
  items: StudentProgressionItemInput[];
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
  studentId: string;
  periodId: string;
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

export interface IntegrationStatusResponse {
  connected: boolean;
  is_enabled: boolean;
  config?: IntegrationDto | null;
}

// ── Connection Types (Desktop-only, pre-database) ─────────────

export type ConnectionMode = "local" | "remote";

export interface ConnectionConfig {
  mode: ConnectionMode;
  local?: { db_filename: string };
  remote?: { server_url: string; api_key?: string };
}

export interface ConnectionValidation {
  is_valid: boolean;
  mode: string;
  message: string;
  resolved_url?: string;
}

export interface RemoteHealthResult {
  reachable: boolean;
  status?: string;
  version?: string;
  api_version?: string;
  latency_ms: number;
  error?: string;
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
  listRoles(): Promise<RoleDto[]>;
  saveRole(req: SaveRoleRequest): Promise<RoleDto>;
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
  listStudents(filter?: StudentFilterParams): Promise<PaginatedList<StudentDetailDto>>;
  getStudent(id: string): Promise<StudentDetailDto>;
  updateStudentStatus(req: UpdateStudentStatusInput): Promise<StudentDto>;
  changeStudyMode(req: ChangeStudyModeInput): Promise<EnrollmentDto>;
  enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto>;
  listEnrollments(filter?: EnrollmentFilterParams): Promise<EnrollmentDto[]>;
  getEnrollment(id: string): Promise<EnrollmentDto>;
  assignGuardian(req: AssignGuardianRequest): Promise<GuardianDto>;
  listGuardians(studentId: string): Promise<GuardianDto[]>;

  // ── Scheduling ──────────────────────────────────────────────
  createRoom(req: CreateRoomRequest): Promise<RoomDto>;
  listRooms(): Promise<RoomDto[]>;
  getRoom(id: string): Promise<RoomDto>;
  createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto>;
  listTimeSlots(periodId: string): Promise<TimeSlotDto[]>;
  scheduleLesson(req: ScheduleLessonRequest): Promise<ScheduledLessonDto>;
  listScheduledLessons(filter?: {
    classSubjectId?: string;
    periodId?: string;
  }): Promise<ScheduledLessonDto[]>;

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
  markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto>;
  listAttendance(classId: string, date: string): Promise<AttendanceDto[]>;

  // ── Student Cohort Progression ──────────────────────────────
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
  assignSecret(req: AssignSecretRequest): Promise<SecretDto>;
  assignProctor(req: AssignProctorRequest): Promise<ProctorDto>;

  // ── Finance ─────────────────────────────────────────────────
  createAccount(req: CreateAccountRequest): Promise<AccountDto>;
  listAccounts(): Promise<AccountDto[]>;
  createFeeTemplate(req: CreateFeeTemplateRequest): Promise<FeeTemplateDto>;
  listFeeTemplates(): Promise<FeeTemplateDto[]>;
  generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto>;
  listStudentFees(filter?: StudentFeeFilterParams): Promise<StudentFeeDto[]>;
  processPayment(req: ProcessPaymentRequest): Promise<PaymentDto>;
  listPayments(filter?: PaymentFilterParams): Promise<PaymentDto[]>;
  listCurrencies(): Promise<CurrencyDto[]>;
  listJournalEntries(): Promise<JournalEntryDto[]>;
  getJournalEntry(id: string): Promise<JournalEntryDto>;
  listDiscountTemplates(): Promise<DiscountTemplateDto[]>;
  createDiscountTemplate(
    req: CreateDiscountTemplateRequest,
  ): Promise<DiscountTemplateDto>;

  // ── Document ────────────────────────────────────────────────
  uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto>;
  listDocuments(): Promise<DocumentDto[]>;
  getDocument(id: string): Promise<DocumentDto>;
  deleteDocument(id: string): Promise<void>;

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

  // ── Report Execution & Printing ─────────────────────────────
  generateReport(
    templateId: string,
    format: string,
    parameters: Record<string, unknown>,
  ): Promise<string>;
  getReportJobStatus(jobId: string): Promise<ReportJobStatusDto>;
  downloadReportFile(jobId: string): Promise<Uint8Array>;
  batchPrint(req: BatchPrintParams): Promise<string>;

  // ── System & Bootstrap ──────────────────────────────────────
  systemHealth(): Promise<HealthResponse>;
  systemHasOrganization(): Promise<boolean>;
  systemGetReadiness(): Promise<OrganizationReadiness>;
  systemGetEntitlement(): Promise<EntitlementResult>;
  bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult>;
  bootstrapInitialSetup(req: InitialSetupRequest): Promise<OrganizationReadiness>;

  // ── Connection (Desktop-only) ───────────────────────────────
  connectionGetConfig(): Promise<ConnectionConfig>;
  connectionSetConfig(mode: ConnectionMode, serverUrl?: string, dbFilename?: string): Promise<ConnectionConfig>;
  connectionValidate(): Promise<ConnectionValidation>;
  connectionCheckRemoteHealth(serverUrl: string): Promise<RemoteHealthResult>;
  connectionConnect(): Promise<ConnectResult>;
  connectionOpenWindow(): Promise<void>;

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

