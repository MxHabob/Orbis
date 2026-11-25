/**
 * Assessment Configuration Queries
 * TanStack Query wrappers for Assessment Schemes and Grades.
 */
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

// ── Grade Scales ────────────────────────────────────────────

/** All grade scales for the current organization. */
export function createGradeScalesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.assessment.schemes(),
    queryFn: () => api.listAssessmentSchemes(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Items for a specific grade scale. */
export function createGradeScaleItemsQuery(scaleId: () => string | undefined) {
  return createQuery(() => ({
    queryKey: queryKeys.assessment.schemes(),
    queryFn: () => api.listAssessmentSchemes(),
    enabled: !!scaleId(),
    staleTime: 10 * 60 * 1000,
  }));
}

// ── Calculation Policies ────────────────────────────────────

/** All calculation policies for the current organization. */
export function createCalcPoliciesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.assessment.schemes(),
    queryFn: () => api.listAssessmentSchemes(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Single calculation policy by ID. */
export function createCalcPolicyQuery(policyId: () => string | undefined) {
  return createQuery(() => ({
    queryKey: queryKeys.assessment.scheme(policyId()!),
    queryFn: () => api.getAssessmentScheme(policyId()!),
    enabled: !!policyId(),
    staleTime: 10 * 60 * 1000,
  }));
}

// ── Assessment Schemes ──────────────────────────────────────

/** All assessment schemes for current organization. */
export function createAssessmentSchemesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.assessment.schemes(),
    queryFn: () => api.listAssessmentSchemes(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Components for a specific assessment scheme. */
export function createSchemeComponentsQuery(
  schemeId: () => string | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.assessment.scheme(schemeId()!),
    queryFn: () => api.getAssessmentScheme(schemeId()!),
    enabled: !!schemeId(),
    staleTime: 10 * 60 * 1000,
  }));
}
