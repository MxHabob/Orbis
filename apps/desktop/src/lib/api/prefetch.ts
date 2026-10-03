/**
 * Smart Prefetching — warms TanStack Query cache before navigation.
 *
 * Preloads data for the destination page on user interaction.
 */
import { queryClient } from "$lib/api";
import { api, queryKeys } from "$lib/api";

export function prefetchRouteData(route: string, orgId: string) {
  if (!orgId) return;

  switch (route) {
    case "students":
      queryClient.prefetchQuery({
        queryKey: queryKeys.student.enrollments(),
        queryFn: () => api.listEnrollments(),
        staleTime: 2 * 60 * 1000,
      });
      queryClient.prefetchQuery({
        queryKey: queryKeys.academic.classes(),
        queryFn: () => api.listClasses(),
        staleTime: 5 * 60 * 1000,
      });
      break;

    case "finance":
      queryClient.prefetchQuery({
        queryKey: queryKeys.finance.accounts(),
        queryFn: async () => {
          const accounts = await api.listAccounts();
          return accounts.filter((account: any) =>
            ["cash", "bank"].includes(account.account_type || account.sub_type),
          );
        },
        staleTime: 5 * 60 * 1000,
      });
      break;

    case "academics":
      queryClient.prefetchQuery({
        queryKey: queryKeys.academic.classes(),
        queryFn: () => api.listClasses(),
        staleTime: 5 * 60 * 1000,
      });
      break;

    case "settings":
      break;

    case "dashboard":
      queryClient.prefetchQuery({
        queryKey: queryKeys.org.dashboard(),
        queryFn: () => api.getDashboard(),
        staleTime: 3 * 60 * 1000,
      });
      break;
  }
}
