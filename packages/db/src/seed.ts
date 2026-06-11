import { eq, sql, getTableName } from "drizzle-orm";
import { randomUUID } from "crypto";
import * as schema from "./schema";
import { db } from "./client";

// ────────────────────────────────────────────────
// Config
// ────────────────────────────────────────────────
const isProduction = process.env.NODE_ENV === "production";

if (isProduction && !process.env.BYPASS_SEED_PROD_CHECK) {
  console.error("❌ Seeding is prohibited in production environment");
  process.exit(1);
}

// ────────────────────────────────────────────────
// Handle Arguments
// ────────────────────────────────────────────────
const args = process.argv.slice(2);
const yearArg = args.find(arg => arg.startsWith('--'));

// Default year if not provided
const academicYearInput = yearArg ? yearArg.slice(2) : "2025-2026";

console.log(`\n🚀 Seeding database for academic year: ${academicYearInput}`);

// ────────────────────────────────────────────────
// Seed Academic Years & Terms
// ────────────────────────────────────────────────
async function seedAcademicStructure(yearName: string): Promise<{ yearId: number }> {
  console.log("→ Seeding academic year & terms...");

  const startYearMatch = yearName.match(/\d{4}/);
  const startYear = startYearMatch ? parseInt(startYearMatch[0]) : new Date().getFullYear();
  const endYear = startYear + 1;

  // Insert Year
  await db.insert(schema.academicYears)
    .values({ 
      yearName, 
      isCurrent: true,
      startDate: `${startYear}-08-20`,
      endDate: `${endYear}-06-20`
    })
    .onConflictDoUpdate({
      target: schema.academicYears.yearName,
      set: { isCurrent: true }
    });

  const [currentYear] = await db
    .select({ id: schema.academicYears.id })
    .from(schema.academicYears)
    .where(eq(schema.academicYears.yearName, yearName))
    .limit(1);

  if (!currentYear) throw new Error("Academic year could not be created/found");

  // Create Terms
  const terms = [
    {
      name: "الترم الاول",
      academicYearId: currentYear.id,
      startDate: `${startYear}-09-01`,
      endDate: `${startYear}-12-30`,
      isCurrent: true,
    },
    {
      name: "الترم الثاني",
      academicYearId: currentYear.id,
      startDate: `${endYear}-02-01`,
      endDate: `${endYear}-06-20`,
      isCurrent: false,
    },
  ];

  for (const term of terms) {
    await db.insert(schema.academicTerms)
      .values(term)
      .onConflictDoNothing();
  }

  console.log("✓ Academic structure completed");
  return { yearId: currentYear.id };
}

// ────────────────────────────────────────────────
// Seed Subjects
// ────────────────────────────────────────────────
async function seedSubjects() {
  console.log("→ Seeding subjects...");
  const subjectsData = [
    { name: "القرآن الكريم",       code: "QUR", color: "#FFD700" }, // Gold
    { name: "التربية الاسلامية",    code: "ISL", color: "#2E7D32" }, // Green
    { name: "اللغه العربية",       code: "ARA", color: "#795548" }, // Brown
    { name: "اللغة الانجليزية",     code: "ENG", color: "#1976D2" }, // Blue
    { name: "العلوم",              code: "SCI", color: "#7B1FA2" }, // Purple
    { name: "الرياضيات",           code: "MAT", color: "#E64A19" }, // Deep Orange
    { name: "الاجتماعيات",         code: "SST", color: "#0097A7" }, // Teal
  ];

  await db.insert(schema.subjects).values(subjectsData).onConflictDoNothing();
  console.log("✓ Subjects completed");
}

// ────────────────────────────────────────────────
// Seed Classes (Grades only)
// ────────────────────────────────────────────────
async function seedClasses(yearId: number) {
  console.log("→ Seeding grades (1 to 9)...");
  const grades = [
    "الصف الأول", "الصف الثاني", "الصف الثالث", 
    "الصف الرابع", "الصف الخامس", "الصف السادس",
    "الصف السابع", "الصف الثامن", "الصف التاسع"
  ];

  // Professional gradient-like palette (HSL based for attractiveness)
  const colors = [
    "#f44336", "#e91e63", "#9c27b0", 
    "#673ab7", "#3f51b5", "#2196f3", 
    "#00bcd4", "#009688", "#4caf50"
  ];

  const classesData = grades.map((name, index) => ({
    name: name,
    type: "classroom" as const,
    division: null,
    parentClassId: null,
    academicYearId: yearId,
    color: colors[index] || "#e6e6e6",
    sortOrder: index + 1,
  }));

  await db.insert(schema.classes).values(classesData).onConflictDoNothing();
  console.log("✓ Grades completed");
}

// ────────────────────────────────────────────────
// Main Seeding Flow
// ────────────────────────────────────────────────
export async function seedDatabase(academicYear = "2025-2026") {
  // 1. Academic Structure
  const { yearId } = await seedAcademicStructure(academicYear);

  // 2. Subjects
  await seedSubjects();

  // 3. Classes (Grades)
  await seedClasses(yearId);
}

async function main() {
  try {
    await seedDatabase(academicYearInput);

    console.log("\n" + "=".repeat(60));
    console.log("           Seeding completed successfully ✓");
    console.log("=".repeat(60) + "\n");

  } catch (err) {
    console.error("❌ Seeding failed:", err);
    process.exit(1);
  }
}

if (import.meta.main) {
  main();
}

