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
  getCurrentYear(): Promise<AcademicYear>;
  listYears(): Promise<AcademicYearInfo[]>;
  switchYear(periodId: string): Promise<void>;
  isYearArchived(periodId: string): Promise<boolean>;
  getAcademicContext(): Promise<AcademicContextResponse>;
  getSetupStatus(): Promise<boolean>;
  initializeSchoolSetup(payload: any): Promise<void>;
  getPeriodTemplates(): Promise<any[]>;

  // ── Promotion ─────────────────────────────────────────────────
  listPromotionJobs(): Promise<PromotionJob[]>;
  getPromotionJob(jobId: string): Promise<PromotionJob>;
  getPromotionDetails(jobId: string): Promise<PromotionDetail[]>;
  promotionDryRun(config: PromotionConfig): Promise<PromotionJob>;
  executePromotion(jobId: string): Promise<PromotionJob>;
  rollbackPromotion(jobId: string, userId: string): Promise<void>;
  overridePromotionDecision(request: Record<string, unknown>): Promise<void>;
  getPromotionPolicy(): Promise<Record<string, unknown>>;

  // ── Calendar / Periods ──────────────────────────────────────
  listCalendarEvents(periodId: string): Promise<any[]>;
  listPendingCalendarEvents(): Promise<any[]>;
  createCalendarEvent(data: any): Promise<any>;
  executeCalendarEvent(eventId: string): Promise<any>;
  deleteCalendarEvent(eventId: string): Promise<any>;
  generateDefaultCalendar(periodId: string): Promise<any>;
  getOrgPolicy(): Promise<AcademicPolicy | null>;
  updateOrgPolicy(policy: Partial<AcademicPolicy>): Promise<void>;

  // ── Overrides / Approvals ─────────────────────────────────────
  listOverrideRequests(
    params?: Record<string, unknown>,
  ): Promise<OverrideRequest[]>;
  getOverrideRequest(requestId: string): Promise<OverrideRequest>;
  getPendingApprovals(approverRole: string): Promise<OverrideRequest[]>;
  processApproval(requestId: string, action: any): Promise<void>;
  cancelOverride(requestId: string, userId: string): Promise<void>;

  // ── Document Templates ────────────────────────────────────────
  listDocumentTemplates(
    params?: Record<string, unknown>,
  ): Promise<DocumentTemplate[]>;
  getDocumentTemplate(templateId: string): Promise<DocumentTemplate>;
  createDocumentTemplate(userId: string, data: any): Promise<DocumentTemplate>;
  deleteDocumentTemplate(templateId: string): Promise<void>;
  publishDocumentTemplate(templateId: string, userId: string): Promise<void>;
  archiveDocumentTemplate(templateId: string): Promise<void>;
  restoreDocumentTemplate(templateId: string): Promise<void>;
  updateDocumentTemplate(
    templateId: string,
    userId: string,
    data: any,
  ): Promise<DocumentTemplate>;

  // ── AI Autopilot ──────────────────────────────────────────────
  getAutopilotTasks(): Promise<AutopilotTask[]>;
  getAutopilotDiagnostics(): Promise<Record<string, unknown>>;
  runAutopilotTick(): Promise<void>;

  // ── AI Agent ──────────────────────────────────────────────────
  listAgentSessions(): Promise<AgentSession[]>;
  getAgentSessionLogs(sessionId: string): Promise<AgentActionLog[]>;
  approveAgentAction(actionLogId: string, approvedBy: string): Promise<void>;
  rejectAgentAction(
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
  listAuditLogs(): Promise<any[]>;
  getAllAuditLogs(): Promise<any[]>;

  // ── Bulk Import ─────────────────────────────────────────────
  importStudents(data: any): Promise<any>;
  importGrades(data: any): Promise<any>;
  createBulkStudents(userId: string, data: any): Promise<any>;
  createBulkGrades(
    userId: string,
    periodId: string | null,
    data: any,
  ): Promise<any>;

  // ── Export ──────────────────────────────────────────────────
  exportStudents(): Promise<any>;
  exportGrades(): Promise<any>;

  // ── Enrollment (extended) ─────────────────────────────────────
  transitionEnrollmentStatus(
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
  listDocumentsByOwner(ownerId: string): Promise<any[]>;
}
