import { createQuery, createMutation } from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type CalculateRankingsRequest,
  type RankingItemDto,
  type ClassDto,
  type SubjectDto,
  type AcademicPeriodDto,
  type ClassSubjectDto,
  type AssessmentSchemeDto,
  type TimeSlotDto,
  type ScheduledLessonDto,
  type PrerequisiteCheckResultDto,
} from "$lib/api";

/** All academic periods for current year. */
export function createPeriodsQuery(orgId: () => string) {
  return createQuery<AcademicPeriodDto[]>(() => ({
    queryKey: queryKeys.calendar.periods(orgId()),
    queryFn: () => api.listAcademicPeriods(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Periods for a specific year. */
export function createPeriodsByYearQuery(
  orgId: () => string,
  periodId: () => string,
) {
  return createQuery<AcademicPeriodDto[]>(() => ({
    queryKey: queryKeys.calendar.periods(orgId()),
    queryFn: () => api.listAcademicPeriods(),
    enabled: !!orgId() && !!periodId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Classes for an organization. */
export function createClassesQuery(orgId: () => string) {
  return createQuery<ClassDto[]>(() => ({
    queryKey: queryKeys.academic.classes(orgId()),
    queryFn: () => api.listClasses(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Subjects for an organization. */
export function createSubjectsQuery(orgId: () => string) {
  return createQuery<SubjectDto[]>(() => ({
    queryKey: queryKeys.academic.subjects(orgId()),
    queryFn: () => api.listSubjects(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Class subjects / assignments. */
export function createClassSubjectsQuery(
  classId: () => string | undefined,
  periodId: () => string | undefined,
) {
  return createQuery<ClassSubjectDto[]>(() => ({
    queryKey: queryKeys.academic.classSubjects(classId()!, periodId()!),
    queryFn: () => api.listClassSubjects(classId()!, periodId()!),
    enabled: !!classId() && !!periodId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Assessments for a class-subject. */
export function createAssessmentsQuery(
  params: () => {
    periodId?: string;
    classSubjectId?: string;
  },
) {
  return createQuery<AssessmentSchemeDto[]>(() => ({
    queryKey: queryKeys.assessment.schemes(),
    queryFn: () => api.listAssessmentSchemes(),
    enabled: !!params().periodId,
    staleTime: 2 * 60 * 1000,
  }));
}

/** Time slots for an academic period. */
export function createTimeSlotsQuery(
  orgId: () => string | undefined,
  periodId: () => string | undefined,
) {
  return createQuery<TimeSlotDto[]>(() => ({
    queryKey: queryKeys.scheduling.timeSlots(periodId()!),
    queryFn: () => api.listTimeSlots(periodId()!),
    enabled: !!orgId() && !!periodId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Class schedule lessons. */
export function createClassScheduleQuery(
  orgId: () => string | undefined,
  periodId: () => string | undefined,
  classId: () => string | undefined,
) {
  return createQuery<ScheduledLessonDto[]>(() => ({
    queryKey: queryKeys.scheduling.lessons({ periodId: periodId() }),
    queryFn: () => api.listScheduledLessons({ periodId: periodId() }),
    enabled: !!orgId() && !!periodId() && !!classId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Teacher schedule lessons. */
export function createTeacherScheduleQuery(
  orgId: () => string | undefined,
  periodId: () => string | undefined,
  teacherId: () => string | undefined,
) {
  return createQuery<ScheduledLessonDto[]>(() => ({
    queryKey: queryKeys.scheduling.lessons({ periodId: periodId() }),
    queryFn: () => api.listScheduledLessons({ periodId: periodId() }),
    enabled: !!orgId() && !!periodId() && !!teacherId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Check student prerequisites for a subject */
export function createPrerequisitesQuery(
  studentId: () => string,
  subjectId: () => string,
) {
  return createQuery<PrerequisiteCheckResultDto[]>(() => ({
    queryKey: queryKeys.academic.prerequisites(studentId(), subjectId()),
    queryFn: () => api.checkPrerequisites(studentId(), subjectId()),
    enabled: !!studentId() && !!subjectId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Calculate student rankings using authoritative backend engine */
export function createCalculateRankingsMutation() {
  return createMutation(() => ({
    mutationFn: (params: CalculateRankingsRequest): Promise<RankingItemDto[]> => {
      return api.calculateRankings(params);
    },
  }));
}

/** Generate student certificates & grade sheets */
export function createGenerateReportMutation() {
  return createMutation(() => ({
    mutationFn: (params: {
      reportCode: string;
      format?: string;
      parameters?: Record<string, unknown>;
    }) => {
      return api.generateReport(
        params.reportCode,
        params.format || "PDF",
        params.parameters || {},
      );
    },
  }));
}



