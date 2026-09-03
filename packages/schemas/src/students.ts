import { z } from "zod";

const emptyToNull = (val: unknown) =>
  typeof val === "string" && val.trim() === "" ? null : val;

export const studentRegistrationSchema = z.object({
  name: z.string().trim().min(1).max(150),
  name_en: z.preprocess(emptyToNull, z.string().trim().max(150).nullable().optional()),
  email: z.preprocess(emptyToNull, z.string().trim().email().nullable().optional()),
  password: z.string().min(12).max(128),
  phone: z.preprocess(
    emptyToNull,
    z.string().trim().regex(/^\+?[0-9]{7,15}$/).nullable().optional()
  ),
  gender: z.preprocess(emptyToNull, z.string().trim().min(1).nullable().optional()),
  birth_date: z.preprocess(emptyToNull, z.string().trim().min(1).nullable().optional()),
  nationality: z.preprocess(emptyToNull, z.string().trim().nullable().optional()),
  national_id: z.preprocess(
    emptyToNull,
    z.string().trim().regex(/^[0-9]{7,15}$/).nullable().optional()
  ),
  address: z.preprocess(emptyToNull, z.string().trim().nullable().optional()),
  class_id: z.string().trim().min(1),
  guardian_name: z.preprocess(emptyToNull, z.string().trim().nullable().optional()),
  guardian_phone: z.preprocess(
    emptyToNull,
    z.string().trim().regex(/^\+?[0-9]{7,15}$/).nullable().optional()
  ),
  guardian_relationship: z.preprocess(
    emptyToNull,
    z.string().trim().min(1).nullable().optional()
  ),
});

export type StudentRegistrationInput = z.infer<typeof studentRegistrationSchema>;

export const guardianFormInputSchema = z.object({
  fullName: z.string().trim().min(1),
  phone: z.string().trim().regex(/^\+?[0-9]{7,15}$/),
  email: z.string().trim().email().or(z.literal("")),
  nationalId: z.string().trim().regex(/^[0-9]{7,15}$/).or(z.literal("")),
  relationship: z.string().min(1),
  paymentSharePercent: z.number().finite().min(0).max(100),
});

export type GuardianFormInput = z.infer<typeof guardianFormInputSchema>;
