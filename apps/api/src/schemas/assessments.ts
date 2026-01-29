import { z } from "@hono/zod-openapi";

export const assessmentTypeEnum = z.enum([
  'exam', 'quiz', 'assignment', 'participation', 'oral', 'attendance', 'behavior', 'homework'
]);

export const assessmentItemSchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the assessment item",
    example: 1,
  }),
  title: z.string().min(1).max(255).openapi({
    description: "Title of the assessment",
    example: "Midterm Exam",
  }),
  type: assessmentTypeEnum.openapi({
    description: "Type of the assessment",
    example: "exam",
  }),
  maxScore: z.string().openapi({
    description: "Maximum possible score",
    example: "100.00",
  }),
  weight: z.string().nullable().openapi({
    description: "Weight of the assessment in final grade",
    example: "1.00",
  }),
  subjectId: z.number().nullable().openapi({
    description: "Subject ID",
    example: 1,
  }),
  classId: z.number().nullable().openapi({
    description: "Class ID",
    example: 1,
  }),
  month: z.number().int().min(1).max(12).openapi({
    description: "Month of the assessment",
    example: 10,
  }),
  termId: z.number().nullable().openapi({
    description: "Term ID",
    example: 1,
  }),
  isPublished: z.boolean().default(false).openapi({
    description: "Whether the assessment results are published",
    example: false,
  }),
});

export const createAssessmentItemSchema = assessmentItemSchema.omit({ id: true });
export const updateAssessmentItemSchema = assessmentItemSchema.partial().required({ id: true });
