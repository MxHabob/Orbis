import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const root = process.cwd();
const messagesDir = join(root, "apps", "desktop", "messages");
const mobileDictDir = join(root, "apps", "mobile", "src", "lib", "i18n", "dictionaries");

const englishPatterns = [
  { from: /\bTeacher\/ Faculty\b/gi, to: "Academic staff" },
  { from: /\bTeacher\s*\/\s*Faculty\b/gi, to: "Academic staff" },
  { from: /\bAssigned Teacher\b/gi, to: "Assigned staff member" },
  { from: /\bPrimary Teacher\b/gi, to: "Primary staff member" },
  { from: /\bSelect a teacher\b/gi, to: "Select a staff member" },
  { from: /\bSelect teacher\b/gi, to: "Select staff member" },
  { from: /\bTeacher Notes\b/gi, to: "Notes" },
  { from: /\bTeacher Loads\b/gi, to: "Assignments" },
  { from: /\bTeacher Schedule\b/gi, to: "Schedule" },
  { from: /\bTarget Teacher\b/gi, to: "Target staff member" },
  { from: /\bTeacher:\b/gi, to: "Staff member:" },
  { from: /\bTeacher\b/gi, to: "Academic staff member" },
  { from: /\bTeachers\b/gi, to: "Academic staff" },
  { from: /\bteacher\b/gi, to: "staff member" },
  { from: /\bteachers\b/gi, to: "staff members" },
  { from: /\bFaculty\b/gi, to: "Academic staff" },
  { from: /\bProfessor\b/gi, to: "Academic staff member" },
  { from: /\bInstructor\b/gi, to: "Academic staff member" },
  { from: /\bLecturer\b/gi, to: "Academic staff member" },
  { from: /\bDoctor\b/gi, to: "Staff member" },
  { from: /\bDoctors\b/gi, to: "Staff members" },
  { from: /\bdoctor\b/gi, to: "staff member" },
  { from: /\bteachers?\s*available\b/gi, to: "staff members available" },
  { from: /\bNo teachers available\b/gi, to: "No staff members available" },
  { from: /\bNo teacher\b/gi, to: "No staff member" },
  { from: /\bAdd Teacher\b/gi, to: "Add staff member" },
  { from: /\bTeacher & Subject Assignments\b/gi, to: "Staff and subject assignments" },
  { from: /\bTeacher\s*\(Optional\)\b/gi, to: "Staff member (optional)" },
  { from: /\bTeacher\s+role\b/gi, to: "Academic staff role" },
  { from: /\bClassroom instruction, daily attendance, gradebook entry, student conduct logging, and schedules\.?\b/gi, to: "Academic operations, attendance tracking, grade entry, conduct activity logging, and schedule management." },
];

const arabicPatterns = [
  { from: /عضو هيئة التدريسون/gi, to: "الأعضاء الأكاديميون" },
  { from: /عضو هيئة التدريس/gi, to: "عضو أكاديمي" },
  { from: /عضو هيئة تدريس/gi, to: "عضو أكاديمي" },
  { from: /عضو هيئة التدريسين/gi, to: "أعضاء أكاديميين" },
  { from: /المعلمون النشطون/gi, to: "الأعضاء الأكاديميون النشطون" },
  { from: /المعلمون?/gi, to: "الأعضاء الأكاديميون" },
  { from: /المعلمات?/gi, to: "الأعضاء الأكاديميون" },
  { from: /المعلم/gi, to: "عضو أكاديمي" },
  { from: /المعلمين/gi, to: "الأعضاء الأكاديميين" },
  { from: /الدكتور/gi, to: "عضو أكاديمي" },
  { from: /الأستاذ/gi, to: "عضو أكاديمي" },
  { from: /الاستاذ/gi, to: "عضو أكاديمي" },
  { from: /أستاذ/gi, to: "عضو أكاديمي" },
  { from: /المدرس/gi, to: "عضو أكاديمي" },
  { from: /معلم/gi, to: "عضو أكاديمي" },
  { from: /دكتور/gi, to: "عضو أكاديمي" },
  { from: /إسناد المعلمين والمواد/gi, to: "إسناد الأعضاء الأكاديميين والمواد" },
  { from: /المعلمين والمواد/gi, to: "الأعضاء الأكاديميين والمواد" },
  { from: /عضو هيئة تدريس \(اختياري\)/gi, to: "عضو أكاديمي (اختياري)" },
  { from: /اختر عضو هيئة التدريس/gi, to: "اختر عضوًا أكاديميًا" },
  { from: /جميع عضو هيئة التدريسين/gi, to: "جميع الأعضاء الأكاديميين" },
  { from: /لا يوجد عضو هيئة تدريسون/gi, to: "لا يوجد أعضاء أكاديميون" },
  { from: /بدون عضو هيئة تدريس/gi, to: "بدون عضو أكاديمي" },
  { from: /عضو هيئة التدريس المعين/gi, to: "العضو الأكاديمي المعين" },
  { from: /جدول عضو هيئة التدريس/gi, to: "جدول النشاط" },
  { from: /عضو هيئة التدريس المشرف/gi, to: "العضو الأكاديمي المشرف" },
  { from: /العضو الأكاديمي \(اختياري\)/gi, to: "العضو الأكاديمي (اختياري)" },
  { from: /اضافة Teacher/gi, to: "إضافة عضو أكاديمي" },
  { from: /Teacher/gi, to: "عضو أكاديمي" },
];

const mobileTextPatterns = [
  { from: /Teacher tools/gi, to: "Academic tools" },
  { from: /Teacher/i, to: "Academic staff member" },
  { from: /Teachers/gi, to: "Academic staff" },
  { from: /Professor/gi, to: "Academic staff member" },
  { from: /Instructor/gi, to: "Academic staff member" },
  { from: /Lecturer/gi, to: "Academic staff member" },
  { from: /Faculty/gi, to: "Academic staff" },
  { from: /Doctor/gi, to: "Staff member" },
  { from: /معلّم|معلم|مدرس|أستاذ|دكتور|عضو هيئة التدريس/gi, to: "عضو أكاديمي" },
  { from: /المعلمون|المعلمين|أعضاء هيئة التدريس/gi, to: "الأعضاء الأكاديميون" },
  { from: /Teacher Notes/gi, to: "Notes" },
  { from: /Teacher Schedule/gi, to: "Schedule" },
  { from: /Class roster/gi, to: "Section roster" },
  { from: /Class schedule/gi, to: "Section schedule" },
  { from: /My classes/gi, to: "My sections" },
  { from: /class\b/gi, to: "section" },
  { from: /فصل/gi, to: "شعبة" },
  { from: /الفصل/gi, to: "الشعبة" },
  { from: /فصولي/gi, to: "شعبتي" },
  { from: /المعلم|الدكتور|الأستاذ/gi, to: "عضو أكاديمي" },
];

function transformString(value, language) {
  const patterns = language === "ar" ? arabicPatterns : englishPatterns;
  let result = value;
  for (const { from, to } of patterns) {
    result = result.replace(from, to);
  }
  return result;
}

function transformObject(value, language) {
  if (Array.isArray(value)) {
    return value.map((item) => transformObject(item, language));
  }

  if (value && typeof value === "object") {
    const next = {};
    for (const [key, entry] of Object.entries(value)) {
      next[key] = transformObject(entry, language);
    }
    return next;
  }

  if (typeof value === "string") {
    return transformString(value, language);
  }

  return value;
}

async function rewriteLanguageFile(language) {
  const filePath = join(messagesDir, `${language}.json`);
  const fileContent = await readFile(filePath, "utf8");
  const parsed = JSON.parse(fileContent);
  const transformed = transformObject(parsed, language);
  await writeFile(filePath, `${JSON.stringify(transformed, null, 2)}\n`, "utf8");
  console.log(`Updated ${language}.json`);
}

async function rewriteMobileDictionary(language) {
  const filePath = join(mobileDictDir, `${language}.ts`);
  const fileContent = await readFile(filePath, "utf8");
  const transformed = fileContent.replace(/:\s*"((?:[^"\\]|\\.)*)"/g, (_, value) => {
    const next = value.replace(/\\["\\nrtbf]/g, (match) => {
      if (match === '\\n') return "\n";
      if (match === '\\r') return "\r";
      if (match === '\\t') return "\t";
      if (match === '\\b') return "\b";
      if (match === '\\f') return "\f";
      return match;
    });
    const normalized = mobileTextPatterns.reduce(
      (acc, { from, to }) => acc.replace(from, to),
      next,
    );
    return `: "${normalized.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"`;
  });
  await writeFile(filePath, transformed, "utf8");
  console.log(`Updated mobile ${language}.ts`);
}

await rewriteLanguageFile("en");
await rewriteLanguageFile("ar");
await rewriteMobileDictionary("en");
await rewriteMobileDictionary("ar");
