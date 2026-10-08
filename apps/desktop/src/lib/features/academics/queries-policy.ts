/**
 * Policy queries — TanStack Query wrappers for academic policy.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

export function createOrgAcademicPolicyQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.org.settings(),
    queryFn: async () => {
      const settings = await api.getSettings();
      return settings.academic;
    },
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
    mutationFn: async (vars: {
      orgId: string;
      payload?: Record<string, any>;
      policy?: Record<string, any>;
    }) => {
      const current = await api.getSettings();
      const policyData = vars.payload ?? vars.policy ?? {};
      return api.updateSettings({
        ...current,
        academic: {
          default_grading_system:
            current.academic?.default_grading_system || "percentage",
          ...policyData,
        },
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.org.settings(),
      });
    },
  }));
}
