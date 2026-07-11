/**
 * Platform command interfaces — Desktop-specific operations.
 */
import type {
  AcademicYear, AcademicYearInfo, AcademicContextResponse,
  PromotionJob, PromotionDetail, PromotionConfig,
  OverrideRequest, CalendarEvent, OrgPolicy,
  DocumentTemplate, AutopilotStatus, AutopilotTask,
  AgentSession, AgentActionLog,
} from "../types";

export interface PlatformCommands {
  // ── Academic Years ────────────────────────────────────────────
  getCurrentYear(): Promise<AcademicYear>;
  listYears(orgId: string): Promise<AcademicYearInfo[]>;
  switchYear(yearId: string): Promise<void>;
  isYearArchived(): Promise<boolean>;
  getAcademicContext(orgId: string): Promise<AcademicContextResponse>;
  getSetupStatus(orgId: string): Promise<boolean>;

  // ── Promotion ─────────────────────────────────────────────────
  listPromotionJobs(orgId: string): Promise<PromotionJob[]>;
  getPromotionJob(jobId: string): Promise<PromotionJob>;
  getPromotionDetails(jobId: string): Promise<PromotionDetail[]>;
  promotionDryRun(orgId: string, config: PromotionConfig): Promise<PromotionJob>;
  executePromotion(jobId: string): Promise<PromotionJob>;
  rollbackPromotion(jobId: string, userId: string): Promise<void>;
  overridePromotionDecision(request: Record<string, unknown>): Promise<void>;
  getPromotionPolicy(orgId: string): Promise<Record<string, unknown>>;

  // ── Calendar ──────────────────────────────────────────────────
  listCalendarEvents(orgId: string, cycleId: string): Promise<CalendarEvent[]>;
  listPendingCalendarEvents(orgId: string): Promise<CalendarEvent[]>;
  getOrgPolicy(orgId: string): Promise<OrgPolicy>;

  // ── Overrides / Approvals ─────────────────────────────────────
  listOverrideRequests(orgId: string, params?: Record<string, unknown>): Promise<OverrideRequest[]>;
  getOverrideRequest(requestId: string): Promise<OverrideRequest>;
  getPendingApprovals(orgId: string, approverRole: string): Promise<OverrideRequest[]>;

  // ── Document Templates ────────────────────────────────────────
  listDocumentTemplates(orgId: string, params?: Record<string, unknown>): Promise<DocumentTemplate[]>;
  getDocumentTemplate(orgId: string, templateId: string): Promise<DocumentTemplate>;

  // ── AI Autopilot ──────────────────────────────────────────────
  getAutopilotStatus(orgId: string): Promise<AutopilotStatus>;
  getAutopilotTasks(orgId: string): Promise<AutopilotTask[]>;
  getAutopilotDiagnostics(orgId: string): Promise<Record<string, unknown>>;
  configureAutopilot(orgId: string, isEnabled: boolean, paused: boolean, mode: string): Promise<void>;
  runAutopilotTick(orgId: string): Promise<void>;

  // ── AI Agent ──────────────────────────────────────────────────
  listAgentSessions(orgId: string): Promise<AgentSession[]>;
  getAgentSessionLogs(orgId: string, sessionId: string): Promise<AgentActionLog[]>;
  approveAgentAction(orgId: string, actionLogId: string, approvedBy: string): Promise<void>;
  rejectAgentAction(orgId: string, actionLogId: string, rejectedBy: string, reason: string): Promise<void>;

  // ── Auth (desktop-specific) ───────────────────────────────────
  login(identifier: string, password: string): Promise<{ user: any; access_token: string }>;
  logout(token: string): Promise<void>;

  // ── Import/Export ─────────────────────────────────────────────
  importStudents(orgId: string, data: any): Promise<any>;
  importGrades(orgId: string, data: any): Promise<any>;
  exportStudents(orgId: string): Promise<any>;
  exportGrades(orgId: string): Promise<any>;

  // ── Audit ─────────────────────────────────────────────────────
  listAuditLogs(orgId: string): Promise<any[]>;

  // ── Enrollment (extended) ─────────────────────────────────────
  transitionEnrollmentStatus(orgId: string, enrollmentId: string, status: string): Promise<void>;

  // ── Guardians (extended) ──────────────────────────────────────
  createGuardianLink(data: Record<string, unknown>): Promise<any>;
  updateGuardianLink(linkId: string, data: Record<string, unknown>): Promise<any>;

  // ── Documents ─────────────────────────────────────────────────
  listDocumentsByOwner(orgId: string, ownerId: string): Promise<any[]>;
}
