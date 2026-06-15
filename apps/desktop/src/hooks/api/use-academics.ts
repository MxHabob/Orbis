import { useQuery, useMutation, useQueryClient, useSuspenseQuery } from "@tanstack/react-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { getOrgId, getUserId } from "@/lib/session";
import type { JsonRecord } from "@orbis/desktop-client/types";

export function useCycles(params?: JsonRecord) {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.cycles.all(orgId),
    queryFn: () => commands.academics.getCycles(orgId, params),
    enabled: !!orgId,
  });
}

export function useCurrentYear() {
  const orgId = getOrgId();
  return useSuspenseQuery({
    queryKey: queryKeys.cycles.current(orgId),
    queryFn: () => commands.academics.getCurrentYear(orgId),
  });
}

export function useAcademicYears(params?: JsonRecord) {
  const orgId = getOrgId();
  return useSuspenseQuery({
    queryKey: queryKeys.cycles.all(orgId),
    queryFn: () => commands.academics.getYears(orgId),
  });
}

export function useAcademicTerms(yearId: string) {
  const orgId = getOrgId();
  return useSuspenseQuery({
    queryKey: queryKeys.cycles.terms(yearId),
    queryFn: () => commands.academics.getTerms(orgId, yearId),
  });
}

export function useClasses(params?: JsonRecord) {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.classes.all(params),
    queryFn: () => commands.classes.getAll(orgId, params),
    enabled: !!orgId,
  });
}

export function useClassesSuspense(params?: JsonRecord) {
  const orgId = getOrgId();
  return useSuspenseQuery({
    queryKey: queryKeys.classes.all(params),
    queryFn: () => commands.classes.getAll(orgId, params),
  });
}

export function useCreateClass() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (request: JsonRecord) =>
      commands.classes.create(getOrgId(), request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["classes"] }),
  });
}

export function useUpdateClass() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({
      classId,
      request,
    }: {
      classId: string;
      request: JsonRecord;
    }) => commands.classes.update(classId, request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["classes"] }),
  });
}

export function useDeleteClass() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (classId: string) => commands.classes.delete(classId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["classes"] }),
  });
}

export function useSubjects() {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.subjects.all(orgId),
    queryFn: () => commands.subjects.getAll(orgId),
    enabled: !!orgId,
  });
}

export function useDashboardStats() {
  const orgId = getOrgId();
  return useQuery({
    queryKey: queryKeys.dashboard.stats(orgId),
    queryFn: () => commands.dashboard.getStats(orgId),
    enabled: !!orgId,
  });
}

export function useWidgetPreferences(userId?: string) {
  const id = userId ?? getUserId();
  return useQuery({
    queryKey: queryKeys.widgets.preferences(id),
    queryFn: () => commands.widgets.getWidgetPreferences(id),
    enabled: !!id,
  });
}

export function useUpdateWidgetPreferences() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({
      userId,
      preferences,
    }: {
      userId: string;
      preferences: unknown;
    }) => commands.widgets.updateWidgetPreferences(userId, preferences),
    onSuccess: (_, { userId }) => {
      qc.invalidateQueries({
        queryKey: queryKeys.widgets.preferences(userId),
      });
    },
  });
}
