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

export interface PaginatedResponse<T> {
  items: T[];
  count: number;
  nextCursor?: string | null;
  hasMore: boolean;
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
  updateUser(id: string, req: { full_name?: string }): Promise<UserDto>;

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

  // ── Student ─────────────────────────────────────────────────
  enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto>;
  listEnrollments(filter?: {
    classId?: string;
    studentId?: string;
  }): Promise<EnrollmentDto[]>;
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
  listAssessmentSchemes(): Promise<AssessmentSchemeDto[]>;
  getAssessmentScheme(id: string): Promise<AssessmentSchemeDto>;
  recordGrade(req: RecordGradeRequest): Promise<GradeDto>;
  listGrades(filter?: {
    studentId?: string;
    classSubjectId?: string;
  }): Promise<GradeDto[]>;
  calculateGrades(req: CalculateGradesRequest): Promise<CalculationResultDto>;
  markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto>;
  listAttendance(classId: string, date: string): Promise<AttendanceDto[]>;

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
  listStudentFees(filter?: {
    enrollmentId?: string;
    studentId?: string;
  }): Promise<StudentFeeDto[]>;
  processPayment(req: ProcessPaymentRequest): Promise<PaymentDto>;
  listPayments(filter?: { feeId?: string }): Promise<PaymentDto[]>;
  listCurrencies(): Promise<CurrencyDto[]>;

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
}
