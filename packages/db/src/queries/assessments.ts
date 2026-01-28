import { db } from "@db/client";
import { assessmentItems, subjects, classes, academicTerms } from "@db/schema";
import { eq, and } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const assessmentCache = new RedisCache("assessments", 60 * 60);

export type CreateAssessmentParams = typeof assessmentItems.$inferInsert;

export const createAssessment = async (data: CreateAssessmentParams) => {
  const [result] = await db.insert(assessmentItems).values(data).returning();
  
  // Invalidate cache for this class/subject
  if (result) {
    await assessmentCache.delete(`class:${result.classId}`);
  }
  return result;
};

export const getAssessmentsForClass = async (classId: number, subjectId?: number, termId?: number) => {
  // Simple caching strategy for list
  const cacheKey = `class:${classId}:${subjectId || 'all'}:${termId || 'all'}`;
  const cached = await assessmentCache.get(cacheKey);
  
  if (cached) return cached;

  const conditions = [eq(assessmentItems.classId, classId)];

  if (subjectId) {
    conditions.push(eq(assessmentItems.subjectId, subjectId));
  }
  if (termId) {
    conditions.push(eq(assessmentItems.termId, termId));
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
        code: subjects.code
      },
      term: {
        name: academicTerms.name
      }
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

export const updateAssessment = async (id: number, data: Partial<CreateAssessmentParams>) => {
  const [result] = await db.update(assessmentItems).set(data).where(eq(assessmentItems.id, id)).returning();
  if (result) await assessmentCache.delete(`class:${result.classId}`);
  return result;
};

export const deleteAssessment = async (id: number) => {
  const [existing] = await db.select().from(assessmentItems).where(eq(assessmentItems.id, id));
  await db.delete(assessmentItems).where(eq(assessmentItems.id, id));
  if (existing) await assessmentCache.delete(`class:${existing.classId}`);
};
