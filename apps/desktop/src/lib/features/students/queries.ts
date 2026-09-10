/**
 * Students domain queries — enrollments and guardians.
 * Wraps OrbisApiClient with TanStack Query and queryKeys factory.
 */
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

/** All enrollments for an organization. */
export function createEnrollmentsQuery(
  orgId: () => string,
  filter?: () => { classId?: string; studentId?: string } | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollments(filter?.()),
    queryFn: () => api.listEnrollments(filter?.()),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Single enrollment for detail pages. */
export function createEnrollmentQuery(enrollmentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollment(enrollmentId()),
    queryFn: () => api.getEnrollment(enrollmentId()),
    enabled: !!enrollmentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Guardians list for a student. */
export function createGuardiansQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.guardians(studentId()),
    queryFn: () => api.listGuardians(studentId()),
    enabled: !!studentId(),
    staleTime: 5 * 60 * 1000,
  }));
}
