import type { inferRouterInputs, inferRouterOutputs } from "@trpc/server";
import { createTRPCRouter } from "../init";
import { academicYearRouter } from "./academic-year";
import { assessmentRouter } from "./assessment";
import { attendanceRouter } from "./attendance";
import { classRouter } from "./class";
import { examRouter } from "./exam";
import { gradeRouter } from "./grade";
import { lessonRouter } from "./lesson";
import { levelRouter } from "./level";
import { parentRouter } from "./parent";
import { paymentRouter } from "./payment";
import { reportRouter } from "./report";
import { studentRouter } from "./student";
import { subjectRouter } from "./subject";
import { teacherRouter } from "./teacher";
import { suggestedActionsRouter } from "./suggested-actions";
import { searchRouter } from "./search";
import { userRouter } from "./user";

export const appRouter = createTRPCRouter({
  academicYear: academicYearRouter,
  student: studentRouter,
  class: classRouter,
  teacher: teacherRouter,
  grade: gradeRouter,
  exam: examRouter,
  attendance: attendanceRouter,
  payment: paymentRouter,
  report: reportRouter,
  subject: subjectRouter,
  lesson: lessonRouter,
  parent: parentRouter,
  level: levelRouter,
  assessment: assessmentRouter,

  user: userRouter,
  search: searchRouter,
  suggestedActions: suggestedActionsRouter,
});

export type AppRouter = typeof appRouter;
export type RouterOutputs = inferRouterOutputs<AppRouter>;
export type RouterInputs = inferRouterInputs<AppRouter>;
