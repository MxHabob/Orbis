import * as m from "$lib/paraglide/messages";
/**
 * Academic Calendar & Policy queries.
 * Migrated: calendar events → periods, periodId → periodId.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Periods for a given year (replaces calendar events). */
export function createCalendarEventsQuery(
  orgId: string | (() => string),
  periodId: string | (() => string),
) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  const getPeriodId = typeof periodId === "function" ? periodId : () => periodId;
  return createQuery(() => ({
    queryKey: ["calendar-periods", getOrgId(), getPeriodId()],
    queryFn: () => commands.calendar.listAcademicPeriodsByYear(getPeriodId()),
    enabled: !!getOrgId() && !!getPeriodId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Pending/active periods (replaces pending calendar events). */
export function createPendingCalendarEventsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: ["calendar-pending", getOrgId()],
    queryFn: () => commands.calendar.listAcademicPeriods(),
    enabled: !!getOrgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Unified academic policy for org. */
export function createOrgPolicyQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: ["org-policy", getOrgId()],
    queryFn: () => commands.platform_policy.getAutopilotConfig(getOrgId()),
    enabled: !!getOrgId(),
    staleTime: 30 * 60 * 1000,
  }));
}
