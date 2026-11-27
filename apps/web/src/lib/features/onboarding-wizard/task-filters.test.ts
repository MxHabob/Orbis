import { expect, test } from "bun:test";
import { filterSetupTasks } from "./task-filters";

const calendar = {
  category: "calendar",
  status: "in_progress",
  is_required: true,
  title: "Academic year",
  title_ar: "السنة الدراسية",
  description: "Set the start date",
  description_ar: "تحديد تاريخ البداية",
} as const;

test("combines task status, category, and bilingual search", () => {
  const completedCalendar = { ...calendar, status: "completed" } as const;
  const blockedCalendar = { ...calendar, status: "blocked" } as const;
  const finance = { ...calendar, category: "finance" } as const;
  const tasks = [completedCalendar, blockedCalendar, calendar, finance];

  expect(
    filterSetupTasks(tasks, {
      category: "calendar",
      status: "pending",
      search: "  السنة ",
    }),
  ).toEqual([calendar]);
  expect(
    filterSetupTasks(tasks, {
      category: "all",
      status: "blocked",
      search: "START DATE",
    }),
  ).toEqual([blockedCalendar]);
  expect(
    filterSetupTasks(tasks, {
      category: "calendar",
      status: "completed",
      search: "",
    }),
  ).toEqual([completedCalendar]);
});
