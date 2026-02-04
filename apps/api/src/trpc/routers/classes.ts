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
    .input(
      z
        .object({
          academicYearId: z.number().optional().nullable(),
          q: z.string().optional().nullable(),
          sort: z.array(z.string()).nullish(),
          cursor: z.number().optional().nullable(),
          limit: z.number().optional().default(50),
          parentId: z.string().optional().nullable(),
          ids: z.array(z.string()).optional().nullable(),
        })
        .optional(),
    )
    .query(async ({ input }) => {
      const limit = input?.limit ?? 50;
      const cursor = input?.cursor ?? 0;

      const data = await getClassesV3({
        ...input,
        limit: limit + 1,
        offset: cursor,
        flat: !!(input?.q || input?.sort), // Auto-flatten if searching/sorting
      });

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
