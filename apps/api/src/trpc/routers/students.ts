import {
  createStudentSchema,
  updateStudentSchema,
} from "@api/schemas/students";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createStudent,
  deleteStudent,
  getStudentByNumber,
  getStudentByUserId,
  updateStudent,
  getStudentsByClassId,
  getAllStudents,
} from "@midday/db/queries";
import { z } from "zod";

export const studentsRouter = createTRPCRouter({
  getAll: protectedProcedure.query(async () => {
    return getAllStudents();
  }),
  getByClassId: protectedProcedure
    .input(z.object({ classId: z.number() }))
    .query(async ({ input }) => {
      return getStudentsByClassId(input.classId);
    }),
  getByUserId: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .query(async ({ input }) => {
      return getStudentByUserId(input.userId);
    }),

  getByNumber: protectedProcedure
    .input(z.object({ studentNumber: z.string() }))
    .query(async ({ input }) => {
      return getStudentByNumber(input.studentNumber);
    }),

  create: protectedProcedure
    .input(createStudentSchema)
    .mutation(async ({ input }) => {
      return createStudent(input);
    }),

  update: protectedProcedure
    .input(updateStudentSchema)
    .mutation(async ({ input }) => {
      return updateStudent(input.userId, input);
    }),

  delete: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .mutation(async ({ input }) => {
      return deleteStudent(input.userId);
    }),

  import: protectedProcedure
    .input(z.object({
      classId: z.number(),
      students: z.array(z.object({
        studentNumber: z.string(),
        name: z.string(),
        phone: z.string().optional(),
        email: z.string().optional(),
        sex: z.enum(['male', 'female', 'unspecified']).optional()
      }))
    }))
    .mutation(async ({ input }) => {
      const { bulkImportStudents } = await import("@midday/db/queries");
      return bulkImportStudents(input.classId, input.students);
    }),
});

