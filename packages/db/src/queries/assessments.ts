import { db } from "@db/client";
import { academicTerms, assessmentItems, classes, subjects } from "@db/schema";
import { RedisCache } from "@orbis/cache";
import { and, eq, inArray } from "drizzle-orm";

const assessmentCache = new RedisCache("assessments", 60 * 60);

export type CreateAssessmentParams = typeof assessmentItems.$inferInsert;

export const createAssessment = async (data: CreateAssessmentParams) => {
  const [result] = await db.insert(assessmentItems).values(data).returning();

  // Invalidate cache for this class/subject
  if (result) {
    await assessmentCache.deletePattern(`class:${result.classId}*`);
  }
  return result;
};

export const getAssessmentsForClass = async (
  classId: number,
  subjectId?: number,
  termId?: number,
  month?: number,
) => {
  // Simple caching strategy for list
  const cacheKey = `class:${classId}:${subjectId || "all"}:${termId || "all"}:${month || "all"}`;
  const cached = await assessmentCache.get<any[]>(cacheKey);

  if (cached) return cached;

  const conditions = [eq(assessmentItems.classId, classId)];

  if (subjectId) {
    conditions.push(eq(assessmentItems.subjectId, subjectId));
  }
  if (termId) {
    conditions.push(eq(assessmentItems.termId, termId));
  }
  if (month) {
    conditions.push(eq(assessmentItems.month, month));
  }

  const results = await db
    .select({
      id: assessmentItems.id,
      title: assessmentItems.title,
      type: assessmentItems.type,
      maxScore: assessmentItems.maxScore,
      weight: assessmentItems.weight,
      month: assessmentItems.month,
      subject: {
        name: subjects.name,
        code: subjects.code,
      },
      term: {
        name: academicTerms.name,
      },
    })
    .from(assessmentItems)
    .leftJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .leftJoin(academicTerms, eq(assessmentItems.termId, academicTerms.id))
    .where(and(...conditions));

  if (results) {
    await assessmentCache.set(cacheKey, results);
  }
  return results;
};

export const updateAssessment = async (
  id: number,
  data: Partial<CreateAssessmentParams>,
) => {
  const [result] = await db
    .update(assessmentItems)
    .set(data)
    .where(eq(assessmentItems.id, id))
    .returning();
  if (result) await assessmentCache.deletePattern(`class:${result.classId}*`);
  return result;
};

export const deleteAssessment = async (id: number) => {
  const [existing] = await db
    .select()
    .from(assessmentItems)
    .where(eq(assessmentItems.id, id));
  await db.delete(assessmentItems).where(eq(assessmentItems.id, id));
  if (existing)
    await assessmentCache.deletePattern(`class:${existing.classId}*`);
};

export const getMonthlyAssessmentItems = async (params: {
  classId: number;
  subjectId: number;
  termId: number;
  month: number | number[];
}) => {
  const monthCondition = Array.isArray(params.month)
    ? inArray(assessmentItems.month, params.month)
    : eq(assessmentItems.month, params.month);

  return db
    .select()
    .from(assessmentItems)
    .where(
      and(
        eq(assessmentItems.classId, params.classId),
        eq(assessmentItems.subjectId, params.subjectId),
        eq(assessmentItems.termId, params.termId),
        monthCondition,
      ),
    );
};

export const initializeStandardMonthlyAssessments = async (params: {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
}) => {
  const itemsToCreate = [
    {
      title: "المواظبة",
      type: "attendance" as const,
      maxScore: "10",
      weight: "1.0",
      classId: params.classId,
      subjectId: params.subjectId,
      termId: params.termId,
      month: params.month,
      isPublished: true,
    },
    {
      title: "السلوك",
      type: "behavior" as const,
      maxScore: "10",
      weight: "1.0",
      classId: params.classId,
      subjectId: params.subjectId,
      termId: params.termId,
      month: params.month,
      isPublished: true,
    },
    {
      title: "الشفوي",
      type: "oral" as const,
      maxScore: "20",
      weight: "1.0",
      classId: params.classId,
      subjectId: params.subjectId,
      termId: params.termId,
      month: params.month,
      isPublished: true,
    },
    {
      title: "الواجبات",
      type: "homework" as const,
      maxScore: "20",
      weight: "1.0",
      classId: params.classId,
      subjectId: params.subjectId,
      termId: params.termId,
      month: params.month,
      isPublished: true,
    },
    {
      title: "الاختبار التحريري",
      type: "exam" as const,
      maxScore: "40",
      weight: "1.0",
      classId: params.classId,
      subjectId: params.subjectId,
      termId: params.termId,
      month: params.month,
      isPublished: true,
    },
  ];

  const results = await db
    .insert(assessmentItems)
    .values(itemsToCreate)
    .returning();

  if (results.length > 0) {
    await assessmentCache.deletePattern(`class:${params.classId}*`);
  }

  return results;
};

export const initializeTermExamAssessments = async (params: {
  classId: number;
  subjectId: number;
  termId: number;
  month: number; // 4 for Mid-term, 8 for Final
}) => {
  const isMidTerm = params.month === 4;

  const itemsToCreate = [
    {
      title: isMidTerm ? "اختبار النصفي" : "الاختبار النهائي",
      type: "exam" as const,
      maxScore: "30",
      weight: "1.0",
      classId: params.classId,
      subjectId: params.subjectId,
      termId: params.termId,
      month: params.month,
      isPublished: true,
    },
  ];

  const results = await db
    .insert(assessmentItems)
    .values(itemsToCreate)
    .returning();

  if (results.length > 0) {
    await assessmentCache.deletePattern(`class:${params.classId}*`);
  }

  return results;
};
