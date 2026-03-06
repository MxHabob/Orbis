import { db } from "@db/client";
import { students, classes, users, enrollments, academicYears } from "@db/schema";
import { eq, asc, desc, and, or, ilike, inArray } from "drizzle-orm";

import { RedisCache } from "@midday/cache";

const studentCache = new RedisCache("student", 30 * 60);

export const getAllStudentss = async () => {
    return db
        .select({
            userId: students.userId,
            studentNumber: students.studentNumber,
            secretNumber: students.secretNumber,
            currentClassId: students.currentClassId,
            status: students.status,
            notes: students.notes,
            stats: students.stats,
            name: users.name,
            image: users.image,
            email: users.email,
            sex: users.sex,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
            },
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(academicYears, eq(
            db.select({ id: academicYears.id })
              .from(academicYears)
              .where(eq(academicYears.isCurrent, true))
              .limit(1),
            academicYears.id
        )) // Fallback to current year if enrollment missing or complex
        .orderBy(asc(users.name))
        .limit(10);
};

export const getStudentsByClassId = async (
  classId: number, 
  limit = 50, 
  offset = 0, 
  q?: string, 
  sort?: string[] | null,
  academicYearId?: number
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

    // Get the current academic year if not provided
    let yearId = academicYearId;
    if (!yearId) {
        const [currentYear] = await db
            .select({ id: academicYears.id })
            .from(academicYears)
            .where(eq(academicYears.isCurrent, true))
            .limit(1);
        
        yearId = currentYear?.id;
    }

    // Get the class IDs for the selected class and its children (sections)
    const childClasses = await db
        .select({ id: classes.id })
        .from(classes)
        .where(eq(classes.parentClassId, classId));

    const targetClassIds = childClasses.length > 0
        ? [classId, ...childClasses.map(c => c.id)]
        : [classId];

    // Use enrollment-based lookup — MUST filter by academic year
    // This ensures reports show students who ARE in this class for the targeted year
    return db
        .select({
            userId: students.userId,
            studentNumber: students.studentNumber,
            secretNumber: students.secretNumber,
            currentClassId: students.currentClassId,
            status: students.status,
            notes: students.notes,
            stats: students.stats,
            name: users.name,
            image: users.image,
            email: users.email,
            sex: users.sex,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
            },
            enrollmentDate: enrollments.enrolledAt,
        })
        .from(enrollments)
        .innerJoin(students, eq(enrollments.studentId, students.userId))
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(academicYears, eq(enrollments.academicYearId, academicYears.id))
        .where(
            and(
                targetClassIds.length === 1
                    ? eq(enrollments.classId, targetClassIds[0]!)
                    : inArray(enrollments.classId, targetClassIds),
                yearId ? eq(enrollments.academicYearId, yearId) : undefined,
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
            notes: students.notes,
            stats: students.stats,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
            },
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(enrollments, and(eq(enrollments.studentId, students.userId), eq(enrollments.classId, students.currentClassId)))
        .leftJoin(academicYears, eq(enrollments.academicYearId, academicYears.id))
        .where(eq(students.userId, userId))
        .orderBy(desc(academicYears.yearName))
        .limit(1);
    
    if (student) {
        await studentCache.set(cacheKey, student);
    }
    return student;
};

export const getStudentsByUserIds = async (userIds: string[]) => {
    if (userIds.length === 0) return [];
    
    return db
        .select({
            userId: students.userId,
            studentNumber: students.studentNumber,
            secretNumber: students.secretNumber,
            currentClassId: students.currentClassId,
            status: students.status,
            notes: students.notes,
            stats: students.stats,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
            },
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(enrollments, and(eq(enrollments.studentId, students.userId), eq(enrollments.classId, students.currentClassId)))
        .leftJoin(academicYears, eq(enrollments.academicYearId, academicYears.id))
        .where(inArray(students.userId, userIds));
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
            notes: students.notes,
            stats: students.stats,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
            },
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(enrollments, and(eq(enrollments.studentId, students.userId), eq(enrollments.classId, students.currentClassId)))
        .leftJoin(academicYears, eq(enrollments.academicYearId, academicYears.id))
        .where(eq(students.userId, id))
        .orderBy(desc(academicYears.yearName))
        .limit(1);
    
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
            notes: students.notes,
            stats: students.stats,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
            },
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(enrollments, and(eq(enrollments.studentId, students.userId), eq(enrollments.classId, students.currentClassId)))
        .leftJoin(academicYears, eq(enrollments.academicYearId, academicYears.id))
        .where(eq(students.studentNumber, studentNumber))
        .orderBy(desc(academicYears.yearName))
        .limit(1);
    
    if (student) {
        await studentCache.set(cacheKey, student);
    }
    return student;
};

export type CreateStudentParams = typeof students.$inferInsert;

export const generateSecretNumber = () => {
    // Generate 6 characters alphanumeric (e.g., 24A7F9)
    const chars = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    let result = '';
    for (let i = 0; i < 6; i++) {
        result += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return result;
}

export const generateNextStudentNumber = async (transaction?: any) => {
    const currentYear = new Date().getFullYear();
    const prefix = `STU-${currentYear}-`;
    
    const client = transaction || db;

    const [lastStudent] = await client
        .select({ studentNumber: students.studentNumber })
        .from(students)
        .where(ilike(students.studentNumber, `${prefix}%`))
        .orderBy(desc(students.studentNumber))
        .limit(1);
    
    let nextSeq = 1;
    if (lastStudent) {
        // Expected format: STU-2025-001
        const parts = lastStudent.studentNumber.split('-');
        if (parts.length >= 3) {
            const lastSeq = parseInt(parts[2]);
            if (!isNaN(lastSeq)) {
                nextSeq = lastSeq + 1;
            }
        }
    }
    
    return `${prefix}${nextSeq.toString().padStart(3, '0')}`;
}

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
    const { userId, name, email, phone, sex, studentNumber, secretNumber, currentClassId, status, notes, stats } = data;

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
            notes,
            stats,
        }).where(eq(students.userId, targetUserId));

        // Invalidate class caches as student counts likely changed
        const classCache = new RedisCache("classes", 60 * 60);
        await classCache.invalidate().catch(() => {});
    } else {
        // ... (CREATE branch remains same)
        // Create
        return db.transaction(async (tx) => {
            const [newUser] = await tx.insert(users).values({
                name,
                email,
                phone,
                sex,
                role: 'student',
            }).returning();

            if (!newUser) throw new Error("Failed to create user");
            targetUserId = newUser.id;

            // Generate numbers if not provided
            const finalStudentNumber = studentNumber || await generateNextStudentNumber(tx);
            const finalSecretNumber = secretNumber || generateSecretNumber();

            await tx.insert(students).values({
                userId: targetUserId,
                studentNumber: finalStudentNumber,
                secretNumber: finalSecretNumber,
                currentClassId,
                status,
                notes,
                stats,
            });

            // Ensure enrollment if class is provided
            if (currentClassId) {
                const [currentYear] = await tx.select().from(academicYears).where(eq(academicYears.isCurrent, true)).limit(1);
                if (currentYear) {
                    await tx.insert(enrollments).values({
                        studentId: targetUserId,
                        classId: currentClassId,
                        academicYearId: currentYear.id,
                    }).onConflictDoNothing();
                }
            }

            await studentCache.delete(`user:${targetUserId}`);
            return { id: targetUserId };
        });
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

export const bulkImportStudents = async (
  classId: number, 
  studentData: {
    studentNumber?: string;
    secretNumber?: string;
    name: string;
    phone?: string;
    email?: string;
    sex?: 'male' | 'female' | 'unspecified';
  }[],
  academicYearId?: number
) => {
  // 1. Get academic year: Use provided ID or current one from DB
  let yearId = academicYearId;
  
  if (!yearId) {
    const [currentYear] = await db
      .select()
      .from(academicYears)
      .where(eq(academicYears.isCurrent, true))
      .limit(1);
      
    if (!currentYear) throw new Error("No current academic year found and no yearId provided");
    yearId = currentYear.id;
  }

  const results: { studentNumber: string | undefined; status: string; error?: string }[] = [];

  // Use transaction for bulk import to ensure numbers are consistent if generated
  return db.transaction(async (tx) => {
    for (const item of studentData) {
      try {
        let userId: string;
        let finalStudentNumber = item.studentNumber;
        let finalSecretNumber = item.secretNumber;

        // Check if student exists by number (if provided)
        let existingStudent = null;
        if (finalStudentNumber) {
           [existingStudent] = await tx
            .select()
            .from(students)
            .where(eq(students.studentNumber, finalStudentNumber))
            .limit(1);
        }
        
        if (existingStudent) {
          userId = existingStudent.userId;
          // Update user info and sync current class
          await tx.update(users).set({ 
            name: item.name, 
            phone: item.phone, 
            email: item.email,
            sex: item.sex || 'unspecified'
          }).where(eq(users.id, userId));

          // Update student's current class
          await tx.update(students).set({
            currentClassId: classId,
            secretNumber: finalSecretNumber || existingStudent.secretNumber, // Keep existing if not provided
          }).where(eq(students.userId, userId));
        } else {
          const [newUser] = await tx.insert(users).values({
            name: item.name,
            phone: item.phone,
            email: item.email,
            role: 'student',
            sex: item.sex || 'unspecified'
          }).returning();
          
          if (!newUser) throw new Error("Failed to create user");
          userId = newUser.id;

          // Generate numbers if not provided
          if (!finalStudentNumber) {
              finalStudentNumber = await generateNextStudentNumber(tx);
          }
          if (!finalSecretNumber) {
              finalSecretNumber = generateSecretNumber();
          }

          await tx.insert(students).values({
            userId,
            studentNumber: finalStudentNumber,
            secretNumber: finalSecretNumber,
            currentClassId: classId,
          });
        }

        // 2. Ensure enrollment in the target year
        await tx.insert(enrollments).values({
          studentId: userId,
          classId,
          academicYearId: yearId,
        }).onConflictDoUpdate({
          target: [enrollments.studentId, enrollments.classId, enrollments.academicYearId],
          set: {
            status: 'active',
            enrolledAt: new Date(),
          }
        });

        results.push({ studentNumber: finalStudentNumber, status: 'success' });
      } catch (error) {
        results.push({ studentNumber: item.studentNumber || 'unknown', status: 'error', error: (error as Error).message });
      }
    }
    return results;
  });
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

