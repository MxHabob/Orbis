import { z } from "zod";

export const updateUserSchema = z.object({
  fullName: z.string().min(2).max(100).optional(),
  locale: z.string().optional(),
  timezone: z.string().optional(),
  avatarUrl: z.string().url().optional().nullable(),
  dateFormat: z.string().optional(),
  weekStartsOn: z.number().min(0).max(6).optional(),
});

export const updatePasswordSchema = z.object({
  currentPassword: z.string().min(1),
  newPassword: z.string().min(8),
});
