import {
  executePromotionSchema,
  overridePromotionSchema,
  previewPromotionSchema,
  promotionHistorySchema,
  promotionStatisticsSchema,
  rollbackPromotionSchema,
} from "@api/schemas/promotions";
import { adminProcedure, createTRPCRouter } from "@api/trpc/init";
import {
  executeBulkPromotion,
  getPromotionHistory,
  getPromotionStatistics,
  overrideStudentPromotion,
  previewClassPromotion,
  rollbackPromotion,
} from "@orbis/db/queries";

export const promotionsRouter = createTRPCRouter({
  /**
   * معاينة الترفيع - عرض قائمة الطلاب مع نتائجهم ومقترح الترفيع
   * Preview promotion results for a class before executing
   */
  preview: adminProcedure
    .input(previewPromotionSchema)
    .query(async ({ input }) => {
      return previewClassPromotion({
        classId: input.classId,
        academicYearId: input.academicYearId,
      });
    }),

  /**
   * تنفيذ الترفيع الجماعي
   * Execute bulk promotion for a class of students
   */
  execute: adminProcedure
    .input(executePromotionSchema)
    .mutation(async ({ input, ctx }) => {
      return executeBulkPromotion({
        classId: input.classId,
        fromAcademicYearId: input.fromAcademicYearId,
        toAcademicYearId: input.toAcademicYearId,
        decisions: input.decisions,
        decisionById: ctx.session.user.id,
      });
    }),

  /**
   * التراجع عن الترفيع
   * Rollback a previous promotion operation
   */
  rollback: adminProcedure
    .input(rollbackPromotionSchema)
    .mutation(async ({ input, ctx }) => {
      return rollbackPromotion({
        classId: input.classId,
        academicYearId: input.academicYearId,
        studentIds: input.studentIds,
        rollbackById: ctx.session.user.id,
      });
    }),

  /**
   * سجل الترفيعات
   * Get promotion history with filters
   */
  historyRecords: adminProcedure
    .input(promotionHistorySchema)
    .query(async ({ input }) => {
      return getPromotionHistory({
        academicYearId: input.academicYearId,
        classId: input.classId,
        studentId: input.studentId,
        status: input.status,
        limit: input.limit,
        offset: input.offset,
      });
    }),

  /**
   * إحصائيات الترفيع
   * Get promotion statistics for a year/class
   */
  statistics: adminProcedure
    .input(promotionStatisticsSchema)
    .query(async ({ input }) => {
      return getPromotionStatistics({
        academicYearId: input.academicYearId,
        classId: input.classId,
      });
    }),

  /**
   * تجاوز يدوي - ترفيع أو تأخير طالب بقرار إداري
   * Override promotion decision for a single student
   */
  override: adminProcedure
    .input(overridePromotionSchema)
    .mutation(async ({ input, ctx }) => {
      return overrideStudentPromotion({
        studentId: input.studentId,
        fromClassId: input.fromClassId,
        toClassId: input.toClassId,
        fromAcademicYearId: input.fromAcademicYearId,
        toAcademicYearId: input.toAcademicYearId,
        status: input.status,
        decisionById: ctx.session.user.id,
        notes: input.notes,
      });
    }),
});
