/**
 * Policy queries — TanStack Query wrappers for academic policy.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";
import type { CalculationEngineSettings } from "$lib/api";

export function createOrgAcademicPolicyQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: [...queryKeys.org.settings(), "calculation-engine"],
    queryFn: () => api.getCalculationEngineSettings(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

export function createPromotionPolicyQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.org.settings(),
    queryFn: async () => {
      const settings = await api.getSettings();
      return settings.enrollment;
    },
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

export function useUpdateAcademicPolicy() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (settings: CalculationEngineSettings) =>
      api.updateCalculationEngineSettings(settings),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [...queryKeys.org.settings(), "calculation-engine"],
      });
    },
  }));
}
