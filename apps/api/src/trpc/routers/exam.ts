import {
  bulkCreateExamSeatingsSchema,
  createExamCommitteeSchema,
  createExamSchema,
  createExamSeatingSchema,
  getExamsSchema,
  getStudentExamSeatingSchema,
  updateExamCommitteeSchema,
  updateExamSchema,
} from "@api/schemas/exam";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { examCache } from "@midday/cache/exam-cache";
import {
  bulkCreateExamSeatings,
  createExam,
  createExamCommittee,
  createExamSeating,
  deleteExam,
  deleteExamCommittee,
  deleteExamSeating,
  getExamById,
  getExamCommittees,
  getExamSeatings,
  getExams,
  getStudentExamSeating,
  updateExam,
  updateExamCommittee,
} from "@midday/db/queries/exam";
import { z } from "zod";

export const examRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getExamsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getExams(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await examCache.getById(input.id);
      if (cached) return cached;

      const result = await getExamById(db, { id: input.id });
      if (result) {
        await examCache.setById(input.id, result);
      }
      return result;
    }),

  create: protectedProcedure
    .input(createExamSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createExam(db, input);
    }),

  update: protectedProcedure
    .input(updateExamSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const result = await updateExam(db, { id, ...data });
      await examCache.invalidate(id);
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await deleteExam(db, { id: input.id });
      await examCache.invalidate(input.id);
      return result;
    }),

  // Committees
  getCommittees: protectedProcedure
    .input(z.object({ examId: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await examCache.getCommittees(input.examId);
      if (cached) return cached;

      const result = await getExamCommittees(db, { examId: input.examId });
      if (result) {
        await examCache.setCommittees(input.examId, result);
      }
      return result;
    }),

  createCommittee: protectedProcedure
    .input(createExamCommitteeSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createExamCommittee(db, input);
      await examCache.invalidate(input.examId);
      return result;
    }),

  updateCommittee: protectedProcedure
    .input(updateExamCommitteeSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      // We need examId to invalidate cache, but update doesn't take it.
      // We accept it might be stale until TTL or fetch original first.
      return updateExamCommittee(db, { id, ...data });
    }),

  deleteCommittee: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      return deleteExamCommittee(db, { id: input.id });
    }),

  // Seatings
  getSeatings: protectedProcedure
    .input(z.object({ committeeId: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getExamSeatings(db, { committeeId: input.committeeId });
    }),

  getStudentSeating: protectedProcedure
    .input(getStudentExamSeatingSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await examCache.getStudentSeating(input.studentId, input.examId);
      if (cached) return cached;

      const result = await getStudentExamSeating(db, input);
      if (result) {
        await examCache.setStudentSeating(input.studentId, input.examId, result);
      }
      return result;
    }),

  createSeating: protectedProcedure
    .input(createExamSeatingSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createExamSeating(db, input);
    }),

  bulkCreateSeatings: protectedProcedure
    .input(bulkCreateExamSeatingsSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return bulkCreateExamSeatings(db, input);
    }),

  deleteSeating: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      return deleteExamSeating(db, { id: input.id });
    }),
});
