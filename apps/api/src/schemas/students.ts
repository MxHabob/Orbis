import { z } from "@hono/zod-openapi";

export const studentStatusEnum = z.enum(['active', 'graduated', 'transferred', 'suspended']);

export const studentSchema = z.object({
  userId: z.string().uuid().openapi({
    description: "User ID of the student",
    example: "550e8400-e29b-41d4-a716-446655440000",
  }),
  studentNumber: z.string().max(20).openapi({
    description: "Official student number",
    example: "ST2025001",
  }),
  secretNumber: z.string().max(20).nullable().openapi({
    description: "Secret number for anonymous grading",
    example: "SEC123",
  }),
  currentClassId: z.number().nullable().openapi({
    description: "ID of the current class",
    example: 1,
  }),
  status: studentStatusEnum.openapi({
    description: "Status of the student",
    example: "active",
  }),
  stats: z.record(z.string(), z.any()).nullable().openapi({
    description: "Additional statistics or metadata",
  }),
});

export const createStudentSchema = studentSchema;
export const updateStudentSchema = studentSchema.partial().required({ userId: true });

export const upsertStudentSchema = z.object({
  userId: z.string().uuid().optional(),
  name: z.string().min(2),
  email: z.string().email().optional().nullable(),
  phone: z.string().optional().nullable(),
  sex: z.enum(['male', 'female', 'unspecified']).default('unspecified'),
  studentNumber: z.string().max(20).optional().nullable(),
  secretNumber: z.string().max(20).optional().nullable(),
  currentClassId: z.number().optional().nullable(),
  status: studentStatusEnum.default('active'),
  stats: z.record(z.string(), z.any()).optional().nullable(),
});

export const importStudentsSchema = z.object({
  classId: z.number(),
  students: z.array(z.object({
    studentNumber: z.string().optional(),
    secretNumber: z.string().optional(),
    name: z.string(),
    phone: z.string().optional(),
    email: z.string().optional(),
    sex: z.enum(['male', 'female', 'unspecified']).default('unspecified'),
  })),
});


