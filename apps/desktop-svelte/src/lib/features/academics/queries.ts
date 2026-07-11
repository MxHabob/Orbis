/**
 * Academic domain queries — cycles, classes, subjects, schedules.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All academic cycles for an organization. */
export function createCyclesQuery(orgId: string) {
  return createQuery({
    queryKey: ["cycles", orgId],
    queryFn: () => commands.academic.listCycles(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000, // Cycles rarely change — 10 min
  });
}

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
    staleTime: 10 * 60 * 1000, // Subjects rarely change
  });
}

/** Class subjects / assignments. */
export function createClassSubjectsQuery(classId?: string, cycleId?: string) {
  return createQuery({
    queryKey: ["class-subjects", classId, cycleId],
    queryFn: () => commands.academic.listClassSubjects(classId!, cycleId!),
    enabled: !!classId && !!cycleId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Assessments for a class-subject. */
export function createAssessmentsQuery(params: {
  cycleId?: string;
  classSubjectId?: string;
}) {
  return createQuery({
    queryKey: ["assessments", params.cycleId, params.classSubjectId],
    queryFn: () => commands.assessment.listAssessments(params.cycleId!, params.classSubjectId),
    enabled: !!params.cycleId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Time slots for an academic cycle. */
export function createTimeSlotsQuery(cycleId?: string) {
  return createQuery({
    queryKey: ["time-slots", cycleId],
    queryFn: () => commands.scheduling.listTimeSlots(cycleId!),
    enabled: !!cycleId,
    staleTime: 10 * 60 * 1000,
  });
}

/** Class schedule lessons. */
export function createClassScheduleQuery(cycleId?: string, classId?: string) {
  return createQuery({
    queryKey: ["class-schedule", cycleId, classId],
    queryFn: () => commands.scheduling.listLessonsByClass(cycleId!, classId!),
    enabled: !!cycleId && !!classId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Teacher schedule lessons. */
export function createTeacherScheduleQuery(cycleId?: string, teacherId?: string) {
  return createQuery({
    queryKey: ["teacher-schedule", cycleId, teacherId],
    queryFn: () => commands.scheduling.listLessonsByTeacher(cycleId!, teacherId!),
    enabled: !!cycleId && !!teacherId,
    staleTime: 2 * 60 * 1000,
  });
}
