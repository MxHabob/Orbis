/**
 * Assessment Configuration Queries
 * TanStack Query wrappers for Grade Scales, Calculation Policies,
 * and Assessment Snapshots — the dynamic Assessment Engine data.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

// ── Grade Scales ────────────────────────────────────────────

/** All grade scales for the current organization. */
export function createGradeScalesQuery(orgId: () => string) {
  return createQuery({
    get queryKey() {
      return ["grade-scales", orgId()];
    },
    queryFn: () => commands.assessment.listGradeScales(),
    get enabled() {
      return !!orgId();
    },
    staleTime: 10 * 60 * 1000,
  });
}

/** Items for a specific grade scale. */
export function createGradeScaleItemsQuery(scaleId: () => string | undefined) {
  return createQuery({
    get queryKey() {
      return ["grade-scale-items", scaleId()];
    },
    queryFn: () => commands.assessment.listGradeScaleItems(scaleId()!),
    get enabled() {
      return !!scaleId();
    },
    staleTime: 10 * 60 * 1000,
  });
}

// ── Calculation Policies ────────────────────────────────────

/** All calculation policies for the current organization. */
export function createCalcPoliciesQuery(orgId: () => string) {
  return createQuery({
    get queryKey() {
      return ["calc-policies", orgId()];
    },
    queryFn: () => commands.assessment.listCalculationPolicies(),
    get enabled() {
      return !!orgId();
    },
    staleTime: 10 * 60 * 1000,
  });
}

/** Single calculation policy by ID. */
export function createCalcPolicyQuery(policyId: () => string | undefined) {
  return createQuery({
    get queryKey() {
      return ["calc-policy", policyId()];
    },
    queryFn: () => commands.assessment.getCalcPolicy(policyId()!),
    get enabled() {
      return !!policyId();
    },
    staleTime: 10 * 60 * 1000,
  });
}
