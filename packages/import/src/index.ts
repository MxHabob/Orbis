import { z } from "zod";

const fieldConfig = z.object({
  label: z.string(),
  required: z.boolean().optional(),
});

export const mappableFields = {
  students: {
    fullName: { label: "Full Name", required: true },
    email: { label: "Email", required: true },
    nationalId: { label: "National ID", required: false },
    studentNumber: { label: "Student Number", required: false },
  },
  grades: {
    studentEmail: { label: "Student Email", required: true },
    assessment: { label: "Assessment", required: true },
    score: { label: "Score", required: true },
  },
} as const;

export const importSchema = z.object({
  file: z.instanceof(File).optional(),
  type: z.enum(["students", "grades"]).default("students"),
  academicYearId: z.number().optional(),
  classId: z.number().optional(),
  subjectId: z.number().optional(),
  termId: z.number().optional(),
  month: z.number().optional(),
  autoGenerateStudentNumber: z.boolean().optional(),
});

export const studentImportSchema = importSchema.extend({
  type: z.literal("students"),
  fullName: z.string().optional(),
  email: z.string().optional(),
  nationalId: z.string().optional(),
  studentNumber: z.string().optional(),
  metadataMappings: z
    .array(
      z.object({
        key: z.string(),
        column: z.string(),
      }),
    )
    .optional(),
});

export const gradeImportSchema = importSchema.extend({
  type: z.literal("grades"),
  studentEmail: z.string().optional(),
  assessment: z.string().optional(),
  score: z.string().optional(),
});

export type ImportCsvFormData = z.infer<typeof importSchema>;
export type StudentImportFormData = z.infer<typeof studentImportSchema>;
export type GradeImportFormData = z.infer<typeof gradeImportSchema>;

type ParseFileOptions = {
  header?: boolean;
  skipEmptyLines?: boolean;
};

export async function parseFile<T extends Record<string, string>>(
  file: File,
  options: ParseFileOptions = {},
): Promise<{ data: T[]; meta: { fields: string[] } }> {
  const text = await file.text();
  const lines = text
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => (options.skipEmptyLines ? line.length > 0 : true));

  if (lines.length === 0) {
    return { data: [], meta: { fields: [] } };
  }

  const headers = parseCsvLine(lines[0]!);
  const rows = (options.header === false ? lines : lines.slice(1))
    .filter(Boolean)
    .map((line) => {
      const cells = parseCsvLine(line);
      return headers.reduce<Record<string, string>>((row, header, index) => {
        row[header] = cells[index] ?? "";
        return row;
      }, {}) as T;
    });

  return { data: rows, meta: { fields: headers } };
}

function parseCsvLine(line: string): string[] {
  const result: string[] = [];
  let current = "";
  let inQuotes = false;

  for (const char of line) {
    if (char === '"') {
      inQuotes = !inQuotes;
      continue;
    }
    if (char === "," && !inQuotes) {
      result.push(current);
      current = "";
      continue;
    }
    current += char;
  }

  result.push(current);
  return result;
}
