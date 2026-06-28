#!/usr/bin/env bun
import os from "node:os";
import path from "node:path";
import { existsSync, mkdirSync, readFileSync, unlinkSync } from "node:fs";
import { Database } from "bun:sqlite";
import readline from "node:readline";

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

function printError(message: string) {
  console.log(c.red(`✖  ${message}`));
  console.log(c.gray("│"));
}

// ── Interactive Question Helpers ──────────────────────────────────────────

async function ask(
  query: string,
  defaultValue = "",
  validator?: (val: string) => boolean,
): Promise<string> {
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
        printError("Invalid input, please try again.");
        resolve(ask(query, defaultValue, validator));
      } else {
        resolve(finalValue);
      }
    });
  });
}

async function askPassword(query: string): Promise<string> {
  return new Promise((resolve) => {
    process.stdout.write(`${c.gray("│")}  ${query}: `);

    let password = "";
    const isRaw = process.stdin.isRaw;
    process.stdin.setRawMode(true);
    process.stdin.resume();

    const handler = (char: Buffer) => {
      const code = char[0];

      // Enter key (Carriage Return = 13, Line Feed = 10)
      if (code === 13 || code === 10) {
        process.stdin.removeListener("data", handler);
        process.stdin.setRawMode(isRaw);
        process.stdin.pause();
        process.stdout.write("\n");
        resolve(password);
      }
      // Ctrl+C (3)
      else if (code === 3) {
        process.exit(1);
      }
      // Backspace (127 or 8)
      else if (code === 127 || code === 8) {
        if (password.length > 0) {
          password = password.slice(0, -1);
          process.stdout.write("\b \b");
        }
      }
      // Printable characters
      else {
        const s = char.toString("utf-8");
        // Filter out control sequences and escape characters
        const printable = s.replace(/[\x00-\x1F\x7F-\x9F]/g, "");
        if (printable.length > 0) {
          password += printable;
          process.stdout.write("*".repeat(printable.length));
        }
      }
    };

    process.stdin.on("data", handler);
  });
}

// ── Database Path & Identifier Resolver ────────────────────────────────────

function getDbPathSingle(identifier: string): string {
  const home = os.homedir();
  let appDataDir = "";

  if (process.platform === "win32") {
    const appData =
      process.env.APPDATA || path.join(home, "AppData", "Roaming");
    appDataDir = path.join(appData, identifier);
  } else if (process.platform === "darwin") {
    appDataDir = path.join(home, "Library", "Application Support", identifier);
  } else {
    const xdgData =
      process.env.XDG_DATA_HOME || path.join(home, ".local", "share");
    appDataDir = path.join(xdgData, identifier);
  }

  return path.join(appDataDir, "orbis.db");
}

function getDbPaths(isDevExplicit = false): string[] {
  const paths: string[] = [];

  if (isDevExplicit) {
    paths.push(getDbPathSingle("com.mulverse.orbis.dev"));
  } else {
    // production database
    paths.push(getDbPathSingle("com.mulverse.orbis.svelte"));

    // Auto-detect if development directory exists on the system
    const devPath = path.dirname(getDbPathSingle("com.mulverse.orbis.dev"));
    if (existsSync(devPath)) {
      paths.push(getDbPathSingle("com.mulverse.orbis.dev"));
    }
  }

  // Remove duplicates
  return Array.from(new Set(paths));
}

// ── Help & Usage Guides ─────────────────────────────────────────────────────

function printHelp() {
  console.log(c.bold("Usage:"));
  console.log(`  bun run start [command] [options]\n`);

  console.log(c.bold("Commands:"));
  console.log(
    `  ${c.cyan("init")}          Start school system installation & db migrations (Default)`,
  );
  console.log(
    `  ${c.cyan("db:reset")}      Completely reset and delete local SQLite database files\n`,
  );

  console.log(c.bold("Options:"));
  console.log(`  ${c.yellow("-h, --help")}    Show this help message`);
  console.log(`  ${c.yellow("-v, --version")} Show installer CLI version`);
  console.log(
    `  ${c.yellow("--dev")}          Target only the development database (com.mulverse.orbis.dev)\n`,
  );

  console.log(c.bold("Auto-Install Options (for silent installations):"));
  console.log(`  ${c.purple("--school")}       English School Name`);
  console.log(`  ${c.purple("--school-ar")}    Arabic School Name`);
  console.log(`  ${c.purple("--email")}        Admin login email address`);
  console.log(
    `  ${c.purple("--password")}     Admin password (min 6 characters)`,
  );
  console.log(
    `  ${c.purple("--admin")}        Admin English name (default: School Admin)`,
  );
  console.log(
    `  ${c.purple("--admin-ar")}     Admin Arabic name (default: مدير المدرسة)`,
  );
  console.log(`  ${c.purple("--timezone")}     Timezone (default: Asia/Aden)`);
  console.log(`  ${c.purple("--phone")}        Phone number`);
  console.log();
}

// ── Database Schema Migration ──────────────────────────────────────────────

function runDatabaseMigration(db: Database) {
  const dbDir = path.resolve(
    __dirname,
    "..",
    "..",
    "..",
    "apps",
    "api-rust",
    "crates",
    "core-domain",
    "src",
    "database",
  );
  const coreSchemaPath = path.join(dbDir, "core_schema.sql");
  const schemaPath = path.join(dbDir, "schema.sql");

  if (!existsSync(coreSchemaPath) || !existsSync(schemaPath)) {
    printError(`Schema SQL files not found in: ${dbDir}`);
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
        .map((line) => line.trim())
        .filter((line) => !line.startsWith("--") && line.length > 0)
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
}

// ── Db Reset Command ────────────────────────────────────────────────────────

async function handleDatabaseReset(isDev = false) {
  printHeader("Orbis Database Reset Tool");
  const dbPaths = getDbPaths(isDev);

  let foundAny = false;
  for (const dbPath of dbPaths) {
    if (existsSync(dbPath)) {
      foundAny = true;
    }
  }

  if (!foundAny) {
    printInfo(`No database files found in standard locations.`);
    console.log(c.purple("└  Completed."));
    return;
  }

  const confirm = await ask(
    "Confirm deleting database files and wiping all data? (yes/no)",
    "no",
  );
  if (confirm.toLowerCase() === "yes") {
    for (const dbPath of dbPaths) {
      if (existsSync(dbPath)) {
        try {
          unlinkSync(dbPath);
          printSuccess(`Database deleted successfully from ${dbPath}`);
        } catch (e) {
          printError(`Failed to delete database at ${dbPath}: ${e}`);
        }
      }
    }
  } else {
    printInfo("Reset canceled by user.");
  }

  console.log(c.purple("└  Completed."));
}

// ── Custom Recursive COA & Financials Seeder ───────────────────────────────

interface AccountNode {
  code: string;
  name_ar: string;
  name_en: string;
  type: string;
  level: number;
  children?: AccountNode[];
}

function seedDefaultFinancialsAndPermissions(
  db: Database,
  orgId: string,
  baseCurrency: string,
) {
  const now = new Date().toISOString();

  // 1. Seed Currencies (YER, USD, SAR)
  const defaultCurrencies = [
    {
      code: "YER",
      name: "Yemeni Rial",
      nameAr: "ريال يمني",
      symbol: "﷼",
      position: "after",
      decimals: 0,
      rate: 1.0,
    },
    {
      code: "USD",
      name: "US Dollar",
      nameAr: "دولار أمريكي",
      symbol: "$",
      position: "before",
      decimals: 2,
      rate: 250.0,
    },
    {
      code: "SAR",
      name: "Saudi Riyal",
      nameAr: "ريال سعودي",
      symbol: "ر.س",
      position: "after",
      decimals: 2,
      rate: 66.0,
    },
  ];

  for (const cur of defaultCurrencies) {
    const isBase = cur.code === baseCurrency ? 1 : 0;
    db.run(
      `INSERT OR IGNORE INTO currencies (
        code, org_id, name, name_ar, symbol, symbol_position, decimal_places, exchange_rate, is_base, is_active, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?)`,
      [
        cur.code,
        orgId,
        cur.name,
        cur.nameAr,
        cur.symbol,
        cur.position,
        cur.decimals,
        cur.rate,
        isBase,
        now,
      ],
    );
  }

  // 2. Seed Chart of Accounts from JSON
  const jsonPath = path.join(__dirname, "educational_chart_of_accounts.json");

  if (!existsSync(jsonPath)) {
    printError(`Local Chart of Accounts JSON file not found at ${jsonPath}`);
    process.exit(1);
  }

  const coaData = JSON.parse(readFileSync(jsonPath, "utf-8"));
  const accounts: AccountNode[] = coaData.accounts;

  // Helper to map type to lowercase database types
  function getDbType(type: string): string {
    const t = type.toLowerCase();
    if (
      t === "asset" ||
      t === "liability" ||
      t === "equity" ||
      t === "revenue" ||
      t === "expense"
    ) {
      return t;
    }
    return "asset";
  }

  // Helper to map code to system required sub-types
  function getSubType(code: string): string | null {
    if (code === "110101") return "treasury_control";
    if (code === "110301") return "student_receivable";
    if (code === "210401") return "employee_payable";
    if (code === "3201") return "retained_earnings";
    if (code === "4101") return "tuition_revenue";
    if (code === "5101") return "salaries_expense";
    return null;
  }

  // Helper to determine control status
  function isControl(code: string, hasChildren: boolean): number {
    if (hasChildren) return 1;
    if (code === "110101" || code === "110301" || code === "210401") return 1;
    return 0;
  }

  // Recursive insertion function
  function insertAccount(
    node: AccountNode,
    parentId: string | null,
    parentPath: string | null,
  ) {
    const fullId = `${orgId}_${node.code}`;
    const dbType = getDbType(node.type);
    const subType = getSubType(node.code);

    const hasChildren = !!(node.children && node.children.length > 0);
    const controlAccount = isControl(node.code, hasChildren);
    const systemAccount = 1;

    const pathVal = parentPath ? `${parentPath}/${fullId}` : fullId;
    const depth = node.level - 1;

    db.run(
      `INSERT OR IGNORE INTO ledger_accounts (
        id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_control_account, is_system_account, is_active, created_at, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0.0, 0.0, ?, ?, 1, ?, ?)`,
      [
        fullId,
        orgId,
        node.code,
        node.name_en,
        node.name_ar,
        dbType,
        subType,
        parentId,
        pathVal,
        depth,
        baseCurrency,
        controlAccount,
        systemAccount,
        now,
        now,
      ],
    );

    if (node.children) {
      for (const child of node.children) {
        insertAccount(child, fullId, pathVal);
      }
    }
  }

  for (const acc of accounts) {
    insertAccount(acc, null, null);
  }

  // 3. Seed Default Permissions
  const permissions = [
    {
      code: "OrganizationRead",
      name: "Read Organization Info",
      nameAr: "عرض معلومات المنشأة",
      module: "organization",
      action: "read",
      resource: "info",
    },
    {
      code: "OrganizationWrite",
      name: "Write Organization Info",
      nameAr: "تعديل معلومات المنشأة",
      module: "organization",
      action: "write",
      resource: "info",
    },
    {
      code: "UserRead",
      name: "Read Users Directory",
      nameAr: "استعراض دليل المستخدمين",
      module: "users",
      action: "read",
      resource: "directory",
    },
    {
      code: "UserWrite",
      name: "Create and Edit Users",
      nameAr: "إنشاء وتعديل حسابات المستخدمين",
      module: "users",
      action: "write",
      resource: "account",
    },
    {
      code: "UserDelete",
      name: "Delete Users",
      nameAr: "حذف حسابات المستخدمين",
      module: "users",
      action: "delete",
      resource: "account",
    },
    {
      code: "CycleRead",
      name: "Read Academic Cycles",
      nameAr: "عرض الفترات والأعوام الدراسية",
      module: "academics",
      action: "read",
      resource: "cycle",
    },
    {
      code: "CycleWrite",
      name: "Create and Edit Cycles",
      nameAr: "إنشاء وتعديل الفترات والأعوام",
      module: "academics",
      action: "write",
      resource: "cycle",
    },
  ];

  for (const p of permissions) {
    db.run(
      `INSERT OR IGNORE INTO permissions (
        code, name, name_ar, description, module, action, resource, requires_scope, scope_types, is_system, is_active, created_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, 0, '["global"]', 1, 1, ?)`,
      [p.code, p.name, p.nameAr, p.name, p.module, p.action, p.resource, now],
    );
  }
}

// ── Main Commands Orchestration ─────────────────────────────────────────────

async function main() {
  const args = process.argv.slice(2);
  const command = args[0];
  const isDev = args.includes("--dev");

  // Help command
  if (args.includes("-h") || args.includes("--help") || command === "help") {
    printHeader("Orbis OS Installer CLI Help");
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
    await handleDatabaseReset(isDev);
    return;
  }

  // Fallback to init command
  await handleInstallation(isDev);
}

async function handleInstallation(isDev = false) {
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

  const dbPaths = getDbPaths(isDev);
  printInfo(`Resolved database targets:`);
  for (const dbPath of dbPaths) {
    console.log(`${c.gray("│")}   - ${c.cyan(dbPath)}`);
  }
  console.log(c.gray("│"));

  // Ensure all parent directories exist
  for (const dbPath of dbPaths) {
    const dir = path.dirname(dbPath);
    if (!existsSync(dir)) {
      mkdirSync(dir, { recursive: true });
    }
  }

  // Run schema migration on all databases
  for (const dbPath of dbPaths) {
    const db = new Database(dbPath);
    let tablesExist = false;
    try {
      const check = db
        .query(
          "SELECT name FROM sqlite_master WHERE type='table' AND name='organizations'",
        )
        .get() as any;
      if (check && check.name === "organizations") {
        tablesExist = true;
      }
    } catch (e) {}

    if (!tablesExist) {
      printInfo(
        `Initializing database schema table mappings for: ${path.basename(path.dirname(dbPath))}`,
      );
      runDatabaseMigration(db);
    }
    db.close();
  }

  // Check if admin user already exists on the main database
  const primaryDb = new Database(dbPaths[0]);
  let adminExists = false;
  try {
    const adminCount = primaryDb
      .query("SELECT COUNT(*) as count FROM users WHERE primary_role = 'admin'")
      .get() as any;
    if (adminCount && adminCount.count > 0) {
      adminExists = true;
    }
  } catch (e) {}
  primaryDb.close();

  if (adminExists) {
    printSuccess(
      "An administrator account already exists. Skipping user registration.",
    );
    console.log(
      c.purple("└  Installation completed successfully! Launch the UI."),
    );
    process.exit(0);
  }

  const schoolNameAr = argSchoolAr || "";
  const schoolNameEn = schoolNameEnVal();
  function schoolNameEnVal() {
    return argSchool || "";
  }
  const timezone = argTimezone || "Asia/Aden";

  // Default values loaded automatically without prompts
  const currency = "YER";
  const locale = "ar";

  let adminNameAr = argAdminAr || "مدير المدرسة";
  let adminNameEn = argAdmin || "School Admin";
  let adminEmail = argEmail || "";
  let adminPhone = argPhone || "777777777";
  let adminPassword = argPassword || "";

  let finalSchoolNameAr = schoolNameAr;
  let finalSchoolNameEn = schoolNameEn;
  let finalTimezone = timezone;

  if (!isAutomated) {
    printStep("School Information Setup");
    finalSchoolNameAr = await ask(
      "Enter School Name (Arabic)",
      "مدرسة صنعاء النموذجية",
    );
    finalSchoolNameEn = await ask(
      "Enter School Name (English)",
      "Sanaa Model School",
    );
    finalTimezone = await ask("Enter Timezone", "Asia/Aden");
    console.log(c.gray("│"));

    printStep("Administrator Account Setup");
    adminNameAr = await ask(
      "Enter Administrator Name (Arabic)",
      "مدير المدرسة",
    );
    adminNameEn = await ask(
      "Enter Administrator Name (English)",
      "School Admin",
    );
    adminEmail = await ask(
      "Enter Login Email Address",
      "admin@school.com",
      (val) => {
        return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val);
      },
    );
    adminPhone = await ask("Enter Phone Number (Optional)", "777777777");

    while (adminPassword.trim().length < 6) {
      adminPassword = await askPassword("Enter Login Password (Min 6 chars)");
      if (adminPassword.trim().length < 6) {
        printError("Password must be at least 6 characters.");
      }
    }
    console.log(c.gray("│"));
  }

  printInfo("Encrypting credentials and bootstrapping databases...");

  // Hash password using bcrypt
  const passwordHash = await Bun.password.hash(adminPassword.trim(), {
    algorithm: "bcrypt",
    cost: 10,
  });

  const orgId = crypto.randomUUID();
  const userId = crypto.randomUUID();
  const orgSlug =
    finalSchoolNameEn
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/(^-|-$)/g, "") || "school";
  const now = new Date().toISOString();

  // Populate all target databases
  for (const dbPath of dbPaths) {
    const db = new Database(dbPath);
    db.run("PRAGMA foreign_keys = OFF;");

    // Insert Organization
    db.run(
      `INSERT OR IGNORE INTO organizations (
        id, name, name_ar, slug, timezone, locale, currency, hijri_enabled,
        max_users, max_students, max_storage_gb, subscription_plan, is_active,
        is_verified, is_deleted, created_at, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, 1, 1000, 5000, 100, 'free', 1, 1, 0, ?, ?)`,
      [
        orgId,
        finalSchoolNameEn,
        finalSchoolNameAr,
        orgSlug,
        finalTimezone,
        locale,
        currency,
        now,
        now,
      ],
    );

    // Insert Admin User
    db.run(
      `INSERT OR IGNORE INTO users (
        id, org_id, email, email_verified, phone, phone_verified, password_hash,
        mfa_enabled, full_name, name_ar, name_en, primary_role, status, prefs, meta,
        is_deleted, created_at, updated_at
      ) VALUES (?, ?, ?, 1, ?, 1, ?, 0, ?, ?, ?, 'admin', 'active', '{}', '{}', 0, ?, ?)`,
      [
        userId,
        orgId,
        adminEmail,
        adminPhone,
        passwordHash,
        adminNameEn,
        adminNameAr,
        adminNameEn,
        now,
        now,
      ],
    );

    // Automatically seed Default Currencies, Chart of Accounts, and default permissions
    seedDefaultFinancialsAndPermissions(db, orgId, currency);

    db.run("PRAGMA foreign_keys = ON;");
    db.close();
  }

  printSuccess("Organization and administrator record successfully created!");
  console.log(c.purple("└  Installation completed successfully!"));
  console.log(
    `   ${c.bold("School:")}      ${finalSchoolNameAr} (${finalSchoolNameEn})`,
  );
  console.log(`   ${c.bold("Login Email:")}  ${adminEmail}`);
  console.log(`   ${c.bold("Main DB:")}       ${dbPaths[0]}`);
  if (dbPaths.length > 1) {
    console.log(`   ${c.bold("Dev DB:")}        ${dbPaths[1]}`);
  }
  console.log();
  console.log(
    c.cyan("   You can now launch the Orbis app and log in with this account."),
  );
  console.log(
    c.cyan(
      "   You can now open the app and log in to continue configuring the system.\n",
    ),
  );
}

main().catch((err) => {
  console.log(`${c.gray("│")}  ${c.red(`✖  Installation failed: ${err}`)}`);
  process.exit(1);
});
