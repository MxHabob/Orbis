import { z } from "zod";

export const examSessionInputSchema = z
  .object({
    name: z.string().trim().optional(),
    name_ar: z.string().trim().min(1),
    session_type: z.enum(["final", "midterm", "quiz", "practical"]),
    start_date: z.string().date(),
    end_date: z.string().date(),
    period_id: z.string().trim().min(1),
  })
  .refine((value) => Boolean(value.name || value.name_ar), {
    path: ["name"],
    message: "Name is required",
  })
  .refine((value) => value.end_date >= value.start_date, {
    path: ["end_date"],
    message: "End date must be after start date",
  });

export const examSubjectInputSchema = z.object({
  session_id: z.string().trim().min(1),
  subject_id: z.string().trim().min(1),
  max_score: z.number().finite().positive(),
  duration_minutes: z.number().int().positive(),
  exam_type: z.enum(["written", "oral", "practical", "mixed"]),
});

export type ExamSessionInput = z.infer<typeof examSessionInputSchema>;
export type ExamSubjectInput = z.infer<typeof examSubjectInputSchema>;
