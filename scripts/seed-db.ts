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
  const orgs = db.query("SELECT DISTINCT org_id FROM user_roles").all() as { org_id: string }[];
  if (orgs.length === 0) {
    console.error("❌ No organization found in user_roles!");
    process.exit(1);
  }
  const orgId = orgs[0].org_id;
  console.log("✅ Found active Organization ID:", orgId);

  const sqlFile = path.resolve(__dirname, "seed_school_finance.sql");
  let sql = fs.readFileSync(sqlFile, "utf8");
  
  // Replace org id
  sql = sql.replace(/default-org-id/g, orgId);

  // Strip single-line SQL comments (-- ...)
  const cleanSql = sql
    .split("\n")
    .map(line => {
      const idx = line.indexOf("--");
      if (idx !== -1) {
        return line.substring(0, idx);
      }
      return line;
    })
    .join("\n");

  const rawStatements = cleanSql.split(";");
  console.log("Raw split count:", rawStatements.length);

  const statements = rawStatements
    .map(s => s.trim())
    .filter(s => s.length > 0);

  console.log("Filtered statements count:", statements.length);
  
  db.transaction(() => {
    for (let i = 0; i < statements.length; i++) {
      console.log(`Executing statement ${i + 1}/${statements.length}...`);
      try {
        db.run(statements[i]);
      } catch (e) {
        console.error(`❌ Statement ${i + 1} failed:`, statements[i].substring(0, 100) + "...");
        throw e; // roll back transaction
      }
    }
  })();

  const ledgersCount = db.query("SELECT COUNT(*) as count FROM ledger_accounts").get() as { count: number };
  const treasuriesCount = db.query("SELECT COUNT(*) as count FROM treasuries").get() as { count: number };
  console.log("🎉 Seeding successful!");
  console.log("Current Ledgers in DB:", ledgersCount.count);
  console.log("Current Treasuries in DB:", treasuriesCount.count);
} catch (e) {
  console.error("❌ Seeding failed:", e);
} finally {
  db.close();
}
