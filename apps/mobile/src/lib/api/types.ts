/**
 * @module @orbis/mobile-api/types
 *
 * Strongly-typed API client interfaces and domain DTOs for the Orbis Mobile Application.
 * Directly re-exports from @orbis/sdk with mobile error envelopes.
 */

import type {
  IntegrationDto,
  ValidationIssueDto,
  ValidationResultDto,
} from "@orbis/sdk";

export type {
  AcademicContextResponse,
  AttendanceDto,
  BatchMarkAttendanceRequest,
  BatchRecordGradesRequest,
  BehaviorRecordDto,
  DashboardCountsDto,
  SectionDto,
  EnrollmentDto,
  ExamScheduleDto,
  ExamSessionDto,
  GradeDto,
  GuardianDto,
  MarkAttendanceRequest,
  OrganizationSettingsDto,
  OrgDto,
  RecordGradeItemRequest,
  RecordGradeRequest,
  ScheduledLessonDto,
  SectionSubjectDto,
  StudentDto,
  StudentFeeDto,
  SubjectDto,
  TemplateDto,
  TimeSlotDto,
  UserDeviceDto,
  UserDto,
} from "@orbis/sdk";

export type ValidationResult = ValidationResultDto;
export type { ValidationIssueDto };

// ═══════════════════════════════════════════════════════════════
// SHARED ENVELOPES
// ═══════════════════════════════════════════════════════════════

export interface ApiResponse<T> {
  data: T;
  request_id?: string;
  requestId?: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  count: number;
  nextCursor?: string | null;
  next_cursor?: string | null;
  hasMore: boolean;
  has_more: boolean;
}

export interface IntegrationStatusResponse {
  connected: boolean;
  is_enabled: boolean;
  config?: IntegrationDto | null;
}

// ═══════════════════════════════════════════════════════════════
// ERROR MODELS
// ═══════════════════════════════════════════════════════════════

export class OrbisError extends Error {
  code: string;
  details?: unknown;
  requestId?: string;

  constructor(
    message: string,
    code = "UNKNOWN_ERROR",
    details?: unknown,
    requestId?: string,
  ) {
    super(message);
    this.name = "OrbisError";
    this.code = code;
    this.details = details;
    this.requestId = requestId;
  }

  static fromUnknown(err: unknown): OrbisError {
    if (err instanceof OrbisError) return err;
    if (err instanceof Error) {
      return new OrbisError(err.message, "INTERNAL_ERROR", err);
    }
    if (typeof err === "object" && err !== null) {
      const record = err as Record<string, unknown>;
      return new OrbisError(
        typeof record.message === "string" ? record.message : "Unknown error",
        typeof record.code === "string" ? record.code : "UNKNOWN_ERROR",
        record.details ?? record,
        typeof record.request_id === "string"
          ? (record.request_id as string)
          : typeof record.requestId === "string"
            ? (record.requestId as string)
            : undefined,
      );
    }
    return new OrbisError(String(err));
  }
}

export class ApiRequestError extends Error {
  constructor(
    public readonly status: number,
    public readonly error: OrbisError,
  ) {
    super(error.message);
    this.name = "ApiRequestError";
  }
}
