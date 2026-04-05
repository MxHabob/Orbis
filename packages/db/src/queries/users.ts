import type { Database } from "@db/client";
import { users } from "@db/schema";
import { userCache } from "@midday/cache";
import { eq, or } from "drizzle-orm";

export const getStaffUsers = async (db: Database) => {
  return db.select().from(users).where(or(eq(users.role, "staff"), eq(users.role, "teacher")));
};

export const getParentUsers = async (db: Database) => {
  return db.select().from(users).where(eq(users.role, "parent"));
};

export const getUserById = async (db: Database, id: string) => {
  const cached = await userCache.get(id);
  if (cached) return cached;

  const [result] = await db
    .select() // Select all fields
    .from(users)
    .where(eq(users.id, id));

  if (result) {
    await userCache.setById(id, result);
  }

  return result;
};

export type UpdateUserParams = {
  id: string;
  name?: string | null;
  email?: string | null;
  image?: string | null;
  phone?: string | null;
  locale?: string;
  metadata?: any;
  role?: "admin" | "student" | "parent" | "teacher" | "staff";
  timezone?: string | null;
  timeFormat?: number;
  weekStartsOnMonday?: boolean;
};

export const updateUser = async (db: Database, data: UpdateUserParams) => {
  const { id, ...updateData } = data;

  const [result] = await db
    .update(users)
    .set(updateData)
    .where(eq(users.id, id))
    .returning();
  
  // Invalidate auth user cache
  await userCache.invalidate(id);

  return result;
};

export const deleteUser = async (db: Database, id: string) => {
  await db.delete(users).where(eq(users.id, id));
  await userCache.invalidate(id);
  return { id };
};
export const getUserWithRoles = async (db: Database, id: string) => {
  const result = await getUserById(db, id);

  if (!result) return null;

  return {
    ...result,
    employeeRoles: [],
  };
};
export const createUsers = async (db: Database, data: (typeof users.$inferInsert)[]) => {
  return db
    .insert(users)
    .values(data)
    .onConflictDoNothing()
    .returning();
};