import type { SetupTaskDto } from "$lib/api/types";

export type SetupTaskFilter =
  | "all"
  | "required"
  | "pending"
  | "blocked"
  | "completed";

type SearchableTask = Pick<
  SetupTaskDto,
  | "category"
  | "status"
  | "is_required"
  | "title"
  | "title_ar"
  | "description"
  | "description_ar"
>;

export function filterSetupTasks<Task extends SearchableTask>(
  tasks: readonly Task[],
  {
    category,
    status,
    search,
  }: { category: string; status: SetupTaskFilter; search: string },
): Task[] {
  const query = search.trim().toLocaleLowerCase();

  return tasks.filter((task) => {
    if (category !== "all" && task.category !== category) return false;
    if (status === "required" && !task.is_required) return false;
    if (
      status === "pending" &&
      task.status !== "not_started" &&
      task.status !== "in_progress"
    ) {
      return false;
    }
    if (
      (status === "blocked" || status === "completed") &&
      task.status !== status
    ) {
      return false;
    }
    return (
      !query ||
      [task.title, task.title_ar, task.description, task.description_ar].some(
        (value) => value?.toLocaleLowerCase().includes(query),
      )
    );
  });
}
