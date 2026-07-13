/**
 * Academic Year & Promotion queries — TanStack Query hooks.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import type { PromotionConfig } from "@orbis/sdk-core";

export function createCurrentYearQuery(orgId: string) {
  return createQuery({
    queryKey: ["academicYears", "current", orgId],
    queryFn: () => commands.platform.getCurrentYear(orgId),
    enabled: !!orgId,
    staleTime: 30 * 60 * 1000,
  });
}

export function createIsArchivedQuery(periodId: string) {
  return createQuery({
    queryKey: ["academicYears", "isArchived", periodId],
    queryFn: () => commands.platform.isYearArchived(periodId),
    enabled: !!periodId,
    staleTime: 30 * 60 * 1000,
  });
}

export function createSwitchYearMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (periodId: string) => commands.platform.switchYear(periodId),
    onSuccess: () => {
      queryClient.invalidateQueries();
    },
  });
}

export function createPromotionJobsQuery(orgId: string) {
  return createQuery({
    queryKey: ["promotionJobs", orgId],
    queryFn: () => commands.platform.listPromotionJobs(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}

export function createPromotionJobQuery(jobId: string) {
  return createQuery({
    queryKey: ["promotionJobs", "detail", jobId],
    queryFn: () => commands.platform.getPromotionJob(jobId),
    enabled: !!jobId,
    staleTime: 30 * 1000,
  });
}

export function createPromotionDetailsQuery(jobId: string) {
  return createQuery({
    queryKey: ["promotionDetails", jobId],
    queryFn: () => commands.platform.getPromotionDetails(jobId),
    enabled: !!jobId,
    staleTime: 30 * 1000,
  });
}

export function createDryRunMutation(orgId: string) {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (config: PromotionConfig) =>
      commands.platform.promotionDryRun(orgId, config),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
    },
  });
}

export function createExecutePromotionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (jobId: string) => commands.platform.executePromotion(jobId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  });
}

export function createOverrideDecisionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (request: Record<string, unknown>) =>
      commands.platform.overridePromotionDecision(request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionDetails"] });
    },
  });
}

export function createRollbackMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: ({ jobId, userId }: { jobId: string; userId: string }) =>
      commands.platform.rollbackPromotion(jobId, userId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  });
}
