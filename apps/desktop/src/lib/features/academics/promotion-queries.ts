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
import type { PromotionConfig } from "@orbis/sdk";

export function createPromotionJobsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["promotion-jobs", orgId()],
    queryFn: () => commands.enrollment_workflow.listJobs(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

export function createPromotionJobQuery(jobId: string | (() => string)) {
  const getJobId = typeof jobId === "function" ? jobId : () => jobId;
  return createQuery(() => ({
    queryKey: ["promotionJobs", "detail", getJobId()],
    queryFn: () => commands.enrollment_workflow.getJob(getJobId()),
    enabled: !!getJobId(),
    staleTime: 30 * 1000,
  }));
}

export function createPromotionDetailsQuery(jobId: string | (() => string)) {
  const getJobId = typeof jobId === "function" ? jobId : () => jobId;
  return createQuery(() => ({
    queryKey: ["promotionDetails", getJobId()],
    queryFn: () => commands.enrollment_workflow.getJob(getJobId()),
    enabled: !!getJobId(),
    staleTime: 30 * 1000,
  }));
}

export function createDryRunMutation(orgId: string) {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (config: PromotionConfig) =>
      commands.enrollment_workflow.startDryRun(config.source_period_id || "", config as any),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
    },
  }));
}

export function createExecutePromotionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (jobId: string) => commands.enrollment_workflow.execute(jobId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  }));
}

export function createOverrideDecisionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (request: Record<string, unknown>) =>
      commands.enrollment_workflow.execute((request.jobId as string) || ""),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionDetails"] });
    },
  }));
}

export function createRollbackMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ jobId, userId }: { jobId: string; userId: string }) =>
      commands.enrollment_workflow.rollback(jobId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  }));
}
