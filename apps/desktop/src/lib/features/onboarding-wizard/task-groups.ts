import type {
  CategoryProgressDto,
  OnboardingStateDto,
  SetupTaskDto,
} from "$lib/api/types";

export interface OnboardingTaskGroup extends CategoryProgressDto {
  tasks: SetupTaskDto[];
}

export function groupOnboardingTasks(
  state: Pick<OnboardingStateDto, "categories" | "tasks">,
): OnboardingTaskGroup[] {
  const categoryIds = new Set(state.categories.map(({ category }) => category));
  const unknownCategories = [
    ...new Set(
      state.tasks
        .map(({ category }) => category)
        .filter((category) => !categoryIds.has(category)),
    ),
  ];

  if (unknownCategories.length > 0) {
    throw new Error(
      `Onboarding response is missing category summaries for: ${unknownCategories.join(", ")}`,
    );
  }

  return state.categories.map((category) => ({
    ...category,
    tasks: state.tasks.filter((task) => task.category === category.category),
  }));
}
