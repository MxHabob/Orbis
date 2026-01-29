import { z } from "@hono/zod-openapi";

export const subjectSchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the subject",
    example: 1,
  }),
  name: z.string().min(1).max(255).openapi({
    description: "Name of the subject",
    example: "Mathematics",
  }),
  code: z.string().max(50).nullable().openapi({
    description: "Code of the subject",
    example: "MATH101",
  }),
  color: z.string().max(7).openapi({
    description: "Color for the subject UI",
    example: "#FFF3E0",
  }),
});

export const createSubjectSchema = subjectSchema.omit({ id: true });
export const updateSubjectSchema = subjectSchema.partial().required({ id: true });
