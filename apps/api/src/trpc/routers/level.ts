import {
  createLevelSchema,
  getLevelsByStageSchema,
  getLevelsSchema,
  updateLevelSchema,
} from "@api/schemas/level";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { levelCache } from "@midday/cache/level-cache";
import {
  createLevel,
  deleteLevel,
  getLevelById,
  getLevels,
  getLevelsByStage,
  updateLevel,
} from "@midday/db/queries/level";
import { z } from "zod";

export const levelRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getLevelsSchema)
    .query(async ({ input, ctx: { db } }) => {
      if (!input.cursor && !input.stage && !input.sort) {
        const cached = await levelCache.getAll();
        if (cached) return cached;
      }
      return getLevels(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await levelCache.getById(input.id);
      if (cached) return cached;

      const result = await getLevelById(db, { id: input.id });
      if (result) {
        await levelCache.setById(input.id, result);
      }
      return result;
    }),

  getByStage: protectedProcedure
    .input(getLevelsByStageSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await levelCache.getByStage(input.stage);
      if (cached) return cached;

      const result = await getLevelsByStage(db, input);
      if (result) {
        await levelCache.setByStage(input.stage, result);
      }
      return result;
    }),

  create: protectedProcedure
    .input(createLevelSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createLevel(db, input);
      await levelCache.invalidate();
      return result;
    }),

  update: protectedProcedure
    .input(updateLevelSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const result = await updateLevel(db, { id, ...data });
      await levelCache.invalidate(id);
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await deleteLevel(db, { id: input.id });
      await levelCache.invalidate(input.id);
      return result;
    }),
});
