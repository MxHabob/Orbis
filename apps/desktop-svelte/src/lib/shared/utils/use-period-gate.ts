/**
 * use-period-gate — Reactive composable for Period Gate checks.
 *
 * Wraps the `checkPeriodGate` / `checkEnrollmentEligibility` SDK calls
 * in a TanStack Query with 30s staleTime (matching backend cache TTL).
 *
 * Usage:
 *   const gate = usePeriodGate("enrollment", () => periodId, () => classId);
 *   // gate.data → GateVerdict
 *   // gate.isAllowed → boolean shortcut
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import type { GateType, GateVerdict } from "@orbis/sdk-core";

export function usePeriodGate(
  gateType: GateType,
  periodId: () => string,
  classId?: () => string | undefined,
) {
  const query = createQuery(() => ({
    queryKey: ["period-gate", gateType, periodId(), classId?.() ?? ""],
    queryFn: async (): Promise<GateVerdict> => {
      if (gateType === "enrollment" && classId?.()) {
        return commands.academic.checkEnrollmentEligibility(
          periodId(),
          classId()!,
        );
      }
      return commands.academic.checkPeriodGate(gateType, periodId());
    },
    staleTime: 30_000,
    refetchOnWindowFocus: true,
    enabled: !!periodId(),
  }));

  return {
    get data() {
      return query.data as GateVerdict | undefined;
    },
    get isLoading() {
      return query.isLoading;
    },
    get isError() {
      return query.isError;
    },
    get isAllowed() {
      const d = query.data?.decision;
      return d === "allowed" || d === "late";
    },
    get isBlocked() {
      return query.data?.decision === "blocked";
    },
    get requiresOverride() {
      return query.data?.decision === "requires_override";
    },
    get reason() {
      return query.data?.reason ?? null;
    },
    get capacity() {
      return query.data?.context?.capacity ?? null;
    },
    refetch: () => query.refetch(),
  };
}
