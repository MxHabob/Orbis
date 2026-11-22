import { z } from "zod";

const localizedName = z.object({
  name: z.string().trim().optional(),
  name_ar: z.string().trim().optional(),
});

const hasName = (value: { name?: string; name_ar?: string }) =>
  Boolean(value.name || value.name_ar);

export const createSubjectInputSchema = localizedName
  .extend({
    name: z.string().trim().default(""),
    name_ar: z.string().trim().default(""),
    code: z.string().trim().min(1),
    color: z.string().trim().nullable().optional(),
    description: z.string().trim().default(""),
    credit_hours: z.number().int().positive().max(20).default(3),
    sort_order: z.number().int().positive().max(1000).default(1),
    is_core: z.boolean().default(true),
    is_elective: z.boolean().default(false),
    is_lab_required: z.boolean().default(false),
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
    is_exam_hall: z.boolean().default(false),
    floor: z.number().int().nullable(),
    org_unit_id: z.string().trim(),
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

export const assessmentSchemeInputSchema = z
  .object({
    name: z.string().trim().default(""),
    name_ar: z.string().trim().default(""),
    description: z.string().trim().default(""),
    max_score: z.number().positive().max(1000).default(100),
    level: z.enum([
      "monthly_period",
      "term",
      "annual",
      "coursework",
      "midterm",
      "final",
      "module_assessment",
      "certificate",
      "certificate_cumulative",
    ]),
    institution_scope: z.enum(["school", "university", "institute", "all"]),
    is_default: z.boolean().default(false),
  })
  .refine((value) => Boolean(value.name || value.name_ar), {
    path: ["name"],
    message: "Name is required",
  });

export const assessmentComponentInputSchema = z
  .object({
    code: z.string().trim().default(""),
    name: z.string().trim().default(""),
    name_ar: z.string().trim().default(""),
    component_type: z.string().trim().min(1),
    weight: z.number().min(0).max(100),
    max_score: z.number().positive().max(1000),
    passing_score: z.number().min(0).max(1000),
    sort_order: z.number().int().positive(),
    is_required: z.boolean().default(true),
  })
  .refine((value) => Boolean(value.name || value.name_ar), {
    path: ["name"],
    message: "Name is required",
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

export const classFormInputSchema = z
  .object({
    unit_id: z.string().trim().default(""),
    name: z.string().trim().default(""),
    name_ar: z.string().trim().default(""),
    code: z.string().trim().max(50).default(""),
    grade_level: z.string().trim().max(50).default(""),
    track: z.string().trim().max(50).default(""),
    capacity: z.number().int().min(1).max(100),
    color: z.string().trim().default(""),
  })
  .refine((value) => Boolean(value.name || value.name_ar), {
    path: ["name"],
    message: "Name is required",
  });

export const classSubjectFormInputSchema = z.object({
  class_id: z.string().trim().min(1),
  subject_id: z.string().trim().min(1),
  teacher_id: z.string().trim().min(1),
  period_id: z.string().trim().min(1),
  weekly_hours: z.number().int().min(1).max(30),
});

export const lessonFormInputSchema = z.object({
  period_id: z.string().trim().min(1),
  class_id: z.string().trim().min(1),
  subject_id: z.string().trim().min(1),
  teacher_id: z.string().trim().min(1),
  time_slot_id: z.string().trim().min(1),
  day_of_week: z.number().int().min(1).max(7),
});

export type CreateRoomInput = z.infer<typeof createRoomInputSchema>;
export type CreateSubjectInput = z.infer<typeof createSubjectInputSchema>;
export type CreateAssessmentInput = z.infer<typeof createAssessmentInputSchema>;
export type AssessmentSchemeInput = z.infer<typeof assessmentSchemeInputSchema>;
export type AssessmentComponentInput = z.infer<
  typeof assessmentComponentInputSchema
>;
export type CreateTimeSlotInput = z.infer<typeof createTimeSlotInputSchema>;
export type ClassFormInput = z.infer<typeof classFormInputSchema>;
export type ClassSubjectFormInput = z.infer<typeof classSubjectFormInputSchema>;
export type LessonFormInput = z.infer<typeof lessonFormInputSchema>;
