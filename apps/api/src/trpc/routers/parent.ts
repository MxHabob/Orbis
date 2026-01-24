import {
  createParentSchema,
  getParentByPhoneSchema,
  getParentsSchema,
  updateParentSchema,
} from "@api/schemas/parent";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { parentCache } from "@midday/cache/parent-cache";
import {
  createParent,
  deleteParent,
  getParentById,
  getParentByPhone,
  getParentChildren,
  getParents,
  updateParent,
} from "@midday/db/queries/parent";
import { z } from "zod";

export const parentRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getParentsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getParents(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await parentCache.getById(input.userId);
      if (cached) return cached;

      const result = await getParentById(db, { userId: input.userId });
      if (result) {
        await parentCache.setById(input.userId, result);
      }
      return result;
    }),

  getByPhone: protectedProcedure
    .input(getParentByPhoneSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await parentCache.getByPhone(input.phone);
      if (cached) return cached;

      const result = await getParentByPhone(db, { phone: input.phone });
      if (result) {
        await parentCache.setByPhone(input.phone, result);
      }
      return result;
    }),

  getChildren: protectedProcedure
    .input(z.object({ parentId: z.string().uuid() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await parentCache.getChildren(input.parentId);
      if (cached) return cached;

      const result = await getParentChildren(db, { parentId: input.parentId });
      if (result) {
        await parentCache.setChildren(input.parentId, result);
      }
      return result;
    }),

  create: protectedProcedure
    .input(createParentSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      // Check if user exists by phone first?
      const existing = await getParentByPhone(db, { phone: input.phone });
      if (existing) {
        throw new Error("Parent with this phone already exists");
      }
      return createParent(db, input);
    }),

  update: protectedProcedure
    .input(updateParentSchema.extend({ userId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { userId, ...data } = input;
      const result = await updateParent(db, { userId, ...data });
      await parentCache.invalidate(userId);
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await deleteParent(db, { userId: input.userId });
      await parentCache.invalidate(input.userId);
      return result;
    }),
});
