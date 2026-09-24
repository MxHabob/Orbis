/**
 * Centralized Filter Configurations for Orbis Desktop
 * Defines filterable fields for each domain entity for FilterTrigger/FilterBuilder.
 */
import type { FieldDefinition } from "@orbis/ui/filter-builder";

// ── IAM / Users ──────────────────────────────────────────────────────────
export const userFilterFields: FieldDefinition[] = [
  { id: "full_name", label: "Name", type: "string" },
  { id: "email", label: "Email", type: "string" },
  { id: "phone", label: "Phone", type: "string" },
  {
    id: "role",
    label: "Role",
    type: "select",
    options: [
      { label: "Admin", value: "admin" },
      { label: "Teacher", value: "teacher" },
      { label: "Student", value: "student" },
      { label: "Parent", value: "parent" },
      { label: "Staff", value: "staff" },
    ],
  },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
      { label: "Suspended", value: "suspended" },
    ],
  },
  { id: "created_at", label: "Created At", type: "date" },
];

// ── Students / Enrollments ────────────────────────────────────────────────
export const studentFilterFields: FieldDefinition[] = [
  { id: "full_name", label: "Name", type: "string" },
  { id: "student_number", label: "Student Number", type: "string" },
  { id: "national_id", label: "National ID", type: "string" },
  { id: "class_name", label: "Class", type: "string" },
  {
    id: "sex",
    label: "Gender",
    type: "select",
    options: [
      { label: "Male", value: "male" },
      { label: "Female", value: "female" },
    ],
  },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
      { label: "Graduated", value: "graduated" },
      { label: "Withdrawn", value: "withdrawn" },
    ],
  },
  { id: "enrollment_date", label: "Enrollment Date", type: "date" },
];

export const enrollmentFilterFields: FieldDefinition[] = [
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Graduated", value: "graduated" },
      { label: "Withdrawn", value: "withdrawn" },
      { label: "Transferred", value: "transferred" },
      { label: "Suspended", value: "suspended" },
    ],
  },
  { id: "enrollment_date", label: "Enrollment Date", type: "date" },
  { id: "level", label: "Level", type: "string" },
];

// ── Grades / Assessment ──────────────────────────────────────────────────
export const gradesFilterFields: FieldDefinition[] = [
  { id: "subject_name", label: "Subject", type: "string" },
  { id: "score", label: "Score", type: "number" },
  { id: "letter_grade", label: "Grade", type: "string" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Pass", value: "pass" },
      { label: "Fail", value: "fail" },
    ],
  },
];

export const gradeFilterFields: FieldDefinition[] = [
  { id: "score", label: "Score", type: "number" },
  { id: "enrollment_id", label: "Enrollment ID", type: "string" },
  { id: "subject_name", label: "Subject", type: "string" },
];

// ── Attendance ───────────────────────────────────────────────────────────
export const attendanceFilterFields: FieldDefinition[] = [
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Present", value: "present" },
      { label: "Absent", value: "absent" },
      { label: "Late", value: "late" },
      { label: "Excused", value: "excused" },
    ],
  },
  { id: "date", label: "Date", type: "date" },
  { id: "session_name", label: "Session", type: "string" },
];

// ── Behavior ─────────────────────────────────────────────────────────────
export const behaviorFilterFields: FieldDefinition[] = [
  {
    id: "record_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Positive", value: "Positive" },
      { label: "Negative", value: "Negative" },
      { label: "Neutral", value: "Neutral" },
    ],
  },
  { id: "points", label: "Points", type: "number" },
  { id: "date", label: "Date", type: "date" },
];

// ── Finance / Accounts ───────────────────────────────────────────────────
export const accountFilterFields: FieldDefinition[] = [
  { id: "code", label: "Account Code", type: "string" },
  { id: "name", label: "Account Name", type: "string" },
  {
    id: "item_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Asset", value: "Asset" },
      { label: "Liability", value: "Liability" },
      { label: "Equity", value: "Equity" },
      { label: "Revenue", value: "Revenue" },
      { label: "Expense", value: "Expense" },
    ],
  },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
    ],
  },
];

export const studentFeeFilterFields: FieldDefinition[] = [
  { id: "student_name", label: "Student", type: "string" },
  { id: "description", label: "Description", type: "string" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Paid", value: "paid" },
      { label: "Partial", value: "partial" },
      { label: "Unpaid", value: "unpaid" },
      { label: "Overdue", value: "overdue" },
    ],
  },
  { id: "due_date", label: "Due Date", type: "date" },
  { id: "net_amount", label: "Amount", type: "number" },
];

export const feeTemplateFilterFields: FieldDefinition[] = [
  { id: "name", label: "Template Name", type: "string" },
  {
    id: "category",
    label: "Category",
    type: "select",
    options: [
      { label: "Tuition", value: "tuition" },
      { label: "Transport", value: "transport" },
      { label: "Books", value: "books" },
      { label: "Uniform", value: "uniform" },
      { label: "Exam", value: "exam" },
      { label: "Other", value: "other" },
    ],
  },
  { id: "amount", label: "Amount", type: "number" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
    ],
  },
];

export const discountTemplateFilterFields: FieldDefinition[] = [
  { id: "name", label: "Template Name", type: "string" },
  {
    id: "discount_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Sibling", value: "sibling" },
      { label: "Academic Excellence", value: "academic_excellence" },
      { label: "Need-based", value: "need_based" },
      { label: "Staff Child", value: "staff_child" },
      { label: "Custom", value: "custom" },
    ],
  },
  {
    id: "calculation_method",
    label: "Method",
    type: "select",
    options: [
      { label: "Percentage", value: "percentage" },
      { label: "Fixed Amount", value: "fixed" },
    ],
  },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
    ],
  },
];

// ── Rooms / Facilities ───────────────────────────────────────────────────
export const roomFilterFields: FieldDefinition[] = [
  { id: "name", label: "Name", type: "string" },
  { id: "building", label: "Building", type: "string" },
  { id: "floor", label: "Floor", type: "string" },
  { id: "capacity", label: "Capacity", type: "number" },
  {
    id: "room_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Classroom", value: "classroom" },
      { label: "Lab", value: "lab" },
      { label: "Hall", value: "hall" },
      { label: "Office", value: "office" },
    ],
  },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
    ],
  },
];

// ── Audit Logs ───────────────────────────────────────────────────────────
export const auditFilterFields: FieldDefinition[] = [
  { id: "action", label: "Action", type: "string" },
  { id: "entity_name", label: "Entity", type: "string" },
  { id: "user_name", label: "User", type: "string" },
  { id: "created_at", label: "Timestamp", type: "date" },
];

// ── Subjects ─────────────────────────────────────────────────────────────
export const subjectFilterFields: FieldDefinition[] = [
  { id: "name", label: "Subject Name", type: "string" },
  { id: "code", label: "Code", type: "string" },
  { id: "credit_hours", label: "Credit Hours", type: "number" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
    ],
  },
];

// ── Currencies ───────────────────────────────────────────────────────────
export const currencyFilterFields: FieldDefinition[] = [
  { id: "code", label: "Currency Code", type: "string" },
  { id: "name", label: "Currency Name", type: "string" },
  { id: "symbol", label: "Symbol", type: "string" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
    ],
  },
];

