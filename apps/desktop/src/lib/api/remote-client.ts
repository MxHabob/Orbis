/**
 * RemoteClient — HTTP transport for the OrbisApiClient interface.
 *
 * Wraps standard fetch calls to the Orbis REST API. In production,
 * this is used when the desktop app connects to a remote server
 * instead of the local embedded database.
 */

import type {
  OrbisApiClient,
  ApiResponse,
  LoginRequest, LoginResponse, SessionResponse,
  RegisterUserRequest, UserDto,
  OrgDto, UpdateOrgRequest, OrgUnitDto, CreateOrgUnitRequest,
  OrganizationSettings, DashboardCounts,
  AcademicYearDto, AcademicPeriodDto, AcademicContextResponse, PeriodTemplateDto,
  SubjectDto, ClassDto, ClassSubjectDto,
  EnrollmentDto, GuardianDto,
  RoomDto, TimeSlotDto, ScheduledLessonDto,
  AssessmentSchemeDto, GradeDto, AttendanceDto,
  ExamSessionDto, ExamScheduleDto,
  AccountDto, FeeTemplateDto, StudentFeeDto, PaymentDto, CurrencyDto,
  DocumentDto,
  IntegrationDto,
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
  async getSettings(): Promise<OrganizationSettings> {
    return this.get('/api/v1/organizations/current/settings');
  }
  async updateSettings(settings: OrganizationSettings): Promise<OrganizationSettings> {
    return this.put('/api/v1/organizations/current/settings', settings);
  }
  async getDashboard(): Promise<DashboardCounts> {
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
  async listAcademicYears(): Promise<AcademicYearDto[]> {
    return this.get('/api/v1/academic-years');
  }
  async getAcademicContext(): Promise<AcademicContextResponse> {
    return this.get('/api/v1/academic-years/current');
  }
  async getAcademicYear(id: string): Promise<AcademicYearDto> {
    return this.get(`/api/v1/academic-years/${id}`);
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
  async listSubjects(): Promise<SubjectDto[]> {
    return this.get('/api/v1/subjects');
  }
  async getSubject(id: string): Promise<SubjectDto> {
    return this.get(`/api/v1/subjects/${id}`);
  }
  async listClasses(): Promise<ClassDto[]> {
    return this.get('/api/v1/classes');
  }
  async getClass(id: string): Promise<ClassDto> {
    return this.get(`/api/v1/classes/${id}`);
  }
  async listClassSubjects(classId: string, periodId: string): Promise<ClassSubjectDto[]> {
    return this.get('/api/v1/class-subjects', { class_id: classId, period_id: periodId });
  }

  // ── Student ─────────────────────────────────────────────────
  async listEnrollments(filter?: { classId?: string; studentId?: string }): Promise<EnrollmentDto[]> {
    return this.get('/api/v1/enrollments', { class_id: filter?.classId, student_id: filter?.studentId });
  }
  async getEnrollment(id: string): Promise<EnrollmentDto> {
    return this.get(`/api/v1/enrollments/${id}`);
  }
  async listGuardians(studentId: string): Promise<GuardianDto[]> {
    return this.get('/api/v1/student-guardians', { student_id: studentId });
  }

  // ── Scheduling ──────────────────────────────────────────────
  async listRooms(): Promise<RoomDto[]> {
    return this.get('/api/v1/rooms');
  }
  async getRoom(id: string): Promise<RoomDto> {
    return this.get(`/api/v1/rooms/${id}`);
  }
  async listTimeSlots(periodId: string): Promise<TimeSlotDto[]> {
    return this.get('/api/v1/time-slots', { period_id: periodId });
  }
  async listScheduledLessons(filter?: { classSubjectId?: string; periodId?: string }): Promise<ScheduledLessonDto[]> {
    return this.get('/api/v1/scheduled-lessons', { class_subject_id: filter?.classSubjectId, period_id: filter?.periodId });
  }

  // ── Assessment ──────────────────────────────────────────────
  async listAssessmentSchemes(): Promise<AssessmentSchemeDto[]> {
    return this.get('/api/v1/assessment-schemes');
  }
  async getAssessmentScheme(id: string): Promise<AssessmentSchemeDto> {
    return this.get(`/api/v1/assessment-schemes/${id}`);
  }
  async listGrades(filter?: { studentId?: string; classSubjectId?: string }): Promise<GradeDto[]> {
    return this.get('/api/v1/grades', { student_id: filter?.studentId, class_subject_id: filter?.classSubjectId });
  }
  async listAttendance(classId: string, date: string): Promise<AttendanceDto[]> {
    return this.get('/api/v1/attendance', { class_id: classId, date });
  }

  // ── Exam ────────────────────────────────────────────────────
  async listExamSessions(periodId: string): Promise<ExamSessionDto[]> {
    return this.get('/api/v1/exam-sessions', { period_id: periodId });
  }
  async getExamSession(id: string): Promise<ExamSessionDto> {
    return this.get(`/api/v1/exam-sessions/${id}`);
  }
  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return this.get('/api/v1/exam-schedules', { session_id: sessionId });
  }

  // ── Finance ─────────────────────────────────────────────────
  async listAccounts(): Promise<AccountDto[]> {
    return this.get('/api/v1/accounts');
  }
  async listFeeTemplates(): Promise<FeeTemplateDto[]> {
    return this.get('/api/v1/fee-templates');
  }
  async listStudentFees(filter?: { enrollmentId?: string; studentId?: string }): Promise<StudentFeeDto[]> {
    return this.get('/api/v1/student-fees', { enrollment_id: filter?.enrollmentId, student_id: filter?.studentId });
  }
  async listPayments(filter?: { feeId?: string }): Promise<PaymentDto[]> {
    return this.get('/api/v1/payments', { fee_id: filter?.feeId });
  }
  async listCurrencies(): Promise<CurrencyDto[]> {
    return this.get('/api/v1/currencies');
  }

  // ── Document ────────────────────────────────────────────────
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
    return this.post(`/api/v1/templates/${id}/revert`, { targetVersion });
  }
  async previewTemplate(documentJson: string, contextData: any, locale?: string): Promise<Uint8Array> {
    const res = await this.post<string>('/api/v1/templates/preview', { documentJson, contextData, locale });
    return new TextEncoder().encode(res);
  }
  async validateTemplate(documentJson: string): Promise<ValidationResult> {
    return this.post('/api/v1/templates/validate', { documentJson });
  }

  // ── Template Bindings ───────────────────────────────────────
  async listTemplateBindings(definitionId: string): Promise<TemplateBindingDto[]> {
    return this.get(`/api/v1/report-definitions/${definitionId}/bindings`);
  }
  async createTemplateBinding(definitionId: string, templateId: string, locale: string, format: string): Promise<TemplateBindingDto> {
    return this.post(`/api/v1/report-definitions/${definitionId}/bindings`, { templateId, locale, format });
  }

  // ── Report Definitions ──────────────────────────────────────
  async listReportDefinitions(domain?: string, category?: string): Promise<ReportDefinitionDto[]> {
    return this.get('/api/v1/report-definitions', { domain, category });
  }
  async getReportDefinition(code: string): Promise<ReportDefinitionDto> {
    return this.get(`/api/v1/report-definitions/${code}`);
  }
}
