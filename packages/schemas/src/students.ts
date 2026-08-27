import { z } from "zod";

export const studentRegistrationSchema = z.object({
  name: z.string().trim().min(1).max(150),
  email: z.string().trim().email().nullable(),
  phone: z.string().trim().regex(/^\+?[0-9]{7,15}$/).nullable(),
  gender: z.string().trim().min(1).nullable(),
  birth_date: z.string().trim().min(1).nullable(),
  nationality: z.string().trim().nullable().optional(),
  national_id: z.string().trim().regex(/^[0-9]{7,15}$/).nullable(),
  address: z.string().trim().nullable().optional(),
  class_id: z.string().trim().min(1),
  guardian_name: z.string().trim().nullable(),
  guardian_phone: z.string().trim().regex(/^\+?[0-9]{7,15}$/).nullable(),
  guardian_relationship: z.string().trim().min(1).nullable(),
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
