export type ElementType = "text" | "variable" | "image" | "table" | "shape";

export interface TableColumn {
  id: string;
  header: string;
  width: number;
}

export interface EditorElement {
  id: string;
  type: ElementType;
  content: string;
  variableKey?: string;
  x: number; // percentage
  y: number; // percentage
  fontSize?: number;
  fontWeight?: string;
  color?: string;
  align?: "left" | "center" | "right";
  width?: number; // percentage of canvas
  height?: number; // percentage of canvas
  opacity?: number;
  rotation?: number;
  fontFamily?: string;
  borderColor?: string;
  borderWidth?: number;
  backgroundColor?: string;
  columns?: TableColumn[]; // For table type
}

export const PAPER_SIZES = {
  a4: { width: 595, height: 842 },
  letter: { width: 612, height: 792 },
};

export const VARIABLES = [
  { group: "Student Data", items: [
    { label: "Student Name", value: "{{student_name}}" },
    { label: "Academic Number", value: "{{academic_number}}" },
    { label: "National ID", value: "{{national_id}}" },
    { label: "Department / Specialization", value: "{{department}}" },
    { label: "Section", value: "{{section}}" },
  ]},
  { group: "Academic Results", items: [
    { label: "Grades Table", value: "table", isTable: true },
    { label: "Grade / Level", value: "{{class_name}}" },
    { label: "Total Grade (Numeric)", value: "{{total_grade}}" },
    { label: "Total Grade (Words)", value: "{{total_grade_text}}" },
    { label: "General Grade", value: "{{grade_label}}" },
    { label: "Final Result", value: "{{final_result}}" },
    { label: "Rank", value: "{{rank}}" },
  ]},
  { group: "Institution Data", items: [
    { label: "Organization Name", value: "{{org_name}}" },
    { label: "School Administration", value: "{{admin_name}}" },
    { label: "Vice Principal", value: "{{vp_name}}" },
    { label: "Report Date", value: "{{issue_date}}" },
  ]}
];

export const FONTS = [
  { label: "Default Font", value: "inherit" },
  { label: "Kufi Font", value: "'Kufam', sans-serif" },
  { label: "Amiri Font (Official)", value: "'Amiri', serif" },
  { label: "Cairo Font", value: "'Cairo', sans-serif" },
];
