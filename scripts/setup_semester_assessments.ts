import { Database } from "bun:sqlite";

const DB_PATH =
  "C:\\Users\\YZLPN3\\AppData\\Roaming\\com.mulverse.orbis.dev\\orbis.db";
const ORG_ID = "f4c151ce-4131-4103-adc0-b546d6673113";
const DEFAULT_TEACHER_ID = "114e6aee-3394-4c3b-bcd2-e267747d4dd9"; // Fathi Sherihan
const CREATED_BY = "453b8c02-1b28-4c33-b119-59a3c8a7468d"; // Mohamed Alhabob

const S1_CYCLE_ID = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s1"; // Semester 1
const S2_CYCLE_ID = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113-s2"; // Semester 2

const SEMESTER_ASSESSMENTS = {
  [S1_CYCLE_ID]: [
    {
      title: "Semester 1 Classwork",
      title_ar: "أعمال الفصل الأول",
      type: "classwork",
      max_score: 20,
      weight: 20,
      grade_group_name: "أعمال الفصل",
    },
    {
      title: "Semester 1 Final Exam",
      title_ar: "الاختبار النهائي للنصف الأول",
      type: "exam",
      max_score: 30,
      weight: 30,
      grade_group_name: "الامتحان النهائي",
    },
  ],
  [S2_CYCLE_ID]: [
    {
      title: "Semester 2 Classwork",
      title_ar: "أعمال الفصل الثاني",
      type: "classwork",
      max_score: 20,
      weight: 20,
      grade_group_name: "أعمال الفصل",
    },
    {
      title: "Semester 2 Final Exam",
      title_ar: "الاختبار النهائي للنصف الثاني",
      type: "exam",
      max_score: 30,
      weight: 30,
      grade_group_name: "الامتحان النهائي",
    },
  ],
};

function setupSemesterAssessments() {
  console.log("Re-configuring Semester 1 & Semester 2 assessments...");
  const db = new Database(DB_PATH);

  try {
    db.run("BEGIN TRANSACTION");

    // 1. Delete standard monthly assessments from Semester 1 and Semester 2 cycles
    db.run(
      `DELETE FROM assessments 
       WHERE cycle_id IN (?, ?)`,
      [S1_CYCLE_ID, S2_CYCLE_ID],
    );
    console.log("Cleared old assessments from Semester 1 and Semester 2.");

    // 2. Get all class_subjects mappings
    const classSubjects = db
      .query(
        "SELECT id, class_id, subject_id FROM class_subjects WHERE is_active = 1",
      )
      .all() as any[];

    console.log(`Found ${classSubjects.length} class-subject links.`);

    let addedCount = 0;
    const now = new Date().toISOString();

    // 3. Add Semester Work (20) & Final Exam (30) to each Semester cycle
    for (const cs of classSubjects) {
      // Check if this class belongs to Grades 1 to 9
      const isGrade1to9 = cs.class_id.startsWith("class-grade");
      if (!isGrade1to9) continue;

      for (const semId of [S1_CYCLE_ID, S2_CYCLE_ID]) {
        const templates = SEMESTER_ASSESSMENTS[semId];
        for (const template of templates) {
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
              semId,
              cs.id,
              template.title,
              template.title_ar,
              template.type,
              template.max_score,
              template.weight,
              template.grade_group_name,
              CREATED_BY,
              now,
              now,
            ],
          );
          addedCount++;
        }
      }
    }

    db.run("COMMIT");
    console.log("Semester assessments configured successfully!");
    console.log(
      `- Added ${addedCount} correct assessments (Classwork out of 20 & Final Exam out of 30) for Semester 1 & 2.`,
    );
  } catch (error) {
    db.run("ROLLBACK");
    console.error("Failed to setup semester assessments:", error);
  } finally {
    db.close();
  }
}

setupSemesterAssessments();
