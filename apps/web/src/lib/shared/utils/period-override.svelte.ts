import { periodOverrideTargets, type PeriodOverrideTarget } from "$lib/api/period-override";

class PeriodOverrideStore {
  request = $state<{ orgId: string; targets: PeriodOverrideTarget[] } | null>(null);

  open(orgId: string, targets: PeriodOverrideTarget[]): boolean {
    if (!orgId || targets.length === 0) return false;
    const unique = new Map(targets.map((target) => [
      `${target.override_type}:${target.entity_id}:${target.period_id}`, target,
    ]));
    this.request = { orgId, targets: [...unique.values()] };
    return true;
  }

  handleError(orgId: string, error: unknown): boolean {
    return this.open(orgId, periodOverrideTargets(error));
  }

  dismiss() {
    this.request = null;
  }
}

export const periodOverrides = new PeriodOverrideStore();
