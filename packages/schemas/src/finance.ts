import { z } from "zod";

export const feeTemplateInputSchema = z.object({
  name: z.string().trim().min(3).max(100),
  name_ar: z.string().trim().nullable().optional(),
  code: z.string().trim().nullable().optional(),
  category: z.enum(["tuition", "transport", "books", "uniform", "exam", "other"]),
  default_amount: z.number().finite().positive(),
  revenue_account_id: z.string().trim().nullable().optional(),
  receivable_account_id: z.string().trim().nullable().optional(),
  applicable_levels: z.string().nullable().optional(),
  frequency: z.enum(["once", "monthly", "termly", "yearly"]),
  is_mandatory: z.boolean(),
});

export const feeAssignmentInputSchema = z.object({
  enrollment_id: z.string().trim().min(1),
  fee_template_id: z.string().trim().min(1),
  amount_override: z.number().finite().positive().nullable().optional(),
});

export type FeeAssignmentInput = z.infer<typeof feeAssignmentInputSchema>;
export type FeeTemplateInput = z.infer<typeof feeTemplateInputSchema>;
