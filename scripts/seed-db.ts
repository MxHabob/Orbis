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

console.log("Found database at:", dbPath);
const db = new Database(dbPath);

try {
  // Let's get or create the organization
  let orgId = "org-sanaa-model";
  const existingOrg = db.query("SELECT id FROM organizations LIMIT 1").get() as { id: string } | null;
  if (existingOrg) {
    orgId = existingOrg.id;
    console.log("✅ Found active Organization ID:", orgId);
  } else {
    console.log("Creating default organization: مدرسة صنعاء النموذجية");
    db.prepare(`
      INSERT INTO organizations (id, name, slug, timezone, locale, currency, hijri_enabled, max_users, max_students, max_storage_gb, subscription_plan, is_active, is_verified, is_deleted, primary_color, created_at, updated_at)
      VALUES (?, 'مدرسة صنعاء النموذجية', 'sanaa-model-school', 'Asia/Aden', 'ar', 'YER', 1, 100, 1000, 50, 'premium', 1, 1, 0, '#8b5cf6', datetime('now'), datetime('now'))
    `).run(orgId);
  }

  // Ensure default admin user exists
  const existingUser = db.query("SELECT id FROM users WHERE primary_role = 'admin' LIMIT 1").get() as { id: string } | null;
  if (!existingUser) {
    console.log("Creating default admin user...");
    const adminId = crypto.randomUUID();
    db.prepare(`
      INSERT INTO users (id, org_id, email, email_verified, phone_verified, mfa_enabled, full_name, name_ar, primary_role, status, prefs, meta, is_deleted, created_at, updated_at)
      VALUES (?, ?, 'admin@orbis.edu', 1, 0, 0, 'Admin User', 'مدير النظام', 'admin', 'active', '{}', '{}', 0, datetime('now'), datetime('now'))
    `).run(adminId, orgId);

    db.prepare(`
      INSERT INTO user_roles (id, user_id, role, org_id, scope_type, is_primary, assigned_at)
      VALUES (?, ?, 'admin', ?, 'org', 1, datetime('now'))
    `).run(crypto.randomUUID(), adminId, orgId);
  }

  // 1. Create Academic Cycle (Hijri 1447)
  const cycleId = "cycle-1447";
  db.prepare("DELETE FROM academic_cycles WHERE id = ?").run(cycleId);
  console.log("Creating academic cycle: 1447 Hijri...");
  db.prepare(`
    INSERT INTO academic_cycles (id, org_id, name, name_ar, start_date, end_date, type, grading_system, is_current, is_locked, is_enrollment_open, created_at, updated_at)
    VALUES (?, ?, 'Academic Year 1447 Hijri', 'العام الدراسي 1447 هـ', '2026-06-21', '2027-02-21', 'yearly', 'percentage', 1, 0, 1, datetime('now'), datetime('now'))
  `).run(cycleId, orgId);

  // 2. Create Standard Time Slots
  db.prepare("DELETE FROM time_slots WHERE cycle_id = ?").run(cycleId);
  console.log("Creating time slots...");
  const defaultTimeSlots = [
    { name: "1st Period", name_ar: "الحصة الأولى", start_time: "08:00", end_time: "08:45" },
    { name: "2nd Period", name_ar: "الحصة الثانية", start_time: "08:45", end_time: "09:30" },
    { name: "3rd Period", name_ar: "الحصة الثالثة", start_time: "09:45", end_time: "10:30" },
    { name: "4th Period", name_ar: "الحصة الرابعة", start_time: "10:30", end_time: "11:15" },
    { name: "5th Period", name_ar: "الحصة الخامسة", start_time: "11:30", end_time: "12:15" },
  ];
  defaultTimeSlots.forEach((ts, idx) => {
    const [sh, sm] = ts.start_time.split(":").map(Number);
    const [eh, em] = ts.end_time.split(":").map(Number);
    const duration = (eh * 60 + em) - (sh * 60 + sm);
    db.prepare(`
      INSERT INTO time_slots (id, org_id, cycle_id, slot_number, name, name_ar, start_time, end_time, duration_minutes, is_break, is_active, created_at, updated_at)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 1, datetime('now'), datetime('now'))
    `).run(crypto.randomUUID(), orgId, cycleId, idx + 1, ts.name, ts.name_ar, ts.start_time, ts.end_time, duration);
  });

  // 3. Create Root Org Unit (School)
  const unitId = "unit-sanaa-model";
  db.prepare("DELETE FROM org_units WHERE id = ?").run(unitId);
  console.log("Creating root school org unit...");
  db.prepare(`
    INSERT INTO org_units (id, org_id, name, name_ar, type, path, depth, is_active, created_at, updated_at)
    VALUES (?, ?, 'Sanaa Model School', 'مدرسة صنعاء النموذجية', 'school', 'school', 0, 1, datetime('now'), datetime('now'))
  `).run(unitId, orgId);

  // 4. Create Buildings and Rooms
  db.prepare("DELETE FROM rooms WHERE org_id = ?").run(orgId);
  console.log("Creating rooms for Building 1 & Building 2...");
  
  // Building 1: 2 floors, 20 classrooms, 3 computer labs, 2 chemistry labs
  // Floor 1: rooms 1 to 10
  for (let i = 1; i <= 10; i++) {
    const rId = `room-b1-f1-${i}`;
    db.prepare(`
      INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, building, floor, is_active, created_at, updated_at)
      VALUES (?, ?, ?, ?, 30, 'classroom', 'المبنى 1', 1, 1, datetime('now'), datetime('now'))
    `).run(rId, orgId, `Classroom 1-0${i}`, `فصل 1-0${i}`);
  }
  // Floor 2: rooms 11 to 20
  for (let i = 11; i <= 20; i++) {
    const rId = `room-b1-f2-${i}`;
    db.prepare(`
      INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, building, floor, is_active, created_at, updated_at)
      VALUES (?, ?, ?, ?, 30, 'classroom', 'المبنى 1', 2, 1, datetime('now'), datetime('now'))
    `).run(rId, orgId, `Classroom 1-${i}`, `فصل 1-${i}`);
  }
  // Computer labs
  for (let i = 1; i <= 3; i++) {
    const rId = `room-b1-lab-comp-${i}`;
    db.prepare(`
      INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, building, floor, is_active, created_at, updated_at)
      VALUES (?, ?, ?, ?, 25, 'computer_lab', 'المبنى 1', 2, 1, datetime('now'), datetime('now'))
    `).run(rId, orgId, `Computer Lab 1-${i}`, `مختبر الحاسوب 1-${i}`);
  }
  // Chemistry labs
  for (let i = 1; i <= 2; i++) {
    const rId = `room-b1-lab-chem-${i}`;
    db.prepare(`
      INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, building, floor, is_active, created_at, updated_at)
      VALUES (?, ?, ?, ?, 25, 'chemistry_lab', 'المبنى 1', 1, 1, datetime('now'), datetime('now'))
    `).run(rId, orgId, `Chemistry Lab 1-${i}`, `مختبر الكيمياء 1-${i}`);
  }

  // Building 2: 3 floors, 7 classrooms per floor, total 21
  for (let floor = 1; floor <= 3; floor++) {
    for (let room = 1; room <= 7; room++) {
      const roomNum = `${floor}-0${room}`;
      const rId = `room-b2-f${floor}-${room}`;
      db.prepare(`
        INSERT INTO rooms (id, org_id, name, name_ar, capacity, room_type, building, floor, is_active, created_at, updated_at)
        VALUES (?, ?, ?, ?, 30, 'classroom', 'المبنى 2', ?, 1, datetime('now'), datetime('now'))
      `).run(rId, orgId, `Classroom 2-${roomNum}`, `فصل 2-${roomNum}`, floor);
    }
  }

  // 5. Create Subjects
  db.prepare("DELETE FROM subjects WHERE org_id = ?").run(orgId);
  console.log("Creating subjects...");
  const subjectsData = [
    { code: "QUR", name: "Holy Quran", name_ar: "القرآن الكريم", color: "#10b981", icon: "book-open", sort: 1 },
    { code: "ISL", name: "Islamic Studies", name_ar: "الدراسات الإسلامية", color: "#0d9488", icon: "bookmark", sort: 2 },
    { code: "ARB", name: "Arabic Language", name_ar: "اللغة العربية", color: "#f59e0b", icon: "pen-tool", sort: 3 },
    { code: "MTH", name: "Mathematics", name_ar: "الرياضيات", color: "#3b82f6", icon: "binary", sort: 4 },
    { code: "SCI", name: "Science", name_ar: "العلوم", color: "#8b5cf6", icon: "flask-conical", sort: 5 },
    { code: "ENG", name: "English Language", name_ar: "اللغة الإنجليزية", color: "#6366f1", icon: "languages", sort: 6 },
    { code: "SOC", name: "Social Studies", name_ar: "الدراسات الاجتماعية", color: "#f97316", icon: "globe", sort: 7 },
  ];
  subjectsData.forEach((s) => {
    const sId = `subject-${s.code.toLowerCase()}`;
    db.prepare(`
      INSERT INTO subjects (id, org_id, unit_id, code, name, name_ar, is_core, is_elective, requires_lab, requires_practical, color, icon, sort_order, is_active, created_at, updated_at)
      VALUES (?, ?, ?, ?, ?, ?, 1, 0, 0, 0, ?, ?, ?, 1, datetime('now'), datetime('now'))
    `).run(sId, orgId, unitId, s.code, s.name, s.name_ar, s.color, s.icon, s.sort);
  });

  // 6. Create Stages & Classes
  db.prepare("DELETE FROM classes WHERE org_id = ?").run(orgId);
  console.log("Creating classes...");
  const classesData = [
    { level: "kg1", name: "KG1", name_ar: "روضة أولى", color: "#f43f5e" },
    { level: "kg2", name: "KG2", name_ar: "روضة ثانية", color: "#ec4899" },
    { level: "grade1", name: "Grade 1", name_ar: "الصف الأول", color: "#d946ef" },
    { level: "grade2", name: "Grade 2", name_ar: "الصف الثاني", color: "#a855f7" },
    { level: "grade3", name: "Grade 3", name_ar: "الصف الثالث", color: "#6366f1" },
    { level: "grade4", name: "Grade 4", name_ar: "الصف الرابع", color: "#3b82f6" },
    { level: "grade5", name: "Grade 5", name_ar: "الصف الخامس", color: "#0ea5e9" },
    { level: "grade6", name: "Grade 6", name_ar: "الصف السادس", color: "#06b6d4" },
    { level: "grade7", name: "Grade 7", name_ar: "الصف السابع", color: "#14b8a6" },
    { level: "grade8", name: "Grade 8", name_ar: "الصف الثامن", color: "#10b981" },
    { level: "grade9", name: "Grade 9", name_ar: "الصف التاسع", color: "#22c55e" },
  ];

  const createdClassIds: string[] = [];
  classesData.forEach((c) => {
    const cId = `class-${c.level}`;
    db.prepare(`
      INSERT INTO classes (id, org_id, unit_id, cycle_id, name, name_ar, level, division, capacity, color, is_active, is_deleted, created_at, updated_at)
      VALUES (?, ?, ?, ?, ?, ?, ?, 'أ', 30, ?, 1, 0, datetime('now'), datetime('now'))
    `).run(cId, orgId, unitId, cycleId, c.name, c.name_ar, c.level, c.color);
    createdClassIds.push(cId);
  });

  // 7. Create 32 Teachers
  console.log("Generating 32 teachers...");
  const now = new Date().toISOString();
  
  // Clean previous teachers for clean setup
  db.prepare("DELETE FROM user_roles WHERE role = 'teacher' AND org_id = ?").run(orgId);
  db.prepare("DELETE FROM users WHERE primary_role = 'teacher' AND org_id = ?").run(orgId);

  const teacherIds: string[] = [];
  for (let i = 1; i <= 32; i++) {
    const userId = crypto.randomUUID();
    const roleId = crypto.randomUUID();
    const email = `teacher${i}@orbis.edu`;
    const fullName = `Teacher ${i}`;
    const nameAr = `معلم ${i}`;

    db.prepare(`
      INSERT INTO users (
        id, org_id, email, email_verified, phone_verified, mfa_enabled,
        full_name, name_ar, primary_role, status, prefs, meta, is_deleted,
        created_at, updated_at
      ) VALUES (?, ?, ?, 1, 0, 0, ?, ?, 'teacher', 'active', '{}', '{}', 0, ?, ?)
    `).run(userId, orgId, email, fullName, nameAr, now, now);

    db.prepare(`
      INSERT INTO user_roles (
        id, user_id, role, org_id, scope_type, is_primary, assigned_at
      ) VALUES (?, ?, 'teacher', ?, 'org', 1, ?)
    `).run(roleId, userId, orgId, now);

    teacherIds.push(userId);
  }

  // 8. Create Class Subjects & Associate Teachers
  db.prepare("DELETE FROM class_subjects WHERE cycle_id = ?").run(cycleId);
  console.log("Associating subjects to classes and distributing teachers...");
  
  let teacherIndex = 0;
  createdClassIds.forEach((classId) => {
    subjectsData.forEach((s) => {
      const csId = crypto.randomUUID();
      const subjectId = `subject-${s.code.toLowerCase()}`;
      const teacherId = teacherIds[teacherIndex % teacherIds.length];
      teacherIndex++;

      db.prepare(`
        INSERT INTO class_subjects (id, class_id, subject_id, cycle_id, primary_teacher_id, weekly_hours, is_active, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, 4, 1, datetime('now'), datetime('now'))
      `).run(csId, classId, subjectId, cycleId, teacherId);
    });
  });

  // 9. COA & Treasuries Seeding
  console.log("Seeding Chart of Accounts (COA) and Treasuries...");
  db.prepare("DELETE FROM treasuries WHERE org_id = ?").run(orgId);
  db.prepare("DELETE FROM ledger_accounts WHERE org_id = ?").run(orgId);

  // Asset Ledger Accounts
  const ledgerAccounts = [
    { id: 'ledger-1101-safe-main', code: '1101', name: 'School Main Safe Cashbox', name_ar: 'حساب صندوق النقدية الرئيسي للمدرسة', type: 'asset', sub_type: 'cash', is_cash: 1, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-1102-safe-petty', code: '1102', name: 'Petty Cash Safe Box', name_ar: 'حساب صندوق العهدة والنثرية', type: 'asset', sub_type: 'cash', is_cash: 1, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-1201-bank-kuraimi', code: '1201', name: 'Kuraimi Current Corporate Bank', name_ar: 'حساب بنك الكريمي الجاري للمدرسة', type: 'asset', sub_type: 'bank', is_cash: 0, is_bank: 1, is_control: 0, currency: 'YER' },
    { id: 'ledger-1202-bank-tadhamon', code: '1202', name: 'Tadhamon School Operating Account', name_ar: 'حساب بنك التضامن التشغيلي للمدرسة', type: 'asset', sub_type: 'bank', is_cash: 0, is_bank: 1, is_control: 0, currency: 'YER' },
    { id: 'ledger-1203-bank-usd', code: '1203', name: 'Kuraimi Corporate USD Account', name_ar: 'حساب بنك الكريمي بالدولار الأمريكي', type: 'asset', sub_type: 'bank', is_cash: 0, is_bank: 1, is_control: 0, currency: 'USD' },
    { id: 'ledger-1204-bank-sar', code: '1204', name: 'Tadhamon Corporate SAR Account', name_ar: 'حساب بنك التضامن بالريال السعودي', type: 'asset', sub_type: 'bank', is_cash: 0, is_bank: 1, is_control: 0, currency: 'SAR' },
    { id: 'ledger-1301-ar-tuition', code: '1301', name: 'Student Tuition Receivables', name_ar: 'حساب ذمم الرسوم الدراسية المستحقة للطلاب', type: 'asset', sub_type: 'receivable', is_cash: 0, is_bank: 0, is_control: 1, currency: 'YER' },
    { id: 'ledger-1302-ar-bus', code: '1302', name: 'Student Bus service Receivables', name_ar: 'حساب ذمم النقل المدرسي للطلاب', type: 'asset', sub_type: 'receivable', is_cash: 0, is_bank: 0, is_control: 1, currency: 'YER' },

    // Liabilities
    { id: 'ledger-2101-liab-deferred', code: '2101', name: 'Deferred Tuition Revenue', name_ar: 'إيرادات الرسوم الدراسية المؤجلة / غير المكتسبة', type: 'liability', sub_type: 'deferred_revenue', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-2201-liab-ap', code: '2201', name: 'Accounts Payable (Suppliers)', name_ar: 'حساب ذمم الدائنين والموردين', type: 'liability', sub_type: 'payable', is_cash: 0, is_bank: 0, is_control: 1, currency: 'YER' },
    { id: 'ledger-2301-liab-wages', code: '2301', name: 'Accrued Salaries & Wages', name_ar: 'مصروف الرواتب والأجور المستحقة', type: 'liability', sub_type: 'accrued_expense', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },

    // Equity
    { id: 'ledger-3101-eq-capital', code: '3101', name: 'Shareholder Paid-in Capital', name_ar: 'رأس مال المدرسة المدفوع والتأسيسي', type: 'equity', sub_type: 'capital', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-3201-eq-retained', code: '3201', name: 'Retained Earnings', name_ar: 'حساب الأرباح والخسائر المبقاة والمدورة', type: 'equity', sub_type: 'retained_earnings', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },

    // Revenues
    { id: 'ledger-4101-rev-kg', code: '4101', name: 'Kindergarten Tuition Income', name_ar: 'إيرادات الرسوم الدراسية - رياض الأطفال', type: 'revenue', sub_type: 'income', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-4102-rev-primary', code: '4102', name: 'Primary School Tuition Income', name_ar: 'إيرادات الرسوم الدراسية - المرحلة الابتدائية', type: 'revenue', sub_type: 'income', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-4103-rev-middle', code: '4103', name: 'Middle School Tuition Income', name_ar: 'إيرادات الرسوم الدراسية - المرحلة المتوسطة', type: 'revenue', sub_type: 'income', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-4104-rev-high', code: '4104', name: 'High School Tuition Income', name_ar: 'إيرادات الرسوم الدراسية - المرحلة الثانوية', type: 'revenue', sub_type: 'income', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-4201-rev-bus', code: '4201', name: 'School Bus Services Income', name_ar: 'إيرادات خدمة النقل والمواصلات المدرسية', type: 'revenue', sub_type: 'income', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-4202-rev-uniform', code: '4202', name: 'Books and Uniform Sales Revenue', name_ar: 'إيرادات بيع الكتب والزي المدرسي', type: 'revenue', sub_type: 'income', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-4301-rev-activities', code: '4301', name: 'Activities, Laboratory & IT Fees', name_ar: 'إيرادات الأنشطة الطلابية والمعامل وتقنية المعلومات', type: 'revenue', sub_type: 'income', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },

    // Expenses
    { id: 'ledger-5101-exp-salaries-aca', code: '5101', name: 'Academic Salaries Expense', name_ar: 'مصروف رواتب الكادر التعليمي والتدريس', type: 'expense', sub_type: 'cost_of_sales', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-5102-exp-salaries-adm', code: '5102', name: 'Administrative & Service wages', name_ar: 'مصروف رواتب الإداريين والحراس والصيانة', type: 'expense', sub_type: 'cost_of_sales', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-5201-exp-rent', code: '5201', name: 'Premises Rent Expense', name_ar: 'مصروف إيجار مبنى المدرسة والمنشآت', type: 'expense', sub_type: 'overhead', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-5202-exp-utilities', code: '5202', name: 'Utilities: Electricity & Water', name_ar: 'مصروف كهرباء ومياه مبنى المدرسة', type: 'expense', sub_type: 'overhead', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-5203-exp-erp', code: '5203', name: 'Software Licenses & ERP system', name_ar: 'مصروف تراخيص البرمجيات والأنظمة والإنترنت', type: 'expense', sub_type: 'overhead', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-5301-exp-maintenance', code: '5301', name: 'Facility Maintenance & Supplies', name_ar: 'مصروف صيانة المبنى والتكييف والمستلزمات', type: 'expense', sub_type: 'overhead', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
    { id: 'ledger-5401-exp-marketing', code: '5401', name: 'Marketing, Admission & PR campaigns', name_ar: 'مصروف الحملات الإعلانية والتسويق والتسجيل', type: 'expense', sub_type: 'marketing', is_cash: 0, is_bank: 0, is_control: 0, currency: 'YER' },
  ];

  ledgerAccounts.forEach((acc) => {
    db.prepare(`
      INSERT INTO ledger_accounts (id, org_id, code, name, name_ar, type, sub_type, parent_id, path, depth, currency, opening_balance, current_balance, is_bank_account, is_cash_account, is_control_account, is_active, created_at, updated_at)
      VALUES (?, ?, ?, ?, ?, ?, ?, NULL, ?, 1, ?, 0.0, 0.0, ?, ?, ?, 1, datetime('now'), datetime('now'))
    `).run(acc.id, orgId, acc.code, acc.name, acc.name_ar, acc.type, acc.sub_type, acc.code, acc.currency, acc.is_bank, acc.is_cash, acc.is_control);
  });

  // Treasury Cashboxes & Banks
  const treasuries = [
    { id: 'treasury-1-safe-main', name: 'School Main Safe Cashbox', name_ar: 'الصندوق الرئيسي للمدرسة', code: 'SAFE-MAIN', type: 'cash', ledger: 'ledger-1101-safe-main', currency: 'YER', is_default: 1 },
    { id: 'treasury-2-safe-petty', name: 'Petty Cash safe box', name_ar: 'صندوق العهدة والمصروفات النثرية', code: 'SAFE-PETTY', type: 'cash', ledger: 'ledger-1102-safe-petty', currency: 'YER', is_default: 0 },
    { id: 'treasury-3-bank-kuraimi', name: 'Kuraimi Current Corporate Account', name_ar: 'حساب شركة المدرسة - بنك الكريمي', code: 'BANK-MAIN', type: 'bank', ledger: 'ledger-1201-bank-kuraimi', currency: 'YER', is_default: 0, bank: 'Kuraimi Bank', branch: 'Sanaa Branch', acct: '123456789' },
    { id: 'treasury-4-bank-tadhamon', name: 'Tadhamon School Operating Account', name_ar: 'حساب بنك التضامن التشغيلي للمدرسة', code: 'BANK-OPER', type: 'bank', ledger: 'ledger-1202-bank-tadhamon', currency: 'YER', is_default: 0, bank: 'Tadhamon Bank', branch: 'Sanaa Branch', acct: '987654321' },
    { id: 'treasury-5-bank-usd', name: 'Kuraimi USD Corporate Account', name_ar: 'حساب بنك الكريمي بالدولار الأمريكي', code: 'BANK-USD', type: 'bank', ledger: 'ledger-1203-bank-usd', currency: 'USD', is_default: 0, bank: 'Kuraimi Bank', branch: 'Sanaa Branch', acct: '1122334455' },
    { id: 'treasury-6-bank-sar', name: 'Tadhamon SAR Operating Account', name_ar: 'حساب بنك التضامن بالريال السعودي', code: 'BANK-SAR', type: 'bank', ledger: 'ledger-1204-bank-sar', currency: 'SAR', is_default: 0, bank: 'Tadhamon Bank', branch: 'Sanaa Branch', acct: '5544332211' },
  ];

  treasuries.forEach((t) => {
    db.prepare(`
      INSERT INTO treasuries (id, org_id, unit_id, name, name_ar, code, "type", ledger_account_id, bank_name, bank_branch, account_number, opening_balance, current_balance, currency, authorized_users, is_default, is_active, created_at, updated_at)
      VALUES (?, ?, NULL, ?, ?, ?, ?, ?, ?, ?, ?, 0.0, 0.0, ?, '["admin"]', ?, 1, datetime('now'), datetime('now'))
    `).run(t.id, orgId, t.name, t.name_ar, t.code, t.type, t.ledger, t.bank || null, t.branch || null, t.acct || null, t.currency, t.is_default);
  });

  console.log("🎉 Seeding Sanaa Model School data completely successful!");
} catch (e) {
  console.error("❌ Seeding database failed:", e);
} finally {
  db.close();
}
