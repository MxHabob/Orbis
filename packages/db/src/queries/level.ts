import type { Database } from "@db/client";
import {
  levels,
  classes,
  curriculumSubjects,
  students,
} from "@db/schema";
import { and, asc, desc, eq, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetLevelByIdParams = {
  id: number;
};

export const getLevelById = async (
  db: Database,
  params: GetLevelByIdParams,
) => {
  const [result] = await db
    .select({
      id: levels.id,
      name: levels.name,
      stage: levels.stage,
      order: levels.order,
      classCount: sql<number>`cast(count(distinct ${classes.id}) as int)`,
      studentCount: sql<number>`cast(count(distinct ${students.userId}) as int)`,
    })
    .from(levels)
    .where(eq(levels.id, params.id))
    .leftJoin(classes, eq(classes.levelId, levels.id))
    .leftJoin(students, eq(students.currentClassId, classes.id))
    .groupBy(levels.id);

  return result;
};

export type GetLevelsParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  stage?: string | null;
};

export const getLevels = async (
  db: Database,
  params: GetLevelsParams,
) => {
  const { sort, cursor, pageSize = 50, stage } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (stage) {
    whereConditions.push(eq(levels.stage, stage as any));
  }

  const query = db
    .select({
      id: levels.id,
      name: levels.name,
      stage: levels.stage,
      order: levels.order,
      classCount: sql<number>`cast(count(distinct ${classes.id}) as int)`,
      studentCount: sql<number>`cast(count(distinct ${students.userId}) as int)`,
    })
    .from(levels)
    .leftJoin(classes, eq(classes.levelId, levels.id))
    .leftJoin(students, eq(students.currentClassId, classes.id))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(levels.id);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "name") {
      isAscending
        ? query.orderBy(asc(levels.name))
        : query.orderBy(desc(levels.name));
    } else if (column === "order") {
      isAscending
        ? query.orderBy(asc(levels.order))
        : query.orderBy(desc(levels.order));
    }
  } else {
    // Default sort by order ascending
    query.orderBy(asc(levels.order));
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

export type CreateLevelParams = {
  name: string;
  stage: string;
  order: number;
};

export const createLevel = async (
  db: Database,
  params: CreateLevelParams,
) => {
  const [result] = await db
    .insert(levels)
    .values(params)
    .returning();

  return result;
};

export type UpdateLevelParams = {
  id: number;
  name?: string;
  stage?: string;
  order?: number;
};

export const updateLevel = async (
  db: Database,
  params: UpdateLevelParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(levels)
    .set(rest)
    .where(eq(levels.id, id))
    .returning();

  return result;
};

export type DeleteLevelParams = {
  id: number;
};

export const deleteLevel = async (
  db: Database,
  params: DeleteLevelParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(levels)
    .where(eq(levels.id, id))
    .returning();

  return result;
};

// Get levels by stage
export type GetLevelsByStageParams = {
  stage: "basic" | "secondary";
};

export const getLevelsByStage = async (
  db: Database,
  params: GetLevelsByStageParams,
) => {
  const { stage } = params;

  const data = await db
    .select({
      id: levels.id,
      name: levels.name,
      stage: levels.stage,
      order: levels.order,
    })
    .from(levels)
    .where(eq(levels.stage, stage))
    .orderBy(asc(levels.order));

  return data;
};
