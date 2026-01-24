import { z } from "zod";

/**
 * Stage enum
 */
export const stageSchema = z.enum(["basic", "secondary"]);

/**
 * Schema for creating a level
 */
export const createLevelSchema = z.object({
  name: z.string().min(1).max(100),
  stage: stageSchema,
  order: z.coerce.number().int().positive(),
});

/**
 * Schema for updating a level
 */
export const updateLevelSchema = z.object({
  name: z.string().min(1).max(100).optional(),
  stage: stageSchema.optional(),
  order: z.coerce.number().int().positive().optional(),
});

/**
 * Schema for getting levels
 */
export const getLevelsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  sort: z.array(z.string()).length(2).optional(),
  stage: stageSchema.optional().nullable(),
});

/**
 * Schema for getting levels by stage
 */
export const getLevelsByStageSchema = z.object({
  stage: stageSchema,
});
