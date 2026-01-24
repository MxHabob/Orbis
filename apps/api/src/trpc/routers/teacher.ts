import {
  addTeacherSubjectSchema,
  assignTeacherToClassSchema,
  createTeacherSchema,
  getTeacherClassAssignmentsSchema,
  getTeachersSchema,
  updateTeacherSchema,
} from "@api/schemas/teacher";
import { getTeacherScheduleSchema } from "@api/schemas/lesson";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { teacherCache } from "@midday/cache/teacher-cache";
import { getTeacherSchedule } from "@midday/db/queries/lesson";
import {
  addTeacherSubject,
  assignTeacherToClass,
  createTeacher,
  deleteTeacher,
  getTeacherById,
  getTeacherClassAssignments,
  getTeacherSubjects,
  getTeachers,
  removeTeacherSubject,
  unassignTeacherFromClass,
  updateTeacher,
} from "@midday/db/queries/teacher";
import { z } from "zod";

export const teacherRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getTeachersSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getTeachers(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await teacherCache.getById(input.userId);
      if (cached) return cached;

      const result = await getTeacherById(db, { userId: input.userId });
      if (result) {
        await teacherCache.setById(input.userId, result);
      }
      return result;
    }),

  create: protectedProcedure
    .input(createTeacherSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createTeacher(db, input);
    }),

  update: protectedProcedure
    .input(updateTeacherSchema.extend({ userId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { userId, ...data } = input;
      const result = await updateTeacher(db, { userId, ...data });
      await teacherCache.invalidate(userId);
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await deleteTeacher(db, { userId: input.userId });
      await teacherCache.invalidate(input.userId);
      return result;
    }),

  // Subjects
  getSubjects: protectedProcedure
    .input(z.object({ teacherId: z.string().uuid() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await teacherCache.getSubjects(input.teacherId);
      if (cached) return cached;

      const result = await getTeacherSubjects(db, { teacherId: input.teacherId });
      if (result) {
        await teacherCache.setSubjects(input.teacherId, result);
      }
      return result;
    }),

  addSubject: protectedProcedure
    .input(addTeacherSubjectSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await addTeacherSubject(db, input);
      await teacherCache.invalidate(input.employeeId);
      return result;
    }),

  removeSubject: protectedProcedure
    .input(z.object({ id: z.number(), teacherId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await removeTeacherSubject(db, { id: input.id });
      await teacherCache.invalidate(input.teacherId);
      return result;
    }),

  // Assignments
  getClassAssignments: protectedProcedure
    .input(getTeacherClassAssignmentsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getTeacherClassAssignments(db, input);
    }),

  assignToClass: protectedProcedure
    .input(assignTeacherToClassSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await assignTeacherToClass(db, input);
      await teacherCache.invalidate(input.employeeId);
      return result;
    }),

  unassignFromClass: protectedProcedure
    .input(z.object({ id: z.number(), teacherId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await unassignTeacherFromClass(db, { id: input.id });
      await teacherCache.invalidate(input.teacherId);
      return result;
    }),

  // Schedule
  getSchedule: protectedProcedure
    .input(getTeacherScheduleSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getTeacherSchedule(db, input);
    }),
});
