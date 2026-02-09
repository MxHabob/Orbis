import { db } from "@db/client";
import {
  gradeEntries,
  assessmentItems,
  subjects,
  classes,
  academicTerms,
  enrollments,
  students,
  users,
  academicYears,
} from "@db/schema";
import { eq, and, inArray, asc } from "drizzle-orm";

export type CertificateType = "monthly" | "mid_term" | "year_end";

/** Map assessment type to English column key for monthly certificate */
const MONTHLY_TYPE_MAP: Record<string, string> = {
  behavior: "Behavior",
  attendance: "Attendance",
  participation: "Participation",
  oral: "Oral",
  exam: "Written Exam",
  assignment: "Assignment",
  homework: "Assignment",
  quiz: "Written Exam",
};

export type CertificateDataParams = {
  studentId: string;
  classId: number;
  academicYearId: number;
  termId: number;
  certificateType: CertificateType;
  /** Required for certificateType === 'monthly'. Month 1-12 */
  month?: number;
};

export type SubjectGradeRow = {
  subjectId: number;
  subjectName: string;
  subjectCode: string | null;
  /** For monthly: { Behavior, Attendance, Participation, Oral, Written Exam, Total } */
  /** For mid_term: { Monthly Average, Mid-term Exam, Total } */
  /** For year_end: { Term 1 Average, Term 2 Average, Total } */
  columns: Record<string, number | null>;
  total: number | null;
};

export type CertificateDataResult = {
  studentName: string | null;
  studentNumber: string | null;
  className: string;
  academicYearName: string;
  termName: string;
  certificateType: CertificateType;
  month?: number;
  subjectRows: SubjectGradeRow[];
  schoolName: string;
};

const DEFAULT_SCHOOL_NAME = "Sanaa Model School";

export async function getCertificateData(
  params: CertificateDataParams
): Promise<CertificateDataResult | null> {
  const { studentId, classId, academicYearId, termId, certificateType, month } = params;

  // 1. Student + enrollment check
  const [enrollment] = await db
    .select({
      studentId: enrollments.studentId,
      classId: enrollments.classId,
    })
    .from(enrollments)
    .where(
      and(
        eq(enrollments.studentId, studentId),
        eq(enrollments.classId, classId),
        eq(enrollments.academicYearId, academicYearId),
        eq(enrollments.status, "active"),
      ),
    )
    .limit(1);

  if (!enrollment) return null;

  // 2. Student info
  const [studentRow] = await db
    .select({
      name: users.name,
      studentNumber: students.studentNumber,
    })
    .from(students)
    .innerJoin(users, eq(students.userId, users.id))
    .where(eq(students.userId, studentId))
    .limit(1);

  if (!studentRow) return null;

  // 3. Class + year + term names
  const [classRow] = await db
    .select({ name: classes.name })
    .from(classes)
    .where(eq(classes.id, classId))
    .limit(1);

  const [yearRow] = await db
    .select({ yearName: academicYears.yearName })
    .from(academicYears)
    .where(eq(academicYears.id, academicYearId))
    .limit(1);

  const [termRow] = await db
    .select({ name: academicTerms.name })
    .from(academicTerms)
    .where(eq(academicTerms.id, termId))
    .limit(1);

  // For year_end we need subjects from both terms
  const termIdsForSubjects = certificateType === "year_end"
    ? (await db.select({ id: academicTerms.id }).from(academicTerms).where(eq(academicTerms.academicYearId, academicYearId))).map((t) => t.id)
    : [termId];

  // 4. Distinct subjects for this class in this term (from assessment_items)
  const subjectRows = await db
    .selectDistinct({
      subjectId: subjects.id,
      subjectName: subjects.name,
      subjectCode: subjects.code,
    })
    .from(assessmentItems)
    .innerJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .where(
      and(
        eq(assessmentItems.classId, classId),
        termIdsForSubjects.length === 1
          ? eq(assessmentItems.termId, termIdsForSubjects[0]!)
          : inArray(assessmentItems.termId, termIdsForSubjects)
      )
    )
    .orderBy(asc(subjects.name));

  if (subjectRows.length === 0) {
    return {
      studentName: studentRow.name,
      studentNumber: studentRow.studentNumber,
      className: classRow?.name ?? "",
      academicYearName: yearRow?.yearName ?? "",
      termName: termRow?.name ?? "",
      certificateType,
      month,
      subjectRows: [],
      schoolName: DEFAULT_SCHOOL_NAME,
    };
  }

  const subjectIds = subjectRows.map((s) => s.subjectId);

  let monthsToFetch: number[];
  let termIdsToFetch: number[] = [termId];
  if (certificateType === "monthly") {
    if (month == null || month < 1 || month > 12) return null;
    monthsToFetch = [month];
  } else if (certificateType === "mid_term") {
    monthsToFetch = [9, 10, 11, 13];
  } else {
    monthsToFetch = [9, 10, 11, 12, 13, 1, 2, 14];
    const yearTerms = await db
      .select({ id: academicTerms.id })
      .from(academicTerms)
      .where(eq(academicTerms.academicYearId, academicYearId))
      .orderBy(asc(academicTerms.id));
    termIdsToFetch = yearTerms.map((t) => t.id);
  }

  // 5. All assessment items for these subjects + months (and both terms for year_end)
  const items = await db
    .select({
      id: assessmentItems.id,
      subjectId: assessmentItems.subjectId,
      termId: assessmentItems.termId,
      type: assessmentItems.type,
      month: assessmentItems.month,
      maxScore: assessmentItems.maxScore,
      title: assessmentItems.title,
    })
    .from(assessmentItems)
    .where(
      and(
        eq(assessmentItems.classId, classId),
        inArray(assessmentItems.termId, termIdsToFetch),
        inArray(assessmentItems.subjectId, subjectIds),
        inArray(assessmentItems.month, monthsToFetch)
      )
    )
    .orderBy(asc(assessmentItems.subjectId), asc(assessmentItems.month), asc(assessmentItems.id));

  const itemIds = items.map((i) => i.id);

  // 6. Grades for this student for these items
  const grades = await db
    .select({
      assessmentItemId: gradeEntries.assessmentItemId,
      score: gradeEntries.score,
    })
    .from(gradeEntries)
    .where(
      and(
        eq(gradeEntries.studentId, studentId),
        inArray(gradeEntries.assessmentItemId, itemIds)
      )
    );

  const gradeByItemId = new Map(
    grades.map((g) => [g.assessmentItemId, g.score])
  );

  function parseScore(s: string | null): number | null {
    if (s == null || s === "" || s === "null") return null;
    const n = Number.parseFloat(s);
    return Number.isNaN(n) ? null : n;
  }

  const subjectGradeRows: SubjectGradeRow[] = subjectRows.map((subj) => {
    const subjItems = items.filter((i) => i.subjectId === subj.subjectId);
    const columns: Record<string, number | null> = {};
    let total: number | null = null;

    if (certificateType === "monthly") {
      const monthItems = subjItems.filter((i) => i.month === month);
      let sum = 0;
      let hasAny = false;
      for (const it of monthItems) {
        const colKey = MONTHLY_TYPE_MAP[it.type] ?? it.type;
        const score = parseScore(gradeByItemId.get(it.id) ?? null);
        if (score != null) {
          columns[colKey] = score;
          sum += score;
          hasAny = true;
        } else {
          columns[colKey] = null;
        }
      }
      if (hasAny) total = sum;
      // Ensure standard column keys exist for header
      ["Behavior", "Attendance", "Participation", "Oral", "Written Exam"].forEach((k) => {
        if (columns[k] === undefined) columns[k] = null;
      });
    } else if (certificateType === "mid_term") {
      const monthlyItems = subjItems.filter((i) => [9, 10, 11].includes(i.month));
      const examItems = subjItems.filter((i) => i.month === 13);
      let monthlyTotal = 0;
      let examScore: number | null = null;
      for (const it of monthlyItems) {
        const s = parseScore(gradeByItemId.get(it.id) ?? null);
        if (s != null) monthlyTotal += s;
      }
      const examItem = examItems[0];
      if (examItem) {
        examScore = parseScore(gradeByItemId.get(examItem.id) ?? null);
      }
      columns["Monthly Average"] = monthlyTotal > 0 ? monthlyTotal : null;
      columns["Mid-term Exam"] = examScore;
      total =
        monthlyTotal > 0 && examScore != null
          ? monthlyTotal + examScore
          : examScore ?? (monthlyTotal > 0 ? monthlyTotal : null);
    } else {
      // year_end: aggregate by termId (first term id = term 1, second = term 2)
      const [term1Id, term2Id] = termIdsToFetch;
      let t1 = 0;
      let t2 = 0;
      for (const it of subjItems) {
        const s = parseScore(gradeByItemId.get(it.id) ?? null);
        if (s == null) continue;
        if (it.termId === term1Id) t1 += s;
        if (term2Id != null && it.termId === term2Id) t2 += s;
      }
      columns["Term 1 Average"] = t1 > 0 ? t1 : null;
      columns["Term 2 Average"] = t2 > 0 ? t2 : null;
      total = t1 > 0 && t2 > 0 ? t1 + t2 : t1 > 0 ? t1 : t2 > 0 ? t2 : null;
    }

    return {
      subjectId: subj.subjectId,
      subjectName: subj.subjectName,
      subjectCode: subj.subjectCode,
      columns,
      total,
    };
  });

  return {
    studentName: studentRow.name,
    studentNumber: studentRow.studentNumber,
    className: classRow?.name ?? "",
    academicYearName: yearRow?.yearName ?? "",
    termName: termRow?.name ?? "",
    certificateType,
    month,
    subjectRows: subjectGradeRows,
    schoolName: DEFAULT_SCHOOL_NAME,
  };
}
