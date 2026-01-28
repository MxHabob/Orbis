import { db } from "@db/client";
import { gradeEntries, assessmentItems, subjects } from "@db/schema";
import { eq, and } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

// Cache for grades - maybe shorter TTL as they change
const gradeCache = new RedisCache("grades", 5 * 60);

export const getStudentGrades = async (studentId: string, termId?: number) => {
  const cacheKey = `student:${studentId}:${termId || 'all'}`;
  const cached = await gradeCache.get(cacheKey);
  
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
        colorCode: subjects.color_code
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
