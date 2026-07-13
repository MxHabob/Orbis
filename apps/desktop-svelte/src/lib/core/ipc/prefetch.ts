/**
 * Smart Prefetching — warms TanStack Query cache before navigation.
 *
 * Called on sidebar `mouseenter` to preload data for the destination page.
 * By the time the user clicks, data is already in cache → instant page load.
 */
import { queryClient } from "$lib/core/query/client";
import { commands } from "$lib/core/ipc";

export function prefetchRouteData(route: string, orgId: string) {
  if (!orgId) return;

  switch (route) {
    case "students":
      queryClient.prefetchQuery({
        queryKey: ["enrollments", orgId],
        queryFn: () => commands.academic.listEnrollments(orgId),
        staleTime: 2 * 60 * 1000,
      });
      queryClient.prefetchQuery({
        queryKey: ["classes", orgId],
        queryFn: () => commands.academic.listClasses(orgId),
        staleTime: 5 * 60 * 1000,
      });
      break;

    case "finance":
      queryClient.prefetchQuery({
        queryKey: ["treasuries", orgId],
        queryFn: () => commands.finance.listTreasuries(orgId),
        staleTime: 5 * 60 * 1000,
      });
      break;

    case "academics":
      queryClient.prefetchQuery({
        queryKey: ["classes", orgId],
        queryFn: () => commands.academic.listClasses(orgId),
        staleTime: 5 * 60 * 1000,
      });
      break;

    case "settings":
      // Settings pages are mostly forms, no heavy prefetch needed
      break;

    case "dashboard":
      queryClient.prefetchQuery({
        queryKey: ["dashboard-stats", orgId],
        queryFn: () => commands.organization.getDashboardStats(orgId),
        staleTime: 3 * 60 * 1000,
      });
      break;
  }
}
