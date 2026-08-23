import * as m from "$lib/paraglide/messages";
/**
 * Policy queries — TanStack Query wrappers for academic policy and promotion rules.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

export function createOrgAcademicPolicyQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["org-academic-policy", orgId()],
    queryFn: () => commands.platform_policy.getAcademicPolicy(orgId()),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

export function createPromotionPolicyQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["promotion-policy", orgId()],
    queryFn: () => commands.platform_policy.getAcademicPolicy(orgId()),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

export function useUpdateAcademicPolicy() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (vars: { orgId: string; payload: Record<string, any> }) =>
      commands.platform_policy.updateAcademicPolicy(vars.orgId, vars.payload),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({
        queryKey: ["org-academic-policy", vars.orgId],
      });
      queryClient.invalidateQueries({
        queryKey: ["promotion-policy", vars.orgId],
      });
      queryClient.invalidateQueries({
        queryKey: ["autopilot-config", vars.orgId],
      });
    },
  }));
}
