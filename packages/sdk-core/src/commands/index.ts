/**
 * OrbisCommands — the unified command interface.
 *
 * Every platform adapter must implement this interface:
 * - Desktop → @orbis/sdk-desktop (Tauri invoke)
 * - Web → @orbis/sdk (HTTP fetch)
 * - Mobile → @orbis/sdk-mobile (future)
 */
import type { AcademicCommands } from "./academic";
import type { AssessmentCommands } from "./assessment";
import type { FinanceCommands } from "./finance";
import type { SchedulingCommands } from "./scheduling";
import type { ExamCommands } from "./exam";
import type { IAMCommands } from "./iam";
import type { OrganizationCommands } from "./organization";

export interface OrbisCommands {
  academic: AcademicCommands;
  assessment: AssessmentCommands;
  finance: FinanceCommands;
  scheduling: SchedulingCommands;
  exam: ExamCommands;
  iam: IAMCommands;
  organization: OrganizationCommands;
}

export type { AcademicCommands } from "./academic";
export type { AssessmentCommands } from "./assessment";
export type { FinanceCommands } from "./finance";
export type { SchedulingCommands } from "./scheduling";
export type { ExamCommands } from "./exam";
export type { IAMCommands } from "./iam";
export type { OrganizationCommands } from "./organization";
