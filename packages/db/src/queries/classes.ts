import { db } from "@db/client";
import { classes, enrollments, academicYears } from "@db/schema";
import { eq, and, isNull, inArray, ilike, sql, desc, asc, or } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const classCache = new RedisCache("classes", 60 * 60);

export const getClassesV3 = async (
  params?: number | {
    academicYearId?: number | string | null;
    q?: string | null;
    sort?: string[] | null;
    flat?: boolean;
    limit?: number;
    offset?: number;
  }
) => {
  // academicYearId fallback to current if not provided
  let yearId = typeof params === 'number' 
    ? params 
    : (params?.academicYearId ? Number(params.academicYearId) : undefined);

  if (!yearId && typeof params === 'object') {
    const [currentYear] = await db
      .select({ id: academicYears.id })
      .from(academicYears)
      .where(eq(academicYears.isCurrent, true))
      .limit(1);
    
    yearId = currentYear?.id;
  }

  const q = typeof params === 'object' ? params?.q : undefined;
  const sort = typeof params === 'object' ? params?.sort : undefined;
  const flat = typeof params === 'object' ? params?.flat : false;
  const limit = typeof params === 'object' ? params?.limit : undefined;
  const offset = typeof params === 'object' ? params?.offset : 0;

  const cacheKey = `classes_v3:${yearId ?? "all"}:${q ?? "none"}:${sort?.join(",") ?? "default"}:${flat}:${limit}:${offset}`;
  const cached = await classCache.get<any[]>(cacheKey);
  if (cached && !q && !sort) return cached; // Only use cache for default view

  const orderBy = [];
  if (sort?.length) {
    sort.forEach((s) => {
      const [column, direction] = s.split(":");
      const dirFunc = direction === "desc" ? desc : asc;
      switch (column) {
        case "name": orderBy.push(dirFunc(classes.name)); break;
        case "type": orderBy.push(dirFunc(classes.type)); break;
        case "division": orderBy.push(dirFunc(classes.division)); break;
      }
    });
  }
  if (orderBy.length === 0) {
    orderBy.push(asc(classes.sortOrder));
    orderBy.push(asc(classes.name));
    orderBy.push(asc(classes.division));
  }

  // Build student count subquery — MUST be scoped by academic year to avoid double counting across years
  // Using DISTINCT for double safety, though enrollments should be unique
  const studentCountSql = yearId
    ? sql<number>`(SELECT count(DISTINCT e.student_id) FROM enrollments e WHERE e.class_id = ${classes.id} AND e.deleted_at IS NULL AND e.status = 'active' AND e.academic_year_id = ${yearId})::int`
    : sql<number>`(SELECT count(DISTINCT e.student_id) FROM enrollments e WHERE e.class_id = ${classes.id} AND e.deleted_at IS NULL AND e.status = 'active')::int`;

  const where = and(
    isNull(classes.deletedAt),
    q ? or(
      ilike(classes.name, `%${q}%`),
      ilike(classes.division, `%${q}%`)
    ) : undefined,
    !flat && !q ? isNull(classes.parentClassId) : undefined
  );

  const rootsQuery = db
    .select({
      id: classes.id,
      name: classes.name,
      type: classes.type,
      division: classes.division,
      color: classes.color,
      parentClassId: classes.parentClassId,
      sortOrder: classes.sortOrder,
      studentCount: studentCountSql,
      scheduleCount: sql<number>`(SELECT count(*) FROM scheduled_lessons l WHERE l.class_id = ${classes.id})::int`,
      parentClassName: sql<string | null>`(SELECT name FROM classes p WHERE p.id = ${classes.parentClassId})`,
    })
    .from(classes)
    .where(where)
    .orderBy(...orderBy);

  if (limit) {
    rootsQuery.limit(limit);
  }
  if (offset) {
    rootsQuery.offset(offset);
  }

  const rawRoots = await rootsQuery;
  const roots = rawRoots.map(row => ({
    id: row.id,
    name: row.name,
    type: row.type,
    division: row.division,
    color: row.color,
    parentClassId: row.parentClassId,
    sortOrder: row.sortOrder,
    _count: {
      students: row.studentCount,
      schedule: row.scheduleCount,
    },
    parentClass: {
      name: row.parentClassName,
    }
  }));

  if (flat || q || !roots.length) {
    if (!q && !sort && !flat) await classCache.set(cacheKey, roots, 3600);
    return roots;
  }

  // Fetch all children for roots in one go (only for tree view without search)
  const childStudentCountSql = yearId
    ? sql<number>`(SELECT count(DISTINCT e.student_id) FROM enrollments e WHERE e.class_id = ${classes.id} AND e.deleted_at IS NULL AND e.status = 'active' AND e.academic_year_id = ${yearId})::int`
    : sql<number>`(SELECT count(DISTINCT e.student_id) FROM enrollments e WHERE e.class_id = ${classes.id} AND e.deleted_at IS NULL AND e.status = 'active')::int`;

  const rawChildren = await db
    .select({
      id: classes.id,
      name: classes.name,
      type: classes.type,
      division: classes.division,
      color: classes.color,
      parentClassId: classes.parentClassId,
      sortOrder: classes.sortOrder,
      studentCount: childStudentCountSql,
      scheduleCount: sql<number>`(SELECT count(*) FROM scheduled_lessons l WHERE l.class_id = ${classes.id})::int`,
    })
    .from(classes)
    .where(
      and(
        isNull(classes.deletedAt),
        inArray(classes.parentClassId, roots.map(r => r.id))
      )
    )
    .orderBy(asc(classes.sortOrder), asc(classes.name));

  const children = rawChildren.map(row => ({
    ...row,
    _count: {
      students: row.studentCount,
      schedule: row.scheduleCount,
    }
  }));

  // Group children by parentId
  const childrenMap = new Map<number, any[]>();
  for (const child of children) {
    const pid = child.parentClassId!;
    if (!childrenMap.has(pid)) childrenMap.set(pid, []);
    childrenMap.get(pid)!.push(child);
  }

  // Combine
  const formatted = roots.map(cls => ({
    ...cls,
    children: childrenMap.get(cls.id) ?? [],
  }));

  if (!q && !sort && !flat) await classCache.set(cacheKey, formatted, 3600);
  return formatted;
};

export const getClassById = async (id: number) => {
  const cacheKey = `id:${id}`;
  const cached = await classCache.get<typeof classes.$inferSelect>(cacheKey);
  if (cached !== undefined) return cached;

  const [result] = await db
    .select()
    .from(classes)
    .where(and(eq(classes.id, id), isNull(classes.deletedAt)))
    .limit(1);

  if (result) {
    await classCache.set(cacheKey, result);
  }
  return result ?? null;
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
    // Clear all class caches since tree structure may have changed
    await classCache.delete(`classes_v3:all:none:default:false:undefined:0`).catch(() => {});
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
    await classCache.delete(`classes_v3:all:none:default:false:undefined:0`).catch(() => {});
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
    await classCache.delete(`classes_v3:all:none:default:false:undefined:0`).catch(() => {});
    if (result.parentClassId) {
        await classCache.delete(`parent:${result.parentClassId}`);
    }
  }
  return result;
};
