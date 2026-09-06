/**
 * RemoteClient — HTTP transport for the OrbisApiClient interface.
 *
 * Wraps HTTP calls to the Orbis REST API. In production,
 * this is used when connecting to a remote server.
 */

import type {
  OrbisApiClient,
  ApiResponse,
  LoginRequest, LoginResponse, SessionResponse,
  RegisterUserRequest, UserDto,
  OrgDto, UpdateOrgRequest, OrgUnitDto, CreateOrgUnitRequest,
  OrganizationSettingsDto, DashboardCountsDto,
  AcademicYearDto, AcademicPeriodDto, AcademicContextResponse, PeriodTemplateDto,
  CreateYearRequest, CreatePeriodRequest,
  SubjectDto, ClassDto, ClassSubjectDto,
  CreateSubjectRequest, CreateClassRequest, AssignClassSubjectRequest,
  EnrollmentDto, GuardianDto,
  EnrollStudentRequest, AssignGuardianRequest,
  RoomDto, TimeSlotDto, ScheduledLessonDto,
  CreateRoomRequest, CreateTimeSlotRequest, ScheduleLessonRequest,
  AssessmentSchemeDto, GradeDto, AttendanceDto, CalculationResultDto,
  CreateSchemeRequest, RecordGradeRequest, MarkAttendanceRequest, CalculateGradesRequest,
  ExamSessionDto, ExamScheduleDto, SeatingDto, SecretDto, ProctorDto,
  CreateExamSessionRequest, ScheduleExamRequest, AssignSeatingRequest, AssignSecretRequest, AssignProctorRequest,
  AccountDto, FeeTemplateDto, StudentFeeDto, PaymentDto, CurrencyDto,
  CreateAccountRequest, CreateFeeTemplateRequest, GenerateFeeRequest, ProcessPaymentRequest,
  DocumentDto, UploadDocumentRequest,
  IntegrationDto, ConnectAppRequest,
  TemplateDto, CreateTemplateRequest, UpdateTemplateRequest,
  TemplateVersionDto, TemplateBindingDto,
  ValidationResult, ReportDefinitionDto,
  OrbisError,
} from './types';

export class ApiRequestError extends Error {
  constructor(
    public readonly status: number,
    public readonly error: OrbisError,
  ) {
    super(error.message);
    this.name = 'ApiRequestError';
  }
}

export class RemoteClient implements OrbisApiClient {
  private baseUrl: string;
  private token: string | null = null;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, '');
  }

  setToken(token: string | null): void {
    this.token = token;
  }

  private async request<T>(method: string, path: string, body?: unknown, query?: Record<string, string | undefined>): Promise<T> {
    const url = new URL(`${this.baseUrl}${path}`);
    if (query) {
      Object.entries(query).forEach(([k, v]) => {
        if (v !== undefined) url.searchParams.set(k, v);
      });
    }

    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (this.token) headers['Authorization'] = `Bearer ${this.token}`;

    const res = await fetch(url.toString(), {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!res.ok) {
      const errBody = await res.json().catch(() => ({ code: 'UNKNOWN', message: res.statusText }));
      throw new ApiRequestError(res.status, errBody as OrbisError);
    }

    if (res.status === 204) return undefined as unknown as T;

    const json: ApiResponse<T> = await res.json();
    return json.data;
  }

  private get<T>(path: string, query?: Record<string, string | undefined>): Promise<T> {
    return this.request<T>('GET', path, undefined, query);
  }
  private post<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>('POST', path, body);
  }
  private put<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>('PUT', path, body);
  }
  private del<T>(path: string): Promise<T> {
    return this.request<T>('DELETE', path);
  }

  // ── Auth ────────────────────────────────────────────────────
  async login(req: LoginRequest): Promise<LoginResponse> {
    const res = await this.post<LoginResponse>('/api/v1/auth/login', req);
    this.token = res.token;
    return res;
  }
  async getSession(): Promise<SessionResponse> {
    return this.get('/api/v1/auth/session');
  }
  async logout(): Promise<void> {
    this.token = null;
  }

  // ── IAM ─────────────────────────────────────────────────────
  async registerUser(req: RegisterUserRequest): Promise<UserDto> {
    return this.post('/api/v1/iam/users', req);
  }
  async listUsers(): Promise<UserDto[]> {
    return this.get('/api/v1/iam/users');
  }
  async getUser(id: string): Promise<UserDto> {
    return this.get(`/api/v1/iam/users/${id}`);
  }

  // ── Organization ────────────────────────────────────────────
  async getCurrentOrg(): Promise<OrgDto> {
    return this.get('/api/v1/organizations/current');
  }
  async updateCurrentOrg(req: UpdateOrgRequest): Promise<OrgDto> {
    return this.put('/api/v1/organizations/current', req);
  }
  async getSettings(): Promise<OrganizationSettingsDto> {
    return this.get('/api/v1/organizations/current/settings');
  }
  async updateSettings(settings: OrganizationSettingsDto): Promise<OrganizationSettingsDto> {
    return this.put('/api/v1/organizations/current/settings', settings);
  }
  async getDashboard(): Promise<DashboardCountsDto> {
    return this.get('/api/v1/organizations/current/dashboard');
  }

  // ── OrgUnits ────────────────────────────────────────────────
  async createOrgUnit(req: CreateOrgUnitRequest): Promise<OrgUnitDto> {
    return this.post('/api/v1/org-units', req);
  }
  async listOrgUnits(): Promise<OrgUnitDto[]> {
    return this.get('/api/v1/org-units');
  }
  async getOrgUnit(id: string): Promise<OrgUnitDto> {
    return this.get(`/api/v1/org-units/${id}`);
  }
  async deleteOrgUnit(id: string): Promise<void> {
    await this.del(`/api/v1/org-units/${id}`);
  }

  // ── Calendar ────────────────────────────────────────────────
  async createAcademicYear(req: CreateYearRequest): Promise<AcademicYearDto> {
    return this.post('/api/v1/academic-years', req);
  }
  async listAcademicYears(): Promise<AcademicYearDto[]> {
    return this.get('/api/v1/academic-years');
  }
  async getAcademicContext(): Promise<AcademicContextResponse> {
    return this.get('/api/v1/academic-years/current');
  }
  async getAcademicYear(id: string): Promise<AcademicYearDto> {
    return this.get(`/api/v1/academic-years/${id}`);
  }
  async createAcademicPeriod(req: CreatePeriodRequest): Promise<AcademicPeriodDto> {
    return this.post('/api/v1/academic-periods', req);
  }
  async listAcademicPeriods(): Promise<AcademicPeriodDto[]> {
    return this.get('/api/v1/academic-periods');
  }
  async getAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return this.get(`/api/v1/academic-periods/${id}`);
  }
  async listPeriodTemplates(): Promise<PeriodTemplateDto[]> {
    return this.get('/api/v1/period-templates');
  }

  // ── Academic ────────────────────────────────────────────────
  async createSubject(req: CreateSubjectRequest): Promise<SubjectDto> {
    return this.post('/api/v1/subjects', req);
  }
  async listSubjects(): Promise<SubjectDto[]> {
    return this.get('/api/v1/subjects');
  }
  async getSubject(id: string): Promise<SubjectDto> {
    return this.get(`/api/v1/subjects/${id}`);
  }
  async createClass(req: CreateClassRequest): Promise<ClassDto> {
    return this.post('/api/v1/classes', req);
  }
  async listClasses(): Promise<ClassDto[]> {
    return this.get('/api/v1/classes');
  }
  async getClass(id: string): Promise<ClassDto> {
    return this.get(`/api/v1/classes/${id}`);
  }
  async assignClassSubject(req: AssignClassSubjectRequest): Promise<ClassSubjectDto> {
    return this.post('/api/v1/class-subjects', req);
  }
  async listClassSubjects(classId: string, periodId: string): Promise<ClassSubjectDto[]> {
    return this.get('/api/v1/class-subjects', { class_id: classId, period_id: periodId });
  }

  // ── Student ─────────────────────────────────────────────────
  async enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto> {
    return this.post('/api/v1/enrollments', req);
  }
  async listEnrollments(filter?: { classId?: string; studentId?: string }): Promise<EnrollmentDto[]> {
    return this.get('/api/v1/enrollments', { class_id: filter?.classId, student_id: filter?.studentId });
  }
  async getEnrollment(id: string): Promise<EnrollmentDto> {
    return this.get(`/api/v1/enrollments/${id}`);
  }
  async assignGuardian(req: AssignGuardianRequest): Promise<GuardianDto> {
    return this.post('/api/v1/student-guardians', req);
  }
  async listGuardians(studentId: string): Promise<GuardianDto[]> {
    return this.get('/api/v1/student-guardians', { student_id: studentId });
  }

  // ── Scheduling ──────────────────────────────────────────────
  async createRoom(req: CreateRoomRequest): Promise<RoomDto> {
    return this.post('/api/v1/rooms', req);
  }
  async listRooms(): Promise<RoomDto[]> {
    return this.get('/api/v1/rooms');
  }
  async getRoom(id: string): Promise<RoomDto> {
    return this.get(`/api/v1/rooms/${id}`);
  }
  async createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto> {
    return this.post('/api/v1/time-slots', req);
  }
  async listTimeSlots(periodId: string): Promise<TimeSlotDto[]> {
    return this.get('/api/v1/time-slots', { period_id: periodId });
  }
  async scheduleLesson(req: ScheduleLessonRequest): Promise<ScheduledLessonDto> {
    return this.post('/api/v1/scheduled-lessons', req);
  }
  async listScheduledLessons(filter?: { classSubjectId?: string; periodId?: string }): Promise<ScheduledLessonDto[]> {
    return this.get('/api/v1/scheduled-lessons', { class_subject_id: filter?.classSubjectId, period_id: filter?.periodId });
  }

  // ── Assessment ──────────────────────────────────────────────
  async createAssessmentScheme(req: CreateSchemeRequest): Promise<AssessmentSchemeDto> {
    return this.post('/api/v1/assessment-schemes', req);
  }
  async listAssessmentSchemes(): Promise<AssessmentSchemeDto[]> {
    return this.get('/api/v1/assessment-schemes');
  }
  async getAssessmentScheme(id: string): Promise<AssessmentSchemeDto> {
    return this.get(`/api/v1/assessment-schemes/${id}`);
  }
  async recordGrade(req: RecordGradeRequest): Promise<GradeDto> {
    return this.post('/api/v1/grades', req);
  }
  async listGrades(filter?: { studentId?: string; classSubjectId?: string }): Promise<GradeDto[]> {
    return this.get('/api/v1/grades', { student_id: filter?.studentId, class_subject_id: filter?.classSubjectId });
  }
  async calculateGrades(req: CalculateGradesRequest): Promise<CalculationResultDto> {
    return this.post('/api/v1/grades/calculate', req);
  }
  async markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto> {
    return this.post('/api/v1/attendance', req);
  }
  async listAttendance(classId: string, date: string): Promise<AttendanceDto[]> {
    return this.get('/api/v1/attendance', { class_id: classId, date });
  }

  // ── Exam ────────────────────────────────────────────────────
  async createExamSession(req: CreateExamSessionRequest): Promise<ExamSessionDto> {
    return this.post('/api/v1/exam-sessions', req);
  }
  async listExamSessions(periodId: string): Promise<ExamSessionDto[]> {
    return this.get('/api/v1/exam-sessions', { period_id: periodId });
  }
  async getExamSession(id: string): Promise<ExamSessionDto> {
    return this.get(`/api/v1/exam-sessions/${id}`);
  }
  async scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto> {
    return this.post('/api/v1/exam-schedules', req);
  }
  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return this.get('/api/v1/exam-schedules', { session_id: sessionId });
  }
  async assignSeating(req: AssignSeatingRequest): Promise<SeatingDto> {
    return this.post('/api/v1/exam-seatings', req);
  }
  async assignSecret(req: AssignSecretRequest): Promise<SecretDto> {
    return this.post('/api/v1/exam-secrets', req);
  }
  async assignProctor(req: AssignProctorRequest): Promise<ProctorDto> {
    return this.post('/api/v1/exam-proctors', req);
  }

  // ── Finance ─────────────────────────────────────────────────
  async createAccount(req: CreateAccountRequest): Promise<AccountDto> {
    return this.post('/api/v1/accounts', req);
  }
  async listAccounts(): Promise<AccountDto[]> {
    return this.get('/api/v1/accounts');
  }
  async createFeeTemplate(req: CreateFeeTemplateRequest): Promise<FeeTemplateDto> {
    return this.post('/api/v1/fee-templates', req);
  }
  async listFeeTemplates(): Promise<FeeTemplateDto[]> {
    return this.get('/api/v1/fee-templates');
  }
  async generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto> {
    return this.post('/api/v1/student-fees', req);
  }
  async listStudentFees(filter?: { enrollmentId?: string; studentId?: string }): Promise<StudentFeeDto[]> {
    return this.get('/api/v1/student-fees', { enrollment_id: filter?.enrollmentId, student_id: filter?.studentId });
  }
  async processPayment(req: ProcessPaymentRequest): Promise<PaymentDto> {
    return this.post('/api/v1/payments', req);
  }
  async listPayments(filter?: { feeId?: string }): Promise<PaymentDto[]> {
    return this.get('/api/v1/payments', { fee_id: filter?.feeId });
  }
  async listCurrencies(): Promise<CurrencyDto[]> {
    return this.get('/api/v1/currencies');
  }

  // ── Document ────────────────────────────────────────────────
  async uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto> {
    return this.post('/api/v1/documents', req);
  }
  async listDocuments(): Promise<DocumentDto[]> {
    return this.get('/api/v1/documents');
  }
  async getDocument(id: string): Promise<DocumentDto> {
    return this.get(`/api/v1/documents/${id}`);
  }
  async deleteDocument(id: string): Promise<void> {
    await this.del(`/api/v1/documents/${id}`);
  }

  // ── Integration ─────────────────────────────────────────────
  async connectApp(req: ConnectAppRequest): Promise<IntegrationDto> {
    return this.post('/api/v1/integrations', req);
  }
  async listIntegrations(): Promise<IntegrationDto[]> {
    return this.get('/api/v1/integrations');
  }

  // ── Templates ───────────────────────────────────────────────
  async createTemplate(req: CreateTemplateRequest): Promise<TemplateDto> {
    return this.post('/api/v1/templates', req);
  }
  async getTemplate(id: string): Promise<TemplateDto> {
    return this.get(`/api/v1/templates/${id}`);
  }
  async listTemplates(filter?: { documentType?: string; entityType?: string; status?: string }): Promise<TemplateDto[]> {
    return this.get('/api/v1/templates', {
      document_type: filter?.documentType,
      entity_type: filter?.entityType,
      status: filter?.status,
    });
  }
  async updateTemplate(id: string, req: UpdateTemplateRequest): Promise<TemplateDto> {
    return this.put(`/api/v1/templates/${id}`, req);
  }
  async deleteTemplate(id: string): Promise<void> {
    await this.del(`/api/v1/templates/${id}`);
  }
  async publishTemplate(id: string): Promise<TemplateDto> {
    return this.post(`/api/v1/templates/${id}/publish`);
  }
  async archiveTemplate(id: string): Promise<void> {
    await this.post(`/api/v1/templates/${id}/archive`);
  }
  async getTemplateVersions(id: string): Promise<TemplateVersionDto[]> {
    return this.get(`/api/v1/templates/${id}/versions`);
  }
  async revertTemplateVersion(id: string, targetVersion: number): Promise<TemplateDto> {
    return this.post(`/api/v1/templates/${id}/revert`, { version: targetVersion });
  }
  async previewTemplate(documentJson: string, contextData: any, locale?: string): Promise<Uint8Array> {
    const res = await this.post<number[]>('/api/v1/templates/preview', {
      document_json: documentJson,
      context_data: contextData,
      locale,
    });
    return new Uint8Array(res);
  }
  async validateTemplate(documentJson: string): Promise<ValidationResult> {
    return this.post('/api/v1/templates/validate', { document_json: documentJson });
  }

  // ── Template Bindings ───────────────────────────────────────
  async listTemplateBindings(definitionId: string): Promise<TemplateBindingDto[]> {
    return this.get(`/api/v1/report-definitions/${definitionId}/bindings`);
  }
  async createTemplateBinding(definitionId: string, templateId: string, locale: string, format: string): Promise<TemplateBindingDto> {
    return this.post(`/api/v1/report-definitions/${definitionId}/bindings`, {
      template_id: templateId,
      locale,
      format,
    });
  }

  // ── Report Definitions ──────────────────────────────────────
  async listReportDefinitions(domain?: string, category?: string): Promise<ReportDefinitionDto[]> {
    return this.get('/api/v1/report-definitions', { domain, category });
  }
  async getReportDefinition(code: string): Promise<ReportDefinitionDto> {
    return this.get(`/api/v1/report-definitions/${code}`);
  }
}
