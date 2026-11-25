/**
 * Dashboard domain queries — stats overview.
 */
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

import type { DashboardAccessDto, DashboardCountsDto } from "$lib/api/types";

export function dashboardBranch(
  access: DashboardAccessDto | undefined,
  saved: string | null,
): string | undefined {
  if (!access) return undefined;
  if (saved && access.units.some((unit) => unit.id === saved)) return saved;
  return access.organization_wide ? undefined : access.units[0]?.id;
}

export function createDashboardAccessQuery(orgId: () => string, userId: () => string) {
  return createQuery<DashboardAccessDto>(() => ({
    queryKey: queryKeys.org.dashboardAccess(orgId(), userId()),
    queryFn: () => api.getDashboardAccess(),
    enabled: !!orgId() && !!userId(),
    staleTime: 30_000,
  }));
}

/** Dashboard statistics for the overview page. */
export function createDashboardStatsQuery(
  orgId: () => string,
  userId: () => string,
  selectedBranch: () => string | null,
) {
  const access = createDashboardAccessQuery(orgId, userId);
  const stats = createQuery<DashboardCountsDto>(() => {
    const branch = dashboardBranch(access.data, selectedBranch());
    return {
      queryKey: [...queryKeys.org.dashboard(orgId()), userId(), branch ?? null],
      queryFn: () => api.getDashboard(branch),
      enabled: !!orgId() && !!userId() && access.isSuccess,
      staleTime: 30_000,
    };
  });
  return { access, stats };
}
