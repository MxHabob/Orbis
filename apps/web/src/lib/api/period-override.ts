export type PeriodOverrideTarget = {
  period_id: string;
  entity_id: string;
} & (
  | { override_type: "grade_change"; entity_type: "section_subject" }
  | {
      override_type: "late_enrollment";
      entity_type: "section" | "academic_period";
    }
);

export function gradePeriodOverride(
  periodId: string,
  sectionSubjectId: string,
): PeriodOverrideTarget {
  return {
    override_type: "grade_change",
    entity_type: "section_subject",
    entity_id: sectionSubjectId,
    period_id: periodId,
  };
}

export const ACTIVE_PERIOD_STATES = new Set([
  "active",
  "active_teaching",
  "final_exams",
  "grade_submission",
  "enrollment_open",
]);

export function isPeriodActive(
  period:
    | { status?: string | null; is_locked?: boolean | number | null }
    | undefined
    | null,
): boolean {
  if (!period || isPeriodLocked(period)) return false;
  return Boolean(period.status && ACTIVE_PERIOD_STATES.has(period.status));
}

export function isPeriodLocked(
  period: { is_locked?: boolean | number | null } | undefined | null,
): boolean {
  return Boolean(period?.is_locked);
}

export function isGradeEntryOpen(
  period:
    | { is_locked?: boolean | number | null; can_enter_grades?: boolean | null }
    | undefined
    | null,
): boolean {
  return Boolean(period && !isPeriodLocked(period) && period.can_enter_grades);
}

export function isGradePeriodOverrideable(
  period: { status: string; period_kind: string } | undefined,
): boolean {
  return (
    !!period &&
    period.status !== "archived" &&
    (period.period_kind === "teaching" || period.period_kind === "examination")
  );
}

function isPeriodOverrideTarget(value: unknown): value is PeriodOverrideTarget {
  if (typeof value !== "object" || !value) return false;
  if (
    !("period_id" in value) ||
    typeof value.period_id !== "string" ||
    !value.period_id
  )
    return false;
  if (
    !("entity_id" in value) ||
    typeof value.entity_id !== "string" ||
    !value.entity_id
  )
    return false;
  if (!("override_type" in value) || !("entity_type" in value)) return false;
  return (
    (value.override_type === "grade_change" &&
      value.entity_type === "section_subject") ||
    (value.override_type === "late_enrollment" &&
      (value.entity_type === "section" ||
        value.entity_type === "academic_period"))
  );
}

export function parsePeriodOverrideTargets(
  value: unknown,
): PeriodOverrideTarget[] {
  return Array.isArray(value) ? value.filter(isPeriodOverrideTarget) : [];
}

export function periodOverrideTargets(error: unknown): PeriodOverrideTarget[] {
  if (typeof error !== "object" || !error) return [];
  const payload = "error" in error ? error.error : error;
  if (
    typeof payload !== "object" ||
    !payload ||
    !("code" in payload) ||
    payload.code !== "PERIOD_OVERRIDE_REQUIRED" ||
    !("details" in payload)
  )
    return [];
  const details = payload.details;
  if (typeof details !== "object" || !details || !("targets" in details))
    return [];
  return parsePeriodOverrideTargets(details.targets);
}
