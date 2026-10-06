import type { SetupTaskDto } from "$lib/api/types";
import * as m from "$lib/paraglide/messages";

// ═══════════════════════════════════════════════════════════════
// Onboarding Step Model
// ═══════════════════════════════════════════════════════════════

export type OnboardingStepStatus =
  | "completed"
  | "in_progress"
  | "not_started"
  | "blocked";

export interface OnboardingStep {
  /** Unique step identifier */
  id: string;
  /** Route path relative to /[orgId]/onboarding/ */
  route: string;
  /** Backend task IDs grouped into this step */
  taskIds: string[];
  /** Localized label function / accessor */
  getLabel: () => string;
  /** Localized description function / accessor */
  getDescription: () => string;
  /** Localized step label getter */
  label: string;
  /** Localized step description getter */
  description: string;
  /** Step number (1-based) */
  order: number;
  /** Whether any task in this step is required */
  isRequired: boolean;
  /** Derived status from backend tasks */
  status: OnboardingStepStatus;
  /** Progress percentage (0-100) */
  progress: number;
  /** Whether the user can navigate to this step */
  isAccessible: boolean;
}

export interface StepDefinition {
  id: string;
  route: string;
  taskIds: string[];
  getLabel: () => string;
  getDescription: () => string;
  order: number;
}

export const STEP_DEFINITIONS: StepDefinition[] = [
  {
    id: "organization",
    route: "organization",
    taskIds: ["org_profile"],
    getLabel: () => m.onboarding_step1_label(),
    getDescription: () => m.onboarding_step1_desc(),
    order: 1,
  },
  {
    id: "structure",
    route: "structure",
    taskIds: ["org_units", "facilities_rooms"],
    getLabel: () => m.onboarding_step2_label(),
    getDescription: () => m.onboarding_step2_desc(),
    order: 2,
  },
  {
    id: "calendar",
    route: "calendar",
    taskIds: ["academic_year", "academic_periods"],
    getLabel: () => m.onboarding_step3_label(),
    getDescription: () => m.onboarding_step3_desc(),
    order: 3,
  },
  {
    id: "academics",
    route: "academics",
    taskIds: ["academic_subjects", "academic_classes", "teaching_assignments"],
    getLabel: () => m.onboarding_step4_label(),
    getDescription: () => m.onboarding_step4_desc(),
    order: 4,
  },
  {
    id: "finance",
    route: "finance",
    taskIds: ["chart_of_accounts", "fee_templates", "discount_templates"],
    getLabel: () => m.onboarding_step5_label(),
    getDescription: () => m.onboarding_step5_desc(),
    order: 5,
  },
  {
    id: "assessment",
    route: "assessment",
    taskIds: ["assessment_schemes", "academic_policy"],
    getLabel: () => m.onboarding_step6_label(),
    getDescription: () => m.onboarding_step6_desc(),
    order: 6,
  },
  {
    id: "operations",
    route: "operations",
    taskIds: [
      "student_registration_policy",
      "document_requirements",
      "system_templates",
      "integrations_setup",
    ],
    getLabel: () => m.onboarding_step7_label(),
    getDescription: () => m.onboarding_step7_desc(),
    order: 7,
  },
  {
    id: "review",
    route: "review",
    taskIds: [], // Review uses ALL tasks
    getLabel: () => m.onboarding_step8_label(),
    getDescription: () => m.onboarding_step8_desc(),
    order: 8,
  },
];

// ═══════════════════════════════════════════════════════════════
// Step Derivation — produces OnboardingStep[] from backend tasks
// ═══════════════════════════════════════════════════════════════

export function deriveSteps(tasks: SetupTaskDto[]): OnboardingStep[] {
  return STEP_DEFINITIONS.map((def) => {
    // Review step uses ALL tasks
    const stepTasks =
      def.id === "review"
        ? tasks
        : tasks.filter((t) => def.taskIds.includes(t.id));

    const status = deriveStepStatus(stepTasks, def.id === "review");
    const progress = deriveStepProgress(stepTasks);
    const isRequired = stepTasks.some((t) => t.is_required);

    // A step is accessible if it has at least one non-blocked task,
    // OR if it's the review step
    const isAccessible =
      def.id === "review" ||
      stepTasks.some((t) => t.status !== "blocked");

    return {
      id: def.id,
      route: def.route,
      taskIds: def.taskIds,
      getLabel: def.getLabel,
      getDescription: def.getDescription,
      get label() {
        return def.getLabel();
      },
      get description() {
        return def.getDescription();
      },
      order: def.order,
      isRequired,
      status,
      progress,
      isAccessible,
    };
  });
}

function deriveStepStatus(
  tasks: SetupTaskDto[],
  isReview: boolean,
): OnboardingStepStatus {
  if (tasks.length === 0) return "not_started";

  if (isReview) {
    const requiredTasks = tasks.filter((t) => t.is_required);
    if (requiredTasks.every((t) => t.status === "completed")) return "completed";
    if (requiredTasks.some((t) => t.status === "completed")) return "in_progress";
    return "not_started";
  }

  if (tasks.every((t) => t.status === "completed")) return "completed";
  if (tasks.every((t) => t.status === "blocked")) return "blocked";
  if (
    tasks.some(
      (t) =>
        t.status === "completed" ||
        t.status === "in_progress" ||
        t.progress > 0,
    )
  )
    return "in_progress";
  return "not_started";
}

function deriveStepProgress(tasks: SetupTaskDto[]): number {
  if (tasks.length === 0) return 0;
  const sum = tasks.reduce((acc, t) => acc + t.progress, 0);
  return Math.round(sum / tasks.length);
}

/**
 * Find the first incomplete, accessible step — used for resume navigation.
 */
export function findNextIncompleteStep(
  steps: OnboardingStep[],
): OnboardingStep | null {
  return (
    steps.find(
      (s) =>
        s.id !== "review" &&
        s.status !== "completed" &&
        s.isAccessible,
    ) ?? null
  );
}

/**
 * Get tasks for a specific step from the full task list.
 */
export function getTasksForStep(
  stepId: string,
  tasks: SetupTaskDto[],
): SetupTaskDto[] {
  const def = STEP_DEFINITIONS.find((d) => d.id === stepId);
  if (!def) return [];
  if (stepId === "review") return tasks;
  return tasks.filter((t) => def.taskIds.includes(t.id));
}
