/**
 * LocalClient — Tauri IPC transport for the OrbisApiClient interface.
 *
 * Each method calls `invoke<T>('command_name', args)` and unwraps the
 * `ApiResponse<T>` envelope.
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  OrbisApiClient,
  LoginRequest,
  LoginResponse,
  SessionResponse,
  RegisterUserRequest,
  UpdateUserRequest,
  UserDto,
  RoleDto,
  SaveRoleRequest,
  PermissionRuleDto,
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
  UpdateSchemeRequest,
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
  IntegrationStatusResponse,
  ConnectAppRequest,
  TemplateDto,
  CreateTemplateRequest,
  UpdateTemplateRequest,
  TemplateVersionDto,
  TemplateBindingDto,
  ValidationResult,
  ReportDefinitionDto,
  StudentDto,
  StudentDetailDto,
  RegisterStudentInput,
  StudentFilterParams,
  EnrollmentFilterParams,
  UpdateStudentStatusInput,
  ChangeStudyModeInput,
  PaginatedList,
  ReportJobStatusDto,
  BatchPrintParams,
  JournalEntryDto,
  DiscountTemplateDto,
  CreateDiscountTemplateRequest,
  StudentFeeFilterParams,
  PaymentFilterParams,
  PrerequisiteCheckResultDto,
  ApprovalStepDto,
  OverrideRequestDto,
  SubmitOverrideRequestInput,
  ApproveOverrideInput,
  RejectOverrideInput,
  PromotionJobDto,
  PromotionDetailDto,
  StudentProgressionItemInput,
  PromoteCohortInput,
  CumulativeGpaRecordDto,
  CalculateGpaInput,
  HealthResponse,
  OrganizationReadiness,
  EntitlementResult,
  BootstrapRequest,
  BootstrapResult,
  InitialSetupRequest,
  ConnectionConfig,
  ConnectionMode,
  ConnectionValidation,
  RemoteHealthResult,
  ConnectResult,
} from "./types";

function unwrap<T>(response: { data: T } | T): T {
  if (response && typeof response === "object" && "data" in response) {
    return (response as { data: T }).data;
  }
  return response as T;
}

export class LocalClient implements OrbisApiClient {
  // ── Auth & MFA & Devices ────────────────────────────────────
  async login(req: LoginRequest): Promise<LoginResponse> {
    return unwrap(await invoke("auth_login", { request: req }));
  }
  async getSession(): Promise<SessionResponse> {
    const token =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("orbis_token")
        : null;
    return unwrap(await invoke("auth_get_session", { request: { token } }));
  }
  async logout(): Promise<void> {
    await invoke("auth_logout");
  }
  async mfaEnroll(
    req?: import("./types").MfaEnrollRequest,
  ): Promise<import("./types").MfaEnrollResponse> {
    return unwrap(await invoke("auth_mfa_enroll", { request: req ?? {} }));
  }
  async mfaVerifyEnrollment(
    req: import("./types").MfaVerifyEnrollmentRequest,
  ): Promise<boolean> {
    return unwrap(await invoke("auth_mfa_verify_enrollment", { request: req }));
  }
  async mfaChallenge(
    req: import("./types").MfaChallengeRequest,
  ): Promise<LoginResponse> {
    return unwrap(await invoke("auth_mfa_challenge", { request: req }));
  }
  async linkGoogle(
    req: import("./types").GoogleAuthRequest,
  ): Promise<import("./types").UserIdentityDto> {
    return unwrap(await invoke("auth_link_google", { request: req }));
  }
  async loginWithGoogle(
    req: import("./types").GoogleAuthRequest,
  ): Promise<LoginResponse> {
    return unwrap(await invoke("auth_login_google", { request: req }));
  }
  async listDevices(): Promise<import("./types").UserDeviceDto[]> {
    return unwrap(await invoke("auth_list_devices"));
  }
  async revokeDevice(id: string): Promise<boolean> {
    return unwrap(await invoke("auth_revoke_device", { id }));
  }

  // ── IAM ─────────────────────────────────────────────────────
  async registerUser(req: RegisterUserRequest): Promise<UserDto> {
    return unwrap(await invoke("iam_register_user", { request: req }));
  }
  async listUsers(): Promise<UserDto[]> {
    return unwrap(await invoke("iam_list_users"));
  }
  async getUser(id: string): Promise<UserDto> {
    return unwrap(await invoke("iam_get_user", { id }));
  }
  async updateUser(id: string, req: UpdateUserRequest): Promise<UserDto> {
    return unwrap(await invoke("iam_update_user", { id, request: req }));
  }
  async listRoles(): Promise<RoleDto[]> {
    return unwrap(await invoke("iam_list_roles"));
  }
  async saveRole(req: SaveRoleRequest): Promise<RoleDto> {
    return unwrap(await invoke("iam_save_role", { request: req }));
  }
  async listPermissions(): Promise<PermissionRuleDto[]> {
    return unwrap(await invoke("iam_list_permissions"));
  }
  async savePermissions(req: PermissionRuleDto[]): Promise<boolean> {
    return unwrap(await invoke("iam_save_permissions", { request: req }));
  }

  // ── Organization ────────────────────────────────────────────
  async getCurrentOrg(): Promise<OrgDto> {
    return unwrap(await invoke("org_get_current"));
  }
  async updateCurrentOrg(req: UpdateOrgRequest): Promise<OrgDto> {
    return unwrap(await invoke("org_update_current", { request: req }));
  }
  async getSettings(): Promise<OrganizationSettingsDto> {
    return unwrap(await invoke("org_get_settings"));
  }
  async updateSettings(
    settings: OrganizationSettingsDto,
  ): Promise<OrganizationSettingsDto> {
    return unwrap(await invoke("org_update_settings", { settings }));
  }
  async getDashboard(): Promise<DashboardCountsDto> {
    return unwrap(await invoke("org_get_dashboard"));
  }

  // ── OrgUnits ────────────────────────────────────────────────
  async createOrgUnit(req: CreateOrgUnitRequest): Promise<OrgUnitDto> {
    return unwrap(await invoke("org_create_unit", { request: req }));
  }
  async listOrgUnits(): Promise<OrgUnitDto[]> {
    return unwrap(await invoke("org_list_units"));
  }
  async getOrgUnit(id: string): Promise<OrgUnitDto> {
    return unwrap(await invoke("org_get_unit", { id }));
  }
  async deleteOrgUnit(id: string): Promise<void> {
    await invoke("org_delete_unit", { id });
  }
  async selectOrgUnit(id?: string): Promise<void> {
    await invoke("org_select_unit", { unitId: id ?? null });
  }

  // ── Calendar ────────────────────────────────────────────────
  async createAcademicYear(req: CreateYearRequest): Promise<AcademicYearDto> {
    return unwrap(await invoke("calendar_create_year", { request: req }));
  }
  async listAcademicYears(): Promise<AcademicYearDto[]> {
    return unwrap(await invoke("calendar_list_years"));
  }
  async getAcademicContext(): Promise<AcademicContextResponse> {
    return unwrap(await invoke("calendar_get_context"));
  }
  async getAcademicYear(id: string): Promise<AcademicYearDto> {
    return unwrap(await invoke("calendar_get_year", { id }));
  }
  async createAcademicPeriod(
    req: CreatePeriodRequest,
  ): Promise<AcademicPeriodDto> {
    return unwrap(await invoke("calendar_create_period", { request: req }));
  }
  async listAcademicPeriods(): Promise<AcademicPeriodDto[]> {
    return unwrap(await invoke("calendar_list_periods"));
  }
  async getAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return unwrap(await invoke("calendar_get_period", { id }));
  }
  async lockAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return unwrap(await invoke("calendar_lock_period", { id }));
  }
  async unlockAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return unwrap(await invoke("calendar_unlock_period", { id }));
  }
  async closeAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return unwrap(await invoke("calendar_close_period", { id }));
  }
  async listPeriodTemplates(): Promise<PeriodTemplateDto[]> {
    return unwrap(await invoke("calendar_list_period_templates"));
  }

  // ── Academic ────────────────────────────────────────────────
  async createSubject(req: CreateSubjectRequest): Promise<SubjectDto> {
    return unwrap(await invoke("academic_create_subject", { request: req }));
  }
  async listSubjects(): Promise<SubjectDto[]> {
    return unwrap(await invoke("academic_list_subjects"));
  }
  async getSubject(id: string): Promise<SubjectDto> {
    return unwrap(await invoke("academic_get_subject", { id }));
  }
  async createClass(req: CreateClassRequest): Promise<ClassDto> {
    return unwrap(await invoke("academic_create_class", { request: req }));
  }
  async listClasses(): Promise<ClassDto[]> {
    return unwrap(await invoke("academic_list_classes"));
  }
  async getClass(id: string): Promise<ClassDto> {
    return unwrap(await invoke("academic_get_class", { id }));
  }
  async assignClassSubject(
    req: AssignClassSubjectRequest,
  ): Promise<ClassSubjectDto> {
    return unwrap(
      await invoke("academic_assign_class_subject", { request: req }),
    );
  }
  async listClassSubjects(
    classId: string,
    periodId: string,
  ): Promise<ClassSubjectDto[]> {
    return unwrap(
      await invoke("academic_list_class_subjects", { classId, periodId }),
    );
  }
  async checkPrerequisites(
    studentId: string,
    subjectId: string,
  ): Promise<PrerequisiteCheckResultDto[]> {
    return unwrap(
      await invoke("academic_check_prerequisites", { studentId, subjectId }),
    );
  }

  // ── Student ─────────────────────────────────────────────────
  async registerStudent(req: RegisterStudentInput): Promise<StudentDetailDto> {
    return unwrap(await invoke("student_register_student", { request: req }));
  }
  async listStudents(filter?: StudentFilterParams): Promise<PaginatedList<StudentDetailDto>> {
    return unwrap(await invoke("student_list_students", { filter: filter ?? {} }));
  }
  async getStudent(id: string): Promise<StudentDetailDto> {
    return unwrap(await invoke("student_get_student", { id }));
  }
  async updateStudentStatus(req: UpdateStudentStatusInput): Promise<StudentDto> {
    return unwrap(await invoke("student_update_student_status", { request: req }));
  }
  async changeStudyMode(req: ChangeStudyModeInput): Promise<EnrollmentDto> {
    return unwrap(await invoke("student_change_study_mode", { request: req }));
  }
  async enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto> {
    return unwrap(await invoke("student_enroll_student", { request: req }));
  }
  async listEnrollments(filter?: EnrollmentFilterParams): Promise<EnrollmentDto[]> {
    return unwrap(
      await invoke("student_list_enrollments", { filter: filter ?? {} }),
    );
  }
  async getEnrollment(id: string): Promise<EnrollmentDto> {
    return unwrap(await invoke("student_get_enrollment", { id }));
  }
  async assignGuardian(req: AssignGuardianRequest): Promise<GuardianDto> {
    return unwrap(await invoke("student_assign_guardian", { request: req }));
  }
  async listGuardians(studentId: string): Promise<GuardianDto[]> {
    return unwrap(await invoke("student_list_guardians", { studentId }));
  }

  // ── Scheduling ──────────────────────────────────────────────
  async createRoom(req: CreateRoomRequest): Promise<RoomDto> {
    const payload = {
      name: req.name,
      capacity: req.capacity,
      roomType: req.room_type ?? (req as any).roomType ?? "classroom",
      room_type: req.room_type ?? (req as any).roomType ?? "classroom",
    };
    return unwrap(await invoke("scheduling_create_room", { request: payload }));
  }
  async listRooms(): Promise<RoomDto[]> {
    return unwrap(await invoke("scheduling_list_rooms"));
  }
  async getRoom(id: string): Promise<RoomDto> {
    return unwrap(await invoke("scheduling_get_room", { id }));
  }
  async createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto> {
    return unwrap(
      await invoke("scheduling_create_time_slot", { request: req }),
    );
  }
  async listTimeSlots(periodId: string): Promise<TimeSlotDto[]> {
    return unwrap(await invoke("scheduling_list_time_slots", { periodId }));
  }
  async scheduleLesson(
    req: ScheduleLessonRequest,
  ): Promise<ScheduledLessonDto> {
    return unwrap(await invoke("scheduling_schedule_lesson", { request: req }));
  }
  async listScheduledLessons(filter?: {
    classSubjectId?: string;
    periodId?: string;
  }): Promise<ScheduledLessonDto[]> {
    return unwrap(
      await invoke("scheduling_list_lessons", { filter: filter ?? {} }),
    );
  }

  // ── Assessment ──────────────────────────────────────────────
  async createAssessmentScheme(
    req: CreateSchemeRequest,
  ): Promise<AssessmentSchemeDto> {
    return unwrap(await invoke("assessment_create_scheme", { request: req }));
  }
  async updateAssessmentScheme(
    id: string,
    req: UpdateSchemeRequest,
  ): Promise<AssessmentSchemeDto> {
    return unwrap(
      await invoke("assessment_update_scheme", {
        request: {
          id,
          name: req.name,
          description: req.description,
          maxScore: req.max_score,
          formula: req.formula,
        },
      }),
    );
  }
  async deleteAssessmentScheme(id: string): Promise<void> {
    unwrap(await invoke("assessment_delete_scheme", { id }));
  }
  async listAssessmentSchemes(): Promise<AssessmentSchemeDto[]> {
    return unwrap(await invoke("assessment_list_schemes"));
  }
  async getAssessmentScheme(id: string): Promise<AssessmentSchemeDto> {
    return unwrap(await invoke("assessment_get_scheme", { id }));
  }
  async recordGrade(req: RecordGradeRequest): Promise<GradeDto> {
    return unwrap(await invoke("assessment_record_grade", { request: req }));
  }
  async listGrades(filter?: {
    studentId?: string;
    classSubjectId?: string;
  }): Promise<GradeDto[]> {
    return unwrap(
      await invoke("assessment_list_grades", { filter: filter ?? {} }),
    );
  }
  async calculateGrades(
    req: CalculateGradesRequest,
  ): Promise<CalculationResultDto> {
    return unwrap(
      await invoke("assessment_calculate_grades", { request: req }),
    );
  }
  async calculateGpa(
    req: CalculateGpaInput,
  ): Promise<CumulativeGpaRecordDto> {
    return unwrap(
      await invoke("assessment_calculate_gpa", { request: req }),
    );
  }
  async calculateRankings(
    req: CalculateRankingsRequest,
  ): Promise<RankingItemDto[]> {
    return unwrap(
      await invoke("assessment_calculate_rankings", { request: req }),
    );
  }
  async markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto> {
    return unwrap(await invoke("assessment_mark_attendance", { request: req }));
  }
  async listAttendance(
    classId: string,
    date: string,
  ): Promise<AttendanceDto[]> {
    return unwrap(
      await invoke("assessment_list_attendance", { classId, date }),
    );
  }

  // ── Student Cohort Progression ──────────────────────────────
  async promoteCohort(req: PromoteCohortInput): Promise<PromotionJobDto> {
    return unwrap(
      await invoke("progression_promote_cohort", { request: req }),
    );
  }
  async listPromotionJobs(): Promise<PromotionJobDto[]> {
    return unwrap(await invoke("progression_list_jobs"));
  }
  async getPromotionJob(id: string): Promise<PromotionJobDto> {
    return unwrap(await invoke("progression_get_job", { id }));
  }

  // ── Workflow & Academic Overrides ───────────────────────────
  async submitOverride(
    req: SubmitOverrideRequestInput,
  ): Promise<OverrideRequestDto> {
    return unwrap(
      await invoke("workflow_submit_override", { request: req }),
    );
  }
  async approveOverride(
    req: ApproveOverrideInput,
  ): Promise<OverrideRequestDto> {
    return unwrap(
      await invoke("workflow_approve_override", { request: req }),
    );
  }
  async rejectOverride(
    req: RejectOverrideInput,
  ): Promise<OverrideRequestDto> {
    return unwrap(
      await invoke("workflow_reject_override", { request: req }),
    );
  }
  async listOverrides(status?: string): Promise<OverrideRequestDto[]> {
    return unwrap(await invoke("workflow_list_overrides", { status }));
  }
  async getOverride(id: string): Promise<OverrideRequestDto> {
    return unwrap(await invoke("workflow_get_override", { id }));
  }

  // ── Exam ────────────────────────────────────────────────────
  async createExamSession(
    req: CreateExamSessionRequest,
  ): Promise<ExamSessionDto> {
    return unwrap(await invoke("exam_create_session", { request: req }));
  }
  async listExamSessions(periodId: string): Promise<ExamSessionDto[]> {
    return unwrap(await invoke("exam_list_sessions", { periodId }));
  }
  async getExamSession(id: string): Promise<ExamSessionDto> {
    return unwrap(await invoke("exam_get_session", { id }));
  }
  async scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto> {
    return unwrap(await invoke("exam_schedule_exam", { request: req }));
  }
  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return unwrap(await invoke("exam_list_schedules", { sessionId }));
  }
  async assignSeating(req: AssignSeatingRequest): Promise<SeatingDto> {
    return unwrap(await invoke("exam_assign_seating", { request: req }));
  }
  async assignSecret(req: AssignSecretRequest): Promise<SecretDto> {
    return unwrap(await invoke("exam_assign_secret", { request: req }));
  }
  async assignProctor(req: AssignProctorRequest): Promise<ProctorDto> {
    return unwrap(await invoke("exam_assign_proctor", { request: req }));
  }

  // ── Finance ─────────────────────────────────────────────────
  async createAccount(req: CreateAccountRequest): Promise<AccountDto> {
    return unwrap(await invoke("finance_create_account", { request: req }));
  }
  async listAccounts(): Promise<AccountDto[]> {
    return unwrap(await invoke("finance_list_accounts"));
  }
  async createFeeTemplate(
    req: CreateFeeTemplateRequest,
  ): Promise<FeeTemplateDto> {
    return unwrap(
      await invoke("finance_create_fee_template", { request: req }),
    );
  }
  async listFeeTemplates(): Promise<FeeTemplateDto[]> {
    return unwrap(await invoke("finance_list_fee_templates"));
  }
  async generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto> {
    return unwrap(await invoke("finance_generate_fee", { request: req }));
  }
  async listStudentFees(
    filter?: StudentFeeFilterParams,
  ): Promise<StudentFeeDto[]> {
    return unwrap(
      await invoke("finance_list_student_fees", { filter: filter ?? {} }),
    );
  }
  async processPayment(req: ProcessPaymentRequest): Promise<PaymentDto> {
    return unwrap(await invoke("finance_process_payment", { request: req }));
  }
  async listPayments(
    filter?: PaymentFilterParams,
  ): Promise<PaymentDto[]> {
    return unwrap(
      await invoke("finance_list_payments", { filter: filter ?? {} }),
    );
  }
  async listCurrencies(): Promise<CurrencyDto[]> {
    return unwrap(await invoke("finance_list_currencies"));
  }
  async listJournalEntries(): Promise<JournalEntryDto[]> {
    return unwrap(await invoke("finance_list_journal_entries"));
  }
  async getJournalEntry(id: string): Promise<JournalEntryDto> {
    return unwrap(await invoke("finance_get_journal_entry", { id }));
  }
  async listDiscountTemplates(): Promise<DiscountTemplateDto[]> {
    return unwrap(await invoke("finance_list_discount_templates"));
  }
  async createDiscountTemplate(
    req: CreateDiscountTemplateRequest,
  ): Promise<DiscountTemplateDto> {
    return unwrap(
      await invoke("finance_create_discount_template", { request: req }),
    );
  }

  // ── Document ────────────────────────────────────────────────
  async uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto> {
    return unwrap(await invoke("document_upload", { request: req }));
  }
  async listDocuments(): Promise<DocumentDto[]> {
    return unwrap(await invoke("document_list"));
  }
  async getDocument(id: string): Promise<DocumentDto> {
    return unwrap(await invoke("document_get", { id }));
  }
  async deleteDocument(id: string): Promise<void> {
    await invoke("document_delete", { id });
  }

  // ── Integration ─────────────────────────────────────────────
  async connectApp(req: ConnectAppRequest): Promise<IntegrationDto> {
    return unwrap(await invoke("integration_connect_app", { request: req }));
  }
  async listIntegrations(): Promise<IntegrationDto[]> {
    return unwrap(await invoke("integration_list"));
  }
  async getIntegrationStatus(
    providerName: string,
  ): Promise<IntegrationStatusResponse> {
    return unwrap(await invoke("integration_get_status", { providerName }));
  }
  async disconnectApp(providerName: string): Promise<boolean> {
    return unwrap(await invoke("integration_disconnect", { providerName }));
  }
  async toggleIntegration(
    providerName: string,
    isEnabled: boolean,
  ): Promise<boolean> {
    return unwrap(
      await invoke("integration_toggle", { providerName, isEnabled }),
    );
  }

  // ── Templates ───────────────────────────────────────────────
  async createTemplate(req: CreateTemplateRequest): Promise<TemplateDto> {
    return unwrap(await invoke("template_create", { request: req }));
  }
  async getTemplate(id: string): Promise<TemplateDto> {
    return unwrap(await invoke("template_get", { id }));
  }
  async listTemplates(filter?: {
    documentType?: string;
    entityType?: string;
    status?: string;
  }): Promise<TemplateDto[]> {
    return unwrap(await invoke("template_list", { filter: filter ?? {} }));
  }
  async updateTemplate(
    id: string,
    req: UpdateTemplateRequest,
  ): Promise<TemplateDto> {
    return unwrap(await invoke("template_update", { id, request: req }));
  }
  async deleteTemplate(id: string): Promise<void> {
    await invoke("template_delete", { id });
  }
  async publishTemplate(id: string): Promise<TemplateDto> {
    return unwrap(await invoke("template_publish", { id }));
  }
  async archiveTemplate(id: string): Promise<void> {
    await invoke("template_archive", { id });
  }
  async getTemplateVersions(id: string): Promise<TemplateVersionDto[]> {
    return unwrap(await invoke("template_get_versions", { id }));
  }
  async revertTemplateVersion(
    id: string,
    targetVersion: number,
  ): Promise<TemplateDto> {
    return unwrap(
      await invoke("template_revert_version", { id, version: targetVersion }),
    );
  }
  async previewTemplate(
    documentJson: string,
    contextData: any,
    locale?: string,
  ): Promise<Uint8Array> {
    const res = await invoke<number[]>("template_preview", {
      documentJson,
      contextData,
      locale,
    });
    return new Uint8Array(res);
  }
  async validateTemplate(documentJson: string): Promise<ValidationResult> {
    return unwrap(await invoke("template_validate", { documentJson }));
  }

  // ── Template Bindings ───────────────────────────────────────
  async listTemplateBindings(
    definitionId: string,
  ): Promise<TemplateBindingDto[]> {
    return unwrap(await invoke("template_list_bindings", { definitionId }));
  }
  async createTemplateBinding(
    definitionId: string,
    templateId: string,
    locale: string,
    format: string,
  ): Promise<TemplateBindingDto> {
    return unwrap(
      await invoke("template_create_binding", {
        definitionId,
        templateId,
        locale,
        format,
      }),
    );
  }

  // ── Report Definitions ──────────────────────────────────────
  async listReportDefinitions(
    domain?: string,
    category?: string,
  ): Promise<ReportDefinitionDto[]> {
    return unwrap(
      await invoke("report_list_definitions", { domain, category }),
    );
  }
  async getReportDefinition(code: string): Promise<ReportDefinitionDto> {
    return unwrap(await invoke("report_get_definition", { code }));
  }

  // ── Report Execution & Printing ─────────────────────────────
  async generateReport(
    templateId: string,
    format: string,
    parameters: Record<string, unknown>,
  ): Promise<string> {
    return unwrap(
      await invoke("generate_report", {
        templateId,
        format,
        parameters,
      }),
    );
  }
  async getReportJobStatus(jobId: string): Promise<ReportJobStatusDto> {
    return unwrap(await invoke("get_report_job_status", { jobId }));
  }
  async downloadReportFile(jobId: string): Promise<Uint8Array> {
    const bytes: number[] = unwrap(
      await invoke("download_report_file", { jobId }),
    );
    return new Uint8Array(bytes);
  }
  async batchPrint(req: BatchPrintParams): Promise<string> {
    return unwrap(await invoke("batch_print", { ...req }));
  }

  // ── System & Bootstrap ──────────────────────────────────────
  async systemHealth(): Promise<HealthResponse> {
    return invoke<HealthResponse>("system_health");
  }
  async systemHasOrganization(): Promise<boolean> {
    return invoke<boolean>("system_has_organization");
  }
  async systemGetReadiness(): Promise<OrganizationReadiness> {
    return invoke<OrganizationReadiness>("system_get_readiness");
  }
  async systemGetEntitlement(): Promise<EntitlementResult> {
    return invoke<EntitlementResult>("system_get_entitlement");
  }
  async bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult> {
    return invoke<BootstrapResult>("bootstrap_organization", req as unknown as Record<string, unknown>);
  }
  async bootstrapInitialSetup(req: InitialSetupRequest): Promise<OrganizationReadiness> {
    return invoke<OrganizationReadiness>("bootstrap_initial_setup", req as unknown as Record<string, unknown>);
  }

  // ── Connection (Desktop-only) ───────────────────────────────
  async connectionGetConfig(): Promise<ConnectionConfig> {
    return invoke<ConnectionConfig>("connection_get_config");
  }
  async connectionSetConfig(mode: ConnectionMode, serverUrl?: string, dbFilename?: string): Promise<ConnectionConfig> {
    return invoke<ConnectionConfig>("connection_set_config", {
      mode,
      serverUrl: serverUrl ?? null,
      dbFilename: dbFilename ?? null,
    });
  }
  async connectionValidate(): Promise<ConnectionValidation> {
    return invoke<ConnectionValidation>("connection_validate");
  }
  async connectionCheckRemoteHealth(serverUrl: string): Promise<RemoteHealthResult> {
    return invoke<RemoteHealthResult>("connection_check_remote_health", { serverUrl });
  }
  async connectionConnect(): Promise<ConnectResult> {
    return invoke<ConnectResult>("connection_connect");
  }
  async connectionOpenWindow(): Promise<void> {
    return invoke<void>("open_connection_window");
  }

  // ── AI Platform ─────────────────────────────────────────────
  async aiChat(request: import("./types").AiChatRequest): Promise<import("./types").AiChatResponse> {
    return unwrap(await invoke("ai_chat", { request }));
  }
  async aiListTools(): Promise<import("./types").AiToolDefinition[]> {
    return unwrap(await invoke("ai_list_tools"));
  }
  async aiSaveSession(session: import("./types").AiStoredSession): Promise<boolean> {
    return unwrap(await invoke("ai_save_session", { session }));
  }
  async aiLoadSessions(orgId: string): Promise<import("./types").AiStoredSession[]> {
    return unwrap(await invoke("ai_load_sessions", { orgId }));
  }
  async aiDeleteSession(orgId: string, sessionId: string): Promise<boolean> {
    return unwrap(await invoke("ai_delete_session", { orgId, sessionId }));
  }
  async aiSaveContext(orgId: string, context: Record<string, unknown>): Promise<boolean> {
    return unwrap(await invoke("ai_save_context", { orgId, context }));
  }
  async aiLoadContext(orgId: string): Promise<Record<string, unknown> | null> {
    return unwrap(await invoke("ai_load_context", { orgId }));
  }

  // ── Autopilot System ────────────────────────────────────────
  async autopilotListTasks(params?: {
    status?: string;
    task_type?: string;
    limit?: number;
    offset?: number;
  }): Promise<import("./types").ListTasksResponse> {
    return unwrap(
      await invoke("autopilot_list_tasks", {
        status: params?.status ?? null,
        taskType: params?.task_type ?? null,
        limit: params?.limit ?? null,
        offset: params?.offset ?? null,
      }),
    );
  }

  async autopilotRunTick(dryRun?: boolean): Promise<import("./types").AutopilotTaskDto[]> {
    return unwrap(
      await invoke("autopilot_run_tick", {
        dryRun: dryRun ?? false,
      }),
    );
  }

  async autopilotGetTask(id: string): Promise<import("./types").AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_get_task", { id }));
  }

  async autopilotExecuteTask(id: string): Promise<import("./types").AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_execute_task", { id }));
  }

  async autopilotRetryTask(id: string): Promise<import("./types").AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_retry_task", { id }));
  }

  async autopilotDismissTask(id: string): Promise<import("./types").AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_dismiss_task", { id }));
  }

  async autopilotGetDiagnostics(): Promise<import("./types").AutopilotDiagnosticsDto> {
    return unwrap(await invoke("autopilot_get_diagnostics"));
  }
}
