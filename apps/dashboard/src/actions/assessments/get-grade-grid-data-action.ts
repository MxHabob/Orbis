"use server";

import { authActionClient } from "@/actions/safe-action";
import { getGradeGridData } from "@orbis/db/queries/grades";
import { z } from "zod";

const schema = z.object({
  classId: z.number(),
  subjectId: z.number(),
  termId: z.number(),
  month: z.number().min(1).max(14),
  academicYearId: z.number(),
});

export const getGradeGridDataAction = authActionClient
  .schema(schema)
  .metadata({
    name: "get-grade-grid-data",
  })
  .action(async ({ parsedInput: { classId, subjectId, termId, month, academicYearId } }) => {
    const data = await getGradeGridData({
        classId,
        subjectId,
        termId,
        month,
        academicYearId
    });

    return data;
  });
