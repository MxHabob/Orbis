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
  return createQuery(() => ({
    queryKey: ["grade-scales", orgId()],
    queryFn: () => commands.assessment.listGradeScales(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Items for a specific grade scale. */
export function createGradeScaleItemsQuery(scaleId: () => string | undefined) {
  return createQuery(() => ({
    queryKey: ["grade-scale-items", scaleId()],
    queryFn: () => commands.assessment.listGradeScaleItems(scaleId()!),
    enabled: !!scaleId(),
    staleTime: 10 * 60 * 1000,
  }));
}

// ── Calculation Policies ────────────────────────────────────

/** All calculation policies for the current organization. */
export function createCalcPoliciesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["calc-policies", orgId()],
    queryFn: () => commands.assessment.listCalculationPolicies(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Single calculation policy by ID. */
export function createCalcPolicyQuery(policyId: () => string | undefined) {
  return createQuery(() => ({
    queryKey: ["calc-policy", policyId()],
    queryFn: () => commands.assessment.getCalcPolicy(policyId()!),
    enabled: !!policyId(),
    staleTime: 10 * 60 * 1000,
  }));
}

// ── Assessment Schemes ──────────────────────────────────────

/** All assessment schemes for current organization. */
export function createAssessmentSchemesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["assessment-schemes", orgId()],
    queryFn: () => commands.assessment.listAssessmentSchemes(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Components for a specific assessment scheme. */
export function createSchemeComponentsQuery(schemeId: () => string | undefined) {
  return createQuery(() => ({
    queryKey: ["scheme-components", schemeId()],
    queryFn: () => commands.assessment.listComponents(schemeId()!),
    enabled: !!schemeId(),
    staleTime: 10 * 60 * 1000,
  }));
}
