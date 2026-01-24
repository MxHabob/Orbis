import type { Database } from "@db/client";
import { employeeRoles, users } from "@db/schema";
import { eq } from "drizzle-orm";

export const getUserWithRoles = async (db: Database, userId: string) => {
  const user = await db.query.users.findFirst({
    where: eq(users.id, userId),
    columns: {
      id: true,
      role: true,
      email: true,
      name: true,
    },
  });

  if (!user) return null;

  const roles = await db
    .select({ role: employeeRoles.role })
    .from(employeeRoles)
    .where(eq(employeeRoles.employeeId, userId));

  return {
    ...user,
    employeeRoles: roles.map((r) => r.role),
  };
};

export type UpdateUserParams = {
  id: string;
  name?: string | null;
  image?: string | null;
  // Add other fields from schema if needed (locale, timezone etc)
};

export const updateUser = async (db: Database, params: UpdateUserParams) => {
  const { id, ...data } = params;

  // Filter undefined values
  const updateData: Record<string, any> = {};
  if (data.name !== undefined) updateData.name = data.name;
  if (data.image !== undefined) updateData.image = data.image;

  if (Object.keys(updateData).length === 0) return null;

  const [updatedUser] = await db
    .update(users)
    .set({
      ...updateData,
      updatedAt: new Date(),
    })
    .where(eq(users.id, id))
    .returning();

  return updatedUser;
};
