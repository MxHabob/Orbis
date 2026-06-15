import { z } from "zod";

export const createAcademicYearSchema = z.object({
  yearName: z.string().min(1),
  startDate: z.string().min(1),
  endDate: z.string().min(1),
  isCurrent: z.boolean().optional(),
});

export const createAcademicTermSchema = z.object({
  name: z.string().min(1),
  academicYearId: z.number().optional(),
  startDate: z.string().min(1),
  endDate: z.string().min(1),
  isCurrent: z.boolean().optional(),
});
