/** Assessment domain types. */

export interface Grade {
  id: string;
  org_id: string;
  period_id: string;
  enrollment_id: string;
  assessment_id?: string | null;
  class_subject_id?: string | null;
  subject_name: string;
  subject_name_ar?: string | null;
  score: number;
  max_score: number;
  weight: number;
  grade_letter?: string | null;
  grade_points?: number | null;
  exam_type: string;
  semester: string;
  notes?: string | null;
  graded_by: string;
  graded_at: string;
  is_final: number;
  created_at: string;
  updated_at: string;
}

export interface Attendance {
  id: string;
  org_id: string;
  period_id: string;
  enrollment_id: string;
  class_id: string;
  date: string;
  status: string;
  check_in_time?: string | null;
  check_out_time?: string | null;
  period?: string | null;
  subject_name?: string | null;
  recorded_by: string;
  notes?: string | null;
  excuse_type?: string | null;
  is_excused: number;
  created_at: string;
}

export interface BehaviorRecord {
  id: string;
  org_id: string;
  period_id: string;
  enrollment_id: string;
  behavior_type: string;
  severity: string;
  title: string;
  description: string;
  incident_date: string;
  status: string;
  points?: number | null;
  action_taken?: string | null;
  reported_by: string;
  ai_confidence?: number | null;
  created_at: string;
  updated_at: string;
}

export interface Assessment {
  id: string;
  org_id: string;
  cycle_id: string;
  class_subject_id: string;
  title: string;
  title_ar?: string;
  description?: string;
  type: string;
  max_score: number;
  passing_score?: number;
  weight: number;
  grade_group_name: string;
  assessment_date?: string;
  due_date?: string;
  duration_minutes?: number;
  is_published: boolean;
  published_at?: string;
  published_by?: string;
  is_locked: boolean;
  locked_at?: string;
  locked_by?: string;
  grading_method?: string;
  rubric_id?: string;
  instructions?: string;
  instructions_ar?: string;
  attachments?: Record<string, any>;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface GradeReportCard {
  enrollment_id: string;
  student_id: string;
  student_name: string;
  period_name: string;
  subjects: SubjectGradeSummary[];
  overall_percentage: number;
  overall_gpa: number;
  overall_letter: string;
  total_subjects: number;
  passed_subjects: number;
  failed_subjects: number;
}

export interface SubjectGradeSummary {
  subject_name: string;
  subject_name_ar?: string | null;
  final_score: number;
  max_score: number;
  percentage: number;
  grade_letter: string;
  grade_points: number;
  passed: boolean;
}

export interface AttendanceReport {
  enrollment_id: string;
  student_id: string;
  student_name: string;
  total_days: number;
  present_days: number;
  absent_days: number;
  late_days: number;
  excused_days: number;
  attendance_rate: number;
  is_at_risk: boolean;
}

export interface GradeAnalytics {
  class_average: number;
  class_median: number;
  class_min: number;
  class_max: number;
  pass_count: number;
  fail_count: number;
  pass_rate: number;
}
export interface AssessmentScheme {
  id: string;
  name: string;
}

export interface Snapshot {
  id: string;
}

export interface AssessmentOverride {
  id: string;
  status: string;
}
