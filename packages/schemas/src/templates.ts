import { z } from "zod";

export const buildingInputSchema = z.object({
  name: z.string().trim().min(1).max(100),
});

export const templateInputSchema = z
  .object({
    name: z.string().trim().optional(),
    name_ar: z.string().trim().optional(),
    description: z.string().trim().max(500).optional(),
    entity_type: z.string().trim().min(1),
    document_type: z.string().trim().min(1),
    preset: z.string().trim().min(1),
  })
  .refine((value) => Boolean(value.name || value.name_ar), {
    path: ["name"],
    message: "Template name is required",
  });

export type BuildingInput = z.infer<typeof buildingInputSchema>;
export type TemplateInput = z.infer<typeof templateInputSchema>;
