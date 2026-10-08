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
  UpdateOrgUnitRequest,
  OrganizationSettingsDto,
  DashboardCountsDto,
  AcademicYearDto,
  AcademicPeriodDto,
  AcademicContextResponse,
  PeriodTemplateDto,
  CreateYearRequest,
  UpdateYearRequest,
  CreatePeriodRequest,
  UpdatePeriodRequest,
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
  UpdateStudentStatusInput,
  UpdateStudentProfileInput,
  ChangeStudyModeInput,
  TransferClassInput,
  PaginatedList,
  ReportJobStatusDto,
  BatchPrintParams,
  JournalEntryDto,
  CreateJournalEntryRequest,
  ReverseJournalRequest,
  SplitFeeRequest,
  RefundStudentFeeRequest,
  DiscountTemplateDto,
  CreateDiscountTemplateRequest,
  StudentFeeFilterParams,
  PaymentFilterParams,
  StatementResponseDto,
  StatementFilterParams,
  ProcessDisbursementRequest,
  CreateSubAccountRequest,
  SettleGatewayPayoutRequest,
  PostPayrollRequest,
  DisbursePayrollRequest,
  PrerequisiteCheckResultDto,
  EmployeeDetailDto,
  AccountProvisioningResultDto,
  RegisterEmployeeResponseDto,
  PaginatedEmployeesDto,
  RegisterEmployeeRequest,
  UpdateEmployeeRequest,
  EmployeeFilterInput,
  ProvisionEmployeeAccountRequest,
  GroupDetailDto,
  GroupMemberInfoDto,
  CreateGroupRequest,
  UpdateGroupRequest,
  BulkAssignGroupRequest,
  ApprovalStepDto,
  OverrideRequestDto,
  SubmitOverrideRequestInput,
  ApproveOverrideInput,
  RejectOverrideInput,
  PromotionJobDto,
  PromotionDetailDto,
  StudentProgressionItemInput,
  PromoteCohortInput,
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
  AiChatRequest,
  AiChatResponse,
  AiStoredSession,
  AiToolDefinition,
  AutopilotDiagnosticsDto,
  AutopilotTaskDto,
  BulkRegisterEmployeeCommandItem,
  BulkRegisterEmployeeCommandResultDto,
  GoogleAuthRequest,
  ListTasksResponse,
  MfaChallengeRequest,
  MfaEnrollRequest,
  MfaEnrollResponse,
  MfaVerifyEnrollmentRequest,
  UserDeviceDto,
  UserIdentityDto,
  ProvisionUserRequest,
  OnboardingStateDto,
  ProvisionDefaultsReqDto,
  ProvisionDefaultsResultDto,
  LicenseStateDto,
  ActivateKeyRequest,
  ActivationResultDto,
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
    req?: MfaEnrollRequest,
  ): Promise<MfaEnrollResponse> {
    return unwrap(await invoke("auth_mfa_enroll", { request: req ?? {} }));
  }
  async mfaVerifyEnrollment(
    req: MfaVerifyEnrollmentRequest,
  ): Promise<boolean> {
    return unwrap(await invoke("auth_mfa_verify_enrollment", { request: req }));
  }
  async mfaChallenge(
    req: MfaChallengeRequest,
  ): Promise<LoginResponse> {
    return unwrap(await invoke("auth_mfa_challenge", { request: req }));
  }
  async linkGoogle(
    req: GoogleAuthRequest,
  ): Promise<UserIdentityDto> {
    return unwrap(await invoke("auth_link_google", { request: req }));
  }
  async loginWithGoogle(
    req: GoogleAuthRequest,
  ): Promise<LoginResponse> {
    return unwrap(await invoke("auth_login_google", { request: req }));
  }
  async listDevices(): Promise<UserDeviceDto[]> {
    return unwrap(await invoke("auth_list_devices"));
  }
  async revokeDevice(id: string): Promise<boolean> {
    return unwrap(await invoke("auth_revoke_device", { id }));
  }

  // ── IAM ─────────────────────────────────────────────────────
  async registerUser(req: RegisterUserRequest): Promise<UserDto> {
    return unwrap(await invoke("iam_register_user", { request: req }));
  }
  async provisionUser(req: ProvisionUserRequest): Promise<AccountProvisioningResultDto> {
    return unwrap(await invoke("iam_provision_user", { request: req }));
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
  async resetUserPassword(userId: string): Promise<AccountProvisioningResultDto> {
    return unwrap(await invoke("iam_reset_user_password", { userId }));
  }
  async listRoles(): Promise<RoleDto[]> {
    return unwrap(await invoke("iam_list_roles"));
  }
  async saveRole(req: SaveRoleRequest): Promise<RoleDto> {
    return unwrap(await invoke("iam_save_role", { request: req }));
  }
  async deleteRole(id: string): Promise<boolean> {
    return unwrap(await invoke("iam_delete_role", { roleId: id }));
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
  async updateOrgUnit(id: string, req: UpdateOrgUnitRequest): Promise<OrgUnitDto> {
    return unwrap(await invoke("org_update_unit", { id, request: req }));
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
  async updateAcademicYear(
    id: string,
    req: UpdateYearRequest,
  ): Promise<AcademicYearDto> {
    return unwrap(await invoke("calendar_update_year", { id, request: req }));
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
  async activateAcademicYear(id: string): Promise<AcademicYearDto> {
    return unwrap(await invoke("calendar_activate_year", { id }));
  }
  async closeAcademicYear(id: string): Promise<AcademicYearDto> {
    return unwrap(await invoke("calendar_close_year", { id }));
  }
  async archiveAcademicYear(id: string): Promise<AcademicYearDto> {
    return unwrap(await invoke("calendar_archive_year", { id }));
  }
  async createAcademicPeriod(
    req: CreatePeriodRequest,
  ): Promise<AcademicPeriodDto> {
    return unwrap(await invoke("calendar_create_period", { request: req }));
  }
  async updateAcademicPeriod(
    id: string,
    req: UpdatePeriodRequest,
  ): Promise<AcademicPeriodDto> {
    return unwrap(await invoke("calendar_update_period", { id, request: req }));
  }
  async deleteAcademicPeriod(id: string): Promise<boolean> {
    return unwrap(await invoke("calendar_delete_period", { id }));
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
  async checkDuplicate(req: DuplicateCheckRequest): Promise<DuplicateCheckResponse> {
    return unwrap(await invoke("student_check_duplicate", { request: req }));
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
  async updateStudentProfile(req: UpdateStudentProfileInput): Promise<StudentDetailDto> {
    return unwrap(await invoke("student_update_profile", { request: req }));
  }
  async changeStudyMode(req: ChangeStudyModeInput): Promise<EnrollmentDto> {
    return unwrap(await invoke("student_change_study_mode", { request: req }));
  }
  async transferClass(req: TransferClassInput): Promise<EnrollmentDto> {
    return unwrap(await invoke("student_transfer_class", { request: req }));
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
  async removeGuardian(studentId: string, guardianId: string): Promise<boolean> {
    return unwrap(await invoke("student_remove_guardian", { studentId, guardianId }));
  }
  async deleteStudent(id: string): Promise<boolean> {
    return unwrap(await invoke("student_delete_student", { id }));
  }
  async restoreStudent(id: string): Promise<StudentDto> {
    return unwrap(await invoke("student_restore_student", { id }));
  }
  async bulkRegisterStudents( items: BulkRegisterStudentCommandItem[], options?: { atomic?: boolean; dryRun?: boolean }): Promise<BulkRegisterStudentCommandResultDto> {
    return unwrap(await invoke("student_bulk_register", {
      items,
      atomic: options?.atomic ?? true,
      dryRun: options?.dryRun ?? false,
    }));
  }

  // ── Employee Affairs ────────────────────────────────────────
  async registerEmployee(req: RegisterEmployeeRequest): Promise<RegisterEmployeeResponseDto> {
    return unwrap(await invoke("employee_register_employee", { request: req }));
  }
  async listEmployees(filter?: EmployeeFilterInput): Promise<PaginatedEmployeesDto> {
    return unwrap(await invoke("employee_list_employees", { filter: filter ?? {} }));
  }
  async getEmployee(id: string): Promise<EmployeeDetailDto> {
    return unwrap(await invoke("employee_get_employee", { id }));
  }
  async getEmployeeStats(): Promise<Record<string, number>> {
    return unwrap(await invoke("employee_get_employee_stats"));
  }
  async updateEmployee(id: string, req: UpdateEmployeeRequest): Promise<EmployeeDetailDto> {
    return unwrap(await invoke("employee_update_employee", { id, request: req }));
  }
  async updateEmployeeStatus(id: string, status: string): Promise<EmployeeDetailDto> {
    return unwrap(await invoke("employee_update_employee_status", { id, status }));
  }
  async deleteEmployee(id: string): Promise<boolean> {
    return unwrap(await invoke("employee_delete_employee", { id }));
  }
  async restoreEmployee(id: string): Promise<EmployeeDetailDto> {
    return unwrap(await invoke("employee_restore_employee", { id }));
  }
  async provisionEmployeeAccount(id: string, req: ProvisionEmployeeAccountRequest): Promise<AccountProvisioningResultDto> {
    return unwrap(await invoke("employee_provision_account", { id, request: req }));
  }
  async assignEmployeeGroups(id: string, groupIds: string[]): Promise<boolean> {
    return unwrap(await invoke("employee_assign_groups", { id, groupIds }));
  }
  async bulkRegisterEmployees(
    items: BulkRegisterEmployeeCommandItem[],
    options?: { atomic?: boolean; dryRun?: boolean }
  ): Promise<BulkRegisterEmployeeCommandResultDto> {
    return unwrap(await invoke("employee_bulk_register", {
      items,
      atomic: options?.atomic ?? true,
      dryRun: options?.dryRun ?? false,
    }));
  }

  // ── User Groups ─────────────────────────────────────────────
  async createGroup(req: CreateGroupRequest): Promise<GroupDetailDto> {
    return unwrap(await invoke("group_create_group", { request: req }));
  }
  async listGroups(): Promise<GroupDetailDto[]> {
    return unwrap(await invoke("group_list_groups"));
  }
  async getGroup(id: string): Promise<GroupDetailDto> {
    return unwrap(await invoke("group_get_group", { id }));
  }
  async updateGroup(id: string, req: UpdateGroupRequest): Promise<GroupDetailDto> {
    return unwrap(await invoke("group_update_group", { id, request: req }));
  }
  async deleteGroup(id: string): Promise<boolean> {
    return unwrap(await invoke("group_delete_group", { id }));
  }
  async getGroupMembers(id: string): Promise<GroupMemberInfoDto[]> {
    return unwrap(await invoke("group_get_members", { id }));
  }
  async addGroupMember(id: string, userId: string): Promise<boolean> {
    return unwrap(await invoke("group_add_member", { id, userId }));
  }
  async removeGroupMember(id: string, userId: string): Promise<boolean> {
    return unwrap(await invoke("group_remove_member", { id, userId }));
  }
  async bulkAssignGroup(req: BulkAssignGroupRequest): Promise<number> {
    return unwrap(await invoke("group_bulk_assign", { request: req }));
  }
  async setGroupPermissions(id: string, permissions: string[]): Promise<boolean> {
    return unwrap(await invoke("group_set_permissions", { id, permissions }));
  }

  // ── Scheduling ──────────────────────────────────────────────
  async createRoom(req: CreateRoomRequest): Promise<RoomDto> {
    return unwrap(await invoke("scheduling_create_room", { request: req }));
  }
  async listRooms(): Promise<RoomDto[]> {
    return unwrap(await invoke("scheduling_list_rooms"));
  }
  async getRoom(id: string): Promise<RoomDto> {
    return unwrap(await invoke("scheduling_get_room", { id }));
  }
  async updateRoom(id: string, req: UpdateRoomRequest): Promise<RoomDto> {
    return unwrap(await invoke("scheduling_update_room", { id, request: req }));
  }
  async deleteRoom(id: string): Promise<void> {
    await invoke("scheduling_delete_room", { id });
  }
  async createTimeSlot(req: CreateTimeSlotRequest): Promise<TimeSlotDto> {
    return unwrap(
      await invoke("scheduling_create_time_slot", { request: req }),
    );
  }
  async batchCreateTimeSlots(req: BatchCreateTimeSlotsRequest): Promise<TimeSlotDto[]> {
    return unwrap(
      await invoke("scheduling_batch_create_time_slots", { request: req }),
    );
  }
  async listTimeSlots(periodId?: string): Promise<TimeSlotDto[]> {
    return unwrap(
      await invoke("scheduling_list_time_slots", {
        periodId: periodId || "",
      }),
    );
  }
  async getTimeSlot(id: string): Promise<TimeSlotDto> {
    return unwrap(await invoke("scheduling_get_time_slot", { id }));
  }
  async updateTimeSlot(id: string, req: UpdateTimeSlotRequest): Promise<TimeSlotDto> {
    return unwrap(await invoke("scheduling_update_time_slot", { id, request: req }));
  }
  async deleteTimeSlot(id: string): Promise<void> {
    await invoke("scheduling_delete_time_slot", { id });
  }
  async scheduleLesson(
    req: ScheduleLessonRequest,
  ): Promise<ScheduledLessonDto> {
    return unwrap(await invoke("scheduling_schedule_lesson", { request: req }));
  }
  async bulkScheduleLessons(
    req: BulkScheduleLessonsRequest,
  ): Promise<ScheduledLessonDto[]> {
    return unwrap(await invoke("scheduling_bulk_schedule_lessons", { request: req }));
  }
  async deleteScheduledLesson(id: string): Promise<void> {
    await invoke("scheduling_delete_scheduled_lesson", { id });
  }
  async listScheduledLessons(
    filterOrPeriodId?:
      | string
      | {
          classSubjectId?: string;
          periodId?: string;
          classId?: string;
          teacherId?: string;
        },
    classSubjectId?: string,
  ): Promise<ScheduledLessonDto[]> {
    const filter =
      typeof filterOrPeriodId === "string"
        ? { period_id: filterOrPeriodId, class_subject_id: classSubjectId }
        : {
            period_id: filterOrPeriodId?.periodId,
            class_subject_id: filterOrPeriodId?.classSubjectId,
            class_id: filterOrPeriodId?.classId,
            teacher_id: filterOrPeriodId?.teacherId,
          };
    return unwrap(
      await invoke("scheduling_list_lessons", {
        filter,
      }),
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
          max_score: req.max_score,
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
  async batchRecordGrades(
    req: BatchRecordGradesRequest,
  ): Promise<BatchRecordGradesResponseDto> {
    return unwrap(
      await invoke("assessment_batch_record_grades", { request: req }),
    );
  }
  async deleteGrade(id: string): Promise<void> {
    unwrap(await invoke("assessment_delete_grade", { id }));
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
    req: CalculateGpaRequest,
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
  async getStudentTermResults(
    studentId: string,
    periodId: string,
    classId?: string,
  ): Promise<StudentTermReportCard> {
    return unwrap(
      await invoke("assessment_get_student_term_results", {
        studentId,
        periodId,
        classId: classId ?? null,
      }),
    );
  }
  async getStudentAnnualResults(
    studentId: string,
    yearId: string,
    classId?: string,
  ): Promise<StudentAnnualReportCard> {
    return unwrap(
      await invoke("assessment_get_student_annual_results", {
        studentId,
        yearId,
        classId: classId ?? null,
      }),
    );
  }
  async getClassTermResults(
    classId: string,
    periodId: string,
  ): Promise<StudentTermReportCard[]> {
    return unwrap(
      await invoke("assessment_get_class_term_results", {
        classId,
        periodId,
      }),
    );
  }
  async getClassAnnualResults(
    classId: string,
    yearId: string,
  ): Promise<StudentAnnualReportCard[]> {
    return unwrap(
      await invoke("assessment_get_class_annual_results", {
        classId,
        yearId,
      }),
    );
  }
  async markAttendance(req: MarkAttendanceRequest): Promise<AttendanceDto> {
    return unwrap(await invoke("assessment_mark_attendance", { request: req }));
  }
  async batchMarkAttendance(
    req: BatchMarkAttendanceRequest,
  ): Promise<BatchMarkAttendanceResponseDto> {
    return unwrap(
      await invoke("assessment_batch_mark_attendance", { request: req }),
    );
  }
  async deleteAttendance(id: string): Promise<void> {
    unwrap(await invoke("assessment_delete_attendance", { id }));
  }
  async listAttendance(
    classId: string,
    date: string,
  ): Promise<AttendanceDto[]> {
    return unwrap(
      await invoke("assessment_list_attendance", { classId, date }),
    );
  }
  async listStudentAttendance(studentId: string): Promise<AttendanceDto[]> {
    return unwrap(
      await invoke("assessment_list_student_attendance", { studentId }),
    );
  }
  async recordBehavior(req: RecordBehaviorRequest): Promise<BehaviorRecordDto> {
    return unwrap(
      await invoke("assessment_record_behavior", { request: req }),
    );
  }
  async deleteBehavior(id: string): Promise<void> {
    unwrap(await invoke("assessment_delete_behavior", { id }));
  }
  async listStudentBehaviorRecords(
    studentId: string,
  ): Promise<BehaviorRecordDto[]> {
    return unwrap(
      await invoke("assessment_list_student_behavior_records", { studentId }),
    );
  }
  async listAllBehavior(): Promise<BehaviorRecordDto[]> {
    return unwrap(await invoke("assessment_list_all_behavior"));
  }

  // ── Student Cohort Progression ──────────────────────────────
  async evaluateCohort(
    sourcePeriodId: string,
  ): Promise<StudentProgressionItemInput[]> {
    return unwrap(
      await invoke("progression_evaluate_cohort", {
        sourcePeriodId,
      }),
    );
  }
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
  async updateExamSession(
    id: string,
    req: UpdateExamSessionRequest,
  ): Promise<ExamSessionDto> {
    return unwrap(await invoke("exam_update_session", { id, request: req }));
  }
  async deleteExamSession(id: string): Promise<void> {
    unwrap(await invoke("exam_delete_session", { id }));
  }
  async scheduleExam(req: ScheduleExamRequest): Promise<ExamScheduleDto> {
    return unwrap(await invoke("exam_schedule_exam", { request: req }));
  }
  async listExamSchedules(sessionId: string): Promise<ExamScheduleDto[]> {
    return unwrap(await invoke("exam_list_schedules", { sessionId }));
  }
  async deleteExamSchedule(id: string): Promise<void> {
    unwrap(await invoke("exam_delete_schedule", { id }));
  }
  async assignSeating(req: AssignSeatingRequest): Promise<SeatingDto> {
    return unwrap(await invoke("exam_assign_seating", { request: req }));
  }
  async listSeating(sessionId: string): Promise<SeatingDto[]> {
    return unwrap(await invoke("exam_list_seating", { sessionId }));
  }
  async generateSeating(req: GenerateSeatingRequest): Promise<SeatingDto[]> {
    return unwrap(await invoke("exam_generate_seating", { request: req }));
  }
  async deleteSeating(id: string): Promise<void> {
    unwrap(await invoke("exam_delete_seating", { id }));
  }
  async assignSecret(req: AssignSecretRequest): Promise<SecretDto> {
    return unwrap(await invoke("exam_assign_secret", { request: req }));
  }
  async listSecrets(scheduleId: string): Promise<SecretDto[]> {
    return unwrap(await invoke("exam_list_secrets", { scheduleId }));
  }
  async generateSecrets(req: GenerateSecretsRequest): Promise<SecretDto[]> {
    return unwrap(await invoke("exam_generate_secrets", { request: req }));
  }
  async deleteSecret(id: string): Promise<void> {
    unwrap(await invoke("exam_delete_secret", { id }));
  }
  async assignProctor(req: AssignProctorRequest): Promise<ProctorDto> {
    return unwrap(await invoke("exam_assign_proctor", { request: req }));
  }
  async listProctors(scheduleId: string): Promise<ProctorDto[]> {
    return unwrap(await invoke("exam_list_proctors", { scheduleId }));
  }
  async deleteProctor(id: string): Promise<void> {
    unwrap(await invoke("exam_delete_proctor", { id }));
  }

  // ── Finance ─────────────────────────────────────────────────
  async createAccount(req: CreateAccountRequest): Promise<AccountDto> {
    return unwrap(await invoke("finance_create_account", { request: req }));
  }
  async listAccounts(): Promise<AccountDto[]> {
    return unwrap(await invoke("finance_list_accounts"));
  }
  async getAccount(id: string): Promise<AccountDto> {
    return unwrap(await invoke("finance_get_account", { id }));
  }
  async updateAccount(id: string, req: UpdateAccountRequest): Promise<AccountDto> {
    return unwrap(await invoke("finance_update_account", { id, request: req }));
  }
  async deleteAccount(id: string): Promise<boolean> {
    return unwrap(await invoke("finance_delete_account", { id }));
  }
  async createFeeTemplate(
    req: CreateFeeTemplateRequest,
  ): Promise<FeeTemplateDto> {
    return unwrap(
      await invoke("finance_create_fee_template", { request: req }),
    );
  }
  async getFeeTemplate(id: string): Promise<FeeTemplateDto> {
    return unwrap(await invoke("finance_get_fee_template", { id }));
  }
  async updateFeeTemplate(
    id: string,
    req: UpdateFeeTemplateRequest,
  ): Promise<FeeTemplateDto> {
    return unwrap(
      await invoke("finance_update_fee_template", { request: { id, ...req } }),
    );
  }
  async deleteFeeTemplate(id: string): Promise<boolean> {
    return unwrap(await invoke("finance_delete_fee_template", { id }));
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
  async deleteStudentFee(id: string): Promise<boolean> {
    return unwrap(await invoke("finance_delete_student_fee", { id }));
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
  async deletePayment(id: string): Promise<boolean> {
    return unwrap(await invoke("finance_delete_payment", { id }));
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
  async createJournalEntry(
    req: CreateJournalEntryRequest,
  ): Promise<JournalEntryDto> {
    return unwrap(
      await invoke("finance_create_journal_entry", { request: req }),
    );
  }
  async reverseJournalEntry(
    id: string,
    req: ReverseJournalRequest,
  ): Promise<JournalEntryDto> {
    return unwrap(
      await invoke("finance_reverse_journal_entry", { id, request: req }),
    );
  }
  async deleteJournalEntry(id: string): Promise<boolean> {
    return unwrap(await invoke("finance_delete_journal_entry", { id }));
  }
  async splitStudentFee(
    id: string,
    req: SplitFeeRequest,
  ): Promise<StudentFeeDto[]> {
    return unwrap(
      await invoke("finance_split_student_fee", { id, request: req }),
    );
  }
  async refundStudentFee(
    id: string,
    req: RefundStudentFeeRequest,
  ): Promise<StudentFeeDto> {
    return unwrap(
      await invoke("finance_refund_student_fee", { id, request: req }),
    );
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
  async deleteDiscountTemplate(id: string): Promise<boolean> {
    return unwrap(await invoke("finance_delete_discount_template", { id }));
  }
  async getAccountStatement(
    filter: StatementFilterParams,
  ): Promise<StatementResponseDto> {
    return unwrap(
      await invoke("finance_get_account_statement", { filter }),
    );
  }
  async processDisbursement(
    req: ProcessDisbursementRequest,
  ): Promise<JournalEntryDto> {
    return unwrap(
      await invoke("finance_process_disbursement", { request: req }),
    );
  }
  async createSubAccount(
    req: CreateSubAccountRequest,
  ): Promise<AccountDto> {
    return unwrap(
      await invoke("finance_create_sub_account", { request: req }),
    );
  }
  async settleGatewayPayout(
    req: SettleGatewayPayoutRequest,
  ): Promise<JournalEntryDto> {
    return unwrap(
      await invoke("finance_settle_gateway_payout", { request: req }),
    );
  }
  async postPayrollAccrual(
    req: PostPayrollRequest,
  ): Promise<JournalEntryDto> {
    return unwrap(
      await invoke("finance_post_payroll_accrual", { request: req }),
    );
  }
  async disbursePayroll(
    req: DisbursePayrollRequest,
  ): Promise<JournalEntryDto> {
    return unwrap(
      await invoke("finance_disburse_payroll", { request: req }),
    );
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
    document_type?: string;
    entityType?: string;
    entity_type?: string;
    status?: string;
  }): Promise<TemplateDto[]> {
    return unwrap(
      await invoke("template_list", {
        documentType: filter?.documentType ?? filter?.document_type,
        entityType: filter?.entityType ?? filter?.entity_type,
        status: filter?.status,
      }),
    );
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
  async validateTemplate(documentJson: string): Promise<ValidationResultDto> {
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
  async listAuditLogs(limit?: number, offset?: number): Promise<AuditLogDto[]> {
    return unwrap(await invoke("audit_list_logs", { limit, offset }));
  }
  async getEntityAudit(
    entityType: string,
    entityId: string,
    limit?: number,
    offset?: number,
  ): Promise<AuditLogDto[]> {
    try {
      return unwrap(
        await invoke("get_entity_audit", { entityType, entityId, limit, offset }),
      );
    } catch {
      const logs = await this.listAuditLogs(limit ?? 50, offset ?? 0);
      return logs.filter(
        (l) =>
          l.entity_type === entityType && (!entityId || l.entity_id === entityId),
      );
    }
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
  async systemGetAppInfo(): Promise<AppInfo> {
    return invoke<AppInfo>("system_get_app_info");
  }
  async systemGetChangelog(): Promise<ReleaseChangelogItem[]> {
    return invoke<ReleaseChangelogItem[]>("system_get_changelog");
  }
  async systemCheckForUpdates(): Promise<UpdateCheckResult> {
    return invoke<UpdateCheckResult>("system_check_for_updates");
  }
  async systemInstallUpdate(): Promise<void> {
    return invoke<void>("system_install_update");
  }
  async openChangelogWindow(): Promise<void> {
    return invoke<void>("open_changelog_window");
  }
  async openAgentWindow(): Promise<void> {
    return invoke<void>("open_agent_window");
  }

  // ── License & Activation ────────────────────────────────
  async licenseGetState(): Promise<LicenseStateDto> {
    return invoke<LicenseStateDto>("license_get_state");
  }
  async licenseActivateKey(req: ActivateKeyRequest): Promise<ActivationResultDto> {
    return invoke<ActivationResultDto>("license_activate_key", { request: req });
  }
  async licenseStartTrial(): Promise<LicenseStateDto> {
    return invoke<LicenseStateDto>("license_start_trial");
  }
  async licenseDeactivate(): Promise<LicenseStateDto> {
    return invoke<LicenseStateDto>("license_deactivate");
  }
  async licenseRefresh(): Promise<LicenseStateDto> {
    return invoke<LicenseStateDto>("license_refresh");
  }
  async licenseCheckFeature(featureKey: string): Promise<boolean> {
    return invoke<boolean>("license_check_feature", { featureKey });
  }
  async licenseGetInstallationId(): Promise<string> {
    return invoke<string>("license_get_installation_id");
  }

  async bootstrapDetectEnvironment(): Promise<EnvironmentDetectionResult> {
    return invoke<EnvironmentDetectionResult>("bootstrap_detect_environment");
  }
  async bootstrapGenerateSlug(name: string, nameAr?: string): Promise<SlugGenerationResult> {
    return invoke<SlugGenerationResult>("bootstrap_generate_slug", { name, nameAr: nameAr || null });
  }
  async bootstrapGetStatus(): Promise<BootstrapStatusResponse> {
    return invoke<BootstrapStatusResponse>("bootstrap_get_status");
  }
  async bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult> {
    return invoke<BootstrapResult>("bootstrap_execute", { request: req });
  }
  async completeOnboarding(req: CompleteOrganizationOnboardingRequest): Promise<OnboardingResult> {
    return invoke<OnboardingResult>("onboarding_complete", { request: req });
  }
  async onboardingGetState(orgId?: string): Promise<OnboardingStateDto> {
    return unwrap(await invoke("onboarding_get_state", { orgId: orgId ?? null }));
  }
  async onboardingProvisionDefaults(
    req: ProvisionDefaultsReqDto,
    orgId?: string,
  ): Promise<ProvisionDefaultsResultDto> {
    return unwrap(
      await invoke("onboarding_provision_defaults", {
        request: req,
        orgId: orgId ?? null,
      }),
    );
  }

  // ── Connection & Database Management ─────────────────────────
  async connectionGetConfig(): Promise<ConnectionConfig> {
    return invoke<ConnectionConfig>("connection_get_config");
  }
  async connectionSetConfig(params: {
    mode: ConnectionMode;
    serverUrl?: string;
    dbFilename?: string;
    customPath?: string;
    host?: string;
    port?: number;
    databaseName?: string;
    username?: string;
    password?: string;
    sslMode?: string;
    walMode?: boolean;
    busyTimeoutMs?: number;
  }): Promise<ConnectionConfig> {
    return invoke<ConnectionConfig>("connection_set_config", {
      mode: params.mode,
      serverUrl: params.serverUrl ?? null,
      dbFilename: params.dbFilename ?? null,
      customPath: params.customPath ?? null,
      host: params.host ?? null,
      port: params.port ?? null,
      databaseName: params.databaseName ?? null,
      username: params.username ?? null,
      password: params.password ?? null,
      sslMode: params.sslMode ?? null,
      walMode: params.walMode ?? null,
      busyTimeoutMs: params.busyTimeoutMs ?? null,
    });
  }
  async connectionReset(): Promise<ConnectionConfig> {
    return invoke<ConnectionConfig>("connection_reset");
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

  async databaseGetStatus(): Promise<DatabaseStatusDto> {
    return unwrap(await invoke<{ data: DatabaseStatusDto }>("database_get_status"));
  }
  async databaseCreateLocal(params: { customDir?: string; filename: string }): Promise<DatabaseStatusDto> {
    return unwrap(await invoke<{ data: DatabaseStatusDto }>("database_create_local", {
      customDir: params.customDir ?? null,
      filename: params.filename,
    }));
  }
  async databaseOpenExisting(filePath: string): Promise<DatabaseStatusDto> {
    return unwrap(await invoke<{ data: DatabaseStatusDto }>("database_open_existing", { filePath }));
  }
  async databaseBackup(destinationPath?: string): Promise<BackupResultDto> {
    return unwrap(await invoke<{ data: BackupResultDto }>("database_backup", {
      destinationPath: destinationPath ?? null,
    }));
  }
  async databaseValidateBackupFile(filePath: string): Promise<BackupInspectionDto> {
    return unwrap(await invoke<{ data: BackupInspectionDto }>("database_validate_backup_file", { filePath }));
  }
  async databaseRestore(backupFilePath: string): Promise<RestoreResultDto> {
    return unwrap(await invoke<{ data: RestoreResultDto }>("database_restore", { backupFilePath }));
  }
  async databaseRunMigrations(): Promise<MigrationResultDto> {
    return unwrap(await invoke<{ data: MigrationResultDto }>("database_run_migrations"));
  }
  async databaseCheckIntegrity(): Promise<IntegrityResultDto> {
    return unwrap(await invoke<{ data: IntegrityResultDto }>("database_check_integrity"));
  }
  async databaseOptimize(): Promise<OptimizeResultDto> {
    return unwrap(await invoke<{ data: OptimizeResultDto }>("database_optimize"));
  }
  async databaseGetSettings(): Promise<DatabaseSettingsDto> {
    return unwrap(await invoke<{ data: DatabaseSettingsDto }>("database_get_settings"));
  }
  async databaseUpdateSettings(settings: DatabaseSettingsDto): Promise<DatabaseSettingsDto> {
    return unwrap(await invoke<{ data: DatabaseSettingsDto }>("database_update_settings", { settings }));
  }
  async databasePickFile(params?: { filterName?: string; extensions?: string[] }): Promise<string | null> {
    return invoke<string | null>("database_pick_file", {
      filterName: params?.filterName ?? null,
      extensions: params?.extensions ?? null,
    });
  }
  async databasePickFolder(): Promise<string | null> {
    return invoke<string | null>("database_pick_folder");
  }
  async databasePickSavePath(params?: { defaultFilename?: string; filterName?: string; extensions?: string[] }): Promise<string | null> {
    return invoke<string | null>("database_pick_save_path", {
      defaultFilename: params?.defaultFilename ?? null,
      filterName: params?.filterName ?? null,
      extensions: params?.extensions ?? null,
    });
  }

  // ── AI Platform ─────────────────────────────────────────────
  async aiChat(request: AiChatRequest): Promise<AiChatResponse> {
    return unwrap(await invoke("ai_chat", { request }));
  }
  async aiListTools(): Promise<AiToolDefinition[]> {
    return unwrap(await invoke("ai_list_tools"));
  }
  async aiSaveSession(session: AiStoredSession): Promise<boolean> {
    return unwrap(await invoke("ai_save_session", { session }));
  }
  async aiLoadSessions(orgId: string): Promise<AiStoredSession[]> {
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
  }): Promise<ListTasksResponse> {
    return unwrap(
      await invoke("autopilot_list_tasks", {
        status: params?.status ?? null,
        taskType: params?.task_type ?? null,
        limit: params?.limit ?? null,
        offset: params?.offset ?? null,
      }),
    );
  }

  async autopilotRunTick(dryRun?: boolean): Promise<AutopilotTaskDto[]> {
    return unwrap(
      await invoke("autopilot_run_tick", {
        dryRun: dryRun ?? false,
      }),
    );
  }

  async autopilotGetTask(id: string): Promise<AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_get_task", { id }));
  }

  async autopilotExecuteTask(id: string): Promise<AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_execute_task", { id }));
  }

  async autopilotRetryTask(id: string): Promise<AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_retry_task", { id }));
  }

  async autopilotDismissTask(id: string): Promise<AutopilotTaskDto> {
    return unwrap(await invoke("autopilot_dismiss_task", { id }));
  }

  async autopilotGetDiagnostics(): Promise<AutopilotDiagnosticsDto> {
    return unwrap(await invoke("autopilot_get_diagnostics"));
  }

  // ── Document & Storage Management ───────────────────────────
  async uploadDocument(req: UploadDocumentRequest): Promise<DocumentDto> {
    return unwrap(await invoke("document_upload", { request: req }));
  }

  async uploadFile(req: StorageUploadRequest): Promise<DocumentDto> {
    const arrayBuffer = await req.file.arrayBuffer();
    const data = Array.from(new Uint8Array(arrayBuffer));
    const mime_type = req.file.type || "application/octet-stream";
    const name = req.name || (req.file instanceof File ? req.file.name : "upload.bin");

    return unwrap(
      await invoke("storage_upload_file", {
        request: {
          name,
          mime_type,
          data,
          category: req.category ?? null,
          entity_type: req.entity_type ?? null,
          entity_id: req.entity_id ?? null,
          org_unit_id: req.org_unit_id ?? null,
          folder_id: req.folder_id ?? null,
          asset_type: req.asset_type ?? null,
          description: req.description ?? null,
          academic_year_id: req.academic_year_id ?? null,
          term_id: req.term_id ?? null,
          tags: req.tags ?? null,
          visibility: req.visibility ?? null,
        },
      }),
    );
  }

  async listDocuments(params?: { entity_type?: string; entity_id?: string; folder_id?: string }): Promise<DocumentDto[]> {
    if (params?.folder_id) {
      const contents = await this.getFolderContents(params.folder_id);
      return contents.documents;
    }
    if (params?.entity_type && params?.entity_id) {
      return this.listDocumentsByEntity(params.entity_type, params.entity_id);
    }
    return unwrap(await invoke("document_list"));
  }

  async listDocumentsByEntity(entityType: string, entityId: string): Promise<DocumentDto[]> {
    return unwrap(
      await invoke("document_list_by_entity", {
        entityType,
        entityId,
      }),
    );
  }

  async getDocument(id: string): Promise<DocumentDto> {
    return unwrap(await invoke("document_get", { id }));
  }

  async updateDocument(id: string, req: UpdateDocumentRequest): Promise<DocumentDto> {
    return unwrap(await invoke("document_update", { id, request: req }));
  }

  async deleteDocument(id: string): Promise<void> {
    await invoke("document_delete", { id });
  }

  async archiveDocument(id: string, reason?: string): Promise<DocumentDto> {
    return unwrap(await invoke("document_archive", { id, reason: reason ?? null }));
  }

  async restoreDocument(id: string): Promise<DocumentDto> {
    return unwrap(await invoke("document_restore", { id }));
  }

  async trashDocument(id: string): Promise<void> {
    await invoke("document_trash", { id });
  }

  async purgeDocument(id: string): Promise<void> {
    await invoke("document_purge", { id });
  }

  async listArchivedDocuments(): Promise<DocumentDto[]> {
    return unwrap(await invoke("document_list_archived"));
  }

  async listTrashDocuments(): Promise<DocumentDto[]> {
    return unwrap(await invoke("document_list_trash"));
  }

  async getDocumentAccessUrl(id: string, expiresInSeconds?: number): Promise<string> {
    return unwrap(await invoke("storage_get_url", { id, expiresInSeconds: expiresInSeconds ?? null }));
  }

  async downloadDocumentFile(id: string): Promise<Blob | Uint8Array> {
    const res: { document: DocumentDto; data: number[] } = unwrap(
      await invoke("storage_download_file", { id }),
    );
    const uint8 = new Uint8Array(res.data);
    return new Blob([uint8], { type: res.document.mime_type || "application/octet-stream" });
  }

  async createDocumentRequirement(req: CreateDocumentRequirementRequest): Promise<DocumentRequirementDto> {
    return unwrap(await invoke("document_create_requirement", { request: req }));
  }

  async getDocumentRequirement(id: string): Promise<DocumentRequirementDto> {
    return unwrap(await invoke("document_get_requirement", { id }));
  }

  async updateDocumentRequirement(id: string, req: UpdateDocumentRequirementRequest): Promise<DocumentRequirementDto> {
    return unwrap(await invoke("document_update_requirement", { id, request: req }));
  }

  async listDocumentRequirements(): Promise<DocumentRequirementDto[]> {
    return unwrap(await invoke("document_list_requirements"));
  }

  async deleteDocumentRequirement(id: string): Promise<void> {
    await invoke("document_delete_requirement", { id });
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
    return unwrap(
      await invoke("document_evaluate_student_requirements", {
        studentId,
        gradeLevelId: params?.grade_level_id ?? null,
        isNational: params?.is_national ?? true,
        isSpecialNeeds: params?.is_special_needs ?? false,
        isTransfer: params?.is_transfer ?? false,
      }),
    );
  }

  // ── Folders & Curriculum Knowledge Base ─────────────────────
  async createFolder(req: CreateFolderRequest): Promise<FolderDto> {
    return unwrap(await invoke("folder_create", { request: req }));
  }

  async listFolders(parentId?: string): Promise<FolderDto[]> {
    return unwrap(await invoke("folder_list", { parentId: parentId ?? null }));
  }

  async getFolder(id: string): Promise<FolderDto> {
    return unwrap(await invoke("folder_get", { id }));
  }

  async getFolderContents(id: string): Promise<FolderContentsDto> {
    return unwrap(await invoke("folder_get_contents", { id }));
  }

  async getOrCreateStudentFolder(studentId: string, studentName?: string): Promise<FolderDto> {
    return unwrap(
      await invoke("folder_get_or_create_student", {
        studentId,
        studentName: studentName ?? null,
      }),
    );
  }

  async listCurriculumFolders(): Promise<FolderDto[]> {
    return unwrap(await invoke("folder_list_curriculum"));
  }

  async getOrCreateCurriculumFolder(req: CreateCurriculumFolderRequest): Promise<FolderDto> {
    return unwrap(await invoke("folder_get_or_create_curriculum", { request: req }));
  }

  async updateFolder(id: string, req: UpdateFolderRequest): Promise<FolderDto> {
    return unwrap(await invoke("folder_update", { id, request: req }));
  }

  async deleteFolder(id: string): Promise<void> {
    await invoke("folder_delete", { id });
  }
}
