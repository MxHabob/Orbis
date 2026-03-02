import type { inferRouterInputs, inferRouterOutputs } from "@trpc/server";
import { createTRPCRouter } from "../init";
import { academicRouter } from "./academic";
import { attendanceRouter } from "./attendance";
import { assessmentsRouter } from "./assessments";
import { auditLogsRouter } from "./audit-logs";
import { certificateTemplateRouter } from "./certificate-templates";
import { classesRouter } from "./classes";
import { gradesRouter } from "./grades";
import { lessonsRouter } from "./lessons";
import { notificationsRouter } from "./notifications";
import { parentStudentsRouter } from "./parent-students";
import { studentsRouter } from "./students";
import { subjectsRouter } from "./subjects";
import { userRouter } from "./user";
import { widgetsRouter } from "./widgets";
import { reportsRouter } from "./reports";
import { jobsRouter } from "./jobs";
import { documentsRouter } from "./documents";
import { shortLinksRouter } from "./short-links";
import { promotionsRouter } from "./promotions";

import { searchRouter } from "./search";

export const appRouter = createTRPCRouter({
  academic: academicRouter,
  attendance: attendanceRouter,
  assessments: assessmentsRouter,
  auditLogs: auditLogsRouter,
  certificateTemplates: certificateTemplateRouter,
  classes: classesRouter,
  grades: gradesRouter,
  lessons: lessonsRouter,
  notifications: notificationsRouter,
  parentStudents: parentStudentsRouter,
  students: studentsRouter,
  subjects: subjectsRouter,
  user: userRouter,
  widgets: widgetsRouter,
  reports: reportsRouter,
  jobs: jobsRouter,
  documents: documentsRouter,
  shortLinks: shortLinksRouter,
  search: searchRouter,
  promotions: promotionsRouter,
});

// export type definition of API
export type AppRouter = typeof appRouter;
export type RouterOutputs = inferRouterOutputs<AppRouter>;
export type RouterInputs = inferRouterInputs<AppRouter>;
