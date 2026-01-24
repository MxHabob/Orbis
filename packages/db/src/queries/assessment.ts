import type { Database } from "@db/client";
import {
  assessmentItems,
  assessmentCategories,
  curriculumSubjects,
  subjects,
  academicTerms,
  academicYears,
  gradeEntries,
} from "@db/schema";
import { and, asc, desc, eq, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

// Assessment Categories

type GetAssessmentCategoryByIdParams = {
  id: number;
};

export const getAssessmentCategoryById = async (
  db: Database,
  params: GetAssessmentCategoryByIdParams,
) => {
  const [result] = await db
    .select({
      id: assessmentCategories.id,
      name: assessmentCategories.name,
      weight: assessmentCategories.weight,
      academicYearId: assessmentCategories.academicYearId,
      curriculumSubjectId: assessmentCategories.curriculumSubjectId,
      order: assessmentCategories.order,
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
      },
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      itemCount: sql<number>`cast(count(${assessmentItems.id}) as int)`,
    })
    .from(assessmentCategories)
    .where(eq(assessmentCategories.id, params.id))
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentCategories.curriculumSubjectId))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .leftJoin(academicYears, eq(academicYears.id, assessmentCategories.academicYearId))
    .leftJoin(assessmentItems, eq(assessmentItems.assessmentCategoryId, assessmentCategories.id))
    .groupBy(assessmentCategories.id, subjects.id, academicYears.id);

  return result;
};

export type GetAssessmentCategoriesParams = {
  academicYearId?: number | null;
  curriculumSubjectId?: number | null;
};

export const getAssessmentCategories = async (
  db: Database,
  params: GetAssessmentCategoriesParams,
) => {
  const { academicYearId, curriculumSubjectId } = params;

  const whereConditions: SQL[] = [];

  if (academicYearId) {
    whereConditions.push(eq(assessmentCategories.academicYearId, academicYearId));
  }

  if (curriculumSubjectId) {
    whereConditions.push(eq(assessmentCategories.curriculumSubjectId, curriculumSubjectId));
  }

  const data = await db
    .select({
      id: assessmentCategories.id,
      name: assessmentCategories.name,
      weight: assessmentCategories.weight,
      academicYearId: assessmentCategories.academicYearId,
      curriculumSubjectId: assessmentCategories.curriculumSubjectId,
      order: assessmentCategories.order,
      subject: {
        id: subjects.id,
        name: subjects.name,
      },
      itemCount: sql<number>`cast(count(${assessmentItems.id}) as int)`,
    })
    .from(assessmentCategories)
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentCategories.curriculumSubjectId))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .leftJoin(assessmentItems, eq(assessmentItems.assessmentCategoryId, assessmentCategories.id))
    .groupBy(assessmentCategories.id, subjects.id)
    .orderBy(asc(assessmentCategories.order));

  return data;
};

export type CreateAssessmentCategoryParams = {
  name: string;
  weight: number;
  academicYearId?: number | null;
  curriculumSubjectId?: number | null;
  order?: number | null;
};

export const createAssessmentCategory = async (
  db: Database,
  params: CreateAssessmentCategoryParams,
) => {
  const [result] = await db
    .insert(assessmentCategories)
    .values(params)
    .returning();

  return result;
};

export type UpdateAssessmentCategoryParams = {
  id: number;
  name?: string;
  weight?: number;
  order?: number | null;
};

export const updateAssessmentCategory = async (
  db: Database,
  params: UpdateAssessmentCategoryParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(assessmentCategories)
    .set(rest)
    .where(eq(assessmentCategories.id, id))
    .returning();

  return result;
};

export type DeleteAssessmentCategoryParams = {
  id: number;
};

export const deleteAssessmentCategory = async (
  db: Database,
  params: DeleteAssessmentCategoryParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(assessmentCategories)
    .where(eq(assessmentCategories.id, id))
    .returning();

  return result;
};

// Assessment Items

type GetAssessmentItemByIdParams = {
  id: number;
};

export const getAssessmentItemById = async (
  db: Database,
  params: GetAssessmentItemByIdParams,
) => {
  const [result] = await db
    .select({
      id: assessmentItems.id,
      title: assessmentItems.title,
      type: assessmentItems.type,
      maxScore: assessmentItems.maxScore,
      weight: assessmentItems.weight,
      dueDate: assessmentItems.dueDate,
      assessmentCategoryId: assessmentItems.assessmentCategoryId,
      curriculumSubjectId: assessmentItems.curriculumSubjectId,
      termId: assessmentItems.termId,
      isMinisterial: assessmentItems.isMinisterial,
      category: {
        id: assessmentCategories.id,
        name: assessmentCategories.name,
        weight: assessmentCategories.weight,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
      },
      term: {
        id: academicTerms.id,
        name: academicTerms.name,
      },
      gradeCount: sql<number>`cast(count(${gradeEntries.id}) as int)`,
    })
    .from(assessmentItems)
    .where(eq(assessmentItems.id, params.id))
    .leftJoin(assessmentCategories, eq(assessmentCategories.id, assessmentItems.assessmentCategoryId))
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentItems.curriculumSubjectId))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .leftJoin(academicTerms, eq(academicTerms.id, assessmentItems.termId))
    .leftJoin(gradeEntries, eq(gradeEntries.assessmentItemId, assessmentItems.id))
    .groupBy(
      assessmentItems.id,
      assessmentCategories.id,
      subjects.id,
      academicTerms.id,
    );

  return result;
};

export type GetAssessmentItemsParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  curriculumSubjectId?: number | null;
  termId?: number | null;
  assessmentCategoryId?: number | null;
  type?: string | null;
  isMinisterial?: boolean | null;
};

export const getAssessmentItems = async (
  db: Database,
  params: GetAssessmentItemsParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 50,
    curriculumSubjectId,
    termId,
    assessmentCategoryId,
    type,
    isMinisterial,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (curriculumSubjectId) {
    whereConditions.push(eq(assessmentItems.curriculumSubjectId, curriculumSubjectId));
  }

  if (termId) {
    whereConditions.push(eq(assessmentItems.termId, termId));
  }

  if (assessmentCategoryId) {
    whereConditions.push(eq(assessmentItems.assessmentCategoryId, assessmentCategoryId));
  }

  if (type) {
    whereConditions.push(eq(assessmentItems.type, type as any));
  }

  if (isMinisterial !== null && isMinisterial !== undefined) {
    whereConditions.push(eq(assessmentItems.isMinisterial, isMinisterial));
  }

  const query = db
    .select({
      id: assessmentItems.id,
      title: assessmentItems.title,
      type: assessmentItems.type,
      maxScore: assessmentItems.maxScore,
      weight: assessmentItems.weight,
      dueDate: assessmentItems.dueDate,
      assessmentCategoryId: assessmentItems.assessmentCategoryId,
      curriculumSubjectId: assessmentItems.curriculumSubjectId,
      termId: assessmentItems.termId,
      isMinisterial: assessmentItems.isMinisterial,
      category: {
        id: assessmentCategories.id,
        name: assessmentCategories.name,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
      },
      term: {
        id: academicTerms.id,
        name: academicTerms.name,
      },
      gradeCount: sql<number>`cast(count(${gradeEntries.id}) as int)`,
    })
    .from(assessmentItems)
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .leftJoin(assessmentCategories, eq(assessmentCategories.id, assessmentItems.assessmentCategoryId))
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentItems.curriculumSubjectId))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .leftJoin(academicTerms, eq(academicTerms.id, assessmentItems.termId))
    .leftJoin(gradeEntries, eq(gradeEntries.assessmentItemId, assessmentItems.id))
    .groupBy(
      assessmentItems.id,
      assessmentCategories.id,
      subjects.id,
      academicTerms.id,
    );

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "title") {
      isAscending
        ? query.orderBy(asc(assessmentItems.title))
        : query.orderBy(desc(assessmentItems.title));
    } else if (column === "due_date") {
      isAscending
        ? query.orderBy(asc(assessmentItems.dueDate))
        : query.orderBy(desc(assessmentItems.dueDate));
    } else if (column === "type") {
      isAscending
        ? query.orderBy(asc(assessmentItems.type))
        : query.orderBy(desc(assessmentItems.type));
    }
  } else {
    // Default sort by due_date descending
    query.orderBy(desc(assessmentItems.dueDate));
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

export type CreateAssessmentItemParams = {
  title: string;
  type: string;
  maxScore?: number;
  weight?: number | null;
  dueDate?: Date | null;
  assessmentCategoryId?: number | null;
  curriculumSubjectId?: number | null;
  termId?: number | null;
  isMinisterial?: boolean;
};

export const createAssessmentItem = async (
  db: Database,
  params: CreateAssessmentItemParams,
) => {
  const { maxScore = 100, isMinisterial = false, ...rest } = params;

  const [result] = await db
    .insert(assessmentItems)
    .values({
      ...rest,
      maxScore,
      isMinisterial,
    })
    .returning();

  return result;
};

export type UpdateAssessmentItemParams = {
  id: number;
  title?: string;
  type?: string;
  maxScore?: number;
  weight?: number | null;
  dueDate?: Date | null;
  assessmentCategoryId?: number | null;
  isMinisterial?: boolean;
};

export const updateAssessmentItem = async (
  db: Database,
  params: UpdateAssessmentItemParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(assessmentItems)
    .set(rest)
    .where(eq(assessmentItems.id, id))
    .returning();

  return result;
};

export type DeleteAssessmentItemParams = {
  id: number;
};

export const deleteAssessmentItem = async (
  db: Database,
  params: DeleteAssessmentItemParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(assessmentItems)
    .where(eq(assessmentItems.id, id))
    .returning();

  return result;
};
