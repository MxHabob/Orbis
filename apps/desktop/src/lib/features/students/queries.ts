/**
 * Students domain queries & mutations — students, enrollments, and guardians.
 * Wraps OrbisApiClient with TanStack Query and queryKeys factory.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type StudentFilterParams,
  type EnrollmentFilterParams,
  type RegisterStudentInput,
  type UpdateStudentStatusInput,
  type ChangeStudyModeInput,
  type EnrollStudentRequest,
  type AssignGuardianRequest,
} from "$lib/api";

/** Paginated and filtered students query. */
export function createStudentsQuery(
  orgId: () => string,
  filter?: () => StudentFilterParams | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.list(filter?.() as Record<string, unknown>),
    queryFn: () => api.listStudents(filter?.()),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Single student profile query (StudentDetail). */
export function createStudentQuery(
  idOrOrg: () => string,
  maybeId?: () => string,
) {
  const studentId = () => (maybeId ? maybeId() : idOrOrg());
  return createQuery(() => ({
    queryKey: queryKeys.student.detail(studentId()),
    queryFn: () => api.getStudent(studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** All enrollments for an organization. */
export function createEnrollmentsQuery(
  orgId: () => string,
  filter?: () => EnrollmentFilterParams | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollments(filter?.() as Record<string, unknown>),
    queryFn: () => api.listEnrollments(filter?.()),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Single enrollment for detail pages. */
export function createEnrollmentQuery(
  idOrOrg: () => string,
  maybeId?: () => string,
) {
  const enrollmentId = () => (maybeId ? maybeId() : idOrOrg());
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

/** Register student mutation */
export function createRegisterStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: RegisterStudentInput) => api.registerStudent(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Update student status mutation */
export function createUpdateStudentStatusMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: UpdateStudentStatusInput) => api.updateStudentStatus(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(vars.studentId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Change study mode mutation */
export function createChangeStudyModeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: ChangeStudyModeInput) => api.changeStudyMode(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Enroll student mutation */
export function createEnrollStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: EnrollStudentRequest) => api.enrollStudent(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Assign guardian mutation */
export function createAssignGuardianMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: AssignGuardianRequest) => api.assignGuardian(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.guardians(vars.student_id) });
    },
  }));
}
