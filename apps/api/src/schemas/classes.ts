import { z } from "@hono/zod-openapi";

export const classSchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the class",
    example: 1,
  }),
  name: z.string().min(1).max(100).openapi({
    description: "Name of the class",
    example: "Grade 10",
  }),
  division: z.string().max(10).nullable().openapi({
    description: "Division or section of the class",
    example: "A",
  }),
  academicYearId: z.number().nullable().openapi({
    description: "ID of the academic year",
    example: 1,
  }),
});

export const createClassSchema = classSchema.omit({ id: true });
export const updateClassSchema = classSchema.partial().required({ id: true });
