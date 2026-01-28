import { db } from "@db/client";
import { subjects } from "@db/schema";
import { eq } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const subjectCache = new RedisCache("subjects", 60 * 60 * 24); // 24 hours

export const getAllSubjects = async () => {
  const cacheKey = "all";
  const cached = await subjectCache.get(cacheKey);
  
  if (cached) return cached;

  const results = await db.select().from(subjects);
  
  if (results) {
    await subjectCache.set(cacheKey, results);
  }
  return results;
};

export const getSubjectById = async (id: number) => {
    const cached = await subjectCache.get(`id:${id}`);
    if (cached) return cached;

    const [result] = await db.select().from(subjects).where(eq(subjects.id, id));

    if (result) {
        await subjectCache.set(`id:${id}`, result);
    }
    return result;
}
