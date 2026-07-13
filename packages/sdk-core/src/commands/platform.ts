/**
 * Platform command interfaces — Desktop-specific operations.
 */
import type {
  AcademicYear,
  AcademicYearInfo,
  AcademicContextResponse,
  PromotionJob,
  PromotionDetail,
  PromotionConfig,
  OverrideRequest,
  AcademicPolicy,
  DocumentTemplate,
  AutopilotStatus,
  AutopilotTask,
  AgentSession,
  AgentActionLog,
} from "../types";

export interface PlatformCommands {
  // ── Academic Years ────────────────────────────────────────────
  getCurrentYear(orgId: string): Promise<AcademicYear>;
  listYears(orgId: string): Promise<AcademicYearInfo[]>;
  switchYear(periodId: string): Promise<void>;
  isYearArchived(periodId: string): Promise<boolean>;
  getAcademicContext(orgId: string): Promise<AcademicContextResponse>;
  getSetupStatus(orgId: string): Promise<boolean>;
  initializeSchoolSetup(orgId: string, payload: any): Promise<void>;
  getPeriodTemplates(orgId: string): Promise<any[]>;

  // ── Promotion ─────────────────────────────────────────────────
  listPromotionJobs(orgId: string): Promise<PromotionJob[]>;
  getPromotionJob(jobId: string): Promise<PromotionJob>;
  getPromotionDetails(jobId: string): Promise<PromotionDetail[]>;
  promotionDryRun(
    orgId: string,
    config: PromotionConfig,
  ): Promise<PromotionJob>;
  executePromotion(jobId: string): Promise<PromotionJob>;
  rollbackPromotion(jobId: string, userId: string): Promise<void>;
  overridePromotionDecision(request: Record<string, unknown>): Promise<void>;
  getPromotionPolicy(orgId: string): Promise<Record<string, unknown>>;

  // ── Calendar / Periods ──────────────────────────────────────
  listCalendarEvents(orgId: string, periodId: string): Promise<any[]>;
  listPendingCalendarEvents(orgId: string): Promise<any[]>;
  getOrgPolicy(orgId: string): Promise<AcademicPolicy | null>;
  updateOrgPolicy(orgId: string, policy: Partial<AcademicPolicy>): Promise<void>;

  // ── Overrides / Approvals ─────────────────────────────────────
  listOverrideRequests(
    orgId: string,
    params?: Record<string, unknown>,
  ): Promise<OverrideRequest[]>;
  getOverrideRequest(requestId: string): Promise<OverrideRequest>;
  getPendingApprovals(
    orgId: string,
    approverRole: string,
  ): Promise<OverrideRequest[]>;
  processApproval(requestId: string, action: any): Promise<void>;
  cancelOverride(requestId: string, userId: string): Promise<void>;

  // ── Document Templates ────────────────────────────────────────
  listDocumentTemplates(
    orgId: string,
    params?: Record<string, unknown>,
  ): Promise<DocumentTemplate[]>;
  getDocumentTemplate(
    orgId: string,
    templateId: string,
  ): Promise<DocumentTemplate>;
  createDocumentTemplate(
    orgId: string,
    userId: string,
    data: any,
  ): Promise<DocumentTemplate>;
  deleteDocumentTemplate(orgId: string, templateId: string): Promise<void>;
  publishDocumentTemplate(
    orgId: string,
    templateId: string,
    userId: string,
  ): Promise<void>;
  archiveDocumentTemplate(orgId: string, templateId: string): Promise<void>;
  restoreDocumentTemplate(orgId: string, templateId: string): Promise<void>;
  updateDocumentTemplate(
    orgId: string,
    templateId: string,
    userId: string,
    data: any,
  ): Promise<DocumentTemplate>;

  // ── AI Autopilot ──────────────────────────────────────────────
  getAutopilotStatus(orgId: string): Promise<AutopilotStatus>;
  getAutopilotTasks(orgId: string): Promise<AutopilotTask[]>;
  getAutopilotDiagnostics(orgId: string): Promise<Record<string, unknown>>;
  configureAutopilot(
    orgId: string,
    isEnabled: boolean,
    paused: boolean,
    mode: string,
  ): Promise<void>;
  runAutopilotTick(orgId: string): Promise<void>;

  // ── AI Agent ──────────────────────────────────────────────────
  listAgentSessions(orgId: string): Promise<AgentSession[]>;
  getAgentSessionLogs(
    orgId: string,
    sessionId: string,
  ): Promise<AgentActionLog[]>;
  approveAgentAction(
    orgId: string,
    actionLogId: string,
    approvedBy: string,
  ): Promise<void>;
  rejectAgentAction(
    orgId: string,
    actionLogId: string,
    rejectedBy: string,
    reason: string,
  ): Promise<void>;

  // ── Auth (desktop-specific) ───────────────────────────────────
  login(
    identifier: string,
    password: string,
  ): Promise<{ user: any; access_token: string }>;
  logout(token: string): Promise<void>;

  // ── Audit ─────────────────────────────────────────────────────
  listAuditLogs(orgId: string): Promise<any[]>;
  getAllAuditLogs(orgId: string): Promise<any[]>;

  // ── Bulk Import ─────────────────────────────────────────────
  importStudents(orgId: string, data: any): Promise<any>;
  importGrades(orgId: string, data: any): Promise<any>;
  createBulkStudents(orgId: string, userId: string, data: any): Promise<any>;
  createBulkGrades(
    orgId: string,
    userId: string,
    periodId: string | null,
    data: any,
  ): Promise<any>;

  // ── Export ──────────────────────────────────────────────────
  exportStudents(orgId: string): Promise<any>;
  exportGrades(orgId: string): Promise<any>;

  // ── Enrollment (extended) ─────────────────────────────────────
  transitionEnrollmentStatus(
    orgId: string,
    enrollmentId: string,
    status: string,
  ): Promise<void>;

  // ── Guardians (extended) ──────────────────────────────────────
  createGuardianLink(data: Record<string, unknown>): Promise<any>;
  updateGuardianLink(
    linkId: string,
    data: Record<string, unknown>,
  ): Promise<any>;
  deleteGuardianLink(linkId: string): Promise<void>;

  // ── Documents ─────────────────────────────────────────────────
  listDocumentsByOwner(orgId: string, ownerId: string): Promise<any[]>;
}
