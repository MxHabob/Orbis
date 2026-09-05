/**
 * LocalClient — Tauri IPC transport for the OrbisApiClient interface.
 * 
 * Each method calls `invoke<T>('command_name', args)` and unwraps the
 * `ApiResponse<T>` envelope. Command names follow the convention:
 *   {module}_{action}_{resource}
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  OrbisApiClient,
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
} from './types';

function unwrap<T>(response: { data: T }): T {
  return response.data;
}

export class LocalClient implements OrbisApiClient {
  // ── Auth ────────────────────────────────────────────────────
  async login(req: LoginRequest): Promise<LoginResponse> {
    return unwrap(await invoke('auth_login', { request: req }));
  }
  async getSession(): Promise<SessionResponse> {
    return unwrap(await invoke('auth_get_session'));
  }
  async logout(): Promise<void> {
    await invoke('auth_logout');
  }

  // ── IAM ─────────────────────────────────────────────────────
  async registerUser(req: RegisterUserRequest): Promise<UserDto> {
    return unwrap(await invoke('iam_register_user', { request: req }));
  }
  async listUsers(): Promise<UserDto[]> {
    return unwrap(await invoke('iam_list_users'));
  }
  async getUser(id: string): Promise<UserDto> {
    return unwrap(await invoke('iam_get_user', { id }));
  }

  // ── Organization ────────────────────────────────────────────
  async getCurrentOrg(): Promise<OrgDto> {
    return unwrap(await invoke('org_get_current'));
  }
  async updateCurrentOrg(req: UpdateOrgRequest): Promise<OrgDto> {
    return unwrap(await invoke('org_update_current', { request: req }));
  }
  async getSettings(): Promise<OrganizationSettings> {
    return unwrap(await invoke('org_get_settings'));
  }
  async updateSettings(settings: OrganizationSettings): Promise<OrganizationSettings> {
    return unwrap(await invoke('org_update_settings', { settings }));
  }
  async getDashboard(): Promise<DashboardCounts> {
    return unwrap(await invoke('org_get_dashboard'));
  }

  // ── OrgUnits ────────────────────────────────────────────────
  async createOrgUnit(req: CreateOrgUnitRequest): Promise<OrgUnitDto> {
    return unwrap(await invoke('org_create_unit', { request: req }));
  }
  async listOrgUnits(): Promise<OrgUnitDto[]> {
    return unwrap(await invoke('org_list_units'));
  }
  async getOrgUnit(id: string): Promise<OrgUnitDto> {
    return unwrap(await invoke('org_get_unit', { id }));
  }
  async deleteOrgUnit(id: string): Promise<void> {
    await invoke('org_delete_unit', { id });
  }

  // ── Calendar ────────────────────────────────────────────────
  async listAcademicYears(): Promise<AcademicYearDto[]> {
    return unwrap(await invoke('calendar_list_years'));
  }
  async getAcademicContext(): Promise<AcademicContextResponse> {
    return unwrap(await invoke('calendar_get_context'));
  }
  async getAcademicYear(id: string): Promise<AcademicYearDto> {
    return unwrap(await invoke('calendar_get_year', { id }));
  }
  async listAcademicPeriods(): Promise<AcademicPeriodDto[]> {
    return unwrap(await invoke('calendar_list_periods'));
  }
  async getAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return unwrap(await invoke('calendar_get_period', { id }));
  }
  async listPeriodTemplates(): Promise<PeriodTemplateDto[]> {
    return unwrap(await invoke('calendar_list_period_templates'));
  }

  // ── Academic ────────────────────────────────────────────────
  async listSubjects(): Promise<SubjectDto[]> {
    return unwrap(await invoke('academic_list_subjects'));
  }
  async getSubject(id: string): Promise<SubjectDto> {
    return unwrap(await invoke('academic_get_subject', { id }));
  }
  async listClasses(): Promise<ClassDto[]> {
    return unwrap(await invoke('academic_list_classes'));
  }
  async getClass(id: string): Promise<ClassDto> {
    return unwrap(await invoke('academic_get_class', { id }));
  }
  async listClassSubjects(classId: string, periodId: string): Promise<ClassSubjectDto[]> {
    return unwrap(await invoke('academic_list_class_subjects', { classId, periodId }));
  }

  // ── Student ─────────────────────────────────────────────────
  async listEnrollments(filter?: { classId?: string; studentId?: string }): Promise<EnrollmentDto[]> {
    return unwrap(await invoke('student_list_enrollments', { filter: filter ?? {} }));
  }
  async getEnrollment(id: string): Promise<EnrollmentDto> {
    return unwrap(await invoke('student_get_enrollment', { id }));
  }
  async listGuardians(studentId: string): Promise<GuardianDto[]> {
    return unwrap(await invoke('student_list_guardians', { studentId }));
  }

  // ── Scheduling ──────────────────────────────────────────────
  async listRooms(): Promise<RoomDto[]> {
    return unwrap(await invoke('scheduling_list_rooms'));
  }
  async getRoom(id: string): Promise<RoomDto> {
    return unwrap(await invoke('scheduling_get_room', { id }));
  }
  async listTimeSlots(periodId: string): Promise<TimeSlotDto[]> {
    return unwrap(await invoke('scheduling_list_time_slots', { periodId }));
  }
  async listScheduledLessons(filter?: { classSubjectId?: string; periodId?: string }): Promise<ScheduledLessonDto[]> {
    return unwrap(await invoke('scheduling_list_lessons', { filter: filter ?? {} }));
  }

  // ── Assessment ──────────────────────────────────────────────
  async listAssessmentSchemes(): Promise<AssessmentSchemeDto[]> {
    return unwrap(await invoke('assessment_list_schemes'));
  }
  async getAssessmentScheme(id: string): Promise<AssessmentSchemeDto> {
    return unwrap(await invoke('assessment_get_scheme', { id }));
  }
  async listGrades(filter?: { studentId?: string; classSubjectId?: string }): Promise<GradeDto[]> {
    return unwrap(await invoke('assessment_list_grades', { filter: filter ?? {} }));
  }
  async listAttendance(classId: string, date: string): Promise<AttendanceDto[]> {
    return unwrap(await invoke('assessment_list_attendance', { classId, date }));
  }

  // ── Exam ────────────────────────────────────────────────────
  async listExamSessions(periodId: string): Promise<ExamSessionDto[]> {
    return unwrap(await invoke('exam_list_sessions', { periodId }));
  }
  async getExamSession(id: string): Promise<ExamSessionDto> {
    return unwrap(await invoke('exam_get_session', { id }));
  }
  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return unwrap(await invoke('exam_list_schedules', { sessionId }));
  }

  // ── Finance ─────────────────────────────────────────────────
  async listAccounts(): Promise<AccountDto[]> {
    return unwrap(await invoke('finance_list_accounts'));
  }
  async listFeeTemplates(): Promise<FeeTemplateDto[]> {
    return unwrap(await invoke('finance_list_fee_templates'));
  }
  async listStudentFees(filter?: { enrollmentId?: string; studentId?: string }): Promise<StudentFeeDto[]> {
    return unwrap(await invoke('finance_list_student_fees', { filter: filter ?? {} }));
  }
  async listPayments(filter?: { feeId?: string }): Promise<PaymentDto[]> {
    return unwrap(await invoke('finance_list_payments', { filter: filter ?? {} }));
  }
  async listCurrencies(): Promise<CurrencyDto[]> {
    return unwrap(await invoke('finance_list_currencies'));
  }

  // ── Document ────────────────────────────────────────────────
  async listDocuments(): Promise<DocumentDto[]> {
    return unwrap(await invoke('document_list'));
  }
  async getDocument(id: string): Promise<DocumentDto> {
    return unwrap(await invoke('document_get', { id }));
  }
  async deleteDocument(id: string): Promise<void> {
    await invoke('document_delete', { id });
  }

  // ── Integration ─────────────────────────────────────────────
  async listIntegrations(): Promise<IntegrationDto[]> {
    return unwrap(await invoke('integration_list'));
  }

  // ── Templates ───────────────────────────────────────────────
  async createTemplate(req: CreateTemplateRequest): Promise<TemplateDto> {
    return unwrap(await invoke('template_create', { input: req }));
  }
  async getTemplate(id: string): Promise<TemplateDto> {
    return unwrap(await invoke('template_get', { id }));
  }
  async listTemplates(filter?: { documentType?: string; entityType?: string; status?: string }): Promise<TemplateDto[]> {
    return unwrap(await invoke('template_list', { filter: filter ?? null }));
  }
  async updateTemplate(id: string, req: UpdateTemplateRequest): Promise<TemplateDto> {
    return unwrap(await invoke('template_update', { id, input: req }));
  }
  async deleteTemplate(id: string): Promise<void> {
    await invoke('template_delete', { id });
  }
  async publishTemplate(id: string): Promise<TemplateDto> {
    return unwrap(await invoke('template_publish', { id }));
  }
  async archiveTemplate(id: string): Promise<void> {
    await invoke('template_archive', { id });
  }
  async getTemplateVersions(id: string): Promise<TemplateVersionDto[]> {
    return unwrap(await invoke('template_get_versions', { id }));
  }
  async revertTemplateVersion(id: string, targetVersion: number): Promise<TemplateDto> {
    return unwrap(await invoke('template_revert_version', { id, targetVersion }));
  }
  async previewTemplate(documentJson: string, contextData: any, locale?: string): Promise<Uint8Array> {
    const bytes = await invoke<number[]>('template_preview', {
      documentJson,
      contextData,
      locale: locale ?? null,
      pageIndex: null,
    });
    return new Uint8Array(bytes);
  }
  async validateTemplate(documentJson: string): Promise<ValidationResult> {
    return unwrap(await invoke('template_validate', { documentJson }));
  }

  // ── Template Bindings ───────────────────────────────────────
  async listTemplateBindings(definitionId: string): Promise<TemplateBindingDto[]> {
    return unwrap(await invoke('template_list_bindings', { definitionId }));
  }
  async createTemplateBinding(definitionId: string, templateId: string, locale: string, format: string): Promise<TemplateBindingDto> {
    return unwrap(await invoke('template_create_binding', { definitionId, templateId, locale, format }));
  }

  // ── Report Definitions ──────────────────────────────────────
  async listReportDefinitions(domain?: string, category?: string): Promise<ReportDefinitionDto[]> {
    return unwrap(await invoke('report_list_definitions', { domain: domain ?? null, category: category ?? null }));
  }
  async getReportDefinition(code: string): Promise<ReportDefinitionDto> {
    return unwrap(await invoke('report_get_definition', { code }));
  }
}
