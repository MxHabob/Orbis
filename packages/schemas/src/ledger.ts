import { z } from "zod";

export const ledgerAccountInputSchema = z.object({
  code: z.string(),
  name: z.string().trim().min(1),
  name_ar: z.string().trim().optional(),
  item_type: z.string(),
  sub_type: z.string().optional(),
  parent_id: z.string().optional(),
  currency: z.string(),
  opening_balance: z.number().finite().optional(),
});

export type LedgerAccountInput = z.infer<typeof ledgerAccountInputSchema>;
