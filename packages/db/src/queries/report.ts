import type { Database } from "@db/client";
import {
  ministerialReports,
  students,
  users,
  classes,
  levels,
  gradeEntries,
  assessmentItems,
  curriculumSubjects,
  subjects,
  academicYears,
  studentEnrollments,
} from "@db/schema";
import { and, asc, desc, eq, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetReportByIdParams = {
  id: number;
};

export const getReportById = async (
  db: Database,
  params: GetReportByIdParams,
) => {
  const [result] = await db
    .select({
      id: ministerialReports.id,
      title: ministerialReports.title,
      reportDate: ministerialReports.reportDate,
      content: ministerialReports.content,
      generatedBy: ministerialReports.generatedBy,
      levelId: ministerialReports.levelId,
      classId: ministerialReports.classId,
      studentId: ministerialReports.studentId,
      totalScore: ministerialReports.totalScore,
      average: ministerialReports.average,
      passStatus: ministerialReports.passStatus,
      reportType: ministerialReports.reportType,
      level: {
        id: levels.id,
        name: levels.name,
        stage: levels.stage,
      },
      class: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
      },
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
      },
    })
    .from(ministerialReports)
    .where(eq(ministerialReports.id, params.id))
    .leftJoin(levels, eq(levels.id, ministerialReports.levelId))
    .leftJoin(classes, eq(classes.id, ministerialReports.classId))
    .leftJoin(students, eq(students.userId, ministerialReports.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .limit(1);

  return result;
};

export type GetReportsParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  levelId?: number | null;
  classId?: number | null;
  studentId?: string | null;
  reportType?: string | null;
};

export const getReports = async (
  db: Database,
  params: GetReportsParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 25,
    levelId,
    classId,
    studentId,
    reportType,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (levelId) {
    whereConditions.push(eq(ministerialReports.levelId, levelId));
  }

  if (classId) {
    whereConditions.push(eq(ministerialReports.classId, classId));
  }

  if (studentId) {
    whereConditions.push(eq(ministerialReports.studentId, studentId));
  }

  if (reportType) {
    whereConditions.push(eq(ministerialReports.reportType, reportType as any));
  }

  const query = db
    .select({
      id: ministerialReports.id,
      title: ministerialReports.title,
      reportDate: ministerialReports.reportDate,
      generatedBy: ministerialReports.generatedBy,
      levelId: ministerialReports.levelId,
      classId: ministerialReports.classId,
      studentId: ministerialReports.studentId,
      totalScore: ministerialReports.totalScore,
      average: ministerialReports.average,
      passStatus: ministerialReports.passStatus,
      reportType: ministerialReports.reportType,
      level: {
        id: levels.id,
        name: levels.name,
      },
      class: {
        id: classes.id,
        name: classes.name,
      },
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        name: users.name,
      },
    })
    .from(ministerialReports)
    .leftJoin(levels, eq(levels.id, ministerialReports.levelId))
    .leftJoin(classes, eq(classes.id, ministerialReports.classId))
    .leftJoin(students, eq(students.userId, ministerialReports.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "report_date") {
      isAscending
        ? query.orderBy(asc(ministerialReports.reportDate))
        : query.orderBy(desc(ministerialReports.reportDate));
    } else if (column === "average") {
      isAscending
        ? query.orderBy(asc(ministerialReports.average))
        : query.orderBy(desc(ministerialReports.average));
    } else if (column === "total_score") {
      isAscending
        ? query.orderBy(asc(ministerialReports.totalScore))
        : query.orderBy(desc(ministerialReports.totalScore));
    }
  } else {
    // Default sort by report_date descending
    query.orderBy(desc(ministerialReports.reportDate));
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

export type CreateReportParams = {
  title: string;
  reportDate: Date;
  content: string;
  generatedBy: string;
  levelId?: number | null;
  classId?: number | null;
  studentId?: string | null;
  totalScore?: number | null;
  average?: number | null;
  passStatus?: string | null;
  reportType?: string | null;
};

export const createReport = async (
  db: Database,
  params: CreateReportParams,
) => {
  const [result] = await db
    .insert(ministerialReports)
    .values(params)
    .returning();

  return result;
};

export type UpdateReportParams = {
  id: number;
  title?: string;
  content?: string;
  totalScore?: number | null;
  average?: number | null;
  passStatus?: string | null;
};

export const updateReport = async (
  db: Database,
  params: UpdateReportParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(ministerialReports)
    .set(rest)
    .where(eq(ministerialReports.id, id))
    .returning();

  return result;
};

export type DeleteReportParams = {
  id: number;
};

export const deleteReport = async (
  db: Database,
  params: DeleteReportParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(ministerialReports)
    .where(eq(ministerialReports.id, id))
    .returning();

  return result;
};

// Generate student ministerial report
export type GenerateStudentMinisterialReportParams = {
  studentId: string;
  academicYearId: number;
  reportType: "ministerial_grade9" | "ministerial_grade12";
  generatedBy: string;
};

export const generateStudentMinisterialReport = async (
  db: Database,
  params: GenerateStudentMinisterialReportParams,
) => {
  const { studentId, academicYearId, reportType, generatedBy } = params;

  // Get student info
  const [student] = await db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      surname: students.surname,
      name: users.name,
      currentClassId: students.currentClassId,
    })
    .from(students)
    .where(eq(students.userId, studentId))
    .innerJoin(users, eq(users.id, students.userId))
    .limit(1);

  if (!student) {
    throw new Error("Student not found");
  }

  // Get ministerial grades
  const grades = await db
    .select({
      subjectId: subjects.id,
      subjectName: subjects.name,
      subjectCode: subjects.code,
      score: gradeEntries.score,
      maxScore: assessmentItems.maxScore,
      orderInReport: curriculumSubjects.orderInReport,
    })
    .from(gradeEntries)
    .where(
      and(
        eq(gradeEntries.studentId, studentId),
        eq(gradeEntries.academicYearId, academicYearId),
        eq(assessmentItems.isMinisterial, true),
      ),
    )
    .innerJoin(assessmentItems, eq(assessmentItems.id, gradeEntries.assessmentItemId))
    .innerJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentItems.curriculumSubjectId))
    .innerJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .orderBy(asc(curriculumSubjects.orderInReport));

  // Calculate totals
  const totalScore = grades.reduce((sum, g) => sum + (Number(g.score) || 0), 0);
  const maxTotalScore = grades.reduce((sum, g) => sum + (Number(g.maxScore) || 0), 0);
  const average = maxTotalScore > 0 ? (totalScore / maxTotalScore) * 100 : 0;
  const passStatus = average >= 50 ? "Pass" : "Fail";

  // Generate content (JSON format)
  const content = JSON.stringify({
    student: {
      id: student.userId,
      number: student.studentNumber,
      name: student.name,
      surname: student.surname,
    },
    grades,
    summary: {
      totalScore,
      maxTotalScore,
      average: Math.round(average * 100) / 100,
      passStatus,
    },
  });

  // Create report
  const [report] = await db
    .insert(ministerialReports)
    .values({
      title: `${reportType === "ministerial_grade9" ? "Grade 9" : "Grade 12"} Ministerial Report - ${student.name}`,
      reportDate: new Date(),
      content,
      generatedBy,
      studentId,
      classId: student.currentClassId,
      totalScore,
      average: Math.round(average * 100) / 100,
      passStatus,
      reportType,
    })
    .returning();

  return report;
};

// Get class performance summary
export type GetClassPerformanceSummaryParams = {
  classId: number;
  academicYearId: number;
};

export const getClassPerformanceSummary = async (
  db: Database,
  params: GetClassPerformanceSummaryParams,
) => {
  const { classId, academicYearId } = params;

  const data = await db
    .select({
      studentId: students.userId,
      studentNumber: students.studentNumber,
      studentName: users.name,
      surname: students.surname,
      totalScore: sql<number>`coalesce(sum(${gradeEntries.score}), 0)`,
      maxScore: sql<number>`coalesce(sum(${assessmentItems.maxScore}), 0)`,
      average: sql<number>`
        case 
          when sum(${assessmentItems.maxScore}) > 0 
          then (coalesce(sum(${gradeEntries.score}), 0) / sum(${assessmentItems.maxScore})) * 100
          else 0
        end
      `,
      gradeCount: sql<number>`cast(count(${gradeEntries.id}) as int)`,
    })
    .from(students)
    .where(eq(students.currentClassId, classId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(
      gradeEntries,
      and(
        eq(gradeEntries.studentId, students.userId),
        eq(gradeEntries.academicYearId, academicYearId),
      ),
    )
    .leftJoin(assessmentItems, eq(assessmentItems.id, gradeEntries.assessmentItemId))
    .groupBy(students.userId, students.studentNumber, users.name, students.surname)
    .orderBy(desc(sql`
      case 
        when sum(${assessmentItems.maxScore}) > 0 
        then (coalesce(sum(${gradeEntries.score}), 0) / sum(${assessmentItems.maxScore})) * 100
        else 0
      end
    `));

  return data;
};

// Get subject performance analysis
export type GetSubjectPerformanceAnalysisParams = {
  classId: number;
  academicYearId: number;
};

export const getSubjectPerformanceAnalysis = async (
  db: Database,
  params: GetSubjectPerformanceAnalysisParams,
) => {
  const { classId, academicYearId } = params;

  const data = await db
    .select({
      subjectId: subjects.id,
      subjectName: subjects.name,
      subjectCode: subjects.code,
      studentCount: sql<number>`cast(count(distinct ${gradeEntries.studentId}) as int)`,
      averageScore: sql<number>`
        case 
          when sum(${assessmentItems.maxScore}) > 0 
          then (coalesce(sum(${gradeEntries.score}), 0) / sum(${assessmentItems.maxScore})) * 100
          else 0
        end
      `,
      highestScore: sql<number>`max(${gradeEntries.score})`,
      lowestScore: sql<number>`min(${gradeEntries.score})`,
      passCount: sql<number>`
        cast(sum(
          case 
            when (${gradeEntries.score} / ${assessmentItems.maxScore}) * 100 >= 50 
            then 1 
            else 0 
          end
        ) as int)
      `,
    })
    .from(gradeEntries)
    .where(eq(gradeEntries.academicYearId, academicYearId))
    .innerJoin(students, eq(students.userId, gradeEntries.studentId))
    .where(eq(students.currentClassId, classId))
    .innerJoin(assessmentItems, eq(assessmentItems.id, gradeEntries.assessmentItemId))
    .innerJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentItems.curriculumSubjectId))
    .innerJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .groupBy(subjects.id, subjects.name, subjects.code)
    .orderBy(asc(subjects.name));

  return data;
};
