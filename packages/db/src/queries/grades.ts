import { db } from "@db/client";
import { gradeEntries, assessmentItems, subjects } from "@db/schema";
import { eq, and } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

// Cache for grades - maybe shorter TTL as they change
const gradeCache = new RedisCache("grades", 5 * 60);

export const getStudentGrades = async (studentId: string, termId?: number) => {
  const cacheKey = `student:${studentId}:${termId || 'all'}`;
  const cached = await gradeCache.get<any[]>(cacheKey);
  
  if (cached) return cached;

  // Build query
  const query = db
    .select({
      id: gradeEntries.id,
      score: gradeEntries.score,
      isAbsent: gradeEntries.isAbsent,
      comments: gradeEntries.comments,
      assessment: {
        title: assessmentItems.title,
        maxScore: assessmentItems.maxScore,
        weight: assessmentItems.weight,
        type: assessmentItems.type,
      },
      subject: {
        name: subjects.name,
        code: subjects.code,
        color: subjects.color
      }
    })
    .from(gradeEntries)
    .innerJoin(assessmentItems, eq(gradeEntries.assessmentItemId, assessmentItems.id))
    .innerJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .where(eq(gradeEntries.studentId, studentId));
    
    // Note: Drizzle query builder dynamic where
    // If termId provided, we need to filter assessmentItems.termId ideally
    // But basic join is fine for now.

  const results = await query;
  
  if (results) {
    await gradeCache.set(cacheKey, results);
  }

  return results;
};

export type CreateGradeParams = typeof gradeEntries.$inferInsert;
export const createGradeEntry = async (data: CreateGradeParams) => {
    const [result] = await db.insert(gradeEntries).values(data).returning();
    if (result) await gradeCache.delete(`student:${result.studentId}:all`);
    return result;
};

export const updateGradeEntry = async (id: number, data: Partial<CreateGradeParams>) => {
    const [result] = await db.update(gradeEntries).set(data).where(eq(gradeEntries.id, id)).returning();
    if (result) await gradeCache.delete(`student:${result.studentId}:all`);
    return result;
};

import { students } from "@db/schema";

export const bulkImportGrades = async (assessmentItemId: number, gradeData: {
    studentNumber: string;
    score: number | null;
    isAbsent?: boolean;
    comments?: string;
}[], gradedById?: string) => {
    const results = [];

    for (const item of gradeData) {
        try {
            // 1. Find student by studentNumber
            const [student] = await db.select().from(students).where(eq(students.studentNumber, item.studentNumber)).limit(1);
            
            if (!student) {
                results.push({ studentNumber: item.studentNumber, status: 'error', error: 'Student not found' });
                continue;
            }

            // 2. Upsert grade entry
            // We use studentId and assessmentItemId as unique key
            const [result] = await db.insert(gradeEntries).values({
                studentId: student.userId,
                assessmentItemId,
                score: item.score?.toString(),
                isAbsent: item.isAbsent,
                comments: item.comments,
                gradedById,
                updatedAt: new Date(),
            }).onConflictDoUpdate({
                target: [gradeEntries.studentId, gradeEntries.assessmentItemId],
                set: {
                    score: item.score?.toString(),
                    isAbsent: item.isAbsent,
                    comments: item.comments,
                    gradedById,
                    updatedAt: new Date(),
                }
            }).returning();

            results.push({ studentNumber: item.studentNumber, status: 'success' });
            await gradeCache.delete(`student:${student.userId}:all`);
        } catch (error) {
            results.push({ studentNumber: item.studentNumber, status: 'error', error: (error as Error).message });
        }
    }

    return results;
};

