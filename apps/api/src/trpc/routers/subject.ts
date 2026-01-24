import {
  createCurriculumSubjectSchema,
  createSubjectSchema,
  getCurriculumSubjectsSchema,
  getSubjectsSchema,
  updateCurriculumSubjectSchema,
  updateSubjectSchema,
} from "@api/schemas/subject";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { subjectCache } from "@midday/cache/subject-cache";
import {
  createCurriculumSubject,
  createSubject,
  deleteCurriculumSubject,
  deleteSubject,
  getCurriculumSubjects,
  getSubjectById,
  getSubjects,
  getSubjectsByTeacher,
  getTeachersBySubject,
  updateCurriculumSubject,
  updateSubject,
} from "@midday/db/queries/subject";
import { z } from "zod";

export const subjectRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getSubjectsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getSubjects(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await subjectCache.getById(input.id);
      if (cached) return cached;

      const result = await getSubjectById(db, { id: input.id });
      if (result) {
        await subjectCache.setById(input.id, result);
      }
      return result;
    }),

  create: protectedProcedure
    .input(createSubjectSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createSubject(db, input);
    }),

  update: protectedProcedure
    .input(updateSubjectSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const result = await updateSubject(db, { id, ...data });
      await subjectCache.invalidate(id);
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await deleteSubject(db, { id: input.id });
      await subjectCache.invalidate(input.id);
      return result;
    }),

  // Curriculum
  getCurriculum: protectedProcedure
    .input(getCurriculumSubjectsSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await subjectCache.getCurriculumSubjects(input.academicYearId, input.levelId ?? undefined);
      if (cached) return cached;

      const result = await getCurriculumSubjects(db, input);
      if (result) {
        await subjectCache.setCurriculumSubjects(input.academicYearId, result, input.levelId ?? undefined);
      }
      return result;
    }),

  createCurriculum: protectedProcedure
    .input(createCurriculumSubjectSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      await subjectCache.invalidate(input.subjectId); // Invalidate because counts might change
      // Also invalidate curriculum list cache - tricky without keys, but assuming TTL handles it or we add specific invalidation
      return createCurriculumSubject(db, input);
    }),

  updateCurriculum: protectedProcedure
    .input(updateCurriculumSubjectSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      return updateCurriculumSubject(db, { id, ...data });
    }),

  deleteCurriculum: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      return deleteCurriculumSubject(db, { id: input.id });
    }),

  // Relations
  getByTeacher: protectedProcedure
    .input(z.object({ teacherId: z.string().uuid() }))
    .query(async ({ input, ctx: { db } }) => {
      return getSubjectsByTeacher(db, { teacherId: input.teacherId });
    }),

  getTeachers: protectedProcedure
    .input(z.object({ subjectId: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getTeachersBySubject(db, { subjectId: input.subjectId });
    }),
});
