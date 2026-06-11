"use server";

import { upsertGrade } from "@orbis/db/queries/grades";
import { revalidatePath } from "next/cache";
import { z } from "zod";
import { authActionClient } from "@/actions/safe-action";

const schema = z.object({
  studentId: z.string().uuid(),
  assessmentItemId: z.number(),
  score: z.number().nullable(), // Null to clear
});

export const upsertGradeAction = authActionClient
  .schema(schema)
  .metadata({
    name: "upsert-grade",
  })
  .action(
    async ({ parsedInput: { studentId, assessmentItemId, score }, ctx }) => {
      // Convert number to string for numeric column
      const scoreString = score === null ? null : score.toString();

      const result = await upsertGrade({
        studentId,
        assessmentItemId,
        score: scoreString,
        gradedById: ctx.user.id,
      });

      revalidatePath("/grades");

      return result;
    },
  );
