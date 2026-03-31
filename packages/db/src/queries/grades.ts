import { db } from "@db/client";
import { gradeEntries, assessmentItems, students, subjects, classes, enrollments, users, academicTerms, academicYears } from "@db/schema";
import { eq, and, desc, asc, inArray } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

// Cache for grades - maybe shorter TTL as they change
const gradeCache = new RedisCache("grades", 5 * 60);

export const getStudentGrades = async (studentId: string, termId?: number) => {
  const cacheKey = `student:${studentId}:${termId || 'all'}`;
  const cached = await gradeCache.get<any[]>(cacheKey);
  
  if (cached) return cached;

  // Build query
  const query = db
    .select({
      id: gradeEntries.id,
      score: gradeEntries.score,
      isAbsent: gradeEntries.isAbsent,
      comments: gradeEntries.comments,
      assessment: {
        title: assessmentItems.title,
        maxScore: assessmentItems.maxScore,
        weight: assessmentItems.weight,
        type: assessmentItems.type,
      },
      subject: {
        name: subjects.name,
        code: subjects.code,
        color: subjects.color
      }
    })
    .from(gradeEntries)
    .innerJoin(assessmentItems, eq(gradeEntries.assessmentItemId, assessmentItems.id))
    .innerJoin(subjects, eq(assessmentItems.subjectId, subjects.id))
    .where(eq(gradeEntries.studentId, studentId))
    .orderBy(asc(subjects.sortOrder), asc(subjects.name), asc(assessmentItems.month), asc(assessmentItems.id));

  const results = await query;
  
  if (results) {
    await gradeCache.set(cacheKey, results);
  }

  return results;
};

export type CreateGradeParams = typeof gradeEntries.$inferInsert;
export const createGradeEntry = async (data: CreateGradeParams) => {
    const [result] = await db.insert(gradeEntries).values(data).returning();
    if (result) await gradeCache.delete(`student:${result.studentId}:all`);
    return result;
};

export const updateGradeEntry = async (id: number, data: Partial<CreateGradeParams>) => {
    const [result] = await db.update(gradeEntries).set(data).where(eq(gradeEntries.id, id)).returning();
    if (result) await gradeCache.delete(`student:${result.studentId}:all`);
    return result;
};


export const bulkImportGrades = async (assessmentItemId: number, gradeData: {
    studentNumber: string;
    score: number | null;
    isAbsent?: boolean;
    comments?: string;
}[], gradedById?: string) => {
    const results = [];

    for (const item of gradeData) {
        try {
            // 1. Find student by studentNumber
            const [student] = await db.select().from(students).where(eq(students.studentNumber, item.studentNumber)).limit(1);
            
            if (!student) {
                results.push({ studentNumber: item.studentNumber, status: 'error', error: 'Student not found' });
                continue;
            }

            // 2. Upsert grade entry
            // We use studentId and assessmentItemId as unique key
            const [result] = await db.insert(gradeEntries).values({
                studentId: student.userId,
                assessmentItemId,
                score: item.score?.toString(),
                isAbsent: item.isAbsent,
                comments: item.comments,
                gradedById,
                updatedAt: new Date(),
            }).onConflictDoUpdate({
                target: [gradeEntries.studentId, gradeEntries.assessmentItemId],
                set: {
                    score: item.score?.toString(),
                    isAbsent: item.isAbsent,
                    comments: item.comments,
                    gradedById,
                    updatedAt: new Date(),
                }
            }).returning();

            results.push({ studentNumber: item.studentNumber, status: 'success' });
            await gradeCache.delete(`student:${student.userId}:all`);
        } catch (error) {
            results.push({ studentNumber: item.studentNumber, status: 'error', error: (error as Error).message });
        }
    }

    return results;
};

export const upsertGrade = async (data: {
  studentId: string;
  assessmentItemId: number;
  score: string | null;
  gradedById: string;
}) => {
  const [result] = await db
    .insert(gradeEntries)
    .values({
      studentId: data.studentId,
      assessmentItemId: data.assessmentItemId,
      score: data.score,
      gradedById: data.gradedById,
      updatedAt: new Date(),
    })
    .onConflictDoUpdate({
      target: [gradeEntries.studentId, gradeEntries.assessmentItemId],
      set: {
        score: data.score,
        gradedById: data.gradedById,
        updatedAt: new Date(),
      },
    })
    .returning();

  if (result) {
    await gradeCache.delete(`student:${data.studentId}:all`);
  }

  return result;
};

export const batchUpsertGrades = async (
  grades: {
    studentId: string;
    assessmentItemId: number;
    score: string | null;
  }[],
  gradedById: string
) => {
  if (grades.length === 0) return [];

  const results = [];
  const studentIdsToInvalidate = new Set<string>();

  // Process in chunks if necessary, but for a class size (~50-100) it's fine
  for (const grade of grades) {
    const [result] = await db
      .insert(gradeEntries)
      .values({
        studentId: grade.studentId,
        assessmentItemId: grade.assessmentItemId,
        score: grade.score,
        gradedById,
        updatedAt: new Date(),
      })
      .onConflictDoUpdate({
        target: [gradeEntries.studentId, gradeEntries.assessmentItemId],
        set: {
          score: grade.score,
          gradedById,
          updatedAt: new Date(),
        },
      })
      .returning();
    
    if (result) {
      results.push(result);
      studentIdsToInvalidate.add(grade.studentId);
    }
  }

  // Invalidate cache for all affected students
  for (const studentId of studentIdsToInvalidate) {
    await gradeCache.delete(`student:${studentId}:all`);
  }

  return results;
};



export const getGradeGridData = async (params: {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
  academicYearId: number;
}) => {
  try {
    const childClasses = await db
      .select({ id: classes.id })
      .from(classes)
      .where(eq(classes.parentClassId, params.classId));

    const targetClassIds = childClasses.length > 0 
      ? [params.classId, ...childClasses.map(c => c.id)] 
      : [params.classId];
    const isMidTerm = params.month === 4;
    const isFinal = params.month === 8;
    
    let monthsToFetch: number | number[] = params.month;
    if (isMidTerm) {
      monthsToFetch = [1, 2, 3, 4];
    } else if (isFinal) {
      monthsToFetch = [5, 6, 7, 8];
    }


    const monthCondition = Array.isArray(monthsToFetch) 
      ? inArray(assessmentItems.month, monthsToFetch)
      : eq(assessmentItems.month, monthsToFetch as number);

    const itemsList = await db
      .select()
      .from(assessmentItems)
      .where(
        and(
          eq(assessmentItems.classId, params.classId),
          eq(assessmentItems.subjectId, params.subjectId),
          eq(assessmentItems.termId, params.termId),
          monthCondition
        )
      )
      .orderBy(asc(assessmentItems.month), asc(assessmentItems.id));

    // 2. Fetch Students enrolled in target classes
    const enrolledStudents = await db
      .select({
        studentId: students.userId,
        studentNumber: students.studentNumber,
        name: users.name,
        image: users.image,
        sex: users.sex,
      })
      .from(enrollments)
      .innerJoin(students, eq(enrollments.studentId, students.userId))
      .innerJoin(users, eq(students.userId, users.id))
      .where(
        and(
          targetClassIds.length === 1 
            ? eq(enrollments.classId, targetClassIds[0]!)
            : inArray(enrollments.classId, targetClassIds),
          eq(enrollments.academicYearId, params.academicYearId),
          eq(enrollments.status, "active")
        )
      )
      .orderBy(users.name);

    // 3. Fetch Grades using standard query builder
    let gradesList: (typeof gradeEntries.$inferSelect)[] = [];

    if (itemsList.length > 0) {
      const itemIds = itemsList.map((i) => i.id);
      gradesList = await db
        .select()
        .from(gradeEntries)
        .where(inArray(gradeEntries.assessmentItemId, itemIds));
    }

    return {
      items: itemsList,
      students: enrolledStudents,
      grades: gradesList,
      month: params.month,
    };
  } catch (error) {
    console.error("Error in getGradeGridData:", error);
    console.error("Params:", params);
    throw error;
  }
};

export const getClassStatistics = async (classId: string, academicYearId?: number) => {
  const cacheKey = `class:${classId}:statistics:${academicYearId || 'current'}`;
  const cached = await gradeCache.get<{
    totalCount: number;
    passedCount: number;
    failedCount: number;
    averageGrade: number;
  }>(cacheKey);

  if (cached) return cached;

  const classIdNum = Number.parseInt(classId);

  // Get the current academic year if not provided
  let yearId = academicYearId;
  if (!yearId) {
      const [currentYear] = await db
          .select({ id: academicYears.id })
          .from(academicYears)
          .where(eq(academicYears.isCurrent, true))
          .limit(1);
      
      yearId = currentYear?.id;
  }

  // Get child classes (sections) if any
  const childClasses = await db
    .select({ id: classes.id })
    .from(classes)
    .where(eq(classes.parentClassId, classIdNum));

  const targetClassIds = childClasses.length > 0
    ? [classIdNum, ...childClasses.map(c => c.id)]
    : [classIdNum];

  // Get all enrollments for this class for the targeted year
  const enrolledStudents = await db
    .select({
      studentId: students.userId,
    })
    .from(enrollments)
    .innerJoin(students, eq(enrollments.studentId, students.userId))
    .where(
      and(
        targetClassIds.length === 1
          ? eq(enrollments.classId, targetClassIds[0]!)
          : inArray(enrollments.classId, targetClassIds),
        yearId ? eq(enrollments.academicYearId, yearId) : undefined,
      )
    );

  const totalCount = enrolledStudents.length;

  if (totalCount === 0) {
    const emptyResult = {
      totalCount: 0,
      passedCount: 0,
      failedCount: 0,
      averageGrade: 0,
    };
    await gradeCache.set(cacheKey, emptyResult);
    return emptyResult;
  }

  // Get assessment items for THIS class and THIS year only
  const classAssessmentItems = await db
    .select({ id: assessmentItems.id })
    .from(assessmentItems)
    .innerJoin(academicTerms, eq(assessmentItems.termId, academicTerms.id))
    .where(
      and(
        eq(assessmentItems.classId, classIdNum),
        yearId ? eq(academicTerms.academicYearId, yearId) : undefined
      )
    );

  if (classAssessmentItems.length === 0) {
    const emptyResult = {
      totalCount,
      passedCount: 0,
      failedCount: 0,
      averageGrade: 0,
    };
    await gradeCache.set(cacheKey, emptyResult);
    return emptyResult;
  }

  // Get grades scoped to THIS class's assessment items
  const studentIds = enrolledStudents.map((s) => s.studentId);
  const assessmentItemIds = classAssessmentItems.map(a => a.id);
  
  const studentGrades = await db
    .select({
      studentId: gradeEntries.studentId,
      score: gradeEntries.score,
      maxScore: assessmentItems.maxScore,
    })
    .from(gradeEntries)
    .innerJoin(assessmentItems, eq(gradeEntries.assessmentItemId, assessmentItems.id))
    .where(
      and(
        inArray(gradeEntries.studentId, studentIds),
        inArray(gradeEntries.assessmentItemId, assessmentItemIds)
      )
    );

  // Calculate statistics per student
  const studentStats = new Map<string, { totalScore: number; maxTotalScore: number }>();

  for (const grade of studentGrades) {
    if (!grade.studentId || !grade.score || grade.score === "null" || grade.score === "") continue;
    
    const score = Number.parseFloat(grade.score || "0");
    if (Number.isNaN(score)) continue;

    const existing = studentStats.get(grade.studentId) || { totalScore: 0, maxTotalScore: 0 };
    existing.totalScore += score;
    existing.maxTotalScore += Number.parseFloat(grade.maxScore || "100");
    studentStats.set(grade.studentId, existing);
  }

  let passedCount = 0;
  let failedCount = 0;
  let totalPercentage = 0;
  let studentsWithGrades = 0;

  for (const [, stats] of studentStats) {
    if (stats.maxTotalScore === 0) continue;
    
    const percentage = (stats.totalScore / stats.maxTotalScore) * 100;
    totalPercentage += percentage;
    studentsWithGrades++;

    if (percentage >= 50) {
      passedCount++;
    } else {
      failedCount++;
    }
  }

  const averageGrade = studentsWithGrades > 0 ? totalPercentage / studentsWithGrades : 0;

  const result = {
    totalCount,
    passedCount,
    failedCount,
    averageGrade,
  };

  await gradeCache.set(cacheKey, result);
  return result;
};

// ==========================================
// Helper: Calculate grade label from percentage
// ==========================================
function roundGrade(value: number): number {
  return Math.round(value * 2) / 2;
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

// ==========================================
// Student Report Data - Complete Rewrite
// ==========================================
// Academic Year Structure:
//   Term 1: Month 1 (محرم), Month 2 (صفر), Month 3 (ربيع أول), Month 4 (ربيع ثاني - اختبار نصفي)
//   Term 2: Month 5 (جمادى أول), Month 6 (جمادى ثاني), Month 7 (رجب), Month 8 (شعبان - اختبار نهائي)
//
// Certificate Types:
//   1. monthly: Shows assessmentItems for a single month (1,2,3 or 5,6,7) + total + grade
//   2. half-yearly (نصفي): Shows محصلة 3 شهور + درجة اختبار نصفي + المجموع + التقدير
//   3. final (نهائي): Shows محصلة الترم1 + اختبار النصفي + محصلة الترم2 + اختبار النهائي + المجموع + التقدير
// ==========================================

export const getStudentReportData = async (studentId: string, type?: string, month?: number, targetClassId?: number) => {
    // 1. Resolve classId: use provided classId (for historical reports) or fall back to currentClassId
    let classId = targetClassId;

    if (!classId) {
      const [student] = await db
        .select({ currentClassId: students.currentClassId })
        .from(students)
        .where(eq(students.userId, studentId))
        .limit(1);

      if (!student?.currentClassId) return { rows: [], columns: [], type: type || "monthly" };
      classId = student.currentClassId;
    }

    // ==========================================
    // MONTHLY CERTIFICATE (شهادة شهرية)
    // ==========================================
    if (type === "monthly" || !type) {
      const targetMonth = month || 1;
      
      // Get assessment items for this specific month
      const monthItems = await db
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
            eq(assessmentItems.month, targetMonth),
          )
        )
        .orderBy(asc(subjects.sortOrder), asc(subjects.name), asc(assessmentItems.id));

      if (monthItems.length === 0) return { rows: [], columns: [], type: "monthly" };

      // Get grades
      const itemIds = monthItems.map(i => i.id);
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

      const gradeMap = new Map(grades.map(g => [g.assessmentItemId, g.score]));

      // Group by subject → show each assessmentItem as a column
      const subjectMap = new Map<string, { subject: string; grades: Record<string, number | null>; total: number; maxTotal: number }>();
      const columnsSet = new Set<string>();

      for (const item of monthItems) {
        if (!subjectMap.has(item.subjectName)) {
          subjectMap.set(item.subjectName, { subject: item.subjectName, grades: {}, total: 0, maxTotal: 0 });
        }
        const row = subjectMap.get(item.subjectName)!;
        columnsSet.add(item.title);

        const scoreStr = gradeMap.get(item.id);
        const score = scoreStr ? parseFloat(scoreStr) : null;
        row.grades[item.title] = score;
        if (score !== null && !isNaN(score)) {
          row.total += score;
        }
        row.maxTotal += parseFloat(item.maxScore || "0");
      }

      const rows = Array.from(subjectMap.values()).map(row => {
        const total = row.maxTotal > 0 ? roundGrade((row.total / row.maxTotal) * 100) : 0;
        return {
          subject: row.subject,
          grades: row.grades,
          total,
          maxTotal: 100,
          gradeLabel: getGradeLabel(total, 100),
        };
      });

      return {
        rows,
        columns: Array.from(columnsSet),
        type: "monthly",
      };
    }

    // ==========================================
    // HALF-YEARLY CERTIFICATE (شهادة نصف العام)
    // Months 1,2,3 → محصلة الشهور | Month 4 → اختبار نصفي | المجموع | التقدير
    // ==========================================
    if (type === "half-yearly") {
      const monthlyMonths = [1, 2, 3]; // الشهور الثلاثة الأولى
      const examMonth = 4; // شهر الاختبار النصفي

      // Get all assessment items for months 1-4
      const allItems = await db
        .select({
          id: assessmentItems.id,
          title: assessmentItems.title,
          type: assessmentItems.type,
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
            inArray(assessmentItems.month, [...monthlyMonths, examMonth]),
          )
        )
        .orderBy(asc(subjects.sortOrder), asc(subjects.name), asc(assessmentItems.month), asc(assessmentItems.id));

      if (allItems.length === 0) return { rows: [], columns: ["محصلة الشهور", "اختبار نصف العام", "المجموع", "التقدير"], type: "half-yearly" };

      // Get grades
      const itemIds = allItems.map(i => i.id);
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

      const gradeMap = new Map(grades.map(g => [g.assessmentItemId, g.score]));

      // Group by subject
      const subjectMap = new Map<string, {
        subject: string;
        monthlyTotal: number;
        monthlyMaxTotal: number;
        examScore: number | null;
        examMaxScore: number;
      }>();

      for (const item of allItems) {
        if (!subjectMap.has(item.subjectName)) {
          subjectMap.set(item.subjectName, {
            subject: item.subjectName,
            monthlyTotal: 0,
            monthlyMaxTotal: 0,
            examScore: null,
            examMaxScore: 0,
          });
        }
        const row = subjectMap.get(item.subjectName)!;
        const scoreStr = gradeMap.get(item.id);
        const score = scoreStr ? parseFloat(scoreStr) : null;

        if (monthlyMonths.includes(item.month)) {
          // Accumulate monthly grades
          if (score !== null && !isNaN(score)) {
            row.monthlyTotal += score;
          }
          row.monthlyMaxTotal += parseFloat(item.maxScore || "0");
        } else if (item.month === examMonth) {
          // Midterm exam score
          if (score !== null && !isNaN(score)) {
            row.examScore = (row.examScore || 0) + score;
          }
          row.examMaxScore += parseFloat(item.maxScore || "0");
        }
      }

      const rows = Array.from(subjectMap.values()).map(row => {
        const scaledMonthly = row.monthlyMaxTotal > 0 ? roundGrade((row.monthlyTotal / row.monthlyMaxTotal) * 20) : 0;
        const scaledExam = row.examMaxScore > 0 ? roundGrade(((row.examScore || 0) / row.examMaxScore) * 30) : 0;
        const total = roundGrade(scaledMonthly + scaledExam);
        const maxTotal = 50;
        
        return {
          subject: row.subject,
          grades: {
            "محصلة الشهور": scaledMonthly,
            "اختبار نصف العام": scaledExam,
          } as Record<string, number | null>,
          total,
          maxTotal,
          gradeLabel: getGradeLabel(total, maxTotal),
        };
      });

      return {
        rows,
        columns: ["محصلة الشهور", "اختبار نصف العام"],
        type: "half-yearly",
      };
    }

    // ==========================================
    // FINAL CERTIFICATE (شهادة نهاية العام)
    // محصلة الترم الأول (1,2,3) | اختبار نصف العام (4) | محصلة الترم الثاني (5,6,7) | الاختبار النهائي (8) | المجموع | التقدير
    // ==========================================
    if (type === "final") {
      const term1Months = [1, 2, 3];
      const midtermMonth = 4;
      const term2Months = [5, 6, 7];
      const finalExamMonth = 8;

      // Get ALL assessment items across the entire year (months 1-8)
      const allItems = await db
        .select({
          id: assessmentItems.id,
          title: assessmentItems.title,
          type: assessmentItems.type,
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
            inArray(assessmentItems.month, [...term1Months, midtermMonth, ...term2Months, finalExamMonth]),
          )
        )
        .orderBy(asc(subjects.sortOrder), asc(subjects.name), asc(assessmentItems.month), asc(assessmentItems.id));

      if (allItems.length === 0) {
        return {
          rows: [],
          columns: ["المحصلة الأولى", "اختبار نصف العام", "المحصلة الثانية", "الاختبار النهائي"],
          type: "final",
        };
      }

      // Get grades
      const itemIds = allItems.map(i => i.id);
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

      const gradeMap = new Map(grades.map(g => [g.assessmentItemId, g.score]));

      // Group by subject
      const subjectMap = new Map<string, {
        subject: string;
        term1Total: number;
        term1MaxTotal: number;
        midtermScore: number | null;
        midtermMaxScore: number;
        term2Total: number;
        term2MaxTotal: number;
        finalExamScore: number | null;
        finalExamMaxScore: number;
      }>();

      for (const item of allItems) {
        if (!subjectMap.has(item.subjectName)) {
          subjectMap.set(item.subjectName, {
            subject: item.subjectName,
            term1Total: 0,
            term1MaxTotal: 0,
            midtermScore: null,
            midtermMaxScore: 0,
            term2Total: 0,
            term2MaxTotal: 0,
            finalExamScore: null,
            finalExamMaxScore: 0,
          });
        }
        const row = subjectMap.get(item.subjectName)!;
        const scoreStr = gradeMap.get(item.id);
        const score = scoreStr ? parseFloat(scoreStr) : null;

        if (term1Months.includes(item.month)) {
          if (score !== null && !isNaN(score)) row.term1Total += score;
          row.term1MaxTotal += parseFloat(item.maxScore || "0");
        } else if (item.month === midtermMonth) {
          if (score !== null && !isNaN(score)) row.midtermScore = (row.midtermScore || 0) + score;
          row.midtermMaxScore += parseFloat(item.maxScore || "0");
        } else if (term2Months.includes(item.month)) {
          if (score !== null && !isNaN(score)) row.term2Total += score;
          row.term2MaxTotal += parseFloat(item.maxScore || "0");
        } else if (item.month === finalExamMonth) {
          if (score !== null && !isNaN(score)) row.finalExamScore = (row.finalExamScore || 0) + score;
          row.finalExamMaxScore += parseFloat(item.maxScore || "0");
        }
      }

      const rows = Array.from(subjectMap.values()).map(row => {
        const scaledT1 = row.term1MaxTotal > 0 ? roundGrade((row.term1Total / row.term1MaxTotal) * 20) : 0;
        const scaledMidterm = row.midtermMaxScore > 0 ? roundGrade(((row.midtermScore || 0) / row.midtermMaxScore) * 30) : 0;
        const scaledT2 = row.term2MaxTotal > 0 ? roundGrade((row.term2Total / row.term2MaxTotal) * 20) : 0;
        const scaledFinal = row.finalExamMaxScore > 0 ? roundGrade(((row.finalExamScore || 0) / row.finalExamMaxScore) * 30) : 0;
        
        const total = roundGrade(scaledT1 + scaledMidterm + scaledT2 + scaledFinal);
        const maxTotal = 100;

        return {
          subject: row.subject,
          grades: {
            "المحصلة الأولى": scaledT1,
            "اختبار نصف العام": scaledMidterm,
            "المحصلة الثانية": scaledT2,
            "الاختبار النهائي": scaledFinal,
          } as Record<string, number | null>,
          total,
          maxTotal,
          gradeLabel: getGradeLabel(total, maxTotal),
        };
      });

      return {
        rows,
        columns: ["المحصلة الأولى", "اختبار نصف العام", "المحصلة الثانية", "الاختبار النهائي"],
        type: "final",
      };
    }

    // Fallback
    return { rows: [], columns: [], type: type || "monthly" };
};
