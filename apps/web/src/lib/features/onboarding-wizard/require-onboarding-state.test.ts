import { describe, expect, it } from "bun:test";
import { requireOnboardingState } from "./require-onboarding-state";

describe("Onboarding state confirmation", () => {
  it("returns the authoritative state unchanged", () => {
    const state = {
      org_id: "org-1",
      org_name: "Orbis",
      institution_type: "k12_school",
      total_tasks: 1,
      completed_tasks: 1,
      required_tasks: 1,
      required_completed: 1,
      optional_tasks: 0,
      optional_completed: 0,
      blocked_tasks: 0,
      progress_percentage: 100,
      required_progress_percentage: 100,
      is_institution_ready: true,
      tasks: [],
      categories: [],
      warnings: [],
    };

    expect(requireOnboardingState(state, "refresh failed")).toBe(state);
  });

  it("does not manufacture task state after a failed server refresh", () => {
    expect(() => requireOnboardingState(null, "refresh failed")).toThrow(
      "refresh failed",
    );
  });
});
