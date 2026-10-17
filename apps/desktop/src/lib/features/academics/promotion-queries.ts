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
  type StudentProgressionItemInput,
  type PromoteCohortInput,
} from "$lib/api";

/** List all cohort promotion jobs */
export function createprogressionJobsQuery(orgId: () => string) {
  return createQuery<progressionJobDto[]>(() => ({
    queryKey: queryKeys.progression.jobs(orgId()),
    queryFn: () => api.listprogressionJobs(),
    enabled: !!orgId(),
    staleTime: 60 * 1000,
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
    mutationFn: (req: PromoteCohortInput): Promise<progressionJobDto> =>
      api.promoteCohort(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.progression.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
    },
  }));
}

