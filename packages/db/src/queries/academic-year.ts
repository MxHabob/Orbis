import type { Database } from "@db/client";
import { academicYears, academicTerms } from "@db/schema";
import { and, asc, desc, eq, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetAcademicYearByIdParams = {
  id: number;
};

export const getAcademicYearById = async (
  db: Database,
  params: GetAcademicYearByIdParams,
) => {
  const [result] = await db
    .select({
      id: academicYears.id,
      year: academicYears.year,
      startDate: academicYears.startDate,
      endDate: academicYears.endDate,
      isCurrent: academicYears.isCurrent,
      isActive: academicYears.isActive,
      createdAt: academicYears.createdAt,
      updatedAt: academicYears.updatedAt,
      termCount: sql<number>`cast(count(${academicTerms.id}) as int)`,
    })
    .from(academicYears)
    .where(eq(academicYears.id, params.id))
    .leftJoin(academicTerms, eq(academicTerms.academicYearId, academicYears.id))
    .groupBy(academicYears.id);

  return result;
};

type GetCurrentAcademicYearParams = {
  // No params needed
};

export const getCurrentAcademicYear = async (
  db: Database,
  _params?: GetCurrentAcademicYearParams,
) => {
  const [result] = await db
    .select({
      id: academicYears.id,
      year: academicYears.year,
      startDate: academicYears.startDate,
      endDate: academicYears.endDate,
      isCurrent: academicYears.isCurrent,
      isActive: academicYears.isActive,
      createdAt: academicYears.createdAt,
      updatedAt: academicYears.updatedAt,
    })
    .from(academicYears)
    .where(and(eq(academicYears.isCurrent, true), eq(academicYears.isActive, true)))
    .limit(1);

  return result;
};

export type GetAcademicYearsParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  activeOnly?: boolean;
};

export const getAcademicYears = async (
  db: Database,
  params: GetAcademicYearsParams,
) => {
  const { sort, cursor, pageSize = 25, activeOnly = false } = params;

  const whereConditions: SQL[] = [];

  if (activeOnly) {
    whereConditions.push(eq(academicYears.isActive, true));
  }

  const query = db
    .select({
      id: academicYears.id,
      year: academicYears.year,
      startDate: academicYears.startDate,
      endDate: academicYears.endDate,
      isCurrent: academicYears.isCurrent,
      isActive: academicYears.isActive,
      createdAt: academicYears.createdAt,
      updatedAt: academicYears.updatedAt,
      termCount: sql<number>`cast(count(${academicTerms.id}) as int)`,
    })
    .from(academicYears)
    .leftJoin(academicTerms, eq(academicTerms.academicYearId, academicYears.id))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(academicYears.id);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "year") {
      isAscending
        ? query.orderBy(asc(academicYears.year))
        : query.orderBy(desc(academicYears.year));
    } else if (column === "start_date") {
      isAscending
        ? query.orderBy(asc(academicYears.startDate))
        : query.orderBy(desc(academicYears.startDate));
    } else if (column === "created_at") {
      isAscending
        ? query.orderBy(asc(academicYears.createdAt))
        : query.orderBy(desc(academicYears.createdAt));
    }
  } else {
    // Default sort by start date descending
    query.orderBy(desc(academicYears.startDate));
  }

  // Apply pagination
  const offset = cursor ? Number.parseInt(cursor, 10) : 0;
  query.limit(pageSize).offset(offset);

  const data = await query;

  const nextCursor =
    data && data.length === pageSize
      ? (offset + pageSize).toString()
      : undefined;

  return {
    meta: {
      cursor: nextCursor ?? null,
      hasPreviousPage: offset > 0,
      hasNextPage: data && data.length === pageSize,
    },
    data,
  };
};

export type CreateAcademicYearParams = {
  year: string;
  startDate: Date;
  endDate?: Date | null;
  isCurrent?: boolean;
  isActive?: boolean;
};

export const createAcademicYear = async (
  db: Database,
  params: CreateAcademicYearParams,
) => {
  const { year, startDate, endDate, isCurrent = false, isActive = true } = params;

  // If setting as current, unset other current years
  if (isCurrent) {
    await db
      .update(academicYears)
      .set({ isCurrent: false })
      .where(eq(academicYears.isCurrent, true));
  }

  const [result] = await db
    .insert(academicYears)
    .values({
      year,
      startDate,
      endDate,
      isCurrent,
      isActive,
    })
    .returning();

  return result;
};

export type UpdateAcademicYearParams = {
  id: number;
  year?: string;
  startDate?: Date;
  endDate?: Date | null;
  isCurrent?: boolean;
  isActive?: boolean;
};

export const updateAcademicYear = async (
  db: Database,
  params: UpdateAcademicYearParams,
) => {
  const { id, isCurrent, ...rest } = params;

  // If setting as current, unset other current years
  if (isCurrent) {
    await db
      .update(academicYears)
      .set({ isCurrent: false })
      .where(eq(academicYears.isCurrent, true));
  }

  const [result] = await db
    .update(academicYears)
    .set({
      ...rest,
      isCurrent,
      updatedAt: new Date(),
    })
    .where(eq(academicYears.id, id))
    .returning();

  return result;
};

export type DeleteAcademicYearParams = {
  id: number;
};

export const deleteAcademicYear = async (
  db: Database,
  params: DeleteAcademicYearParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(academicYears)
    .where(eq(academicYears.id, id))
    .returning();

  return result;
};

// Academic Terms Queries

type GetAcademicTermsByYearParams = {
  academicYearId: number;
};

export const getAcademicTermsByYear = async (
  db: Database,
  params: GetAcademicTermsByYearParams,
) => {
  const { academicYearId } = params;

  const data = await db
    .select({
      id: academicTerms.id,
      academicYearId: academicTerms.academicYearId,
      name: academicTerms.name,
      startDate: academicTerms.startDate,
      endDate: academicTerms.endDate,
      order: academicTerms.order,
    })
    .from(academicTerms)
    .where(eq(academicTerms.academicYearId, academicYearId))
    .orderBy(asc(academicTerms.order));

  return data;
};

export type CreateAcademicTermParams = {
  academicYearId: number;
  name: string;
  startDate: Date;
  endDate?: Date | null;
  order: number;
};

export const createAcademicTerm = async (
  db: Database,
  params: CreateAcademicTermParams,
) => {
  const [result] = await db
    .insert(academicTerms)
    .values(params)
    .returning();

  return result;
};

export type UpdateAcademicTermParams = {
  id: number;
  name?: string;
  startDate?: Date;
  endDate?: Date | null;
  order?: number;
};

export const updateAcademicTerm = async (
  db: Database,
  params: UpdateAcademicTermParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(academicTerms)
    .set(rest)
    .where(eq(academicTerms.id, id))
    .returning();

  return result;
};

export type DeleteAcademicTermParams = {
  id: number;
};

export const deleteAcademicTerm = async (
  db: Database,
  params: DeleteAcademicTermParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(academicTerms)
    .where(eq(academicTerms.id, id))
    .returning();

  return result;
};
