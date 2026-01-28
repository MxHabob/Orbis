import {
  createSubjectSchema,
  updateSubjectSchema,
} from "@api/schemas/subjects";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createSubject,
  deleteSubject,
  getAllSubjects,
  getSubjectById,
  updateSubject,
} from "@midday/db/queries";
import { z } from "zod";

export const subjectsRouter = createTRPCRouter({
  getAll: protectedProcedure.query(async () => {
    return getAllSubjects();
  }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input }) => {
      return getSubjectById(input.id);
    }),

  create: protectedProcedure
    .input(createSubjectSchema)
    .mutation(async ({ input }) => {
      return createSubject(input);
    }),

  update: protectedProcedure
    .input(updateSubjectSchema)
    .mutation(async ({ input }) => {
      return updateSubject(input.id, input);
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteSubject(input.id);
    }),
});
