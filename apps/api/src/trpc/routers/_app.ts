import type { inferRouterInputs, inferRouterOutputs } from "@trpc/server";
import { createTRPCRouter } from "../init";
import { academicRouter } from "./academic";
import { assessmentsRouter } from "./assessments";
import { auditLogsRouter } from "./audit-logs";
import { classesRouter } from "./classes";
import { notificationsRouter } from "./notifications";
import { studentsRouter } from "./students";
import { subjectsRouter } from "./subjects";
import { userRouter } from "./user";

export const appRouter = createTRPCRouter({
  academic: academicRouter,
  assessments: assessmentsRouter,
  auditLogs: auditLogsRouter,
  classes: classesRouter,
  notifications: notificationsRouter,
  students: studentsRouter,
  subjects: subjectsRouter,
  user: userRouter,
});

// export type definition of API
export type AppRouter = typeof appRouter;
export type RouterOutputs = inferRouterOutputs<AppRouter>;
export type RouterInputs = inferRouterInputs<AppRouter>;
