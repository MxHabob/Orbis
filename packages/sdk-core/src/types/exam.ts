/** Exam domain types. */

export interface ExamSession {
  id: string;
  org_id: string;
  period_id: string;
  name: string;
  name_ar?: string | null;
  session_type: string;
  status: string;
  start_date: string;
  end_date: string;
  is_results_published: number;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface ExamSubject {
  id: string;
  session_id: string;
  subject_id: string;
  subject_name: string;
  exam_date: string;
  start_time: string;
  end_time: string;
  max_score: number;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface ExamSchedule {
  id: string;
  exam_session_id: string;
  exam_subject_id: string;
  exam_date: string;
  start_time: string;
  end_time: string;
  period: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface ExamHall {
  id: string;
  org_id: string;
  name: string;
  name_ar?: string | null;
  capacity: number;
  is_active: number;
  created_at: string;
  updated_at: string;
}

export interface SeatingNumber {
  id: string;
  exam_subject_id: string;
  enrollment_id: string;
  seat_number: number;
  hall_id: string;
  created_at: string;
}
