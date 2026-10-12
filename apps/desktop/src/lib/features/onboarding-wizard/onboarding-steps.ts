import type { SetupTaskDto } from "$lib/api/types";
import * as m from "$lib/paraglide/messages";

// ═══════════════════════════════════════════════════════════════
// Onboarding Step Model
// ═══════════════════════════════════════════════════════════════

export type OnboardingStepStatus =
  | "completed"
  | "in_progress"
  | "not_started"
  | "blocked"
  | "optional"
  | "not_applicable";

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
  /** Backend task IDs the user can access within this step */
  accessibleTaskIds: string[];
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
    taskIds: ["academic_calendar", "academic_periods"],
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
    id: "scheduling",
    route: "scheduling",
    taskIds: ["scheduling_setup"],
    getLabel: () => m.onboarding_step_scheduling_label(),
    getDescription: () => m.onboarding_step_scheduling_desc(),
    order: 5,
  },
  {
    id: "finance",
    route: "finance",
    taskIds: ["chart_of_accounts", "fee_templates", "discount_templates"],
    getLabel: () => m.onboarding_step5_label(),
    getDescription: () => m.onboarding_step5_desc(),
    order: 6,
  },
  {
    id: "assessment",
    route: "assessment",
    taskIds: ["calculation_policy"],
    getLabel: () => m.onboarding_step6_label(),
    getDescription: () => m.onboarding_step6_desc(),
    order: 7,
  },
  {
    id: "operations",
    route: "operations",
    taskIds: [
      "student_registration_policy",
      "document_requirements",
      "system_templates",
    ],
    getLabel: () => m.onboarding_step7_label(),
    getDescription: () => m.onboarding_step7_desc(),
    order: 8,
  },
  {
    id: "integrations",
    route: "integrations",
    taskIds: ["integrations_setup"],
    getLabel: () => m.onboarding_step_integrations_label(),
    getDescription: () => m.onboarding_step_integrations_desc(),
    order: 9,
  },
  {
    id: "review",
    route: "review",
    taskIds: [], // Review uses ALL tasks
    getLabel: () => m.onboarding_step8_label(),
    getDescription: () => m.onboarding_step8_desc(),
    order: 10,
  },
];

// ═══════════════════════════════════════════════════════════════
// Step Derivation — produces OnboardingStep[] from backend tasks
// ═══════════════════════════════════════════════════════════════

export function deriveSteps(
  tasks: SetupTaskDto[],
  hasPermission: (permission: string) => boolean,
): OnboardingStep[] {
  const taskById = new Map(tasks.map((task) => [task.id, task]));
  return STEP_DEFINITIONS.map((def) => {
    // Review step uses ALL tasks
    const stepTasks =
      def.id === "review"
        ? tasks
        : tasks.filter((t) => def.taskIds.includes(t.id));

    const status = deriveStepStatus(stepTasks, def.id === "review");
    const progress = deriveStepProgress(stepTasks);
    const isRequired = stepTasks.some((t) => t.is_required);
    const accessibleTaskIds = stepTasks.filter((task) => {
      const canAccess = task.status !== "blocked" &&
        task.status !== "not_applicable" &&
        task.required_permissions.every(hasPermission);
      const dependenciesMet = task.dependencies.every((dependencyId) => {
        const dependency = taskById.get(dependencyId);
        return dependency?.status === "completed" ||
          dependency?.status === "optional" ||
          dependency?.status === "not_applicable";
      });
      return canAccess && dependenciesMet;
    }).map((task) => task.id);
    const isAccessible = def.id === "review" || accessibleTaskIds.length > 0;

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
      accessibleTaskIds,
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
    if (requiredTasks.every((t) =>
      t.status === "completed" || t.status === "not_applicable"
    )) return "completed";
    if (requiredTasks.some((t) => t.status === "completed")) return "in_progress";
    return "not_started";
  }

  const applicableTasks = tasks.filter((task) => task.status !== "not_applicable");
  if (applicableTasks.length === 0) return "not_applicable";
  const requiredTasks = applicableTasks.filter((task) => task.is_required);
  const statusTasks = requiredTasks.length > 0 ? requiredTasks : applicableTasks;
  if (statusTasks.every((task) => task.status === "completed")) return "completed";
  if (statusTasks.every((task) => task.status === "optional")) return "optional";
  if (statusTasks.some((task) => task.status === "blocked")) return "blocked";
  if (
    statusTasks.some(
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
  const requiredTasks = tasks.filter((task) => task.is_required);
  const progressTasks = requiredTasks.length > 0 ? requiredTasks : tasks;
  const sum = progressTasks.reduce((acc, task) => acc + task.progress, 0);
  return Math.round(sum / progressTasks.length);
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
        s.status !== "not_applicable" &&
        s.isAccessible,
    ) ?? null
  );
}

export function findStepForTask(
  steps: OnboardingStep[],
  taskId: string | undefined,
): OnboardingStep | null {
  if (!taskId) return null;
  return steps.find((step) => step.taskIds.includes(taskId)) ?? null;
}

export function isTaskAccessible(
  steps: OnboardingStep[],
  taskId: string | undefined,
): boolean {
  const step = findStepForTask(steps, taskId);
  return Boolean(taskId && step?.accessibleTaskIds.includes(taskId));
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
