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
  type progressionJobDto,
  type StudentProgressionItemDto,
  type ProgressionDetailDto,
  type PromoteCohortInput,
} from "$lib/api";

/** List all cohort promotion jobs */
export function createprogressionJobsQuery(orgId: () => string) {
  return createQuery<progressionJobDto[]>(() => ({
    queryKey: queryKeys.progression.jobs(orgId()),
    queryFn: () => api.listprogressionJobs(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
    refetchInterval: 5 * 1000,
  }));
}

/** Get a single promotion job by ID */
export function createprogressionJobQuery(jobId: string | (() => string)) {
  const getJobId = typeof jobId === "function" ? jobId : () => jobId;
  return createQuery<progressionJobDto>(() => ({
    queryKey: queryKeys.progression.job(getJobId()),
    queryFn: () => api.getprogressionJob(getJobId()),
    enabled: !!getJobId(),
    staleTime: 30 * 1000,
  }));
}

/** Get per-student details for a promotion job */
export function createProgressionJobDetailsQuery(jobId: () => string | null) {
  return createQuery<ProgressionDetailDto[]>(() => ({
    queryKey: [...queryKeys.progression.job(jobId() ?? ""), "details"],
    queryFn: () => api.getprogressionJobDetails(jobId()!),
    enabled: !!jobId(),
    staleTime: 60 * 1000,
  }));
}

/** Evaluate student cohort promotion readiness and recommendations */
export function createEvaluateCohortMutation() {
  return createMutation(() => ({
    mutationFn: ({
      sourcePeriodId,
      targetPeriodId,
    }: {
      sourcePeriodId: string;
      targetPeriodId?: string;
    }): Promise<StudentProgressionItemDto[]> =>
      api.evaluateCohort(sourcePeriodId, targetPeriodId),
  }));
}

/** Execute a cohort promotion job */
export function createPromoteCohortMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: PromoteCohortInput): Promise<progressionJobDto> =>
      api.promoteCohort(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.progression.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
    },
  }));
}

/** Rollback a completed cohort promotion job */
export function createRollbackPromotionMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (jobId: string): Promise<progressionJobDto> =>
      api.rollbackprogressionJob(jobId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.progression.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}
