import { db } from "@db/client";
import { students, classes, users, enrollments, academicYears, parentStudents } from "@db/schema";
import { eq, asc, desc, and, or, ilike, inArray, sql } from "drizzle-orm";

import { RedisCache } from "@midday/cache";

const studentCache = new RedisCache("student", 30 * 60);

/**
 * Normalize a student name for duplicate comparison.
 * Trims whitespace, collapses multiple spaces, and lowercases.
 */
export const normalizeStudentName = (name: string): string => {
  return name.trim().replace(/\s+/g, ' ').toLowerCase();
};

/**
 * Find existing students by name+sex pairs (for duplicate detection).
 * Returns matching students with their current class info.
 */
export const findExistingStudentsByNameAndSex = async (
  pairs: { name: string; sex: string }[]
): Promise<{
  userId: string;
  studentNumber: string;
  currentClassId: number | null;
  name: string | null;
  sex: string;
  className: string | null;
}[]> => {
  if (pairs.length === 0) return [];

  const uniqueNames = [...new Set(pairs.map(p => normalizeStudentName(p.name)))];
  if (uniqueNames.length === 0) return [];

  // Query students whose names match (case-insensitive, trimmed)
  const nameConditions = uniqueNames.map(name =>
    sql`LOWER(TRIM(${users.name})) = ${name}`
  );

  const existingStudents = await db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      currentClassId: students.currentClassId,
      name: users.name,
      sex: users.sex,
      className: classes.name,
    })
    .from(students)
    .innerJoin(users, eq(students.userId, users.id))
    .leftJoin(classes, eq(students.currentClassId, classes.id))
    .where(or(...nameConditions));

  // Filter for exact name+sex pair matches
  const pairSet = new Set(
    pairs.map(p => `${normalizeStudentName(p.name)}|${(p.sex || 'unspecified').toLowerCase()}`)
  );

  return existingStudents.filter(s =>
    pairSet.has(`${normalizeStudentName(s.name || '')}|${(s.sex || 'unspecified').toLowerCase()}`)
  );
};

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
            metadata: students.metadata,
            name: users.name,
            image: users.image,
            email: users.email,
            sex: users.sex,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
                academicYearId: enrollments.academicYearId,
            },
        })
        .from(students)
        .innerJoin(users, eq(students.userId, users.id))
        .leftJoin(classes, eq(students.currentClassId, classes.id))
        .leftJoin(enrollments, and(eq(enrollments.studentId, students.userId), eq(enrollments.classId, students.currentClassId)))
        .leftJoin(academicYears, eq(enrollments.academicYearId, academicYears.id))
        .orderBy(desc(academicYears.yearName), asc(users.name))
        .limit(50);
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
            metadata: students.metadata,
            name: users.name,
            image: users.image,
            email: users.email,
            sex: users.sex,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
                academicYearId: enrollments.academicYearId,
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
    const cacheKey = `user:v2:${userId}`;
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
            metadata: students.metadata,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
                academicYearId: enrollments.academicYearId,
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
        // Fetch parents
        const parents = await db
            .select({
                relationship: parentStudents.relationship,
                name: users.name,
                phone: users.phone,
            })
            .from(parentStudents)
            .innerJoin(users, eq(parentStudents.parentId, users.id))
            .where(eq(parentStudents.studentId, student.userId));

        const father = parents.find(p => p.relationship === 'father');
        const mother = parents.find(p => p.relationship === 'mother');

        const enrichedStudent = {
            ...student,
            fatherName: father?.name || null,
            fatherPhone: father?.phone || null,
            motherName: mother?.name || null,
            motherPhone: mother?.phone || null,
        };

        await studentCache.set(cacheKey, enrichedStudent);
        return enrichedStudent;
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
            metadata: students.metadata,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
                academicYearId: enrollments.academicYearId,
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
    const cacheKey = `user:v2:${id}`;
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
            metadata: students.metadata,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
                academicYearId: enrollments.academicYearId,
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
        // Fetch parents
        const parents = await db
            .select({
                relationship: parentStudents.relationship,
                name: users.name,
                phone: users.phone,
            })
            .from(parentStudents)
            .innerJoin(users, eq(parentStudents.parentId, users.id))
            .where(eq(parentStudents.studentId, student.userId));

        const father = parents.find(p => p.relationship === 'father');
        const mother = parents.find(p => p.relationship === 'mother');

        const enrichedStudent = {
            ...student,
            fatherName: father?.name || null,
            fatherPhone: father?.phone || null,
            motherName: mother?.name || null,
            motherPhone: mother?.phone || null,
        };

        await studentCache.set(cacheKey, enrichedStudent);
        return enrichedStudent;
    }
    return student;
};

export const getStudentByNumber = async (studentNumber: string) => {
    const cacheKey = `user:v2:${studentNumber}`;
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
            metadata: students.metadata,
            name: users.name,
            email: users.email,
            phone: users.phone,
            sex: users.sex,
            image: users.image,
            currentClass: {
                id: classes.id,
                name: classes.name,
                yearName: academicYears.yearName,
                academicYearId: enrollments.academicYearId,
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
        // Fetch parents
        const parents = await db
            .select({
                relationship: parentStudents.relationship,
                name: users.name,
                phone: users.phone,
            })
            .from(parentStudents)
            .innerJoin(users, eq(parentStudents.parentId, users.id))
            .where(eq(parentStudents.studentId, student.userId));

        const father = parents.find(p => p.relationship === 'father');
        const mother = parents.find(p => p.relationship === 'mother');

        const enrichedStudent = {
            ...student,
            fatherName: father?.name || null,
            fatherPhone: father?.phone || null,
            motherName: mother?.name || null,
            motherPhone: mother?.phone || null,
        };

        await studentCache.set(cacheKey, enrichedStudent);
        return enrichedStudent;
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
    await studentCache.delete(`user:v2:${userId}`);
    if (result) await studentCache.delete(`number:${result.studentNumber}`);
    return result;
};

export const upsertStudent = async (data: any) => {
    const { 
        userId, name, email, phone, sex, studentNumber, 
        secretNumber, currentClassId, status, notes, 
        stats, metadata, fatherName, fatherPhone, 
        motherName, motherPhone 
    } = data;

    let targetUserId = userId;

    if (targetUserId) {
        // Update user
        await db.update(users).set({
            name,
            email,
            phone,
            sex,
            updatedAt: new Date(),
        }).where(eq(users.id, targetUserId));

        // Update student record
        await db.update(students).set({
            studentNumber,
            secretNumber,
            currentClassId,
            status,
            notes,
            stats,
            metadata,
        }).where(eq(students.userId, targetUserId));

        // Handle parents for existing student
        await db.transaction(async (tx) => {
            if (fatherName) {
                // Check if already has a father
                const [existingFather] = await tx
                    .select()
                    .from(parentStudents)
                    .where(and(eq(parentStudents.studentId, targetUserId), eq(parentStudents.relationship, 'father')))
                    .limit(1);

                if (!existingFather) {
                    const [fatherUser] = await tx
                        .insert(users)
                        .values({
                            name: fatherName,
                            phone: fatherPhone || null,
                            role: "parent",
                        })
                        .returning({ id: users.id });

                    if (fatherUser) {
                        await tx.insert(parentStudents).values({
                            parentId: fatherUser.id,
                            studentId: targetUserId,
                            relationship: "father",
                            isPrimary: true,
                        });
                    }
                } else {
                    // Update father's info
                    await tx.update(users).set({
                        name: fatherName,
                        phone: fatherPhone || null,
                    }).where(eq(users.id, existingFather.parentId));
                }
            }

            if (motherName) {
                // Check if already has a mother
                const [existingMother] = await tx
                    .select()
                    .from(parentStudents)
                    .where(and(eq(parentStudents.studentId, targetUserId), eq(parentStudents.relationship, 'mother')))
                    .limit(1);

                if (!existingMother) {
                    const [motherUser] = await tx
                        .insert(users)
                        .values({
                            name: motherName,
                            phone: motherPhone || null,
                            role: "parent",
                        })
                        .returning({ id: users.id });

                    if (motherUser) {
                        await tx.insert(parentStudents).values({
                            parentId: motherUser.id,
                            studentId: targetUserId,
                            relationship: "mother",
                            isPrimary: !fatherName,
                        });
                    }
                } else {
                    // Update mother's info
                    await tx.update(users).set({
                        name: motherName,
                        phone: motherPhone || null,
                    }).where(eq(users.id, existingMother.parentId));
                }
            }
        });

        // Invalidate class caches as student counts likely changed
        const classCache = new RedisCache("classes", 60 * 60);
        await classCache.invalidate().catch(() => {});
    } else {
        // CHECK FOR DUPLICATE before creating (by name + sex)
        const existingDuplicates = await findExistingStudentsByNameAndSex([
            { name: name || '', sex: sex || 'unspecified' }
        ]);

        if (existingDuplicates.length > 0) {
            const dup = existingDuplicates[0]!;
            const dupClassName = dup.className || 'غير محدد';
            const sexLabel = sex === 'male' ? 'ذكر' : sex === 'female' ? 'أنثى' : '';
            throw new Error(
                `الطالب "${name}"${sexLabel ? ` (${sexLabel})` : ''} مسجل بالفعل في الصف "${dupClassName}" برقم "${dup.studentNumber}". لا يمكن إضافته مرة أخرى.`
            );
        }

        // Create new student
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
                metadata,
            });

            // Handle parents for new student
            if (fatherName) {
                const [fatherUser] = await tx
                    .insert(users)
                    .values({
                        name: fatherName,
                        phone: fatherPhone || null,
                        role: "parent",
                    })
                    .returning({ id: users.id });

                if (fatherUser) {
                    await tx.insert(parentStudents).values({
                        parentId: fatherUser.id,
                        studentId: targetUserId,
                        relationship: "father",
                        isPrimary: true,
                    });
                }
            }

            if (motherName) {
                const [motherUser] = await tx
                    .insert(users)
                    .values({
                        name: motherName,
                        phone: motherPhone || null,
                        role: "parent",
                    })
                    .returning({ id: users.id });

                if (motherUser) {
                    await tx.insert(parentStudents).values({
                        parentId: motherUser.id,
                        studentId: targetUserId,
                        relationship: "mother",
                        isPrimary: !fatherName,
                    });
                }
            }

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

            await studentCache.delete(`user:v2:${targetUserId}`);
            return { id: targetUserId };
        });
    }

    await studentCache.delete(`user:v2:${targetUserId}`);
    return { id: targetUserId };
};

export const deleteStudent = async (userId: string) => {
    // Usually we delete the user, and cascade deletes student.
    // But if we delete just student profile:
    await db.delete(students).where(eq(students.userId, userId));
    await studentCache.delete(`user:v2:${userId}`);
}

export type BulkImportResult = {
  summary: {
    total: number;
    added: number;
    skipped: number;
    failed: number;
  };
  skippedStudents: {
    name: string;
    reason: string;
    existingClass: string;
    existingStudentNumber: string;
  }[];
  failedStudents: {
    rowNumber: number;
    name: string;
    error: string;
  }[];
};

export const bulkImportStudents = async (
  classId: number, 
  studentData: {
    studentNumber?: string;
    secretNumber?: string;
    name: string;
    phone?: string;
    email?: string;
    sex?: 'male' | 'female' | 'unspecified';
    metadata?: any;
    fatherName?: string;
    fatherPhone?: string;
    motherName?: string;
    motherPhone?: string;
  }[],
  academicYearId?: number
): Promise<BulkImportResult> => {
  // 1. Get academic year
  let yearId = academicYearId;
  if (!yearId) {
    const [currentYear] = await db
      .select()
      .from(academicYears)
      .where(eq(academicYears.isCurrent, true))
      .limit(1);
    if (!currentYear) throw new Error("لم يتم العثور على سنة دراسية حالية");
    yearId = currentYear.id;
  }

  const result: BulkImportResult = {
    summary: { total: studentData.length, added: 0, skipped: 0, failed: 0 },
    skippedStudents: [],
    failedStudents: [],
  };

  if (studentData.length === 0) return result;

  // 2. Pre-check for duplicates by name+sex (batch query)
  const pairs = studentData.map(s => ({
    name: s.name,
    sex: (s.sex || 'unspecified') as string,
  }));
  const existingByNameSex = await findExistingStudentsByNameAndSex(pairs);

  // Build lookup map: normalized "name|sex" → duplicate info
  const duplicateMap = new Map<string, typeof existingByNameSex[0]>();
  for (const dup of existingByNameSex) {
    const key = `${normalizeStudentName(dup.name || '')}|${(dup.sex || 'unspecified').toLowerCase()}`;
    duplicateMap.set(key, dup);
  }

  // 3. Also check for duplicates by studentNumber (if provided)
  const providedNumbers = studentData
    .map(s => s.studentNumber)
    .filter((n): n is string => !!n);

  const existingByNumber = new Map<string, { userId: string; className: string | null }>(); 
  if (providedNumbers.length > 0) {
    const existingWithNumbers = await db
      .select({
        studentNumber: students.studentNumber,
        userId: students.userId,
        className: classes.name,
      })
      .from(students)
      .leftJoin(classes, eq(students.currentClassId, classes.id))
      .where(inArray(students.studentNumber, providedNumbers));

    for (const s of existingWithNumbers) {
      existingByNumber.set(s.studentNumber, { userId: s.userId, className: s.className });
    }
  }

  // 4. Separate students: skip duplicates, collect new students
  const studentsToAdd: { item: typeof studentData[0]; originalIndex: number }[] = [];
  const seenInBatch = new Set<string>(); // Deduplicate within the same CSV

  for (let i = 0; i < studentData.length; i++) {
    const item = studentData[i]!;
    const sex = (item.sex || 'unspecified').toLowerCase();
    const nameKey = `${normalizeStudentName(item.name)}|${sex}`;

    // Check duplicate by name+sex against DB
    const dupByName = duplicateMap.get(nameKey);
    if (dupByName) {
      result.summary.skipped++;
      result.skippedStudents.push({
        name: item.name,
        reason: `مسجل بالفعل في الصف "${dupByName.className || 'غير محدد'}"`,
        existingClass: dupByName.className || 'غير محدد',
        existingStudentNumber: dupByName.studentNumber,
      });
      continue;
    }

    // Check duplicate by studentNumber against DB
    if (item.studentNumber && existingByNumber.has(item.studentNumber)) {
      const dupByNum = existingByNumber.get(item.studentNumber)!;
      result.summary.skipped++;
      result.skippedStudents.push({
        name: item.name,
        reason: `رقم الطالب "${item.studentNumber}" مستخدم بالفعل`,
        existingClass: dupByNum.className || 'غير محدد',
        existingStudentNumber: item.studentNumber,
      });
      continue;
    }

    // Check duplicate within the same CSV file
    if (seenInBatch.has(nameKey)) {
      result.summary.skipped++;
      result.skippedStudents.push({
        name: item.name,
        reason: 'مكرر داخل نفس الملف',
        existingClass: '-',
        existingStudentNumber: '-',
      });
      continue;
    }
    seenInBatch.add(nameKey);

    studentsToAdd.push({ item, originalIndex: i });
  }

  // 5. Process each new student individually (own transaction per student)
  for (const { item, originalIndex } of studentsToAdd) {
    try {
      await db.transaction(async (tx) => {
        // Create user
        const [newUser] = await tx.insert(users).values({
          name: item.name,
          phone: item.phone || null,
          email: item.email || null,
          role: 'student',
          sex: (item.sex || 'unspecified') as any,
        }).returning();

        if (!newUser) throw new Error("فشل إنشاء المستخدم");

        // Generate numbers if not provided
        const finalStudentNumber = item.studentNumber || await generateNextStudentNumber(tx);
        const finalSecretNumber = item.secretNumber || generateSecretNumber();

        // Create student record
        await tx.insert(students).values({
          userId: newUser.id,
          studentNumber: finalStudentNumber,
          secretNumber: finalSecretNumber,
          currentClassId: classId,
          metadata: item.metadata || {},
        });

        // Insert Parents if available
        if (item.fatherName) {
           const [fatherUser] = await tx
              .insert(users)
              .values({
                name: item.fatherName,
                phone: item.fatherPhone || null,
                role: "parent",
              })
              .returning({ id: users.id });
              
           if (fatherUser) {
              await tx.insert(parentStudents).values({
                 parentId: fatherUser.id,
                 studentId: newUser.id,
                 relationship: "father",
                 isPrimary: true,
              });
           }
        }

        if (item.motherName) {
           const [motherUser] = await tx
              .insert(users)
              .values({
                name: item.motherName,
                phone: item.motherPhone || null,
                role: "parent",
              })
              .returning({ id: users.id });
              
           if (motherUser) {
              await tx.insert(parentStudents).values({
                 parentId: motherUser.id,
                 studentId: newUser.id,
                 relationship: "mother",
                 isPrimary: !item.fatherName,
              });
           }
        }

        // Create enrollment
        await tx.insert(enrollments).values({
          studentId: newUser.id,
          classId,
          academicYearId: yearId!,
        }).onConflictDoNothing();
      });

      result.summary.added++;
    } catch (error) {
      result.summary.failed++;
      result.failedStudents.push({
        rowNumber: originalIndex + 1,
        name: item.name,
        error: (error as Error).message || 'خطأ غير معروف',
      });
    }
  }

  // Invalidate class cache since student counts changed
  if (result.summary.added > 0) {
    const classCache = new RedisCache("classes", 60 * 60);
    await classCache.invalidate().catch(() => {});
  }

  return result;
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

/**
 * Get data for the class students and parents export.
 */
export const getStudentsParentsData = async (classId: number) => {
  const studentsDataRaw = await db
    .select({
      studentNumber: students.studentNumber,
      secretNumber: students.secretNumber,
      studentName: users.name,
      studentEmail: users.email,
      studentSex: users.sex,
      studentStatus: students.status,
      studentPhone: users.phone,
      studentId: students.userId,
    })
    .from(students)
    .innerJoin(users, eq(students.userId, users.id))
    .where(eq(students.currentClassId, classId))
    .orderBy(asc(users.name));

  const studentIds = studentsDataRaw.map((s) => s.studentId);
  const parentsDataRaw =
    studentIds.length > 0
      ? await db
          .select({
            studentId: parentStudents.studentId,
            relationship: parentStudents.relationship,
            parentName: users.name,
            parentPhone: users.phone,
          })
          .from(parentStudents)
          .innerJoin(users, eq(parentStudents.parentId, users.id))
          .where(inArray(parentStudents.studentId, studentIds))
      : [];

  return studentsDataRaw.map((s) => {
    const p = parentsDataRaw.filter((p) => p.studentId === s.studentId);
    const father = p.find((p) => p.relationship === "father" || p.relationship === "parent");
    const mother = p.find((p) => p.relationship === "mother");
    return {
      studentNumber: s.studentNumber,
      secretNumber: s.secretNumber,
      studentName: s.studentName,
      studentEmail: s.studentEmail,
      studentSex: s.studentSex,
      studentStatus: s.studentStatus,
      phone: s.studentPhone,
      fatherName: father?.parentName || null,
      fatherPhone: father?.parentPhone || null,
      motherName: mother?.parentName || null,
      motherPhone: mother?.parentPhone || null,
      metadata: s.studentId ? undefined : undefined // placeholder if needed downstream
    };
  });
};


