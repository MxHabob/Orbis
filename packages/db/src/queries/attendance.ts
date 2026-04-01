import { db } from "@db/client";
import { attendanceRecords, students, users, scheduledLessons, subjects } from "@db/schema";
import { eq, and, sql, asc } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const attendanceCache = new RedisCache("attendance", 10 * 60);

export const getAttendanceByLesson = async (lessonId: number, date: string) => {
  const cacheKey = `lesson:${lessonId}:${date}`;
  const cached = await attendanceCache.get<any[]>(cacheKey);
  if (cached) return cached;

  const results = await db
    .select({
      id: attendanceRecords.id,
      date: attendanceRecords.date,
      status: attendanceRecords.status,
      notes: attendanceRecords.notes,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        name: users.name,
      }
    })
    .from(attendanceRecords)
    .innerJoin(students, eq(attendanceRecords.studentId, students.userId))
    .innerJoin(users, eq(students.userId, users.id))
    .where(
      and(
        eq(attendanceRecords.scheduledLessonId, lessonId),
        eq(attendanceRecords.date, date)
      )
    );

  if (results) {
    await attendanceCache.set(cacheKey, results);
  }
  return results;
};

export const getStudentAttendance = async (studentId: string, startDate?: string, endDate?: string) => {
    const cacheKey = `student:${studentId}:${startDate || 'all'}:${endDate || 'all'}`;
    const cached = await attendanceCache.get<any[]>(cacheKey);
    if (cached) return cached;

    const conditions = [eq(attendanceRecords.studentId, studentId)];
    if (startDate) conditions.push(sql`${attendanceRecords.date} >= ${startDate}`);
    if (endDate) conditions.push(sql`${attendanceRecords.date} <= ${endDate}`);

    const results = await db
        .select({
            id: attendanceRecords.id,
            date: attendanceRecords.date,
            status: attendanceRecords.status,
            lesson: {
                id: scheduledLessons.id,
                subject: subjects.name,
            }
        })
        .from(attendanceRecords)
        .innerJoin(scheduledLessons, eq(attendanceRecords.scheduledLessonId, scheduledLessons.id))
        .innerJoin(subjects, eq(scheduledLessons.subjectId, subjects.id))
        .where(and(...conditions));
    
    if (results) {
        await attendanceCache.set(cacheKey, results);
    }
    return results;
};

export type MarkAttendanceParams = typeof attendanceRecords.$inferInsert;
export const markAttendance = async (data: MarkAttendanceParams) => {
  const [result] = await db.insert(attendanceRecords)
    .values(data)
    .onConflictDoUpdate({
      target: [attendanceRecords.studentId, attendanceRecords.scheduledLessonId],
      set: {
        status: data.status,
        notes: data.notes,
        arrivalTime: data.arrivalTime,
        markedById: data.markedById,
        updatedAt: new Date(),
      }
    })
    .returning();
  
  if (result) {
    await attendanceCache.delete(`lesson:${result.scheduledLessonId}:${result.date}`);
    await attendanceCache.delete(`student:${result.studentId}:all:all`);
  }
  return result;
};

export const bulkMarkAttendance = async (records: MarkAttendanceParams[]) => {
    const results = [];
    for (const record of records) {
        try {
            const res = await markAttendance(record);
            results.push(res);
        } catch (e) {
            console.error("Failed to mark attendance for student", record.studentId, e);
        }
    }
    return results;
};

/**
 * Get data for the class attendance sheet export.
 */
export const getAttendanceSheetData = async (classId: number) => {


  return db
    .select({
      name: users.name,
      phone: users.phone,
      status: students.status,
    })
    .from(students)
    .innerJoin(users, eq(students.userId, users.id))
    .where(eq(students.currentClassId, classId))
    .orderBy(asc(users.name));
};
