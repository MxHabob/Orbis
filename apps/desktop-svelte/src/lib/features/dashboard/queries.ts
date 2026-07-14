/**
 * Dashboard domain queries — stats overview.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Dashboard statistics for the overview page. */
export function createDashboardStatsQuery(orgId: string) {
  return createQuery({
    queryKey: ["dashboard-stats", orgId],
    queryFn: () => commands.organization.getDashboardStats(),
    enabled: !!orgId,
    staleTime: 3 * 60 * 1000, // 3 minutes
  });
}
