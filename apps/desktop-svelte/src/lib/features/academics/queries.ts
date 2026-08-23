/**
 * Academic domain queries — periods, classes, subjects, schedules.
 * Wraps IPC commands with TanStack Query for automatic caching.
 *
 * Migrated: periods → periods, periodId → periodId/periodId.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All academic periods for current year. */
export function createPeriodsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["periods", orgId()],
    queryFn: () => commands.calendar.listAcademicPeriods(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Periods for a specific year (cross-year queries). */
export function createPeriodsByYearQuery(
  orgId: () => string,
  periodId: () => string,
) {
  return createQuery(() => ({
    queryKey: ["periods", orgId(), periodId()],
    queryFn: () => commands.calendar.listAcademicPeriodsByYear(periodId()),
    enabled: !!orgId() && !!periodId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Classes for an organization. */
export function createClassesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["classes", orgId()],
    queryFn: () => commands.academic.listClasses(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Subjects for an organization. */
export function createSubjectsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["subjects", orgId()],
    queryFn: () => commands.academic.listSubjects(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Class subjects / assignments. */
export function createClassSubjectsQuery(
  classId: () => string | undefined,
  periodId: () => string | undefined,
) {
  return createQuery(() => ({
    queryKey: ["class-subjects", classId(), periodId()],
    queryFn: () => commands.academic.listClassSubjects(classId()!, periodId()!),
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
  return createQuery(() => ({
    queryKey: ["assessments", params().periodId, params().classSubjectId],
    queryFn: () =>
      commands.assessment.listAssessments(
        params().periodId!,
        params().classSubjectId,
      ),
    enabled: !!params().periodId,
    staleTime: 2 * 60 * 1000,
  }));
}

/** Time slots for an academic year. */
export function createTimeSlotsQuery(
  orgId: () => string | undefined,
  periodId: () => string | undefined,
) {
  return createQuery(() => ({
    queryKey: ["time-slots", orgId(), periodId()],
    queryFn: () => commands.scheduling.listTimeSlots(periodId()!),
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
  return createQuery(() => ({
    queryKey: ["class-schedule", orgId(), periodId(), classId()],
    queryFn: () =>
      commands.scheduling.listLessonsByClass(periodId()!, classId()!),
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
  return createQuery(() => ({
    queryKey: ["teacher-schedule", orgId(), periodId(), teacherId()],
    queryFn: () =>
      commands.scheduling.listLessonsByTeacher(periodId()!, teacherId()!),
    enabled: !!orgId() && !!periodId() && !!teacherId(),
    staleTime: 2 * 60 * 1000,
  }));
}
