import { useQuery, useMutation, useQueryClient, useSuspenseQuery } from "@tanstack/react-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { getOrgId, getUserId } from "@/lib/session";
import type { JsonRecord } from "@orbis/desktop-client/types";

export function useEnrollments(params?: JsonRecord) {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.students.all(orgId),
    queryFn: () => commands.enrollments.getAll(orgId, params),
    enabled: !!orgId,
  });
}

export function useEnrollment(enrollmentId: string) {
  return useQuery({
    queryKey: queryKeys.students.detail(enrollmentId),
    queryFn: () => commands.enrollments.getById(enrollmentId),
    enabled: !!enrollmentId,
  });
}

export function useCreateEnrollment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (request: JsonRecord) =>
      commands.enrollments.create(getOrgId(), request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["students"] }),
  });
}

export function useGrades(params?: JsonRecord) {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.grades.all(orgId),
    queryFn: () => commands.grades.getAll(orgId, params),
    enabled: !!orgId,
  });
}

export function useGradeAnalytics(params?: JsonRecord) {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.grades.analytics(orgId, params),
    queryFn: () => commands.grades.getAnalytics(orgId, params),
    enabled: !!orgId,
  });
}

export function useClassStatistics(classId: string, params?: JsonRecord) {
  return useQuery({
    queryKey: queryKeys.grades.classStatistics(classId, params),
    queryFn: () => commands.grades.getAnalytics(getOrgId(), {
      class_id: classId,
      ...params,
    }),
    enabled: !!classId,
  });
}

export function useSubmitGrade() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (request: JsonRecord) =>
      commands.grades.submit(getOrgId(), getUserId(), request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["grades"] }),
  });
}

export function useSubmitGradesBulk() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (request: JsonRecord) =>
      commands.grades.submitBulk(getOrgId(), getUserId(), request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["grades"] }),
  });
}

export function useReportCard(enrollmentId: string) {
  return useSuspenseQuery({
    queryKey: queryKeys.grades.reportCard(enrollmentId),
    queryFn: () => commands.grades.getReportCard(enrollmentId),
  });
}

export function useAttendance(params?: JsonRecord) {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.attendance.all(orgId),
    queryFn: () => commands.attendance.getAll(orgId, params),
    enabled: !!orgId,
  });
}

export function useDocuments(params?: JsonRecord) {
  return useQuery({
    queryKey: queryKeys.documents.all(params),
    queryFn: () => commands.documents.getAll(params),
  });
}

export function useCertificates() {
  return useQuery({
    queryKey: queryKeys.certificates.all(),
    queryFn: () => commands.certificates.getAll(),
  });
}

export function useCertificate(id: string) {
  return useQuery({
    queryKey: queryKeys.certificates.detail(id),
    queryFn: () => commands.certificates.getById(id),
    enabled: !!id,
  });
}

export function useNotifications(userId?: string, params?: JsonRecord) {
  const id = userId ?? getUserId();
  return useQuery({
    queryKey: queryKeys.notifications.all(id, params),
    queryFn: () => commands.notifications.getAll(id, params),
    enabled: !!id,
  });
}

export function useAssessments(params: {
  classId?: string;
  subjectId?: string;
  termId?: string;
  month?: number;
}) {
  return useQuery({
    queryKey: queryKeys.assessments.byClass(params.classId),
    queryFn: () => commands.assessments.getByClass(params),
    enabled: !!params.classId,
  });
}
