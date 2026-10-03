import { z } from "zod";

export const feeTemplateInputSchema = z.object({
  name: z.string().trim().min(3).max(100),
  name_ar: z.string().trim().nullable().optional(),
  code: z.string().trim().nullable().optional(),
  category: z.enum([
    "tuition",
    "transport",
    "books",
    "uniform",
    "exam",
    "other",
  ]),
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

export const discountTemplateInputSchema = z.object({
  name: z.string().trim().min(1),
  code: z.string().trim().nullable().optional(),
  item_type: z.enum(["manual", "scholarship", "system"]),
  calculation_method: z.enum(["percentage", "fixed"]),
  value: z.number().finite().min(0),
  priority: z.number().int().nonnegative().nullable().optional(),
  requires_approval: z.number().int().min(0).max(1),
  is_auto_apply: z.number().int().min(0).max(1),
  is_stackable: z.number().int().min(0).max(1),
  is_active: z.number().int().min(0).max(1),
});

export const paymentRecordInputSchema = z.object({
  fee_id: z.string().trim().min(1),
  amount: z.number().finite().positive(),
  method: z.enum([
    "cash",
    "bank_transfer",
    "check",
    "card",
    "credit_card",
    "mada",
    "apple_pay",
    "other",
  ]),
  reference: z.string().trim().optional(),
  notes: z.string().trim().optional(),
  cash_account_id: z.string().trim().min(1),
  currency: z
    .string()
    .trim()
    .regex(/^[A-Za-z]{3}$/),
  exchange_rate: z.number().finite().positive().optional(),
});

export const currencyInputSchema = z.object({
  code: z.string().trim().toUpperCase().min(2).max(3),
  name: z.string().trim().min(1),
  name_ar: z.string().trim().nullable().optional(),
  symbol: z.string().trim().min(1),
  symbol_position: z.enum(["before", "after"]),
  decimal_places: z.number().int().min(0).max(4),
  exchange_rate: z.number().finite().positive(),
  is_base: z.boolean(),
});

const accountName = z
  .object({
    name: z.string().trim().optional(),
    name_ar: z.string().trim().optional(),
  })
  .refine((value) => Boolean(value.name || value.name_ar), {
    path: ["name"],
    message: "Name is required",
  });

export const generalLedgerAccountInputSchema = accountName.extend({
  code: z.string().optional(),
  parent_id: z.string().trim().min(1),
  item_type: z.string().trim().min(1),
  currency: z.string(),
  sub_type: z.string().optional(),
});

export const cashAccountInputSchema = accountName.extend({
  code: z.string().optional(),
  item_type: z.literal("asset"),
  currency: z.string().trim().min(1),
  sub_type: z.enum(["cash", "bank"]),
});

export const entityAccountInputSchema = accountName.extend({
  code: z.string().optional(),
  item_type: z.string(),
  currency: z.string(),
  sub_type: z.enum(["vendor", "partner", "employee", "student"]),
});

export type FeeAssignmentInput = z.infer<typeof feeAssignmentInputSchema>;
export type FeeTemplateInput = z.infer<typeof feeTemplateInputSchema>;
export type DiscountTemplateInput = z.infer<typeof discountTemplateInputSchema>;
export type PaymentRecordInput = z.infer<typeof paymentRecordInputSchema>;
export type CurrencyInput = z.infer<typeof currencyInputSchema>;
export type GeneralLedgerAccountInput = z.infer<
  typeof generalLedgerAccountInputSchema
>;
export type CashAccountInput = z.infer<typeof cashAccountInputSchema>;
export type EntityAccountInput = z.infer<typeof entityAccountInputSchema>;
