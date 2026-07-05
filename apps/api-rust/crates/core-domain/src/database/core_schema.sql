-- Orbis Core Tables (required for app bootstrap)

CREATE TABLE IF NOT EXISTS organizations (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  name_ar TEXT,
  slug TEXT NOT NULL UNIQUE,
  code TEXT UNIQUE,
  timezone TEXT NOT NULL,
  locale TEXT NOT NULL,
  currency TEXT NOT NULL,
  date_format TEXT,
  hijri_enabled INTEGER NOT NULL,
  max_users INTEGER NOT NULL,
  max_students INTEGER NOT NULL,
  max_storage_gb INTEGER NOT NULL,
  settings TEXT,
  features TEXT,
  subscription_plan TEXT NOT NULL,
  subscription_expires_at TEXT,
  logo TEXT,
  favicon TEXT,
  primary_color TEXT,
  is_active INTEGER NOT NULL,
  is_verified INTEGER NOT NULL,
  is_deleted INTEGER NOT NULL,
  deleted_at TEXT,
  deleted_by TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  email TEXT,
  email_verified INTEGER NOT NULL,
  email_verified_at TEXT,
  phone TEXT,
  phone_verified INTEGER NOT NULL,
  phone_verified_at TEXT,
  password_hash TEXT,
  password_changed_at TEXT,
  password_expires_at TEXT,
  mfa_enabled INTEGER NOT NULL,
  mfa_secret TEXT,
  mfa_backup_codes TEXT,
  full_name TEXT NOT NULL,
  name_ar TEXT,
  name_en TEXT,
  avatar TEXT,
  sex TEXT,
  date_of_birth TEXT,
  place_of_birth TEXT,
  nationality TEXT,
  national_id TEXT,
  passport_number TEXT,
  address TEXT,
  city TEXT,
  country TEXT,
  postal_code TEXT,
  emergency_contact TEXT,
  blood_type TEXT,
  allergies TEXT,
  medical_conditions TEXT,
  medical_notes TEXT,
  social_links TEXT,
  bio TEXT,
  primary_role TEXT NOT NULL,
  status TEXT NOT NULL,
  last_login_at TEXT,
  last_login_ip TEXT,
  login_count INTEGER,
  failed_logins INTEGER,
  locked_until TEXT,
  prefs TEXT NOT NULL,
  meta TEXT NOT NULL,
  is_deleted INTEGER NOT NULL,
  deleted_at TEXT,
  deleted_by TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  created_by TEXT
);

CREATE TABLE IF NOT EXISTS user_sessions (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  token TEXT NOT NULL UNIQUE,
  refresh_token TEXT UNIQUE,
  device_id TEXT,
  device_type TEXT,
  device_name TEXT,
  os TEXT,
  browser TEXT,
  ip_address TEXT,
  user_agent TEXT,
  is_current INTEGER NOT NULL,
  is_trusted INTEGER NOT NULL,
  mfa_verified INTEGER NOT NULL,
  expires_at TEXT NOT NULL,
  refresh_expires_at TEXT,
  last_active_at TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS academic_cycles (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  code TEXT,
  "type" TEXT NOT NULL,
  parent_id TEXT,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  hijri_start_date TEXT,
  hijri_end_date TEXT,
  exam_start_date TEXT,
  exam_end_date TEXT,
  weight INTEGER,
  grading_system TEXT NOT NULL,
  passing_grade INTEGER,
  max_grade INTEGER,
  is_current INTEGER NOT NULL,
  is_locked INTEGER NOT NULL,
  is_enrollment_open INTEGER NOT NULL,
  is_grade_entry_open INTEGER NOT NULL DEFAULT 0,
  grade_entry_start TEXT,
  grade_entry_end TEXT,
  locked_at TEXT,
  locked_by TEXT,
  settings TEXT,
  sort_order INTEGER,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS org_units (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  code TEXT,
  "type" TEXT NOT NULL,
  parent_id TEXT,
  path TEXT NOT NULL,
  depth INTEGER NOT NULL,
  head_id TEXT,
  capacity INTEGER,
  room_number TEXT,
  floor INTEGER,
  building TEXT,
  settings TEXT,
  sort_order INTEGER,
  is_active INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS classes (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  unit_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  level TEXT NOT NULL,
  division TEXT,
  homeroom_teacher_id TEXT,
  capacity INTEGER,
  current_students INTEGER,
  room_id TEXT,
  color TEXT,
  is_active INTEGER NOT NULL,
  is_deleted INTEGER NOT NULL,
  deleted_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS enrollments (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  student_id TEXT NOT NULL,
  class_id TEXT NOT NULL,
  student_number TEXT NOT NULL UNIQUE,
  status TEXT NOT NULL,
  enrollment_date TEXT NOT NULL,
  enrollment_type TEXT NOT NULL,
  withdrawal_date TEXT,
  withdrawal_reason TEXT,
  final_grade TEXT,
  final_score INTEGER,
  final_rank INTEGER,
  is_promoted INTEGER,
  promotion_decision TEXT,
  promotion_notes TEXT,
  meta TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grades (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  assessment_id TEXT,
  class_subject_id TEXT,
  subject_name TEXT NOT NULL,
  subject_name_ar TEXT,
  score REAL NOT NULL,
  max_score REAL NOT NULL,
  weight REAL NOT NULL,
  grade_letter TEXT,
  grade_points REAL,
  exam_type TEXT NOT NULL,
  semester TEXT NOT NULL,
  notes TEXT,
  graded_by TEXT NOT NULL,
  graded_at TEXT NOT NULL,
  is_final INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_grades_assessment ON grades(assessment_id);
CREATE INDEX IF NOT EXISTS idx_grades_class_subject ON grades(class_subject_id, cycle_id);

CREATE TABLE IF NOT EXISTS attendance (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  class_id TEXT NOT NULL,
  date TEXT NOT NULL,
  status TEXT NOT NULL,
  check_in_time TEXT,
  check_out_time TEXT,
  period TEXT,
  subject_name TEXT,
  recorded_by TEXT NOT NULL,
  notes TEXT,
  excuse_type TEXT,
  excuse_document TEXT,
  is_excused INTEGER NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS behavior_records (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  "type" TEXT NOT NULL,
  severity TEXT NOT NULL,
  category TEXT,
  title TEXT NOT NULL,
  title_ar TEXT,
  description TEXT NOT NULL,
  incident_date TEXT NOT NULL,
  incident_time TEXT,
  location TEXT,
  points INTEGER,
  action_taken TEXT,
  action_details TEXT,
  action_completed INTEGER,
  reported_by TEXT NOT NULL,
  reviewed_by TEXT,
  reviewed_at TEXT,
  witnesses TEXT,
  parent_notified INTEGER NOT NULL,
  parent_notified_at TEXT,
  parent_response TEXT,
  evidence_urls TEXT,
  status TEXT NOT NULL,
  resolution_notes TEXT,
  resolved_at TEXT,
  resolved_by TEXT,
  is_private INTEGER NOT NULL,
  ai_source TEXT,
  ai_confidence REAL,
  ai_model_version TEXT,
  camera_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS notifications (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  title TEXT NOT NULL,
  title_ar TEXT,
  message TEXT NOT NULL,
  message_ar TEXT,
  "type" TEXT NOT NULL,
  priority TEXT NOT NULL,
  is_read INTEGER NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS audit_log (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  user_id TEXT,
  action TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  old_data TEXT,
  new_data TEXT,
  ip_address TEXT,
  user_agent TEXT,
  session_id TEXT,
  metadata TEXT NOT NULL,
  created_at TEXT NOT NULL
);

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Academic Calendar Events ──
-- Each event represents an academic milestone with one of three execution modes:
--   manual:    no automation, user must act
--   suggested: system notifies + recommends, user confirms
--   automatic: system executes when prerequisites are met
-- ══════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS academic_calendar_events (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,

  -- Event identity
  event_type TEXT NOT NULL,
    -- enrollment_open, enrollment_close, add_drop_open, add_drop_close,
    -- classes_start, classes_end, exam_prep_start,
    -- exam_start, exam_end, grade_entry_open, grade_entry_close,
    -- results_announcement, holiday, break, teacher_training,
    -- promotion_window_open, promotion_window_close,
    -- cycle_start, cycle_end, custom
  title TEXT NOT NULL,
  title_ar TEXT,
  description TEXT,

  -- Timing
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  start_time TEXT,
  end_time TEXT,

  -- Execution mode (tri-modal: manual | suggested | automatic)
  execution_mode TEXT NOT NULL DEFAULT 'suggested',

  -- Automated action (JSON: {"action": "open_enrollment", "params": {...}})
  auto_action TEXT,
  -- Prerequisites (JSON: conditions that must be true before execution)
  prerequisites TEXT,

  -- Execution tracking
  executed_at TEXT,
  executed_by TEXT,
  execution_result TEXT,
  execution_notes TEXT,

  -- Recurrence
  is_recurring INTEGER NOT NULL DEFAULT 0,
  recurrence_rule TEXT,

  -- Display
  color TEXT,
  is_active INTEGER NOT NULL DEFAULT 1,
  sort_order INTEGER,
  created_by TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,

  FOREIGN KEY (cycle_id) REFERENCES academic_cycles(id)
);

CREATE INDEX IF NOT EXISTS idx_ace_org_cycle ON academic_calendar_events(org_id, cycle_id);
CREATE INDEX IF NOT EXISTS idx_ace_type ON academic_calendar_events(event_type);
CREATE INDEX IF NOT EXISTS idx_ace_date ON academic_calendar_events(start_date, end_date);

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Organization Academic Policies ──
-- Per-organization configuration for academic behaviors.
-- ══════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS org_academic_policies (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL UNIQUE,

  -- Grading defaults
  default_grading_system TEXT NOT NULL DEFAULT 'percentage',
  default_passing_grade INTEGER NOT NULL DEFAULT 50,
  default_max_grade INTEGER NOT NULL DEFAULT 100,

  -- Enrollment policies
  enrollment_auto_close_on_capacity INTEGER NOT NULL DEFAULT 0,
  allow_late_enrollment INTEGER NOT NULL DEFAULT 1,
  late_enrollment_requires_approval INTEGER NOT NULL DEFAULT 1,

  -- Grade entry policies
  grade_entry_auto_close_on_deadline INTEGER NOT NULL DEFAULT 1,
  allow_grade_modification_after_close INTEGER NOT NULL DEFAULT 0,
  grade_modification_requires_approval INTEGER NOT NULL DEFAULT 1,

  -- Calendar defaults
  default_event_execution_mode TEXT NOT NULL DEFAULT 'suggested',

  -- Extensibility
  settings TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Promotion Policies ──
-- Per-organization promotion configuration with configurable fee carry-over.
-- ══════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS promotion_policies (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL UNIQUE,

  -- Scoring
  passing_score INTEGER NOT NULL DEFAULT 50,
  min_attendance_rate REAL NOT NULL DEFAULT 75.0,

  -- Graduation
  auto_graduate_final_level INTEGER NOT NULL DEFAULT 1,
  final_level TEXT,

  -- Exclusions
  exclude_withdrawn INTEGER NOT NULL DEFAULT 1,

  -- Fee carry-over policy (4 modes)
  fee_carry_over_mode TEXT NOT NULL DEFAULT 'ignore',
    -- ignore:           do not transfer fees
    -- carry_all:        transfer all outstanding balances
    -- carry_selected:   transfer only selected fee categories
    -- require_approval: require finance approval before transfer
  fee_carry_over_categories TEXT,

  -- Data carry-over
  carry_over_guardians INTEGER NOT NULL DEFAULT 1,
  carry_over_time_slots INTEGER NOT NULL DEFAULT 1,
  carry_over_class_subjects INTEGER NOT NULL DEFAULT 1,

  -- Auto-create academic structure in new year
  auto_create_cycles INTEGER NOT NULL DEFAULT 1,
  cycle_template TEXT,

  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
