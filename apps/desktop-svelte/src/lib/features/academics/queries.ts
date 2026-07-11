/**
 * Academic domain queries — periods, classes, subjects, schedules.
 * Wraps IPC commands with TanStack Query for automatic caching.
 *
 * Migrated: cycles → periods, cycleId → periodId/yearId.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All academic periods for current year. */
export function createPeriodsQuery(orgId: string) {
  return createQuery({
    queryKey: ["periods", orgId],
    queryFn: () => commands.academic.listCycles(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000,
  });
}

/** Periods for a specific year (cross-year queries). */
export function createPeriodsByYearQuery(orgId: string, yearId: string) {
  return createQuery({
    queryKey: ["periods", orgId, yearId],
    queryFn: () => commands.academic.listPeriodsByYear(orgId, yearId),
    enabled: !!orgId && !!yearId,
    staleTime: 10 * 60 * 1000,
  });
}

// Backward compat alias
export const createCyclesQuery = createPeriodsQuery;

/** Classes for an organization. */
export function createClassesQuery(orgId: string) {
  return createQuery({
    queryKey: ["classes", orgId],
    queryFn: () => commands.academic.listClasses(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Subjects for an organization. */
export function createSubjectsQuery(orgId: string) {
  return createQuery({
    queryKey: ["subjects", orgId],
    queryFn: () => commands.academic.listSubjects(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000,
  });
}

/** Class subjects / assignments. */
export function createClassSubjectsQuery(classId?: string, periodId?: string) {
  return createQuery({
    queryKey: ["class-subjects", classId, periodId],
    queryFn: () => commands.academic.listClassSubjects(classId!, periodId!),
    enabled: !!classId && !!periodId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Assessments for a class-subject. */
export function createAssessmentsQuery(params: {
  periodId?: string;
  classSubjectId?: string;
}) {
  return createQuery({
    queryKey: ["assessments", params.periodId, params.classSubjectId],
    queryFn: () => commands.assessment.listAssessments(params.periodId!, params.classSubjectId),
    enabled: !!params.periodId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Time slots for an academic year. */
export function createTimeSlotsQuery(yearId?: string) {
  return createQuery({
    queryKey: ["time-slots", yearId],
    queryFn: () => commands.scheduling.listTimeSlots(yearId!),
    enabled: !!yearId,
    staleTime: 10 * 60 * 1000,
  });
}

/** Class schedule lessons. */
export function createClassScheduleQuery(yearId?: string, classId?: string) {
  return createQuery({
    queryKey: ["class-schedule", yearId, classId],
    queryFn: () => commands.scheduling.listLessonsByClass(yearId!, classId!),
    enabled: !!yearId && !!classId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Teacher schedule lessons. */
export function createTeacherScheduleQuery(yearId?: string, teacherId?: string) {
  return createQuery({
    queryKey: ["teacher-schedule", yearId, teacherId],
    queryFn: () => commands.scheduling.listLessonsByTeacher(yearId!, teacherId!),
    enabled: !!yearId && !!teacherId,
    staleTime: 2 * 60 * 1000,
  });
}
