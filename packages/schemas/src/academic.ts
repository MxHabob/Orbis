import { z } from "zod";

const localizedName = z.object({
  name: z.string().trim().optional(),
  name_ar: z.string().trim().optional(),
});

const hasName = (value: { name?: string; name_ar?: string }) =>
  Boolean(value.name || value.name_ar);

export const createSubjectInputSchema = localizedName
  .extend({
    code: z.string().trim().min(1),
    color: z.string().trim().nullable().optional(),
  })
  .refine(hasName, { path: ["name"], message: "Name is required" });

export const createRoomInputSchema = localizedName
  .extend({
    capacity: z.number().int().positive().nullable(),
    room_type: z.enum([
      "classroom",
      "laboratory",
      "lecture_hall",
      "office",
      "other",
    ]),
    building: z.string().trim().nullable(),
    floor: z.number().int().nullable(),
  })
  .refine(hasName, { path: ["name"], message: "Name is required" });

export const createAssessmentInputSchema = z
  .object({
    class_id: z.string().trim().min(1),
    subject_id: z.string().trim().min(1),
    term_id: z.string().trim().min(1),
    month: z.number().int().min(1).max(12).optional(),
    title: z.string().trim().optional(),
    title_ar: z.string().trim().optional(),
    max_score: z.number().positive(),
    weight: z.number().min(0).max(100),
    item_type: z.enum(["exam", "quiz", "homework", "classwork"]),
  })
  .refine((value) => Boolean(value.title || value.title_ar), {
    path: ["title"],
    message: "Title is required",
  });

const clockTime = z.string().regex(/^([01]\d|2[0-3]):[0-5]\d$/, "Invalid time");

export const createTimeSlotInputSchema = z
  .object({
    period_id: z.string().trim().min(1),
    slot_number: z.number().int().positive(),
    name: z.string().trim().min(1),
    name_ar: z.string().trim().min(1),
    start_time: clockTime,
    end_time: clockTime,
    duration_minutes: z.number().int().positive(),
    is_break: z.boolean(),
    applicable_days: z.string().trim().min(1),
  })
  .refine((value) => value.end_time > value.start_time, {
    path: ["end_time"],
    message: "End time must be after start time",
  });

export type CreateRoomInput = z.infer<typeof createRoomInputSchema>;
export type CreateSubjectInput = z.infer<typeof createSubjectInputSchema>;
export type CreateAssessmentInput = z.infer<typeof createAssessmentInputSchema>;
export type CreateTimeSlotInput = z.infer<typeof createTimeSlotInputSchema>;
