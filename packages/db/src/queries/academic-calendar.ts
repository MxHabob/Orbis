import { db } from "@db/client";
import { academicYears, academicTerms, classes, timeSlots } from "@db/schema";
import { eq, desc } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

const calendarCache = new RedisCache("calendar", 60 * 60 * 24); // 24 hours

export const getCurrentAcademicYear = async () => {
  const cacheKey = "year:current";
  const cached = await calendarCache.get<typeof academicYears.$inferSelect>(cacheKey);
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
  const cached = await calendarCache.get<any[]>(cacheKey);
  if (cached) return cached;

  const results = await db.query.academicYears.findMany({
    orderBy: (years, { desc }) => [desc(years.yearName)],
    with: {
      terms: true,
    },
  });

  if (results) {
    await calendarCache.set(cacheKey, results);
  }
  return results;
};

export const getAcademicYearById = async (id: number) => {
  const [result] = await db
    .select()
    .from(academicYears)
    .where(eq(academicYears.id, id))
    .limit(1);
  return result;
};

export const getCurrentTerm = async () => {
    const cacheKey = "term:current";
    const cached = await calendarCache.get<typeof academicTerms.$inferSelect>(cacheKey);
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
    const cached = await calendarCache.get<Array<typeof academicTerms.$inferSelect>>(cacheKey);
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
  return await db.transaction(async (tx) => {
    const [result] = await tx.insert(academicYears).values(data).returning();

    if (!result) throw new Error("Failed to create academic year");

    const yearId = result.id;
    const yearName = result.yearName;

    // 1. Create Terms (Default structure)
    const startYearMatch = yearName.match(/\d{4}/);
    const startYear = startYearMatch ? parseInt(startYearMatch[0]) : new Date().getFullYear();
    const endYear = startYear + 1;

    await tx.insert(academicTerms).values([
      {
        name: "الترم الاول",
        academicYearId: yearId,
        startDate: `${startYear}-09-01`,
        endDate: `${startYear}-12-30`,
        isCurrent: true,
      },
      {
        name: "الترم الثاني",
        academicYearId: yearId,
        startDate: `${endYear}-02-01`,
        endDate: `${endYear}-06-20`,
        isCurrent: false,
      },
    ]);

    // 2. Create Classes (Default Grades 1-9)
    const grades = [
      "الصف الأول", "الصف الثاني", "الصف الثالث", 
      "الصف الرابع", "الصف الخامس", "الصف السادس",
      "الصف السابع", "الصف الثامن", "الصف التاسع"
    ];
    const colors = ["#f44336", "#e91e63", "#9c27b0", "#673ab7", "#3f51b5", "#2196f3", "#00bcd4", "#009688", "#4caf50"];

    await tx.insert(classes).values(
      grades.map((name, index) => ({
        name,
        color: colors[index] || "#e6e6e6",
        sortOrder: index,
        type: "classroom" as const,
      }))
    );

    await calendarCache.delete("years:all");
    return result;
  });
};

export const updateAcademicYear = async (id: number, data: Partial<CreateAcademicYearParams>) => {
  const [result] = await db.update(academicYears).set(data).where(eq(academicYears.id, id)).returning();
  await calendarCache.delete("years:all");
  return result;
};

export const deleteAcademicYear = async (id: number) => {
  const [result] = await db.delete(academicYears).where(eq(academicYears.id, id)).returning();
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

export const deleteAcademicTerm = async (id: number) => {
  const [result] = await db
    .delete(academicTerms)
    .where(eq(academicTerms.id, id))
    .returning();
  if (result) {
    await calendarCache.delete(`terms:${result.academicYearId}`);
    if (result.isCurrent) await calendarCache.delete("term:current");
  }
  return result;
};
