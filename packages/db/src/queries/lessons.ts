import { db } from "@db/client";
import { timeSlots, scheduledLessons, classes, subjects, users } from "@db/schema";
import { eq, and, asc } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const lessonCache = new RedisCache("lessons", 60 * 60);

// ==========================================
// Time Slots
// ==========================================

export const getTimeSlots = async (yearId: number) => {
  const cacheKey = `slots:${yearId}`;
  const cached = await lessonCache.get<typeof timeSlots.$inferSelect[]>(cacheKey);
  if (cached) return cached;

  const results = await db
    .select()
    .from(timeSlots)
    .where(eq(timeSlots.academicYearId, yearId))
    .orderBy(asc(timeSlots.slotNumber));

  if (results) {
    await lessonCache.set(cacheKey, results);
  }
  return results;
};

export type CreateTimeSlotParams = typeof timeSlots.$inferInsert;
export const createTimeSlot = async (data: CreateTimeSlotParams) => {
  const [result] = await db.insert(timeSlots).values(data).returning();
  if (result) {
    await lessonCache.delete(`slots:${result.academicYearId}`);
  }
  return result;
};

// ==========================================
// Scheduled Lessons
// ==========================================

export const getClassSchedule = async (classId: number, yearId: number) => {
  const cacheKey = `schedule:class:${classId}:${yearId}`;
  const cached = await lessonCache.get<any[]>(cacheKey);
  if (cached) return cached;

  const results = await db
    .select({
      id: scheduledLessons.id,
      dayOfWeek: scheduledLessons.dayOfWeek,
      timeSlot: {
        id: timeSlots.id,
        slotNumber: timeSlots.slotNumber,
        startTime: timeSlots.startTime,
        endTime: timeSlots.endTime,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        color: subjects.color,
      },
      teacher: {
        id: users.id,
        name: users.name,
      },
      notes: scheduledLessons.notes,
      classId: scheduledLessons.classId,
      academicYearId: scheduledLessons.academicYearId,
      timeSlotId: scheduledLessons.timeSlotId,
      subjectId: scheduledLessons.subjectId,
      teacherId: scheduledLessons.teacherId,
    })


    .from(scheduledLessons)
    .innerJoin(timeSlots, eq(scheduledLessons.timeSlotId, timeSlots.id))
    .innerJoin(subjects, eq(scheduledLessons.subjectId, subjects.id))
    .innerJoin(users, eq(scheduledLessons.teacherId, users.id))
    .where(
      and(
        eq(scheduledLessons.classId, classId),
        eq(scheduledLessons.academicYearId, yearId),
        eq(scheduledLessons.isActive, true)
      )
    );

  if (results) {
    await lessonCache.set(cacheKey, results);
  }
  return results;
};

export const getTeacherSchedule = async (teacherId: string, yearId: number) => {
    const cacheKey = `schedule:teacher:${teacherId}:${yearId}`;
    const cached = await lessonCache.get<any[]>(cacheKey);
    if (cached) return cached;
  
    const results = await db
      .select({
        id: scheduledLessons.id,
        dayOfWeek: scheduledLessons.dayOfWeek,
        timeSlot: {
          id: timeSlots.id,
          slotNumber: timeSlots.slotNumber,
          startTime: timeSlots.startTime,
          endTime: timeSlots.endTime,
        },
        class: {
          id: classes.id,
          name: classes.name,
        },
        subject: {
          id: subjects.id,
          name: subjects.name,
        },
        notes: scheduledLessons.notes,
        classId: scheduledLessons.classId,
        academicYearId: scheduledLessons.academicYearId,
        timeSlotId: scheduledLessons.timeSlotId,
        subjectId: scheduledLessons.subjectId,
        teacherId: scheduledLessons.teacherId,
      })


      .from(scheduledLessons)
      .innerJoin(timeSlots, eq(scheduledLessons.timeSlotId, timeSlots.id))
      .innerJoin(classes, eq(scheduledLessons.classId, classes.id))
      .innerJoin(subjects, eq(scheduledLessons.subjectId, subjects.id))
      .where(
        and(
          eq(scheduledLessons.teacherId, teacherId),
          eq(scheduledLessons.academicYearId, yearId),
          eq(scheduledLessons.isActive, true)
        )
      );
  
    if (results) {
      await lessonCache.set(cacheKey, results);
    }
    return results;
};

export type CreateScheduledLessonParams = typeof scheduledLessons.$inferInsert;
export const createScheduledLesson = async (data: CreateScheduledLessonParams) => {
  const [result] = await db.insert(scheduledLessons).values(data).returning();
  if (result) {
    await lessonCache.delete(`schedule:class:${result.classId}:${result.academicYearId}`);
    await lessonCache.delete(`schedule:teacher:${result.teacherId}:${result.academicYearId}`);
  }
  return result;
};

export const getScheduledLessonById = async (id: number) => {
  const [result] = await db
    .select({
      id: scheduledLessons.id,
      dayOfWeek: scheduledLessons.dayOfWeek,
      classId: scheduledLessons.classId,
      timeSlotId: scheduledLessons.timeSlotId,
      subjectId: scheduledLessons.subjectId,
      teacherId: scheduledLessons.teacherId,
      academicYearId: scheduledLessons.academicYearId,
      notes: scheduledLessons.notes,
    })
    .from(scheduledLessons)
    .where(eq(scheduledLessons.id, id));

  return result;
};

export const updateScheduledLesson = async (id: number, data: Partial<CreateScheduledLessonParams>) => {
  const [result] = await db
    .update(scheduledLessons)
    .set({ ...data, updatedAt: new Date() })
    .where(eq(scheduledLessons.id, id))
    .returning();

  if (result) {
    await lessonCache.delete(`schedule:class:${result.classId}:${result.academicYearId}`);
    await lessonCache.delete(`schedule:teacher:${result.teacherId}:${result.academicYearId}`);
  }
  return result;
};

export const deleteScheduledLesson = async (id: number) => {
    const [existing] = await db.select().from(scheduledLessons).where(eq(scheduledLessons.id, id));
    if (existing) {
        await db.delete(scheduledLessons).where(eq(scheduledLessons.id, id));
        await lessonCache.delete(`schedule:class:${existing.classId}:${existing.academicYearId}`);
        await lessonCache.delete(`schedule:teacher:${existing.teacherId}:${existing.academicYearId}`);
    }
};

