import { expect, test } from "bun:test";
import { gradePeriodOverride, periodOverrideTargets } from "./period-override";

test("reads the same period override targets from HTTP and desktop errors", () => {
  const target = gradePeriodOverride("period", "offering");
  const payload = {
    code: "PERIOD_OVERRIDE_REQUIRED",
    details: { targets: [target] },
  };
  expect(periodOverrideTargets(payload)).toEqual([target]);
  expect(periodOverrideTargets({ error: payload })).toEqual([target]);
});

test("ordinary errors and invalid period targets never open the override flow", () => {
  expect(
    periodOverrideTargets({
      code: "FORBIDDEN",
      details: { targets: [gradePeriodOverride("period", "offering")] },
    }),
  ).toEqual([]);
  expect(
    periodOverrideTargets({
      code: "PERIOD_OVERRIDE_REQUIRED",
      details: {
        targets: [
          {
            override_type: "late_enrollment",
            entity_type: "section_subject",
            entity_id: "offering",
            period_id: "period",
          },
          {
            override_type: "grade_change",
            entity_type: "section_subject",
            entity_id: "offering",
          },
        ],
      },
    }),
  ).toEqual([]);
});
