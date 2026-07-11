/**
 * OrbisCommands — the unified command interface.
 *
 * Every platform adapter must implement this interface:
 * - Desktop → @orbis/sdk-desktop (Tauri invoke)
 * - Web → @orbis/sdk (HTTP fetch)
 */
import type { AcademicCommands } from "./academic";
import type { AssessmentCommands } from "./assessment";
import type { FinanceCommands } from "./finance";
import type { SchedulingCommands } from "./scheduling";
import type { ExamCommands } from "./exam";
import type { IAMCommands } from "./iam";
import type { OrganizationCommands } from "./organization";
import type { PlatformCommands } from "./platform";
import type { DocforgeCommands } from "./docforge";
import type { PermissionsCommands } from "./permissions";
import type { BulkImportCommands } from "./bulkImport";
import type { AuditLogsCommands } from "./auditLogs";

export interface OrbisCommands {
  academic: AcademicCommands;
  assessment: AssessmentCommands;
  finance: FinanceCommands;
  scheduling: SchedulingCommands;
  exam: ExamCommands;
  iam: IAMCommands;
  organization: OrganizationCommands;
  platform: PlatformCommands;
  docforge: DocforgeCommands;
  permissions: PermissionsCommands;
  bulkImport: BulkImportCommands;
  auditLogs: AuditLogsCommands;
}

export type { AcademicCommands } from "./academic";
export type { AssessmentCommands } from "./assessment";
export type { FinanceCommands } from "./finance";
export type { SchedulingCommands } from "./scheduling";
export type { ExamCommands } from "./exam";
export type { IAMCommands } from "./iam";
export type { OrganizationCommands } from "./organization";
export type { PlatformCommands } from "./platform";
export type { DocforgeCommands } from "./docforge";
export type { PermissionsCommands } from "./permissions";
export type { BulkImportCommands } from "./bulkImport";
export type { AuditLogsCommands } from "./auditLogs";
