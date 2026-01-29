import { db } from "@db/client";
import { academicYears, classes } from "@db/schema";
import { eq, and, isNull, inArray } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const classCache = new RedisCache("classes", 60 * 60);

export const getClassesV3 = async (academicYearId?: number) => {
  const cacheKey = `root_classes_with_children:${academicYearId ?? "all"}`;
  const cached = await classCache.get<any[]>(cacheKey);
  if (cached) return cached;

  // Fetch root classes + year name directly
  const roots = await db
    .select({
      id: classes.id,
      name: classes.name,
      type: classes.type,
      division: classes.division,
      color: classes.color,
      academicYearId: classes.academicYearId,
      parentClassId: classes.parentClassId,
      yearName: academicYears.yearName,  // ← flattened
    })
    .from(classes)
    .leftJoin(academicYears, eq(classes.academicYearId, academicYears.id))
    .where(
      and(
        isNull(classes.deletedAt),
        isNull(classes.parentClassId),
        academicYearId ? eq(classes.academicYearId, academicYearId) : undefined
      )
    )
    .orderBy(classes.id, classes.name);

  if (!roots.length) {
    await classCache.set(cacheKey, [], 3600);
    return [];
  }

  // Fetch all children in one go
  const children = await db
    .select({
      id: classes.id,
      name: classes.name,
      type: classes.type,
      division: classes.division,
      color: classes.color,
      academicYearId: classes.academicYearId,
      parentClassId: classes.parentClassId,
    })
    .from(classes)
    .where(
      and(
        isNull(classes.deletedAt),
        inArray(classes.parentClassId, roots.map(r => r.id))
      )
    );

  // Group children by parentId
  const childrenMap = new Map<number, typeof children>();
  for (const child of children) {
    const pid = child.parentClassId!;
    if (!childrenMap.has(pid)) childrenMap.set(pid, []);
    childrenMap.get(pid)!.push(child);
  }

  // Combine
  const formatted = roots.map(cls => ({
    ...cls,
    yearName: cls.yearName ?? null,
    children: childrenMap.get(cls.id) ?? [],
  }));

  await classCache.set(cacheKey, formatted, 3600);
  return formatted;
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
