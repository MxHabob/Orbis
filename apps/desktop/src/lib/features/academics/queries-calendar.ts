/**
 * Academic Calendar & Policy queries.
 */
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

/** Periods for a given year. */
export function createCalendarEventsQuery(
  orgId: string | (() => string),
  periodId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  const getPeriodId =
    typeof periodId === "function" ? periodId : () => periodId;
  return createQuery(() => ({
    queryKey: queryKeys.calendar.periods(),
    queryFn: () => api.listAcademicPeriods(),
    enabled: !!getOrgId() && !!getPeriodId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Pending/active periods. */
export function createPendingCalendarEventsQuery(
  orgId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.calendar.periods(),
    queryFn: () => api.listAcademicPeriods(),
    enabled: !!getOrgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Unified academic policy for org. */
export function createOrgPolicyQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.org.settings(),
    queryFn: () => api.getSettings(),
    enabled: !!getOrgId(),
    staleTime: 30 * 60 * 1000,
  }));
}
