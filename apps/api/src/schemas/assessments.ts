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
  maxScore: z.number().openapi({
    description: "Maximum possible score",
    example: 100,
  }),
  weight: z.number().nullable().openapi({
    description: "Weight of the assessment in final grade",
    example: 1.0,
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
});

export const createAssessmentItemSchema = assessmentItemSchema.omit({ id: true });
export const updateAssessmentItemSchema = assessmentItemSchema.partial().required({ id: true });

export const gradeEntrySchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the grade entry",
    example: 1,
  }),
  studentId: z.string().uuid().nullable().openapi({
    description: "Student User ID",
    example: "550e8400-e29b-41d4-a716-446655440000",
  }),
  assessmentItemId: z.number().nullable().openapi({
    description: "Assessment Item ID",
    example: 1,
  }),
  score: z.number().nullable().openapi({
    description: "Score achieved",
    example: 85.5,
  }),
  isAbsent: z.boolean().nullable().openapi({
    description: "Whether the student was absent",
    example: false,
  }),
  comments: z.string().nullable().openapi({
    description: "Comments on the grade",
    example: "Good work",
  }),
  metadata: z.record(z.string(), z.any()).nullable().openapi({
    description: "Additional metadata",
  }),
  gradedById: z.string().uuid().nullable().openapi({
    description: "User ID of the grader",
  }),
  updatedAt: z.coerce.date().openapi({
    description: "Last update timestamp",
  }),
});

export const createGradeEntrySchema = gradeEntrySchema.omit({ id: true, updatedAt: true });
export const updateGradeEntrySchema = gradeEntrySchema.partial().required({ id: true });

export const importGradesSchema = z.object({
  assessmentItemId: z.number(),
  grades: z.array(z.object({
    studentNumber: z.string(),
    score: z.number().nullable(),
    isAbsent: z.boolean().default(false),
    comments: z.string().optional(),
  })),
});

