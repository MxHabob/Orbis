import * as m from "$lib/paraglide/messages";
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

export function createPromotionJobsQuery(orgId: () => string) {
  return createQuery({
    get queryKey() {
      return ["promotion-jobs", orgId()];
    },
    queryFn: () => commands.enrollment_workflow.listJobs(),
    get enabled() {
      return !!orgId();
    },
    staleTime: 60 * 1000,
  });
}

export function createPromotionJobQuery(jobId: string) {
  return createQuery({
    queryKey: ["promotionJobs", "detail", jobId],
    queryFn: () => commands.enrollment_workflow.getJob(jobId),
    enabled: !!jobId,
    staleTime: 30 * 1000,
  });
}

export function createPromotionDetailsQuery(jobId: string) {
  return createQuery({
    queryKey: ["promotionDetails", jobId],
    queryFn: () => commands.enrollment_workflow.getJob(jobId),
    enabled: !!jobId,
    staleTime: 30 * 1000,
  });
}

export function createDryRunMutation(orgId: string) {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (config: PromotionConfig) =>
      commands.enrollment_workflow.startDryRun(config.source_period_id || "", config as any),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
    },
  });
}

export function createExecutePromotionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (jobId: string) => commands.enrollment_workflow.execute(jobId),
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
      commands.enrollment_workflow.execute((request.jobId as string) || ""),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionDetails"] });
    },
  });
}

export function createRollbackMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: ({ jobId, userId }: { jobId: string; userId: string }) =>
      commands.enrollment_workflow.rollback(jobId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  });
}
