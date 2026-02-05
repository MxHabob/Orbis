import { db } from "@midday/db/client";
import { 
  gradeEntries, 
  assessmentItems, 
  subjects, 
  students, 
  users, 
  classes, 
  enrollments,
  academicTerms
} from "@midday/db/schema";
import { eq, and, inArray, asc } from "drizzle-orm";

export interface SubjectGrade {
  subjectId: number;
  subjectName: string;
  participation: number;
  behavior: number;
  attendance: number;
  exam: number;
  homework: number;
  total: number;
  maxScore: number;
  grade: string; // e.g. "Excellent", "Good"
}

export interface StudentCertificate {
  studentId: string;
  studentName: string;
  studentNumber: string;
  grades: SubjectGrade[];
  totalScore: number;
  totalMaxScore: number;
  percentage: number;
  grade: string;
  isPassed: boolean;
  rank: number;
}

export interface CertificateBatch {
  classId: number;
  className: string;
  period: string; // e.g. "October", "Mid-term", "Final"
  certificates: StudentCertificate[];
}

const monthNames: Record<number, string> = {
  1: "January", 2: "February", 3: "March", 4: "April", 5: "May", 6: "June",
  7: "July", 8: "August", 9: "September", 10: "October", 11: "November", 12: "December",
  13: "Mid-Term Exam", 14: "Final Exam"
};

/**
 * Helper to calculate grade (تقدير) based on percentage
 */
function calculateGrade(percentage: number): string {
  if (percentage >= 90) return "Excellent (ممتاز)";
  if (percentage >= 80) return "Very Good (جيد جداً)";
  if (percentage >= 70) return "Good (جيد)";
  if (percentage >= 60) return "Acceptable (مقبول)";
  if (percentage >= 50) return "Weak (ضعيف)";
  return "Fail (راسب)";
}

/**
 * Common logic to process grades into certificates
 */
function processStudentGrades(
  enrolledStudents: { id: string, name: string | null, studentNumber: string }[],
  items: { id: number, type: string, maxScore: string, subjectId: number | null, subjectName: string }[],
  grades: { studentId: string | null, assessmentItemId: number | null, score: string | null }[]
): StudentCertificate[] {
  let certificates: StudentCertificate[] = enrolledStudents.map(student => {
    const studentGradesMap: Record<number, SubjectGrade> = {};

    items.forEach(item => {
      if (!studentGradesMap[item.subjectId!]) {
        studentGradesMap[item.subjectId!] = {
          subjectId: item.subjectId!,
          subjectName: item.subjectName,
          participation: 0,
          behavior: 0,
          attendance: 0,
          exam: 0,
          homework: 0,
          total: 0,
          maxScore: 0,
          grade: "",
        };
      }

      const gradeData = grades.find(g => g.studentId === student.id && g.assessmentItemId === item.id);
      const score = parseFloat(gradeData?.score || "0");
      const subGrade = studentGradesMap[item.subjectId!]!;

      switch (item.type) {
        case 'participation': subGrade.participation += score; break;
        case 'behavior': subGrade.behavior += score; break;
        case 'attendance': subGrade.attendance += score; break;
        case 'exam': subGrade.exam += score; break;
        case 'homework': subGrade.homework += score; break;
      }
      subGrade.total += score;
      subGrade.maxScore += parseFloat(item.maxScore);
    });

    const finalGrades = Object.values(studentGradesMap).map(g => ({
      ...g,
      grade: calculateGrade(g.maxScore > 0 ? (g.total / g.maxScore) * 100 : 0)
    }));

    const totalScore = finalGrades.reduce((sum, g) => sum + g.total, 0);
    const totalMaxScore = finalGrades.reduce((sum, g) => sum + g.maxScore, 0);
    const percentage = totalMaxScore > 0 ? (totalScore / totalMaxScore) * 100 : 0;

    return {
      studentId: student.id,
      studentName: student.name || "Unknown",
      studentNumber: student.studentNumber,
      grades: finalGrades,
      totalScore,
      totalMaxScore,
      percentage,
      grade: calculateGrade(percentage),
      isPassed: percentage >= 50,
      rank: 0,
    };
  });

  // Calculate Ranks
  certificates.sort((a, b) => b.totalScore - a.totalScore);
  certificates = certificates.map((c, index) => ({ ...c, rank: index + 1 }));

  return certificates;
}

/**
 * Generate certificates for a specific month
 */
export async function getMonthlyCertificates(classId: number, month: number, academicYearId: number): Promise<CertificateBatch> {
  const [classInfo] = await db.select().from(classes).where(eq(classes.id, classId)).limit(1);
  if (!classInfo) throw new Error("Class not found");

  const enrolledStudents = await db
    .select({
      id: students.userId,
      name: users.name,
      studentNumber: students.studentNumber,
    })
    .from(enrollments)
    .innerJoin(students, eq(enrollments.studentId, students.userId))
    .innerJoin(users, eq(students.userId, users.id))
    .where(
      and(
        eq(enrollments.classId, classId),
        eq(enrollments.academicYearId, academicYearId),
        eq(enrollments.status, "active")
      )
    );

  const studentIds = enrolledStudents.map(s => s.id);
  if (studentIds.length === 0) {
    return { classId, className: classInfo.name, period: monthNames[month] || `Month ${month}`, certificates: [] };
  }

  const items = await db
    .select({
      id: assessmentItems.id,
      title: assessmentItems.title,
      type: assessmentItems.type,
      maxScore: assessmentItems.maxScore,
      subjectId: assessmentItems.subjectId,
      subjectName: subjects.name,
    })
    .from(assessmentItems)
    .innerJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .where(
      and(
        eq(assessmentItems.classId, classId),
        eq(assessmentItems.month, month)
      )
    );

  const itemIds = items.map(i => i.id);
  const grades = itemIds.length > 0 ? await db
    .select({
      studentId: gradeEntries.studentId,
      assessmentItemId: gradeEntries.assessmentItemId,
      score: gradeEntries.score,
    })
    .from(gradeEntries)
    .where(
      and(
        inArray(gradeEntries.studentId, studentIds),
        inArray(gradeEntries.assessmentItemId, itemIds)
      )
    ) : [];

  return {
    classId,
    className: classInfo.name,
    period: monthNames[month] || `Month ${month}`,
    certificates: processStudentGrades(enrolledStudents, items, grades)
  };
}

/**
 * Generate certificates for a specific term (Semester)
 */
export async function getTermCertificates(classId: number, termId: number, academicYearId: number): Promise<CertificateBatch> {
  const monthsInTerm = termId === 1 ? [9, 10, 11, 13] : [12, 1, 2, 14]; 
  
  const [classInfo] = await db.select().from(classes).where(eq(classes.id, classId)).limit(1);
  if (!classInfo) throw new Error("Class not found");

  const enrolledStudents = await db
    .select({
      id: students.userId,
      name: users.name,
      studentNumber: students.studentNumber,
    })
    .from(enrollments)
    .innerJoin(students, eq(enrollments.studentId, students.userId))
    .innerJoin(users, eq(students.userId, users.id))
    .where(
      and(
        eq(enrollments.classId, classId),
        eq(enrollments.academicYearId, academicYearId),
        eq(enrollments.status, "active")
      )
    );

  const studentIds = enrolledStudents.map(s => s.id);
  if (studentIds.length === 0) return { classId, className: classInfo.name, period: `Term ${termId}`, certificates: [] };

  const items = await db
    .select({
      id: assessmentItems.id,
      title: assessmentItems.title,
      type: assessmentItems.type,
      maxScore: assessmentItems.maxScore,
      subjectId: assessmentItems.subjectId,
      subjectName: subjects.name,
    })
    .from(assessmentItems)
    .innerJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .where(
      and(
        eq(assessmentItems.classId, classId),
        eq(assessmentItems.termId, termId),
        inArray(assessmentItems.month, monthsInTerm)
      )
    );

  const itemIds = items.map(i => i.id);
  const grades = itemIds.length > 0 ? await db
    .select({
      studentId: gradeEntries.studentId,
      assessmentItemId: gradeEntries.assessmentItemId,
      score: gradeEntries.score,
    })
    .from(gradeEntries)
    .where(
      and(
        inArray(gradeEntries.studentId, studentIds),
        inArray(gradeEntries.assessmentItemId, itemIds)
      )
    ) : [];

  return {
    classId,
    className: classInfo.name,
    period: termId === 1 ? "Semester 1" : "Semester 2",
    certificates: processStudentGrades(enrolledStudents, items, grades)
  };
}

/**
 * Generate certificates for the whole academic year
 */
export async function getYearlyCertificates(classId: number, academicYearId: number): Promise<CertificateBatch> {
  const [classInfo] = await db.select().from(classes).where(eq(classes.id, classId)).limit(1);
  if (!classInfo) throw new Error("Class not found");

  const enrolledStudents = await db
    .select({
      id: students.userId,
      name: users.name,
      studentNumber: students.studentNumber,
    })
    .from(enrollments)
    .innerJoin(students, eq(enrollments.studentId, students.userId))
    .innerJoin(users, eq(students.userId, users.id))
    .where(
      and(
        eq(enrollments.classId, classId),
        eq(enrollments.academicYearId, academicYearId),
        eq(enrollments.status, "active")
      )
    );

  const studentIds = enrolledStudents.map(s => s.id);
  if (studentIds.length === 0) return { classId, className: classInfo.name, period: "Academic Year", certificates: [] };

  const terms = await db.select({ id: academicTerms.id }).from(academicTerms).where(eq(academicTerms.academicYearId, academicYearId));
  const termIds = terms.map(t => t.id);

  if (termIds.length === 0) return { classId, className: classInfo.name, period: "Academic Year", certificates: [] };

  const items = await db
    .select({
      id: assessmentItems.id,
      title: assessmentItems.title,
      type: assessmentItems.type,
      maxScore: assessmentItems.maxScore,
      subjectId: assessmentItems.subjectId,
      subjectName: subjects.name,
    })
    .from(assessmentItems)
    .innerJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .where(
      and(
        eq(assessmentItems.classId, classId),
        inArray(assessmentItems.termId, termIds)
      )
    );

  const itemIds = items.map(i => i.id);
  const grades = itemIds.length > 0 ? await db
    .select({
      studentId: gradeEntries.studentId,
      assessmentItemId: gradeEntries.assessmentItemId,
      score: gradeEntries.score,
    })
    .from(gradeEntries)
    .where(
      and(
        inArray(gradeEntries.studentId, studentIds),
        inArray(gradeEntries.assessmentItemId, itemIds)
      )
    ) : [];

  return {
    classId,
    className: classInfo.name,
    period: "Academic Year",
    certificates: processStudentGrades(enrolledStudents, items, grades)
  };
}

/**
 * Generate a single student certificate for a specific month
 */
export async function getStudentMonthlyCertificate(studentId: string, month: number, academicYearId: number): Promise<StudentCertificate | null> {
  const [student] = await db.select({ userId: students.userId, name: users.name, studentNumber: students.studentNumber, currentClassId: students.currentClassId }).from(students).innerJoin(users, eq(students.userId, users.id)).where(eq(students.userId, studentId)).limit(1);
  if (!student) return null;

  const batch = await getMonthlyCertificates(student.currentClassId!, month, academicYearId);
  return batch.certificates.find(c => c.studentId === studentId) || null;
}

/**
 * Generate a single student certificate for a specific term
 */
export async function getStudentTermCertificate(studentId: string, termId: number, academicYearId: number): Promise<StudentCertificate | null> {
  const [student] = await db.select({ userId: students.userId, name: users.name, studentNumber: students.studentNumber, currentClassId: students.currentClassId }).from(students).innerJoin(users, eq(students.userId, users.id)).where(eq(students.userId, studentId)).limit(1);
  if (!student) return null;

  const batch = await getTermCertificates(student.currentClassId!, termId, academicYearId);
  return batch.certificates.find(c => c.studentId === studentId) || null;
}

/**
 * Generate a single student certificate for the yearly academic year
 */
export async function getStudentYearlyCertificate(studentId: string, academicYearId: number): Promise<StudentCertificate | null> {
  const [student] = await db.select({ userId: students.userId, name: users.name, studentNumber: students.studentNumber, currentClassId: students.currentClassId }).from(students).innerJoin(users, eq(students.userId, users.id)).where(eq(students.userId, studentId)).limit(1);
  if (!student) return null;

  const batch = await getYearlyCertificates(student.currentClassId!, academicYearId);
  return batch.certificates.find(c => c.studentId === studentId) || null;
}

