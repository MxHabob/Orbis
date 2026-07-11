/**
 * Academic domain types — mirrors Rust entities in svc-academic + svc-organization.
 */

export interface AcademicCycle {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  code?: string | null;
  cycle_type: string;
  parent_id?: string | null;
  start_date: string;
  end_date: string;
  hijri_start_date?: string | null;
  hijri_end_date?: string | null;
  exam_start_date?: string | null;
  exam_end_date?: string | null;
  weight?: number | null;
  grading_system: string;
  passing_grade?: number | null;
  max_grade?: number | null;
  is_current: number;
  is_locked: number;
  is_enrollment_open: number;
  is_grade_entry_open: number;
  grade_entry_start?: string | null;
  grade_entry_end?: string | null;
  settings?: string | null;
  sort_order?: number | null;
  created_at: string;
  updated_at: string;
}

export interface Class {
  id: string;
  org_id: string;
  unit_id: string;
  cycle_id: string;
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
  cycle_id: string;
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
  code: string;
  name: string;
  name_ar?: string | null;
  is_core: number;
  is_elective: number;
  credit_hours?: number | null;
  is_active: number;
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
  primary_teacher_id?: string | null;
  weekly_hours?: number | null;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface StudentGuardian {
  id: string;
  student_id: string;
  guardian_id: string;
  relationship: string;
  is_primary: number;
  created_at: string;
  updated_at: string;
}

export interface Document {
  id: string;
  org_id: string;
  owner_id: string;
  name: string;
  title?: string | null;
  item_type: string;
  url?: string | null;
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
  timezone: string;
  locale: string;
  currency: string;
  subscription_plan: string;
  is_active: number;
  created_at: string;
  updated_at: string;
}
