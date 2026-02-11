import { z } from "zod/v3";

export const mappableFields = {
  students: {
    studentNumber: { label: "Student Number", required: true },
    name: { label: "Full Name", required: true },
    phone: { label: "Phone", required: false },
    email: { label: "Email", required: false },
    sex: { label: "Sex (Male/Female)", required: false },
  },
  grades: {
    studentNumber: { label: "Student Number", required: true },
    score: { label: "Score", required: true },
    isAbsent: { label: "Is Absent", required: false },
    comments: { label: "Comments", required: false },
  },
} as const;

export const importSchema = z.object({
  file: z.custom<File>(),
  type: z.enum(["students", "grades"]).default("students"),
  // Student fields
  studentNumber: z.string().optional(),
  autoGenerateStudentNumber: z.boolean().default(false),
  name: z.string().optional(),
  phone: z.string().optional(),
  email: z.string().optional(),
  sex: z.string().optional(),
  // Grade fields
  score: z.string().optional(),
  isAbsent: z.string().optional(),
  comments: z.string().optional(),
  // School context
  classId: z.number().optional(),
  academicYearId: z.number().optional(),
  subjectId: z.number().optional(),
  termId: z.number().optional(),
  assessmentItemId: z.number().optional(),
  month: z.number().optional(),
  
  table: z.array(z.record(z.string(), z.string())).optional(),
  mappings: z.record(z.string(), z.string()).optional(),
}).refine((data) => {
    if (data.type === "students") {
      if (!data.classId) return false;
      if (!data.autoGenerateStudentNumber && !data.studentNumber) return false;
      if (!data.name) return false;
    }
    
    if (data.type === "grades") {
      if (!data.classId) return false;
      if (!data.subjectId) return false;
      if (!data.month) return false;
      if (!data.studentNumber) return false;
    }
    
    return true;
}, {
  message: "Required fields are missing",
  path: ["classId"]
});

export type ImportCsvFormData = z.infer<typeof importSchema>;
