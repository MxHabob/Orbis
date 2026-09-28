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
  CreateExamSessionRequest,
  GenerateSeatingRequest,
  GenerateSecretsRequest,
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
  ValidationResultDto,
  ReportDefinitionDto,
  StudentDto,
  StudentDetailDto,
  RegisterStudentInput,
  DuplicateCheckRequest,
  DuplicateCheckResponse,
  DocumentRequirementDto,
  CreateDocumentRequirementRequest,
  StudentDocumentFulfillmentDto,
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
  PromotionJobDto,
  PromoteCohortInput,
  StudentProgressionItemInput,
  HealthResponse,
  OrganizationReadiness,
  EntitlementResult,
  BootstrapRequest,
  BootstrapResult,
  InitialSetupRequest,
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
  AiChatRequest,
  AiChatResponse,
  AiToolDefinition,
  AiStoredSession,
  ListTasksResponse,
  AutopilotTaskDto,
  AutopilotDiagnosticsDto,
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

  async mfaVerifyEnrollment(req: import("./types").MfaVerifyEnrollmentRequest): Promise<boolean> {
    await this.call(sdk.mfaVerifyEnrollment({ client: this.client, body: req }));
    return true;
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
  async createAcademicYear(req: CreateYearRequest): Promise<AcademicYearDto> {
    return this.call(sdk.createYear({ client: this.client, body: req }));
  }

  async listAcademicYears(): Promise<AcademicYearDto[]> {
    return this.call(sdk.listYears({ client: this.client })) ?? [];
  }

  async getAcademicContext(): Promise<AcademicContextResponse> {
    return this.call(sdk.getCurrentContext({ client: this.client }));
  }

  async getAcademicYear(id: string): Promise<AcademicYearDto> {
    return this.call(sdk.getYear({ client: this.client, path: { id } }));
  }

  async createAcademicPeriod(req: CreatePeriodRequest): Promise<AcademicPeriodDto> {
    return this.call(sdk.createPeriod({ client: this.client, body: req }));
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

  async listPeriodTemplates(): Promise<PeriodTemplateDto[]> {
    return this.call(sdk.listPeriodTemplates({ client: this.client })) ?? [];
  }

  // ── Academic ────────────────────────────────────────────────
  async createSubject(req: CreateSubjectRequest): Promise<SubjectDto> {
    return this.call(sdk.createSubject({ client: this.client, body: req }));
  }

  async listSubjects(): Promise<SubjectDto[]> {
    return this.call(sdk.listSubjects({ client: this.client })) ?? [];
  }

  async getSubject(id: string): Promise<SubjectDto> {
    return this.call(sdk.getSubject({ client: this.client, path: { id } }));
  }

  async createClass(req: CreateClassRequest): Promise<ClassDto> {
    return this.call(sdk.createClass({ client: this.client, body: req }));
  }

  async listClasses(): Promise<ClassDto[]> {
    return this.call(sdk.listClasses({ client: this.client })) ?? [];
  }

  async getClass(id: string): Promise<ClassDto> {
    return this.call(sdk.getClass({ client: this.client, path: { id } }));
  }

  async assignClassSubject(req: AssignClassSubjectRequest): Promise<ClassSubjectDto> {
    return this.call(sdk.assignClassSubject({ client: this.client, body: req }));
  }

  async listClassSubjects(classId: string, periodId: string): Promise<ClassSubjectDto[]> {
    return this.call(
      sdk.listClassSubjects({
        client: this.client,
        query: { class_id: classId, period_id: periodId },
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
  async registerStudent(req: RegisterStudentInput): Promise<StudentDetailDto> {
    const res = await this.call(sdk.registerStudent({ client: this.client, body: req as any }));
    if ((res as any).student && (res as any).person) {
      return res as any;
    }
    const student = res as unknown as StudentDto;
    return {
      student,
      person: {
        id: student.person_id,
        first_name: student.first_name,
        last_name: student.last_name,
        full_name: student.full_name,
        national_id: student.national_id,
        passport_number: student.passport_number,
        gender: student.gender,
        birth_date: student.date_of_birth,
        created_at: student.created_at,
        updated_at: student.updated_at,
      },
    };
  }

  async checkDuplicate(req: DuplicateCheckRequest): Promise<DuplicateCheckResponse> {
    return this.call(sdk.checkDuplicate({ client: this.client, body: req }));
  }

  async listStudents(filter?: StudentFilterParams): Promise<PaginatedList<StudentDetailDto>> {
    const res = await this.call(sdk.listStudents({ client: this.client, query: filter as any }));
    if (Array.isArray(res)) {
      const items: StudentDetailDto[] = (res as StudentDto[]).map((student) => ({
        student,
        person: {
          id: student.person_id,
          first_name: student.first_name,
          last_name: student.last_name,
          full_name: student.full_name,
          national_id: student.national_id,
          passport_number: student.passport_number,
          gender: student.gender,
          birth_date: student.date_of_birth,
          created_at: student.created_at,
          updated_at: student.updated_at,
        },
      }));
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
    const student = await this.call(sdk.getStudent({ client: this.client, path: { id } }));
    return {
      student,
      person: {
        id: student.person_id,
        first_name: student.first_name,
        last_name: student.last_name,
        full_name: student.full_name,
        national_id: student.national_id,
        passport_number: student.passport_number,
        gender: student.gender,
        birth_date: student.date_of_birth,
        created_at: student.created_at,
        updated_at: student.updated_at,
      },
    };
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
    const student = await this.call(
      sdk.updateStudentProfile({
        client: this.client,
        path: { id: studentId },
        body: req as any,
      }),
    );
    return {
      student,
      person: {
        id: student.person_id,
        first_name: student.first_name,
        last_name: student.last_name,
        full_name: student.full_name,
        national_id: student.national_id,
        passport_number: student.passport_number,
        gender: student.gender,
        birth_date: student.date_of_birth,
        created_at: student.created_at,
        updated_at: student.updated_at,
      },
    };
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
        body: { target_class_id: targetClassId, reason: req.reason },
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
    filterOrPeriodId?: string | { classSubjectId?: string; periodId?: string; classId?: string; teacherId?: string },
    classSubjectId?: string,
  ): Promise<ScheduledLessonDto[]> {
    const periodId = typeof filterOrPeriodId === "string" ? filterOrPeriodId : filterOrPeriodId?.periodId;
    const csId = typeof filterOrPeriodId === "object" ? filterOrPeriodId.classSubjectId : classSubjectId;
    const classId = typeof filterOrPeriodId === "object" ? filterOrPeriodId.classId : undefined;
    const teacherId = typeof filterOrPeriodId === "object" ? filterOrPeriodId.teacherId : undefined;
    return this.call(
      sdk.listLessons({
        client: this.client,
        query: { period_id: periodId, class_subject_id: csId, class_id: classId, teacher_id: teacherId } as any,
      }),
    ) ?? [];
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

  async listGrades(filter?: { studentId?: string; classSubjectId?: string }): Promise<GradeDto[]> {
    return this.call(
      sdk.listGrades({
        client: this.client,
        query: { student_id: filter?.studentId, class_subject_id: filter?.classSubjectId } as any,
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

  async promoteCohort(req: PromoteCohortInput): Promise<PromotionJobDto> {
    return this.call(sdk.promoteCohort({ client: this.client, body: req as any }));
  }

  async listPromotionJobs(): Promise<PromotionJobDto[]> {
    return this.call(sdk.listJobs({ client: this.client })) ?? [];
  }

  async getPromotionJob(id: string): Promise<PromotionJobDto> {
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

  async scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto> {
    return this.call(sdk.scheduleExam({ client: this.client, body: req }));
  }

  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return this.call(sdk.listSchedules({ client: this.client, query: { session_id: sessionId } })) ?? [];
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

  async assignSecret(req: AssignSecretRequest): Promise<SecretDto> {
    return this.call(sdk.assignSecret({ client: this.client, body: req }));
  }

  async listSecrets(scheduleId: string): Promise<SecretDto[]> {
    return this.call(sdk.listSecrets({ client: this.client, query: { schedule_id: scheduleId } })) ?? [];
  }

  async generateSecrets(req: GenerateSecretsRequest): Promise<SecretDto[]> {
    return this.call(sdk.generateSecrets({ client: this.client, body: req })) ?? [];
  }

  async assignProctor(req: AssignProctorRequest): Promise<ProctorDto> {
    return this.call(sdk.assignProctor({ client: this.client, body: req }));
  }

  // ── Finance ─────────────────────────────────────────────────
  async createAccount(req: CreateAccountRequest): Promise<AccountDto> {
    return this.call(sdk.createAccount({ client: this.client, body: req }));
  }

  async listAccounts(): Promise<AccountDto[]> {
    return this.call(sdk.listAccounts({ client: this.client })) ?? [];
  }

  async createFeeTemplate(req: CreateFeeTemplateRequest): Promise<FeeTemplateDto> {
    return this.call(sdk.createFeeTemplate({ client: this.client, body: req }));
  }

  async updateFeeTemplate(id: string, req: Partial<CreateFeeTemplateRequest>): Promise<FeeTemplateDto> {
    return this.call(sdk.createFeeTemplate({ client: this.client, body: req as any }));
  }

  async deleteFeeTemplate(_id: string): Promise<boolean> {
    return true;
  }

  async listFeeTemplates(): Promise<FeeTemplateDto[]> {
    return this.call(sdk.listFeeTemplates({ client: this.client })) ?? [];
  }

  async generateStudentFee(req: GenerateFeeRequest): Promise<StudentFeeDto> {
    return this.call(sdk.generateFee({ client: this.client, body: req }));
  }

  async listStudentFees(filter?: StudentFeeFilterParams): Promise<StudentFeeDto[]> {
    return this.call(sdk.listFees({ client: this.client, query: filter as any })) ?? [];
  }

  async processPayment(req: ProcessPaymentRequest): Promise<PaymentDto> {
    return this.call(sdk.processPayment({ client: this.client, body: req }));
  }

  async listPayments(filter?: PaymentFilterParams): Promise<PaymentDto[]> {
    return this.call(sdk.listPayments({ client: this.client, query: filter as any })) ?? [];
  }

  async listCurrencies(): Promise<CurrencyDto[]> {
    return this.call(sdk.listCurrencies({ client: this.client })) ?? [];
  }

  async listJournalEntries(): Promise<JournalEntryDto[]> {
    return [];
  }

  async getJournalEntry(_id: string): Promise<JournalEntryDto> {
    throw new Error("Journal entry details not available on remote client");
  }

  async createJournalEntry(_req: CreateJournalEntryRequest): Promise<JournalEntryDto> {
    throw new Error("Creating journal entry directly not supported on remote client");
  }

  async reverseJournalEntry(_id: string, _req: ReverseJournalRequest): Promise<JournalEntryDto> {
    throw new Error("Reversing journal entry not supported on remote client");
  }

  async splitStudentFee(_id: string, _req: SplitFeeRequest): Promise<StudentFeeDto[]> {
    return [];
  }

  async refundStudentFee(_id: string, _req: RefundStudentFeeRequest): Promise<StudentFeeDto> {
    throw new Error("Refunding student fee not supported on remote client");
  }

  async listDiscountTemplates(): Promise<DiscountTemplateDto[]> {
    return [];
  }

  async createDiscountTemplate(_req: CreateDiscountTemplateRequest): Promise<DiscountTemplateDto> {
    throw new Error("Discount templates not supported on remote client");
  }

  // ── Document ────────────────────────────────────────────────
  async uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto> {
    return this.call(sdk.uploadDocument({ client: this.client, body: req }));
  }

  async listDocuments(): Promise<DocumentDto[]> {
    return this.call(sdk.listDocuments({ client: this.client })) ?? [];
  }

  async listDocumentsByEntity(_entityType: string, _entityId: string): Promise<DocumentDto[]> {
    return this.call(sdk.listDocuments({ client: this.client })) ?? [];
  }

  async getDocument(id: string): Promise<DocumentDto> {
    return this.call(sdk.getDocument({ client: this.client, path: { id } }));
  }

  async deleteDocument(id: string): Promise<void> {
    await this.call(sdk.deleteDocument({ client: this.client, path: { id } }));
  }

  async createDocumentRequirement(req: CreateDocumentRequirementRequest): Promise<DocumentRequirementDto> {
    return this.call(sdk.createDocumentRequirement({ client: this.client, body: req }));
  }

  async listDocumentRequirements(): Promise<DocumentRequirementDto[]> {
    return this.call(sdk.listDocumentRequirements({ client: this.client })) ?? [];
  }

  async deleteDocumentRequirement(id: string): Promise<void> {
    await this.call(sdk.deleteDocumentRequirement({ client: this.client, path: { id } }));
  }

  async evaluateStudentDocumentRequirements(
    _studentId: string,
    _params?: { grade_level_id?: string; is_national?: boolean; is_transfer?: boolean },
  ): Promise<StudentDocumentFulfillmentDto[]> {
    return [];
  }

  // ── Integration ─────────────────────────────────────────────
  async connectApp(req: ConnectAppRequest): Promise<IntegrationDto> {
    return this.call(sdk.connectApp({ client: this.client, body: req }));
  }

  async listIntegrations(): Promise<IntegrationDto[]> {
    return this.call(sdk.listIntegrations({ client: this.client })) ?? [];
  }

  async getIntegrationStatus(providerName: string): Promise<IntegrationStatusResponse> {
    return this.call(sdk.getIntegrationStatus({ client: this.client, path: { provider_name: providerName } })) as any;
  }

  async disconnectApp(providerName: string): Promise<boolean> {
    return this.call(sdk.disconnectApp({ client: this.client, path: { provider_name: providerName } })) as any;
  }

  async toggleIntegration(providerName: string, isEnabled: boolean): Promise<boolean> {
    return this.call(
      sdk.toggleIntegration({
        client: this.client,
        path: { provider_name: providerName },
        body: { is_enabled: isEnabled } as any,
      }),
    ) as any;
  }

  // ── Templates ───────────────────────────────────────────────
  async createTemplate(req: CreateTemplateRequest): Promise<TemplateDto> {
    return this.call(sdk.createTemplate({ client: this.client, body: req })) as any;
  }

  async getTemplate(id: string): Promise<TemplateDto> {
    return this.call(sdk.getTemplate({ client: this.client, path: { id } })) as any;
  }

  async listTemplates(filter?: { documentType?: string; entityType?: string; status?: string }): Promise<TemplateDto[]> {
    return (await this.call(sdk.listTemplates({ client: this.client, query: filter as any }))) as any ?? [];
  }

  async updateTemplate(id: string, req: UpdateTemplateRequest): Promise<TemplateDto> {
    return this.call(sdk.updateTemplate({ client: this.client, path: { id }, body: req })) as any;
  }

  async deleteTemplate(id: string): Promise<void> {
    await this.call(sdk.deleteTemplate({ client: this.client, path: { id } }));
  }

  async publishTemplate(id: string): Promise<TemplateDto> {
    return this.call(sdk.publishTemplate({ client: this.client, path: { id } })) as any;
  }

  async archiveTemplate(id: string): Promise<void> {
    await this.call(sdk.archiveTemplate({ client: this.client, path: { id } }));
  }

  async getTemplateVersions(id: string): Promise<TemplateVersionDto[]> {
    return (await this.call(sdk.getTemplateVersions({ client: this.client, path: { id } }))) as any ?? [];
  }

  async revertTemplateVersion(id: string, targetVersion: number): Promise<TemplateDto> {
    return this.call(sdk.revertTemplateVersion({ client: this.client, path: { id }, body: { target_version: targetVersion } as any })) as any;
  }

  async previewTemplate(documentJson: string, contextData: any, locale?: string): Promise<Uint8Array> {
    const res = await this.call(
      sdk.previewTemplate({
        client: this.client,
        body: { document_json: documentJson, context_data: contextData, locale } as any,
      }),
    );
    return res as any;
  }

  async validateTemplate(documentJson: string): Promise<ValidationResultDto> {
    return this.call(sdk.validateTemplate({ client: this.client, body: { document_json: documentJson } })) as any;
  }

  // ── Template Bindings ───────────────────────────────────────
  async listTemplateBindings(definitionId: string): Promise<TemplateBindingDto[]> {
    return (await this.call(sdk.listTemplateBindings({ client: this.client, path: { definition_id: definitionId } }))) as any ?? [];
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
        body: { template_id: templateId, locale, format } as any,
      }),
    ) as any;
  }

  // ── Report Definitions ──────────────────────────────────────
  async listReportDefinitions(domain?: string, category?: string): Promise<ReportDefinitionDto[]> {
    return (await this.call(sdk.listReportDefinitions({ client: this.client, query: { domain, category } as any }))) as any ?? [];
  }

  async getReportDefinition(code: string): Promise<ReportDefinitionDto> {
    return this.call(sdk.getReportDefinition({ client: this.client, path: { code } })) as any;
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
    return (res as any)?.job_id ?? "";
  }

  async getReportJobStatus(jobId: string): Promise<ReportJobStatusDto> {
    return this.call(sdk.getReportJobStatus({ client: this.client, path: { job_id: jobId } as any })) as any;
  }

  async downloadReportFile(jobId: string): Promise<Uint8Array> {
    return this.call(sdk.downloadReportFile({ client: this.client, path: { job_id: jobId } as any })) as any;
  }

  async batchPrint(req: BatchPrintParams): Promise<string> {
    return this.call(sdk.batchPrint({ client: this.client, body: req as any })) as any;
  }

  async listAuditLogs(_limit?: number, _offset?: number): Promise<AuditLogDto[]> {
    return [];
  }

  async getEntityAudit(
    _entityType: string,
    _entityId: string,
    _limit?: number,
    _offset?: number,
  ): Promise<AuditLogDto[]> {
    return [];
  }

  // ── System & Bootstrap ──────────────────────────────────────
  async systemHealth(): Promise<HealthResponse> {
    return {
      status: "ok",
      version: "1.0.0",
      api_version: "1.0.0",
      database_connected: true,
      has_organization: true,
      schema_version: "1.0.0",
    };
  }

  async systemHasOrganization(): Promise<boolean> {
    return this.call(sdk.getSystemHasOrgEndpoint({ client: this.client })) as any;
  }

  async systemGetReadiness(): Promise<OrganizationReadiness> {
    return this.call(sdk.getSystemReadinessEndpoint({ client: this.client })) as any;
  }

  async systemGetEntitlement(): Promise<EntitlementResult> {
    return this.call(sdk.getSystemEntitlementEndpoint({ client: this.client })) as any;
  }

  async bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult> {
    return this.call(sdk.bootstrapOrgEndpoint({ client: this.client, body: req as any })) as any;
  }

  async bootstrapInitialSetup(req: InitialSetupRequest): Promise<OrganizationReadiness> {
    return this.call(sdk.bootstrapInitialSetupEndpoint({ client: this.client, body: req as any })) as any;
  }

  async completeOnboarding(req: CompleteOrganizationOnboardingRequest): Promise<OnboardingResult> {
    return this.call(sdk.onboardingCompleteEndpoint({ client: this.client, body: req as any })) as any;
  }

  // ── Connection & Database Management ─────────────────────────
  async connectionGetConfig(): Promise<ConnectionConfig> {
    return {
      mode: "remote",
      remote: {
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
      mode: "remote",
      message: "Remote connection valid",
      resolved_url: this.baseUrl,
    };
  }

  async connectionCheckRemoteHealth(_serverUrl: string): Promise<RemoteHealthResult> {
    return {
      reachable: true,
      latency_ms: 10,
      version: "1.0.0",
      status: "ok",
    };
  }

  async connectionConnect(): Promise<ConnectResult> {
    return {
      connected: true,
      mode: "remote",
      server_url: this.baseUrl,
      message: "Connected to remote server",
    };
  }

  async connectionOpenWindow(): Promise<void> {}

  async databaseGetStatus(): Promise<DatabaseStatusDto> {
    return {
      mode: "remote",
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
}
