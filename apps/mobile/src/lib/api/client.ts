/**
 * @module @orbis/mobile-api/client
 *
 * Full-featured HTTP Client for the Orbis Mobile Application.
 * Implements the OrbisApiClient contract covering all 15 operational domains.
 * Automatically injects Bearer JWT authentication tokens, handles API envelopes,
 * and normalizes errors into OrbisError/ApiRequestError.
 */

import { getApiBaseUrl } from "./config";
import {
  type OrbisApiClient,
  type ApiResponse,
  type LoginRequest,
  type LoginResponse,
  type SessionResponse,
  type RegisterUserRequest,
  type UserDto,
  type OrgDto,
  type UpdateOrgRequest,
  type OrgUnitDto,
  type CreateOrgUnitRequest,
  type OrganizationSettingsDto,
  type DashboardCountsDto,
  type AcademicYearDto,
  type AcademicPeriodDto,
  type AcademicContextResponse,
  type PeriodTemplateDto,
  type CreateYearRequest,
  type CreatePeriodRequest,
  type SubjectDto,
  type ClassDto,
  type ClassSubjectDto,
  type CreateSubjectRequest,
  type CreateClassRequest,
  type AssignClassSubjectRequest,
  type EnrollmentDto,
  type GuardianDto,
  type EnrollStudentRequest,
  type AssignGuardianRequest,
  type RoomDto,
  type TimeSlotDto,
  type ScheduledLessonDto,
  type CreateRoomRequest,
  type CreateTimeSlotRequest,
  type ScheduleLessonRequest,
  type AssessmentSchemeDto,
  type GradeDto,
  type AttendanceDto,
  type CalculationResultDto,
  type CreateSchemeRequest,
  type RecordGradeRequest,
  type MarkAttendanceRequest,
  type CalculateGradesRequest,
  type ExamSessionDto,
  type ExamScheduleDto,
  type SeatingDto,
  type SecretDto,
  type ProctorDto,
  type CreateExamSessionRequest,
  type ScheduleExamRequest,
  type AssignSeatingRequest,
  type AssignSecretRequest,
  type AssignProctorRequest,
  type AccountDto,
  type FeeTemplateDto,
  type StudentFeeDto,
  type PaymentDto,
  type CurrencyDto,
  type CreateAccountRequest,
  type CreateFeeTemplateRequest,
  type GenerateFeeRequest,
  type ProcessPaymentRequest,
  type DocumentDto,
  type UploadDocumentRequest,
  type IntegrationDto,
  type ConnectAppRequest,
  type IntegrationStatusResponse,
  type TemplateDto,
  type CreateTemplateRequest,
  type UpdateTemplateRequest,
  type TemplateVersionDto,
  type TemplateBindingDto,
  type ValidationResult,
  type ReportDefinitionDto,
  type MfaEnrollRequest,
  type MfaEnrollResponse,
  type MfaVerifyEnrollmentRequest,
  type MfaChallengeRequest,
  type GoogleAuthRequest,
  type UserDeviceDto,
  type UserIdentityDto,
  OrbisError,
  ApiRequestError,
} from "./types";

export class MobileApiClient implements OrbisApiClient {
  private token: string | null = null;
  private customBaseUrl: string | null = null;

  constructor(baseUrl?: string) {
    if (baseUrl) {
      this.customBaseUrl = baseUrl.replace(/\/$/, "");
    }
  }

  public getBaseUrl(): string {
    return this.customBaseUrl ?? getApiBaseUrl();
  }

  public setBaseUrl(url: string | null): void {
    this.customBaseUrl = url ? url.replace(/\/$/, "") : null;
  }

  public setToken(token: string | null): void {
    this.token = token;
  }

  public getToken(): string | null {
    return this.token;
  }

  public clearToken(): void {
    this.token = null;
  }

  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
    query?: Record<string, string | number | boolean | undefined | null>,
  ): Promise<T> {
    const base = this.getBaseUrl();
    const url = new URL(`${base}${path}`);

    if (query) {
      Object.entries(query).forEach(([k, v]) => {
        if (v !== undefined && v !== null) {
          url.searchParams.set(k, String(v));
        }
      });
    }

    const headers: Record<string, string> = {
      Accept: "application/json",
    };

    if (!(body instanceof FormData)) {
      headers["Content-Type"] = "application/json";
    }

    if (this.token) {
      headers["Authorization"] = `Bearer ${this.token}`;
    }

    let response: Response;
    try {
      response = await fetch(url.toString(), {
        method,
        headers,
        body:
          body instanceof FormData
            ? body
            : body !== undefined
              ? JSON.stringify(body)
              : undefined,
      });
    } catch (networkError) {
      throw new OrbisError(
        `Network request failed: ${networkError instanceof Error ? networkError.message : String(networkError)}`,
        "NETWORK_ERROR",
        networkError,
      );
    }

    if (!response.ok) {
      let errBody: unknown;
      try {
        errBody = await response.json();
      } catch {
        errBody = {
          code: "HTTP_ERROR",
          message:
            response.statusText ||
            `Request failed with status ${response.status}`,
        };
      }
      const orbisErr = OrbisError.fromUnknown(errBody);
      throw new ApiRequestError(response.status, orbisErr);
    }

    if (response.status === 204) {
      return undefined as unknown as T;
    }

    const json: ApiResponse<T> = await response.json();
    return json.data !== undefined ? json.data : (json as unknown as T);
  }

  private get<T>(
    path: string,
    query?: Record<string, string | number | boolean | undefined | null>,
  ): Promise<T> {
    return this.request<T>("GET", path, undefined, query);
  }

  private post<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>("POST", path, body);
  }

  private put<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>("PUT", path, body);
  }

  private del<T>(path: string): Promise<T> {
    return this.request<T>("DELETE", path);
  }

  // ── 1. Auth, MFA & Devices ──────────────────────────────────
  async login(req: LoginRequest): Promise<LoginResponse> {
    const res = await this.post<LoginResponse>("/api/v1/auth/login", req);
    if (res.token) {
      this.token = res.token;
    }
    return res;
  }

  async getSession(): Promise<SessionResponse> {
    return this.get<SessionResponse>("/api/v1/auth/session");
  }

  async logout(): Promise<void> {
    try {
      await this.post("/api/v1/auth/logout");
    } catch {
      // Ignore network errors on logout
    } finally {
      this.token = null;
    }
  }

  async mfaEnroll(req?: MfaEnrollRequest): Promise<MfaEnrollResponse> {
    return this.post<MfaEnrollResponse>("/api/v1/auth/mfa/enroll", req ?? {});
  }

  async mfaVerifyEnrollment(req: MfaVerifyEnrollmentRequest): Promise<boolean> {
    return this.post<boolean>("/api/v1/auth/mfa/verify-enrollment", req);
  }

  async mfaChallenge(req: MfaChallengeRequest): Promise<LoginResponse> {
    const res = await this.post<LoginResponse>(
      "/api/v1/auth/mfa/challenge",
      req,
    );
    if (res.token) {
      this.token = res.token;
    }
    return res;
  }

  async linkGoogle(req: GoogleAuthRequest): Promise<UserIdentityDto> {
    return this.post<UserIdentityDto>(
      "/api/v1/auth/identities/google/link",
      req,
    );
  }

  async loginWithGoogle(req: GoogleAuthRequest): Promise<LoginResponse> {
    const res = await this.post<LoginResponse>(
      "/api/v1/auth/identities/google/login",
      req,
    );
    if (res.token) {
      this.token = res.token;
    }
    return res;
  }

  async listDevices(): Promise<UserDeviceDto[]> {
    return this.get<UserDeviceDto[]>("/api/v1/auth/devices");
  }

  async revokeDevice(id: string): Promise<boolean> {
    return this.del<boolean>(`/api/v1/auth/devices/${id}`);
  }

  // ── 2. IAM ──────────────────────────────────────────────────
  async registerUser(req: RegisterUserRequest): Promise<UserDto> {
    return this.post<UserDto>("/api/v1/iam/users", req);
  }

  async listUsers(): Promise<UserDto[]> {
    return this.get<UserDto[]>("/api/v1/iam/users");
  }

  async getUser(id: string): Promise<UserDto> {
    return this.get<UserDto>(`/api/v1/iam/users/${id}`);
  }

  async updateUser(id: string, req: { full_name?: string }): Promise<UserDto> {
    return this.put<UserDto>(`/api/v1/iam/users/${id}`, req);
  }

  // ── 3. Organization ─────────────────────────────────────────
  async getCurrentOrg(): Promise<OrgDto> {
    return this.get<OrgDto>("/api/v1/organizations/current");
  }

  async updateCurrentOrg(req: UpdateOrgRequest): Promise<OrgDto> {
    return this.put<OrgDto>("/api/v1/organizations/current", req);
  }

  async getSettings(): Promise<OrganizationSettingsDto> {
    return this.get<OrganizationSettingsDto>(
      "/api/v1/organizations/current/settings",
    );
  }

  async updateSettings(
    settings: OrganizationSettingsDto,
  ): Promise<OrganizationSettingsDto> {
    return this.put<OrganizationSettingsDto>(
      "/api/v1/organizations/current/settings",
      settings,
    );
  }

  async getDashboard(): Promise<DashboardCountsDto> {
    return this.get<DashboardCountsDto>(
      "/api/v1/organizations/current/dashboard",
    );
  }

  // ── 4. OrgUnits ─────────────────────────────────────────────
  async createOrgUnit(req: CreateOrgUnitRequest): Promise<OrgUnitDto> {
    return this.post<OrgUnitDto>("/api/v1/org-units", req);
  }

  async listOrgUnits(): Promise<OrgUnitDto[]> {
    return this.get<OrgUnitDto[]>("/api/v1/org-units");
  }

  async getOrgUnit(id: string): Promise<OrgUnitDto> {
    return this.get<OrgUnitDto>(`/api/v1/org-units/${id}`);
  }

  async deleteOrgUnit(id: string): Promise<void> {
    await this.del(`/api/v1/org-units/${id}`);
  }

  async selectOrgUnit(_id?: string): Promise<void> {
    // Mobile context selector
  }

  // ── 5. Calendar ─────────────────────────────────────────────
  async createAcademicYear(req: CreateYearRequest): Promise<AcademicYearDto> {
    return this.post<AcademicYearDto>("/api/v1/academic-years", req);
  }

  async listAcademicYears(): Promise<AcademicYearDto[]> {
    return this.get<AcademicYearDto[]>("/api/v1/academic-years");
  }

  async getAcademicContext(): Promise<AcademicContextResponse> {
    return this.get<AcademicContextResponse>("/api/v1/academic-years/current");
  }

  async getAcademicYear(id: string): Promise<AcademicYearDto> {
    return this.get<AcademicYearDto>(`/api/v1/academic-years/${id}`);
  }

  async createAcademicPeriod(
    req: CreatePeriodRequest,
  ): Promise<AcademicPeriodDto> {
    return this.post<AcademicPeriodDto>("/api/v1/academic-periods", req);
  }

  async listAcademicPeriods(): Promise<AcademicPeriodDto[]> {
    return this.get<AcademicPeriodDto[]>("/api/v1/academic-periods");
  }

  async getAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return this.get<AcademicPeriodDto>(`/api/v1/academic-periods/${id}`);
  }

  async listPeriodTemplates(): Promise<PeriodTemplateDto[]> {
    return this.get<PeriodTemplateDto[]>("/api/v1/period-templates");
  }

  // ── 6. Academic ─────────────────────────────────────────────
  async createSubject(req: CreateSubjectRequest): Promise<SubjectDto> {
    return this.post<SubjectDto>("/api/v1/subjects", req);
  }

  async listSubjects(): Promise<SubjectDto[]> {
    return this.get<SubjectDto[]>("/api/v1/subjects");
  }

  async getSubject(id: string): Promise<SubjectDto> {
    return this.get<SubjectDto>(`/api/v1/subjects/${id}`);
  }

  async createClass(req: CreateClassRequest): Promise<ClassDto> {
    return this.post<ClassDto>("/api/v1/classes", req);
  }

  async listClasses(): Promise<ClassDto[]> {
    return this.get<ClassDto[]>("/api/v1/classes");
  }

  async getClass(id: string): Promise<ClassDto> {
    return this.get<ClassDto>(`/api/v1/classes/${id}`);
  }

  async assignClassSubject(
    req: AssignClassSubjectRequest,
  ): Promise<ClassSubjectDto> {
    return this.post<ClassSubjectDto>("/api/v1/class-subjects", req);
  }

  async listClassSubjects(
    classId: string,
    periodId: string,
  ): Promise<ClassSubjectDto[]> {
    return this.get<ClassSubjectDto[]>("/api/v1/class-subjects", {
      class_id: classId,
      period_id: periodId,
    });
  }

  // ── 7. Student ──────────────────────────────────────────────
  async enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto> {
    return this.post<EnrollmentDto>("/api/v1/enrollments", req);
  }

  async listEnrollments(filter?: {
    classId?: string;
    studentId?: string;
  }): Promise<EnrollmentDto[]> {
    return this.get<EnrollmentDto[]>("/api/v1/enrollments", {
      class_id: filter?.classId,
      student_id: filter?.studentId,
    });
  }

  async getEnrollment(id: string): Promise<EnrollmentDto> {
    return this.get<EnrollmentDto>(`/api/v1/enrollments/${id}`);
  }

  async assignGuardian(req: AssignGuardianRequest): Promise<GuardianDto> {
    return this.post<GuardianDto>("/api/v1/student-guardians", req);
  }

  async listGuardians(studentId: string): Promise<GuardianDto[]> {
    return this.get<GuardianDto[]>("/api/v1/student-guardians", {
      student_id: studentId,
    });
  }

  // ── 8. Scheduling ───────────────────────────────────────────
  async createRoom(req: CreateRoomRequest): Promise<RoomDto> {
    return this.post<RoomDto>("/api/v1/rooms", req);
  }

  async listRooms(): Promise<RoomDto[]> {
    return this.get<RoomDto[]>("/api/v1/rooms");
  }

  async getRoom(id: string): Promise<RoomDto> {
    return this.get<RoomDto>(`/api/v1/rooms/${id}`);
  }

  async createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto> {
    return this.post<TimeSlotDto>("/api/v1/time-slots", req);
  }

  async listTimeSlots(periodId: string): Promise<TimeSlotDto[]> {
    return this.get<TimeSlotDto[]>("/api/v1/time-slots", {
      period_id: periodId,
    });
  }

  async scheduleLesson(
    req: ScheduleLessonRequest,
  ): Promise<ScheduledLessonDto> {
    return this.post<ScheduledLessonDto>("/api/v1/scheduled-lessons", req);
  }

  async listScheduledLessons(filter?: {
    classSubjectId?: string;
    periodId?: string;
  }): Promise<ScheduledLessonDto[]> {
    return this.get<ScheduledLessonDto[]>("/api/v1/scheduled-lessons", {
      class_subject_id: filter?.classSubjectId,
      period_id: filter?.periodId,
    });
  }

  // ── 9. Assessment ───────────────────────────────────────────
  async createAssessmentScheme(
    req: CreateSchemeRequest,
  ): Promise<AssessmentSchemeDto> {
    return this.post<AssessmentSchemeDto>("/api/v1/assessment-schemes", req);
  }

  async listAssessmentSchemes(): Promise<AssessmentSchemeDto[]> {
    return this.get<AssessmentSchemeDto[]>("/api/v1/assessment-schemes");
  }

  async getAssessmentScheme(id: string): Promise<AssessmentSchemeDto> {
    return this.get<AssessmentSchemeDto>(`/api/v1/assessment-schemes/${id}`);
  }

  async recordGrade(req: RecordGradeRequest): Promise<GradeDto> {
    return this.post<GradeDto>("/api/v1/grades", req);
  }

  async listGrades(filter?: {
    studentId?: string;
    classSubjectId?: string;
  }): Promise<GradeDto[]> {
    return this.get<GradeDto[]>("/api/v1/grades", {
      student_id: filter?.studentId,
      class_subject_id: filter?.classSubjectId,
    });
  }

  async calculateGrades(
    req: CalculateGradesRequest,
  ): Promise<CalculationResultDto> {
    return this.post<CalculationResultDto>("/api/v1/grades/calculate", req);
  }

  async markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto> {
    return this.post<AttendanceDto>("/api/v1/attendance", req);
  }

  async listAttendance(
    classIdOrFilter?:
      | string
      | { classId?: string; studentId?: string; date?: string },
    date?: string,
  ): Promise<AttendanceDto[]> {
    if (typeof classIdOrFilter === "string") {
      return this.get<AttendanceDto[]>("/api/v1/attendance", {
        class_id: classIdOrFilter,
        date,
      });
    }
    return this.get<AttendanceDto[]>("/api/v1/attendance", {
      class_id: classIdOrFilter?.classId,
      student_id: classIdOrFilter?.studentId,
      date: classIdOrFilter?.date ?? date,
    });
  }

  // ── 10. Exam ────────────────────────────────────────────────
  async createExamSession(
    req: CreateExamSessionRequest,
  ): Promise<ExamSessionDto> {
    return this.post<ExamSessionDto>("/api/v1/exam-sessions", req);
  }

  async listExamSessions(periodId: string): Promise<ExamSessionDto[]> {
    return this.get<ExamSessionDto[]>("/api/v1/exam-sessions", {
      period_id: periodId,
    });
  }

  async getExamSession(id: string): Promise<ExamSessionDto> {
    return this.get<ExamSessionDto>(`/api/v1/exam-sessions/${id}`);
  }

  async scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto> {
    return this.post<ExamScheduleDto>("/api/v1/exam-schedules", req);
  }

  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return this.get<ExamScheduleDto[]>("/api/v1/exam-schedules", {
      session_id: sessionId,
    });
  }

  async assignSeating(req: AssignSeatingRequest): Promise<SeatingDto> {
    return this.post<SeatingDto>("/api/v1/exam-seating", req);
  }

  async assignSecret(req: AssignSecretRequest): Promise<SecretDto> {
    return this.post<SecretDto>("/api/v1/exam-secrets", req);
  }

  async assignProctor(req: AssignProctorRequest): Promise<ProctorDto> {
    return this.post<ProctorDto>("/api/v1/exam-proctors", req);
  }

  // ── 11. Finance ─────────────────────────────────────────────
  async createAccount(req: CreateAccountRequest): Promise<AccountDto> {
    return this.post<AccountDto>("/api/v1/accounts", req);
  }

  async listAccounts(): Promise<AccountDto[]> {
    return this.get<AccountDto[]>("/api/v1/accounts");
  }

  async createFeeTemplate(
    req: CreateFeeTemplateRequest,
  ): Promise<FeeTemplateDto> {
    return this.post<FeeTemplateDto>("/api/v1/fee-templates", req);
  }

  async listFeeTemplates(): Promise<FeeTemplateDto[]> {
    return this.get<FeeTemplateDto[]>("/api/v1/fee-templates");
  }

  async generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto> {
    return this.post<StudentFeeDto>("/api/v1/student-fees", req);
  }

  async listStudentFees(filter?: {
    enrollmentId?: string;
    studentId?: string;
  }): Promise<StudentFeeDto[]> {
    return this.get<StudentFeeDto[]>("/api/v1/student-fees", {
      enrollment_id: filter?.enrollmentId,
      student_id: filter?.studentId,
    });
  }

  async processPayment(req: ProcessPaymentRequest): Promise<PaymentDto> {
    return this.post<PaymentDto>("/api/v1/payments", req);
  }

  async listPayments(filter?: { feeId?: string }): Promise<PaymentDto[]> {
    return this.get<PaymentDto[]>("/api/v1/payments", {
      fee_id: filter?.feeId,
    });
  }

  async listCurrencies(): Promise<CurrencyDto[]> {
    return this.get<CurrencyDto[]>("/api/v1/currencies");
  }

  // ── 12. Document ────────────────────────────────────────────
  async uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto> {
    return this.post<DocumentDto>("/api/v1/documents", req);
  }

  async listDocuments(): Promise<DocumentDto[]> {
    return this.get<DocumentDto[]>("/api/v1/documents");
  }

  async getDocument(id: string): Promise<DocumentDto> {
    return this.get<DocumentDto>(`/api/v1/documents/${id}`);
  }

  async deleteDocument(id: string): Promise<void> {
    await this.del(`/api/v1/documents/${id}`);
  }

  // ── 13. Integration ─────────────────────────────────────────
  async connectApp(req: ConnectAppRequest): Promise<IntegrationDto> {
    return this.post<IntegrationDto>("/api/v1/integrations", req);
  }

  async listIntegrations(): Promise<IntegrationDto[]> {
    return this.get<IntegrationDto[]>("/api/v1/integrations");
  }

  async getIntegrationStatus(
    providerName: string,
  ): Promise<IntegrationStatusResponse> {
    return this.get<IntegrationStatusResponse>(
      `/api/v1/integrations/${providerName}/status`,
    );
  }

  async disconnectApp(providerName: string): Promise<boolean> {
    return this.del<boolean>(`/api/v1/integrations/${providerName}`);
  }

  async toggleIntegration(
    providerName: string,
    isEnabled: boolean,
  ): Promise<boolean> {
    return this.put<boolean>(`/api/v1/integrations/${providerName}/toggle`, {
      is_enabled: isEnabled,
    });
  }

  // ── 14. Templates ───────────────────────────────────────────
  async createTemplate(req: CreateTemplateRequest): Promise<TemplateDto> {
    return this.post<TemplateDto>("/api/v1/templates", req);
  }

  async getTemplate(id: string): Promise<TemplateDto> {
    return this.get<TemplateDto>(`/api/v1/templates/${id}`);
  }

  async listTemplates(filter?: {
    documentType?: string;
    entityType?: string;
    status?: string;
  }): Promise<TemplateDto[]> {
    return this.get<TemplateDto[]>("/api/v1/templates", {
      document_type: filter?.documentType,
      entity_type: filter?.entityType,
      status: filter?.status,
    });
  }

  async updateTemplate(
    id: string,
    req: UpdateTemplateRequest,
  ): Promise<TemplateDto> {
    return this.put<TemplateDto>(`/api/v1/templates/${id}`, req);
  }

  async deleteTemplate(id: string): Promise<void> {
    await this.del(`/api/v1/templates/${id}`);
  }

  async publishTemplate(id: string): Promise<TemplateDto> {
    return this.post<TemplateDto>(`/api/v1/templates/${id}/publish`);
  }

  async archiveTemplate(id: string): Promise<void> {
    await this.post(`/api/v1/templates/${id}/archive`);
  }

  async getTemplateVersions(id: string): Promise<TemplateVersionDto[]> {
    return this.get<TemplateVersionDto[]>(`/api/v1/templates/${id}/versions`);
  }

  async revertTemplateVersion(
    id: string,
    targetVersion: number,
  ): Promise<TemplateDto> {
    return this.post<TemplateDto>(`/api/v1/templates/${id}/revert`, {
      version: targetVersion,
    });
  }

  async previewTemplate(
    documentJson: string,
    contextData: any,
    locale?: string,
  ): Promise<Uint8Array> {
    const res = await this.post<number[]>("/api/v1/templates/preview", {
      document_json: documentJson,
      context_data: contextData,
      locale,
    });
    return new Uint8Array(res);
  }

  async validateTemplate(documentJson: string): Promise<ValidationResult> {
    return this.post<ValidationResult>("/api/v1/templates/validate", {
      document_json: documentJson,
    });
  }

  // ── 15. Template Bindings & Report Definitions ──────────────
  async listTemplateBindings(
    definitionId: string,
  ): Promise<TemplateBindingDto[]> {
    return this.get<TemplateBindingDto[]>(
      `/api/v1/report-definitions/${definitionId}/bindings`,
    );
  }

  async createTemplateBinding(
    definitionId: string,
    templateId: string,
    locale: string,
    format: string,
  ): Promise<TemplateBindingDto> {
    return this.post<TemplateBindingDto>(
      `/api/v1/report-definitions/${definitionId}/bindings`,
      {
        template_id: templateId,
        locale,
        format,
      },
    );
  }

  async listReportDefinitions(
    domain?: string,
    category?: string,
  ): Promise<ReportDefinitionDto[]> {
    return this.get<ReportDefinitionDto[]>("/api/v1/report-definitions", {
      domain,
      category,
    });
  }

  async getReportDefinition(code: string): Promise<ReportDefinitionDto> {
    return this.get<ReportDefinitionDto>(`/api/v1/report-definitions/${code}`);
  }
}

// Global Singleton Instance for easy import
export const api = new MobileApiClient();
export const getApiClient = (): MobileApiClient => api;
