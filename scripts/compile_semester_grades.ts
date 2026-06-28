import { Database } from "bun:sqlite";

const DB_PATH =
  "C:\\Users\\YZLPN3\\AppData\\Roaming\\com.mulverse.orbis.dev\\orbis.db";
const ORG_ID = "f4c151ce-4131-4103-adc0-b546d6673113";
const GRADED_BY = "453b8c02-1b28-4c33-b119-59a3c8a7468d"; // Mohamed Alhabob

const S1_CYCLE_ID = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s1";
const S2_CYCLE_ID = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s2";

// Months corresponding to Semester 1 (Month 1, 2, 3)
const S1_MONTHS = [
  "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s1-m1",
  "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s1-m2",
  "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s1-m3",
];

const S1_M4 = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s1-m4"; // Semester 1 - Month 4 (Final Exam)

// Months corresponding to Semester 2 (Month 1, 2, 3)
const S2_MONTHS = [
  "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s2-m1",
  "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s2-m2",
  "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s2-m3",
];

const S2_M4 = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s2-m4"; // Semester 2 - Month 4 (Final Exam)

function compileGrades(semesterNum: 1 | 2) {
  const semCycleId = semesterNum === 1 ? S1_CYCLE_ID : S2_CYCLE_ID;
  const months = semesterNum === 1 ? S1_MONTHS : S2_MONTHS;
  const m4CycleId = semesterNum === 1 ? S1_M4 : S2_M4;
  const semesterName = semesterNum === 1 ? "Semester 1" : "Semester 2";

  const classworkTitleAr =
    semesterNum === 1 ? "أعمال الفصل الأول" : "أعمال الفصل الثاني";
  const finalExamTitleAr =
    semesterNum === 1
      ? "الاختبار النهائي للنصف الأول"
      : "الاختبار النهائي للنصف الثاني";

  console.log(`Starting Grade Compilation for ${semesterName}...`);
  const db = new Database(DB_PATH);

  try {
    db.run("BEGIN TRANSACTION");

    // 1. Get all enrollments
    const enrollments = db
      .query("SELECT id, student_id FROM enrollments WHERE is_active = 1")
      .all() as any[];

    console.log(`Found ${enrollments.length} active student enrollments.`);

    // 2. Get all class subject mappings
    const classSubjects = db
      .query(
        `SELECT cs.id, cs.class_id, cs.subject_id, s.name as subject_name, s.name_ar as subject_name_ar 
       FROM class_subjects cs
       JOIN subjects s ON cs.subject_id = s.id
       WHERE cs.is_active = 1`,
      )
      .all() as any[];

    // 3. Find the target semester assessments (Classwork out of 20 and Final Exam out of 30)
    const semesterAssessments = db
      .query(
        `SELECT id, class_subject_id, title_ar FROM assessments 
       WHERE cycle_id = ? AND title_ar IN (?, ?)`,
      )
      .all(semCycleId, classworkTitleAr, finalExamTitleAr) as any[];

    const classworkAssessmentMap = new Map<string, string>(); // class_subject_id -> assessment_id
    const finalExamAssessmentMap = new Map<string, string>(); // class_subject_id -> assessment_id
    for (const ass of semesterAssessments) {
      if (ass.title_ar === classworkTitleAr) {
        classworkAssessmentMap.set(ass.class_subject_id, ass.id);
      } else if (ass.title_ar === finalExamTitleAr) {
        finalExamAssessmentMap.set(ass.class_subject_id, ass.id);
      }
    }

    let classworkCompiled = 0;
    let finalExamCompiled = 0;
    const now = new Date().toISOString();

    for (const enrollment of enrollments) {
      // -------------------------------------------------------------
      // PART A: Compile Month 1, 2, 3 average -> أعمال الفصل (20%)
      // -------------------------------------------------------------
      const monthlyGrades = db
        .query(
          `SELECT class_subject_id, cycle_id, score, max_score 
         FROM grades 
         WHERE enrollment_id = ? AND cycle_id IN (${months.map(() => "?").join(",")})`,
        )
        .all(enrollment.id, ...months) as any[];

      // Group monthly grades by class_subject
      const monthlyGradesBySubject = new Map<string, any[]>();
      for (const g of monthlyGrades) {
        if (!monthlyGradesBySubject.has(g.class_subject_id)) {
          monthlyGradesBySubject.set(g.class_subject_id, []);
        }
        monthlyGradesBySubject.get(g.class_subject_id)!.push(g);
      }

      for (const [
        classSubId,
        subjectGrades,
      ] of monthlyGradesBySubject.entries()) {
        const cs = classSubjects.find((c) => c.id === classSubId);
        if (!cs) continue;

        const targetClassworkAssId = classworkAssessmentMap.get(classSubId);
        if (!targetClassworkAssId) continue;

        // Group by month to calculate month percentages
        const monthGroup = new Map<
          string,
          { score: number; maxScore: number }
        >();
        for (const g of subjectGrades) {
          if (!monthGroup.has(g.cycle_id)) {
            monthGroup.set(g.cycle_id, { score: 0, maxScore: 0 });
          }
          const mData = monthGroup.get(g.cycle_id)!;
          mData.score += g.score;
          mData.maxScore += g.max_score;
        }

        let totalPct = 0;
        let activeMonthsCount = 0;

        for (const [_, mData] of monthGroup.entries()) {
          if (mData.maxScore > 0) {
            totalPct += (mData.score / mData.maxScore) * 100;
            activeMonthsCount++;
          }
        }

        if (activeMonthsCount === 0) continue;

        const avgPct = totalPct / activeMonthsCount;
        const compiledScore = Math.round((avgPct / 100) * 20 * 100) / 100;

        // Upsert Classwork Grade
        const existingClasswork = db
          .query(
            `SELECT id FROM grades 
           WHERE enrollment_id = ? AND assessment_id = ? AND cycle_id = ?`,
          )
          .get(enrollment.id, targetClassworkAssId, semCycleId) as any;

        if (existingClasswork) {
          db.run(`UPDATE grades SET score = ?, updated_at = ? WHERE id = ?`, [
            compiledScore,
            now,
            existingClasswork.id,
          ]);
        } else {
          const gradeId = crypto.randomUUID();
          db.run(
            `INSERT INTO grades (
              id, org_id, cycle_id, enrollment_id, assessment_id, class_subject_id, 
              subject_name, subject_name_ar, score, max_score, weight, 
              exam_type, semester, notes, graded_by, graded_at, is_final, 
              created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 20, 20, 'classwork', ?, '', ?, ?, 1, ?, ?)`,
            [
              gradeId,
              ORG_ID,
              semCycleId,
              enrollment.id,
              targetClassworkAssId,
              classSubId,
              cs.subject_name,
              cs.subject_name_ar,
              compiledScore,
              semesterName,
              GRADED_BY,
              now,
              now,
              now,
            ],
          );
        }
        classworkCompiled++;
      }

      // -------------------------------------------------------------
      // PART B: Copy Month 4 Final Exam -> الاختبار النهائي للفصل (30%)
      // -------------------------------------------------------------
      const finalExamGrades = db
        .query(
          `SELECT class_subject_id, score, max_score 
         FROM grades 
         WHERE enrollment_id = ? AND cycle_id = ?`,
        )
        .all(enrollment.id, m4CycleId) as any[];

      for (const g of finalExamGrades) {
        const cs = classSubjects.find((c) => c.id === g.class_subject_id);
        if (!cs) continue;

        const targetFinalExamAssId = finalExamAssessmentMap.get(
          g.class_subject_id,
        );
        if (!targetFinalExamAssId) continue;

        // Since it's final exam, we copy the score directly.
        // If max_score was not 30, we scale it. But Month 4 has it out of 30 already!
        let finalExamScore = g.score;
        if (g.max_score !== 30 && g.max_score > 0) {
          finalExamScore = Math.round((g.score / g.max_score) * 30 * 100) / 100;
        }

        // Upsert Final Exam Grade
        const existingFinalExam = db
          .query(
            `SELECT id FROM grades 
           WHERE enrollment_id = ? AND assessment_id = ? AND cycle_id = ?`,
          )
          .get(enrollment.id, targetFinalExamAssId, semCycleId) as any;

        if (existingFinalExam) {
          db.run(`UPDATE grades SET score = ?, updated_at = ? WHERE id = ?`, [
            finalExamScore,
            now,
            existingFinalExam.id,
          ]);
        } else {
          const gradeId = crypto.randomUUID();
          db.run(
            `INSERT INTO grades (
              id, org_id, cycle_id, enrollment_id, assessment_id, class_subject_id, 
              subject_name, subject_name_ar, score, max_score, weight, 
              exam_type, semester, notes, graded_by, graded_at, is_final, 
              created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 30, 30, 'exam', ?, '', ?, ?, 1, ?, ?)`,
            [
              gradeId,
              ORG_ID,
              semCycleId,
              enrollment.id,
              targetFinalExamAssId,
              g.class_subject_id,
              cs.subject_name,
              cs.subject_name_ar,
              finalExamScore,
              semesterName,
              GRADED_BY,
              now,
              now,
              now,
            ],
          );
        }
        finalExamCompiled++;
      }
    }

    db.run("COMMIT");
    console.log(
      `Successfully compiled ${classworkCompiled} classwork & copied ${finalExamCompiled} exams for ${semesterName}!`,
    );
  } catch (error) {
    db.run("ROLLBACK");
    console.error("Compilation failed, transaction rolled back:", error);
  } finally {
    db.close();
  }
}

// Compile Semester 1
compileGrades(1);
// Compile Semester 2
compileGrades(2);
