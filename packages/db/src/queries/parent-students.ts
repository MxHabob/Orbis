import type { Database } from "@db/client";
import { classes, parentStudents, students, users } from "@db/schema";
import { RedisCache } from "@orbis/cache";
import { and, eq, inArray } from "drizzle-orm";

const parentStudentCache = new RedisCache("parent-student", 30 * 60);

// ==========================================
// Link parent to student(s)
// ==========================================
export type LinkParentStudentParams = {
  parentId: string;
  studentId: string;
  relationship?: string;
  isPrimary?: boolean;
};

export const linkParentToStudent = async (
  db: Database,
  data: LinkParentStudentParams,
) => {
  const [result] = await db
    .insert(parentStudents)
    .values({
      parentId: data.parentId,
      studentId: data.studentId,
      relationship: data.relationship ?? "parent",
      isPrimary: data.isPrimary ?? false,
    })
    .onConflictDoUpdate({
      target: [parentStudents.parentId, parentStudents.studentId],
      set: {
        relationship: data.relationship ?? "parent",
        isPrimary: data.isPrimary ?? false,
      },
    })
    .returning();

  // Invalidate caches for both sides
  await Promise.all([
    parentStudentCache.delete(`parent:${data.parentId}`),
    parentStudentCache.delete(`student:${data.studentId}`),
  ]);

  return result;
};

// ==========================================
// Bulk link parent to multiple students
// ==========================================
export const linkParentToStudents = async (
  db: Database,
  parentId: string,
  studentLinks: {
    studentId: string;
    relationship?: string;
    isPrimary?: boolean;
  }[],
) => {
  if (studentLinks.length === 0) return [];

  const values = studentLinks.map((link) => ({
    parentId,
    studentId: link.studentId,
    relationship: link.relationship ?? "parent",
    isPrimary: link.isPrimary ?? false,
  }));

  const results = await db
    .insert(parentStudents)
    .values(values)
    .onConflictDoNothing()
    .returning();

  // Invalidate caches
  const cacheKeys = [
    `parent:${parentId}`,
    ...studentLinks.map((l) => `student:${l.studentId}`),
  ];
  await Promise.all(cacheKeys.map((key) => parentStudentCache.delete(key)));

  return results;
};

// ==========================================
// Unlink parent from student
// ==========================================
export const unlinkParentFromStudent = async (
  db: Database,
  parentId: string,
  studentId: string,
) => {
  await db
    .delete(parentStudents)
    .where(
      and(
        eq(parentStudents.parentId, parentId),
        eq(parentStudents.studentId, studentId),
      ),
    );

  await Promise.all([
    parentStudentCache.delete(`parent:${parentId}`),
    parentStudentCache.delete(`student:${studentId}`),
  ]);
};

// ==========================================
// Get children (students) for a parent
// ==========================================
export const getChildrenByParentId = async (db: Database, parentId: string) => {
  const cacheKey = `parent:${parentId}`;
  const cached = await parentStudentCache.get<any[]>(cacheKey);
  if (cached) return cached;

  const result = await db
    .select({
      linkId: parentStudents.id,
      relationship: parentStudents.relationship,
      isPrimary: parentStudents.isPrimary,
      studentId: students.userId,
      studentNumber: students.studentNumber,
      status: students.status,
      name: users.name,
      email: users.email,
      image: users.image,
      sex: users.sex,
      currentClass: {
        id: classes.id,
        name: classes.name,
      },
    })
    .from(parentStudents)
    .innerJoin(students, eq(parentStudents.studentId, students.userId))
    .innerJoin(users, eq(students.userId, users.id))
    .leftJoin(classes, eq(students.currentClassId, classes.id))
    .where(eq(parentStudents.parentId, parentId));

  await parentStudentCache.set(cacheKey, result);
  return result;
};

// ==========================================
// Get parents for a student
// ==========================================
export const getParentsByStudentId = async (
  db: Database,
  studentId: string,
) => {
  const cacheKey = `student:${studentId}`;
  const cached = await parentStudentCache.get<any[]>(cacheKey);
  if (cached) return cached;

  const result = await db
    .select({
      linkId: parentStudents.id,
      relationship: parentStudents.relationship,
      isPrimary: parentStudents.isPrimary,
      parentId: users.id,
      name: users.name,
      email: users.email,
      phone: users.phone,
      image: users.image,
    })
    .from(parentStudents)
    .innerJoin(users, eq(parentStudents.parentId, users.id))
    .where(eq(parentStudents.studentId, studentId));

  await parentStudentCache.set(cacheKey, result);
  return result;
};

// ==========================================
// Update relationship details
// ==========================================
export const updateParentStudentLink = async (
  db: Database,
  parentId: string,
  studentId: string,
  data: { relationship?: string; isPrimary?: boolean },
) => {
  const [result] = await db
    .update(parentStudents)
    .set(data)
    .where(
      and(
        eq(parentStudents.parentId, parentId),
        eq(parentStudents.studentId, studentId),
      ),
    )
    .returning();

  await Promise.all([
    parentStudentCache.delete(`parent:${parentId}`),
    parentStudentCache.delete(`student:${studentId}`),
  ]);

  return result;
};
