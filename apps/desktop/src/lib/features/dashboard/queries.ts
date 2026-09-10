/**
 * Dashboard domain queries — stats overview.
 */
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

/** Dashboard statistics for the overview page. */
export function createDashboardStatsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.org.dashboard(),
    queryFn: () => api.getDashboard(),
    enabled: !!getOrgId(),
    staleTime: 3 * 60 * 1000,
  }));
}
