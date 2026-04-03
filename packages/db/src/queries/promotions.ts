import { db } from "@db/client";
import {
  students,
  classes,
  users,
  enrollments,
  academicYears,
  gradeEntries,
  assessmentItems,
  subjects,
  studentPromotions,
  auditLogs,
} from "@db/schema";
import { eq, and, asc, inArray, isNull, desc, sql } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

// Cache instances to invalidate after promotion
const classCache = new RedisCache("classes", 60 * 60);
const studentCache = new RedisCache("student", 30 * 60);
const gradeCache = new RedisCache("grades", 5 * 60);

// ==========================================
// Helper: Grade calculation (same as grades.ts)
// ==========================================
function roundGrade(value: number): number {
  return Math.round(value);
}

function getGradeLabel(total: number, maxTotal: number): string {
  if (maxTotal <= 0) return "";
  const percentage = (total / maxTotal) * 100;
  if (percentage >= 90) return "ممتاز";
  if (percentage >= 80) return "جيد جداً";
  if (percentage >= 70) return "جيد";
  if (percentage >= 50) return "مقبول";
  return "راسب";
}

const PASS_THRESHOLD = 50; // 50%

// ==========================================
// 1. Calculate Final Score for a Student
// ==========================================
export const calculateStudentFinalScore = async (
  studentId: string,
  classId: number
): Promise<{
  finalScore: number;
  maxScore: number;
  percentage: number;
  gradeLabel: string;
  passed: boolean;
  subjectScores: {
    subject: string;
    total: number;
    maxTotal: number;
    percentage: number;
    passed: boolean;
  }[];
}> => {
  const term1Months = [1, 2, 3];
  const midtermMonth = 4;
  const term2Months = [5, 6, 7];
  const finalExamMonth = 8;

  // Get ALL assessment items for the class across the year
  const allItems = await db
    .select({
      id: assessmentItems.id,
      maxScore: assessmentItems.maxScore,
      month: assessmentItems.month,
      subjectId: assessmentItems.subjectId,
      subjectName: subjects.name,
    })
    .from(assessmentItems)
    .innerJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .where(
      and(
        eq(assessmentItems.classId, classId),
        inArray(assessmentItems.month, [
          ...term1Months,
          midtermMonth,
          ...term2Months,
          finalExamMonth,
        ])
      )
    )
    .orderBy(asc(subjects.sortOrder), asc(subjects.name), asc(assessmentItems.month));

  if (allItems.length === 0) {
    return {
      finalScore: 0,
      maxScore: 100,
      percentage: 0,
      gradeLabel: "",
      passed: false,
      subjectScores: [],
    };
  }

  // Get grades
  const itemIds = allItems.map((i) => i.id);
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

  const gradeMap = new Map(grades.map((g) => [g.assessmentItemId, g.score]));

  // Group by subject and calculate
  const subjectMap = new Map<
    string,
    {
      subject: string;
      term1Total: number;
      term1MaxTotal: number;
      midtermScore: number;
      midtermMaxScore: number;
      term2Total: number;
      term2MaxTotal: number;
      finalExamScore: number;
      finalExamMaxScore: number;
    }
  >();

  for (const item of allItems) {
    if (!subjectMap.has(item.subjectName)) {
      subjectMap.set(item.subjectName, {
        subject: item.subjectName,
        term1Total: 0,
        term1MaxTotal: 0,
        midtermScore: 0,
        midtermMaxScore: 0,
        term2Total: 0,
        term2MaxTotal: 0,
        finalExamScore: 0,
        finalExamMaxScore: 0,
      });
    }
    const row = subjectMap.get(item.subjectName)!;
    const scoreStr = gradeMap.get(item.id);
    const score =
      scoreStr ? Number.parseFloat(scoreStr) : 0;

    if (term1Months.includes(item.month)) {
      if (!Number.isNaN(score)) row.term1Total += score;
      row.term1MaxTotal += Number.parseFloat(item.maxScore || "0");
    } else if (item.month === midtermMonth) {
      if (!Number.isNaN(score)) row.midtermScore += score;
      row.midtermMaxScore += Number.parseFloat(item.maxScore || "0");
    } else if (term2Months.includes(item.month)) {
      if (!Number.isNaN(score)) row.term2Total += score;
      row.term2MaxTotal += Number.parseFloat(item.maxScore || "0");
    } else if (item.month === finalExamMonth) {
      if (!Number.isNaN(score)) row.finalExamScore += score;
      row.finalExamMaxScore += Number.parseFloat(item.maxScore || "0");
    }
  }

  const subjectScores: {
    subject: string;
    total: number;
    maxTotal: number;
    percentage: number;
    passed: boolean;
  }[] = [];

  let grandTotal = 0;
  let subjectCount = 0;

  for (const [, row] of subjectMap) {
    const scaledT1 =
      row.term1MaxTotal > 0
        ? roundGrade((row.term1Total / row.term1MaxTotal) * 20)
        : 0;
    const scaledMidterm =
      row.midtermMaxScore > 0
        ? roundGrade((row.midtermScore / row.midtermMaxScore) * 30)
        : 0;
    const scaledT2 =
      row.term2MaxTotal > 0
        ? roundGrade((row.term2Total / row.term2MaxTotal) * 20)
        : 0;
    const scaledFinal =
      row.finalExamMaxScore > 0
        ? roundGrade((row.finalExamScore / row.finalExamMaxScore) * 30)
        : 0;

    const total = roundGrade(scaledT1 + scaledMidterm + scaledT2 + scaledFinal);
    const maxTotal = 100;
    const percentage = total;

    subjectScores.push({
      subject: row.subject,
      total,
      maxTotal,
      percentage,
      passed: percentage >= PASS_THRESHOLD,
    });

    grandTotal += percentage;
    subjectCount++;
  }

  const overallPercentage =
    subjectCount > 0 ? roundGrade(grandTotal / subjectCount) : 0;

  return {
    finalScore: overallPercentage,
    maxScore: 100,
    percentage: overallPercentage,
    gradeLabel: getGradeLabel(overallPercentage, 100),
    passed: overallPercentage >= PASS_THRESHOLD,
    subjectScores,
  };
};

// ==========================================
// 2. Preview Promotion for a Class
// ==========================================
export const previewClassPromotion = async (params: {
  classId: number;
  academicYearId: number;
}) => {
  const { classId, academicYearId } = params;

  // Get child classes (sections) if any
  const childClasses = await db
    .select({ id: classes.id })
    .from(classes)
    .where(eq(classes.parentClassId, classId));

  const targetClassIds =
    childClasses.length > 0
      ? [classId, ...childClasses.map((c) => c.id)]
      : [classId];

  // Get the class info
  const [classInfo] = await db
    .select({
      id: classes.id,
      name: classes.name,
      sortOrder: classes.sortOrder,
    })
    .from(classes)
    .where(eq(classes.id, classId))
    .limit(1);

  if (!classInfo) throw new Error("الصف غير موجود");

  // Find the next class by sortOrder (classes are now permanent, no year filter needed)
  const [nextClass] = await db
    .select({
      id: classes.id,
      name: classes.name,
      sortOrder: classes.sortOrder,
    })
    .from(classes)
    .where(
      and(
        isNull(classes.parentClassId),
        isNull(classes.deletedAt),
        sql`${classes.sortOrder} > ${classInfo.sortOrder}`
      )
    )
    .orderBy(asc(classes.sortOrder))
    .limit(1);

  const isLastGrade = !nextClass; // e.g. الصف التاسع

  // Get enrolled students
  const enrolledStudents = await db
    .select({
      studentId: students.userId,
      studentNumber: students.studentNumber,
      name: users.name,
      image: users.image,
      sex: users.sex,
      currentClassId: students.currentClassId,
    })
    .from(enrollments)
    .innerJoin(students, eq(enrollments.studentId, students.userId))
    .innerJoin(users, eq(students.userId, users.id))
    .where(
      and(
        targetClassIds.length === 1
          ? eq(enrollments.classId, targetClassIds[0]!)
          : inArray(enrollments.classId, targetClassIds),
        eq(enrollments.academicYearId, academicYearId),
        eq(enrollments.status, "active")
      )
    )
    .orderBy(users.name);

  // Check if any student already has a promotion record for this year
  const existingPromotions = await db
    .select({
      studentId: studentPromotions.studentId,
      status: studentPromotions.status,
    })
    .from(studentPromotions)
    .where(
      and(
        inArray(
          studentPromotions.studentId,
          enrolledStudents.map((s) => s.studentId)
        ),
        eq(studentPromotions.fromAcademicYearId, academicYearId)
      )
    );

  const promotedStudentIds = new Set(
    existingPromotions.map((p) => p.studentId)
  );

  // Calculate scores for each student
  const results = [];

  for (const student of enrolledStudents) {
    const scoreData = await calculateStudentFinalScore(
      student.studentId,
      classId
    );

    const alreadyPromoted = promotedStudentIds.has(student.studentId);

    results.push({
      studentId: student.studentId,
      studentNumber: student.studentNumber,
      name: student.name,
      image: student.image,
      sex: student.sex,
      currentClassId: student.currentClassId,
      finalScore: scoreData.percentage,
      gradeLabel: scoreData.gradeLabel,
      passed: scoreData.passed,
      subjectScores: scoreData.subjectScores,
      suggestedStatus: alreadyPromoted
        ? ("already_promoted" as const)
        : isLastGrade && scoreData.passed
          ? ("graduated" as const)
          : scoreData.passed
            ? ("promoted" as const)
            : ("retained" as const),
      toClassId: scoreData.passed
        ? (!isLastGrade ? nextClass?.id : undefined)
        : classId,
      alreadyPromoted,
    });
  }

  // Sort: promoted first, then retained
  results.sort((a, b) => {
    if (a.passed !== b.passed) return a.passed ? -1 : 1;
    return (b.finalScore || 0) - (a.finalScore || 0);
  });

  return {
    classInfo: {
      id: classInfo.id,
      name: classInfo.name,
      sortOrder: classInfo.sortOrder,
    },
    nextClass: nextClass
      ? { id: nextClass.id, name: nextClass.name }
      : null,
    isLastGrade,
    academicYearId,
    students: results,
    summary: {
      total: results.length,
      passed: results.filter((r) => r.passed).length,
      failed: results.filter((r) => !r.passed).length,
      alreadyPromoted: results.filter((r) => r.alreadyPromoted).length,
      graduated: isLastGrade
        ? results.filter((r) => r.passed).length
        : 0,
    },
  };
};

// ==========================================
// 3. Execute Bulk Promotion
// ==========================================
export const executeBulkPromotion = async (params: {
  classId: number;
  fromAcademicYearId: number;
  toAcademicYearId: number;
  decisions: {
    studentId: string;
    status: "promoted" | "retained" | "graduated" | "conditional";
    toClassId?: number;
    notes?: string;
  }[];
  decisionById: string;
}) => {
  const {
    classId,
    fromAcademicYearId,
    toAcademicYearId,
    decisions,
    decisionById,
  } = params;

  // Validate the target year exists
  const [toYear] = await db
    .select()
    .from(academicYears)
    .where(eq(academicYears.id, toAcademicYearId))
    .limit(1);

  if (!toYear) throw new Error("السنة الدراسية المستهدفة غير موجودة");

  // Get class info to find next class
  const [classInfo] = await db
    .select({
      id: classes.id,
      name: classes.name,
      sortOrder: classes.sortOrder,
    })
    .from(classes)
    .where(eq(classes.id, classId))
    .limit(1);

  if (!classInfo) throw new Error("الصف غير موجود");

  // Classes are now permanent — find next class by sortOrder (no year-based lookup needed!)
  const [nextClass] = await db
    .select({
      id: classes.id,
      name: classes.name,
      sortOrder: classes.sortOrder,
    })
    .from(classes)
    .where(
      and(
        isNull(classes.parentClassId),
        isNull(classes.deletedAt),
        sql`${classes.sortOrder} > ${classInfo.sortOrder}`
      )
    )
    .orderBy(asc(classes.sortOrder))
    .limit(1);

  const results: {
    studentId: string;
    status: string;
    success: boolean;
    error?: string;
  }[] = [];

  for (const decision of decisions) {
    try {
      // Calculate final score
      const scoreData = await calculateStudentFinalScore(
        decision.studentId,
        classId
      );

      let targetClassId: number | null = null;

      if (decision.status === "promoted" || decision.status === "conditional") {
        // Promoted: move to next class (by sortOrder)
        targetClassId = decision.toClassId || nextClass?.id || null;

        if (!targetClassId) {
          results.push({
            studentId: decision.studentId,
            status: "error",
            success: false,
            error: "لم يتم العثور على الصف التالي",
          });
          continue;
        }
      } else if (decision.status === "retained") {
        // Retained: stay in SAME class (which is permanent), just new enrollment year
        targetClassId = classId;
      }
      // For graduated: targetClassId remains null

      // 1. Insert promotion record
      await db
        .insert(studentPromotions)
        .values({
          studentId: decision.studentId,
          fromClassId: classId,
          toClassId: targetClassId,
          fromAcademicYearId: fromAcademicYearId,
          toAcademicYearId: toAcademicYearId,
          finalScore: scoreData.percentage.toString(),
          status: decision.status,
          decision:
            decision.status === "conditional" || decision.status === "retained"
              ? "manual"
              : scoreData.passed
                ? "auto"
                : "manual",
          decisionById,
          notes: decision.notes || null,
        })
        .onConflictDoUpdate({
          target: [
            studentPromotions.studentId,
            studentPromotions.fromAcademicYearId,
          ],
          set: {
            toClassId: targetClassId,
            toAcademicYearId: toAcademicYearId,
            finalScore: scoreData.percentage.toString(),
            status: decision.status,
            decision:
              decision.status === "conditional" || decision.status === "retained"
                ? "manual"
                : scoreData.passed
                  ? "auto"
                  : "manual",
            decisionById,
            notes: decision.notes || null,
            promotedAt: new Date(),
          },
        });

      // 2. Mark OLD enrollment as 'transferred' (preserves history, fixes student counts)
      await db
        .update(enrollments)
        .set({ status: "transferred" })
        .where(
          and(
            eq(enrollments.studentId, decision.studentId),
            eq(enrollments.classId, classId),
            eq(enrollments.academicYearId, fromAcademicYearId),
            eq(enrollments.status, "active")
          )
        );

      // 3. Update student's current class (if not graduated)
      if (decision.status === "graduated") {
        await db
          .update(students)
          .set({ status: "graduated" })
          .where(eq(students.userId, decision.studentId));
      } else if (targetClassId) {
        await db
          .update(students)
          .set({ currentClassId: targetClassId })
          .where(eq(students.userId, decision.studentId));

        // 4. Create enrollment in new year
        await db
          .insert(enrollments)
          .values({
            studentId: decision.studentId,
            classId: targetClassId,
            academicYearId: toAcademicYearId,
            status: "active",
          })
          .onConflictDoNothing();
      }

      // 4. Audit log
      await db.insert(auditLogs).values({
        action: "promote",
        tableName: "student_promotions",
        recordId: decision.studentId,
        oldData: { classId, academicYearId: fromAcademicYearId },
        newData: {
          toClassId: targetClassId,
          toAcademicYearId: toAcademicYearId,
          status: decision.status,
          finalScore: scoreData.percentage,
        },
        userId: decisionById,
      });

      results.push({
        studentId: decision.studentId,
        status: decision.status,
        success: true,
      });
      // Invalidate caches for this student
      await studentCache.delete(`user:${decision.studentId}`).catch(() => {});
      await gradeCache.delete(`student:${decision.studentId}:all`).catch(() => {});

    } catch (error) {
      results.push({
        studentId: decision.studentId,
        status: "error",
        success: false,
        error: (error as Error).message,
      });
    }
  }

  // Invalidate class caches globally after bulk promotion
  await classCache.delete(`classes_v3:${fromAcademicYearId}:none:default:false:undefined:0`).catch(() => {});
  await classCache.delete(`classes_v3:${toAcademicYearId}:none:default:false:undefined:0`).catch(() => {});
  await classCache.delete(`classes_v3:all:none:default:false:undefined:0`).catch(() => {});

  return {
    total: results.length,
    successful: results.filter((r) => r.success).length,
    failed: results.filter((r) => !r.success).length,
    results,
  };
};

// ==========================================
// 4. Rollback Promotion
// ==========================================
export const rollbackPromotion = async (params: {
  classId: number;
  academicYearId: number;
  studentIds?: string[]; // Optional: specific students, or all
  rollbackById: string;
}) => {
  const { classId, academicYearId, studentIds, rollbackById } = params;

  // Get promotions to rollback
  const conditions = [
    eq(studentPromotions.fromClassId, classId),
    eq(studentPromotions.fromAcademicYearId, academicYearId),
  ];

  if (studentIds?.length) {
    conditions.push(inArray(studentPromotions.studentId, studentIds));
  }

  const promotions = await db
    .select()
    .from(studentPromotions)
    .where(and(...conditions));

  if (promotions.length === 0) {
    throw new Error("لا توجد عمليات ترفيع للتراجع عنها");
  }

  const results: {
    studentId: string;
    success: boolean;
    error?: string;
  }[] = [];

  for (const promo of promotions) {
    try {
      // 1. Revert student's current class
      await db
        .update(students)
        .set({
          currentClassId: promo.fromClassId,
          status: "active",
        })
        .where(eq(students.userId, promo.studentId));

      // 2. Restore old enrollment back to active
      await db
        .update(enrollments)
        .set({ status: "active" })
        .where(
          and(
            eq(enrollments.studentId, promo.studentId),
            eq(enrollments.classId, promo.fromClassId),
            eq(enrollments.academicYearId, promo.fromAcademicYearId)
          )
        );

      // 3. Delete new enrollment (if exists)
      if (promo.toClassId) {
        await db
          .delete(enrollments)
          .where(
            and(
              eq(enrollments.studentId, promo.studentId),
              eq(enrollments.classId, promo.toClassId),
              eq(enrollments.academicYearId, promo.toAcademicYearId)
            )
          );
      }

      // 4. Delete promotion record
      await db
        .delete(studentPromotions)
        .where(eq(studentPromotions.id, promo.id));

      // 5. Audit log
      await db.insert(auditLogs).values({
        action: "rollback_promote",
        tableName: "student_promotions",
        recordId: promo.studentId,
        oldData: {
          toClassId: promo.toClassId,
          status: promo.status,
        },
        newData: {
          restoredTo: promo.fromClassId,
        },
        userId: rollbackById,
      });

      // 6. Invalidate caches
      await studentCache.delete(`user:${promo.studentId}`).catch(() => {});
      await gradeCache.delete(`student:${promo.studentId}:all`).catch(() => {});

      results.push({ studentId: promo.studentId, success: true });
    } catch (error) {
      results.push({
        studentId: promo.studentId,
        success: false,
        error: (error as Error).message,
      });
    }
  }

  return {
    total: results.length,
    successful: results.filter((r) => r.success).length,
    failed: results.filter((r) => !r.success).length,
    results,
  };
};

// ==========================================
// 5. Get Promotion History
// ==========================================
export const getPromotionHistory = async (params: {
  academicYearId?: number;
  classId?: number;
  studentId?: string;
  status?: string;
  limit?: number;
  offset?: number;
}) => {
  const { academicYearId, classId, studentId, status, limit = 50, offset = 0 } = params;

  const conditions = [];

  if (academicYearId) {
    conditions.push(
      eq(studentPromotions.fromAcademicYearId, academicYearId)
    );
  }
  if (classId) {
    conditions.push(eq(studentPromotions.fromClassId, classId));
  }
  if (studentId) {
    conditions.push(eq(studentPromotions.studentId, studentId));
  }
  if (status) {
    conditions.push(
      eq(studentPromotions.status, status as any)
    );
  }

  const results = await db
    .select({
      id: studentPromotions.id,
      studentId: studentPromotions.studentId,
      studentName: users.name,
      studentNumber: students.studentNumber,
      fromClassId: studentPromotions.fromClassId,
      toClassId: studentPromotions.toClassId,
      fromAcademicYearId: studentPromotions.fromAcademicYearId,
      toAcademicYearId: studentPromotions.toAcademicYearId,
      finalScore: studentPromotions.finalScore,
      status: studentPromotions.status,
      decision: studentPromotions.decision,
      notes: studentPromotions.notes,
      promotedAt: studentPromotions.promotedAt,
      fromClassName: sql<string>`(SELECT name FROM classes WHERE id = ${studentPromotions.fromClassId})`,
      toClassName: sql<string | null>`(SELECT name FROM classes WHERE id = ${studentPromotions.toClassId})`,
      fromYearName: sql<string>`(SELECT year_name FROM academic_years WHERE id = ${studentPromotions.fromAcademicYearId})`,
      toYearName: sql<string>`(SELECT year_name FROM academic_years WHERE id = ${studentPromotions.toAcademicYearId})`,
    })
    .from(studentPromotions)
    .innerJoin(students, eq(studentPromotions.studentId, students.userId))
    .innerJoin(users, eq(students.userId, users.id))
    .where(conditions.length > 0 ? and(...conditions) : undefined)
    .orderBy(desc(studentPromotions.promotedAt))
    .limit(limit)
    .offset(offset);

  return results;
};

// ==========================================
// 6. Promotion Statistics
// ==========================================
export const getPromotionStatistics = async (params: {
  academicYearId: number;
  classId?: number;
}) => {
  const { academicYearId, classId } = params;

  const conditions = [
    eq(studentPromotions.fromAcademicYearId, academicYearId),
  ];
  if (classId) {
    conditions.push(eq(studentPromotions.fromClassId, classId));
  }

  const allPromotions = await db
    .select({
      status: studentPromotions.status,
      finalScore: studentPromotions.finalScore,
      fromClassId: studentPromotions.fromClassId,
      fromClassName: sql<string>`(SELECT name FROM classes WHERE id = ${studentPromotions.fromClassId})`,
    })
    .from(studentPromotions)
    .where(and(...conditions));

  const total = allPromotions.length;
  const promoted = allPromotions.filter((p) => p.status === "promoted").length;
  const retained = allPromotions.filter((p) => p.status === "retained").length;
  const graduated = allPromotions.filter((p) => p.status === "graduated").length;
  const conditional = allPromotions.filter((p) => p.status === "conditional").length;

  const scores = allPromotions
    .map((p) => Number.parseFloat(p.finalScore || "0"))
    .filter((s) => !Number.isNaN(s) && s > 0);

  const averageScore =
    scores.length > 0
      ? scores.reduce((a, b) => a + b, 0) / scores.length
      : 0;

  const highestScore = scores.length > 0 ? Math.max(...scores) : 0;
  const lowestScore = scores.length > 0 ? Math.min(...scores) : 0;

  // Per-class breakdown
  const classBreakdown = new Map<
    number,
    { className: string; total: number; promoted: number; retained: number; graduated: number }
  >();

  for (const p of allPromotions) {
    if (!classBreakdown.has(p.fromClassId)) {
      classBreakdown.set(p.fromClassId, {
        className: p.fromClassName || "",
        total: 0,
        promoted: 0,
        retained: 0,
        graduated: 0,
      });
    }
    const entry = classBreakdown.get(p.fromClassId)!;
    entry.total++;
    if (p.status === "promoted" || p.status === "conditional") entry.promoted++;
    else if (p.status === "retained") entry.retained++;
    else if (p.status === "graduated") entry.graduated++;
  }

  return {
    total,
    promoted,
    retained,
    graduated,
    conditional,
    passRate: total > 0 ? roundGrade((promoted + graduated + conditional) / total * 100) : 0,
    averageScore: roundGrade(averageScore),
    highestScore: roundGrade(highestScore),
    lowestScore: roundGrade(lowestScore),
    classBreakdown: Array.from(classBreakdown.values()),
  };
};

// ==========================================
// 7. Override Single Student Promotion
// ==========================================
export const overrideStudentPromotion = async (params: {
  studentId: string;
  fromClassId: number;
  toClassId: number;
  fromAcademicYearId: number;
  toAcademicYearId: number;
  status: "promoted" | "retained" | "graduated" | "conditional";
  decisionById: string;
  notes?: string;
}) => {
  const {
    studentId,
    fromClassId,
    toClassId,
    fromAcademicYearId,
    toAcademicYearId,
    status,
    decisionById,
    notes,
  } = params;

  const scoreData = await calculateStudentFinalScore(studentId, fromClassId);

  // Upsert promotion record
  const [result] = await db
    .insert(studentPromotions)
    .values({
      studentId,
      fromClassId,
      toClassId: status === "graduated" ? null : toClassId,
      fromAcademicYearId,
      toAcademicYearId,
      finalScore: scoreData.percentage.toString(),
      status,
      decision: "override",
      decisionById,
      notes: notes || null,
    })
    .onConflictDoUpdate({
      target: [
        studentPromotions.studentId,
        studentPromotions.fromAcademicYearId,
      ],
      set: {
        toClassId: status === "graduated" ? null : toClassId,
        toAcademicYearId,
        finalScore: scoreData.percentage.toString(),
        status,
        decision: "override",
        decisionById,
        notes: notes || null,
        promotedAt: new Date(),
      },
    })
    .returning();

  // Update student
  if (status === "graduated") {
    await db
      .update(students)
      .set({ status: "graduated" })
      .where(eq(students.userId, studentId));
  } else {
    const targetClassId = status === "retained" ? fromClassId : toClassId;
    await db
      .update(students)
      .set({ currentClassId: targetClassId })
      .where(eq(students.userId, studentId));

    // Create enrollment
    await db
      .insert(enrollments)
      .values({
        studentId,
        classId: targetClassId,
        academicYearId: toAcademicYearId,
        status: "active",
      })
      .onConflictDoNothing();
  }

  // Audit
  await db.insert(auditLogs).values({
    action: "override_promote",
    tableName: "student_promotions",
    recordId: studentId,
    newData: {
      toClassId,
      status,
      finalScore: scoreData.percentage,
      decision: "override",
    },
    userId: decisionById,
  });

  return result;
};
