import { db } from "@db/client";
import { subjects } from "@db/schema";
import { eq, asc } from "drizzle-orm";
import { RedisCache } from "@orbis/cache";

const subjectCache = new RedisCache("subjects", 60 * 60 * 24); // 24 hours

export const getAllSubjects = async () => {
  const cacheKey = "all";
  const cached = await subjectCache.get<Array<typeof subjects.$inferSelect>>(cacheKey);
  
  if (cached) return cached;

  const results = await db.select().from(subjects).orderBy(asc(subjects.sortOrder), asc(subjects.id));
  
  if (results) {
    await subjectCache.set(cacheKey, results);
  }
  return results;
};

export const getSubjectById = async (id: number) => {
    const cached = await subjectCache.get<typeof subjects.$inferSelect>(`id:${id}`);
    if (cached !== undefined) return cached;

    const [result] = await db.select().from(subjects).where(eq(subjects.id, id));

    if (result) {
        await subjectCache.set(`id:${id}`, result);
    }
    return result ?? null;
}

export type CreateSubjectParams = typeof subjects.$inferInsert;
export const createSubject = async (data: CreateSubjectParams) => {
    const [result] = await db.insert(subjects).values(data).returning();
    await subjectCache.delete("all");
    return result;
};

export const updateSubject = async (id: number, data: Partial<CreateSubjectParams>) => {
    const [result] = await db.update(subjects).set(data).where(eq(subjects.id, id)).returning();
    await subjectCache.delete("all");
    await subjectCache.delete(`id:${id}`);
    return result;
};

export const deleteSubject = async (id: number) => {
    await db.delete(subjects).where(eq(subjects.id, id));
    await subjectCache.delete("all");
    await subjectCache.delete(`id:${id}`);
};
