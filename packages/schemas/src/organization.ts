import { z } from "zod";

export const orgUnitInputSchema = z.object({
  name: z.string().trim().min(1),
  name_ar: z.string().trim().optional(),
  code: z.string().trim().optional(),
  unit_type: z.string().trim().min(1),
  parent_id: z.string().trim().optional(),
  head_id: z.string().trim().optional(),
  capacity: z.number().int().nonnegative().nullable().optional(),
  room_number: z.string().trim().optional(),
  floor: z.number().int().nonnegative().nullable().optional(),
  building: z.string().trim().optional(),
  data_isolation: z.string().trim().min(1),
});

export type OrgUnitInput = z.infer<typeof orgUnitInputSchema>;
