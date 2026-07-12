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
import type { IamCommands } from "./iam";
import type { OrganizationCommands } from "./organization";
import type { PlatformCommands } from "./platform";
import type { DocforgeCommands } from "./docforge";

export interface OrbisCommands {
  academic: AcademicCommands;
  assessment: AssessmentCommands;
  finance: FinanceCommands;
  scheduling: SchedulingCommands;
  exam: ExamCommands;
  iam: IamCommands;
  organization: OrganizationCommands;
  platform: PlatformCommands;
  docforge: DocforgeCommands;
}

export type { AcademicCommands } from "./academic";
export type { AssessmentCommands } from "./assessment";
export type { FinanceCommands } from "./finance";
export type { SchedulingCommands } from "./scheduling";
export type { ExamCommands } from "./exam";
export type { IamCommands } from "./iam";
export type { OrganizationCommands } from "./organization";
export type { PlatformCommands } from "./platform";
export type { DocforgeCommands } from "./docforge";
