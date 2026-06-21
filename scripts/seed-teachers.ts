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
  // 1. Get org_id
  const orgs = db.query("SELECT DISTINCT org_id FROM user_roles").all() as { org_id: string }[];
  if (orgs.length === 0) {
    console.error("❌ No organization found in user_roles!");
    process.exit(1);
  }
  const orgId = orgs[0].org_id;
  console.log("✅ Found active Organization ID:", orgId);

  // 2. Generate and insert 32 teachers
  console.log("Generating 32 teachers...");
  const now = new Date().toISOString();

  db.transaction(() => {
    for (let i = 1; i <= 32; i++) {
      const userId = crypto.randomUUID();
      const roleId = crypto.randomUUID();
      const email = `teacher${i}@orbis.edu`;
      const fullName = `Teacher ${i}`;
      const nameAr = `معلم ${i}`;

      // Insert into users
      db.prepare(`
        INSERT INTO users (
          id, org_id, email, email_verified, phone_verified, mfa_enabled,
          full_name, name_ar, primary_role, status, prefs, meta, is_deleted,
          created_at, updated_at
        ) VALUES (?, ?, ?, 1, 0, 0, ?, ?, 'teacher', 'active', '{}', '{}', 0, ?, ?)
      `).run(userId, orgId, email, fullName, nameAr, now, now);

      // Insert into user_roles
      db.prepare(`
        INSERT INTO user_roles (
          id, user_id, role, org_id, scope_type, is_primary, assigned_at
        ) VALUES (?, ?, 'teacher', ?, 'org', 1, ?)
      `).run(roleId, userId, orgId, now);
    }
  })();

  const teachersCount = db.query("SELECT COUNT(*) as count FROM users WHERE primary_role = 'teacher'").get() as { count: number };
  console.log("🎉 Successfully created 32 teachers!");
  console.log("Total teachers in DB now:", teachersCount.count);
} catch (e) {
  console.error("❌ Seeding teachers failed:", e);
} finally {
  db.close();
}
