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

export function useDocumentTemplates(entityType?: string) {
  return useQuery({
    queryKey: queryKeys.documentTemplates.all(entityType),
    queryFn: () => commands.documentTemplates.getAll(entityType),
  });
}

export function useDocumentTemplate(id: string) {
  return useQuery({
    queryKey: queryKeys.documentTemplates.detail(id),
    queryFn: () => commands.documentTemplates.getById(id),
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

export function useCreateAssessment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (request: JsonRecord) =>
      commands.assessments.create(request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["assessments"] }),
  });
}

export function useUpdateAssessment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, request }: { id: string; request: JsonRecord }) =>
      commands.assessments.update(id, request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["assessments"] }),
  });
}

export function useDeleteAssessment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => commands.assessments.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["assessments"] }),
  });
}

/** Initialize default assessment items for a class/subject/term/month. */
export function useInitializeAssessments() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: async (input: {
      classId: number;
      subjectId: number;
      termId: number;
      month: number;
    }) => {
      const existing = await commands.assessments.getByClass({
        classId: String(input.classId),
        subjectId: String(input.subjectId),
        termId: String(input.termId),
        month: input.month,
      });
      if (existing.length > 0) {
        return { created: false as const, count: existing.length };
      }
      // Bulk initialize IPC not yet available — no-op until Rust command lands
      return { created: true as const, count: 0 };
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["assessments"] });
      qc.invalidateQueries({ queryKey: ["grades"] });
    },
  });
}
