/** Shared entity types — mirror core-domain SeaORM models (snake_case from Rust IPC). */

export interface CommandError {
  message: string;
  code?: string | null;
}

export interface User {
  id: string;
  org_id: string;
  email?: string | null;
  full_name: string;
  name_ar?: string | null;
  name_en?: string | null;
  avatar?: string | null;
  role?: string | null;
  [key: string]: unknown;
}

export interface LoginResponse {
  user: User;
  access_token: string;
  refresh_token: string;
}

export type JsonRecord = Record<string, unknown>;

export type AcademicCycle = JsonRecord;
export type Class = JsonRecord;
export type Enrollment = JsonRecord;
export type Grade = JsonRecord;
export type Attendance = JsonRecord;
export type Document = JsonRecord;
export type DocumentTemplate = JsonRecord;
export type Subject = JsonRecord;
export type Assessment = JsonRecord;
export type Notification = JsonRecord;
export type Organization = JsonRecord;
export type DashboardStats = JsonRecord;
export type GradeAnalytics = JsonRecord;

// ── Schedule ─────────────────────────────────────────────────────────────────
export type TimeSlot = JsonRecord;
export type ClassSubject = JsonRecord;
export type ScheduledLesson = JsonRecord;
export type ScheduleConflict = JsonRecord;
export type SolveJob = JsonRecord;

// ── Guardians ────────────────────────────────────────────────────────────────
export type GuardianLink = JsonRecord;
