import { z } from "zod";

export function createLoginInputSchema(messages: {
  identifierRequired: string;
  passwordRequired: string;
}) {
  return z.object({
    identifier: z.string().trim().min(1, messages.identifierRequired),
    password: z.string().min(1, messages.passwordRequired),
  });
}

export const loginInputSchema = createLoginInputSchema({
  identifierRequired: "Account is required.",
  passwordRequired: "Password is required.",
});

export type LoginInput = z.infer<typeof loginInputSchema>;
