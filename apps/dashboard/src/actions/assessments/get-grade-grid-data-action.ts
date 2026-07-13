"use server";

import { getGradeGridData } from "@orbis/db/queries/grades";
import { z } from "zod";
import { authActionClient } from "@/actions/safe-action";

const schema = z.object({
  classId: z.number(),
  subjectId: z.number(),
  termId: z.number(),
  month: z.number().min(1).max(14),
  academicPeriodId: z.number(),
});

export const getGradeGridDataAction = authActionClient
  .schema(schema)
  .metadata({
    name: "get-grade-grid-data",
  })
  .action(
    async ({
      parsedInput: { classId, subjectId, termId, month, academicPeriodId },
    }) => {
      const data = await getGradeGridData({
        classId,
        subjectId,
        termId,
        month,
        academicPeriodId,
      });

      return data;
    },
  );
