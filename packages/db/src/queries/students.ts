import { db } from "@db/client";
import { students, classes, users, enrollments } from "@db/schema";
import { eq, asc, desc, and, or, ilike, inArray } from "drizzle-orm";

import { RedisCache } from "@midday/cache";

const studentCache = new RedisCache("student", 30 * 60);

export const getAllStudents = async () => {
    const cacheKey = `all`;
    const cached = await studentCache.get<typeof students.$inferSelect>(cacheKey);
    if (cached) return cached;

    const [student] = await db
        .select()
        .from(students);
    
    if (student) {
        await studentCache.set(cacheKey, student);
    }
    return student;
};

export const getStudentsByClassId = async (
  classId: number, 
  limit = 50, 
  offset = 0, 
  q?: string, 
  sort?: string[] | null
) => {
    const orderBy = [];

    if (sort?.length) {
        sort.forEach((s) => {
            const [column, direction] = s.split(":");
            const dirFunc = direction === "desc" ? desc : asc;

            switch (column) {
                case "user.name":
                case "name":
                    orderBy.push(dirFunc(users.name));
                    break;
                case "studentNumber":
                case "student_number":
                    orderBy.push(dirFunc(students.studentNumber));
                    break;
                case "status":
                    orderBy.push(dirFunc(students.status));
                    break;
            }
        });
    }

    if (orderBy.length === 0) {
        orderBy.push(asc(users.name));
    }

    // Get the class IDs for the selected class and its children (sections)
    const classIds = db
        .select({ id: classes.id })
        .from(classes)
        .where(
            or(
                eq(classes.id, classId),
                eq(classes.parentClassId, classId)
            )
        );

    return db
        .select({
            userId: students.userId,
            studentNumber: students.studentNumber,
            secretNumber: students.secretNumber,
            currentClassId: students.currentClassId,
            status: students.status,
            stats: students.stats,
            name: users.name,
            image: users.image,
            email: users.email,
            sex: users.sex,
            currentClass: {
                id: classes.id,
                name: classes.name,
            },
            enrollmentDate: enrollments.enrolledAt,
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(enrollments, and(
            eq(enrollments.studentId, students.userId),
            inArray(enrollments.classId, classIds) // Get enrollment for any relevant class
        ))
        .where(
            and(
                inArray(students.currentClassId, classIds),
                q ? or(
                    ilike(users.name, `%${q}%`),
                    ilike(students.studentNumber, `%${q}%`)
                ) : undefined
            )
        )
        .orderBy(...orderBy)
        .limit(limit)
        .offset(offset);
};


export const getStudentByUserId = async (userId: string) => {
    const cacheKey = `user:${userId}`;
    const cached = await studentCache.get<any>(cacheKey);
    if (cached) return cached;

    const [student] = await db
        .select({
            userId: students.userId,
            studentNumber: students.studentNumber,
            secretNumber: students.secretNumber,
            currentClassId: students.currentClassId,
            status: students.status,
            stats: students.stats,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .where(eq(students.userId, userId));
    
    if (student) {
        await studentCache.set(cacheKey, student);
    }
    return student;
};

export const getStudentById = async (id: string) => {
    const cacheKey = `user:${id}`;
    const cached = await studentCache.get<any>(cacheKey);
    if (cached) return cached;

    const [student] = await db
        .select({
            userId: students.userId,
            studentNumber: students.studentNumber,
            secretNumber: students.secretNumber,
            currentClassId: students.currentClassId,
            status: students.status,
            stats: students.stats,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .where(eq(students.userId, id));
    
    if (student) {
        await studentCache.set(cacheKey, student);
    }
    return student;
};

export const getStudentByNumber = async (studentNumber: string) => {
    const cacheKey = `number:${studentNumber}`;
    const cached = await studentCache.get<any>(cacheKey);
    if (cached) return cached;

    const [student] = await db
        .select({
            userId: students.userId,
            studentNumber: students.studentNumber,
            secretNumber: students.secretNumber,
            currentClassId: students.currentClassId,
            status: students.status,
            stats: students.stats,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .where(eq(students.studentNumber, studentNumber));
    
    if (student) {
        await studentCache.set(cacheKey, student);
    }
    return student;
};

export type CreateStudentParams = typeof students.$inferInsert;
export const createStudent = async (data: CreateStudentParams) => {
    const [result] = await db.insert(students).values(data).returning();
    return result;
};

export const updateStudent = async (userId: string, data: Partial<CreateStudentParams>) => {
    const [result] = await db.update(students).set(data).where(eq(students.userId, userId)).returning();
    await studentCache.delete(`user:${userId}`);
    if (result) await studentCache.delete(`number:${result.studentNumber}`);
    return result;
};

export const upsertStudent = async (data: any) => {
    const { userId, name, email, phone, sex, studentNumber, secretNumber, currentClassId, status, stats } = data;

    let targetUserId = userId;

    if (targetUserId) {
        // Update
        await db.update(users).set({
            name,
            email,
            phone,
            sex,
            updatedAt: new Date(),
        }).where(eq(users.id, targetUserId));

        await db.update(students).set({
            studentNumber,
            secretNumber,
            currentClassId,
            status,
            stats,
        }).where(eq(students.userId, targetUserId));
    } else {
        // Create
        const [newUser] = await db.insert(users).values({
            name,
            email,
            phone,
            sex,
            role: 'student',
        }).returning();

        if (!newUser) throw new Error("Failed to create user");
        targetUserId = newUser.id;

        await db.insert(students).values({
            userId: targetUserId,
            studentNumber,
            secretNumber,
            currentClassId,
            status,
            stats,
        });

        // Ensure enrollment if class is provided
        if (currentClassId) {
            const [currentYear] = await db.select().from(academicYears).where(eq(academicYears.isCurrent, true)).limit(1);
            if (currentYear) {
                await db.insert(enrollments).values({
                    studentId: targetUserId,
                    classId: currentClassId,
                    academicYearId: currentYear.id,
                }).onConflictDoNothing();
            }
        }
    }

    await studentCache.delete(`user:${targetUserId}`);
    return { id: targetUserId };
};

export const deleteStudent = async (userId: string) => {
    // Usually we delete the user, and cascade deletes student.
    // But if we delete just student profile:
    await db.delete(students).where(eq(students.userId, userId));
    await studentCache.delete(`user:${userId}`);
}

import { academicYears } from "@db/schema";

export const bulkImportStudents = async (classId: number, studentData: {
  studentNumber: string;
  name: string;
  phone?: string;
  email?: string;
  sex?: 'male' | 'female' | 'unspecified';
}[]) => {
  // Get current academic year
  const [currentYear] = await db.select().from(academicYears).where(eq(academicYears.isCurrent, true)).limit(1);
  if (!currentYear) throw new Error("No current academic year found");

  const results = [];

  for (const item of studentData) {
    try {
      // 1. Create or get user
      // We check by email or phone if provided, otherwise we'll have to create a new user.
      // For simplicity in this demo/impl, we create a new user for each student if not exists by phone/email.
      // But usually, students are unique by studentNumber.
      
      const [existingStudent] = await db.select().from(students).where(eq(students.studentNumber, item.studentNumber)).limit(1);
      
      let userId: string;

      if (existingStudent) {
        userId = existingStudent.userId;
        // Update user info
        await db.update(users).set({ 
          name: item.name, 
          phone: item.phone, 
          email: item.email,
          sex: item.sex || 'unspecified'
        }).where(eq(users.id, userId));
      } else {
        const [newUser] = await db.insert(users).values({
          name: item.name,
          phone: item.phone,
          email: item.email,
          role: 'student',
          sex: item.sex || 'unspecified'
        }).returning();
        if (!newUser) throw new Error("Failed to create user");
        userId = newUser.id;

        await db.insert(students).values({
          userId,
          studentNumber: item.studentNumber,
          currentClassId: classId,
        });
      }

      // 2. Ensure enrollment
      await db.insert(enrollments).values({
        studentId: userId,
        classId,
        academicYearId: currentYear.id,
      }).onConflictDoNothing();

      results.push({ studentNumber: item.studentNumber, status: 'success' });
    } catch (error) {
      results.push({ studentNumber: item.studentNumber, status: 'error', error: (error as Error).message });
    }
  }

  return results;
};

export const getStudentEnrollments = async (studentId: string) => {
    return db
        .select({
            id: enrollments.id,
            status: enrollments.status,
            enrolledAt: enrollments.enrolledAt,
            class: {
                id: classes.id,
                name: classes.name,
            },
            academicYear: {
                id: academicYears.id,
                name: academicYears.yearName,
            }
        })
        .from(enrollments)
        .innerJoin(classes, eq(enrollments.classId, classes.id))
        .innerJoin(academicYears, eq(enrollments.academicYearId, academicYears.id))
        .where(eq(enrollments.studentId, studentId));
};

