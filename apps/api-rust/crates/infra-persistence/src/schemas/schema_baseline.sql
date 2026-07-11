-- Orbis SQLite Schema (auto-generated)
PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS user_accounts (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  provider_account_id TEXT NOT NULL,
  provider_email TEXT,
  provider_name TEXT,
  provider_picture TEXT,
  access_token TEXT,
  refresh_token TEXT,
  id_token TEXT,
  expires_at TEXT,
  scope TEXT,
  token_type TEXT,
  meta TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_roles (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  role TEXT NOT NULL,
  org_id TEXT NOT NULL,
  scope_type TEXT NOT NULL,
  scope_id TEXT,
  valid_from TEXT,
  valid_until TEXT,
  is_primary INTEGER NOT NULL,
  assigned_by TEXT,
  assigned_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS permissions (
  id INTEGER PRIMARY KEY,
  code TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  description TEXT,
  module TEXT NOT NULL,
  action TEXT NOT NULL,
  resource TEXT NOT NULL,
  requires_scope INTEGER NOT NULL,
  scope_types TEXT,
  is_system INTEGER NOT NULL,
  is_active INTEGER NOT NULL,
  created_at TEXT NOT NULL
);

DROP TABLE IF EXISTS role_permissions;

CREATE TABLE IF NOT EXISTS role_permissions (
  role TEXT NOT NULL,
  document_type TEXT NOT NULL,
  level INTEGER NOT NULL DEFAULT 0,
  only_if_creator INTEGER NOT NULL DEFAULT 0,
  actions TEXT NOT NULL,
  created_at TEXT NOT NULL,
  PRIMARY KEY (role, document_type)
);

CREATE TABLE IF NOT EXISTS user_permissions (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  permission_id INTEGER NOT NULL,
  grant_type TEXT NOT NULL,
  conditions TEXT,
  valid_from TEXT,
  valid_until TEXT,
  reason TEXT,
  granted_by TEXT,
  granted_at TEXT NOT NULL,
  is_active INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS verifications (
  id TEXT PRIMARY KEY,
  user_id TEXT,
  identifier TEXT NOT NULL,
  value TEXT NOT NULL,
  "type" TEXT NOT NULL,
  attempts INTEGER,
  max_attempts INTEGER,
  expires_at TEXT NOT NULL,
  used_at TEXT,
  ip_address TEXT,
  user_agent TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS notification_preferences (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  notification_type TEXT NOT NULL,
  in_app_enabled INTEGER NOT NULL,
  email_enabled INTEGER NOT NULL,
  sms_enabled INTEGER NOT NULL,
  push_enabled INTEGER NOT NULL,
  whatsapp_enabled INTEGER NOT NULL,
  quiet_hours_enabled INTEGER NOT NULL,
  quiet_hours_start TEXT,
  quiet_hours_end TEXT,
  digest_mode TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS documents (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  owner_id TEXT NOT NULL,
  uploaded_by TEXT,
  name TEXT NOT NULL,
  title TEXT,
  description TEXT,
  "type" TEXT NOT NULL,
  visibility TEXT NOT NULL,
  size INTEGER,
  mime_type TEXT,
  extension TEXT,
  storage_provider TEXT NOT NULL,
  bucket TEXT,
  key TEXT NOT NULL,
  url TEXT,
  thumbnail_url TEXT,
  parent_id TEXT,
  path TEXT,
  depth INTEGER,
  shared_with TEXT,
  share_link TEXT,
  share_link_expires_at TEXT,
  share_password TEXT,
  version INTEGER,
  previous_version_id TEXT,
  is_latest_version INTEGER NOT NULL,
  processing_status TEXT,
  ocr_text TEXT,
  extracted_data TEXT,
  is_pinned INTEGER NOT NULL,
  is_starred INTEGER NOT NULL,
  is_template INTEGER NOT NULL,
  tags TEXT,
  meta TEXT,
  deleted_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subjects (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  unit_id TEXT,
  code TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  name_en TEXT,
  description TEXT,
  short_description TEXT,
  is_core INTEGER NOT NULL,
  is_elective INTEGER NOT NULL,
  credit_hours INTEGER,
  requires_lab INTEGER NOT NULL,
  requires_practical INTEGER NOT NULL,
  max_students_per_class INTEGER,
  default_grade_groups TEXT,
  prerequisites TEXT,
  learning_outcomes TEXT,
  color TEXT,
  icon TEXT,
  cover_image TEXT,
  sort_order INTEGER,
  is_active INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS class_subjects (
  id TEXT PRIMARY KEY,
  class_id TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  primary_teacher_id TEXT NOT NULL,
  assistant_teacher_id TEXT,
  substitute_teacher_id TEXT,
  grade_groups TEXT,
  weekly_hours INTEGER,
  theory_hours INTEGER,
  practical_hours INTEGER,
  assessment_config TEXT,
  is_active INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS student_guardians (
  id TEXT PRIMARY KEY,
  student_id TEXT NOT NULL,
  guardian_id TEXT NOT NULL,
  relationship TEXT NOT NULL,
  relationship_ar TEXT,
  is_primary INTEGER NOT NULL,
  is_emergency_contact INTEGER NOT NULL,
  is_authorized_pickup INTEGER NOT NULL,
  can_view_grades INTEGER NOT NULL,
  can_view_attendance INTEGER NOT NULL,
  can_view_behavior INTEGER NOT NULL,
  can_view_finance INTEGER NOT NULL,
  can_view_schedule INTEGER NOT NULL,
  can_receive_notifications INTEGER NOT NULL,
  is_payer INTEGER NOT NULL,
  payment_share_percent REAL,
  occupation TEXT,
  employer TEXT,
  work_phone TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS time_slots (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  slot_number INTEGER NOT NULL,
  name TEXT,
  name_ar TEXT,
  start_time TEXT NOT NULL,
  end_time TEXT NOT NULL,
  duration_minutes INTEGER NOT NULL,
  is_break INTEGER NOT NULL,
  break_type TEXT,
  applicable_days TEXT,
  sort_order INTEGER,
  is_active INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS scheduled_lessons (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  class_id TEXT NOT NULL,
  class_subject_id TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  teacher_id TEXT NOT NULL,
  substitute_teacher_id TEXT,
  time_slot_id TEXT NOT NULL,
  day_of_week INTEGER NOT NULL,
  room_id TEXT,
  room_name TEXT,
  status TEXT NOT NULL,
  is_substituted INTEGER NOT NULL,
  substitution_reason TEXT,
  topic TEXT,
  topic_ar TEXT,
  objectives TEXT,
  materials TEXT,
  homework TEXT,
  homework_due_date TEXT,
  attendance_taken INTEGER NOT NULL,
  attendance_taken_at TEXT,
  attendance_taken_by TEXT,
  attendance_method TEXT,
  notes TEXT,
  cancellation_reason TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assessments (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  class_subject_id TEXT NOT NULL,
  title TEXT NOT NULL,
  title_ar TEXT,
  description TEXT,
  "type" TEXT NOT NULL,
  max_score REAL NOT NULL,
  passing_score REAL,
  weight REAL NOT NULL,
  grade_group_name TEXT NOT NULL,
  assessment_date TEXT,
  due_date TEXT,
  duration_minutes INTEGER,
  is_published INTEGER NOT NULL,
  published_at TEXT,
  published_by TEXT,
  is_locked INTEGER NOT NULL,
  locked_at TEXT,
  locked_by TEXT,
  grading_method TEXT,
  rubric_id TEXT,
  instructions TEXT,
  instructions_ar TEXT,
  attachments TEXT,
  created_by TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_assessments_cs ON assessments(class_subject_id, cycle_id);

CREATE TABLE IF NOT EXISTS grade_history (
  id TEXT PRIMARY KEY,
  grade_id TEXT NOT NULL,
  score REAL,
  grade_letter TEXT,
  is_absent INTEGER,
  is_excused INTEGER,
  comment TEXT,
  changed_by TEXT,
  changed_at TEXT NOT NULL,
  change_reason TEXT,
  change_type TEXT NOT NULL,
  snapshot TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS attendance_daily (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  date TEXT NOT NULL,
  status TEXT NOT NULL,
  lessons TEXT,
  total_lessons INTEGER,
  present_lessons INTEGER,
  absent_lessons INTEGER,
  late_lessons INTEGER,
  excused_lessons INTEGER,
  late_minutes INTEGER,
  excuse_type TEXT,
  excuse_document TEXT,
  excuse_notes TEXT,
  is_verified INTEGER NOT NULL,
  verified_by TEXT,
  verified_at TEXT,
  marked_by TEXT NOT NULL,
  marked_at TEXT NOT NULL,
  mark_method TEXT,
  device_id TEXT,
  location TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS attendance_summary (
  id TEXT PRIMARY KEY,
  enrollment_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  total_lessons INTEGER,
  present_lessons INTEGER,
  absent_lessons INTEGER,
  late_lessons INTEGER,
  excused_lessons INTEGER,
  late_minutes INTEGER,
  attendance_rate REAL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- ── Currencies ──
CREATE TABLE IF NOT EXISTS currencies (
  code TEXT NOT NULL,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  symbol TEXT NOT NULL,
  symbol_position TEXT NOT NULL DEFAULT 'after',
  decimal_places INTEGER NOT NULL DEFAULT 0,
  exchange_rate REAL NOT NULL DEFAULT 1.0,
  is_base INTEGER NOT NULL DEFAULT 0,
  is_active INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (code, org_id)
);

-- ── Ledger Accounts (Chart of Accounts with sub-accounts for students/employees) ──
CREATE TABLE IF NOT EXISTS ledger_accounts (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  code TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  "type" TEXT NOT NULL,
  sub_type TEXT,
  parent_id TEXT,
  path TEXT NOT NULL,
  depth INTEGER NOT NULL,
  currency TEXT NOT NULL,
  opening_balance REAL DEFAULT 0,
  current_balance REAL DEFAULT 0,
  entity_type TEXT,
  entity_id TEXT,
  is_control_account INTEGER NOT NULL DEFAULT 0,
  is_system_account INTEGER NOT NULL DEFAULT 0,
  is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_ledger_entity ON ledger_accounts(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_ledger_parent ON ledger_accounts(parent_id);
CREATE INDEX IF NOT EXISTS idx_ledger_org_code ON ledger_accounts(org_id, code);

-- ── Journal Entries (Source of Truth for all financial movements) ──
CREATE TABLE IF NOT EXISTS journal_entries (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  transaction_ref TEXT NOT NULL,
  accounting_date TEXT NOT NULL,
  fiscal_period_id TEXT NOT NULL,
  amount REAL NOT NULL,
  debit_account_id TEXT NOT NULL,
  credit_account_id TEXT NOT NULL,
  currency TEXT NOT NULL,
  exchange_rate REAL,
  base_currency_amount REAL,
  source_type TEXT NOT NULL,
  source_id TEXT,
  source_module TEXT,
  description TEXT NOT NULL,
  description_ar TEXT,
  metadata TEXT,
  is_reversed INTEGER NOT NULL DEFAULT 0,
  reversed_by TEXT,
  reversal_reason TEXT,
  is_reconciled INTEGER NOT NULL DEFAULT 0,
  reconciled_at TEXT,
  reconciliation_ref TEXT,
  status TEXT NOT NULL DEFAULT 'posted',
  posted_by TEXT,
  posted_at TEXT,
  approved_by TEXT,
  approved_at TEXT,
  created_by TEXT,
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_je_debit ON journal_entries(debit_account_id);
CREATE INDEX IF NOT EXISTS idx_je_credit ON journal_entries(credit_account_id);
CREATE INDEX IF NOT EXISTS idx_je_source ON journal_entries(source_type, source_id);

-- ── Fee Templates ──
CREATE TABLE IF NOT EXISTS fee_templates (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  code TEXT,
  category TEXT NOT NULL,
  default_amount REAL NOT NULL DEFAULT 0,
  revenue_account_id TEXT NOT NULL,
  receivable_account_id TEXT NOT NULL,
  applicable_levels TEXT,
  frequency TEXT NOT NULL DEFAULT 'once',
  installment_config TEXT,
  late_fee_rules TEXT,
  is_mandatory INTEGER NOT NULL DEFAULT 1,
  is_active INTEGER NOT NULL DEFAULT 1,
  sort_order INTEGER,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- ── Discount Templates ──
CREATE TABLE IF NOT EXISTS discount_templates (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  code TEXT,
  "type" TEXT NOT NULL,
  calculation_method TEXT NOT NULL,
  value INTEGER NOT NULL,
  eligibility_rules TEXT,
  limits TEXT,
  requires_approval INTEGER NOT NULL DEFAULT 0,
  approver_roles TEXT,
  is_auto_apply INTEGER NOT NULL DEFAULT 0,
  is_stackable INTEGER NOT NULL DEFAULT 0,
  priority INTEGER,
  is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- ── Student Fees (Invoice only — balance comes from ledger_accounts) ──
CREATE TABLE IF NOT EXISTS student_fees (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  fee_template_id TEXT NOT NULL,
  original_amount REAL NOT NULL,
  discount_amount REAL NOT NULL DEFAULT 0,
  net_amount REAL NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  due_date TEXT NOT NULL,
  discount_details TEXT,
  parent_fee_id TEXT,
  installment_number INTEGER,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_sf_enrollment ON student_fees(enrollment_id);

-- ── Payments (Multi-currency, supports students + employees) ──
CREATE TABLE IF NOT EXISTS payments (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  fee_id TEXT,
  enrollment_id TEXT,
  account_id TEXT NOT NULL,
  amount REAL NOT NULL,
  currency TEXT NOT NULL,
  exchange_rate REAL NOT NULL DEFAULT 1.0,
  base_amount REAL NOT NULL,
  method TEXT NOT NULL,
  reference TEXT,
  receipt_number TEXT,
  journal_entry_id TEXT,
  treasury_id TEXT,
  payer_name TEXT,
  payer_phone TEXT,
  payment_type TEXT NOT NULL DEFAULT 'receipt',
  related_payment_id TEXT,
  notes TEXT,
  paid_at TEXT NOT NULL,
  created_at TEXT NOT NULL,
  created_by TEXT
);
CREATE INDEX IF NOT EXISTS idx_pay_account ON payments(account_id);

-- ── Treasuries ──
CREATE TABLE IF NOT EXISTS treasuries (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  unit_id TEXT,
  name TEXT NOT NULL,
  name_ar TEXT,
  code TEXT,
  "type" TEXT NOT NULL,
  ledger_account_id TEXT NOT NULL,
  bank_name TEXT,
  bank_branch TEXT,
  account_number TEXT,
  account_name TEXT,
  iban TEXT,
  swift_code TEXT,
  opening_balance REAL DEFAULT 0,
  current_balance REAL DEFAULT 0,
  currency TEXT NOT NULL,
  authorized_users TEXT,
  is_default INTEGER NOT NULL DEFAULT 0,
  is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- ── Treasury Transactions ──
CREATE TABLE IF NOT EXISTS treasury_transactions (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  treasury_id TEXT NOT NULL,
  journal_entry_id TEXT NOT NULL,
  "type" TEXT NOT NULL,
  category TEXT NOT NULL,
  amount REAL NOT NULL,
  currency TEXT NOT NULL,
  exchange_rate REAL,
  reference_type TEXT,
  reference_id TEXT,
  description TEXT NOT NULL,
  notes TEXT,
  attachments TEXT,
  created_by TEXT,
  approved_by TEXT,
  approved_at TEXT,
  transaction_date TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS system_log (
  id TEXT PRIMARY KEY,
  org_id TEXT,
  level TEXT NOT NULL,
  category TEXT NOT NULL,
  message TEXT NOT NULL,
  details TEXT,
  stack_trace TEXT,
  user_id TEXT,
  request_id TEXT,
  endpoint TEXT,
  method TEXT,
  duration_ms INTEGER,
  memory_usage_mb INTEGER,
  cpu_usage_percent INTEGER,
  timestamp TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS data_transfers (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  "type" TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  file_name TEXT,
  file_url TEXT,
  file_size INTEGER,
  file_format TEXT,
  total_records INTEGER,
  success_records INTEGER,
  failed_records INTEGER,
  skipped_records INTEGER,
  error_log TEXT,
  warning_log TEXT,
  status TEXT NOT NULL,
  started_at TEXT,
  completed_at TEXT,
  cancelled_at TEXT,
  cancel_reason TEXT,
  initiated_by TEXT,
  config TEXT,
  mapping TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report_definitions (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  description TEXT,
  base_entity TEXT NOT NULL,
  columns TEXT NOT NULL,
  joins TEXT,
  default_filters TEXT,
  group_by TEXT,
  sort_by TEXT,
  is_system INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS document_templates (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  layout_type TEXT NOT NULL,
  content TEXT NOT NULL,
  page_size TEXT NOT NULL,
  orientation TEXT NOT NULL,
  margins TEXT NOT NULL,
  is_default INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS rooms (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  capacity INTEGER,
  room_type TEXT NOT NULL,
  building TEXT,
  floor INTEGER,
  is_active INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS teacher_availability (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  teacher_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  day_of_week INTEGER NOT NULL,
  time_slot_id TEXT NOT NULL,
  is_available INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS solve_jobs (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  job_type TEXT NOT NULL,
  target_id TEXT NOT NULL,
  mode TEXT,
  status TEXT NOT NULL,
  problem_snapshot TEXT,
  best_solution TEXT,
  score TEXT,
  items_created INTEGER NOT NULL,
  error_message TEXT,
  created_by TEXT,
  started_at TEXT,
  completed_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- ══════════════════════════════════════════════════════════════════════════════
-- ── Examination Management System ──
-- ══════════════════════════════════════════════════════════════════════════════

-- ── Student Ordering (Centralized Ordering Engine) ──
CREATE TABLE IF NOT EXISTS student_ordering (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL UNIQUE,
  student_id TEXT NOT NULL,
  class_id TEXT NOT NULL,
  sort_key TEXT NOT NULL,
  order_in_class INTEGER NOT NULL,
  order_in_division INTEGER,
  order_in_level INTEGER,
  order_in_org INTEGER,
  level TEXT NOT NULL,
  division TEXT,
  student_name_ar TEXT,
  student_sex TEXT,
  academic_number TEXT,
  is_frozen INTEGER NOT NULL DEFAULT 0,
  frozen_at TEXT,
  frozen_by TEXT,
  strategy TEXT NOT NULL DEFAULT 'name_ar_asc',
  version INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_so_class_order ON student_ordering(class_id, order_in_class);
CREATE INDEX IF NOT EXISTS idx_so_sort_key ON student_ordering(org_id, cycle_id, sort_key);
CREATE INDEX IF NOT EXISTS idx_so_level ON student_ordering(org_id, cycle_id, level, order_in_level);

-- ── Exam Sessions (Examination Cycles) ──
CREATE TABLE IF NOT EXISTS exam_sessions (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  cycle_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  code TEXT,
  "type" TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'draft',
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  hijri_start_date TEXT,
  hijri_end_date TEXT,
  applicable_levels TEXT,
  applicable_class_ids TEXT,
  settings TEXT,
  grading_deadline TEXT,
  results_publish_date TEXT,
  is_results_published INTEGER NOT NULL DEFAULT 0,
  created_by TEXT NOT NULL,
  approved_by TEXT,
  approved_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_es_org_cycle ON exam_sessions(org_id, cycle_id);
CREATE INDEX IF NOT EXISTS idx_es_status ON exam_sessions(org_id, status);

-- ── Exam Subjects ──
CREATE TABLE IF NOT EXISTS exam_subjects (
  id TEXT PRIMARY KEY,
  exam_session_id TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  class_subject_id TEXT,
  max_score REAL NOT NULL,
  passing_score REAL NOT NULL,
  practical_score REAL DEFAULT 0,
  oral_score REAL DEFAULT 0,
  coursework_score REAL DEFAULT 0,
  exam_written_score REAL DEFAULT 0,
  duration_minutes INTEGER NOT NULL DEFAULT 90,
  exam_type TEXT NOT NULL DEFAULT 'written',
  is_included INTEGER NOT NULL DEFAULT 1,
  weight REAL NOT NULL DEFAULT 1.0,
  instructions TEXT,
  instructions_ar TEXT,
  sort_order INTEGER,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_exs_session ON exam_subjects(exam_session_id);

-- ── Exam Schedules ──
CREATE TABLE IF NOT EXISTS exam_schedules (
  id TEXT PRIMARY KEY,
  exam_session_id TEXT NOT NULL,
  exam_subject_id TEXT NOT NULL,
  exam_date TEXT NOT NULL,
  start_time TEXT NOT NULL,
  end_time TEXT NOT NULL,
  period TEXT NOT NULL DEFAULT 'morning',
  applicable_class_ids TEXT,
  room_ids TEXT,
  status TEXT NOT NULL DEFAULT 'scheduled',
  postponed_to TEXT,
  postponement_reason TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_eschd_session ON exam_schedules(exam_session_id);
CREATE INDEX IF NOT EXISTS idx_eschd_date ON exam_schedules(exam_date);

-- ── Seating Numbers ──
CREATE TABLE IF NOT EXISTS seating_numbers (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  exam_session_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  student_id TEXT NOT NULL,
  class_id TEXT NOT NULL,
  seating_number TEXT NOT NULL,
  sequence_number INTEGER NOT NULL,
  level TEXT NOT NULL,
  division TEXT,
  student_name_ar TEXT,
  is_active INTEGER NOT NULL DEFAULT 1,
  generated_at TEXT NOT NULL,
  generated_by TEXT NOT NULL,
  voided_at TEXT,
  voided_by TEXT,
  void_reason TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_sn_unique ON seating_numbers(exam_session_id, seating_number);
CREATE UNIQUE INDEX IF NOT EXISTS idx_sn_enrollment ON seating_numbers(exam_session_id, enrollment_id);
CREATE INDEX IF NOT EXISTS idx_sn_class ON seating_numbers(exam_session_id, class_id);

-- ── Secret Numbers ──
CREATE TABLE IF NOT EXISTS secret_numbers (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  exam_session_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  student_id TEXT NOT NULL,
  secret_number TEXT NOT NULL,
  secret_hash TEXT NOT NULL,
  barcode_data TEXT,
  qr_data TEXT,
  is_active INTEGER NOT NULL DEFAULT 1,
  is_revealed INTEGER NOT NULL DEFAULT 0,
  revealed_at TEXT,
  revealed_by TEXT,
  generated_at TEXT NOT NULL,
  generated_by TEXT NOT NULL,
  generation_batch TEXT NOT NULL,
  previous_secret_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_secn_unique ON secret_numbers(exam_session_id, secret_number);
CREATE UNIQUE INDEX IF NOT EXISTS idx_secn_enrollment ON secret_numbers(exam_session_id, enrollment_id);

-- ── Exam Halls ──
CREATE TABLE IF NOT EXISTS exam_halls (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  exam_session_id TEXT NOT NULL,
  room_id TEXT NOT NULL,
  hall_number TEXT NOT NULL,
  capacity INTEGER NOT NULL,
  allocated_count INTEGER NOT NULL DEFAULT 0,
  seating_arrangement TEXT DEFAULT 'rows',
  floor_number INTEGER,
  building TEXT,
  is_accessible INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'active',
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_eh_session ON exam_halls(exam_session_id);

-- ── Exam Hall Allocations ──
CREATE TABLE IF NOT EXISTS exam_hall_allocations (
  id TEXT PRIMARY KEY,
  exam_hall_id TEXT NOT NULL,
  exam_session_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  seating_number_id TEXT,
  seat_position TEXT,
  is_present INTEGER,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_eha_unique ON exam_hall_allocations(exam_session_id, enrollment_id);
CREATE INDEX IF NOT EXISTS idx_eha_hall ON exam_hall_allocations(exam_hall_id);

-- ── Exam Proctors ──
CREATE TABLE IF NOT EXISTS exam_proctors (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  exam_session_id TEXT NOT NULL,
  exam_schedule_id TEXT NOT NULL,
  exam_hall_id TEXT NOT NULL,
  teacher_id TEXT NOT NULL,
  role TEXT NOT NULL DEFAULT 'proctor',
  is_confirmed INTEGER NOT NULL DEFAULT 0,
  confirmed_at TEXT,
  attendance_status TEXT,
  check_in_time TEXT,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_ep_schedule ON exam_proctors(exam_schedule_id);
CREATE INDEX IF NOT EXISTS idx_ep_teacher ON exam_proctors(teacher_id);

-- ── Teacher Exam Availability ──
CREATE TABLE IF NOT EXISTS teacher_exam_availability (
  id TEXT PRIMARY KEY,
  org_id TEXT NOT NULL,
  exam_session_id TEXT NOT NULL,
  teacher_id TEXT NOT NULL,
  date TEXT NOT NULL,
  is_available INTEGER NOT NULL DEFAULT 1,
  reason TEXT,
  approved_by TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_tea_teacher ON teacher_exam_availability(teacher_id, exam_session_id);

-- ── Search Optimizations for Students & Guardians ──
CREATE INDEX IF NOT EXISTS idx_users_org_role ON users(org_id, primary_role);
CREATE INDEX IF NOT EXISTS idx_users_national_id ON users(national_id);
CREATE INDEX IF NOT EXISTS idx_users_phone ON users(phone);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_student_guardians_student ON student_guardians(student_id);
CREATE INDEX IF NOT EXISTS idx_student_guardians_guardian ON student_guardians(guardian_id);
