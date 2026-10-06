import type { SetupTaskDto, OnboardingTaskCategory } from "$lib/api/types";
import type { Component } from "svelte";

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
  /** English label */
  label: string;
  /** Arabic label */
  labelAr: string;
  /** Short description */
  description: string;
  /** Arabic description */
  descriptionAr: string;
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

// ═══════════════════════════════════════════════════════════════
// Step Definitions (Static — maps backend tasks to UX steps)
// ═══════════════════════════════════════════════════════════════

export interface StepDefinition {
  id: string;
  route: string;
  taskIds: string[];
  label: string;
  labelAr: string;
  description: string;
  descriptionAr: string;
  order: number;
}

export const STEP_DEFINITIONS: StepDefinition[] = [
  {
    id: "organization",
    route: "organization",
    taskIds: ["org_profile"],
    label: "Organization",
    labelAr: "المنشأة",
    description: "Profile, branding, timezone, and locale",
    descriptionAr: "الملف التعريفي والهوية والإعدادات الإقليمية",
    order: 1,
  },
  {
    id: "structure",
    route: "structure",
    taskIds: ["org_units", "facilities_rooms"],
    label: "Structure",
    labelAr: "الهيكل التنظيمي",
    description: "Branches, campuses, and facilities",
    descriptionAr: "الفروع والمقرات والمرافق التعليمية",
    order: 2,
  },
  {
    id: "calendar",
    route: "calendar",
    taskIds: ["academic_year", "academic_periods"],
    label: "Academic Calendar",
    labelAr: "التقويم الدراسي",
    description: "Academic year, terms, and evaluation periods",
    descriptionAr: "العام الدراسي والفصول والفترات التقويمية",
    order: 3,
  },
  {
    id: "academics",
    route: "academics",
    taskIds: ["academic_subjects", "academic_classes", "teaching_assignments"],
    label: "Academic Structure",
    labelAr: "البنية الأكاديمية",
    description: "Subjects, classes, and teaching assignments",
    descriptionAr: "المواد والفصول وإسناد المعلمين",
    order: 4,
  },
  {
    id: "finance",
    route: "finance",
    taskIds: ["chart_of_accounts", "fee_templates", "discount_templates"],
    label: "Finance & Fees",
    labelAr: "المالية والرسوم",
    description: "Chart of accounts, fee templates, and discounts",
    descriptionAr: "دليل الحسابات وقوالب الرسوم والخصومات",
    order: 5,
  },
  {
    id: "assessment",
    route: "assessment",
    taskIds: ["assessment_schemes", "academic_policy"],
    label: "Assessment & Grading",
    labelAr: "التقييم والدرجات",
    description: "Grading schemes, policies, and pass/fail criteria",
    descriptionAr: "خطط التقييم وسياسات النجاح والرسوب",
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
    label: "Operations",
    labelAr: "العمليات",
    description: "Registration, documents, templates, and integrations",
    descriptionAr: "التسجيل والوثائق والنماذج وبوابات الربط",
    order: 7,
  },
  {
    id: "review",
    route: "review",
    taskIds: [], // Review uses ALL tasks
    label: "Review",
    labelAr: "المراجعة النهائية",
    description: "Review configuration and complete setup",
    descriptionAr: "مراجعة الإعدادات وإكمال التهيئة",
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
      stepTasks.some(
        (t) => t.status !== "blocked"
      );

    return {
      id: def.id,
      route: def.route,
      taskIds: def.taskIds,
      label: def.label,
      labelAr: def.labelAr,
      description: def.description,
      descriptionAr: def.descriptionAr,
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
    // Review is "completed" only when all required tasks are done
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
