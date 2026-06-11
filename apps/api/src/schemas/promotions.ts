import { z } from "zod";

export const promotionStatusSchema = z.enum([
  "promoted",
  "retained",
  "transferred",
  "graduated",
  "conditional",
]);

export const promotionDecisionSchema = z.enum(["auto", "manual", "override"]);

export const previewPromotionSchema = z.object({
  classId: z.number(),
  academicYearId: z.number(),
});

export const executePromotionSchema = z.object({
  classId: z.number(),
  fromAcademicYearId: z.number(),
  toAcademicYearId: z.number(),
  decisions: z.array(
    z.object({
      studentId: z.string().uuid(),
      status: z.enum(["promoted", "retained", "graduated", "conditional"]),
      toClassId: z.number().optional(),
      notes: z.string().optional(),
    }),
  ),
});

export const rollbackPromotionSchema = z.object({
  classId: z.number(),
  academicYearId: z.number(),
  studentIds: z.array(z.string().uuid()).optional(),
});

export const promotionHistorySchema = z.object({
  academicYearId: z.number().optional(),
  classId: z.number().optional(),
  studentId: z.string().uuid().optional(),
  status: promotionStatusSchema.optional(),
  limit: z.number().min(1).max(500).default(50),
  offset: z.number().min(0).default(0),
});

export const promotionStatisticsSchema = z.object({
  academicYearId: z.number(),
  classId: z.number().optional(),
});

export const overridePromotionSchema = z.object({
  studentId: z.string().uuid(),
  fromClassId: z.number(),
  toClassId: z.number(),
  fromAcademicYearId: z.number(),
  toAcademicYearId: z.number(),
  status: z.enum(["promoted", "retained", "graduated", "conditional"]),
  notes: z.string().optional(),
});
