"use server";

import { authActionClient } from "@/actions/safe-action";
import { z } from "zod";
import { 
  getMonthlyAssessmentItems, 
  initializeStandardMonthlyAssessments 
} from "@midday/db/queries/assessments";
import { revalidatePath } from "next/cache";

const schema = z.object({
  classId: z.number(),
  subjectId: z.number(),
  termId: z.number(),
  month: z.number().min(1).max(12),
});

export const initializeMonthlyAssessmentsAction = authActionClient
  .schema(schema)
  .metadata({
    name: "initialize-monthly-assessments",
  })
  .action(async ({ parsedInput: { classId, subjectId, termId, month } }) => {
    // Check if items already exist for this combination using the new query
    const existingItems = await getMonthlyAssessmentItems({
      classId,
      subjectId,
      termId,
      month,
    });

    if (existingItems.length > 0) {
      return { created: false, items: existingItems };
    }

    // Initialize standard items using the new query
    const insertedItems = await initializeStandardMonthlyAssessments({
      classId,
      subjectId,
      termId,
      month,
    });

    revalidatePath("/grades");

    return { created: true, items: insertedItems };
  });
