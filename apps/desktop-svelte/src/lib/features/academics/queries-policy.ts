import * as m from "$lib/paraglide/messages";
/**
 * Policy queries — TanStack Query wrappers for promotion/org policy.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

export function createPromotionPolicyQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["promotion-policy", orgId()],
    queryFn: () => commands.platform_policy.getAutopilotConfig(orgId()),
    enabled: !!orgId(),
    staleTime: 30 * 60 * 1000,
  }));
}

export function createOrgAcademicPolicyQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["org-academic-policy", orgId()],
    queryFn: () => commands.platform_policy.getAutopilotConfig(orgId()),
    enabled: !!orgId(),
    staleTime: 30 * 60 * 1000,
  }));
}
