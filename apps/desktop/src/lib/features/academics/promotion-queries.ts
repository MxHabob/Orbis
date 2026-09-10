/**
 * Academic Year & Promotion queries — TanStack Query hooks.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

export function createPromotionJobsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["promotion-jobs", orgId()],
    queryFn: () => api.listEnrollments(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

export function createPromotionJobQuery(jobId: string | (() => string)) {
  const getJobId = typeof jobId === "function" ? jobId : () => jobId;
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollment(getJobId()),
    queryFn: () => api.getEnrollment(getJobId()),
    enabled: !!getJobId(),
    staleTime: 30 * 1000,
  }));
}

export function createPromotionDetailsQuery(jobId: string | (() => string)) {
  const getJobId = typeof jobId === "function" ? jobId : () => jobId;
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollment(getJobId()),
    queryFn: () => api.getEnrollment(getJobId()),
    enabled: !!getJobId(),
    staleTime: 30 * 1000,
  }));
}

export function createDryRunMutation(_orgId: string) {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_config: any) => {
      return { success: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotion-jobs"] });
    },
  }));
}

export function createExecutePromotionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_jobId: string) => {
      return { success: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotion-jobs"] });
      queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
    },
  }));
}

export function createOverrideDecisionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_request: Record<string, unknown>) => {
      return { success: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotion-jobs"] });
    },
  }));
}

export function createRollbackMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_params: { jobId: string; userId: string }) => {
      return { success: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotion-jobs"] });
      queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
    },
  }));
}
