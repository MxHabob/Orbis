import { db } from "@db/client";
import { classes, academicYears } from "@db/schema";
import { eq, desc } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const classCache = new RedisCache("classes", 60 * 60); // 1 hour

export const getClasses = async (academicYearId?: number) => {
  const cacheKey = `list:${academicYearId || 'all'}`;
  const cached = await classCache.get(cacheKey);
  
  if (cached) return cached;

  let query = db
    .select({
        id: classes.id,
        name: classes.name,
        division: classes.division,
        year: {
            name: academicYears.yearName,
            isCurrent: academicYears.isCurrent
        }
    })
    .from(classes)
    .leftJoin(academicYears, eq(classes.academicYearId, academicYears.id));

  if (academicYearId) {
    query.where(eq(classes.academicYearId, academicYearId));
  }

  const results = await query;
  
  if (results) {
    await classCache.set(cacheKey, results);
  }
  return results;
};

export const getClassById = async (id: number) => {
    const cached = await classCache.get(`id:${id}`);
    if (cached) return cached;

    const [result] = await db
        .select()
        .from(classes)
        .where(eq(classes.id, id));
    
    if (result) {
        await classCache.set(`id:${id}`, result);
    }
    return result;
};
