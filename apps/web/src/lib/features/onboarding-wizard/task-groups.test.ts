import { describe, expect, it } from "bun:test";
import { groupOnboardingTasks } from "./task-groups";
import type {
  CategoryProgressDto,
  OnboardingTaskCategory,
  SetupTaskDto,
} from "$lib/api/types";

const category = (id: OnboardingTaskCategory): CategoryProgressDto => ({
  category: id,
  label: id,
  label_ar: id,
  total_tasks: 1,
  completed_tasks: 0,
  progress_percentage: 0,
});

const task = (
  id: string,
  categoryId: OnboardingTaskCategory,
  status: SetupTaskDto["status"],
): SetupTaskDto => ({
  id,
  category: categoryId,
  title: id,
  title_ar: id,
  description: "",
  description_ar: "",
  is_required: true,
  status,
  dependencies: ["prerequisite"],
  progress: status === "completed" ? 100 : 0,
  route: "/settings",
  action_type: "navigate",
  institution_types: [],
  required_permissions: ["organization:write"],
  completion_condition: "server-defined",
});

describe("Onboarding task grouping", () => {
  it("groups tasks by server categories without changing task data", () => {
    const organizationTask = task("org_profile", "organization", "completed");
    const financeTask = task("fee_templates", "finance", "blocked");
    const groups = groupOnboardingTasks({
      categories: [category("organization"), category("finance")],
      tasks: [organizationTask, financeTask],
    });

    expect(groups.map(({ category: id }) => id)).toEqual([
      "organization",
      "finance",
    ]);
    expect(groups[0].tasks).toEqual([organizationTask]);
    expect(groups[1].tasks).toEqual([financeTask]);
    expect(groups[1].tasks[0].dependencies).toEqual(["prerequisite"]);
    expect(groups[1].tasks[0].required_permissions).toEqual([
      "organization:write",
    ]);
  });

  it("fails explicitly when the server omits a task category summary", () => {
    expect(() =>
      groupOnboardingTasks({
        categories: [category("organization")],
        tasks: [task("fee_templates", "finance", "not_started")],
      }),
    ).toThrow("missing category summaries for: finance");
  });
});
