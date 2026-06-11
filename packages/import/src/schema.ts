import { z } from "zod/v3";

export const mappableFields = {
  students: {
    studentNumber: { label: "Student Number", required: true },
    name: { label: "Full Name", required: true },
    phone: { label: "Phone", required: false },
    email: { label: "Email", required: false },
    sex: { label: "Sex (Male/Female)", required: false },
    fatherName: { label: "Father Name", required: false },
    fatherPhone: { label: "Father Phone", required: false },
    motherName: { label: "Mother Name", required: false },
    motherPhone: { label: "Mother Phone", required: false },
  },
  grades: {
    studentNumber: { label: "Student Number", required: true },
    score: { label: "Score", required: true },
    isAbsent: { label: "Is Absent", required: false },
    comments: { label: "Comments", required: false },
  },
} as const;

/**
 * Common fields shared by all import types
 */
const baseImportSchema = z.object({
  file: z.custom<File>(),
  table: z.array(z.record(z.string(), z.string())).optional(),
  mappings: z.record(z.string(), z.string()).optional(),
  classId: z.number({ required_error: "Class is required" }).optional(),
  academicYearId: z.number().optional(),
});

/**
 * Schema for Importing Students
 */
export const studentImportSchema = baseImportSchema
  .extend({
    type: z.literal("students").default("students"),
    studentNumber: z.string().optional(),
    autoGenerateStudentNumber: z.boolean().default(false),
    name: z.string({ required_error: "Name mapping is required" }).optional(),
    phone: z.string().optional(),
    email: z.string().optional(),
    sex: z.string().optional(),
    fatherName: z.string().optional(),
    fatherPhone: z.string().optional(),
    motherName: z.string().optional(),
    motherPhone: z.string().optional(),
    metadataMappings: z
      .array(
        z.object({
          key: z.string().min(1, "Key is required"),
          column: z.string().min(1, "Column is required"),
        }),
      )
      .optional()
      .default([])
      .refine(
        (mappings) => {
          const keys = mappings.map((m) => m.key.toLowerCase().trim());
          return new Set(keys).size === keys.length;
        },
        {
          message: "Metadata keys must be unique",
        },
      ),
  })
  .refine(
    (data) => {
      if (!data.autoGenerateStudentNumber && !data.studentNumber) return false;
      return !!data.name;
    },
    (data) => {
      if (!data.autoGenerateStudentNumber && !data.studentNumber) {
        return {
          message: "Student Number mapping is required",
          path: ["studentNumber"],
        };
      }
      return { message: "Name mapping is required", path: ["name"] };
    },
  );

export const gradeImportSchema = baseImportSchema.extend({
  type: z.literal("grades").default("grades"),
  subjectId: z.number({ required_error: "Subject is required" }).optional(),
  termId: z.number().optional(),
  assessmentItemId: z.number().optional(),
  month: z.number({ required_error: "Month is required" }).optional(),
  // Identifier field
  identifierType: z
    .enum(["studentNumber", "secretNumber"])
    .default("studentNumber"),
  studentNumber: z.string({ required_error: "Identifier mapping is required" }),
  score: z.string().optional(),
  isAbsent: z.string().optional(),
  comments: z.string().optional(),
});

/**
 * Discriminated union for type-safe handling of all import types
 */
export const importSchema = z.union([studentImportSchema, gradeImportSchema]);

export type StudentImportFormData = z.infer<typeof studentImportSchema>;
export type GradeImportFormData = z.infer<typeof gradeImportSchema>;
export type ImportCsvFormData = StudentImportFormData | GradeImportFormData;
