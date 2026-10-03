/**
 * Academic Year & Student Cohort Promotion queries — TanStack Query hooks.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type PromotionJobDto,
  type StudentProgressionItemInput,
  type PromoteCohortInput,
} from "$lib/api";

/** List all cohort promotion jobs */
export function createPromotionJobsQuery(orgId: () => string) {
  return createQuery<PromotionJobDto[]>(() => ({
    queryKey: queryKeys.progression.jobs(orgId()),
    queryFn: () => api.listPromotionJobs(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
  }));
}

/** Get a single promotion job by ID */
export function createPromotionJobQuery(jobId: string | (() => string)) {
  const getJobId = typeof jobId === "function" ? jobId : () => jobId;
  return createQuery<PromotionJobDto>(() => ({
    queryKey: queryKeys.progression.job(getJobId()),
    queryFn: () => api.getPromotionJob(getJobId()),
    enabled: !!getJobId(),
    staleTime: 30 * 1000,
  }));
}

/** Evaluate student cohort promotion readiness and recommendations */
export function createEvaluateCohortMutation() {
  return createMutation(() => ({
    mutationFn: (sourcePeriodId: string): Promise<StudentProgressionItemInput[]> =>
      api.evaluateCohort(sourcePeriodId),
  }));
}

/** Execute a cohort promotion job */
export function createPromoteCohortMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: PromoteCohortInput): Promise<PromotionJobDto> =>
      api.promoteCohort(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.progression.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
    },
  }));
}

/** Rollback mutation (kept for backwards compatibility) */
export function createRollbackMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: async (_params: { jobId: string; userId?: string }) => {
      return { success: true };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.progression.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
    },
  }));
}

