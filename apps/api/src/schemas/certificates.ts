import { z } from "zod";

export const certificateTypeSchema = z.enum([
  "monthly",
  "mid_term",
  "year_end",
]);

export const getCertificateDataSchema = z
  .object({
    studentId: z.string().uuid(),
    classId: z.number(),
    academicYearId: z.number(),
    termId: z.number(),
    certificateType: certificateTypeSchema,
    month: z.number().min(1).max(12).optional(),
  })
  .refine(
    (data) => {
      if (data.certificateType === "monthly") return data.month != null;
      return true;
    },
    {
      message: "month is required when certificateType is monthly",
      path: ["month"],
    },
  );
