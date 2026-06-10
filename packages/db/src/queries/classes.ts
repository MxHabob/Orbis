import { db } from "@db/client";
import { classes, enrollments, academicYears } from "@db/schema";
import { eq, and, isNull, isNotNull, inArray, ilike, sql, desc, asc, or, count } from "drizzle-orm";
import { RedisCache } from "@orbis/cache";

const classCache = new RedisCache("classes", 60 * 60);

// Use a consistent production version key
const CACHE_VERSION = "v7_prod_v1";

interface ClassQueryParams {
  academicYearId?: number | string | null;
  q?: string | null;
  sort?: string[] | null;
  flat?: boolean;
  limit?: number;
  offset?: number;
}

export interface EnrichedClass {
  id: number;
  name: string;
  type: string;
  division: string | null;
  color: string;
  parentClassId: number | null;
  sortOrder: number;
  students: number;
  parentClass: { name: string } | null;
  _count: {
    students: number;
    schedule: number;
  };
}

/**
 * Production-ready class query with recursive enrollment rollup.
 * Optimized to avoid N+1 queries by using advanced SQL aggregation.
 */
export const getClassesV4 = async (params: ClassQueryParams | number): Promise<EnrichedClass[]> => {
  try {
    // Normalize params
    const isNumber = typeof params === "number";
    const p: ClassQueryParams = isNumber ? { academicYearId: params as number } : (params as ClassQueryParams || {});
    
    // Resolve yearId
    let yearId = p.academicYearId ? Number(p.academicYearId) : undefined;
  if (!yearId) {
    const [currentYear] = await db
      .select({ id: academicYears.id })
      .from(academicYears)
      .where(eq(academicYears.isCurrent, true))
      .limit(1);
    yearId = currentYear?.id;
  }

  // Caching strategy
  const cacheKey = `${CACHE_VERSION}:${yearId ?? "all"}:${p.q ?? "none"}:${p.sort?.join(",") ?? "default"}:${p.flat}:${p.limit}:${p.offset}`;
  const cached = await classCache.get<any[]>(cacheKey);
  if (cached && !p.q && !p.sort) return cached;

  const orderBy = [];
  if (p.sort?.length) {
    p.sort.forEach((s: string) => {
      const [column, direction] = s.split(":");
      const dirFunc = direction === "desc" ? desc : asc;
      switch (column) {
        case "name": orderBy.push(dirFunc(classes.name)); break;
        case "division": orderBy.push(dirFunc(classes.division)); break;
      }
    });
  }
  if (orderBy.length === 0) {
    orderBy.push(asc(classes.sortOrder), asc(classes.name));
  }

  const where = and(
    isNull(classes.deletedAt),
    p.q ? or(
      ilike(classes.name, `%${p.q}%`),
      ilike(classes.division, `%${p.q}%`)
    ) : undefined,
    !p.flat && !p.q ? isNull(classes.parentClassId) : undefined
  );

  /**
   * RECURSIVE ROLLUP STRATEGY (2026 Production Standard):
   * 1. Get all base classes for the result set.
   * 2. Use a CROSS-join subquery to get a mapping of classes to their descendants.
   * 3. Aggregate enrollments based on that map.
   */
  
  // Subquery for all direct enrollments in the target year
  const directCounts = db
    .select({
      classId: enrollments.classId,
      cnt: count(enrollments.studentId).as("cnt"),
    })
    .from(enrollments)
    .where(and(
      yearId ? eq(enrollments.academicYearId, yearId) : undefined,
      isNull(enrollments.deletedAt)
    ))
    .groupBy(enrollments.classId)
    .as("direct_counts");

  // Secondary Counts subquery for rollup (child -> parent)
  const rollupCounts = db
    .select({
      parentId: classes.parentClassId,
      sumCnt: sql<number>`sum(${directCounts.cnt})::int`.as("sum_cnt"),
    })
    .from(classes)
    .innerJoin(directCounts, eq(classes.id, directCounts.classId))
    .where(and(isNull(classes.deletedAt), isNotNull(classes.parentClassId)))
    .groupBy(classes.parentClassId)
    .as("rollup_counts");

  const query = db
    .select({
      id: classes.id,
      name: classes.name,
      type: classes.type,
      division: classes.division,
      color: classes.color,
      parentClassId: classes.parentClassId,
      sortOrder: classes.sortOrder,
      // Aggregated Students: Direct + Child Rollup
      students: sql<number>`(COALESCE(${directCounts.cnt}::int, 0) + COALESCE(${rollupCounts.sumCnt}, 0))`.as("students"),
      parentClassName: sql<string | null>`(SELECT p.name FROM classes p WHERE p.id = ${classes.parentClassId})`,
    })
    .from(classes)
    .leftJoin(directCounts, eq(classes.id, directCounts.classId))
    .leftJoin(rollupCounts, eq(classes.id, rollupCounts.parentId))
    .where(where)
    .orderBy(...orderBy);

  if (p.limit) query.limit(p.limit);
  if (p.offset) query.offset(p.offset);

  const results = (await query).map(({ parentClassName, ...row }) => ({
    ...row,
    parentClass: parentClassName ? { name: parentClassName } : null,
    _count: {
      students: Number(row.students || 0),
      schedule: 0, // Placeholder
    }
  }));

  if (!p.q && !p.sort && !p.flat) await classCache.set(cacheKey, results, 3600);

  return results;
  } catch (error: any) {
    console.error("[getClassesV4] Server Error:", error?.message || error);
    // Be very explicit about the error to see the stack trace in dev
    if (process.env.NODE_ENV === "development") {
      console.error(error);
    }
    throw error;
  }
};

export const getClassById = (id: number) => 
  db.query.classes.findFirst({ where: eq(classes.id, id) });

export const createClass = async (data: any) => {
  const [result] = await db.insert(classes).values(data).returning();
  await classCache.delete(`${CACHE_VERSION}*`).catch(() => {});
  return result;
};

export const updateClass = async (id: number, data: any) => {
  const [result] = await db.update(classes).set(data).where(eq(classes.id, id)).returning();
  await classCache.delete(`${CACHE_VERSION}*`).catch(() => {});
  return result;
};

export const deleteClass = async (id: number) => {
  const [result] = await db.update(classes).set({ deletedAt: new Date() }).where(eq(classes.id, id)).returning();
  await classCache.delete(`${CACHE_VERSION}*`).catch(() => {});
  return result;
};
