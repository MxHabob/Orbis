import { Database } from "bun:sqlite";

const DB_PATH = "C:\\Users\\YZLPN3\\AppData\\Roaming\\com.mulverse.orbis.dev\\orbis.db";
const ORG_ID = "f4c151ce-4131-4103-adc0-b546d6673113";
const DEFAULT_TEACHER_ID = "114e6aee-3394-4c3b-bcd2-e267747d4dd9"; // Fathi Sherihan
const CREATED_BY = "453b8c02-1b28-4c33-b119-59a3c8a7468d"; // Mohamed Alhabob

const S1_M4_CYCLE_ID = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s1-m4"; // Semester 1 - Month 4
const S2_M4_CYCLE_ID = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s2-m4"; // Semester 2 - Month 4

const MONTH4_ASSESSMENTS = {
  [S1_M4_CYCLE_ID]: {
    title: "Semester 1 Final Exam",
    title_ar: "الاختبار النهائي للنصف الأول",
    type: "exam",
    max_score: 30,
    weight: 30,
    grade_group_name: "الامتحان النهائي"
  },
  [S2_M4_CYCLE_ID]: {
    title: "Semester 2 Final Exam",
    title_ar: "الاختبار النهائي للنصف الثاني",
    type: "exam",
    max_score: 30,
    weight: 30,
    grade_group_name: "الامتحان النهائي"
  }
};

function fixMonth4Assessments() {
  console.log("Fixing Month 4 (Final Exam) assessments...");
  const db = new Database(DB_PATH);

  try {
    db.run("BEGIN TRANSACTION");

    // 1. Delete standard 4 assessments from Month 4 of both semesters
    db.run(
      `DELETE FROM assessments 
       WHERE cycle_id IN (?, ?)`,
      [S1_M4_CYCLE_ID, S2_M4_CYCLE_ID]
    );
    console.log("Cleared old assessments from Month 4.");

    // 2. Get all class_subjects mappings
    const classSubjects = db.query(
      "SELECT id, class_id, subject_id FROM class_subjects WHERE is_active = 1"
    ).all() as any[];

    let addedCount = 0;
    const now = new Date().toISOString();

    // 3. Add single Final Exam assessment (30 marks) to Month 4
    for (const cs of classSubjects) {
      const isGrade1to9 = cs.class_id.startsWith("class-grade");
      if (!isGrade1to9) continue;

      for (const m4Id of [S1_M4_CYCLE_ID, S2_M4_CYCLE_ID]) {
        const template = MONTH4_ASSESSMENTS[m4Id];
        const assessmentId = crypto.randomUUID();
        db.run(
          `INSERT INTO assessments (
            id, org_id, cycle_id, class_subject_id, title, title_ar, 
            type, max_score, weight, grade_group_name, is_published, is_locked, 
            created_by, created_at, updated_at
          ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 0, ?, ?, ?)`,
          [
            assessmentId,
            ORG_ID,
            m4Id,
            cs.id,
            template.title,
            template.title_ar,
            template.type,
            template.max_score,
            template.weight,
            template.grade_group_name,
            CREATED_BY,
            now,
            now
          ]
        );
        addedCount++;
      }
    }

    db.run("COMMIT");
    console.log("Month 4 assessments configured successfully!");
    console.log(`- Added ${addedCount} final exam assessments (out of 30) for Month 4 of Semester 1 & 2.`);
  } catch (error) {
    db.run("ROLLBACK");
    console.error("Failed to fix Month 4 assessments:", error);
  } finally {
    db.close();
  }
}

fixMonth4Assessments();
