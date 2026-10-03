import { z } from "zod";

export const loginInputSchema = z.object({
  identifier: z.string().trim().min(1),
  password: z.string().min(1),
});

export type LoginInput = z.infer<typeof loginInputSchema>;
