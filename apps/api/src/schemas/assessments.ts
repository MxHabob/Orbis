import { z } from "@hono/zod-openapi";

export const assessmentTypeEnum = z.enum([
  "exam",
  "quiz",
  "assignment",
  "participation",
  "oral",
  "attendance",
  "behavior",
  "homework",
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

export const createAssessmentItemSchema = assessmentItemSchema.omit({
  id: true,
});
export const updateAssessmentItemSchema = assessmentItemSchema
  .partial()
  .required({ id: true });

export const gradeEntrySchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the grade entry",
    example: 1,
  }),
  studentId: z.string().uuid().openapi({
    description: "Student ID",
    example: "123e4567-e89b-12d3-a456-426614174000",
  }),
  assessmentItemId: z.number().openapi({
    description: "Assessment Item ID",
    example: 1,
  }),
  score: z.number().nullable().openapi({
    description: "Score obtained",
    example: 85,
  }),
  isAbsent: z.boolean().default(false).openapi({
    description: "Is absent",
    example: false,
  }),
  comments: z.string().nullable().optional().openapi({
    description: "Comments",
    example: "Excellent work",
  }),
});

export const createGradeEntrySchema = gradeEntrySchema.omit({ id: true });
export const updateGradeEntrySchema = gradeEntrySchema
  .partial()
  .required({ id: true });
