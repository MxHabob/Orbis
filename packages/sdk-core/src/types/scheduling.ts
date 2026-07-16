/** Scheduling domain types. */

export interface TimeSlot {
  id: string;
  org_id: string;
  cycle_id: string;
  slot_number: number;
  name?: string;
  name_ar?: string;
  start_time: string;
  end_time: string;
  duration_minutes: number;
  is_break: boolean;
  break_type?: string;
  applicable_days?: Record<string, any>;
  sort_order?: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface ScheduledLesson {
  id: string;
  org_id: string;
  cycle_id: string;
  class_id: string;
  class_subject_id: string;
  subject_id: string;
  teacher_id: string;
  substitute_teacher_id?: string;
  time_slot_id: string;
  day_of_week: number;
  room_id?: string;
  room_name?: string;
  status: string;
  is_substituted: boolean;
  substitution_reason?: string;
  topic?: string;
  topic_ar?: string;
  objectives?: string;
  materials?: Record<string, any>;
  homework?: string;
  homework_due_date?: string;
  attendance_taken: boolean;
  attendance_taken_at?: string;
  attendance_taken_by?: string;
  attendance_method?: string;
  notes?: string;
  cancellation_reason?: string;
  created_at: string;
  updated_at: string;
}

export interface Room {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  capacity?: number | null;
  room_type: string;
  building?: string | null;
  floor?: number | null;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface SolveJob {
  id: string;
  org_id: string;
  job_type: string;
  target_id: string;
  mode?: string | null;
  status: string;
  items_created: number;
  error_message?: string | null;
  created_at: string;
  updated_at: string;
}
