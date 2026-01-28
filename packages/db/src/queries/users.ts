import type { Database } from "@db/client";
import { users } from "@db/schema";
import { userCache } from "@midday/cache";
import { eq } from "drizzle-orm";

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
  metadata?: any;
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
