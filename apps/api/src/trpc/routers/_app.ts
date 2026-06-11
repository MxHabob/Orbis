import type { inferRouterInputs, inferRouterOutputs } from "@trpc/server";
import { createTRPCRouter } from "../init";
import { academicRouter } from "./academic";
import { assessmentsRouter } from "./assessments";
import { attendanceRouter } from "./attendance";
import { auditLogsRouter } from "./audit-logs";
import { certificateTemplateRouter } from "./certificate-templates";
import { classesRouter } from "./classes";
import { documentsRouter } from "./documents";
import { gradesRouter } from "./grades";
import { jobsRouter } from "./jobs";
import { lessonsRouter } from "./lessons";
import { notificationsRouter } from "./notifications";
import { parentStudentsRouter } from "./parent-students";
import { promotionsRouter } from "./promotions";
import { reportsRouter } from "./reports";
import { searchRouter } from "./search";
import { shortLinksRouter } from "./short-links";
import { studentsRouter } from "./students";
import { subjectsRouter } from "./subjects";
import { userRouter } from "./user";
import { widgetsRouter } from "./widgets";

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
