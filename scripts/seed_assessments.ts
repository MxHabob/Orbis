import { Database } from "bun:sqlite";

const DB_PATH =
  "C:\\Users\\YZLPN3\\AppData\\Roaming\\com.mulverse.orbis.dev\\orbis.db";
const ORG_ID = "f4c151ce-4131-4103-adc0-b546d6673113";
const DEFAULT_TEACHER_ID = "114e6aee-3394-4c3b-bcd2-e267747d4dd9"; // Fathi Sherihan
const CREATED_BY = "453b8c02-1b28-4c33-b119-59a3c8a7468d"; // Mohamed Alhabob

const YEAR_CYCLE_ID = "cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113";

const ASSESSMENT_TEMPLATES = [
  {
    title: "Homework",
    title_ar: "الواجبات",
    type: "homework",
    max_score: 100,
    weight: 20,
    grade_group_name: "الواجبات",
  },
  {
    title: "Participation",
    title_ar: "المشاركة",
    type: "classwork",
    max_score: 100,
    weight: 20,
    grade_group_name: "المشاركة",
  },
  {
    title: "Attendance",
    title_ar: "الحضور",
    type: "classwork",
    max_score: 100,
    weight: 20,
    grade_group_name: "الحضور",
  },
  {
    title: "Written Exam",
    title_ar: "التحريري",
    type: "exam",
    max_score: 100,
    weight: 40,
    grade_group_name: "التحريري",
  },
];

function seed() {
  console.log("Starting seed script for all terms and monthly cycles...");
  const db = new Database(DB_PATH);

  try {
    db.run("BEGIN TRANSACTION");

    // 1. Get all classes from Grade 1 to Grade 9
    const classes = db
      .query(
        `SELECT id, name, name_ar FROM classes 
       WHERE id LIKE 'class-grade%-f4c151ce-4131-4103-adc0-b546d6673113'`,
      )
      .all() as any[];

    console.log(`Found ${classes.length} classes (Grade 1 to 9).`);

    // 2. Get all subjects
    const subjects = db
      .query(
        `SELECT id, name, name_ar FROM subjects 
       WHERE id LIKE '%-f4c151ce-4131-4103-adc0-b546d6673113'`,
      )
      .all() as any[];

    console.log(`Found ${subjects.length} subjects.`);

    // 3. Get all active child cycles (Terms/Semesters and Months)
    const targetCycles = db
      .query(
        `SELECT id, name FROM academic_cycles 
       WHERE id LIKE 'cycle-1447-f4c151ce-4131-4103-adc0-b546d6673113%' AND type IN ('term', 'month')`,
      )
      .all() as any[];

    console.log(
      `Found ${targetCycles.length} target cycles (Semesters & Months) to seed.`,
    );

    let classSubjectCount = 0;
    let assessmentCount = 0;

    const now = new Date().toISOString();

    for (const cls of classes) {
      for (const subj of subjects) {
        // Find or create class_subject mapping for the Year cycle
        let classSubject = db
          .query(
            "SELECT id FROM class_subjects WHERE class_id = ? AND subject_id = ? AND cycle_id = ? AND is_active = 1",
          )
          .get(cls.id, subj.id, YEAR_CYCLE_ID) as any;

        let classSubjectId = classSubject?.id;

        if (!classSubjectId) {
          classSubjectId = crypto.randomUUID();
          db.run(
            `INSERT INTO class_subjects (
              id, class_id, subject_id, cycle_id, primary_teacher_id, 
              weekly_hours, is_active, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, 4, 1, ?, ?)`,
            [
              classSubjectId,
              cls.id,
              subj.id,
              YEAR_CYCLE_ID,
              DEFAULT_TEACHER_ID,
              now,
              now,
            ],
          );
          classSubjectCount++;
        }

        // Add assessments for each targeted term and month cycle
        for (const cycle of targetCycles) {
          for (const template of ASSESSMENT_TEMPLATES) {
            // Check if assessment already exists
            const existing = db
              .query(
                `SELECT id FROM assessments 
               WHERE class_subject_id = ? AND cycle_id = ? AND title_ar = ?`,
              )
              .get(classSubjectId, cycle.id, template.title_ar) as any;

            if (!existing) {
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
                  cycle.id,
                  classSubjectId,
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
              assessmentCount++;
            }
          }
        }
      }
    }

    db.run("COMMIT");
    console.log("Seeding completed successfully!");
    console.log(`- Created/Verified class-subject mappings.`);
    console.log(
      `- Added ${assessmentCount} new assessments across all semesters & months.`,
    );
  } catch (error) {
    db.run("ROLLBACK");
    console.error("Seeding failed, transaction rolled back:", error);
  } finally {
    db.close();
  }
}

seed();
