import { db } from "../src/client";
import { assessmentItems, academicTerms } from "../src/schema";
import { eq, and, sql, inArray } from "drizzle-orm";

async function migrate() {
  console.log("🚀 Starting Terms Migration...");

  // 1. Get all terms to identify which is first and which is second
  const allTerms = await db.select().from(academicTerms);
  
  // Group by year to ensure we handle multiple academic years if they exist
  const termsByYear: Record<number, any[]> = {};
  allTerms.forEach(term => {
    if (term.academicYearId) {
      if (!termsByYear[term.academicYearId]) termsByYear[term.academicYearId] = [];
      termsByYear[term.academicYearId].push(term);
    }
  });

  for (const yearIdStr in termsByYear) {
    const yearId = parseInt(yearIdStr);
    const yearTerms = termsByYear[yearId].sort((a, b) => (a.startDate || "").localeCompare(b.startDate || ""));
    const term1 = yearTerms[0];
    const term2 = yearTerms[1];

    if (!term1 || !term2) {
      console.warn(`⚠️ Year ${yearId} does not have both terms correctly defined. Skipping.`);
      continue;
    }

    console.log(`\n📅 Processing Academic Year ID: ${yearId}`);
    console.log(`   Term 1: ${term1.name} (ID: ${term1.id})`);
    console.log(`   Term 2: ${term2.name} (ID: ${term2.id})`);

    // Update Term 1 items (Months 1, 2, 3, 4)
    await db.update(assessmentItems)
      .set({ termId: term1.id })
      .where(inArray(assessmentItems.month, [1, 2, 3, 4]));
      
    // Update Term 2 items (Months 5, 6, 7, 8)
    await db.update(assessmentItems)
      .set({ termId: term2.id })
      .where(inArray(assessmentItems.month, [5, 6, 7, 8]));

    console.log(`   ✅ Terms updated for Year ${yearId}`);
  }

  console.log("\n✨ Migration finished successfully.");
  process.exit(0);
}

migrate().catch(err => {
  console.error("❌ Migration failed:", err);
  process.exit(1);
});
