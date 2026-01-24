import type { Database } from "@db/client";
import {
  parents,
  users,
  students,
} from "@db/schema";
import { and, asc, desc, eq, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetParentByIdParams = {
  userId: string;
};

export const getParentById = async (
  db: Database,
  params: GetParentByIdParams,
) => {
  const [result] = await db
    .select({
      userId: parents.userId,
      surname: parents.surname,
      phone: parents.phone,
      address: parents.address,
      name: users.name,
      email: users.email,
      image: users.image,
      role: users.role,
      createdAt: users.createdAt,
      childrenCount: sql<number>`cast(count(${students.userId}) as int)`,
    })
    .from(parents)
    .where(eq(parents.userId, params.userId))
    .innerJoin(users, eq(users.id, parents.userId))
    .leftJoin(students, eq(students.parentId, parents.userId))
    .groupBy(parents.userId, users.id);

  return result;
};

export type GetParentsParams = {
  cursor?: string | null;
  pageSize?: number;
  q?: string | null;
  sort?: string[] | null;
};

export const getParents = async (
  db: Database,
  params: GetParentsParams,
) => {
  const { sort, cursor, pageSize = 25, q } = params;

  const whereConditions: SQL[] = [];

  // Apply search query filter
  if (q) {
    whereConditions.push(
      sql`(${users.name} ILIKE '%' || ${q} || '%' OR ${parents.surname} ILIKE '%' || ${q} || '%' OR ${parents.phone} ILIKE '%' || ${q} || '%')`,
    );
  }

  const query = db
    .select({
      userId: parents.userId,
      surname: parents.surname,
      phone: parents.phone,
      address: parents.address,
      name: users.name,
      email: users.email,
      image: users.image,
      createdAt: users.createdAt,
      childrenCount: sql<number>`cast(count(${students.userId}) as int)`,
    })
    .from(parents)
    .innerJoin(users, eq(users.id, parents.userId))
    .leftJoin(students, eq(students.parentId, parents.userId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(parents.userId, users.id);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "name") {
      isAscending
        ? query.orderBy(asc(users.name))
        : query.orderBy(desc(users.name));
    } else if (column === "surname") {
      isAscending
        ? query.orderBy(asc(parents.surname))
        : query.orderBy(desc(parents.surname));
    } else if (column === "phone") {
      isAscending
        ? query.orderBy(asc(parents.phone))
        : query.orderBy(desc(parents.phone));
    } else if (column === "created_at") {
      isAscending
        ? query.orderBy(asc(users.createdAt))
        : query.orderBy(desc(users.createdAt));
    }
  } else {
    // Default sort by name ascending
    query.orderBy(asc(users.name));
  }

  // Apply pagination
  const offset = cursor ? Number.parseInt(cursor, 10) : 0;
  query.limit(pageSize).offset(offset);

  const data = await query;

  const nextCursor =
    data && data.length === pageSize
      ? (offset + pageSize).toString()
      : undefined;

  return {
    meta: {
      cursor: nextCursor ?? null,
      hasPreviousPage: offset > 0,
      hasNextPage: data && data.length === pageSize,
    },
    data,
  };
};

export type CreateParentParams = {
  userId?: string;
  surname: string;
  phone: string;
  address?: string | null;
  // User fields
  name?: string | null;
  email?: string | null;
  image?: string | null;
};

export const createParent = async (
  db: Database,
  params: CreateParentParams,
) => {
  const { userId, name, email, image, surname, phone, address } = params;

  // Create user first
  const [user] = await db
    .insert(users)
    .values({
      id: userId,
      name,
      email,
      image,
      role: "parent",
    })
    .returning();

  // Create parent
  const [parent] = await db
    .insert(parents)
    .values({
      userId: user.id,
      surname,
      phone,
      address,
    })
    .returning();

  return { ...parent, name: user.name, email: user.email, image: user.image };
};

export type UpdateParentParams = {
  userId: string;
  surname?: string;
  phone?: string;
  address?: string | null;
  // User fields
  name?: string | null;
  email?: string | null;
  image?: string | null;
};

export const updateParent = async (
  db: Database,
  params: UpdateParentParams,
) => {
  const { userId, name, email, image, ...parentData } = params;

  // Update user if user fields provided
  if (name !== undefined || email !== undefined || image !== undefined) {
    await db
      .update(users)
      .set({
        name,
        email,
        image,
        updatedAt: new Date(),
      })
      .where(eq(users.id, userId));
  }

  // Update parent
  const [result] = await db
    .update(parents)
    .set(parentData)
    .where(eq(parents.userId, userId))
    .returning();

  return result;
};

export type DeleteParentParams = {
  userId: string;
};

export const deleteParent = async (
  db: Database,
  params: DeleteParentParams,
) => {
  const { userId } = params;

  // Delete user (cascade will delete parent)
  const [result] = await db
    .delete(users)
    .where(eq(users.id, userId))
    .returning();

  return result;
};

// Get parent's children
export type GetParentChildrenParams = {
  parentId: string;
};

export const getParentChildren = async (
  db: Database,
  params: GetParentChildrenParams,
) => {
  const { parentId } = params;

  const data = await db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      surname: students.surname,
      sex: students.sex,
      birthday: students.birthday,
      status: students.status,
      name: users.name,
      email: users.email,
      image: users.image,
    })
    .from(students)
    .where(eq(students.parentId, parentId))
    .innerJoin(users, eq(users.id, students.userId))
    .orderBy(asc(students.studentNumber));

  return data;
};

// Get parent by phone
export type GetParentByPhoneParams = {
  phone: string;
};

export const getParentByPhone = async (
  db: Database,
  params: GetParentByPhoneParams,
) => {
  const [result] = await db
    .select({
      userId: parents.userId,
      surname: parents.surname,
      phone: parents.phone,
      address: parents.address,
      name: users.name,
      email: users.email,
    })
    .from(parents)
    .where(eq(parents.phone, params.phone))
    .innerJoin(users, eq(users.id, parents.userId))
    .limit(1);

  return result;
};
