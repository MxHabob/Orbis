import {
  bulkCreatePaymentsSchema,
  createPaymentSchema,
  getOverduePaymentsSchema,
  getPaymentStatsByYearSchema,
  getPaymentsSchema,
  getStudentPaymentSummarySchema,
  markPaymentAsPaidSchema,
  updatePaymentSchema,
} from "@api/schemas/payment";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { paymentCache } from "@midday/cache/payment-cache";
import {
  bulkCreatePayments,
  createPayment,
  deletePayment,
  getOverduePayments,
  getPaymentById,
  getPaymentStatsByYear,
  getPayments,
  getStudentPaymentSummary,
  markPaymentAsPaid,
  updatePayment,
} from "@midday/db/queries/payment";
import { z } from "zod";

export const paymentRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getPaymentsSchema)
    .query(async ({ input, ctx: { db } }) => {
      // Could verify studentId against cache if needed
      return getPayments(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getPaymentById(db, { id: input.id });
    }),

  getStudentSummary: protectedProcedure
    .input(getStudentPaymentSummarySchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await paymentCache.getStudentSummary(input.studentId, input.academicYearId);
      if (cached) return cached;

      const result = await getStudentPaymentSummary(db, input);
      if (result) {
        await paymentCache.setStudentSummary(input.studentId, result, input.academicYearId);
      }
      return result;
    }),

  getOverdue: protectedProcedure
    .input(getOverduePaymentsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getOverduePayments(db, input);
    }),

  getStatsByYear: protectedProcedure
    .input(getPaymentStatsByYearSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getPaymentStatsByYear(db, input);
    }),

  create: protectedProcedure
    .input(createPaymentSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createPayment(db, input);
      await paymentCache.invalidateStudent(input.studentId);
      return result;
    }),

  update: protectedProcedure
    .input(updatePaymentSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const original = await getPaymentById(db, { id });
      
      const result = await updatePayment(db, { id, ...data });
      
      if (original) {
        await paymentCache.invalidateStudent(original.studentId);
      }
      return result;
    }),

  markAsPaid: protectedProcedure
    .input(markPaymentAsPaidSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const original = await getPaymentById(db, { id });
      
      const result = await markPaymentAsPaid(db, { id, ...data });
      
      if (original) {
        await paymentCache.invalidateStudent(original.studentId);
      }
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const original = await getPaymentById(db, { id: input.id });
      const result = await deletePayment(db, { id: input.id });
      
      if (original) {
        await paymentCache.invalidateStudent(original.studentId);
      }
      return result;
    }),

  bulkCreate: protectedProcedure
    .input(bulkCreatePaymentsSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await bulkCreatePayments(db, input);
      
      const studentIds = new Set(input.payments.map(p => p.studentId));
      await Promise.all(Array.from(studentIds).map(id => paymentCache.invalidateStudent(id)));
      
      return result;
    }),
});
