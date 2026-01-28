import { db } from "@db/client";
import { classes, academicYears } from "@db/schema";
import { eq } from "drizzle-orm";
import { RedisCache } from "@midday/cache";
import { getCurrentAcademicYear } from "./academic-calendar";

const classCache = new RedisCache("classes", 60 * 60);

export const getClasses = async (academicYearId?: number) => {
  let targetYearId = academicYearId;

  if (!targetYearId) {
    const currentYear = await getCurrentAcademicYear();
    targetYearId = currentYear?.id;
  }

  const cacheKey = `list:${targetYearId || "all"}`;
  const cached = await classCache.get<any[]>(cacheKey);
  if (cached) return cached;

  const results = await db.query.classes.findMany({
    where: targetYearId 
      ? eq(classes.academicYearId, targetYearId) 
      : undefined,

    columns: {
      id: true,
      name: true,
      division: true,
      type: true,
      color: true,
      academicYearId: true,
      parentClassId: true,
    },

    with: {
      year: {
        columns: {
          yearName: true,
          isCurrent: true,
        },
      },
      children: {
        columns: {
          id: true,
          name: true,
          division: true,
          type: true,
          color: true,
          academicYearId: true,
        },
        orderBy: (fields, { asc }) => [asc(fields.division)],
      },
    },

    orderBy: (fields, { asc }) => [asc(fields.name)],
  });


  await classCache.set(cacheKey, results);

  return results;
};

export const getClassById = async (id: number) => {
    const cached = await classCache.get<typeof classes.$inferSelect>(`id:${id}`);
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

export type CreateClassParams = typeof classes.$inferInsert;
export const createClass = async (data: CreateClassParams) => {
    const [result] = await db.insert(classes).values(data).returning();
    await classCache.delete(`list:${data.academicYearId || 'all'}`);
    await classCache.delete(`list:all`);
    return result;
};

export const updateClass = async (id: number, data: Partial<CreateClassParams>) => {
    const [result] = await db.update(classes).set(data).where(eq(classes.id, id)).returning();
    // Broad invalidation for simplicity
    await classCache.delete(`list:all`); 
    if (result) await classCache.delete(`list:${result.academicYearId}`);
    await classCache.delete(`id:${id}`);
    return result;
};

export const deleteClass = async (id: number) => {
    const [existing] = await db.select().from(classes).where(eq(classes.id, id));
    await db.delete(classes).where(eq(classes.id, id));
    if (existing) {
         await classCache.delete(`list:${existing.academicYearId}`);
    }
    await classCache.delete(`list:all`);
    await classCache.delete(`id:${id}`);
};
