import { db } from "@db/client";
import { academicYears, academicTerms } from "@db/schema";
import { eq, desc } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const calendarCache = new RedisCache("calendar", 60 * 60 * 24); // 24 hours

export const getCurrentAcademicYear = async () => {
  const cacheKey = "year:current";
  const cached = await calendarCache.get(cacheKey);
  if (cached) return cached;

  const [currentYear] = await db
    .select()
    .from(academicYears)
    .where(eq(academicYears.isCurrent, true))
    .limit(1);

  if (currentYear) {
    await calendarCache.set(cacheKey, currentYear);
  }
  return currentYear;
};

export const getAcademicYears = async () => {
    const cacheKey = "years:all";
    const cached = await calendarCache.get(cacheKey);
    if (cached) return cached;

    const results = await db.select().from(academicYears).orderBy(desc(academicYears.yearName));

    if (results) {
        await calendarCache.set(cacheKey, results);
    }
    return results;
}

export const getCurrentTerm = async () => {
    const cacheKey = "term:current";
    const cached = await calendarCache.get(cacheKey);
    if (cached) return cached;

    const [currentTerm] = await db
        .select()
        .from(academicTerms)
        .where(eq(academicTerms.isCurrent, true))
        .limit(1);
    
    if (currentTerm) {
        await calendarCache.set(cacheKey, currentTerm);
    }
    return currentTerm;
}

export const getTermsByYear = async (yearId: number) => {
    const cacheKey = `terms:${yearId}`;
    const cached = await calendarCache.get(cacheKey);
    if (cached) return cached;

    const results = await db
        .select()
        .from(academicTerms)
        .where(eq(academicTerms.academicYearId, yearId));
    
    if (results) {
        await calendarCache.set(cacheKey, results);
    }
    return results;
}

export type CreateAcademicYearParams = typeof academicYears.$inferInsert;
export const createAcademicYear = async (data: CreateAcademicYearParams) => {
  const [result] = await db.insert(academicYears).values(data).returning();
  await calendarCache.delete("years:all");
  return result;
};

export const updateAcademicYear = async (id: number, data: Partial<CreateAcademicYearParams>) => {
  const [result] = await db.update(academicYears).set(data).where(eq(academicYears.id, id)).returning();
  await calendarCache.delete("years:all");
  if (result?.isCurrent) await calendarCache.delete("year:current");
  return result;
};

export type CreateAcademicTermParams = typeof academicTerms.$inferInsert;
export const createAcademicTerm = async (data: CreateAcademicTermParams) => {
  const [result] = await db.insert(academicTerms).values(data).returning();
  if (result) await calendarCache.delete(`terms:${result.academicYearId}`);
  return result;
};

export const updateAcademicTerm = async (id: number, data: Partial<CreateAcademicTermParams>) => {
  const [result] = await db.update(academicTerms).set(data).where(eq(academicTerms.id, id)).returning();
  if (result) {
    await calendarCache.delete(`terms:${result.academicYearId}`);
    if (result.isCurrent) await calendarCache.delete("term:current");
  }
  return result;
};
