/**
 * Dashboard domain queries — stats overview.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Dashboard statistics for the overview page. */
export function createDashboardStatsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: ["dashboard-stats", getOrgId()],
    queryFn: () => commands.organization.getDashboardStats(),
    enabled: !!getOrgId(),
    staleTime: 3 * 60 * 1000, // 3 minutes
  }));
}
