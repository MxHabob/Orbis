export interface SchemaField {
  key: string;
  label: string; // for English header matching
  labelAr?: string; // for Arabic header matching
  required: boolean;
  type: "string" | "number" | "email";
  validate?: (value: any) => boolean;
}

export interface ImportSchema {
  fields: SchemaField[];
}

export const STUDENT_IMPORT_SCHEMA: ImportSchema = {
  fields: [
    {
      key: "full_name",
      label: "Full Name",
      labelAr: "الاسم الكامل",
      required: true,
      type: "string",
    },
    {
      key: "email",
      label: "Email",
      labelAr: "البريد الإلكتروني",
      required: true,
      type: "email",
    },
    {
      key: "national_id",
      label: "National ID",
      labelAr: "الرقم الوطني",
      required: false,
      type: "string",
    },
    {
      key: "class_name",
      label: "Class Name",
      labelAr: "اسم الصف",
      required: true,
      type: "string",
    },
  ],
};

export const GRADES_IMPORT_SCHEMA: ImportSchema = {
  fields: [
    {
      key: "student_email",
      label: "Student Email",
      labelAr: "بريد الطالب",
      required: true,
      type: "email",
    },
    {
      key: "subject",
      label: "Subject",
      labelAr: "المادة",
      required: true,
      type: "string",
    },
    {
      key: "grade",
      label: "Grade",
      labelAr: "الدرجة",
      required: true,
      type: "number",
    },
    {
      key: "term",
      label: "Term",
      labelAr: "الفصل الدراسي",
      required: true,
      type: "string",
    },
  ],
};

export const SCHEMAS: Record<string, ImportSchema> = {
  student: STUDENT_IMPORT_SCHEMA,
  grades: GRADES_IMPORT_SCHEMA,
};
