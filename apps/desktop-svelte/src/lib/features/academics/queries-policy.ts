/**
 * Policy queries — TanStack Query wrappers for promotion policy.
 *
 * Policies are organization-level configuration that rarely changes.
 * Long staleTime (30min) to avoid unnecessary re-fetches.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Promotion policy for an organization (passing score, attendance, fee carry-over, etc.). */
export function createPromotionPolicyQuery(orgId: string) {
  return createQuery({
    queryKey: ["promotion-policy", orgId],
    queryFn: () => commands.promotion.getPolicy(orgId),
    enabled: !!orgId,
    staleTime: 30 * 60 * 1000, // 30 minutes — policies rarely change
  });
}

/** Organization academic policy (automation level, enrollment/grade policies, etc.). */
export function createOrgAcademicPolicyQuery(orgId: string) {
  return createQuery({
    queryKey: ["org-academic-policy", orgId],
    queryFn: () => commands.calendar.getPolicy(orgId),
    enabled: !!orgId,
    staleTime: 30 * 60 * 1000, // 30 minutes
  });
}
