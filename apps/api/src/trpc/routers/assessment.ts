import {
  createAssessmentCategorySchema,
  createAssessmentItemSchema,
  getAssessmentCategoriesSchema,
  getAssessmentItemsSchema,
  updateAssessmentCategorySchema,
  updateAssessmentItemSchema,
} from "@api/schemas/assessment";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createAssessmentCategory,
  createAssessmentItem,
  deleteAssessmentCategory,
  deleteAssessmentItem,
  getAssessmentCategories,
  getAssessmentCategoryById,
  getAssessmentItemById,
  getAssessmentItems,
  updateAssessmentCategory,
  updateAssessmentItem,
} from "@midday/db/queries/assessment";
import { z } from "zod";

export const assessmentRouter = createTRPCRouter({
  // Categories
  getCategories: protectedProcedure
    .input(getAssessmentCategoriesSchema)
    .query(async ({ input, ctx: { db } }) => {
      // Could cache by academicYear/curriculumSubject
      return getAssessmentCategories(db, input);
    }),

  getCategoryById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getAssessmentCategoryById(db, { id: input.id });
    }),

  createCategory: protectedProcedure
    .input(createAssessmentCategorySchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createAssessmentCategory(db, input);
    }),

  updateCategory: protectedProcedure
    .input(updateAssessmentCategorySchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      return updateAssessmentCategory(db, { id, ...data });
    }),

  deleteCategory: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      return deleteAssessmentCategory(db, { id: input.id });
    }),

  // Items
  getItems: protectedProcedure
    .input(getAssessmentItemsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getAssessmentItems(db, input);
    }),

  getItemById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getAssessmentItemById(db, { id: input.id });
    }),

  createItem: protectedProcedure
    .input(createAssessmentItemSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createAssessmentItem(db, input);
    }),

  updateItem: protectedProcedure
    .input(updateAssessmentItemSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      return updateAssessmentItem(db, { id, ...data });
    }),

  deleteItem: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      return deleteAssessmentItem(db, { id: input.id });
    }),
});
