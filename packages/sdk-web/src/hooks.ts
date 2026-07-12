/**
 * React Query hooks for @orbis/sdk-web.
 *
 * Pre-built query/mutation hooks that wrap the commands.
 * Usage:
 * ```tsx
 * import { useClasses, useCreateClass } from "@orbis/sdk-web/hooks";
 *
 * function ClassList({ orgId }) {
 *   const { data: classes, isLoading } = useClasses(orgId);
 *   const createClass = useCreateClass();
 *   // ...
 * }
 * ```
 */

import {
  useQuery,
  useMutation,
  useQueryClient,
  type UseQueryOptions,
  type UseMutationOptions,
} from "@tanstack/react-query";
import { commands } from "./commands";

// ── Query Key Factory ───────────────────────────────────────────
export const queryKeys = {
  // Academic
  classes: (orgId: string) => ["classes", orgId] as const,
  subjects: (orgId: string) => ["subjects", orgId] as const,
  orgUnits: (orgId: string) => ["orgUnits", orgId] as const,
  enrollments: (orgId: string) => ["enrollments", orgId] as const,
  cycles: (orgId: string) => ["cycles", orgId] as const,
  guardians: (studentId: string) => ["guardians", studentId] as const,
  documents: (ownerId: string) => ["documents", ownerId] as const,
  notifications: () => ["notifications"] as const,

  // Assessment
  grades: (periodId: string) => ["grades", periodId] as const,
  attendance: (classId: string, date: string) => ["attendance", classId, date] as const,
  assessments: (periodId: string) => ["assessments", periodId] as const,
  behavior: (studentId: string) => ["behavior", studentId] as const,
  reportCard: (enrollmentId: string, periodId: string) => ["reportCard", enrollmentId, periodId] as const,

  // Finance
  feeTemplates: () => ["feeTemplates"] as const,
  studentFees: (enrollmentId: string) => ["studentFees", enrollmentId] as const,
  payments: () => ["payments"] as const,
  treasuries: () => ["treasuries"] as const,
  ledgerAccounts: () => ["ledgerAccounts"] as const,

  // Scheduling
  timeSlots: () => ["timeSlots"] as const,
  rooms: () => ["rooms"] as const,
  schedule: (classId: string, yearId: string) => ["schedule", classId, yearId] as const,
  solverJobs: (yearId: string) => ["solverJobs", yearId] as const,

  // Exam
  examSessions: (periodId: string) => ["examSessions", periodId] as const,
  examHalls: () => ["examHalls"] as const,

  // IAM
  users: () => ["users"] as const,
  user: (userId: string) => ["user", userId] as const,

  // Organization
  organization: () => ["organization"] as const,
  dashboard: () => ["dashboard"] as const,
  settings: () => ["settings"] as const,
} as const;

// ── Academic Hooks ──────────────────────────────────────────────

export function useClasses(orgId: string) {
  return useQuery({
    queryKey: queryKeys.classes(orgId),
    queryFn: () => commands.academic.listClasses(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

export function useCreateClass() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: any) => commands.academic.createClass(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["classes"] }),
  });
}

export function useSubjects(orgId: string) {
  return useQuery({
    queryKey: queryKeys.subjects(orgId),
    queryFn: () => commands.academic.listSubjects(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

export function useCycles(orgId: string) {
  return useQuery({
    queryKey: queryKeys.cycles(orgId),
    queryFn: () => commands.academic.listCycles(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

export function useEnrollments(orgId: string) {
  return useQuery({
    queryKey: queryKeys.enrollments(orgId),
    queryFn: () => commands.academic.listEnrollments(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

export function useCreateEnrollment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: any) => commands.academic.createEnrollment(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["enrollments"] }),
  });
}

// ── Assessment Hooks ────────────────────────────────────────────

export function useGrades(periodId: string) {
  return useQuery({
    queryKey: queryKeys.grades(periodId),
    queryFn: () => commands.assessment.listGrades(periodId),
    enabled: !!periodId,
    staleTime: 5 * 60 * 1000,
  });
}

export function useSubmitGrade() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ periodId, data }: { periodId: string; data: any }) =>
      commands.assessment.submitGrade(periodId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["grades"] }),
  });
}

export function useReportCard(enrollmentId: string, periodId: string) {
  return useQuery({
    queryKey: queryKeys.reportCard(enrollmentId, periodId),
    queryFn: () => commands.assessment.getReportCard(enrollmentId, periodId),
    enabled: !!enrollmentId && !!periodId,
    staleTime: 10 * 60 * 1000,
  });
}

export function useAttendance(classId: string, date: string) {
  return useQuery({
    queryKey: queryKeys.attendance(classId, date),
    queryFn: () => commands.assessment.listAttendance("", classId, date),
    enabled: !!classId && !!date,
    staleTime: 2 * 60 * 1000,
  });
}

export function useAssessments(periodId: string) {
  return useQuery({
    queryKey: queryKeys.assessments(periodId),
    queryFn: () => commands.assessment.listAssessments(periodId),
    enabled: !!periodId,
    staleTime: 5 * 60 * 1000,
  });
}

// ── Finance Hooks ───────────────────────────────────────────────

export function useFeeTemplates() {
  return useQuery({
    queryKey: queryKeys.feeTemplates(),
    queryFn: () => commands.finance.listFeeTemplates(),
    staleTime: 5 * 60 * 1000,
  });
}

export function useStudentFees(enrollmentId: string) {
  return useQuery({
    queryKey: queryKeys.studentFees(enrollmentId),
    queryFn: () => commands.finance.listStudentFees(enrollmentId),
    enabled: !!enrollmentId,
    staleTime: 5 * 60 * 1000,
  });
}

export function usePayments() {
  return useQuery({
    queryKey: queryKeys.payments(),
    queryFn: () => commands.finance.listPayments(),
    staleTime: 5 * 60 * 1000,
  });
}

export function useRecordPayment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: any) => commands.finance.recordPayment(data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["payments"] });
      qc.invalidateQueries({ queryKey: ["studentFees"] });
    },
  });
}

// ── Scheduling Hooks ────────────────────────────────────────────

export function useTimeSlots() {
  return useQuery({
    queryKey: queryKeys.timeSlots(),
    queryFn: () => commands.scheduling.listTimeSlots(""),
    staleTime: 10 * 60 * 1000,
  });
}

export function useRooms() {
  return useQuery({
    queryKey: queryKeys.rooms(),
    queryFn: () => commands.scheduling.listRooms(),
    staleTime: 10 * 60 * 1000,
  });
}

export function useClassSchedule(classId: string, yearId: string) {
  return useQuery({
    queryKey: queryKeys.schedule(classId, yearId),
    queryFn: () => commands.scheduling.listLessonsByClass(yearId, classId),
    enabled: !!classId && !!yearId,
    staleTime: 5 * 60 * 1000,
  });
}

// ── Exam Hooks ──────────────────────────────────────────────────

export function useExamSessions(periodId: string) {
  return useQuery({
    queryKey: queryKeys.examSessions(periodId),
    queryFn: () => commands.exam.listSessions(periodId),
    enabled: !!periodId,
    staleTime: 5 * 60 * 1000,
  });
}

// ── IAM Hooks ───────────────────────────────────────────────────

export function useUsers() {
  return useQuery({
    queryKey: queryKeys.users(),
    queryFn: () => commands.iam.listUsers(),
    staleTime: 5 * 60 * 1000,
  });
}

export function useUser(userId: string) {
  return useQuery({
    queryKey: queryKeys.user(userId),
    queryFn: () => commands.iam.getUser(userId),
    enabled: !!userId,
    staleTime: 5 * 60 * 1000,
  });
}

export function useCreateUser() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: any) => commands.iam.createUser(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["users"] }),
  });
}

// ── Organization Hooks ──────────────────────────────────────────

export function useOrganization() {
  return useQuery({
    queryKey: queryKeys.organization(),
    queryFn: () => commands.organization.getOrganization(""),
    staleTime: 10 * 60 * 1000,
  });
}

export function useDashboardStats() {
  return useQuery({
    queryKey: queryKeys.dashboard(),
    queryFn: () => commands.organization.getDashboardStats(""),
    staleTime: 5 * 60 * 1000,
  });
}
