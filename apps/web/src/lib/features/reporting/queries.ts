/**
 * Reporting domain queries and mutations.
 * Wraps OrbisApiClient report definitions and report generation with TanStack Query.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type ReportDefinitionDto,
  type ReportJobStatusDto,
} from "$lib/api";

/** Query for listing report definitions */
export function createReportDefinitionsQuery(
  orgId: () => string,
  filter?: () => { domain?: string; category?: string } | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.report.definitions(orgId(), filter?.()),
    queryFn: () => {
      const f = filter?.();
      return api.listReportDefinitions(f?.domain, f?.category);
    },
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Query for getting a single report definition by code */
export function createReportDefinitionQuery(code: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.report.definition(code()),
    queryFn: () => api.getReportDefinition(code()),
    enabled: !!code(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Query for checking a background report job status */
export function createReportJobStatusQuery(jobId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.report.jobStatus(jobId()),
    queryFn: () => api.getReportJobStatus(jobId()),
    enabled: !!jobId(),
    refetchInterval: (query) => {
      const data = query.state.data as ReportJobStatusDto | undefined;
      if (data?.status === "completed" || data?.status === "failed") {
        return false;
      }
      return 2000;
    },
  }));
}

/** Mutation for generating reports */
export function createGenerateReportMutation() {
  return createMutation(() => ({
    mutationFn: async (params: {
      reportCode: string;
      format?: string;
      parameters?: Record<string, unknown>;
    }) => {
      return api.generateReport(
        params.reportCode,
        params.format || "PDF",
        params.parameters || {},
      );
    },
  }));
}
