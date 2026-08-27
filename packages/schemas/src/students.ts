import { z } from "zod";

export const guardianFormInputSchema = z.object({
  fullName: z.string().trim().min(1),
  phone: z.string().trim().regex(/^\+?[0-9]{7,15}$/),
  email: z.string().trim().email().or(z.literal("")),
  nationalId: z.string().trim().regex(/^[0-9]{7,15}$/).or(z.literal("")),
  relationship: z.string().min(1),
  paymentSharePercent: z.number().finite().min(0).max(100),
});

export type GuardianFormInput = z.infer<typeof guardianFormInputSchema>;
