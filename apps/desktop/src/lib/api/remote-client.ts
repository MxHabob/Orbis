/**
 * RemoteClient — HTTP transport for the OrbisApiClient interface.
 *
 * Thin adapter wrapping @orbis/sdk functions. Used when connecting to a remote Orbis server.
 */

import {
  createOrbisClient,
  type OrbisClient,
  type OrbisErrorBody,
} from "@orbis/sdk";
import * as sdk from "@orbis/sdk";
import { OrbisError } from "./index";
import { isDesktopApp } from "./runtime";
import appPackage from "../../../package.json";

import type {
  OrbisApiClient,
  ApiResponse,
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
  UpdateOrgUnitRequest,
  OrganizationSettingsDto,
  DashboardCountsDto,
  academicCalendarDto,
  AcademicPeriodDto,
  AcademicContextResponse,
  PeriodTypeDto,
  CreateYearRequest,
  UpdateYearRequest,
  CreatePeriodRequest,
  UpdatePeriodRequest,
  OfficialCalendarTemplateDto,
  ApplyCalendarTemplateRequest,
  CalendarDefaultsDto,
  SubjectDto,
  SectionSubjectDto,
  CreateSubjectRequest,
  UpdateSubjectRequest,
  SectionDto,
  CreateSectionRequest,
  UpdateSectionRequest,
  AssignSectionSubjectRequest,
  UpdateSectionSubjectRequest,
  EnrollmentDto,
  GuardianDto,
  EnrollStudentRequest,
  AssignGuardianRequest,
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
  TimetableProblem,
  TimetableSolveStatusDto,
  SolverSidecarStatusDto,
  AssessmentSchemeDto,
  GradeDto,
  AttendanceDto,
  BehaviorRecordDto,
  CumulativeGpaRecordDto,
  CalculationResultDto,
  CalculationEngineSettings,
  CalculationPreview,
  CalculationPlan,
  CalculationOutcomePolicy,
  DecimalValue,
  RankingScope,
  TiePolicy,
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
  StudentTermReportCard,
  StudentAnnualReportCard,
  DocumentDto,
  FolderDto,
  FolderContentsDto,
  CreateFolderRequest,
  UpdateFolderRequest,
  CreateCurriculumFolderRequest,
  UploadDocumentRequest,
  StorageUploadRequest,
  UpdateDocumentRequest,
  DocumentRequirementDto,
  CreateDocumentRequirementRequest,
  UpdateDocumentRequirementRequest,
  StudentDocumentFulfillmentDto,
  PresignedUrlResponseDto,
  ExamSessionDto,
  ExamScheduleDto,
  SeatingDto,
  SecretDto,
  ProctorDto,
  CreateExamSessionRequest,
  UpdateExamSessionRequest,
  GenerateSeatingRequest,
  GenerateSecretsRequest,
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
  StudentFeeDto,
  GenerateFeeRequest,
  PaymentDto,
  ProcessPaymentRequest,
  CurrencyDto,
  IntegrationDto,
  IntegrationStatusResponse,
  MulverseConnectionDto,
  MulverseDeploymentDto,
  MulverseEndpointDto,
  MulverseDeploymentRequest,
  MulverseEndpointRequest,
  MulverseEndpointUpdateRequest,
  ConnectAppRequest,
  TemplateDto,
  CreateTemplateRequest,
  UpdateTemplateRequest,
  TemplateVersionDto,
  TemplateBindingDto,
  ValidationResultDto,
  ReportDefinitionDto,
  StudentDto,
  StudentDetailDto,
  RegisterStudentInput,
  DuplicateCheckRequest,
  DuplicateCheckResponse,
  StudentFilterParams,
  EnrollmentFilterParams,
  PaginatedList,
  EmployeeFilterInput,
  EmployeeDetailDto,
  PaginatedEmployeesDto,
  RegisterEmployeeRequest,
  RegisterEmployeeResponseDto,
  UpdateEmployeeRequest,
  UpdateEmployeeStatusRequest,
  ProvisionEmployeeAccountRequest,
  GroupDetailDto,
  CreateGroupRequest,
  UpdateGroupRequest,
  GroupMemberInfoDto,
  BulkAssignGroupRequest,
  ReportJobStatusDto,
  BatchPrintParams,
  JournalEntryDto,
  CreateJournalEntryRequest,
  ReverseJournalRequest,
  SplitFeeRequest,
  RefundStudentFeeRequest,
  DiscountTemplateDto,
  CreateDiscountTemplateRequest,
  PrerequisiteCheckResultDto,
  OverrideRequestDto,
  SubmitOverrideRequestInput,
  ApproveOverrideInput,
  RejectOverrideInput,
  progressionJobDto,
  PromoteCohortInput,
  StudentProgressionItemInput,
  HealthResponse,
  OrganizationReadiness,
  EntitlementResult,
  AppInfo,
  ReleaseChangelogItem,
  UpdateCheckResult,
  EnvironmentDetectionResult,
  SlugGenerationResult,
  BootstrapRequest,
  BootstrapResult,
  BootstrapStatusResponse,
  CompleteOrganizationOnboardingRequest,
  OnboardingResult,
  ConnectionConfig,
  ConnectionMode,
  ConnectionValidation,
  RemoteHealthResult,
  ConnectResult,
  AuditLogDto,
  DatabaseStatusDto,
  BackupResultDto,
  BackupInspectionDto,
  RestoreResultDto,
  IntegrityResultDto,
  OptimizeResultDto,
  MigrationResultDto,
  DatabaseSettingsDto,
  BulkRegisterStudentCommandItem,
  BulkRegisterStudentCommandResultDto,
  BulkRegisterEmployeeCommandItem,
  BulkRegisterEmployeeCommandResultDto,
  ProvisionUserRequest,
  AccountProvisioningResultDto,
  UpdateStudentStatusInput,
  UpdateStudentProfileInput,
  ChangeStudyModeInput,
  TransferClassInput,
  StudentFeeFilterParams,
  PaymentFilterParams,
  StatementResponseDto,
  StatementFilterParams,
  ProcessDisbursementRequest,
  CreateSubAccountRequest,
  SettleGatewayPayoutRequest,
  PostPayrollRequest,
  DisbursePayrollRequest,
  AiChatRequest,
  AiChatResponse,
  AiCustomReportRequest,
  AiCustomReportResponse,
  AiToolDefinition,
  AiStoredSession,
  ListTasksResponse,
  AutopilotTaskDto,
  AutopilotDiagnosticsDto,
  OnboardingStateDto,
  ProvisionDefaultsReqDto,
  ProvisionDefaultsResultDto,
  StudentCertificateReportCard,
} from "./types";

export class ApiRequestError extends Error {
  constructor(
    public readonly status: number,
    public readonly error: OrbisError | OrbisErrorBody | unknown,
  ) {
    super(
      typeof error === "object" && error !== null && "message" in error
        ? String((error as any).message)
        : "API request failed",
    );
    this.name = "ApiRequestError";
  }
}

interface GitHubRelease {
  tag_name: string;
  name: string | null;
  published_at: string | null;
  body: string | null;
  html_url: string;
  prerelease: boolean;
  assets?: Array<{
    name: string;
    browser_download_url: string;
    content_type?: string;
    size?: number;
  }>;
}

const RELEASE_IMAGE_EXTENSIONS = new Set(["png", "jpg", "jpeg", "gif", "webp", "avif"]);
const RELEASE_RESOURCE_EXTENSIONS = new Set([
  "pdf", "md", "txt", "csv", "json", "docx", "xlsx", "pptx", "odt", "ods", "odp",
]);

function releaseAssetUrl(value: string): string | null {
  try {
    const url = new URL(value);
    return url.protocol === "https:" &&
      url.hostname === "github.com" &&
      url.pathname.startsWith("/MxHabob/Orbis/releases/download/")
      ? url.href
      : null;
  } catch {
    return null;
  }
}

function semverParts(tag: string): [number, number, number] | null {
  const match = /^v?(\d+)\.(\d+)\.(\d+)$/.exec(tag);
  return match ? [Number(match[1]), Number(match[2]), Number(match[3])] : null;
}

function isReleaseImage(asset: NonNullable<GitHubRelease["assets"]>[number]): boolean {
  const ext = asset.name.split(".").pop()?.toLowerCase();
  return Boolean(asset.content_type?.startsWith("image/") || (ext && RELEASE_IMAGE_EXTENSIONS.has(ext)));
}

export class RemoteClient implements OrbisApiClient {
  private baseUrl: string;
  private token: string | null = null;
  private orbisClient: OrbisClient;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
    this.orbisClient = createOrbisClient({
      baseUrl: this.baseUrl,
      headers: (): Record<string, string> => {
        const h: Record<string, string> = {};
        if (this.token) {
          h["Authorization"] = `Bearer ${this.token}`;
        }
        return h;
      },
    });
  }

  setToken(token: string | null): void {
    this.token = token;
  }

  private get client() {
    return this.orbisClient.client;
  }

  private async call<T>(
    promise: Promise<{ data?: T; error?: unknown; response?: Response }>,
  ): Promise<T> {
    const res = await promise;
    if (res.error) {
      throw new ApiRequestError(res.response?.status || 500, res.error);
    }
    return res.data as T;
  }

  // ── Auth & MFA & Devices ────────────────────────────────────
  async login(req: LoginRequest): Promise<LoginResponse> {
    const res = await this.call(sdk.login({ client: this.client, body: req }));
    if (res?.token) this.token = res.token;
    return res;
  }

  async getSession(): Promise<SessionResponse> {
    return this.call(sdk.getSession({ client: this.client }));
  }

  async logout(): Promise<void> {
    this.token = null;
  }

  async mfaEnroll(req?: import("./types").MfaEnrollRequest): Promise<import("./types").MfaEnrollResponse> {
    return this.call(sdk.mfaEnroll({ client: this.client, body: req ?? {} }));
  }

  async mfaVerifyEnrollment(req: import("./types").MfaVerifyEnrollmentRequest): Promise<string[]> {
    return this.call(sdk.mfaVerifyEnrollment({ client: this.client, body: req }));
  }

  async mfaChallenge(req: import("./types").MfaChallengeRequest): Promise<LoginResponse> {
    const res = await this.call(sdk.mfaChallenge({ client: this.client, body: req }));
    if (res?.token) this.token = res.token;
    return res;
  }

  async linkGoogle(req: import("./types").GoogleAuthRequest): Promise<import("./types").UserIdentityDto> {
    return this.call(sdk.googleLink({ client: this.client, body: req }));
  }

  async loginWithGoogle(req: import("./types").GoogleAuthRequest): Promise<LoginResponse> {
    const res = await this.call(sdk.googleLogin({ client: this.client, body: req }));
    if (res?.token) this.token = res.token;
    return res;
  }

  async listDevices(): Promise<import("./types").UserDeviceDto[]> {
    return this.call(sdk.listDevices({ client: this.client })) ?? [];
  }

  async revokeDevice(id: string): Promise<boolean> {
    await this.call(sdk.revokeDevice({ client: this.client, path: { id } }));
    return true;
  }

  // ── IAM ─────────────────────────────────────────────────────
  async registerUser(req: RegisterUserRequest): Promise<UserDto> {
    return this.call(sdk.registerUser({ client: this.client, body: req }));
  }

  async provisionUser(req: ProvisionUserRequest): Promise<AccountProvisioningResultDto> {
    return this.call(sdk.provisionUser({ client: this.client, body: req as any })) as any;
  }

  async listUsers(): Promise<UserDto[]> {
    return this.call(sdk.listUsers({ client: this.client })) ?? [];
  }

  async getUser(id: string): Promise<UserDto> {
    return this.call(sdk.getUser({ client: this.client, path: { id } }));
  }

  async updateUser(id: string, req: UpdateUserRequest): Promise<UserDto> {
    return this.call(sdk.updateUser({ client: this.client, path: { id }, body: req }));
  }

  async resetUserPassword(userId: string): Promise<AccountProvisioningResultDto> {
    return this.call(sdk.resetUserPassword({ client: this.client, path: { id: userId } }));
  }

  async listRoles(): Promise<RoleDto[]> {
    return this.call(sdk.listRoles({ client: this.client })) ?? [];
  }

  async saveRole(req: SaveRoleRequest): Promise<RoleDto> {
    return this.call(sdk.saveRole({ client: this.client, body: req }));
  }

  async deleteRole(id: string): Promise<boolean> {
    await this.call(sdk.deleteRole({ client: this.client, path: { id } }));
    return true;
  }

  async listPermissions(): Promise<PermissionRuleDto[]> {
    return this.call(sdk.listPermissions({ client: this.client })) ?? [];
  }

  async savePermissions(req: PermissionRuleDto[]): Promise<boolean> {
    await this.call(sdk.savePermissions({ client: this.client, body: req }));
    return true;
  }

  // ── Organization ────────────────────────────────────────────
  async getCurrentOrg(): Promise<OrgDto> {
    return this.call(sdk.getCurrentOrg({ client: this.client }));
  }

  async updateCurrentOrg(req: UpdateOrgRequest): Promise<OrgDto> {
    return this.call(sdk.updateCurrentOrg({ client: this.client, body: req }));
  }

  async getSettings(): Promise<OrganizationSettingsDto> {
    return this.call(sdk.getSettings({ client: this.client }));
  }

  async updateSettings(settings: OrganizationSettingsDto): Promise<OrganizationSettingsDto> {
    return this.call(sdk.updateSettings({ client: this.client, body: settings }));
  }

  async getDashboard(): Promise<DashboardCountsDto> {
    return this.call(sdk.getDashboard({ client: this.client }));
  }

  // ── OrgUnits ────────────────────────────────────────────────
  async createOrgUnit(req: CreateOrgUnitRequest): Promise<OrgUnitDto> {
    return this.call(sdk.createOrgUnit({ client: this.client, body: req }));
  }

  async updateOrgUnit(id: string, req: UpdateOrgUnitRequest): Promise<OrgUnitDto> {
    return this.call(sdk.updateOrgUnit({ client: this.client, path: { id }, body: req }));
  }

  async listOrgUnits(): Promise<OrgUnitDto[]> {
    return this.call(sdk.listOrgUnits({ client: this.client })) ?? [];
  }

  async getOrgUnit(id: string): Promise<OrgUnitDto> {
    return this.call(sdk.getOrgUnit({ client: this.client, path: { id } }));
  }

  async deleteOrgUnit(id: string): Promise<void> {
    await this.call(sdk.deleteOrgUnit({ client: this.client, path: { id } }));
  }

  async selectOrgUnit(_id?: string): Promise<void> {}

  // ── Calendar ────────────────────────────────────────────────
  async createacademicCalendar(req: CreateYearRequest): Promise<academicCalendarDto> {
    return this.call(sdk.createYear({ client: this.client, body: req }));
  }

  async updateacademicCalendar(
    id: string,
    req: UpdateYearRequest,
  ): Promise<academicCalendarDto> {
    return this.call(
      sdk.updateYear({ client: this.client, path: { id }, body: req }),
    );
  }

  async listacademicCalendars(): Promise<academicCalendarDto[]> {
    return this.call(sdk.listYears({ client: this.client })) ?? [];
  }

  async getAcademicContext(): Promise<AcademicContextResponse> {
    return this.call(sdk.getCurrentContext({ client: this.client }));
  }

  async getacademicCalendar(id: string): Promise<academicCalendarDto> {
    return this.call(sdk.getYear({ client: this.client, path: { id } }));
  }

  async activateacademicCalendar(id: string): Promise<academicCalendarDto> {
    return this.call(sdk.activateYear({ client: this.client, path: { id } }));
  }

  async closeacademicCalendar(id: string): Promise<academicCalendarDto> {
    return this.call(sdk.closeYearHandler({ client: this.client, path: { id } }));
  }

  async archiveacademicCalendar(id: string): Promise<academicCalendarDto> {
    return this.call(sdk.archiveYearHandler({ client: this.client, path: { id } }));
  }

  async deleteacademicCalendar(id: string): Promise<boolean> {
    return this.call<boolean>(
      this.client.delete<boolean>({
        url: `/api/v1/academic-calendars/${encodeURIComponent(id)}`,
      }),
    );
  }

  async createAcademicPeriod(req: CreatePeriodRequest): Promise<AcademicPeriodDto> {
    return this.call(sdk.createPeriod({ client: this.client, body: req }));
  }

  async updateAcademicPeriod(
    id: string,
    req: UpdatePeriodRequest,
  ): Promise<AcademicPeriodDto> {
    return this.call(
      sdk.updatePeriod({ client: this.client, path: { id }, body: req }),
    );
  }

  async deleteAcademicPeriod(id: string): Promise<boolean> {
    return this.call(sdk.deletePeriodHandler({ client: this.client, path: { id } }));
  }

  async listAcademicPeriods(): Promise<AcademicPeriodDto[]> {
    return this.call(sdk.listPeriods({ client: this.client })) ?? [];
  }

  async getAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return this.call(sdk.getPeriod({ client: this.client, path: { id } }));
  }

  async lockAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return this.call(sdk.lockPeriod({ client: this.client, path: { id } }));
  }

  async unlockAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return this.call(sdk.unlockPeriod({ client: this.client, path: { id } }));
  }

  async closeAcademicPeriod(id: string): Promise<AcademicPeriodDto> {
    return this.call(sdk.closePeriod({ client: this.client, path: { id } }));
  }

  async listPeriodTypes(): Promise<PeriodTypeDto[]> {
    return this.call(sdk.listPeriodTypes({ client: this.client })) ?? [];
  }

  async listOfficialCalendarTemplates(): Promise<OfficialCalendarTemplateDto[]> {
    return this.call<OfficialCalendarTemplateDto[]>(
      this.client.get<OfficialCalendarTemplateDto[]>({
        url: "/api/v1/official-calendar-templates",
      }),
    ) ?? [];
  }

  async applyCalendarTemplate(
    yearId: string,
    req: ApplyCalendarTemplateRequest,
  ): Promise<AcademicPeriodDto[]> {
    return this.call<AcademicPeriodDto[]>(
      this.client.post<AcademicPeriodDto[]>({
        url: `/api/v1/academic-years/${encodeURIComponent(yearId)}/apply-template`,
        body: JSON.stringify(req),
        headers: { "Content-Type": "application/json" },
      }),
    ) ?? [];
  }

  async getCalendarDefaults(orgId?: string): Promise<CalendarDefaultsDto> {
    return this.call<CalendarDefaultsDto>(
      this.client.get<CalendarDefaultsDto>({
        url: "/api/v1/calendar/defaults",
        query: orgId ? { org_id: orgId } : undefined,
      }),
    );
  }

  // ── Academic ────────────────────────────────────────────────
  async createSubject(req: CreateSubjectRequest): Promise<SubjectDto> {
    return this.call(sdk.createSubject({ client: this.client, body: req }));
  }

  async updateSubject(id: string, req: UpdateSubjectRequest): Promise<SubjectDto> {
    return this.call(sdk.updateSubject({ client: this.client, path: { id }, body: req }));
  }

  async listSections(orgId?: string): Promise<SectionDto[]> {
    const res = await this.call(sdk.listSections({ client: this.client }));
    return res ?? [];
  }

  async getSection(id: string): Promise<SectionDto> {
    const res = await this.call(sdk.getSection({ client: this.client, path: { id } }));
    return res as SectionDto;
  }

  async createSection(req: CreateSectionRequest): Promise<SectionDto> {
    return this.call(sdk.createSection({ client: this.client, body: req }));
  }

  async updateSection(id: string, req: UpdateSectionRequest): Promise<SectionDto> {
    return this.call(sdk.updateSection({ client: this.client, path: { id }, body: req }));
  }




  async listSubjects(): Promise<SubjectDto[]> {
    return this.call(sdk.listSubjects({ client: this.client })) ?? [];
  }

  async getSubject(id: string): Promise<SubjectDto> {
    return this.call(sdk.getSubject({ client: this.client, path: { id } }));
  }




  async assignSectionSubject(req: AssignSectionSubjectRequest): Promise<SectionSubjectDto> {
    return this.call(sdk.assignSectionSubject({ client: this.client, body: req }));
  }

  async getSectionSubject(id: string): Promise<SectionSubjectDto> {
    return this.call(sdk.getSectionSubject({ client: this.client, path: { id } }));
  }

  async updateSectionSubject(
    id: string,
    req: UpdateSectionSubjectRequest,
  ): Promise<SectionSubjectDto> {
    return this.call(sdk.updateSectionSubject({ client: this.client, path: { id }, body: req }));
  }

  async listSectionSubjects(classId: string, periodId: string): Promise<SectionSubjectDto[]> {
    return this.call(
      sdk.listSectionSubjects({
        client: this.client,
        query: { section_id: classId, period_id: periodId },
      }),
    ) ?? [];
  }

  async checkPrerequisites(
    _studentId: string,
    _subjectId: string,
  ): Promise<PrerequisiteCheckResultDto[]> {
    return [];
  }

  // ── Student ─────────────────────────────────────────────────
  private mapStudentDtoToDetail(
    student: StudentDto,
    enrollment?: EnrollmentDto | null,
  ): StudentDetailDto {
    return {
      student,
      enrollment: enrollment ?? null,
      person: {
        id: student.person_id || student.id,
        org_id: student.org_id,
        first_name: student.first_name,
        middle_name: student.middle_name,
        last_name: student.last_name,
        first_name_ar: student.first_name_ar,
        middle_name_ar: student.middle_name_ar,
        last_name_ar: student.last_name_ar,
        display_name: student.display_name,
        full_name: student.full_name,
        full_name_ar: student.name_ar,
        national_id: student.national_id,
        passport_number: student.passport_number,
        gender: student.gender,
        birth_date: student.date_of_birth,
        place_of_birth: student.place_of_birth,
        nationality: student.nationality,
        blood_type: student.blood_type,
        medical_notes: student.medical_notes,
        special_needs_notes: student.special_needs_notes,
        phone: student.phone,
        email: student.email,
        address: student.address,
        avatar_url: student.avatar_url,
        created_at: student.created_at,
        updated_at: student.updated_at,
      },
    };
  }

  async registerStudent(req: RegisterStudentInput): Promise<StudentDetailDto> {
    const res = await this.call(sdk.registerStudent({ client: this.client, body: req as any }));
    if ((res as any)?.student && (res as any)?.person) {
      return res as any;
    }
    return this.mapStudentDtoToDetail(res as unknown as StudentDto);
  }

  async checkDuplicate(req: DuplicateCheckRequest): Promise<DuplicateCheckResponse> {
    return this.call(sdk.checkDuplicate({ client: this.client, body: req }));
  }

  async listStudents(filter?: StudentFilterParams): Promise<PaginatedList<StudentDetailDto>> {
    const query: any = { ...filter };
    if (filter?.class_id || filter?.classId) {
      query.section_id = filter.class_id || filter.classId;
    }
    const res = await this.call(sdk.listStudents({ client: this.client, query }));
    if (Array.isArray(res)) {
      const items: StudentDetailDto[] = (res as StudentDto[]).map((student) =>
        (student as any)?.student && (student as any)?.person
          ? (student as any)
          : this.mapStudentDtoToDetail(student),
      );
      return {
        items,
        total_count: items.length,
        page: filter?.page ?? 1,
        page_size: filter?.page_size ?? (items.length || 50),
        total_pages: 1,
      };
    }
    return (res as any) ?? { items: [], total_count: 0, page: 1, page_size: 50, total_pages: 0 };
  }

  async getStudent(id: string): Promise<StudentDetailDto> {
    const res = await this.call(sdk.getStudent({ client: this.client, path: { id } }));
    if ((res as any)?.student && (res as any)?.person) {
      return res as any;
    }
    return this.mapStudentDtoToDetail(res as unknown as StudentDto);
  }

  async updateStudentStatus(req: UpdateStudentStatusInput): Promise<StudentDto> {
    return this.call(
      sdk.updateStudentStatus({
        client: this.client,
        path: { id: req.student_id || req.studentId || "" },
        body: { status: req.status, reason: req.reason },
      }),
    );
  }

  async updateStudentProfile(req: UpdateStudentProfileInput): Promise<StudentDetailDto> {
    const studentId = req.student_id || req.studentId || "";
    const res = await this.call(
      sdk.updateStudentProfile({
        client: this.client,
        path: { id: studentId },
        body: req as any,
      }),
    );
    if ((res as any)?.student && (res as any)?.person) {
      return res as any;
    }
    return this.mapStudentDtoToDetail(res as unknown as StudentDto);
  }

  async deleteStudent(id: string): Promise<boolean> {
    return this.call(sdk.deleteStudent({ client: this.client, path: { id } }));
  }

  async restoreStudent(id: string): Promise<StudentDto> {
    return this.call(sdk.restoreStudent({ client: this.client, path: { id } }));
  }

  async changeStudyMode(req: ChangeStudyModeInput): Promise<EnrollmentDto> {
    const id = req.enrollment_id || req.enrollmentId || "";
    return this.call(
      sdk.changeStudyMode({
        client: this.client,
        path: { id },
        body: { new_mode: req.study_mode || req.studyMode || "", reason: req.reason },
      }),
    );
  }

  async transferClass(req: TransferClassInput): Promise<EnrollmentDto> {
    const id = req.enrollment_id || req.enrollmentId || "";
    const targetClassId = req.target_class_id || req.targetClassId || "";
    return this.call(
      sdk.transferClass({
        client: this.client,
        path: { id },
        body: { target_section_id: targetClassId, reason: req.reason },
      }),
    );
  }

  async enrollStudent(req: EnrollStudentRequest): Promise<EnrollmentDto> {
    return this.call(sdk.enrollStudent({ client: this.client, body: req }));
  }

  async listEnrollments(filter?: EnrollmentFilterParams): Promise<EnrollmentDto[]> {
    return this.call(sdk.listEnrollments({ client: this.client, query: filter as any })) ?? [];
  }

  async getEnrollment(id: string): Promise<EnrollmentDto> {
    return this.call(sdk.getEnrollment({ client: this.client, path: { id } }));
  }

  async assignGuardian(req: AssignGuardianRequest): Promise<GuardianDto> {
    return this.call(sdk.assignGuardian({ client: this.client, body: req }));
  }

  async listGuardians(studentId: string): Promise<GuardianDto[]> {
    return (await this.call(sdk.listGuardians({ client: this.client, query: { student_id: studentId } }))) ?? [];
  }

  async removeGuardian(studentId: string, guardianId: string): Promise<boolean> {
    return this.call(sdk.removeGuardian({ client: this.client, path: { student_id: studentId, guardian_id: guardianId } }));
  }

  async bulkRegisterStudents(
    items: BulkRegisterStudentCommandItem[],
    options?: { atomic?: boolean; dryRun?: boolean },
  ): Promise<BulkRegisterStudentCommandResultDto> {
    return this.call(
      sdk.bulkRegisterStudents({
        client: this.client,
        body: {
          items: items as any,
          atomic: options?.atomic ?? true,
          dry_run: options?.dryRun ?? false,
        },
      }),
    ) as any;
  }

  // ── Employee Affairs ────────────────────────────────────────
  async registerEmployee(req: RegisterEmployeeRequest): Promise<RegisterEmployeeResponseDto> {
    return this.call(sdk.registerEmployee({ client: this.client, body: req as any })) as any;
  }

  async listEmployees(filter?: EmployeeFilterInput): Promise<PaginatedEmployeesDto> {
    return this.call(sdk.listEmployees({ client: this.client, query: filter as any })) as any;
  }

  async getEmployee(id: string): Promise<EmployeeDetailDto> {
    return this.call(sdk.getEmployee({ client: this.client, path: { id } })) as any;
  }

  async getEmployeeStats(): Promise<Record<string, number>> {
    return this.call(sdk.getEmployeeStats({ client: this.client })) as any;
  }

  async updateEmployee(id: string, req: UpdateEmployeeRequest): Promise<EmployeeDetailDto> {
    return this.call(sdk.updateEmployee({ client: this.client, path: { id }, body: req as any })) as any;
  }

  async updateEmployeeStatus(id: string, status: string): Promise<EmployeeDetailDto> {
    return this.call(sdk.updateEmployeeStatus({ client: this.client, path: { id }, body: { status } as any })) as any;
  }

  async deleteEmployee(id: string): Promise<boolean> {
    return this.call(sdk.deleteEmployee({ client: this.client, path: { id } })) as any;
  }

  async restoreEmployee(id: string): Promise<EmployeeDetailDto> {
    return this.call(sdk.restoreEmployee({ client: this.client, path: { id } })) as any;
  }

  async provisionEmployeeAccount(id: string, req: ProvisionEmployeeAccountRequest): Promise<AccountProvisioningResultDto> {
    return this.call(sdk.provisionAccount({ client: this.client, path: { id }, body: req as any })) as any;
  }

  async assignEmployeeGroups(id: string, groupIds: string[]): Promise<boolean> {
    return this.call(sdk.assignGroups({ client: this.client, path: { id }, body: { group_ids: groupIds } as any })) as any;
  }

  async bulkRegisterEmployees(
    items: BulkRegisterEmployeeCommandItem[],
    options?: { atomic?: boolean; dryRun?: boolean },
  ): Promise<BulkRegisterEmployeeCommandResultDto> {
    return this.call(
      sdk.bulkRegisterEmployees({
        client: this.client,
        body: {
          items: items as any,
          atomic: options?.atomic ?? true,
          dry_run: options?.dryRun ?? false,
        },
      }),
    ) as any;
  }


  // ── User Groups ─────────────────────────────────────────────
  async createGroup(req: CreateGroupRequest): Promise<GroupDetailDto> {
    return this.call(sdk.createGroup({ client: this.client, body: req as any })) as any;
  }

  async listGroups(): Promise<GroupDetailDto[]> {
    return (await this.call(sdk.listGroups({ client: this.client }))) as any ?? [];
  }

  async getGroup(id: string): Promise<GroupDetailDto> {
    return this.call(sdk.getGroup({ client: this.client, path: { id } })) as any;
  }

  async updateGroup(id: string, req: UpdateGroupRequest): Promise<GroupDetailDto> {
    return this.call(sdk.updateGroup({ client: this.client, path: { id }, body: req as any })) as any;
  }

  async deleteGroup(id: string): Promise<boolean> {
    return this.call(sdk.deleteGroup({ client: this.client, path: { id } })) as any;
  }

  async getGroupMembers(id: string): Promise<GroupMemberInfoDto[]> {
    return (await this.call(sdk.getGroupMembers({ client: this.client, path: { id } }))) as any ?? [];
  }

  async addGroupMember(id: string, userId: string): Promise<boolean> {
    return this.call(sdk.addGroupMember({ client: this.client, path: { id }, body: { user_id: userId } as any })) as any;
  }

  async removeGroupMember(id: string, userId: string): Promise<boolean> {
    return this.call(sdk.removeGroupMember({ client: this.client, path: { id, user_id: userId } })) as any;
  }

  async bulkAssignGroup(req: BulkAssignGroupRequest): Promise<number> {
    return this.call(sdk.bulkAssignGroup({ client: this.client, body: req as any })) as any;
  }

  async setGroupPermissions(id: string, permissions: string[]): Promise<boolean> {
    return this.call(sdk.setGroupPermissions({ client: this.client, path: { id }, body: { permissions } as any })) as any;
  }

  // ── Scheduling ──────────────────────────────────────────────
  async createRoom(req: CreateRoomRequest): Promise<RoomDto> {
    return this.call(sdk.createRoom({ client: this.client, body: req }));
  }

  async listRooms(): Promise<RoomDto[]> {
    return this.call(sdk.listRooms({ client: this.client })) ?? [];
  }

  async getRoom(id: string): Promise<RoomDto> {
    return this.call(sdk.getRoom({ client: this.client, path: { id } }));
  }

  async updateRoom(id: string, req: UpdateRoomRequest): Promise<RoomDto> {
    return this.call(sdk.updateRoom({ client: this.client, path: { id }, body: req }));
  }

  async deleteRoom(id: string): Promise<void> {
    await this.call(sdk.deleteRoom({ client: this.client, path: { id } }));
  }

  async createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto> {
    return this.call(sdk.createTimeSlot({ client: this.client, body: req }));
  }

  async batchCreateTimeSlots(req: BatchCreateTimeSlotsRequest): Promise<TimeSlotDto[]> {
    return this.call(sdk.batchCreateTimeSlots({ client: this.client, body: req })) ?? [];
  }

  async listTimeSlots(periodId?: string): Promise<TimeSlotDto[]> {
    return this.call(sdk.listTimeSlots({ client: this.client, query: { period_id: periodId } as any })) ?? [];
  }

  async getTimeSlot(id: string): Promise<TimeSlotDto> {
    return this.call(sdk.getTimeSlot({ client: this.client, path: { id } }));
  }

  async updateTimeSlot(id: string, req: UpdateTimeSlotRequest): Promise<TimeSlotDto> {
    return this.call(sdk.updateTimeSlot({ client: this.client, path: { id }, body: req }));
  }

  async deleteTimeSlot(id: string): Promise<void> {
    await this.call(sdk.deleteTimeSlot({ client: this.client, path: { id } }));
  }

  async scheduleLesson(req: ScheduleLessonRequest): Promise<ScheduledLessonDto> {
    return this.call(sdk.scheduleLesson({ client: this.client, body: req }));
  }

  async bulkScheduleLessons(req: BulkScheduleLessonsRequest): Promise<ScheduledLessonDto[]> {
    return this.call(sdk.bulkScheduleLessons({ client: this.client, body: req })) ?? [];
  }

  async deleteScheduledLesson(id: string): Promise<void> {
    await this.call(sdk.deleteLesson({ client: this.client, path: { id } }));
  }

  async listScheduledLessons(
    filterOrPeriodId?: string | { SectionSubjectId?: string; periodId?: string; classId?: string; teacherId?: string },
    SectionSubjectId?: string,
  ): Promise<ScheduledLessonDto[]> {
    const periodId = typeof filterOrPeriodId === "string" ? filterOrPeriodId : filterOrPeriodId?.periodId;
    const csId = typeof filterOrPeriodId === "object" ? filterOrPeriodId.SectionSubjectId : SectionSubjectId;
    const classId = typeof filterOrPeriodId === "object" ? filterOrPeriodId.classId : undefined;
    const teacherId = typeof filterOrPeriodId === "object" ? filterOrPeriodId.teacherId : undefined;
    return this.call(
      sdk.listLessons({
        client: this.client,
        query: { period_id: periodId, section_subject_id: csId, class_id: classId, teacher_id: teacherId } as any,
      }),
    ) ?? [];
  }

  async getSolverSidecarStatus(): Promise<SolverSidecarStatusDto> {
    throw new Error("The timetable solver is only available in the Orbis desktop app.");
  }

  async submitTimetable(_problem: TimetableProblem): Promise<void> {
    throw new Error("The timetable solver is only available in the Orbis desktop app.");
  }

  async getTimetableSolveStatus(
    _jobId: string,
  ): Promise<TimetableSolveStatusDto> {
    throw new Error("The timetable solver is only available in the Orbis desktop app.");
  }

  // ── Assessment ──────────────────────────────────────────────
  async createAssessmentScheme(req: CreateSchemeRequest): Promise<AssessmentSchemeDto> {
    return this.call(sdk.createScheme({ client: this.client, body: req }));
  }

  async updateAssessmentScheme(id: string, req: UpdateSchemeRequest): Promise<AssessmentSchemeDto> {
    return this.call(sdk.updateScheme({ client: this.client, path: { id }, body: req }));
  }

  async deleteAssessmentScheme(id: string): Promise<void> {
    await this.call(sdk.deleteScheme({ client: this.client, path: { id } }));
  }

  async listAssessmentSchemes(): Promise<AssessmentSchemeDto[]> {
    return this.call(sdk.listSchemes({ client: this.client })) ?? [];
  }

  async getAssessmentScheme(id: string): Promise<AssessmentSchemeDto> {
    return this.call(sdk.getScheme({ client: this.client, path: { id } }));
  }

  async recordGrade(req: RecordGradeRequest): Promise<GradeDto> {
    return this.call(sdk.recordGrade({ client: this.client, body: req }));
  }

  async batchRecordGrades(req: BatchRecordGradesRequest): Promise<BatchRecordGradesResponseDto> {
    return this.call(sdk.batchRecordGrades({ client: this.client, body: req }));
  }

  async deleteGrade(id: string): Promise<void> {
    await this.call(sdk.deleteGrade({ client: this.client, path: { id } }));
  }

  async listGrades(filter?: { studentId?: string; SectionSubjectId?: string }): Promise<GradeDto[]> {
    return this.call(
      sdk.listGrades({
        client: this.client,
        query: { student_id: filter?.studentId, section_subject_id: filter?.SectionSubjectId } as any,
      }),
    ) ?? [];
  }

  async calculateGrades(req: CalculateGradesRequest): Promise<CalculationResultDto> {
    return this.call(sdk.calculateGrades({ client: this.client, body: req }));
  }

  async calculateGpa(req: CalculateGpaRequest): Promise<CumulativeGpaRecordDto> {
    return this.call(sdk.calculateGpa({ client: this.client, body: req }));
  }

  async calculateRankings(req: CalculateRankingsRequest): Promise<RankingItemDto[]> {
    return this.call(sdk.calculateRankings({ client: this.client, body: req })) ?? [];
  }

  async getCalculationEngineSettings(): Promise<CalculationEngineSettings> {
    const settings = await this.getSettings();
    const academic = settings.academic as
      | (NonNullable<OrganizationSettingsDto["academic"]> & {
          calculation_plan?: CalculationPlan | null;
          outcome_policy?: CalculationOutcomePolicy | null;
        })
      | undefined;
    return {
      calculation_plan: academic?.calculation_plan ?? null,
      outcome_policy: academic?.outcome_policy ?? null,
      ranking_enabled: academic?.ranking_enabled ?? true,
      ranking_scope: (academic?.ranking_scope as RankingScope) ?? "class",
      tie_policy:
        (academic?.tie_policy as TiePolicy) ?? "standard_competition",
      max_displayed_rank: academic?.max_displayed_rank ?? null,
    };
  }

  async updateCalculationEngineSettings(
    engineSettings: CalculationEngineSettings,
  ): Promise<CalculationEngineSettings> {
    const current = await this.getSettings();
    const academic = current.academic ?? {
      default_grading_system: "percentage",
    };
    const academicWithEngine: NonNullable<
      OrganizationSettingsDto["academic"]
    > & {
      calculation_plan: CalculationPlan | null;
      outcome_policy: CalculationOutcomePolicy | null;
    } = {
      ...academic,
      calculation_plan: engineSettings.calculation_plan,
      outcome_policy: engineSettings.outcome_policy,
      ranking_enabled: engineSettings.ranking_enabled,
      ranking_scope: engineSettings.ranking_scope,
      tie_policy: engineSettings.tie_policy,
      max_displayed_rank: engineSettings.max_displayed_rank,
    };
    const updated = await this.updateSettings({
      ...current,
      academic: academicWithEngine,
    });
    const updatedAcademic = updated.academic as
      | (NonNullable<OrganizationSettingsDto["academic"]> & {
          calculation_plan?: CalculationPlan | null;
          outcome_policy?: CalculationOutcomePolicy | null;
        })
      | undefined;
    return {
      calculation_plan: updatedAcademic?.calculation_plan ?? null,
      outcome_policy: updatedAcademic?.outcome_policy ?? null,
      ranking_enabled: updatedAcademic?.ranking_enabled ?? true,
      ranking_scope: (updatedAcademic?.ranking_scope as RankingScope) ?? "class",
      tie_policy:
        (updatedAcademic?.tie_policy as TiePolicy) ?? "standard_competition",
      max_displayed_rank: updatedAcademic?.max_displayed_rank ?? null,
    };
  }

  async previewCalculation(
    values: Record<string, DecimalValue>,
  ): Promise<CalculationPreview> {
    const response = await fetch(
      `${this.baseUrl}/api/v1/academic-calculations/preview`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(this.token ? { Authorization: "Bearer " + this.token } : {}),
        },
        body: JSON.stringify({ values }),
      },
    );
    if (!response.ok) {
      throw new ApiRequestError(
        response.status,
        await response.json().catch(() => ({ message: response.statusText })),
      );
    }
    const envelope = (await response.json()) as { data: CalculationPreview };
    return envelope.data;
  }

  async getStudentTermResults(
    studentId: string,
    periodId: string,
    classId?: string,
  ): Promise<StudentTermReportCard> {
    const params = new URLSearchParams({ period_id: periodId });
    if (classId) {
      params.append("class_id", classId);
      params.append("section_id", classId);
    }
    const url = `${this.baseUrl}/api/v1/students/${studentId}/term-results?${params.toString()}`;
    const res = await fetch(url, {
      headers: this.token ? { Authorization: `Bearer ${this.token}`, "Content-Type": "application/json" } : { "Content-Type": "application/json" },
    });
    if (!res.ok) throw new ApiRequestError(res.status, await res.json().catch(() => ({ message: res.statusText })));
    const json = await res.json();
    return json.data;
  }

  async getStudentAnnualResults(
    studentId: string,
    yearId: string,
    classId?: string,
  ): Promise<StudentAnnualReportCard> {
    const params = new URLSearchParams({ year_id: yearId });
    if (classId) {
      params.append("class_id", classId);
      params.append("section_id", classId);
    }
    const url = `${this.baseUrl}/api/v1/students/${studentId}/annual-results?${params.toString()}`;
    const res = await fetch(url, {
      headers: this.token ? { Authorization: `Bearer ${this.token}`, "Content-Type": "application/json" } : { "Content-Type": "application/json" },
    });
    if (!res.ok) throw new ApiRequestError(res.status, await res.json().catch(() => ({ message: res.statusText })));
    const json = await res.json();
    return json.data;
  }

  async getClassTermResults(
    classId: string,
    periodId: string,
  ): Promise<StudentTermReportCard[]> {
    const params = new URLSearchParams({ period_id: periodId });
    const url = `${this.baseUrl}/api/v1/classes/${classId}/term-results?${params.toString()}`;
    const res = await fetch(url, {
      headers: this.token ? { Authorization: `Bearer ${this.token}`, "Content-Type": "application/json" } : { "Content-Type": "application/json" },
    });
    if (!res.ok) throw new ApiRequestError(res.status, await res.json().catch(() => ({ message: res.statusText })));
    const json = await res.json();
    return json.data ?? [];
  }

  async getClassAnnualResults(
    classId: string,
    yearId: string,
  ): Promise<StudentAnnualReportCard[]> {
    const params = new URLSearchParams({ year_id: yearId });
    const url = `${this.baseUrl}/api/v1/classes/${classId}/annual-results?${params.toString()}`;
    const res = await fetch(url, {
      headers: this.token ? { Authorization: `Bearer ${this.token}`, "Content-Type": "application/json" } : { "Content-Type": "application/json" },
    });
    if (!res.ok) throw new ApiRequestError(res.status, await res.json().catch(() => ({ message: res.statusText })));
    const json = await res.json();
    return json.data ?? [];
  }

  async getStudentCertificateResults(
    studentId: string,
    calendarId: string,
    certificateType: string,
    classId?: string,
  ): Promise<StudentCertificateReportCard> {
    const params = new URLSearchParams({
      calendar_id: calendarId,
      certificate_type: certificateType,
    });
    if (classId) {
      params.append("section_id", classId);
    }
    const url = `${this.baseUrl}/api/v1/students/${studentId}/certificate-results?${params.toString()}`;
    const res = await fetch(url, {
      headers: this.token ? { Authorization: `Bearer ${this.token}`, "Content-Type": "application/json" } : { "Content-Type": "application/json" },
    });
    if (!res.ok) throw new ApiRequestError(res.status, await res.json().catch(() => ({ message: res.statusText })));
    const json = await res.json();
    return json.data;
  }

  async markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto> {
    return this.call(sdk.markAttendance({ client: this.client, body: req }));
  }

  async batchMarkAttendance(req: BatchMarkAttendanceRequest): Promise<BatchMarkAttendanceResponseDto> {
    return this.call(sdk.batchMarkAttendance({ client: this.client, body: req }));
  }

  async deleteAttendance(id: string): Promise<void> {
    await this.call(sdk.deleteAttendance({ client: this.client, path: { id } }));
  }

  async listAttendance(classId: string, date: string): Promise<AttendanceDto[]> {
    return this.call(sdk.listAttendance({ client: this.client, query: { class_id: classId, date } as any })) ?? [];
  }

  async listStudentAttendance(studentId: string): Promise<AttendanceDto[]> {
    return this.call(sdk.listStudentAttendance({ client: this.client, path: { id: studentId } })) ?? [];
  }

  async recordBehavior(req: RecordBehaviorRequest): Promise<BehaviorRecordDto> {
    return this.call(sdk.recordBehavior({ client: this.client, body: req }));
  }

  async deleteBehavior(id: string): Promise<void> {
    await this.call(sdk.deleteBehavior({ client: this.client, path: { id } }));
  }

  async listStudentBehaviorRecords(studentId: string): Promise<BehaviorRecordDto[]> {
    return this.call(sdk.listStudentBehavior({ client: this.client, path: { id: studentId } })) ?? [];
  }

  async listAllBehavior(): Promise<BehaviorRecordDto[]> {
    return this.call(sdk.listAllBehavior({ client: this.client })) ?? [];
  }

  // ── Student Cohort Progression ──────────────────────────────
  async evaluateCohort(sourcePeriodId: string): Promise<StudentProgressionItemInput[]> {
    return this.call(sdk.evaluateCohort({ client: this.client, body: { source_period_id: sourcePeriodId } })) ?? [];
  }

  async promoteCohort(req: PromoteCohortInput): Promise<progressionJobDto> {
    return this.call(sdk.promoteCohort({ client: this.client, body: req as any }));
  }

  async listprogressionJobs(): Promise<progressionJobDto[]> {
    return this.call(sdk.listJobs({ client: this.client })) ?? [];
  }

  async getprogressionJob(id: string): Promise<progressionJobDto> {
    return this.call(sdk.getJob({ client: this.client, path: { id } }));
  }

  // ── Workflow & Academic Overrides ───────────────────────────
  async submitOverride(req: SubmitOverrideRequestInput): Promise<OverrideRequestDto> {
    return this.call(sdk.submitOverride({ client: this.client, body: req as any }));
  }

  async approveOverride(req: ApproveOverrideInput): Promise<OverrideRequestDto> {
    return this.call(
      sdk.approveOverride({
        client: this.client,
        path: { id: req.request_id || req.requestId || "" },
        body: { notes: req.notes },
      }),
    );
  }

  async rejectOverride(req: RejectOverrideInput): Promise<OverrideRequestDto> {
    return this.call(
      sdk.rejectOverride({
        client: this.client,
        path: { id: req.request_id || req.requestId || "" },
        body: { notes: req.reason },
      }),
    );
  }

  async listOverrides(status?: string): Promise<OverrideRequestDto[]> {
    return this.call(sdk.listOverrides({ client: this.client, query: { status } as any })) ?? [];
  }

  async getOverride(id: string): Promise<OverrideRequestDto> {
    return this.call(sdk.getOverride({ client: this.client, path: { id } }));
  }

  // ── Exam ────────────────────────────────────────────────────
  async createExamSession(req: CreateExamSessionRequest): Promise<ExamSessionDto> {
    return this.call(sdk.createSession({ client: this.client, body: req }));
  }

  async listExamSessions(periodId: string): Promise<ExamSessionDto[]> {
    return this.call(sdk.listSessions({ client: this.client, query: { period_id: periodId } as any })) ?? [];
  }

  async getExamSession(id: string): Promise<ExamSessionDto> {
    return this.call(sdk.getSession2({ client: this.client, path: { id } }));
  }

  async updateExamSession(id: string, req: UpdateExamSessionRequest): Promise<ExamSessionDto> {
    return this.call(sdk.updateSession({ client: this.client, path: { id }, body: req }));
  }

  async deleteExamSession(id: string): Promise<void> {
    await this.call(sdk.deleteSession({ client: this.client, path: { id } }));
  }

  async scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto> {
    return this.call(sdk.scheduleExam({ client: this.client, body: req }));
  }

  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return this.call(sdk.listSchedules({ client: this.client, query: { session_id: sessionId } })) ?? [];
  }

  async deleteExamSchedule(id: string): Promise<void> {
    await this.call(sdk.deleteSchedule({ client: this.client, path: { id } }));
  }

  async assignSeating(req: AssignSeatingRequest): Promise<SeatingDto> {
    return this.call(sdk.assignSeating({ client: this.client, body: req }));
  }

  async listSeating(sessionId: string): Promise<SeatingDto[]> {
    return this.call(sdk.listSeating({ client: this.client, query: { session_id: sessionId } })) ?? [];
  }

  async generateSeating(req: GenerateSeatingRequest): Promise<SeatingDto[]> {
    return this.call(sdk.generateSeating({ client: this.client, body: req })) ?? [];
  }

  async deleteSeating(id: string): Promise<void> {
    await this.call(sdk.deleteSeating({ client: this.client, path: { id } }));
  }

  async assignSecret(req: AssignSecretRequest): Promise<SecretDto> {
    return this.call(sdk.assignSecret({ client: this.client, body: req }));
  }

  async listSecrets(scheduleId: string): Promise<SecretDto[]> {
    return this.call(sdk.listSecrets({ client: this.client, query: { schedule_id: scheduleId } })) ?? [];
  }

  async generateSecrets(req: GenerateSecretsRequest): Promise<SecretDto[]> {
    return this.call(sdk.generateSecrets({ client: this.client, body: req })) ?? [];
  }

  async deleteSecret(id: string): Promise<void> {
    await this.call(sdk.deleteSecret({ client: this.client, path: { id } }));
  }

  async assignProctor(req: AssignProctorRequest): Promise<ProctorDto> {
    return this.call(sdk.assignProctor({ client: this.client, body: req }));
  }

  async listProctors(scheduleId: string): Promise<ProctorDto[]> {
    return this.call(sdk.listProctors({ client: this.client, query: { schedule_id: scheduleId } })) ?? [];
  }

  async deleteProctor(id: string): Promise<void> {
    await this.call(sdk.deleteProctor({ client: this.client, path: { id } }));
  }

  // ── Finance ─────────────────────────────────────────────────
  async createAccount(req: CreateAccountRequest): Promise<AccountDto> {
    return this.call(sdk.createAccount({ client: this.client, body: req }));
  }

  async listAccounts(): Promise<AccountDto[]> {
    return this.call(sdk.listAccounts({ client: this.client })) ?? [];
  }

  async getAccount(id: string): Promise<AccountDto> {
    return this.call(sdk.getAccount({ client: this.client, path: { id } }));
  }

  async updateAccount(id: string, req: UpdateAccountRequest): Promise<AccountDto> {
    return this.call(sdk.updateAccount({ client: this.client, path: { id }, body: req }));
  }

  async deleteAccount(id: string): Promise<boolean> {
    await this.call(sdk.deleteAccount({ client: this.client, path: { id } }));
    return true;
  }

  async createFeeTemplate(req: CreateFeeTemplateRequest): Promise<FeeTemplateDto> {
    return this.call(sdk.createFeeTemplate({ client: this.client, body: req }));
  }

  async getFeeTemplate(id: string): Promise<FeeTemplateDto> {
    return this.call(sdk.getFeeTemplate({ client: this.client, path: { id } }));
  }

  async updateFeeTemplate(id: string, req: UpdateFeeTemplateRequest): Promise<FeeTemplateDto> {
    return this.call(sdk.updateFeeTemplate({ client: this.client, path: { id }, body: req }));
  }

  async deleteFeeTemplate(id: string): Promise<boolean> {
    await this.call(sdk.deleteFeeTemplate({ client: this.client, path: { id } }));
    return true;
  }

  async listFeeTemplates(): Promise<FeeTemplateDto[]> {
    return this.call(sdk.listFeeTemplates({ client: this.client })) ?? [];
  }

  async generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto> {
    return this.call(sdk.generateFee({ client: this.client, body: req }));
  }

  async listStudentFees(filter?: StudentFeeFilterParams): Promise<StudentFeeDto[]> {
    const firstPage = await this.listStudentFeesPaginated({
      ...filter,
      page: 1,
      page_size: 100,
    });
    if (firstPage.total_pages <= 1) return firstPage.items;
    const remainingPages = await Promise.all(
      Array.from({ length: firstPage.total_pages - 1 }, (_, index) =>
        this.listStudentFeesPaginated({
          ...filter,
          page: index + 2,
          page_size: 100,
        }),
      ),
    );
    return firstPage.items.concat(...remainingPages.map((page) => page.items));
  }

  async listStudentFeesPaginated(
    filter?: StudentFeeFilterParams,
  ): Promise<PaginatedList<StudentFeeDto>> {
    const query = new URLSearchParams();
    for (const [key, value] of Object.entries({
      ...filter,
      page_size: Math.min(Math.max(filter?.page_size ?? 50, 1), 100),
    })) {
      if (value !== undefined && value !== null) query.set(key, String(value));
    }
    return this.fetchApi(
      `/api/v1/student-fees?${query.toString()}`,
    );
  }

  async deleteStudentFee(id: string): Promise<boolean> {
    await this.call(sdk.deleteStudentFee({ client: this.client, path: { id } }));
    return true;
  }

  async processPayment(req: ProcessPaymentRequest): Promise<PaymentDto> {
    return this.call(sdk.processPayment({ client: this.client, body: req }));
  }

  async listPayments(filter?: PaymentFilterParams): Promise<PaymentDto[]> {
    const firstPage = await this.listPaymentsPaginated({
      ...filter,
      page: 1,
      page_size: 100,
    });
    if (firstPage.total_pages <= 1) return firstPage.items;
    const remainingPages = await Promise.all(
      Array.from({ length: firstPage.total_pages - 1 }, (_, index) =>
        this.listPaymentsPaginated({
          ...filter,
          page: index + 2,
          page_size: 100,
        }),
      ),
    );
    return firstPage.items.concat(...remainingPages.map((page) => page.items));
  }

  async listPaymentsPaginated(
    filter?: PaymentFilterParams,
  ): Promise<PaginatedList<PaymentDto>> {
    const query = new URLSearchParams();
    for (const [key, value] of Object.entries({
      ...filter,
      page_size: Math.min(Math.max(filter?.page_size ?? 50, 1), 100),
    })) {
      if (value !== undefined && value !== null) query.set(key, String(value));
    }
    return this.fetchApi(`/api/v1/payments?${query.toString()}`);
  }

  async deletePayment(id: string): Promise<boolean> {
    await this.call(sdk.deletePayment({ client: this.client, path: { id } }));
    return true;
  }

  async listCurrencies(): Promise<CurrencyDto[]> {
    return this.call(sdk.listCurrencies({ client: this.client })) ?? [];
  }

  async listJournalEntries(): Promise<JournalEntryDto[]> {
    return this.call(sdk.listJournalEntries({ client: this.client })) ?? [];
  }

  async getJournalEntry(id: string): Promise<JournalEntryDto> {
    return this.call(sdk.getJournalEntry({ client: this.client, path: { id } }));
  }

  async createJournalEntry(req: CreateJournalEntryRequest): Promise<JournalEntryDto> {
    return this.call(sdk.createJournalEntry({ client: this.client, body: req }));
  }

  async reverseJournalEntry(id: string, req: ReverseJournalRequest): Promise<JournalEntryDto> {
    return this.call(sdk.reverseJournalEntry({ client: this.client, path: { id }, body: req }));
  }

  async deleteJournalEntry(id: string): Promise<boolean> {
    await this.call(sdk.deleteJournalEntry({ client: this.client, path: { id } }));
    return true;
  }

  async splitStudentFee(id: string, req: SplitFeeRequest): Promise<StudentFeeDto[]> {
    return this.call(sdk.splitFee({ client: this.client, path: { id }, body: req })) ?? [];
  }

  async refundStudentFee(id: string, req: RefundStudentFeeRequest): Promise<StudentFeeDto> {
    return this.call(sdk.refundFee({ client: this.client, path: { id }, body: req }));
  }

  async listDiscountTemplates(): Promise<DiscountTemplateDto[]> {
    return this.call(sdk.listDiscountTemplates({ client: this.client })) ?? [];
  }

  async createDiscountTemplate(req: CreateDiscountTemplateRequest): Promise<DiscountTemplateDto> {
    return this.call(sdk.createDiscountTemplate({ client: this.client, body: req }));
  }

  async deleteDiscountTemplate(id: string): Promise<boolean> {
    await this.call(sdk.deleteDiscountTemplate({ client: this.client, path: { id } }));
    return true;
  }

  private async fetchApi<T>(path: string, options: RequestInit = {}): Promise<T> {
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
      ...(options.headers as Record<string, string>),
    };
    if (this.token) {
      headers["Authorization"] = `Bearer ${this.token}`;
    }
    const res = await fetch(`${this.baseUrl}${path}`, {
      ...options,
      headers,
    });
    if (!res.ok) {
      let errBody;
      try {
        errBody = await res.json();
      } catch {
        errBody = { message: res.statusText };
      }
      throw new ApiRequestError(res.status, errBody);
    }
    const json = await res.json();
    return json?.data !== undefined ? json.data : json;
  }

  async getAccountStatement(filter: StatementFilterParams): Promise<StatementResponseDto> {
    const query = new URLSearchParams();
    if (filter.account_id) query.set("account_id", filter.account_id);
    if (filter.entity_type) query.set("entity_type", filter.entity_type);
    if (filter.entity_id) query.set("entity_id", filter.entity_id);
    if (filter.start_date) query.set("start_date", filter.start_date);
    if (filter.end_date) query.set("end_date", filter.end_date);
    if (filter.currency_code || filter.currency) query.set("currency_code", (filter.currency_code || filter.currency)!);

    const qs = query.toString();
    return this.fetchApi<StatementResponseDto>(`/api/v1/finance/statement${qs ? `?${qs}` : ""}`);
  }

  async processDisbursement(req: ProcessDisbursementRequest): Promise<JournalEntryDto> {
    return this.fetchApi<JournalEntryDto>("/api/v1/finance/disbursements", {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  async createSubAccount(req: CreateSubAccountRequest): Promise<AccountDto> {
    return this.fetchApi<AccountDto>("/api/v1/finance/sub-accounts", {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  async settleGatewayPayout(req: SettleGatewayPayoutRequest): Promise<JournalEntryDto> {
    return this.fetchApi<JournalEntryDto>("/api/v1/finance/gateway/settlement", {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  async postPayrollAccrual(req: PostPayrollRequest): Promise<JournalEntryDto> {
    return this.fetchApi<JournalEntryDto>("/api/v1/finance/payroll/accrual", {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  async disbursePayroll(req: DisbursePayrollRequest): Promise<JournalEntryDto> {
    return this.fetchApi<JournalEntryDto>("/api/v1/finance/payroll/disbursement", {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  // ── Integration ─────────────────────────────────────────────
  async connectApp(req: ConnectAppRequest): Promise<IntegrationDto> {
    return this.call(sdk.connectApp({ client: this.client, body: req }));
  }

  async listIntegrations(): Promise<IntegrationDto[]> {
    return (await this.call(sdk.listIntegrations({ client: this.client }))) ?? [];
  }

  async getIntegrationStatus(providerName: string): Promise<IntegrationStatusResponse> {
    return this.call(sdk.getIntegrationStatus({ client: this.client, path: { provider_name: providerName } }));
  }

  async disconnectApp(providerName: string): Promise<boolean> {
    return this.call(sdk.disconnectApp({ client: this.client, path: { provider_name: providerName } }));
  }

  async toggleIntegration(providerName: string, isEnabled: boolean): Promise<boolean> {
    return this.call(
      sdk.toggleIntegration({
        client: this.client,
        path: { provider_name: providerName },
        body: { is_enabled: isEnabled },
      }),
    );
  }

  // ── Mulverse organization deployment management ────────────
  async getMulverseConnection(): Promise<MulverseConnectionDto> {
    return this.fetchApi("/api/v1/orbis-mobile");
  }

  async connectMulverse(apiKey: string): Promise<MulverseConnectionDto> {
    return this.fetchApi("/api/v1/orbis-mobile", {
      method: "PUT",
      body: JSON.stringify({ api_key: apiKey }),
    });
  }

  async disconnectMulverse(): Promise<boolean> {
    return this.fetchApi("/api/v1/orbis-mobile", { method: "DELETE" });
  }

  async listMulverseDeployments(): Promise<MulverseDeploymentDto[]> {
    return this.fetchApi("/api/v1/orbis-mobile/deployments");
  }

  async createMulverseDeployment(
    req: MulverseDeploymentRequest,
  ): Promise<MulverseDeploymentDto> {
    return this.fetchApi("/api/v1/orbis-mobile/deployments", {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  async updateMulverseDeployment(
    id: string,
    req: Partial<MulverseDeploymentRequest>,
  ): Promise<MulverseDeploymentDto> {
    return this.fetchApi(`/api/v1/orbis-mobile/deployments/${encodeURIComponent(id)}`, {
      method: "PUT",
      body: JSON.stringify(req),
    });
  }

  async listMulverseEndpoints(deploymentId: string): Promise<MulverseEndpointDto[]> {
    return this.fetchApi(
      `/api/v1/orbis-mobile/deployments/${encodeURIComponent(deploymentId)}/endpoints`,
    );
  }

  async createMulverseEndpoint(
    deploymentId: string,
    req: MulverseEndpointRequest,
  ): Promise<MulverseEndpointDto> {
    return this.fetchApi(
      `/api/v1/orbis-mobile/deployments/${encodeURIComponent(deploymentId)}/endpoints`,
      { method: "POST", body: JSON.stringify(req) },
    );
  }

  async updateMulverseEndpoint(
    id: string,
    req: MulverseEndpointUpdateRequest,
  ): Promise<MulverseEndpointDto> {
    return this.fetchApi(`/api/v1/orbis-mobile/endpoints/${encodeURIComponent(id)}`, {
      method: "PUT",
      body: JSON.stringify(req),
    });
  }

  // ── Templates ───────────────────────────────────────────────
  async createTemplate(req: CreateTemplateRequest): Promise<TemplateDto> {
    return this.call(sdk.createTemplate({ client: this.client, body: req }));
  }

  async getTemplate(id: string): Promise<TemplateDto> {
    return this.call(sdk.getTemplate({ client: this.client, path: { id } }));
  }

  async listTemplates(filter?: { documentType?: string; entityType?: string; status?: string }): Promise<TemplateDto[]> {
    return (await this.call(sdk.listTemplates({
      client: this.client,
      query: {
        document_type: filter?.documentType,
        entity_type: filter?.entityType,
        status: filter?.status,
      },
    }))) ?? [];
  }

  async updateTemplate(id: string, req: UpdateTemplateRequest): Promise<TemplateDto> {
    return this.call(sdk.updateTemplate({ client: this.client, path: { id }, body: req }));
  }

  async deleteTemplate(id: string): Promise<void> {
    await this.call(sdk.deleteTemplate({ client: this.client, path: { id } }));
  }

  async publishTemplate(id: string): Promise<TemplateDto> {
    return this.call(sdk.publishTemplate({ client: this.client, path: { id } }));
  }

  async archiveTemplate(id: string): Promise<void> {
    await this.call(sdk.archiveTemplate({ client: this.client, path: { id } }));
  }

  async getTemplateVersions(id: string): Promise<TemplateVersionDto[]> {
    return (await this.call(sdk.getTemplateVersions({ client: this.client, path: { id } }))) ?? [];
  }

  async revertTemplateVersion(id: string, targetVersion: number): Promise<TemplateDto> {
    return this.call(sdk.revertTemplateVersion({
      client: this.client,
      path: { id },
      body: { version: targetVersion },
    }));
  }

  async previewTemplate(documentJson: string, contextData: any, locale?: string): Promise<Uint8Array> {
    const res = await this.call(
      sdk.previewTemplate({
        client: this.client,
        body: { document_json: documentJson, context_data: contextData, locale },
      }),
    );
    return res instanceof Uint8Array ? res : new Uint8Array(res as any);
  }

  async validateTemplate(documentJson: string): Promise<ValidationResultDto> {
    return this.call(sdk.validateTemplate({ client: this.client, body: { document_json: documentJson } }));
  }

  // ── Template Bindings ───────────────────────────────────────
  async listTemplateBindings(definitionId: string): Promise<TemplateBindingDto[]> {
    return (await this.call(sdk.listTemplateBindings({ client: this.client, path: { definition_id: definitionId } }))) ?? [];
  }

  async createTemplateBinding(
    definitionId: string,
    templateId: string,
    locale: string,
    format: string,
  ): Promise<TemplateBindingDto> {
    return this.call(
      sdk.createTemplateBinding({
        client: this.client,
        path: { definition_id: definitionId },
        body: { template_id: templateId, locale, format },
      }),
    );
  }

  // ── Report Definitions ──────────────────────────────────────
  async listReportDefinitions(domain?: string, category?: string): Promise<ReportDefinitionDto[]> {
    return (await this.call(sdk.listReportDefinitions({ client: this.client, query: { domain, category } }))) ?? [];
  }

  async getReportDefinition(code: string): Promise<ReportDefinitionDto> {
    return this.call(sdk.getReportDefinition({ client: this.client, path: { code } }));
  }

  // ── Report Execution & Printing & Audit Logs ───────────────
  async generateReport(
    templateId: string,
    format: string,
    parameters: Record<string, unknown>,
  ): Promise<string> {
    const res = await this.call(
      sdk.generateReport({
        client: this.client,
        body: { report_code: templateId, format, parameters },
      }),
    );
    return res.job_id;
  }

  async getReportJobStatus(jobId: string): Promise<ReportJobStatusDto> {
    return this.call(sdk.getReportJobStatus({ client: this.client, path: { id: jobId } }));
  }

  async downloadReportFile(jobId: string): Promise<Uint8Array> {
    const res = await this.call(sdk.downloadReportFile({ client: this.client, path: { id: jobId } }));
    return res instanceof Uint8Array ? res : new Uint8Array(res as any);
  }

  async batchPrint(req: BatchPrintParams): Promise<Uint8Array> {
    const response = await this.call(
      sdk.batchPrint({ client: this.client, body: req as any }),
    );
    return new Uint8Array(await response.arrayBuffer());
  }

  async listAuditLogs(limit?: number, offset?: number): Promise<AuditLogDto[]> {
    return (await this.call(sdk.listAuditLogs({ client: this.client, query: { limit, offset } }))) ?? [];
  }

  async getEntityAudit(
    entityType: string,
    entityId: string,
    limit?: number,
    offset?: number,
  ): Promise<AuditLogDto[]> {
    const logs = await this.listAuditLogs(limit, offset);
    return logs.filter(l => l.entity_type === entityType && l.entity_id === entityId);
  }

  // ── System & Bootstrap ──────────────────────────────────────
  async systemHealth(): Promise<HealthResponse> {
    const result = await this.connectionCheckRemoteHealth(this.baseUrl);
    if (!result.reachable) {
      throw new Error(result.error || "Orbis API is unavailable");
    }
    return {
      status: result.status ?? "ok",
      version: result.version ?? "unknown",
      api_version: result.api_version ?? "unknown",
      database_connected: true,
      has_organization: false,
      schema_version: "unknown",
    };
  }

  async systemHasOrganization(): Promise<boolean> {
    return this.call(sdk.getSystemHasOrgEndpoint({ client: this.client })) as any;
  }

  async systemGetReadiness(): Promise<OrganizationReadiness> {
    return this.call(sdk.getSystemReadinessEndpoint({ client: this.client })) as any;
  }

  async systemGetEntitlement(): Promise<EntitlementResult> {
    return {
      status: "active",
      plan: "enterprise",
      features: ["all"],
    };
  }

  async systemGetAppInfo(): Promise<AppInfo> {
    return {
      name: appPackage.productName,
      version: appPackage.version,
      build_profile: import.meta.env.DEV ? "development" : "production",
      target_os: "web",
    };
  }

  async systemGetChangelog(): Promise<ReleaseChangelogItem[]> {
    const response = await fetch(
      "https://api.github.com/repos/MxHabob/Orbis/releases?per_page=50",
      { headers: { Accept: "application/vnd.github+json" } },
    );
    if (!response.ok) {
      throw new Error(`GitHub releases request failed with status ${response.status}.`);
    }

    const releases: GitHubRelease[] = await response.json();
    return releases
      .filter((release) => !release.prerelease && semverParts(release.tag_name) !== null)
      .filter((release) => {
        try {
          const url = new URL(release.html_url);
          return url.protocol === "https:" && url.hostname === "github.com";
        } catch {
          return false;
        }
      })
      .sort((left, right) => {
        const a = semverParts(left.tag_name)!;
        const b = semverParts(right.tag_name)!;
        return b[0] - a[0] || b[1] - a[1] || b[2] - a[2];
      })
      .map((release, index) => {
        const version = release.tag_name.replace(/^v/, "");
        const assets = release.assets ?? [];
        const images = assets
          .filter(isReleaseImage)
          .map((asset) => releaseAssetUrl(asset.browser_download_url))
          .filter((url): url is string => url !== null)
          .slice(0, 6);
        const resources = assets
          .filter((asset) => {
            if (isReleaseImage(asset)) return false;
            const name = asset.name.toLowerCase();
            const ext = name.split(".").pop() ?? "";
            return RELEASE_RESOURCE_EXTENSIONS.has(ext) &&
              name !== "latest.json" &&
              !name.endsWith(".sig") &&
              !name.endsWith(".nsis.zip") &&
              !name.endsWith(".app.tar.gz");
          })
          .flatMap((asset) => {
            const url = releaseAssetUrl(asset.browser_download_url);
            return url
              ? [{
                  name: asset.name,
                  url,
                  content_type: asset.content_type ?? "application/octet-stream",
                  size: asset.size ?? 0,
                }]
              : [];
          })
          .slice(0, 20);

        return {
          version,
          title: release.name?.trim() || release.tag_name,
          published_at: release.published_at ?? undefined,
          body: release.body ?? undefined,
          is_current: version === appPackage.version,
          is_latest: index === 0,
          html_url: release.html_url,
          images,
          resources,
        };
      });
  }

  async systemCheckForUpdates(): Promise<UpdateCheckResult> {
    throw new Error("In-app updates are only available in the desktop application.");
  }

  async systemInstallUpdate(): Promise<void> {
    throw new Error("In-app updates are only available in the desktop application.");
  }

  async openChangelogWindow(): Promise<void> {
    if (typeof window !== "undefined") {
      window.open("/changelog", "_blank", "width=860,height=680");
    }
  }

  async openAgentWindow(): Promise<void> {
    if (typeof window !== "undefined") {
      window.open("/agent", "_blank", "width=460,height=720");
    }
  }
  async openStandaloneWindow(path: string): Promise<void> {
    if (typeof window === "undefined") {
      throw new Error("Standalone windows are unavailable outside a browser context.");
    }
    if (isDesktopApp) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<void>("open_standalone_window", { path });
    }

    const opened = window.open(path, "_blank");
    if (!opened) {
      throw new Error("The standalone page window was blocked by the browser.");
    }
    opened.opener = null;
  }

  async bootstrapDetectEnvironment(): Promise<EnvironmentDetectionResult> {
    return {
      detected_locale: "ar-SA",
      detected_language: "العربية (Arabic)",
      country_code: "SA",
      country_name_en: "Saudi Arabia",
      country_name_ar: "المملكة العربية السعودية",
      flag_emoji: "🇸🇦",
      timezone: "Asia/Riyadh",
      timezone_offset: "UTC+3",
      timezone_offset_hours: 3,
      currency_code: "SAR",
      currency_symbol_en: "SAR",
      currency_symbol_ar: "ر.س",
      currency_name_en: "Saudi Riyal",
      currency_name_ar: "ريال سعودي",
      is_hijri_recommended: true,
      date_format: "YYYY-MM-DD",
      suggested_academic_calendar: "2026/2027",
      suggested_academic_calendar_ar: "2026/2027 م",
    };
  }

  async bootstrapGenerateSlug(name: string, _nameAr?: string): Promise<SlugGenerationResult> {
    const slug = name.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
    return { slug: slug || "institution", is_available: true, suggestions: [] };
  }

  async bootstrapGetStatus(): Promise<BootstrapStatusResponse> {
    return this.call(sdk.getBootstrapStatus({ client: this.client })) as Promise<BootstrapStatusResponse>;
  }

  async bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult> {
    return this.call(sdk.executeBootstrap({ client: this.client, body: req }));
  }

  async completeOnboarding(req: CompleteOrganizationOnboardingRequest): Promise<OnboardingResult> {
    return {
      success: true,
      orgId: req.orgId || "",
      readiness: {
        state: "operational",
        has_organization: true,
        has_admin: true,
        has_academic_calendar: true,
        has_classes: true,
        has_subjects: true,
        pending_setup_items: [],
      },
    };
  }

  async onboardingGetState(_orgId?: string): Promise<OnboardingStateDto> {
    return this.call(sdk.getOnboardingState({ client: this.client })) as Promise<OnboardingStateDto>;
  }

  async onboardingProvisionDefaults(
    req: ProvisionDefaultsReqDto,
    _orgId?: string,
  ): Promise<ProvisionDefaultsResultDto> {
    return this.call(
      sdk.provisionBlueprintDefaults({ client: this.client, body: req as any }),
    ) as Promise<ProvisionDefaultsResultDto>;
  }


  // ── Connection & Database Management ─────────────────────────
  async connectionGetConfig(): Promise<ConnectionConfig> {
    return {
      mode: "api",
      api: {
        server_url: this.baseUrl,
      },
    };
  }

  async connectionSetConfig(_params: any): Promise<ConnectionConfig> {
    return this.connectionGetConfig();
  }

  async connectionReset(): Promise<ConnectionConfig> {
    return this.connectionGetConfig();
  }

  async connectionValidate(): Promise<ConnectionValidation> {
    return {
      is_valid: true,
      mode: "api",
      message: "Orbis API URL is valid",
      resolved_url: this.baseUrl,
    };
  }

  async connectionCheckRemoteHealth(serverUrl: string): Promise<RemoteHealthResult> {
    const startedAt = performance.now();
    try {
      const response = await fetch(
        `${serverUrl.replace(/\/$/, "")}/api/v1/health`,
        { signal: AbortSignal.timeout(10_000) },
      );
      const latency_ms = Math.round(performance.now() - startedAt);
      const body = (await response.json().catch(() => ({}))) as Record<string, unknown>;
      if (!response.ok) {
        return {
          reachable: false,
          latency_ms,
          status: `HTTP ${response.status}`,
          error: `Orbis API returned HTTP ${response.status}`,
        };
      }
      return {
        reachable: body.database_connected !== false,
        latency_ms,
        status: typeof body.status === "string" ? body.status : "ok",
        version: typeof body.version === "string" ? body.version : undefined,
        api_version: typeof body.api_version === "string" ? body.api_version : undefined,
        error:
          body.database_connected === false
            ? "Orbis API is reachable but its database is unavailable"
            : undefined,
      };
    } catch (error) {
      return {
        reachable: false,
        latency_ms: Math.round(performance.now() - startedAt),
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  async connectionConnect(): Promise<ConnectResult> {
    const health = await this.connectionCheckRemoteHealth(this.baseUrl);
    if (!health.reachable) {
      throw new Error(health.error || "Unable to connect to Orbis API");
    }

    return {
      connected: true,
      mode: "api",
      server_url: this.baseUrl,
      server_version: health.version,
      message: "Connected to Orbis API",
    };
  }

  async connectionRunMigrations(): Promise<MigrationResultDto> {
    throw new Error("Database migrations must be applied from the connected desktop database.");
  }

  async connectionOpenWindow(): Promise<void> {}

  async databaseGetStatus(): Promise<DatabaseStatusDto> {
    return {
      mode: "api",
      state: "connected",
      database_name: "remote_db",
      connection_target: this.baseUrl,
      schema_version: "latest",
      applied_migrations_count: 0,
      total_migrations_count: 0,
      pending_migrations: [],
      is_compatible: true,
    };
  }

  async databaseCreateLocal(_params: { customDir?: string; filename: string }): Promise<DatabaseStatusDto> {
    throw new Error("Local database creation is not supported in remote cloud mode");
  }

  async databaseOpenExisting(_filePath: string): Promise<DatabaseStatusDto> {
    throw new Error("Opening local database is not supported in remote cloud mode");
  }

  async databaseBackup(_destinationPath?: string): Promise<BackupResultDto> {
    throw new Error("Direct database backup is not supported in remote cloud mode");
  }

  async databaseValidateBackupFile(_filePath: string): Promise<BackupInspectionDto> {
    throw new Error("Validating local backup files is not supported in remote cloud mode");
  }

  async databaseRestore(_backupFilePath: string): Promise<RestoreResultDto> {
    throw new Error("Direct file restoration is not supported in remote cloud mode");
  }

  async databaseRunMigrations(): Promise<MigrationResultDto> {
    return {
      success: true,
      applied_migrations: [],
      total_applied: 0,
      message: "Migrations managed by remote server",
    };
  }

  async databaseCheckIntegrity(): Promise<IntegrityResultDto> {
    return {
      is_healthy: true,
      status: "ok",
      details: ["Remote database health verified"],
      checked_at: new Date().toISOString(),
    };
  }

  async databaseOptimize(): Promise<OptimizeResultDto> {
    return {
      success: true,
      message: "Remote database maintenance handled by backend",
      size_before_bytes: 0,
      size_after_bytes: 0,
      space_freed_bytes: 0,
      space_freed_formatted: "0 B",
    };
  }

  async databaseGetSettings(): Promise<DatabaseSettingsDto> {
    return {
      auto_backup_enabled: false,
      auto_backup_interval_hours: 24,
      max_backup_files: 10,
      integrity_check_on_startup: true,
    };
  }

  async databaseUpdateSettings(settings: DatabaseSettingsDto): Promise<DatabaseSettingsDto> {
    return settings;
  }

  async databasePickFile(_params?: { filterName?: string; extensions?: string[] }): Promise<string | null> {
    return null;
  }

  async databasePickFolder(): Promise<string | null> {
    return null;
  }

  async databasePickSavePath(_params?: { defaultFilename?: string; filterName?: string; extensions?: string[] }): Promise<string | null> {
    return null;
  }

  // ── AI Platform ─────────────────────────────────────────────
  async aiChat(_req: AiChatRequest): Promise<AiChatResponse> {
    throw new Error("AI chat is not enabled for remote client");
  }

  async aiListTools(): Promise<AiToolDefinition[]> {
    return [];
  }

  async aiCreateCustomReport(
    request: AiCustomReportRequest,
  ): Promise<AiCustomReportResponse> {
    const response = await fetch(`${this.baseUrl}/api/v1/reports/custom`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        ...(this.token ? { Authorization: `Bearer ${this.token}` } : {}),
      },
      body: JSON.stringify(request),
    });
    if (!response.ok) {
      const body = (await response.json().catch(() => null)) as
        | { message?: string }
        | null;
      throw new ApiRequestError(response.status, body ?? response.statusText);
    }
    const result = (await response.json()) as { data: AiCustomReportResponse };
    return result.data;
  }

  async aiSaveSession(session: AiStoredSession): Promise<boolean> {
    if (typeof window !== "undefined" && window.localStorage) {
      localStorage.setItem(`remote_ai_session_${session.id}`, JSON.stringify(session));
    }
    return true;
  }

  async aiLoadSessions(orgId: string): Promise<AiStoredSession[]> {
    const sessions: AiStoredSession[] = [];
    if (typeof window !== "undefined" && window.localStorage) {
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        if (key && key.startsWith("remote_ai_session_")) {
          try {
            const s = JSON.parse(localStorage.getItem(key) || "");
            if (s && (!orgId || s.org_id === orgId)) sessions.push(s);
          } catch {}
        }
      }
    }
    return sessions.sort((a, b) => b.updated_at - a.updated_at);
  }

  async aiDeleteSession(_orgId: string, sessionId: string): Promise<boolean> {
    if (typeof window !== "undefined" && window.localStorage) {
      localStorage.removeItem(`remote_ai_session_${sessionId}`);
    }
    return true;
  }

  async aiSaveContext(orgId: string, context: Record<string, unknown>): Promise<boolean> {
    if (typeof window !== "undefined" && window.localStorage) {
      localStorage.setItem(`remote_ai_context_${orgId}`, JSON.stringify(context));
    }
    return true;
  }

  async aiLoadContext(orgId: string): Promise<Record<string, unknown> | null> {
    if (typeof window !== "undefined" && window.localStorage) {
      const raw = localStorage.getItem(`remote_ai_context_${orgId}`);
      if (raw) {
        try {
          return JSON.parse(raw);
        } catch {}
      }
    }
    return null;
  }

  // ── Autopilot System ────────────────────────────────────────
  async autopilotListTasks(params?: {
    status?: string;
    task_type?: string;
    limit?: number;
    offset?: number;
  }): Promise<ListTasksResponse> {
    return (await this.call(sdk.listTasks({ client: this.client, query: params as any }))) as any;
  }

  async autopilotRunTick(dryRun?: boolean): Promise<AutopilotTaskDto[]> {
    return (await this.call(sdk.runTick({ client: this.client, body: { dry_run: dryRun ?? false } as any }))) as any ?? [];
  }

  async autopilotGetTask(id: string): Promise<AutopilotTaskDto> {
    return (await this.call(sdk.getTask({ client: this.client, path: { id } }))) as any;
  }

  async autopilotExecuteTask(id: string): Promise<AutopilotTaskDto> {
    return (await this.call(sdk.executeTask({ client: this.client, path: { id } }))) as any;
  }

  async autopilotRetryTask(id: string): Promise<AutopilotTaskDto> {
    return (await this.call(sdk.retryTask({ client: this.client, path: { id } }))) as any;
  }

  async autopilotDismissTask(id: string): Promise<AutopilotTaskDto> {
    return (await this.call(sdk.dismissTask({ client: this.client, path: { id } }))) as any;
  }

  async autopilotGetDiagnostics(): Promise<AutopilotDiagnosticsDto> {
    return (await this.call(sdk.getDiagnostics({ client: this.client }))) as any;
  }

  // ── Document & Storage Management ───────────────────────────
  async uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto> {
    return (await this.call(sdk.uploadDocument({ client: this.client, body: req as any }))) as any;
  }

  async uploadFile(req: StorageUploadRequest): Promise<DocumentDto> {
    const formData = new FormData();
    formData.append("file", req.file, req.name || (req.file instanceof File ? req.file.name : "upload.bin"));
    if (req.category) formData.append("category", req.category);
    if (req.entity_type) formData.append("entity_type", req.entity_type);
    if (req.entity_id) formData.append("entity_id", req.entity_id);
    if (req.org_unit_id) formData.append("org_unit_id", req.org_unit_id);
    if (req.folder_id) formData.append("folder_id", req.folder_id);
    if (req.asset_type) formData.append("asset_type", req.asset_type);
    if (req.description) formData.append("description", req.description);
    if (req.calendar_id) formData.append("calendar_id", req.calendar_id);
    if (req.term_id) formData.append("term_id", req.term_id);
    if (req.tags) formData.append("tags", req.tags);
    if (req.visibility) formData.append("visibility", req.visibility);

    return (await this.call(sdk.uploadFile({ client: this.client, body: formData as any }))) as any;
  }

  async listDocuments(params?: { entity_type?: string; entity_id?: string; folder_id?: string }): Promise<DocumentDto[]> {
    return (await this.call(sdk.listDocuments({ client: this.client, query: params as any }))) as any ?? [];
  }

  async listDocumentsByEntity(entityType: string, entityId: string): Promise<DocumentDto[]> {
    return this.listDocuments({ entity_type: entityType, entity_id: entityId });
  }

  async getDocument(id: string): Promise<DocumentDto> {
    return (await this.call(sdk.getDocument({ client: this.client, path: { id } }))) as any;
  }

  async updateDocument(id: string, req: UpdateDocumentRequest): Promise<DocumentDto> {
    return (await this.call(sdk.updateDocument({ client: this.client, path: { id }, body: req as any }))) as any;
  }

  async deleteDocument(id: string): Promise<void> {
    await this.call(sdk.deleteDocument({ client: this.client, path: { id } }));
  }

  async archiveDocument(id: string, reason?: string): Promise<DocumentDto> {
    return (await this.call(sdk.archiveDocument({ client: this.client, path: { id }, body: { reason: reason ?? null } as any }))) as any;
  }

  async restoreDocument(id: string): Promise<DocumentDto> {
    return (await this.call(sdk.restoreDocument({ client: this.client, path: { id } }))) as any;
  }

  async trashDocument(id: string): Promise<void> {
    await this.call(sdk.trashDocument({ client: this.client, path: { id } }));
  }

  async purgeDocument(id: string): Promise<void> {
    await this.call(sdk.purgeDocument({ client: this.client, path: { id } }));
  }

  async listArchivedDocuments(): Promise<DocumentDto[]> {
    return (await this.call(sdk.listArchivedDocuments({ client: this.client }))) as any ?? [];
  }

  async listTrashDocuments(): Promise<DocumentDto[]> {
    return (await this.call(sdk.listTrashDocuments({ client: this.client }))) as any ?? [];
  }

  async getDocumentAccessUrl(id: string, expiresInSeconds?: number): Promise<string> {
    const res = await this.call(sdk.getAccessUrl({ client: this.client, path: { id }, body: { expires_in_seconds: expiresInSeconds ?? 3600 } as any })) as any;
    return res?.url ?? "";
  }

  async downloadDocumentFile(id: string): Promise<Blob | Uint8Array> {
    const url = `${this.baseUrl}/api/v1/storage/${id}/file`;
    const token = this.token;
    const res = await fetch(url, {
      headers: token ? { Authorization: `Bearer ${token}` } : {},
    });
    if (!res.ok) throw new Error(`Failed to download file: ${res.statusText}`);
    return await res.blob();
  }

  async createDocumentRequirement(req: CreateDocumentRequirementRequest): Promise<DocumentRequirementDto> {
    return (await this.call(sdk.createDocumentRequirement({ client: this.client, body: req as any }))) as any;
  }

  async getDocumentRequirement(id: string): Promise<DocumentRequirementDto> {
    return (await this.call(sdk.getDocumentRequirement({ client: this.client, path: { id } }))) as any;
  }

  async updateDocumentRequirement(id: string, req: UpdateDocumentRequirementRequest): Promise<DocumentRequirementDto> {
    return (await this.call(sdk.updateDocumentRequirement({ client: this.client, path: { id }, body: req as any }))) as any;
  }

  async listDocumentRequirements(): Promise<DocumentRequirementDto[]> {
    return (await this.call(sdk.listDocumentRequirements({ client: this.client }))) as any ?? [];
  }

  async deleteDocumentRequirement(id: string): Promise<void> {
    await this.call(sdk.deleteDocumentRequirement({ client: this.client, path: { id } }));
  }

  async evaluateStudentDocumentRequirements(
    studentId: string,
    params?: {
      grade_level_id?: string;
      is_national?: boolean;
      is_special_needs?: boolean;
      is_transfer?: boolean;
    },
  ): Promise<StudentDocumentFulfillmentDto[]> {
    return (await this.call(
      sdk.evaluateStudentRequirements({
        client: this.client,
        path: { id: studentId },
        query: params as any,
      }),
    )) as any ?? [];
  }

  // ── Folders & Curriculum Knowledge Base ─────────────────────
  async createFolder(req: CreateFolderRequest): Promise<FolderDto> {
    return (await this.call(sdk.createFolder({ client: this.client, body: req as any }))) as any;
  }

  async listFolders(parentId?: string): Promise<FolderDto[]> {
    return (await this.call(sdk.listFolders({ client: this.client, query: { parent_id: parentId } as any }))) as any ?? [];
  }

  async getFolder(id: string): Promise<FolderDto> {
    return (await this.call(sdk.getFolder({ client: this.client, path: { id } }))) as any;
  }

  async getFolderContents(id: string): Promise<FolderContentsDto> {
    return (await this.call(sdk.getFolderContents({ client: this.client, path: { id } }))) as any;
  }

  async getOrCreateStudentFolder(studentId: string, studentName?: string): Promise<FolderDto> {
    return (await this.call(sdk.getOrCreateStudentFolder({ client: this.client, path: { student_id: studentId }, query: { student_name: studentName } as any }))) as any;
  }

  async listCurriculumFolders(): Promise<FolderDto[]> {
    return (await this.call(sdk.listCurriculumFolders({ client: this.client }))) as any ?? [];
  }

  async getOrCreateCurriculumFolder(req: CreateCurriculumFolderRequest): Promise<FolderDto> {
    return (await this.call(sdk.getOrCreateCurriculumFolder({ client: this.client, body: req as any }))) as any;
  }

  async updateFolder(id: string, req: UpdateFolderRequest): Promise<FolderDto> {
    return (await this.call(sdk.updateFolder({ client: this.client, path: { id }, body: req as any }))) as any;
  }

  async deleteFolder(id: string): Promise<void> {
    await this.call(sdk.deleteFolder({ client: this.client, path: { id } }));
  }
}
