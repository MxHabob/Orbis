import * as m from "$lib/paraglide/messages";
/**
 * Policy queries — TanStack Query wrappers for promotion/org policy.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

export function createPromotionPolicyQuery(orgId: () => string) {
  return createQuery({
    get queryKey() { return ["promotion-policy", orgId()]; },
    queryFn: () => commands.platform.getPromotionPolicy(),
    get enabled() { return !!orgId(); },
    staleTime: 30 * 60 * 1000,
  });
}

export function createOrgAcademicPolicyQuery(orgId: () => string) {
  return createQuery({
    get queryKey() { return ["org-academic-policy", orgId()]; },
    queryFn: () => commands.platform.getOrgPolicy(),
    get enabled() { return !!orgId(); },
    staleTime: 30 * 60 * 1000,
  });
}
