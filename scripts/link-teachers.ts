import { Database } from "bun:sqlite";
import * as path from "path";
import * as fs from "fs";

const paths = [
  path.join(process.env.APPDATA || "", "com.mulverse.orbis.dev", "orbis.db"),
  path.join(process.env.APPDATA || "", "com.mulverse.orbis", "orbis.db"),
];

let dbPath = "";
for (const p of paths) {
  if (fs.existsSync(p)) {
    dbPath = p;
    break;
  }
}

if (!dbPath) {
  console.error("❌ SQLite database does not exist.");
  process.exit(1);
}

const db = new Database(dbPath);

try {
  // 1. Get all teachers
  const teachers = db.query("SELECT id, full_name FROM users WHERE primary_role = 'teacher'").all() as { id: string, full_name: string }[];
  if (teachers.length === 0) {
    console.error("❌ No teachers found in the database. Please run seed-teachers.ts first.");
    process.exit(1);
  }
  console.log(`✅ Found ${teachers.length} teachers in the database.`);

  // 2. Get all class subjects
  const classSubjects = db.query("SELECT id FROM class_subjects").all() as { id: string }[];
  console.log(`✅ Found ${classSubjects.length} class subjects to assign.`);

  // 3. Update class_subjects with distributed teachers
  db.transaction(() => {
    classSubjects.forEach((cs, index) => {
      const teacher = teachers[index % teachers.length];
      db.prepare("UPDATE class_subjects SET primary_teacher_id = ? WHERE id = ?").run(teacher.id, cs.id);
    });
  })();

  console.log("🎉 Successfully distributed teachers across all class subjects!");
  
  // Print summary of distribution
  const summary = db.query(`
    SELECT u.full_name, COUNT(cs.id) as count 
    FROM users u 
    JOIN class_subjects cs ON cs.primary_teacher_id = u.id 
    GROUP BY u.id
  `).all() as { full_name: string, count: number }[];
  
  console.log("\nDistribution Summary:");
  summary.forEach(row => {
    console.log(`- ${row.full_name}: ${row.count} subjects`);
  });

} catch (e) {
  console.error("❌ Distribution failed:", e);
} finally {
  db.close();
}
