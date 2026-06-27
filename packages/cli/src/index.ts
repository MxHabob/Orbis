#!/usr/bin/env bun
import os from "node:os";
import path from "node:path";
import { existsSync, mkdirSync, readFileSync, unlinkSync } from "node:fs";
import { Database } from "bun:sqlite";
import readline from "node:readline";
import { fileURLToPath } from "node:url";

// ── Clack-style CLI Styling Helpers ────────────────────────────────────────

const c = {
  purple: (txt: string) => `\x1b[35m${txt}\x1b[0m`,
  cyan: (txt: string) => `\x1b[36m${txt}\x1b[0m`,
  green: (txt: string) => `\x1b[32m${txt}\x1b[0m`,
  yellow: (txt: string) => `\x1b[33m${txt}\x1b[0m`,
  red: (txt: string) => `\x1b[31m${txt}\x1b[0m`,
  gray: (txt: string) => `\x1b[90m${txt}\x1b[0m`,
  bold: (txt: string) => `\x1b[1m${txt}\x1b[0m`,
};

function printHeader(title: string) {
  console.log(c.purple(`◆  ${c.bold(title)}`));
  console.log(c.gray("│"));
}

function printStep(title: string) {
  console.log(c.purple(`◇  ${c.bold(title)}`));
}

function printSuccess(message: string) {
  console.log(c.green(`✔  ${message}`));
  console.log(c.gray("│"));
}

function printInfo(message: string) {
  console.log(c.cyan(`▲  ${message}`));
  console.log(c.gray("│"));
}

// ── Interactive Question Helpers ──────────────────────────────────────────

async function ask(query: string, defaultValue = "", validator?: (val: string) => boolean): Promise<string> {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise((resolve) => {
    const promptText = defaultValue 
      ? `${c.gray("│")}  ${query} ${c.gray(`(${defaultValue})`)}: ` 
      : `${c.gray("│")}  ${query}: `;

    rl.question(promptText, (answer) => {
      rl.close();
      const finalValue = answer.trim() || defaultValue;
      if (validator && !validator(finalValue)) {
        console.log(`${c.gray("│")}  ${c.red("✖  Invalid input, please try again.")}`);
        resolve(ask(query, defaultValue, validator));
      } else {
        resolve(finalValue);
      }
    });
  });
}

async function askPassword(query: string): Promise<string> {
  return new Promise((resolve) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });
    
    process.stdout.write(`${c.gray("│")}  ${query}: `);
    
    let password = "";
    process.stdin.setRawMode(true);
    process.stdin.resume();
    
    const handler = (char: Buffer) => {
      const s = char.toString("utf-8");
      if (s === "\n" || s === "\r" || s === "\u000d") {
        process.stdin.removeListener("data", handler);
        process.stdin.setRawMode(false);
        process.stdin.pause();
        process.stdout.write("\n");
        rl.close();
        resolve(password);
      } else if (s === "\u0003") { // Ctrl+C
        process.exit(1);
      } else if (char[0] === 127 || char[0] === 8) { // Backspace
        if (password.length > 0) {
          password = password.slice(0, -1);
          process.stdout.write("\b \b");
        }
      } else {
        password += s;
        process.stdout.write("*");
      }
    };
    
    process.stdin.on("data", handler);
  });
}

// ── Database Path & Identifier Resolver ────────────────────────────────────

function getDbPath(): string {
  const home = os.homedir();
  const identifier = "com.mulverse.orbis.svelte";
  let appDataDir = "";

  if (process.platform === "win32") {
    const appData = process.env.APPDATA || path.join(home, "AppData", "Roaming");
    appDataDir = path.join(appData, identifier);
  } else if (process.platform === "darwin") {
    appDataDir = path.join(home, "Library", "Application Support", identifier);
  } else {
    const xdgData = process.env.XDG_DATA_HOME || path.join(home, ".local", "share");
    appDataDir = path.join(xdgData, identifier);
  }

  return path.join(appDataDir, "orbis.db");
}

// ── Help & Usage Guides ─────────────────────────────────────────────────────

function printHelp() {
  console.log(c.bold("Usage:"));
  console.log(`  bun run start [command] [options]\n`);
  
  console.log(c.bold("Commands:"));
  console.log(`  ${c.cyan("init")}          Start school system installation & db migrations (Default)`);
  console.log(`  ${c.cyan("db:reset")}      Completely reset and delete local SQLite database file\n`);

  console.log(c.bold("Options:"));
  console.log(`  ${c.yellow("-h, --help")}    Show this help message`);
  console.log(`  ${c.yellow("-v, --version")} Show installer CLI version\n`);

  console.log(c.bold("Auto-Install Options (for silent installations):"));
  console.log(`  ${c.purple("--school")}       English School Name`);
  console.log(`  ${c.purple("--school-ar")}    Arabic School Name`);
  console.log(`  ${c.purple("--email")}        Admin login email address`);
  console.log(`  ${c.purple("--password")}     Admin password (min 6 characters)`);
  console.log(`  ${c.purple("--admin")}        Admin English name (default: School Admin)`);
  console.log(`  ${c.purple("--admin-ar")}     Admin Arabic name (default: مدير المدرسة)`);
  console.log(`  ${c.purple("--timezone")}     Timezone (default: Asia/Aden)`);
  console.log(`  ${c.purple("--phone")}        Phone number`);
  console.log();
}

// ── Database Schema Migration ──────────────────────────────────────────────

function runDatabaseMigration(db: Database) {
  printInfo("Tables not found. Initializing database schema...");
  const __dirname = path.dirname(fileURLToPath(import.meta.url));
  const dbDir = path.resolve(__dirname, "..", "..", "..", "apps", "api-rust", "crates", "core-domain", "src", "database");
  const coreSchemaPath = path.join(dbDir, "core_schema.sql");
  const schemaPath = path.join(dbDir, "schema.sql");

  if (!existsSync(coreSchemaPath) || !existsSync(schemaPath)) {
    console.log(`${c.gray("│")}  ${c.red(`✖  Schema SQL files not found in: ${dbDir}`)}`);
    process.exit(1);
  }

  const coreSchemaSql = readFileSync(coreSchemaPath, "utf-8");
  const schemaSql = readFileSync(schemaPath, "utf-8");

  db.run("PRAGMA foreign_keys = OFF;");
  for (const sql of [coreSchemaSql, schemaSql]) {
    const statements = sql.split(";");
    for (const statement of statements) {
      const cleaned = statement
        .split("\n")
        .map(line => line.trim())
        .filter(line => !line.startsWith("--") && line.length > 0)
        .join("\n")
        .trim();
      if (cleaned.length > 0) {
        try {
          db.run(cleaned);
        } catch (err) {
          // Ignore table duplicate warnings if running repeatedly
        }
      }
    }
  }
  db.run("PRAGMA foreign_keys = ON;");
  printSuccess("Database schema tables initialized.");
}

// ── Db Reset Command ────────────────────────────────────────────────────────

async function handleDatabaseReset() {
  printHeader("Orbis Database Reset Tool");
  const dbPath = getDbPath();
  
  if (!existsSync(dbPath)) {
    printInfo(`No database file found at ${dbPath}`);
    console.log(c.purple("└  Completed."));
    return;
  }

  const confirm = await ask("Are you sure you want to delete the database and wipe all data? (yes/no)", "no");
  if (confirm.toLowerCase() === "yes") {
    try {
      unlinkSync(dbPath);
      printSuccess(`Database deleted successfully from ${dbPath}`);
    } catch (e) {
      console.log(`${c.gray("│")}  ${c.red(`✖  Failed to delete database: ${e}`)}`);
    }
  } else {
    printInfo("Reset canceled by user.");
  }
  
  console.log(c.purple("└  Completed."));
}

// ── Automated Financials & Permissions Seeder ───────────────────────────────

function seedDefaultFinancialsAndPermissions(db: Database, orgId: string, baseCurrency: string) {
  const now = new Date().toISOString();
  
  // 1. Seed Currencies (YER, USD, SAR)
  const defaultCurrencies = [
    { code: "YER", name: "Yemeni Rial", nameAr: "ريال يمني", symbol: "﷼", position: "after", decimals: 0, rate: 1.0 },
    { code: "USD", name: "US Dollar", nameAr: "دولار أمريكي", symbol: "$", position: "before", decimals: 2, rate: 250.0 },
    { code: "SAR", name: "Saudi Riyal", nameAr: "ريال سعودي", symbol: "ر.س", position: "after", decimals: 2, rate: 66.0 }
  ];

  for (const cur of defaultCurrencies) {
    const isBase = cur.code === baseCurrency ? 1 : 0;
    db.run(
      `INSERT OR IGNORE INTO currencies (
        code, org_id, name, name_ar, symbol, symbol_position, decimal_places, exchange_rate, is_base, is_active, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?)`,
      [cur.code, orgId, cur.name, cur.nameAr, cur.symbol, cur.position, cur.decimals, cur.rate, isBase, now]
    );
  }

  // 2. Seed Chart of Accounts
  const coaStructure = [
    // format: [id_suffix, parent_suffix, code, name, name_ar, type, sub_type, is_control, is_system]
    ["assets", null, "1000", "Assets", "الأصول", "asset", null, 1, 1],
    ["current_assets", "assets", "1100", "Current Assets", "الأصول المتداولة", "asset", null, 1, 1],
    ["cash_treasury_control", "current_assets", "1110", "Cash & Treasury Control", "حساب مراقبة النقدية والخزائن", "asset", "treasury_control", 1, 1],
    ["student_receivable_control", "current_assets", "1120", "Accounts Receivable - Students", "ذمم الطلاب المدينين", "asset", "student_receivable", 1, 1],
    ["liabilities", null, "2000", "Liabilities", "الالتزامات", "liability", null, 1, 1],
    ["current_liabilities", "liabilities", "2100", "Current Liabilities", "الالتزامات المتداولة", "liability", null, 1, 1],
    ["employee_payable_control", "current_liabilities", "2110", "Accounts Payable - Employees", "ذمم الموظفين الدائنة", "liability", "employee_payable", 1, 1],
    ["equity", null, "3000", "Equity", "حقوق الملكية", "equity", null, 1, 1],
    ["retained_earnings", "equity", "3100", "Retained Earnings", "الأرباح المحتجزة", "equity", "retained_earnings", 0, 1],
    ["revenue", null, "4000", "Revenue", "الإيرادات", "revenue", null, 1, 1],
    ["tuition_revenue", "revenue", "4100", "Tuition Revenue", "إيرادات الرسوم الدراسية", "revenue", "tuition_revenue", 0, 1],
    ["other_revenue", "revenue", "4200", "Other School Revenue", "إيرادات مدرسية أخرى", "revenue", null, 0, 1],
    ["expenses", null, "5000", "Expenses", "المصروفات", "expense", null, 1, 1],
    ["salaries_expense", "expenses", "5100", "Academic Salaries Expense", "مصروفات رواتب الكادر التعليمي", "expense", "salaries_expense", 0, 1],
    ["operational_expense", "expenses", "5200", "Operational Expenses", "المصروفات التشغيلية", "expense", null, 0, 1],
  ] as const;

  const createdPaths: Record<string, { path: string, depth: number }> = {};

  for (const [id_suffix, parent_suffix, code, name, name_ar, item_type, sub_type, is_control, is_system] of coaStructure) {
    const full_id = `${orgId}_${id_suffix}`;
    const full_parent_id = parent_suffix ? `${orgId}_${parent_suffix}` : null;

    let pathVal = full_id;
    let depth = 0;

    if (full_parent_id && createdPaths[full_parent_id]) {
      const parentInfo = createdPaths[full_parent_id];
      pathVal = `${parentInfo.path}/${full_id}`;
      depth = parentInfo.depth + 1;
    }

    createdPaths[full_id] = { path: pathVal, depth };

    db.run(
      `INSERT OR IGNORE INTO ledger_accounts (
        id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_control_account, is_system_account, is_active, created_at, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0.0, 0.0, ?, ?, 1, ?, ?)`,
      [
        full_id,
        orgId,
        code,
        name,
        name_ar,
        item_type,
        sub_type,
        full_parent_id,
        pathVal,
        depth,
        baseCurrency,
        is_control,
        is_system,
        now,
        now
      ]
    );
  }

  // 3. Seed Default Permissions
  const permissions = [
    { code: "OrganizationRead", name: "Read Organization Info", nameAr: "عرض معلومات المنشأة", module: "organization", action: "read", resource: "info" },
    { code: "OrganizationWrite", name: "Write Organization Info", nameAr: "تعديل معلومات المنشأة", module: "organization", action: "write", resource: "info" },
    { code: "UserRead", name: "Read Users Directory", nameAr: "استعراض دليل المستخدمين", module: "users", action: "read", resource: "directory" },
    { code: "UserWrite", name: "Create and Edit Users", nameAr: "إنشاء وتعديل حسابات المستخدمين", module: "users", action: "write", resource: "account" },
    { code: "UserDelete", name: "Delete Users", nameAr: "حذف حسابات المستخدمين", module: "users", action: "delete", resource: "account" },
    { code: "CycleRead", name: "Read Academic Cycles", nameAr: "عرض الفترات والأعوام الدراسية", module: "academics", action: "read", resource: "cycle" },
    { code: "CycleWrite", name: "Create and Edit Cycles", nameAr: "إنشاء وتعديل الفترات والأعوام", module: "academics", action: "write", resource: "cycle" }
  ];

  for (const p of permissions) {
    db.run(
      `INSERT OR IGNORE INTO permissions (
        code, name, name_ar, description, module, action, resource, requires_scope, scope_types, is_system, is_active, created_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, 0, '["global"]', 1, 1, ?)`,
      [p.code, p.name, p.nameAr, p.name, p.module, p.action, p.resource, now]
    );
  }

  printSuccess("Automatically seeded system currencies, Chart of Accounts, and default permissions.");
}

// ── Main Commands Orchestration ─────────────────────────────────────────────

async function main() {
  const args = process.argv.slice(2);
  const command = args[0];

  // Help command
  if (args.includes("-h") || args.includes("--help") || command === "help") {
    printHeader("Orbis OS Installer CLI help");
    printHelp();
    return;
  }

  // Version command
  if (args.includes("-v") || args.includes("--version")) {
    console.log(`Orbis OS CLI v1.0.0 (2026 Edition)`);
    return;
  }

  // Reset database command
  if (command === "db:reset") {
    await handleDatabaseReset();
    return;
  }

  // Fallback to init command
  await handleInstallation();
}

async function handleInstallation() {
  printHeader("Orbis OS System Installation");
  
  const args = process.argv.slice(2);
  const getArg = (flag: string) => {
    const idx = args.indexOf(flag);
    return idx !== -1 && idx + 1 < args.length ? args[idx + 1] : "";
  };

  const argEmail = getArg("--email");
  const argPassword = getArg("--password");
  const argSchool = getArg("--school");
  const argSchoolAr = getArg("--school-ar");
  const argAdmin = getArg("--admin");
  const argAdminAr = getArg("--admin-ar");
  const argPhone = getArg("--phone");
  const argTimezone = getArg("--timezone");

  const isAutomated = !!(argEmail && argPassword && argSchool && argSchoolAr);

  const dbPath = getDbPath();
  const dbDirName = path.dirname(dbPath);
  printInfo(`Database location resolved: ${dbPath}`);

  // Ensure database directory exists
  if (!existsSync(dbDirName)) {
    mkdirSync(dbDirName, { recursive: true });
  }

  const db = new Database(dbPath);

  // Check if tables are already created
  let tablesExist = false;
  try {
    const check = db.query("SELECT name FROM sqlite_master WHERE type='table' AND name='organizations'").get() as any;
    if (check && check.name === 'organizations') {
      tablesExist = true;
    }
  } catch (e) {
    tablesExist = false;
  }

  // Run migrations if tables do not exist
  if (!tablesExist) {
    runDatabaseMigration(db);
  } else {
    printInfo("Database schema table mappings verified.");
  }

  // Check if admin user already exists
  let adminExists = false;
  try {
    const adminCount = db.query("SELECT COUNT(*) as count FROM users WHERE primary_role = 'admin'").get() as any;
    if (adminCount && adminCount.count > 0) {
      adminExists = true;
    }
  } catch (e) {}

  if (adminExists) {
    printSuccess("An administrator account already exists. Skipping user registration.");
    console.log(c.purple("└  Installation completed successfully! Run 'npm run dev' to open the UI."));
    process.exit(0);
  }

  let schoolNameAr = argSchoolAr;
  let schoolNameEn = argSchool;
  let timezone = argTimezone || "Asia/Aden";
  
  // Default values loaded automatically without prompts
  const currency = "YER";
  const locale = "ar";

  let adminNameAr = argAdminAr || "مدير المدرسة";
  let adminNameEn = argAdmin || "School Admin";
  let adminEmail = argEmail;
  let adminPhone = argPhone || "777777777";
  let adminPassword = argPassword;

  if (!isAutomated) {
    printStep("School Information / إعداد بيانات المدرسة");
    schoolNameAr = await ask("Official School Name (Arabic) / اسم المدرسة الرسمي باللغة العربية", "مدرسة صنعاء النموذجية");
    schoolNameEn = await ask("School Name (English) / اسم المدرسة باللغة الإنجليزية", "Sanaa Model School");
    timezone = await ask("Timezone / المنطقة الزمنية", "Asia/Aden");
    console.log(c.gray("│"));

    printStep("Administrator Account / إعداد حساب المدير");
    adminNameAr = await ask("Administrator Full Name (Arabic) / اسم المدير باللغة العربية", "مدير المدرسة");
    adminNameEn = await ask("Administrator Full Name (English) / اسم المدير باللغة الإنجليزية", "School Admin");
    adminEmail = await ask("Login Email Address / البريد الإلكتروني لتسجيل الدخول", "admin@school.com", (val) => {
      return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val);
    });
    adminPhone = await ask("Phone Number (Optional) / رقم الهاتف (اختياري)", "777777777");
    
    while (adminPassword.length < 6) {
      adminPassword = await askPassword("Login Password (Min 6 chars) / كلمة المرور (6 أحرف على الأقل)");
      if (adminPassword.length < 6) {
        console.log(`${c.gray("│")}  ${c.red("✖  Password must be at least 6 characters.")}`);
      }
    }
    console.log(c.gray("│"));
  }

  printInfo("Encrypting credentials and bootstrapping database...");

  // Hash password using bcrypt
  const passwordHash = await Bun.password.hash(adminPassword, {
    algorithm: "bcrypt",
    cost: 10,
  });

  const orgId = crypto.randomUUID();
  const userId = crypto.randomUUID();
  const orgSlug = schoolNameEn.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-|-$)/g, "") || "school";
  const now = new Date().toISOString();

  db.run("PRAGMA foreign_keys = OFF;");
  
  // Insert Organization
  db.run(
    `INSERT INTO organizations (
      id, name, name_ar, slug, timezone, locale, currency, hijri_enabled,
      max_users, max_students, max_storage_gb, subscription_plan, is_active,
      is_verified, is_deleted, created_at, updated_at
    ) VALUES (?, ?, ?, ?, ?, ?, ?, 1, 1000, 5000, 100, 'free', 1, 1, 0, ?, ?)`,
    [orgId, schoolNameEn, schoolNameAr, orgSlug, timezone, locale, currency, now, now]
  );

  // Insert Admin User
  db.run(
    `INSERT INTO users (
      id, org_id, email, email_verified, phone, phone_verified, password_hash,
      mfa_enabled, full_name, name_ar, name_en, primary_role, status, prefs, meta,
      is_deleted, created_at, updated_at
    ) VALUES (?, ?, ?, 1, ?, 1, ?, 0, ?, ?, ?, 'admin', 'active', '{}', '{}', 0, ?, ?)`,
    [userId, orgId, adminEmail, adminPhone, passwordHash, adminNameEn, adminNameAr, adminNameEn, now, now]
  );

  // Automatically seed Default Currencies, Chart of Accounts, and default permissions
  seedDefaultFinancialsAndPermissions(db, orgId, currency);

  db.run("PRAGMA foreign_keys = ON;");

  printSuccess("Organization and administrator record successfully created!");
  console.log(c.purple("└  Installation completed successfully!"));
  console.log(`   ${c.bold("School:")}      ${schoolNameAr} (${schoolNameEn})`);
  console.log(`   ${c.bold("Login Email:")}  ${adminEmail}`);
  console.log(`   ${c.bold("DB Path:")}      ${dbPath}\n`);
  console.log(c.cyan("   You can now launch the Orbis app and log in with this account."));
  console.log(c.cyan("   يمكنك الآن فتح التطبيق وتسجيل الدخول بحسابك لمتابعة تهيئة النظام.\n"));
}

main().catch((err) => {
  console.log(`${c.gray("│")}  ${c.red(`✖  Installation failed: ${err}`)}`);
  process.exit(1);
});
