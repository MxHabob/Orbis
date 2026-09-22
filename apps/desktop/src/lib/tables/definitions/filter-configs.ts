<script lang="ts">
import { AdjustmentsHorizontal } from "@orbis/icons";
</script>
/**
 * @orbis/desktop — AdjustmentsHorizontal Configurations
 *
 * Defines which fields are filterable for each entity/table.
 * Each config returns FieldDefinition[] that the FilterBuilder uses
 * to render the appropriate field pickers, operators, and value inputs.
 *
 * These configs map directly to the backend column mappers in each
 * service's `build_condition` closure.
 */

import type { FieldDefinition } from "@orbis/ui/filter-builder";

// ═══════════════════════════════════════════════════════════════════════════
// IAM
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// Academic — Enrollments
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// Assessment — Grades
// ═══════════════════════════════════════════════════════════════════════════

export const gradeFilterFields: FieldDefinition[] = [
  { id: "score", label: "Score", type: "number" },
  { id: "enrollment_id", label: "Enrollment ID", type: "string" },
  { id: "subject_name", label: "Subject", type: "string" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Assessment — Attendance
// ═══════════════════════════════════════════════════════════════════════════

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
  { id: "enrollment_id", label: "Enrollment ID", type: "string" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Assessment — Behavior
// ═══════════════════════════════════════════════════════════════════════════

export const behaviorFilterFields: FieldDefinition[] = [
  {
    id: "behavior_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Positive", value: "positive" },
      { label: "Negative", value: "negative" },
      { label: "Neutral", value: "neutral" },
    ],
  },
  {
    id: "severity",
    label: "Severity",
    type: "select",
    options: [
      { label: "Low", value: "low" },
      { label: "Medium", value: "medium" },
      { label: "High", value: "high" },
      { label: "Critical", value: "critical" },
    ],
  },
  { id: "incident_date", label: "Date", type: "date" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Open", value: "open" },
      { label: "Resolved", value: "resolved" },
      { label: "Escalated", value: "escalated" },
    ],
  },
];

// ═══════════════════════════════════════════════════════════════════════════
// Finance — Payments
// ═══════════════════════════════════════════════════════════════════════════

export const paymentFilterFields: FieldDefinition[] = [
  {
    id: "method",
    label: "Payment Method",
    type: "select",
    options: [
      { label: "Cash", value: "cash" },
      { label: "Bank Transfer", value: "bank_transfer" },
      { label: "Card", value: "card" },
      { label: "Check", value: "check" },
    ],
  },
  {
    id: "payment_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Tuition", value: "tuition" },
      { label: "Fee", value: "fee" },
      { label: "Refund", value: "refund" },
    ],
  },
  { id: "paid_at", label: "Paid At", type: "date" },
  { id: "enrollment_id", label: "Enrollment ID", type: "string" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Finance — Student Fees
// ═══════════════════════════════════════════════════════════════════════════

export const studentFeeFilterFields: FieldDefinition[] = [
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Pending", value: "pending" },
      { label: "Paid", value: "paid" },
      { label: "Overdue", value: "overdue" },
      { label: "Waived", value: "waived" },
      { label: "Partial", value: "partial" },
    ],
  },
  { id: "due_date", label: "Due Date", type: "date" },
  { id: "enrollment_id", label: "Enrollment ID", type: "string" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Finance — Journal Entries
// ═══════════════════════════════════════════════════════════════════════════

export const journalFilterFields: FieldDefinition[] = [
  { id: "source_type", label: "Source Type", type: "string" },
  { id: "accounting_date", label: "Accounting Date", type: "date" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Exam — Sessions
// ═══════════════════════════════════════════════════════════════════════════

export const examSessionFilterFields: FieldDefinition[] = [
  { id: "name", label: "Name", type: "string" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Draft", value: "draft" },
      { label: "Published", value: "published" },
      { label: "Active", value: "active" },
      { label: "Completed", value: "completed" },
      { label: "Cancelled", value: "cancelled" },
    ],
  },
  { id: "start_date", label: "Start Date", type: "date" },
  { id: "end_date", label: "End Date", type: "date" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Exam — Halls
// ═══════════════════════════════════════════════════════════════════════════

export const examHallFilterFields: FieldDefinition[] = [
  { id: "capacity", label: "Capacity", type: "number" },
  { id: "building", label: "Building", type: "string" },
  { id: "floor_number", label: "Floor", type: "number" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Exam — Schedules
// ═══════════════════════════════════════════════════════════════════════════

export const examScheduleFilterFields: FieldDefinition[] = [
  { id: "exam_date", label: "Exam Date", type: "date" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Scheduled", value: "scheduled" },
      { label: "Postponed", value: "postponed" },
      { label: "Completed", value: "completed" },
      { label: "Cancelled", value: "cancelled" },
    ],
  },
  { id: "period", label: "Period", type: "string" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Exam — Subjects
// ═══════════════════════════════════════════════════════════════════════════

export const examSubjectFilterFields: FieldDefinition[] = [
  {
    id: "exam_type",
    label: "Exam Type",
    type: "select",
    options: [
      { label: "Written", value: "written" },
      { label: "Oral", value: "oral" },
      { label: "Practical", value: "practical" },
      { label: "Project", value: "project" },
    ],
  },
];

// ═══════════════════════════════════════════════════════════════════════════
// Settings — Rooms & Templates
// ═══════════════════════════════════════════════════════════════════════════

export const roomFilterFields: FieldDefinition[] = [
  {
    id: "room_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Classroom", value: "classroom" },
      { label: "Lab", value: "lab" },
      { label: "Library", value: "library" },
      { label: "Office", value: "office" },
      { label: "Other", value: "other" },
    ],
  },
  { id: "capacity", label: "Capacity", type: "number" },
  { id: "building", label: "Building", type: "string" },
];

export const feeTemplateFilterFields: FieldDefinition[] = [
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
  {
    id: "frequency",
    label: "Frequency",
    type: "select",
    options: [
      { label: "Once", value: "once" },
      { label: "Monthly", value: "monthly" },
      { label: "Termly", value: "termly" },
      { label: "Yearly", value: "yearly" },
    ],
  },
  { id: "default_amount", label: "Amount", type: "number" },
];

export const discountTemplateFilterFields: FieldDefinition[] = [
  {
    id: "discount_type",
    label: "Type",
    type: "select",
    options: [
      { label: "Percentage", value: "percentage" },
      { label: "Fixed", value: "fixed" },
    ],
  },
  { id: "value", label: "Value", type: "number" },
];

export const auditFilterFields: FieldDefinition[] = [
  { id: "action", label: "Action", type: "string" },
  { id: "entity_type", label: "Entity Type", type: "string" },
  { id: "created_at", label: "Date", type: "date" },
];

// ═══════════════════════════════════════════════════════════════════════════
// Finance — Accounts & Fees
// ═══════════════════════════════════════════════════════════════════════════

export const accountFilterFields: FieldDefinition[] = [
  {
    id: "type",
    label: "Type",
    type: "select",
    options: [
      { label: "Asset", value: "asset" },
      { label: "Liability", value: "liability" },
      { label: "Equity", value: "equity" },
      { label: "Revenue", value: "revenue" },
      { label: "Expense", value: "expense" },
    ],
  },
  { id: "currency", label: "Currency", type: "string" },
  { id: "code", label: "Account Code", type: "string" },
];

export const studentFilterFields: FieldDefinition[] = [
  { id: "full_name", label: "Name", type: "string" },
  { id: "student_number", label: "Student Number", type: "string" },
  {
    id: "sex",
    label: "Gender",
    type: "select",
    options: [
      { label: "Male", value: "male" },
      { label: "Female", value: "female" },
    ],
  },
  { id: "enrollment_date", label: "Enrollment Date", type: "date" },
  {
    id: "status",
    label: "Status",
    type: "select",
    options: [
      { label: "Active", value: "active" },
      { label: "Inactive", value: "inactive" },
      { label: "Graduated", value: "graduated" },
      { label: "Transferred", value: "transferred" },
      { label: "Suspended", value: "suspended" },
    ],
  },
];

export const gradesFilterFields: FieldDefinition[] = [
  {
    id: "type",
    label: "Type",
    type: "select",
    options: [
      { label: "Exam", value: "exam" },
      { label: "Assignment", value: "assignment" },
      { label: "Quiz", value: "quiz" },
    ],
  },
];

export const curriculumFilterFields: FieldDefinition[] = [
  {
    id: "type",
    label: "Type",
    type: "select",
    options: [
      { label: "Core", value: "core" },
      { label: "Elective", value: "elective" },
    ],
  },
];
