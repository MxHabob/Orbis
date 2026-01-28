import { db } from "@db/client";
import { students, classes, users } from "@db/schema";
import { eq } from "drizzle-orm";
// We might want to create a specific student cache later, but for now we can use a generic approach or user cache if linked
// Assuming we want to cache student profiles.
import { RedisCache } from "@midday/cache";

const studentCache = new RedisCache("student", 30 * 60);

export const getStudentByUserId = async (userId: string) => {
  const cacheKey = `user:${userId}`;
  const cached = await studentCache.get(cacheKey);
  
  if (cached) return cached;

  const [student] = await db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      secretNumber: students.secretNumber,
      currentClassId: students.currentClassId,
      status: students.status,
      stats: students.stats,
      user: {
        id: users.id,
        name: users.name,
        image: users.image,
      },
      class: {
        id: classes.id,
        name: classes.name,
        division: classes.division
      }
    })
    .from(students)
    .innerJoin(users, eq(students.userId, users.id))
    .leftJoin(classes, eq(students.currentClassId, classes.id))
    .where(eq(students.userId, userId));

  if (student) {
    await studentCache.set(cacheKey, student);
  }

  return student;
};

export const getStudentByNumber = async (studentNumber: string) => {
    const cacheKey = `number:${studentNumber}`;
    const cached = await studentCache.get(cacheKey);
    if (cached) return cached;

    const [student] = await db
        .select()
        .from(students)
        .where(eq(students.studentNumber, studentNumber));
    
    if (student) {
        await studentCache.set(cacheKey, student);
    }
    return student;
};
