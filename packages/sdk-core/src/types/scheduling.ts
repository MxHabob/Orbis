/** Scheduling domain types. */

export interface TimeSlot {
  id: string;
  org_id: string;
  year_id: string;
  slot_number: number;
  name?: string | null;
  name_ar?: string | null;
  start_time: string;
  end_time: string;
  duration_minutes: number;
  is_break: number;
  break_type?: string | null;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface ScheduledLesson {
  id: string;
  org_id: string;
  year_id: string;
  class_id: string;
  class_subject_id: string;
  subject_id: string;
  teacher_id: string;
  substitute_teacher_id?: string | null;
  time_slot_id: string;
  day_of_week: number;
  room_id?: string | null;
  status: string;
  topic?: string | null;
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
