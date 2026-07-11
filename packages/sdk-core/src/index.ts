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
export type * from "./types/index";
export {
  type AcademicCycle, type Class, type Enrollment, type Subject,
  type OrgUnit, type ClassSubject, type StudentGuardian, type Document,
  type Notification, type Organization,
} from "./types/academic";

export {
  type Grade, type Attendance, type BehaviorRecord, type Assessment,
  type GradeReportCard, type AttendanceReport, type GradeAnalytics,
} from "./types/assessment";

export {
  type Payment, type StudentFee, type FeeTemplate, type Treasury,
  type DiscountTemplate, type LedgerAccount, type JournalEntry, type Currency,
} from "./types/finance";

export {
  type TimeSlot, type ScheduledLesson, type Room, type SolveJob,
} from "./types/scheduling";

export {
  type ExamSession, type ExamSubject, type ExamSchedule,
  type ExamHall, type SeatingNumber,
} from "./types/exam";

export {
  type User, type UserRole, type UserSession, type Permission,
} from "./types/iam";

// ── Commands ────────────────────────────────────────────────────
export type { OrbisCommands } from "./commands/index";
export { type AcademicCommands } from "./commands/academic";
export { type AssessmentCommands } from "./commands/assessment";
export { type FinanceCommands } from "./commands/finance";
export { type SchedulingCommands } from "./commands/scheduling";
export { type ExamCommands } from "./commands/exam";
export { type IAMCommands } from "./commands/iam";
export { type OrganizationCommands } from "./commands/organization";
export { type DocforgeCommands } from "./commands/docforge";

// ── Errors ──────────────────────────────────────────────────────
export { OrbisError, extractErrorMessage } from "./errors";

// ── Permissions ─────────────────────────────────────────────────
export { Permissions } from "./permissions";

// ── Events ──────────────────────────────────────────────────────
export { EventTypes } from "./events";

// ── Shared aliases ──────────────────────────────────────────────
export type GuardianLink = StudentGuardian;
export type JsonRecord = Record<string, any>;
