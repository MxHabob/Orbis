"use server";

import { authActionClient } from "@/actions/safe-action";
import { z } from "zod";
import { 
  getMonthlyAssessmentItems, 
  initializeStandardMonthlyAssessments,
  initializeTermExamAssessments
} from "@midday/db/queries/assessments";
import { revalidatePath } from "next/cache";

const schema = z.object({
  classId: z.number(),
  subjectId: z.number(),
  termId: z.number(),
  month: z.number().min(1).max(14),
});

export const initializeAssessmentsAction = authActionClient
  .schema(schema)
  .metadata({
    name: "initialize-assessments",
  })
  .action(async ({ parsedInput: { classId, subjectId, termId, month } }) => {
    console.log("[Action] initializeAssessmentsAction called with:", { classId, subjectId, termId, month });
    try {
      // Check if items already exist for this combination using the new query
      const existingItems = await getMonthlyAssessmentItems({
        classId,
        subjectId,
        termId,
        month,
      });

      if (existingItems.length > 0) {
        return { 
          created: false, 
          count: existingItems.length 
        };
      }

      // Initialize items based on month type
      const isTermExam = month === 4 || month === 8;
      const insertedItems = isTermExam 
        ? await initializeTermExamAssessments({ classId, subjectId, termId, month })
        : await initializeStandardMonthlyAssessments({ classId, subjectId, termId, month });

      // Use a more standard revalidation approach
      revalidatePath("/grades", "page");
      revalidatePath("/(sidebar)/grades", "layout");

      return { 
        created: true, 
        count: insertedItems.length 
      };
    } catch (error) {
      console.error("Error in initializeAssessmentsAction:", error);
      throw new Error(error instanceof Error ? error.message : "Failed to initialize assessments");
    }
  });
