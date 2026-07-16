/**
 * @orbis/sdk-core — Platform-agnostic SDK core.
 *
 * This package provides:
 * - Type definitions (mirroring Rust entities/contracts)
 * - Command interfaces (the "what" — platform adapters provide the "how")
 * - Error handling utilities
 * - Permission constants
 * - Event type constants
 *
 * Platform adapters:
 * - @orbis/sdk-desktop → Tauri IPC (invoke)
 * - @orbis/sdk (existing) → HTTP (fetch) via API Gateway
 */

// ── Types ───────────────────────────────────────────────────────
export {
  type AcademicYear,
  type AcademicYearInfo,
  type AcademicContextResponse,
  type PromotionJob,
  type PromotionDetail,
  type PromotionConfig,
  type ClassMapping,
  type OverrideRequest,
  type AcademicPolicy,
  type DocumentTemplate,
  type AutopilotStatus,
  type AutopilotTask,
  type AgentSession,
  type AgentActionLog,
} from './types/platform.js';

export type * from './types/index.js';
export {
  type AcademicPeriod,
  type Class,
  type Enrollment,
  type Subject,
  type OrgUnit,
  type ClassSubject,
  type StudentGuardian,
  type Notification,
  type Organization,
} from './types/academic.js';

export {
  type Document,
  type UploadDocumentPayload,
} from './types/document.js';

export {
  type Grade,
  type Attendance,
  type BehaviorRecord,
  type Assessment,
  type GradeReportCard,
  type AttendanceReport,
  type GradeAnalytics,
} from './types/assessment.js';

export {
  type Payment,
  type StudentFee,
  type FeeTemplate,
  type Treasury,
  type DiscountTemplate,
  type Account,
  type JournalEntry,
  type Currency,
} from './types/finance.js';

export {
  type TimeSlot,
  type ScheduledLesson,
  type Room,
  type SolveJob,
} from './types/scheduling.js';

export {
  type ExamSession,
  type ExamSubject,
  type ExamSchedule,
  type ExamHall,
  type SeatingNumber,
} from './types/exam.js';

export {
  type User,
  type UserRole,
  type UserSession,
  type Permission,
} from './types/iam.js';

// ── Commands ────────────────────────────────────────────────────
export type { OrbisCommands } from './commands/index.js';
export { type AcademicCommands } from './commands/academic.js';
export { type AssessmentCommands } from './commands/assessment.js';
export { type FinanceCommands } from './commands/finance.js';
export { type SchedulingCommands } from './commands/scheduling.js';
export { type ExamCommands } from './commands/exam.js';
export { type IamCommands, type PermissionRule } from './commands/iam.js';
export { type OrganizationCommands } from './commands/organization.js';
export { type DocforgeCommands } from './commands/docforge.js';

// ── Errors ──────────────────────────────────────────────────────
export { OrbisError, extractErrorMessage } from './errors.js';

// ── Permissions ─────────────────────────────────────────────────
export { Permissions } from './permissions.js';

// ── Events ──────────────────────────────────────────────────────
export { EventTypes } from './events.js';

// ── Shared aliases ──────────────────────────────────────────────
import type { StudentGuardian as _StudentGuardian } from './types/academic.js';
export type GuardianLink = _StudentGuardian;
export type JsonRecord = Record<string, any>;
