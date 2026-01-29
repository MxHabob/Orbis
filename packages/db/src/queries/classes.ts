import { db } from "@db/client";
import { classes } from "@db/schema";
import { eq, and, isNull } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const classCache = new RedisCache("classes", 60 * 60);

export const getClasses = async (yearId?: number) => {
  const cacheKey = `tree:year:${yearId || 'all'}`;
  const cached = await classCache.get<any[]>(cacheKey);
  if (cached) return cached;

  const results = await db.query.classes.findMany({
    where: (classes, { and, eq, isNull }) => {
        const conditions = [isNull(classes.deletedAt), isNull(classes.parentClassId)];
        if (yearId) {
            conditions.push(eq(classes.academicYearId, yearId));
        }
        return and(...conditions);
    },
    with: {
        children: {
            where: (classes, { isNull }) => isNull(classes.deletedAt),
        }
    }
  });

  if (results) {
    await classCache.set(cacheKey, results);
  }
  return results;
};

export const getClassById = async (id: number) => {
  const cacheKey = `id:${id}`;
  const cached = await classCache.get<typeof classes.$inferSelect>(cacheKey);
  if (cached) return cached;

  const [result] = await db
    .select()
    .from(classes)
    .where(and(eq(classes.id, id), isNull(classes.deletedAt)))
    .limit(1);

  if (result) {
    await classCache.set(cacheKey, result);
  }
  return result;
};

export const getSubClasses = async (parentId: number) => {
    const cacheKey = `parent:${parentId}`;
    const cached = await classCache.get<typeof classes.$inferSelect[]>(cacheKey);
    if (cached) return cached;

    const results = await db
        .select()
        .from(classes)
        .where(and(eq(classes.parentClassId, parentId), isNull(classes.deletedAt)));
    
    if (results) {
        await classCache.set(cacheKey, results);
    }
    return results;
}

export type CreateClassParams = typeof classes.$inferInsert;
export const createClass = async (data: CreateClassParams) => {
  const [result] = await db.insert(classes).values(data).returning();
  if (result) {
    await classCache.delete(`year:${result.academicYearId}`);
    if (result.parentClassId) {
        await classCache.delete(`parent:${result.parentClassId}`);
    }
  }
  return result;
};

export const updateClass = async (id: number, data: Partial<CreateClassParams>) => {
  const [result] = await db.update(classes).set(data).where(eq(classes.id, id)).returning();
  if (result) {
    await classCache.delete(`id:${id}`);
    await classCache.delete(`year:${result.academicYearId}`);
    if (result.parentClassId) {
        await classCache.delete(`parent:${result.parentClassId}`);
    }
  }
  return result;
};

export const deleteClass = async (id: number) => {
  const [result] = await db.update(classes).set({ deletedAt: new Date() }).where(eq(classes.id, id)).returning();
  if (result) {
    await classCache.delete(`id:${id}`);
    await classCache.delete(`year:${result.academicYearId}`);
    if (result.parentClassId) {
        await classCache.delete(`parent:${result.parentClassId}`);
    }
  }
  return result;
};
