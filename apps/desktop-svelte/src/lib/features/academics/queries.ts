/**
 * Academic domain queries — cycles, setup status.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All academic cycles for an organization. */
export function createCyclesQuery(orgId: string) {
  return createQuery({
    queryKey: ["cycles", orgId],
    queryFn: () => commands.academics.getCycles(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000, // Cycles rarely change — 10 min
  });
}

/** Classes for an organization. */
export function createClassesQuery(orgId: string) {
  return createQuery({
    queryKey: ["classes", orgId],
    queryFn: () => commands.classes.getAll(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Subjects for an organization. */
export function createSubjectsQuery(orgId: string) {
  return createQuery({
    queryKey: ["subjects", orgId],
    queryFn: () => commands.subjects.getAll(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000, // Subjects rarely change
  });
}

/** Class subjects / assignments. */
export function createClassSubjectsQuery(classId?: string, cycleId?: string) {
  return createQuery({
    queryKey: ["class-subjects", classId, cycleId],
    queryFn: () => commands.schedule.getClassSubjects(classId, cycleId),
    enabled: !!classId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Assessments for a class. */
export function createAssessmentsQuery(params: {
  classId?: string;
  subjectId?: string;
  termId?: string;
}) {
  return createQuery({
    queryKey: ["assessments", params.classId, params.subjectId, params.termId],
    queryFn: () => commands.assessments.getByClass(params),
    enabled: !!params.classId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Time slots for an organization's academic cycle. */
export function createTimeSlotsQuery(orgId: string, cycleId?: string) {
  return createQuery({
    queryKey: ["time-slots", orgId, cycleId],
    queryFn: () => commands.schedule.getTimeSlots(orgId, cycleId!),
    enabled: !!orgId && !!cycleId,
    staleTime: 10 * 60 * 1000,
  });
}

/** Class schedule lessons. */
export function createClassScheduleQuery(orgId: string, cycleId?: string, classId?: string) {
  return createQuery({
    queryKey: ["class-schedule", orgId, cycleId, classId],
    queryFn: () => commands.schedule.getClassSchedule(orgId, cycleId!, classId!),
    enabled: !!orgId && !!cycleId && !!classId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Teacher schedule lessons. */
export function createTeacherScheduleQuery(orgId: string, cycleId?: string, teacherId?: string) {
  return createQuery({
    queryKey: ["teacher-schedule", orgId, cycleId, teacherId],
    queryFn: () => commands.schedule.getTeacherSchedule(orgId, cycleId!, teacherId!),
    enabled: !!orgId && !!cycleId && !!teacherId,
    staleTime: 2 * 60 * 1000,
  });
}
