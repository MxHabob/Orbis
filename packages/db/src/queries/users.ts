import type { Database } from "@db/client";
import { employeeRoles, users } from "@db/schema";
import { eq, inArray } from "drizzle-orm";

export const getAllUsersWithRoles = async (db: Database, _session?: any) => {
  const allUsers = await db.query.users.findMany({
    columns: {
      id: true,
      role: true,
      email: true,
      name: true,
      image: true,
      createdAt: true,
    },
    orderBy: (users, { desc }) => [desc(users.createdAt)],
  });

  if (allUsers.length === 0) {
    return [];
  }

  const userIds = allUsers.map((user) => user.id);

  const allRoles = await db
    .select({
      employeeId: employeeRoles.employeeId,
      role: employeeRoles.role,
    })
    .from(employeeRoles)
    .where(inArray(employeeRoles.employeeId, userIds));

  const rolesMap = allRoles.reduce((acc, row) => {
    const roles = acc[row.employeeId] || [];
    roles.push(row.role);
    acc[row.employeeId] = roles;
    return acc;
  }, {} as Record<string, string[]>);

  return allUsers.map((user) => ({
    ...user,
    employeeRoles: rolesMap[user.id] || [],
  }));
};

export const getUserWithRoles = async (db: Database, userId: string) => {
  const user = await db.query.users.findFirst({
    where: eq(users.id, userId),
    columns: {
      id: true,
      role: true,
      email: true,
      name: true,
      image: true,
      createdAt: true,
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

export type CreateUserParams = {
  name: string;
  email: string;
  role: string;
  image?: string;
};

export const createUser = async (db: Database, params: CreateUserParams) => {
  const { name, email, role, image } = params;

  const [createdUser] = await db
    .insert(users)
    .values({
      name,
      email,
      role: role as any,
      image,
      updatedAt: new Date(),
    })
    .returning();

  return createdUser;
};

export const createManyUsers = async (db: Database, params: CreateUserParams[]) => {
  if (params.length === 0) return [];

  return db.insert(users).values(
    params.map((user) => ({
      name: user.name,
      email: user.email,
      role: user.role as any,
      image: user.image,
      updatedAt: new Date(),
    }))
  ).returning();
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
