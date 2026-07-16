/**
 * Academic domain types — mirrors Rust entities in svc-academic + svc-organization.
 *
 * Migrated: AcademicPeriod → AcademicPeriod, period_id → period_id/period_id.
 */

/** Academic Period (replaces AcademicPeriod) */
export interface AcademicPeriod {
  id: string;
  org_id: string;
  period_id: string;
  parent_id?: string | null;
  name: string;
  name_ar?: string | null;
  code?: string | null;
  period_type: string;
  start_date: string;
  end_date: string;
  hijri_start?: string | null;
  hijri_end?: string | null;
  enrollment_start?: string | null;
  enrollment_end?: string | null;
  is_enrollment_open: number;
  grade_entry_start?: string | null;
  grade_entry_end?: string | null;
  is_grade_entry_open: number;
  grade_entry_status?: string | null;
  exam_start?: string | null;
  exam_end?: string | null;
  grading_system: string;
  passing_grade?: number | null;
  max_grade?: number | null;
  weight?: number | null;
  status: string;
  is_locked: number;
  locked_at?: string | null;
  locked_by?: string | null;
  sort_order?: number | null;
  created_at: string;
  updated_at: string;
}

export interface Class {
  id: string;
  org_id: string;
  unit_id: string;
  period_id: string;
  name: string;
  name_ar?: string | null;
  level: string;
  division?: string | null;
  homeroom_teacher_id?: string | null;
  capacity?: number | null;
  current_students?: number | null;
  room_id?: string | null;
  color?: string | null;
  is_active: number;
  is_deleted: number;
  created_at: string;
  updated_at: string;
}

export interface Enrollment {
  id: string;
  org_id: string;
  period_id: string;
  student_id: string;
  class_id: string;
  student_number: string;
  status: string;
  enrollment_date: string;
  enrollment_type: string;
  withdrawal_date?: string | null;
  withdrawal_reason?: string | null;
  final_grade?: string | null;
  final_score?: number | null;
  final_rank?: number | null;
  is_promoted?: number | null;
  promotion_decision?: string | null;
  promotion_notes?: string | null;
  meta?: string | null;
  created_at: string;
  updated_at: string;
  name_ar?: string | null;
  name_en?: string | null;
  full_name?: string | null;
  avatar?: string | null;
  birth_date?: string | null;
  sex?: string | null;
  email?: string | null;
  phone_number?: string | null;
  phone?: string | null;
  address?: string | null;
  guardian_name?: string | null;
  guardian_phone?: string | null;
}

export interface Subject {
  id: string;
  org_id: string;
  unit_id?: string;
  code: string;
  name: string;
  name_ar?: string;
  name_en?: string;
  description?: string;
  short_description?: string;
  is_core: boolean;
  is_elective: boolean;
  credit_hours?: number;
  requires_lab: boolean;
  requires_practical: boolean;
  max_students_per_class?: number;
  default_grade_groups?: Record<string, any>;
  prerequisites?: Record<string, any>;
  learning_outcomes?: Record<string, any>;
  color?: string;
  icon?: string;
  cover_image?: string;
  sort_order?: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface OrgUnit {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  code?: string | null;
  unit_type: string;
  parent_id?: string | null;
  path: string;
  depth: number;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface ClassSubject {
  id: string;
  class_id: string;
  subject_id: string;
  cycle_id: string;
  primary_teacher_id: string;
  assistant_teacher_id?: string;
  substitute_teacher_id?: string;
  grade_groups?: Record<string, any>;
  weekly_hours?: number;
  theory_hours?: number;
  practical_hours?: number;
  assessment_config?: Record<string, any>;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface StudentGuardian {
  id: string;
  student_id: string;
  guardian_id: string;
  relationship: string;
  relationship_ar?: string;
  is_primary: boolean;
  is_emergency_contact: boolean;
  is_authorized_pickup: boolean;
  can_view_grades: boolean;
  can_view_attendance: boolean;
  can_view_behavior: boolean;
  can_view_finance: boolean;
  can_view_schedule: boolean;
  can_receive_notifications: boolean;
  is_payer: boolean;
  payment_share_percent?: number;
  occupation?: string;
  employer?: string;
  work_phone?: string;
  notes?: string;
  created_at: string;
  updated_at: string;
}

export interface Notification {
  id: string;
  org_id: string;
  user_id: string;
  title: string;
  title_ar?: string | null;
  message: string;
  message_ar?: string | null;
  notification_type: string;
  priority: string;
  is_read: number;
  created_at: string;
}

export interface Organization {
  id: string;
  name: string;
  name_ar?: string | null;
  slug: string;
  code?: string | null;
  current_period_id?: string | null;
  timezone: string;
  locale: string;
  currency: string;
  subscription_plan: string;
  is_active: number;
  created_at: string;
  updated_at: string;
  primary_color?: string | null;
  date_format?: string | null;
  hijri_enabled?: number | null;
  settings?: string | null;
  logo?: string | null;
}
