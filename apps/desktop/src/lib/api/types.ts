/**
 * @module @orbis/desktop-api
 * 
 * Strongly-typed API client interfaces for the Orbis Desktop application.
 * Both LocalClient (Tauri IPC) and RemoteClient (HTTP SDK) implement
 * this interface, ensuring transport-agnostic UI code.
 */

// ═══════════════════════════════════════════════════════════════
// SHARED TYPES
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

export interface OrbisError {
  code: string;
  message: string;
  details?: unknown;
  requestId?: string;
}

// ═══════════════════════════════════════════════════════════════
// DOMAIN DTOs — These mirror the server-side DTOs exactly
// ═══════════════════════════════════════════════════════════════

// ── Auth ────────────────────────────────────────────────────────

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  refreshToken: string;
  user: UserDto;
}

export interface SessionResponse {
  user: UserDto;
  orgId: string;
}

// ── IAM ─────────────────────────────────────────────────────────

export interface UserDto {
  id: string;
  orgId: string;
  email: string | null;
  fullName: string;
  status: string;
}

export interface RegisterUserRequest {
  email: string;
  fullName: string;
  password?: string;
  role: string;
}

// ── Organization ────────────────────────────────────────────────

export interface OrgDto {
  id: string;
  name: string;
  slug: string;
  timezone: string;
  locale: string;
  primaryColor?: string | null;
}

export interface UpdateOrgRequest {
  name?: string;
  timezone?: string;
  locale?: string;
  primaryColor?: string;
}

export interface OrgUnitDto {
  id: string;
  orgId: string;
  parentId?: string | null;
  path: string;
  depth: number;
  name: string;
  unitType: string;
  isActive: boolean;
}

export interface CreateOrgUnitRequest {
  name: string;
  unitType: string;
  parentId?: string | null;
  dataIsolation: string;
}

export interface DashboardCounts {
  totalStudents: number;
  totalTeachers: number;
  totalClasses: number;
  totalEnrollments: number;
  totalSubjects: number;
  activeAcademicYear?: string | null;
  totalPaymentsAmount: number;
  pendingFeesAmount: number;
}

// ── Settings ────────────────────────────────────────────────────

export interface OrganizationSettings {
  academic: AcademicSettings;
  enrollment: EnrollmentSettings;
  finance: FinanceSettings;
  notifications: NotificationSettings;
  autopilot: AutopilotSettings;
}

export interface AcademicSettings {
  defaultGradingSystem: string;
}

export interface EnrollmentSettings {
  registrationEnabled: boolean;
  requireAdminApproval: boolean;
  allowWaitlist: boolean;
  siblingPriorityEnabled: boolean;
  maxClassSizeDefault: number;
  allowLateEnrollment: boolean;
}

export interface FinanceSettings {
  currency: string;
  taxRate: number;
  invoiceDueDays: number;
  lateFeeType: string;
  lateFeeValue: number;
  allowPartialPayments: boolean;
  primaryPaymentGatewayId?: string | null;
  invoicingIntegrationId?: string | null;
}

export interface NotificationSettings {
  emailIntegrationId?: string | null;
  smsIntegrationId?: string | null;
  whatsappIntegrationId?: string | null;
  notifyParentsOnAbsence: boolean;
  notifyParentsOnGradePublish: boolean;
  financeReminderDays: number[];
}

export interface AutopilotSettings {
  enabled: boolean;
  mode: string;
  maxRetries: number;
  autoAdvancePeriods: boolean;
  autoLockGrades: boolean;
  gradeEntryAutoCloseDays: number;
  allowGradeEditAfterClose: boolean;
}

// ── Calendar ────────────────────────────────────────────────────

export interface AcademicYearDto {
  id: string;
  orgId: string;
  code: string;
  name: string;
  nameAr?: string | null;
  startDate: string;
  endDate: string;
  isActive: boolean;
  status: string;
}

export interface AcademicPeriodDto {
  id: string;
  yearId: string;
  name: string;
  startDate: string;
  endDate: string;
  periodType: string;
  status: string;
}

export interface AcademicContextResponse {
  currentYear?: AcademicYearDto | null;
  allYears: AcademicYearDto[];
  periods: AcademicPeriodDto[];
}

export interface PeriodTemplateDto {
  id: string;
  name: string;
}

// ── Academic ────────────────────────────────────────────────────

export interface SubjectDto {
  id: string;
  orgId: string;
  code: string;
  name: string;
  creditHours?: number | null;
  isCore: boolean;
}

export interface ClassDto {
  id: string;
  orgId: string;
  unitId: string;
  periodId: string;
  name: string;
  level: string;
}

export interface ClassSubjectDto {
  id: string;
  classId: string;
  subjectId: string;
  periodId: string;
  primaryTeacherId: string;
  weeklyHours?: number | null;
}

// ── Student ─────────────────────────────────────────────────────

export interface EnrollmentDto {
  id: string;
  studentId: string;
  classId: string;
  periodId: string;
  orgId: string;
  studentNumber: string;
  enrollmentType: string;
  status: string;
  enrollmentDate: string;
}

export interface GuardianDto {
  id: string;
  studentId: string;
  guardianId: string;
  relationship: string;
  isPrimary: boolean;
}

// ── Scheduling ──────────────────────────────────────────────────

export interface RoomDto {
  id: string;
  orgId: string;
  name: string;
  roomType: string;
  capacity: number;
}

export interface TimeSlotDto {
  id: string;
  periodId: string;
  dayOfWeek: number;
  startTime: string;
  endTime: string;
  label: string;
}

export interface ScheduledLessonDto {
  id: string;
  classSubjectId: string;
  timeSlotId: string;
  roomId?: string | null;
  teacherId?: string | null;
}

// ── Assessment ──────────────────────────────────────────────────

export interface AssessmentSchemeDto {
  id: string;
  orgId: string;
  name: string;
  gradingType: string;
}

export interface GradeDto {
  id: string;
  studentId: string;
  classSubjectId: string;
  assessmentType: string;
  score: number;
  maxScore: number;
  letterGrade?: string | null;
}

export interface AttendanceDto {
  id: string;
  studentId: string;
  classId?: string | null;
  classSubjectId?: string | null;
  scheduledLessonId?: string | null;
  date: string;
  status: string;
  note?: string | null;
}

// ── Exam ────────────────────────────────────────────────────────

export interface ExamSessionDto {
  id: string;
  orgId: string;
  periodId: string;
  name: string;
  nameAr?: string | null;
  startDate: string;
  endDate: string;
  status: string;
  isActive: boolean;
}

export interface ExamScheduleDto {
  id: string;
  sessionId: string;
  subjectId: string;
  date: string;
  maxScore: number;
  passingScore: number;
}

// ── Finance ─────────────────────────────────────────────────────

export interface AccountDto {
  id: string;
  orgId: string;
  code: string;
  name: string;
  accountType: string;
  balance: number;
}

export interface FeeTemplateDto {
  id: string;
  orgId: string;
  name: string;
  amount: number;
  feeType: string;
  isActive: boolean;
}

export interface StudentFeeDto {
  id: string;
  studentId: string;
  enrollmentId?: string | null;
  templateId: string;
  netAmount: number;
  paidAmount: number;
  status: string;
}

export interface PaymentDto {
  id: string;
  feeId?: string | null;
  amount: number;
  method: string;
  status: string;
  paidAt?: string | null;
}

export interface CurrencyDto {
  id: string;
  name: string;
  symbol: string;
}

// ── Document ────────────────────────────────────────────────────

export interface DocumentDto {
  id: string;
  orgId: string;
  entityType?: string | null;
  entityId?: string | null;
  fileName: string;
  mimeType: string;
  sizeBytes: number;
  createdAt: string;
}

// ── Integration ─────────────────────────────────────────────────

export interface IntegrationDto {
  id: string;
  orgId: string;
  appKey: string;
  appName: string;
  isActive: boolean;
}

// ── Template ────────────────────────────────────────────────────

export interface TemplateDto {
  id: string;
  name: string;
  nameAr?: string | null;
  description?: string | null;
  descriptionAr?: string | null;
  documentType: string;
  entityType: string;
  status: string;
  version: number;
  publishedVersion?: number | null;
  pageSize: string;
  pageWidthMm?: number | null;
  pageHeightMm?: number | null;
  orientation: string;
  marginsJson: string;
  documentJson: string;
  locale: string;
  masterTemplateId?: string | null;
  code?: string | null;
  tags?: string | null;
  isSystem: boolean;
  isDefault: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateTemplateRequest {
  name: string;
  nameAr?: string | null;
  description?: string | null;
  descriptionAr?: string | null;
  documentType: string;
  entityType: string;
  pageSize?: string | null;
  pageWidthMm?: number | null;
  pageHeightMm?: number | null;
  orientation?: string | null;
  marginsJson?: string | null;
  documentJson?: string | null;
  locale?: string | null;
  code?: string | null;
  tags?: string | null;
  masterTemplateId?: string | null;
}

export interface UpdateTemplateRequest {
  name?: string | null;
  nameAr?: string | null;
  description?: string | null;
  descriptionAr?: string | null;
  pageSize?: string | null;
  orientation?: string | null;
  marginsJson?: string | null;
  documentJson?: string | null;
}

export interface TemplateVersionDto {
  id: string;
  templateId: string;
  version: number;
  changeSummary?: string | null;
  createdBy: string;
  createdAt: string;
}

export interface TemplateBindingDto {
  id: string;
  reportDefinitionId: string;
  templateId: string;
  locale: string;
  format: string;
  isDefault: boolean;
  createdAt: string;
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

// ── Report Definition ───────────────────────────────────────────

export interface ReportDefinitionDto {
  id: string;
  code: string;
  name: string;
  nameAr: string;
  domain: string;
  category: string;
  description?: string | null;
  descriptionAr?: string | null;
  supportedFormats: string[];
  parameters: any[];
  isSystem: boolean;
  isActive: boolean;
}

// ═══════════════════════════════════════════════════════════════
// API CLIENT INTERFACE
// ═══════════════════════════════════════════════════════════════

/**
 * The unified API contract that every transport layer must implement.
 * UI code depends only on this interface — never on `invoke()` or `fetch()` directly.
 */
export interface OrbisApiClient {
  // ── Auth ────────────────────────────────────────────────────
  login(req: LoginRequest): Promise<LoginResponse>;
  getSession(): Promise<SessionResponse>;
  logout(): Promise<void>;

  // ── IAM ─────────────────────────────────────────────────────
  registerUser(req: RegisterUserRequest): Promise<UserDto>;
  listUsers(): Promise<UserDto[]>;
  getUser(id: string): Promise<UserDto>;

  // ── Organization ────────────────────────────────────────────
  getCurrentOrg(): Promise<OrgDto>;
  updateCurrentOrg(req: UpdateOrgRequest): Promise<OrgDto>;
  getSettings(): Promise<OrganizationSettings>;
  updateSettings(settings: OrganizationSettings): Promise<OrganizationSettings>;
  getDashboard(): Promise<DashboardCounts>;

  // ── OrgUnits ────────────────────────────────────────────────
  createOrgUnit(req: CreateOrgUnitRequest): Promise<OrgUnitDto>;
  listOrgUnits(): Promise<OrgUnitDto[]>;
  getOrgUnit(id: string): Promise<OrgUnitDto>;
  deleteOrgUnit(id: string): Promise<void>;

  // ── Calendar ────────────────────────────────────────────────
  listAcademicYears(): Promise<AcademicYearDto[]>;
  getAcademicContext(): Promise<AcademicContextResponse>;
  getAcademicYear(id: string): Promise<AcademicYearDto>;
  listAcademicPeriods(): Promise<AcademicPeriodDto[]>;
  getAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
  listPeriodTemplates(): Promise<PeriodTemplateDto[]>;

  // ── Academic ────────────────────────────────────────────────
  listSubjects(): Promise<SubjectDto[]>;
  getSubject(id: string): Promise<SubjectDto>;
  listClasses(): Promise<ClassDto[]>;
  getClass(id: string): Promise<ClassDto>;
  listClassSubjects(classId: string, periodId: string): Promise<ClassSubjectDto[]>;

  // ── Student ─────────────────────────────────────────────────
  listEnrollments(filter?: { classId?: string; studentId?: string }): Promise<EnrollmentDto[]>;
  getEnrollment(id: string): Promise<EnrollmentDto>;
  listGuardians(studentId: string): Promise<GuardianDto[]>;

  // ── Scheduling ──────────────────────────────────────────────
  listRooms(): Promise<RoomDto[]>;
  getRoom(id: string): Promise<RoomDto>;
  listTimeSlots(periodId: string): Promise<TimeSlotDto[]>;
  listScheduledLessons(filter?: { classSubjectId?: string; periodId?: string }): Promise<ScheduledLessonDto[]>;

  // ── Assessment ──────────────────────────────────────────────
  listAssessmentSchemes(): Promise<AssessmentSchemeDto[]>;
  getAssessmentScheme(id: string): Promise<AssessmentSchemeDto>;
  listGrades(filter?: { studentId?: string; classSubjectId?: string }): Promise<GradeDto[]>;
  listAttendance(classId: string, date: string): Promise<AttendanceDto[]>;

  // ── Exam ────────────────────────────────────────────────────
  listExamSessions(periodId: string): Promise<ExamSessionDto[]>;
  getExamSession(id: string): Promise<ExamSessionDto>;
  listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]>;

  // ── Finance ─────────────────────────────────────────────────
  listAccounts(): Promise<AccountDto[]>;
  listFeeTemplates(): Promise<FeeTemplateDto[]>;
  listStudentFees(filter?: { enrollmentId?: string; studentId?: string }): Promise<StudentFeeDto[]>;
  listPayments(filter?: { feeId?: string }): Promise<PaymentDto[]>;
  listCurrencies(): Promise<CurrencyDto[]>;

  // ── Document ────────────────────────────────────────────────
  listDocuments(): Promise<DocumentDto[]>;
  getDocument(id: string): Promise<DocumentDto>;
  deleteDocument(id: string): Promise<void>;

  // ── Integration ─────────────────────────────────────────────
  listIntegrations(): Promise<IntegrationDto[]>;

  // ── Templates ───────────────────────────────────────────────
  createTemplate(req: CreateTemplateRequest): Promise<TemplateDto>;
  getTemplate(id: string): Promise<TemplateDto>;
  listTemplates(filter?: { documentType?: string; entityType?: string; status?: string }): Promise<TemplateDto[]>;
  updateTemplate(id: string, req: UpdateTemplateRequest): Promise<TemplateDto>;
  deleteTemplate(id: string): Promise<void>;
  publishTemplate(id: string): Promise<TemplateDto>;
  archiveTemplate(id: string): Promise<void>;
  getTemplateVersions(id: string): Promise<TemplateVersionDto[]>;
  revertTemplateVersion(id: string, targetVersion: number): Promise<TemplateDto>;
  previewTemplate(documentJson: string, contextData: any, locale?: string): Promise<Uint8Array>;
  validateTemplate(documentJson: string): Promise<ValidationResult>;

  // ── Template Bindings ───────────────────────────────────────
  listTemplateBindings(definitionId: string): Promise<TemplateBindingDto[]>;
  createTemplateBinding(definitionId: string, templateId: string, locale: string, format: string): Promise<TemplateBindingDto>;

  // ── Report Definitions ──────────────────────────────────────
  listReportDefinitions(domain?: string, category?: string): Promise<ReportDefinitionDto[]>;
  getReportDefinition(code: string): Promise<ReportDefinitionDto>;
}

