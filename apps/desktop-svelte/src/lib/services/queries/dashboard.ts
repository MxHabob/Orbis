/**
 * Dashboard domain queries — stats overview.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/services/ipc";

/** Dashboard statistics for the overview page. */
export function createDashboardStatsQuery(orgId: string) {
  return createQuery({
    queryKey: ["dashboard-stats", orgId],
    queryFn: () => commands.dashboard.getStats(orgId),
    enabled: !!orgId,
    staleTime: 3 * 60 * 1000, // 3 minutes
  });
}
