/**
 * Students domain queries & mutations — students, enrollments, guardians, attendance, behavior, documents, and finance.
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
  type UpdateStudentProfileInput,
  type ChangeStudyModeInput,
  type TransferClassInput,
  type EnrollStudentRequest,
  type AssignGuardianRequest,
  type RecordBehaviorInput,
  type UploadDocumentRequest,
} from "$lib/api";

/** Paginated and filtered students query (StudentDetail items). */
export function createStudentsQuery(
  orgId: () => string,
  filter?: () => StudentFilterParams | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.list(orgId(), filter?.() as Record<string, unknown>),
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
    queryKey: queryKeys.student.enrollments(orgId(), filter?.() as Record<string, unknown>),
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

/** Attendance records list for a student. */
export function createStudentAttendanceQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.attendance(studentId()),
    queryFn: () => api.listStudentAttendance(studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Behavior records list for a student. */
export function createStudentBehaviorQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.behavior(studentId()),
    queryFn: () => api.listStudentBehaviorRecords(studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Documents attached to a student entity. */
export function createStudentDocumentsQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.documents(studentId()),
    queryFn: () => api.listDocumentsByEntity("student", studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Evaluated document requirements for a student. */
export function createStudentDocumentRequirementsQuery(
  studentId: () => string,
  gradeLevelId?: () => string | undefined,
  isNational?: () => boolean | undefined,
  isTransfer?: () => boolean | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.documentRequirements(studentId()),
    queryFn: () => api.evaluateStudentDocumentRequirements(studentId(), {
      grade_level_id: gradeLevelId?.(),
      is_national: isNational?.(),
      is_transfer: isTransfer?.(),
    }),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Financial fees assigned to a student. */
export function createStudentFeesQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.fees(studentId()),
    queryFn: () => api.listStudentFees({ student_id: studentId() }),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Financial payments made by or on behalf of a student. */
export function createStudentPaymentsQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.payments(studentId()),
    queryFn: () => api.listPayments({ student_id: studentId() }),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
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
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(vars.student_id || vars.studentId || "") });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Update student profile mutation */
export function createUpdateStudentProfileMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: UpdateStudentProfileInput) => api.updateStudentProfile(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(vars.student_id || vars.studentId || "") });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Soft delete student mutation */
export function createDeleteStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (studentId: string) => api.deleteStudent(studentId),
    onSuccess: (_, studentId) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(studentId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Restore soft deleted student mutation */
export function createRestoreStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (studentId: string) => api.restoreStudent(studentId),
    onSuccess: (_, studentId) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(studentId) });
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

/** Transfer class mutation */
export function createTransferClassMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: TransferClassInput) => api.transferClass(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.academic.all });
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

/** Remove guardian mutation */
export function createRemoveGuardianMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ studentId, guardianId }: { studentId: string; guardianId: string }) =>
      api.removeGuardian(studentId, guardianId),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.guardians(vars.studentId) });
    },
  }));
}

/** Record behavior mutation */
export function createRecordBehaviorMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: RecordBehaviorInput) => api.recordBehavior(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.behavior(vars.student_id || "") });
    },
  }));
}

/** Upload document mutation */
export function createUploadStudentDocumentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: UploadDocumentRequest) => api.uploadDocument(req),
    onSuccess: (_, vars) => {
      if (vars.entity_id) {
        queryClient.invalidateQueries({ queryKey: queryKeys.student.documents(vars.entity_id) });
        queryClient.invalidateQueries({ queryKey: queryKeys.student.documentRequirements(vars.entity_id) });
      }
    },
  }));
}

/** Delete document mutation */
export function createDeleteDocumentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (docId: string) => api.deleteDocument(docId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

