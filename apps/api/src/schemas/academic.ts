import { z } from "@hono/zod-openapi";

export const academicYearSchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the academic year",
    example: 1,
  }),
  yearName: z.string().min(1).max(20).openapi({
    description: "Name of the academic year",
    example: "2025-2026",
  }),
  isCurrent: z.boolean().openapi({
    description: "Whether this is the current academic year",
    example: true,
  }),
});

export const createAcademicYearSchema = academicYearSchema.omit({ id: true });
export const updateAcademicYearSchema = academicYearSchema.partial().required({ id: true });

export const academicTermSchema = z.object({
  id: z.number().openapi({
    description: "Unique identifier of the academic term",
    example: 1,
  }),
  name: z.string().min(1).max(50).openapi({
    description: "Name of the term",
    example: "First Term",
  }),
  academicYearId: z.number().nullable().openapi({
    description: "ID of the associated academic year",
    example: 1,
  }),
  startDate: z.string().nullable().openapi({
    description: "Start date of the term",
    example: "2025-09-01",
  }),
  endDate: z.string().nullable().openapi({
    description: "End date of the term",
    example: "2026-01-15",
  }),
  isCurrent: z.boolean().nullable().openapi({
    description: "Whether this is the current term",
    example: false,
  }),
});

export const createAcademicTermSchema = academicTermSchema.omit({ id: true });
export const updateAcademicTermSchema = academicTermSchema.partial().required({ id: true });
