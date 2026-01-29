import { z } from "@hono/zod-openapi";

export const gradeEntrySchema = z.object({
  id: z.number().openapi({ example: 1 }),
  studentId: z.string().uuid().openapi({ example: "550e8400-e29b-41d4-a716-446655440000" }),
  assessmentItemId: z.number().openapi({ example: 1 }),
  score: z.string().nullable().openapi({ example: "85.50" }),
  isAbsent: z.boolean().default(false).openapi({ example: false }),
  comments: z.string().nullable().openapi({ example: "Excellent work" }),
  metadata: z.record(z.string(), z.any()).nullable().openapi({ example: {} }),
  gradedById: z.string().uuid().nullable().openapi({ example: "550e8400-e29b-41d4-a716-446655440001" }),
});

export const createGradeEntrySchema = gradeEntrySchema.omit({ id: true });
export const updateGradeEntrySchema = gradeEntrySchema.partial().required({ id: true });

export const bulkImportGradesSchema = z.object({
    assessmentItemId: z.number(),
    gradeData: z.array(z.object({
        studentNumber: z.string(),
        score: z.number().nullable(),
        isAbsent: z.boolean().optional(),
        comments: z.string().optional(),
    })),
});
