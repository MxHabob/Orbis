import { z } from "zod";

/**
 * Payment status enum
 */
export const paymentStatusSchema = z.enum(["pending", "paid", "overdue"]);

/**
 * Fee type enum
 */
export const feeTypeSchema = z.enum([
  "tuition",
  "registration",
  "transportation",
  "books",
  "other",
]);

/**
 * Schema for creating a payment
 */
export const createPaymentSchema = z.object({
  studentId: z.string().uuid(),
  feeType: feeTypeSchema,
  amount: z.coerce.number().positive(),
  dueDate: z.coerce.date(),
  paidDate: z.coerce.date().optional().nullable(),
  status: paymentStatusSchema,
  description: z.string().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for updating a payment
 */
export const updatePaymentSchema = z.object({
  feeType: feeTypeSchema.optional(),
  amount: z.coerce.number().positive().optional(),
  dueDate: z.coerce.date().optional(),
  paidDate: z.coerce.date().optional().nullable(),
  status: paymentStatusSchema.optional(),
  description: z.string().optional().nullable(),
});

/**
 * Schema for getting payments
 */
export const getPaymentsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  sort: z.array(z.string()).length(2).optional(),
  studentId: z.string().uuid().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  status: paymentStatusSchema.optional().nullable(),
  feeType: feeTypeSchema.optional().nullable(),
  dueDateFrom: z.coerce.date().optional().nullable(),
  dueDateTo: z.coerce.date().optional().nullable(),
});

/**
 * Schema for marking payment as paid
 */
export const markPaymentAsPaidSchema = z.object({
  paidDate: z.coerce.date().optional(),
});

/**
 * Schema for getting student payment summary
 */
export const getStudentPaymentSummarySchema = z.object({
  studentId: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for getting overdue payments
 */
export const getOverduePaymentsSchema = z.object({
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
});

/**
 * Schema for getting payment statistics by year
 */
export const getPaymentStatsByYearSchema = z.object({
  academicYearId: z.coerce.number().int().positive(),
});

/**
 * Schema for bulk creating payments
 */
export const bulkCreatePaymentsSchema = z.object({
  payments: z.array(
    z.object({
      studentId: z.string().uuid(),
      feeType: feeTypeSchema,
      amount: z.coerce.number().positive(),
      dueDate: z.coerce.date(),
      status: paymentStatusSchema,
      description: z.string().optional().nullable(),
      academicYearId: z.coerce.number().int().positive().optional().nullable(),
    }),
  ),
});
