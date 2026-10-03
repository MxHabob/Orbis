import { z } from "zod";

const optionalText = z.preprocess(
  (value) => (typeof value === "string" && value.trim() === "" ? null : value),
  z.string().trim().nullable().optional(),
);

export const userCreateInputSchema = z.object({
  full_name: z.string().trim().min(1).max(150),
  name_ar: optionalText,
  email: z.string().trim().email(),
  password: z.string().min(8).max(128),
  primary_role: z.enum(["admin", "teacher", "parent", "student", "staff"]),
  sex: z.enum(["male", "female"]).nullable().optional(),
  phone: z.preprocess(
    (value) =>
      typeof value === "string" && value.trim() === "" ? null : value,
    z
      .string()
      .trim()
      .regex(/^\+?[0-9]{7,15}$/)
      .nullable()
      .optional(),
  ),
  national_id: z.preprocess(
    (value) =>
      typeof value === "string" && value.trim() === "" ? null : value,
    z
      .string()
      .trim()
      .regex(/^[0-9]{7,15}$/)
      .nullable()
      .optional(),
  ),
  date_of_birth: optionalText,
  address: optionalText,
});

export type UserCreateInput = z.infer<typeof userCreateInputSchema>;
