import {
  createClassSchema,
  updateClassSchema,
} from "@api/schemas/classes";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createClass,
  deleteClass,
  getClassById,
  getClassesV3,
  updateClass,
} from "@midday/db/queries";
import { z } from "zod";

export const classesRouter = createTRPCRouter({
  getAll: protectedProcedure
    .input(z.object({ academicYearId: z.number().optional() }).optional())
    .query(async ({ input }) => {
      return getClassesV3(input?.academicYearId);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input }) => {
      return getClassById(input.id);
    }),

  create: protectedProcedure
    .input(createClassSchema)
    .mutation(async ({ input }) => {
      return createClass(input);
    }),

  update: protectedProcedure
    .input(updateClassSchema)
    .mutation(async ({ input }) => {
      return updateClass(input.id, input);
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteClass(input.id);
    }),
});
