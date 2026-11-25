import { describe, expect, it } from "bun:test";
import type { SetupTaskDto } from "$lib/api/types";
import {
  deriveSteps,
  findNextIncompleteStep,
  findStepForTask,
  isTaskAccessible,
  STEP_DEFINITIONS,
} from "./onboarding-steps";

function task(
  id: string,
  status: SetupTaskDto["status"],
  overrides: Partial<SetupTaskDto> = {},
): SetupTaskDto {
  return {
    id,
    category: "organization",
    title: id,
    title_ar: id,
    description: "",
    description_ar: "",
    is_required: true,
    status,
    dependencies: [],
    progress: status === "completed" ? 100 : 0,
    route: "",
    action_type: "",
    institution_types: [],
    required_permissions: [],
    completion_condition: "",
    ...overrides,
  };
}

describe("Onboarding step mapping", () => {
  it("maps every backend task ID to exactly one presentation step", () => {
    const backendTaskIds = [
      "org_profile",
      "org_units",
      "facilities_rooms",
      "academic_calendar",
      "academic_periods",
      "academic_subjects",
      "academic_classes",
      "teaching_assignments",
      "scheduling_setup",
      "calculation_policy",
      "chart_of_accounts",
      "fee_templates",
      "discount_templates",
      "student_registration_policy",
      "document_requirements",
      "system_templates",
      "integrations_setup",
    ].sort();
    const presentationTaskIds = STEP_DEFINITIONS.flatMap(
      (step) => step.taskIds,
    ).sort();

    expect(presentationTaskIds).toEqual(backendTaskIds);
    expect(new Set(presentationTaskIds).size).toBe(presentationTaskIds.length);
  });

  it("maps backend task IDs to their presentation step", () => {
    const hasPermission = () => true;
    const steps = deriveSteps(
      [
        task("org_profile", "completed"),
        task("org_units", "in_progress", { category: "organization" }),
      ],
      hasPermission,
    );

    expect(findStepForTask(steps, "org_profile")?.id).toBe("organization");
    expect(steps.find((step) => step.id === "organization")?.status).toBe(
      "completed",
    );
    expect(steps.find((step) => step.id === "structure")?.status).toBe(
      "in_progress",
    );
  });

  it("preserves completed, in-progress, blocked, optional, and not-applicable statuses", () => {
    const steps = deriveSteps(
      [
        task("org_profile", "completed"),
        task("academic_calendar", "blocked", {
          category: "calendar",
          dependencies: ["org_profile"],
        }),
        task("integrations_setup", "optional", {
          category: "integrations",
          is_required: false,
        }),
        task("calculation_policy", "not_applicable", {
          category: "assessment",
        }),
      ],
      () => true,
    );

    expect(steps.find((step) => step.id === "organization")?.status).toBe(
      "completed",
    );
    expect(steps.find((step) => step.id === "calendar")?.status).toBe(
      "blocked",
    );
    expect(steps.find((step) => step.id === "integrations")?.status).toBe(
      "optional",
    );
    expect(steps.find((step) => step.id === "assessment")?.status).toBe(
      "not_applicable",
    );
  });

  it("requires accessible permissions and satisfied dependencies", () => {
    const tasks = [
      task("org_profile", "in_progress", {
        required_permissions: ["organization:write"],
      }),
      task("org_units", "not_started", {
        category: "organization",
        dependencies: ["org_profile"],
        required_permissions: ["organization:write"],
      }),
      task("academic_calendar", "not_started", {
        category: "calendar",
        dependencies: ["org_profile"],
        required_permissions: ["calendar:write"],
      }),
    ];

    const noPermissions = deriveSteps(tasks, () => false);
    expect(
      noPermissions.find((step) => step.id === "organization")?.isAccessible,
    ).toBe(false);
    expect(
      noPermissions.find((step) => step.id === "calendar")?.isAccessible,
    ).toBe(false);

    const allPermissions = deriveSteps(tasks, () => true);
    expect(
      allPermissions.find((step) => step.id === "organization")?.isAccessible,
    ).toBe(true);
    expect(
      allPermissions.find((step) => step.id === "calendar")?.isAccessible,
    ).toBe(false);
  });

  it("does not make a step accessible when its backend task is blocked", () => {
    const steps = deriveSteps(
      [
        task("org_profile", "blocked", {
          blocking_reason: "Permission denied",
        }),
      ],
      () => true,
    );

    expect(steps.find((step) => step.id === "organization")?.isAccessible).toBe(
      false,
    );
  });

  it("keeps blocked tasks inaccessible when another task in the same step is available", () => {
    const steps = deriveSteps(
      [
        task("org_units", "completed", { category: "organization" }),
        task("facilities_rooms", "blocked", {
          category: "organization",
          dependencies: ["org_units"],
        }),
      ],
      () => true,
    );

    expect(steps.find((step) => step.id === "structure")?.isAccessible).toBe(
      true,
    );
    expect(isTaskAccessible(steps, "org_units")).toBe(true);
    expect(isTaskAccessible(steps, "facilities_rooms")).toBe(false);
  });

  it("does not keep the guided flow on optional tasks after required setup is complete", () => {
    const steps = deriveSteps(
      [
        task("org_profile", "completed"),
        task("org_units", "completed", { category: "organization" }),
        task("facilities_rooms", "optional", {
          category: "organization",
          is_required: false,
        }),
      ],
      () => true,
    );

    expect(steps.find((step) => step.id === "structure")?.status).toBe(
      "completed",
    );
    expect(steps.find((step) => step.id === "structure")?.progress).toBe(100);
    expect(findNextIncompleteStep(steps)).toBeNull();
  });
});
