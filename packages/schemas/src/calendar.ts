import { z } from "zod";

const optionalText = z.string().trim().optional();

export const createPeriodInputSchema = z
  .object({
    name: z.string().trim().min(1),
    name_ar: optionalText,
    code: optionalText,
    period_type: z.string().trim().min(1),
    parent_id: optionalText,
    start_date: z.string().date(),
    end_date: z.string().date(),
    grading_system: z.string().trim().min(1),
    passing_grade: z.number().finite().min(0),
    max_grade: z.number().finite().positive(),
  })
  .refine((value) => value.end_date >= value.start_date, {
    path: ["end_date"],
    message: "End date must be after start date",
  });

export type CreatePeriodInput = z.infer<typeof createPeriodInputSchema>;
