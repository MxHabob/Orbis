import {
  createStudentSchema,
  updateStudentSchema,
  upsertStudentSchema,
} from "@api/schemas/students";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createStudent,
  deleteStudent,
  getStudentByNumber,
  getStudentById,
  updateStudent,
  getStudentByUserId,
  getStudentsByClassId,
  getAllStudents,
  upsertStudent,
  getStudentEnrollments,
} from "@midday/db/queries";
import { z } from "zod";

export const studentsRouter = createTRPCRouter({
  getAll: protectedProcedure.query(async () => {
    return getAllStudents();
  }),
  getByClassId: protectedProcedure
    .input(
      z.object({
        id: z.string(),
        cursor: z.number().nullish(),
        limit: z.number().min(1).max(100).default(50),
        q: z.string().optional(),
        sort: z.array(z.string()).nullish(),
      })
    )
    .query(async ({ input }) => {
      const limit = input.limit ?? 50;
      const cursor = input.cursor ?? 0;
      
      const data = await getStudentsByClassId(
        Number(input.id),
        limit + 1,
        cursor,
        input.q,
        input.sort
      );

      let nextCursor: typeof cursor | null = null;
      if (data.length > limit) {
        data.pop();
        nextCursor = cursor + limit;
      }

      return {
        data,
        meta: {
          cursor: nextCursor,
        },
      };
    }),
  getStudentByUserId: protectedProcedure
    .input(z.object({ userId: z.string() }))
    .query(async ({ input }) => {
      return getStudentByUserId(input.userId);
    }),
  getById: protectedProcedure
    .input(z.object({ id: z.string() }))
    .query(async ({ input }) => {
      return getStudentById(input.id);
    }),

  getByNumber: protectedProcedure
    .input(z.object({ studentNumber: z.string() }))
    .query(async ({ input }) => {
      return getStudentByNumber(input.studentNumber);
    }),

  getEnrollments: protectedProcedure
    .input(z.object({ studentId: z.string().uuid() }))
    .query(async ({ input }) => {
      return getStudentEnrollments(input.studentId);
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
      academicYearId: z.number().optional(),
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
      return bulkImportStudents(input.classId, input.students, input.academicYearId);
    }),
  upsert: protectedProcedure
    .input(upsertStudentSchema)
    .mutation(async ({ input }) => {
      return upsertStudent(input);
    }),
});

