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
  type: z.enum(['classroom','laboratory','library','computer_lab','music_room','exam_hall','workshop','multi-purpose_room']).default('classroom').openapi({
    description: "Type of the class room/space",
    example: "classroom",
  }),
  division: z.string().max(10).nullable().openapi({
    description: "Division or section of the class",
    example: "A",
  }),
  parentClassId: z.number().nullable().openapi({
    description: "ID of the parent class (for sub-sections)",
    example: 1,
  }),
  academicYearId: z.number().nullable().openapi({
    description: "ID of the academic year",
    example: 1,
  }),
  color: z.string().max(7).default("#e6e6e6").openapi({
    description: "Color for the class UI",
    example: "#e6e6e6",
  }),
});

export const createClassSchema = classSchema.omit({ id: true });
export const updateClassSchema = classSchema.partial().required({ id: true });
