import { db } from "@db/client";
import { gradeEntries, assessmentItems, subjects, classes } from "@db/schema";
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

export const upsertGrade = async (data: {
  studentId: string;
  assessmentItemId: number;
  score: string | null;
  gradedById: string;
}) => {
  const [result] = await db
    .insert(gradeEntries)
    .values({
      studentId: data.studentId,
      assessmentItemId: data.assessmentItemId,
      score: data.score,
      gradedById: data.gradedById,
      updatedAt: new Date(),
    })
    .onConflictDoUpdate({
      target: [gradeEntries.studentId, gradeEntries.assessmentItemId],
      set: {
        score: data.score,
        gradedById: data.gradedById,
        updatedAt: new Date(),
      },
    })
    .returning();

  if (result) {
    await gradeCache.delete(`student:${data.studentId}:all`);
  }

  return result;
};

import { enrollments, users } from "@db/schema";
import { asc, inArray, or } from "drizzle-orm";

export const getGradeGridData = async (params: {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
  academicYearId: number;
}) => {
  try {
    // First, check if this class has children (divisions)
    const childClasses = await db
      .select({ id: classes.id })
      .from(classes)
      .where(eq(classes.parentClassId, params.classId));

    // If there are child classes, use them; otherwise use the selected class
    const targetClassIds = childClasses.length > 0 
      ? childClasses.map(c => c.id) 
      : [params.classId];

    // 1. Fetch Assessment Items using standard query builder
    const itemsList = await db
      .select()
      .from(assessmentItems)
      .where(
        and(
          eq(assessmentItems.classId, params.classId),
          eq(assessmentItems.subjectId, params.subjectId),
          eq(assessmentItems.termId, params.termId),
          eq(assessmentItems.month, params.month)
        )
      )
      .orderBy(asc(assessmentItems.id));

    // 2. Fetch Students enrolled in target classes
    const enrolledStudents = await db
      .select({
        studentId: students.userId,
        studentNumber: students.studentNumber,
        name: users.name,
        image: users.image,
        sex: users.sex,
      })
      .from(enrollments)
      .innerJoin(students, eq(enrollments.studentId, students.userId))
      .innerJoin(users, eq(students.userId, users.id))
      .where(
        and(
          targetClassIds.length === 1 
            ? eq(enrollments.classId, targetClassIds[0]!)
            : inArray(enrollments.classId, targetClassIds),
          eq(enrollments.academicYearId, params.academicYearId),
          eq(enrollments.status, "active")
        )
      )
      .orderBy(users.name);

    // 3. Fetch Grades using standard query builder
    let gradesList: (typeof gradeEntries.$inferSelect)[] = [];

    if (itemsList.length > 0) {
      const itemIds = itemsList.map((i) => i.id);
      gradesList = await db
        .select()
        .from(gradeEntries)
        .where(inArray(gradeEntries.assessmentItemId, itemIds));
    }

    return {
      items: itemsList,
      students: enrolledStudents,
      grades: gradesList,
    };
  } catch (error) {
    console.error("Error in getGradeGridData:", error);
    console.error("Params:", params);
    throw error;
  }
};

