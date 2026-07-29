-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- Orbis Unified Schema â€” Single Database Architecture
-- All tables in one database. No registry.db, no year DBs.
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
PRAGMA foreign_keys=ON;

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Organizations â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS organizations (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  name TEXT NOT NULL,
  name_ar TEXT,
  slug TEXT UNIQUE,
  code TEXT UNIQUE,
  timezone TEXT NOT NULL DEFAULT 'Asia/Riyadh',
  locale TEXT NOT NULL DEFAULT 'ar',
  currency TEXT NOT NULL DEFAULT 'SAR',
  date_format TEXT,
  hijri_enabled INTEGER NOT NULL DEFAULT 1,
  max_users INTEGER NOT NULL DEFAULT 500,
  max_students INTEGER NOT NULL DEFAULT 2000,
  max_storage_gb INTEGER NOT NULL DEFAULT 10,
  settings TEXT,
  features TEXT,
  subscription_plan TEXT NOT NULL DEFAULT 'basic',
  subscription_expires_at TEXT,
  logo TEXT,
  favicon TEXT,
  primary_color TEXT,
  is_active INTEGER NOT NULL DEFAULT 1,
  is_verified INTEGER NOT NULL DEFAULT 0,
  is_deleted INTEGER NOT NULL DEFAULT 0,
  deleted_at TEXT,
  deleted_by TEXT,
  current_period_id TEXT,
  metadata TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Users â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS parties (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  party_type TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS persons (
  id TEXT PRIMARY KEY,
  party_id TEXT NOT NULL REFERENCES parties(id) ON DELETE CASCADE,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  gender TEXT,
  birth_date TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  email TEXT,
  email_verified INTEGER NOT NULL DEFAULT 0,
  email_verified_at TEXT,
  phone TEXT,
  phone_verified INTEGER NOT NULL DEFAULT 0,
  phone_verified_at TEXT,
  password_hash TEXT,
  password_changed_at TEXT,
  password_expires_at TEXT,
  mfa_enabled INTEGER NOT NULL DEFAULT 0,
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
  primary_role TEXT NOT NULL DEFAULT 'user',
  status TEXT NOT NULL DEFAULT 'active',
  last_login_at TEXT,
  last_login_ip TEXT,
  login_count INTEGER,
  failed_logins INTEGER,
  locked_until TEXT,
  prefs TEXT NOT NULL DEFAULT '{}',
  meta TEXT NOT NULL DEFAULT '{}',
  is_deleted INTEGER NOT NULL DEFAULT 0,
  deleted_at TEXT,
  deleted_by TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  created_by TEXT,
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Org Units (Branches / Campuses / Divisions) â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS org_units (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  parent_id TEXT,
  name TEXT NOT NULL,
  name_ar TEXT,
  code TEXT,
  unit_type TEXT NOT NULL DEFAULT 'division',
  sort_order INTEGER,
  is_active INTEGER NOT NULL DEFAULT 1,
  path TEXT NOT NULL DEFAULT '/',
  depth INTEGER NOT NULL DEFAULT 0,
  head_id TEXT,
  capacity INTEGER,
  room_number TEXT,
  floor INTEGER,
  building TEXT,
  settings TEXT,
  local_policies TEXT,
  data_isolation TEXT NOT NULL DEFAULT 'shared',
  children_count INTEGER NOT NULL DEFAULT 0,
  descendant_count INTEGER NOT NULL DEFAULT 0,
  metadata TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (org_id) REFERENCES organizations(id),
  FOREIGN KEY (parent_id) REFERENCES org_units(id)
);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Academic Year Management â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS academic_years (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  hijri_start TEXT,
  hijri_end TEXT,
  status TEXT NOT NULL DEFAULT 'planning'
    CHECK (status IN ('planning','active','closing','archived')),
  is_locked INTEGER NOT NULL DEFAULT 0,
  locked_at TEXT,
  locked_by TEXT,
  lock_reason TEXT,
    current_period_id TEXT,
  archived_at TEXT,
  archived_by TEXT,
  notes TEXT,
  metadata TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE INDEX IF NOT EXISTS idx_ay_org ON academic_years(org_id);
CREATE INDEX IF NOT EXISTS idx_ay_org_status ON academic_years(org_id, status);

-- FK: organizations.current_period_id â†’ academic_years.id
-- (enforced at application level since SQLite ALTER TABLE ADD CONSTRAINT is limited)

-- â”€â”€ Academic Periods â”€â”€
CREATE TABLE IF NOT EXISTS academic_periods (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  parent_id TEXT,
  name TEXT NOT NULL,
  name_ar TEXT,
  code TEXT,
  period_type TEXT NOT NULL
    CHECK (period_type IN ('year','semester','trimester','quarter','module','block','custom','term','month')),
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  hijri_start TEXT,
  hijri_end TEXT,
  enrollment_start TEXT,
  enrollment_end TEXT,
  is_enrollment_open INTEGER NOT NULL DEFAULT 0,
  grade_entry_start TEXT,
  grade_entry_end TEXT,
  is_grade_entry_open INTEGER NOT NULL DEFAULT 0,
  exam_start TEXT,
  exam_end TEXT,
  grading_system TEXT NOT NULL DEFAULT 'percentage',
  passing_grade INTEGER,
  max_grade INTEGER,
  weight INTEGER,
  status TEXT NOT NULL DEFAULT 'planned'
    CHECK (status IN ('planned','active','grading','closed')),
  is_locked INTEGER NOT NULL DEFAULT 0,
  locked_at TEXT,
  locked_by TEXT,
  sort_order INTEGER,
  metadata TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (period_id) REFERENCES academic_years(id),
  FOREIGN KEY (parent_id) REFERENCES academic_periods(id),
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE INDEX IF NOT EXISTS idx_ap_period ON academic_periods(period_id);
CREATE INDEX IF NOT EXISTS idx_ap_org_period ON academic_periods(org_id, period_id);

-- â”€â”€ Academic Policies (unified: grading + enrollment + promotion + autopilot) â”€â”€
CREATE TABLE IF NOT EXISTS academic_policies (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  org_unit_id TEXT,
  default_grading_system TEXT NOT NULL DEFAULT 'percentage',
  default_passing_grade INTEGER NOT NULL DEFAULT 50,
  default_max_grade INTEGER NOT NULL DEFAULT 100,
  enrollment_auto_close INTEGER NOT NULL DEFAULT 1,
  allow_late_enrollment INTEGER NOT NULL DEFAULT 0,
  late_enrollment_approval INTEGER NOT NULL DEFAULT 1,
  grade_entry_auto_close INTEGER NOT NULL DEFAULT 1,
  allow_grade_edit_after_close INTEGER NOT NULL DEFAULT 0,
  grade_edit_approval INTEGER NOT NULL DEFAULT 1,
  passing_score INTEGER NOT NULL DEFAULT 50,
  min_attendance_rate REAL NOT NULL DEFAULT 75.0,
  auto_graduate_final_level INTEGER NOT NULL DEFAULT 0,
  final_level TEXT,
  exclude_withdrawn INTEGER NOT NULL DEFAULT 1,
  autopilot_mode TEXT NOT NULL DEFAULT 'suggested'
    CHECK (autopilot_mode IN ('automatic','suggested','manual')),
  autopilot_enabled INTEGER NOT NULL DEFAULT 1,
  max_auto_retries INTEGER NOT NULL DEFAULT 3,
  pause_on_error INTEGER NOT NULL DEFAULT 0,
  auto_open_enrollment INTEGER NOT NULL DEFAULT 1,
  auto_close_enrollment INTEGER NOT NULL DEFAULT 1,
  auto_open_grade_entry INTEGER NOT NULL DEFAULT 1,
  auto_close_grade_entry INTEGER NOT NULL DEFAULT 1,
  auto_lock_periods INTEGER NOT NULL DEFAULT 1,
  auto_advance_periods INTEGER NOT NULL DEFAULT 0,
  auto_archive_years INTEGER NOT NULL DEFAULT 0,
  grace_days_after_deadline INTEGER NOT NULL DEFAULT 3,
  notify_before_deadline_days INTEGER NOT NULL DEFAULT 7,
  default_template_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ap_org_default
  ON academic_policies(org_id) WHERE org_unit_id IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_ap_org_unit
  ON academic_policies(org_id, org_unit_id) WHERE org_unit_id IS NOT NULL;

-- â”€â”€ Period Templates â”€â”€
CREATE TABLE IF NOT EXISTS period_templates (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  institution_type TEXT NOT NULL DEFAULT 'school'
    CHECK (institution_type IN ('school','university','institute','custom')),
  structure TEXT NOT NULL,
  is_default INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Classes & Enrollments â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS classes (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  unit_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  level TEXT NOT NULL,
  division TEXT,
  homeroom_teacher_id TEXT,
  capacity INTEGER,
  current_students INTEGER DEFAULT 0,
  room_id TEXT,
  color TEXT,
  is_active INTEGER NOT NULL DEFAULT 1,
  is_deleted INTEGER NOT NULL DEFAULT 0,
  deleted_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (period_id) REFERENCES academic_periods(id),
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE INDEX IF NOT EXISTS idx_classes_org_period ON classes(org_id, period_id);

CREATE TABLE IF NOT EXISTS enrollments (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  student_id TEXT NOT NULL,
  class_id TEXT NOT NULL,
  student_number TEXT UNIQUE,
  status TEXT NOT NULL DEFAULT 'active'
    CHECK (status IN ('active','transferred','withdrawn','graduated','suspended')),
  enrollment_date TEXT NOT NULL,
  enrollment_type TEXT NOT NULL DEFAULT 'regular',
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
  updated_at TEXT NOT NULL,
  FOREIGN KEY (period_id) REFERENCES academic_periods(id),
  FOREIGN KEY (student_id) REFERENCES users(id),
  FOREIGN KEY (class_id) REFERENCES classes(id)
);

CREATE INDEX IF NOT EXISTS idx_enr_org_period ON enrollments(org_id, period_id);
CREATE INDEX IF NOT EXISTS idx_enr_student ON enrollments(student_id);
CREATE INDEX IF NOT EXISTS idx_enr_class ON enrollments(class_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_enr_unique_active
  ON enrollments(student_id, period_id)
  WHERE status != 'withdrawn';

-- â”€â”€ Promotion Jobs â”€â”€
CREATE TABLE IF NOT EXISTS promotion_jobs (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  source_period_id TEXT NOT NULL,
  target_period_id TEXT,
  status TEXT NOT NULL DEFAULT 'pending'
    CHECK (status IN ('pending','validating','dry_run','confirmed','executing','completed','failed','rolled_back')),
  initiated_by TEXT NOT NULL,
  initiated_by_name TEXT,
  config TEXT NOT NULL,
  dry_run_result TEXT,
  dry_run_at TEXT,
  total_students INTEGER,
  promoted_count INTEGER,
  retained_count INTEGER,
  graduated_count INTEGER,
  withdrawn_count INTEGER,
  excluded_count INTEGER,
  error_log TEXT,
  rollback_available INTEGER NOT NULL DEFAULT 1,
  started_at TEXT,
  completed_at TEXT,
  rolled_back_at TEXT,
  rolled_back_by TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (source_period_id) REFERENCES academic_years(id),
  FOREIGN KEY (target_period_id) REFERENCES academic_years(id)
);

CREATE INDEX IF NOT EXISTS idx_pj_org ON promotion_jobs(org_id);
CREATE INDEX IF NOT EXISTS idx_pj_status ON promotion_jobs(status);

-- â”€â”€ Promotion Details â”€â”€
CREATE TABLE IF NOT EXISTS promotion_details (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  job_id TEXT NOT NULL,
  student_id TEXT NOT NULL,
  student_name TEXT NOT NULL,
  student_name_ar TEXT,
  student_number TEXT,
  source_enrollment_id TEXT NOT NULL,
  source_class_id TEXT NOT NULL,
  source_class_name TEXT,
  source_level TEXT NOT NULL,
  source_division TEXT,
  target_enrollment_id TEXT,
  target_class_id TEXT,
  target_class_name TEXT,
  target_level TEXT,
  decision TEXT NOT NULL
    CHECK (decision IN ('promoted','retained','graduated','excluded')),
  reason TEXT NOT NULL,
  final_score INTEGER,
  attendance_rate REAL,
  manual_override INTEGER NOT NULL DEFAULT 0,
  override_by TEXT,
  override_reason TEXT,
  original_decision TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (job_id) REFERENCES promotion_jobs(id)
);

CREATE INDEX IF NOT EXISTS idx_pd_job ON promotion_details(job_id);
CREATE INDEX IF NOT EXISTS idx_pd_student ON promotion_details(student_id);

-- â”€â”€ Academic Override Requests â”€â”€
CREATE TABLE IF NOT EXISTS academic_override_requests (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  override_type TEXT NOT NULL,
  risk_level TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  field_name TEXT,
  old_value TEXT,
  new_value TEXT,
  reason TEXT NOT NULL,
  requested_by TEXT NOT NULL,
  requested_by_name TEXT,
  requested_at TEXT NOT NULL,
  status TEXT NOT NULL,
  current_approver_role TEXT,
  approval_chain TEXT NOT NULL,
  resolved_by TEXT,
  resolved_by_name TEXT,
  resolved_at TEXT,
  resolution_notes TEXT,
  expires_at TEXT,
  metadata TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Grades â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS grades (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  assessment_id TEXT,
  class_subject_id TEXT,
  subject_name TEXT NOT NULL,
  subject_name_ar TEXT,
  score REAL NOT NULL DEFAULT 0,
  max_score REAL NOT NULL DEFAULT 100,
  weight REAL NOT NULL DEFAULT 1.0,
  grade_letter TEXT,
  grade_points REAL,
  exam_type TEXT NOT NULL DEFAULT 'assignment',
  semester TEXT NOT NULL DEFAULT '',
  notes TEXT,
  graded_by TEXT NOT NULL,
  graded_at TEXT NOT NULL,
  is_final INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (enrollment_id) REFERENCES enrollments(id),
  FOREIGN KEY (org_id) REFERENCES organizations(id),
  FOREIGN KEY (period_id) REFERENCES academic_periods(id)
);

CREATE INDEX IF NOT EXISTS idx_grades_enrollment ON grades(enrollment_id);
CREATE INDEX IF NOT EXISTS idx_grades_assessment ON grades(assessment_id);
CREATE INDEX IF NOT EXISTS idx_grades_org_period ON grades(org_id, period_id);
CREATE INDEX IF NOT EXISTS idx_grades_class_subject ON grades(class_subject_id);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Notifications â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS notifications (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  title TEXT NOT NULL,
  title_ar TEXT,
  body TEXT,
  body_ar TEXT,
  notification_type TEXT NOT NULL,
  entity_type TEXT,
  entity_id TEXT,
  is_read INTEGER NOT NULL DEFAULT 0,
  read_at TEXT,
  created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_notif_user ON notifications(user_id, is_read);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Audit Log â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS audit_log (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  user_id TEXT,
  action TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  changes TEXT,
  ip_address TEXT,
  user_agent TEXT,
  actor_type TEXT DEFAULT 'user',
  actor_session_id TEXT,
  correlation_id TEXT,
  created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_org ON audit_log(org_id, created_at);
CREATE INDEX IF NOT EXISTS idx_audit_entity ON audit_log(entity_type, entity_id);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ User Sessions â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS user_sessions (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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
  is_current INTEGER NOT NULL DEFAULT 0,
  is_trusted INTEGER NOT NULL DEFAULT 0,
  mfa_verified INTEGER NOT NULL DEFAULT 0,
  expires_at TEXT NOT NULL,
  refresh_expires_at TEXT,
  last_active_at TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id)
);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ IAM & Auth â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS user_accounts (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  uploaded_by TEXT,
  entity_type TEXT,
  entity_id TEXT,
  name TEXT NOT NULL,
  description TEXT,
  category TEXT NOT NULL,
  visibility TEXT NOT NULL,
  size INTEGER,
  mime_type TEXT,
  extension TEXT,
  storage_provider TEXT NOT NULL,
  path TEXT NOT NULL,
  url TEXT,
  deleted_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subjects (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
  class_id TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
  updated_at TEXT NOT NULL,
  FOREIGN KEY (period_id) REFERENCES academic_periods(id)
);

CREATE TABLE IF NOT EXISTS student_guardians (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Enterprise Assessment Engine â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS assessment_schemes (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assessment_scheme_components (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  scheme_id TEXT NOT NULL,
  name TEXT NOT NULL,
  component_type TEXT NOT NULL,
  weight REAL NOT NULL,
  max_score REAL NOT NULL,
  passing_score REAL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (scheme_id) REFERENCES assessment_schemes(id)
);

CREATE TABLE IF NOT EXISTS grade_scales (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  "type" TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grade_scale_items (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  scale_id TEXT NOT NULL,
  grade_label TEXT NOT NULL,
  min_score REAL NOT NULL,
  max_score REAL NOT NULL,
  gpa_points REAL,
  is_passing INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (scale_id) REFERENCES grade_scales(id)
);

CREATE TABLE IF NOT EXISTS calculation_policies (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  target_period_id TEXT NOT NULL,
  name TEXT NOT NULL,
  version INTEGER NOT NULL DEFAULT 1,
  rule_type TEXT NOT NULL,
  formula_expression TEXT NOT NULL,
  rounding_policy TEXT NOT NULL DEFAULT 'round_half_up',
  valid_from TEXT,
  valid_until TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS student_assessment_snapshots (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  student_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  class_subject_id TEXT NOT NULL,
  total_score REAL,
  grade_label TEXT,
  gpa_points REAL,
  is_passing INTEGER,
  rank_in_class INTEGER,
  rank_in_level INTEGER,
  calculation_version INTEGER NOT NULL,
  status TEXT NOT NULL DEFAULT 'draft',
  approved_at TEXT,
  approved_by TEXT,
  metadata TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

-- ── Attendance (per-record, used by ops/attendance) ──
CREATE TABLE IF NOT EXISTS attendance (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
  is_excused INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  FOREIGN KEY (enrollment_id) REFERENCES enrollments(id),
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE INDEX IF NOT EXISTS idx_attendance_enr_date ON attendance(enrollment_id, date);
CREATE INDEX IF NOT EXISTS idx_attendance_class_date ON attendance(class_id, date);

-- ── Attendance Daily (aggregate, used by attendance_daily entity) ──
CREATE TABLE IF NOT EXISTS attendance_daily (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
  party_id TEXT REFERENCES parties(id),
  enrollment_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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

-- â”€â”€ Fiscal Years & Periods â”€â”€
CREATE TABLE IF NOT EXISTS fiscal_years (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  name TEXT NOT NULL,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'open'
    CHECK (status IN ('open','closed','archived')),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE TABLE IF NOT EXISTS fiscal_periods (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  fiscal_period_id TEXT NOT NULL,
  name TEXT NOT NULL,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'open'
    CHECK (status IN ('open','closed','archived')),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (org_id) REFERENCES organizations(id),
  FOREIGN KEY (fiscal_period_id) REFERENCES fiscal_years(id)
);

-- â”€â”€ Currencies â”€â”€
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

-- â”€â”€ Accounts (Chart of Accounts) â”€â”€
CREATE TABLE IF NOT EXISTS accounts (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  code TEXT NOT NULL,
  name TEXT NOT NULL,
  name_ar TEXT,
  "type" TEXT NOT NULL
    CHECK ("type" IN ('asset','liability','equity','revenue','expense')),
  sub_type TEXT,
  parent_id TEXT,
  path TEXT NOT NULL,
  depth INTEGER NOT NULL,
  currency TEXT NOT NULL,
  -- We store CurrencyBalance as a JSON string: {"USD": 100.0, "SAR": 375.0}
  balances JSON NOT NULL DEFAULT '{}',
  entity_type TEXT,
  entity_id TEXT,
  is_control_account INTEGER NOT NULL DEFAULT 0,
  is_system_account INTEGER NOT NULL DEFAULT 0,
  is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_accounts_entity ON accounts(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_accounts_parent ON accounts(parent_id);
CREATE INDEX IF NOT EXISTS idx_accounts_org_code ON accounts(org_id, code);

-- â”€â”€ Journal Entries (Source of Truth for all financial movements) â”€â”€
CREATE TABLE IF NOT EXISTS journal_entries (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Fee Templates â”€â”€
CREATE TABLE IF NOT EXISTS fee_templates (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Discount Templates â”€â”€
CREATE TABLE IF NOT EXISTS discount_templates (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Student Fees (Invoice only â€” balance comes from ledger_accounts) â”€â”€
CREATE TABLE IF NOT EXISTS student_fees (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  enrollment_id TEXT NOT NULL,
  fee_template_id TEXT NOT NULL,
  original_amount REAL NOT NULL,
  discount_amount REAL NOT NULL DEFAULT 0,
  net_amount REAL NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  due_date TEXT NOT NULL,
  discount_details TEXT,
  parent_fee_id TEXT,
  installment_number INTEGER,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_sf_enrollment ON student_fees(enrollment_id);

-- â”€â”€ Payments (Multi-currency, supports students + employees) â”€â”€
CREATE TABLE IF NOT EXISTS payments (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Treasuries â”€â”€
CREATE TABLE IF NOT EXISTS treasuries (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Treasury Transactions â”€â”€
CREATE TABLE IF NOT EXISTS treasury_transactions (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- system_log table REMOVED (audit finding A-07):
-- No code references this table. The application uses:
--   • `tracing` crate for structured logging
--   • `domain_events` table for business event sourcing
--   • `audit_log` + `audit_events` tables for user action auditing


CREATE TABLE IF NOT EXISTS data_transfers (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
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
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  teacher_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  day_of_week INTEGER NOT NULL,
  time_slot_id TEXT NOT NULL,
  is_available INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS solve_jobs (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Examination Management System â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

-- â”€â”€ Student Ordering (Centralized Ordering Engine) â”€â”€
CREATE TABLE IF NOT EXISTS student_ordering (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
CREATE INDEX IF NOT EXISTS idx_so_sort_key ON student_ordering(org_id, period_id, sort_key);
CREATE INDEX IF NOT EXISTS idx_so_level ON student_ordering(org_id, period_id, level, order_in_level);

-- â”€â”€ Exam Sessions (Examination Cycles) â”€â”€
CREATE TABLE IF NOT EXISTS exam_sessions (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
CREATE INDEX IF NOT EXISTS idx_es_org_period ON exam_sessions(org_id, period_id);
CREATE INDEX IF NOT EXISTS idx_es_status ON exam_sessions(org_id, status);

-- â”€â”€ Exam Subjects â”€â”€
CREATE TABLE IF NOT EXISTS exam_subjects (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Exam Schedules â”€â”€
CREATE TABLE IF NOT EXISTS exam_schedules (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Seating Numbers â”€â”€
CREATE TABLE IF NOT EXISTS seating_numbers (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Secret Numbers â”€â”€
CREATE TABLE IF NOT EXISTS secret_numbers (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Exam Halls â”€â”€
CREATE TABLE IF NOT EXISTS exam_halls (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Exam Hall Allocations â”€â”€
CREATE TABLE IF NOT EXISTS exam_hall_allocations (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Exam Proctors â”€â”€
CREATE TABLE IF NOT EXISTS exam_proctors (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Teacher Exam Availability â”€â”€
CREATE TABLE IF NOT EXISTS teacher_exam_availability (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
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

-- â”€â”€ Search Optimizations for Students & Guardians â”€â”€
CREATE INDEX IF NOT EXISTS idx_users_org_role ON users(org_id, primary_role) WHERE is_deleted = 0;
CREATE INDEX IF NOT EXISTS idx_users_national_id ON users(national_id);
CREATE INDEX IF NOT EXISTS idx_users_phone ON users(phone);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_student_guardians_student ON student_guardians(student_id);
CREATE INDEX IF NOT EXISTS idx_student_guardians_guardian ON student_guardians(guardian_id);
-- â”€â”€ ai_insights â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
CREATE TABLE IF NOT EXISTS ai_insights (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  insight_type TEXT NOT NULL,
  entity_type TEXT,
  entity_id TEXT,
  scope TEXT NOT NULL DEFAULT 'organization',
  title TEXT NOT NULL,
  title_ar TEXT,
  summary TEXT NOT NULL,
  summary_ar TEXT,
  details TEXT,
  severity TEXT NOT NULL DEFAULT 'info'
    CHECK (severity IN ('info','suggestion','warning','alert','critical')),
  confidence REAL NOT NULL,
  impact_score REAL,
  suggested_action TEXT,
  suggested_params TEXT,
  status TEXT NOT NULL DEFAULT 'new'
    CHECK (status IN ('new','seen','acknowledged','acted_upon','dismissed','expired')),
  acknowledged_by TEXT,
  acknowledged_at TEXT,
  generated_by TEXT NOT NULL,
  valid_from TEXT NOT NULL,
  valid_until TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_ai_org_status ON ai_insights(org_id, status);
CREATE INDEX IF NOT EXISTS idx_ai_entity ON ai_insights(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_ai_severity ON ai_insights(severity)
  WHERE status = 'new';
-- â”€â”€ autopilot_tasks â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
CREATE TABLE IF NOT EXISTS autopilot_tasks (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  calendar_event_id TEXT,
  task_type TEXT NOT NULL,
  scheduled_at TEXT NOT NULL,
  deadline_at TEXT,
  target_entity_type TEXT,
  target_entity_id TEXT,
  params TEXT,
  prerequisites TEXT,
  status TEXT NOT NULL DEFAULT 'pending'
    CHECK (status IN ('pending','ready','executing','completed',
                      'failed','skipped','requires_approval','approved','cancelled')),
  execution_mode TEXT NOT NULL,
  attempts INTEGER NOT NULL DEFAULT 0,
  last_attempt_at TEXT,
  last_error TEXT,
  suggested_at TEXT,
  approved_by TEXT,
  approved_at TEXT,
  rejection_reason TEXT,
  completed_at TEXT,
  result TEXT,
  idempotency_key TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_at_org_status ON autopilot_tasks(org_id, status);
CREATE INDEX IF NOT EXISTS idx_at_scheduled ON autopilot_tasks(scheduled_at)
  WHERE status IN ('pending','ready');

-- â”€â”€ ai_agent_sessions â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
CREATE TABLE IF NOT EXISTS ai_agent_sessions (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  agent_type TEXT NOT NULL
    CHECK (agent_type IN ('assistant','autopilot','analyst','guardian','external')),
  agent_version TEXT,
  acting_as_user_id TEXT NOT NULL,
  acting_as_role TEXT NOT NULL,
  conversation_id TEXT,
  status TEXT NOT NULL DEFAULT 'active'
    CHECK (status IN ('active','completed','suspended','revoked')),
  max_risk_level TEXT NOT NULL DEFAULT 'low',
  requires_approval_above TEXT NOT NULL DEFAULT 'medium',
  actions_executed INTEGER NOT NULL DEFAULT 0,
  actions_suggested INTEGER NOT NULL DEFAULT 0,
  started_at TEXT NOT NULL,
  last_active_at TEXT NOT NULL,
  ended_at TEXT,
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_aas_org_status ON ai_agent_sessions(org_id, status);

-- â”€â”€ ai_action_log â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
CREATE TABLE IF NOT EXISTS ai_action_log (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  session_id TEXT NOT NULL,
  correlation_id TEXT,
  action_code TEXT NOT NULL,
  input_data TEXT,
  output_data TEXT,
  reasoning TEXT NOT NULL,
  confidence REAL,
  status TEXT NOT NULL
    CHECK (status IN ('pending','approved','executed','rejected','failed','rolled_back')),
  risk_level TEXT NOT NULL,
  approval_required INTEGER NOT NULL DEFAULT 0,
  approved_by TEXT,
  approved_at TEXT,
  rejection_reason TEXT,
  affected_entities TEXT,
  idempotency_key TEXT UNIQUE,
  duration_ms INTEGER,
  error TEXT,
  is_reversible INTEGER NOT NULL DEFAULT 0,
  rollback_data TEXT,
  reversed_at TEXT,
  reversed_by TEXT,
  executed_at TEXT NOT NULL,
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_aal_session ON ai_action_log(session_id);
CREATE INDEX IF NOT EXISTS idx_aal_org_date ON ai_action_log(org_id, executed_at);
CREATE INDEX IF NOT EXISTS idx_aal_pending ON ai_action_log(status)
  WHERE status = 'pending';

-- â”€â”€ data_quality_findings â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
CREATE TABLE IF NOT EXISTS data_quality_findings (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  rule_code TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  field_name TEXT,
  severity TEXT NOT NULL DEFAULT 'warning',
  current_value TEXT,
  expected_description TEXT,
  suggested_fix TEXT,
  status TEXT NOT NULL DEFAULT 'open'
    CHECK (status IN ('open','auto_fixed','manually_fixed','ignored','false_positive')),
  fixed_by TEXT,
  fixed_at TEXT,
  detected_at TEXT NOT NULL,
  created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_dqf_org_status ON data_quality_findings(org_id, status);
CREATE INDEX IF NOT EXISTS idx_dqf_entity ON data_quality_findings(entity_type, entity_id);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- Performance Indexes for Common Queries
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE INDEX IF NOT EXISTS idx_enrollment_org_period_class
    ON enrollments(org_id, period_id, class_id);

CREATE INDEX IF NOT EXISTS idx_attendance_daily_enrollment_period_date
    ON attendance_daily(enrollment_id, period_id, date);

-- (Duplicate index removed — idx_at_org_status already covers this)

CREATE INDEX IF NOT EXISTS idx_classes_org_period_covering
    ON classes(org_id, period_id, name, level, capacity);

-- ── Additional Performance Indexes (Audit P2) ──

CREATE INDEX IF NOT EXISTS idx_sas_student_period
    ON student_assessment_snapshots(student_id, period_id);

CREATE INDEX IF NOT EXISTS idx_sas_org_period_status
    ON student_assessment_snapshots(org_id, period_id, status);

CREATE INDEX IF NOT EXISTS idx_class_subjects_class_period
    ON class_subjects(class_id, period_id);

CREATE INDEX IF NOT EXISTS idx_users_party_id
    ON users(party_id) WHERE party_id IS NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_sg_unique_pair
    ON student_guardians(student_id, guardian_id);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ Domain Events â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS domain_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    aggregate_type  TEXT NOT NULL,
    aggregate_id    TEXT NOT NULL,
    event_type      TEXT NOT NULL,
    payload         TEXT NOT NULL,
    actor_id        TEXT NOT NULL,
    actor_type      TEXT NOT NULL,
    correlation_id  TEXT DEFAULT '',
    version         INTEGER NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_de_aggregate ON domain_events(aggregate_type, aggregate_id, version);
CREATE INDEX IF NOT EXISTS idx_de_type_time ON domain_events(event_type, created_at);
CREATE INDEX IF NOT EXISTS idx_de_correlation ON domain_events(correlation_id) WHERE correlation_id != '';
CREATE INDEX IF NOT EXISTS idx_de_actor ON domain_events(actor_id, created_at);

-- â”€â”€ Process State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
CREATE TABLE IF NOT EXISTS process_log (
    id              TEXT PRIMARY KEY,
    process_type    TEXT NOT NULL,
    org_id          TEXT NOT NULL,
    status          TEXT NOT NULL,
    steps_json      TEXT NOT NULL,
    started_at      TEXT NOT NULL,
    completed_at    TEXT,
    error           TEXT
);

CREATE INDEX IF NOT EXISTS idx_pl_org ON process_log(org_id, process_type, started_at);

-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
-- â”€â”€ CQRS DocForge Projections & Schema Registry â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
-- â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

CREATE TABLE IF NOT EXISTS docforge_projections (
    id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
    org_id TEXT NOT NULL,
    entity_type TEXT NOT NULL, -- e.g., 'student_report_card', 'invoice'
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL, -- JSONB payload for template rendering
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_docforge_proj_org_entity ON docforge_projections(org_id, entity_type, entity_id);

CREATE TABLE IF NOT EXISTS docforge_schema_registry (
    id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
    org_id TEXT NOT NULL,
    provider_name TEXT NOT NULL, -- e.g. 'assessment_engine', 'finance'
    schema_payload TEXT NOT NULL, -- JSON Schema
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_docforge_schema_org ON docforge_schema_registry(org_id, provider_name);

-- ??????????????????????????????????????????????????????????????????????
-- ?? Generic Audit Events Table ??
-- ??????????????????????????????????????????????????????????????????????

CREATE TABLE IF NOT EXISTS audit_events (
    id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
    org_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    action TEXT NOT NULL,
    payload TEXT NOT NULL, -- JSONB payload of the state or diff
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_events_entity ON audit_events(org_id, entity_type, entity_id);


-- ══════════════════════════════════════════════════════════════════
-- ── Behavior Records ──
-- ══════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS behavior_records (
  id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
  org_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
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
  parent_notified INTEGER NOT NULL DEFAULT 0,
  parent_notified_at TEXT,
  parent_response TEXT,
  evidence_urls TEXT,
  status TEXT NOT NULL DEFAULT 'reported',
  resolution_notes TEXT,
  resolved_at TEXT,
  resolved_by TEXT,
  is_private INTEGER NOT NULL DEFAULT 0,
  ai_source TEXT,
  ai_confidence REAL,
  ai_model_version TEXT,
  camera_id TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY (enrollment_id) REFERENCES enrollments(id),
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE INDEX IF NOT EXISTS idx_behavior_enrollment ON behavior_records(enrollment_id);
CREATE INDEX IF NOT EXISTS idx_behavior_org_period ON behavior_records(org_id, period_id);

-- ══════════════════════════════════════════════════════════════════
-- ── Current Students (materialized view for fast lookup) ──
-- ══════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS current_students (
  org_id TEXT NOT NULL,
  student_id TEXT NOT NULL,
  period_id TEXT NOT NULL,
  class_id TEXT NOT NULL,
  is_active INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (org_id, student_id, period_id),
  FOREIGN KEY (student_id) REFERENCES users(id),
  FOREIGN KEY (class_id) REFERENCES classes(id),
  FOREIGN KEY (org_id) REFERENCES organizations(id)
);

CREATE INDEX IF NOT EXISTS idx_cs_class ON current_students(class_id);
CREATE INDEX IF NOT EXISTS idx_cs_period ON current_students(period_id, is_active);

-- ── current_students Triggers ──
-- Derives is_active from enrollment status (active = 1, others = 0)

CREATE TRIGGER IF NOT EXISTS enrollments_after_insert
AFTER INSERT ON enrollments
BEGIN
    INSERT INTO current_students (org_id, student_id, period_id, class_id, is_active, updated_at)
    VALUES (
        NEW.org_id, NEW.student_id, NEW.period_id, NEW.class_id,
        CASE WHEN NEW.status = 'active' THEN 1 ELSE 0 END,
        NEW.updated_at
    )
    ON CONFLICT (org_id, student_id, period_id) DO UPDATE SET
        class_id = excluded.class_id,
        is_active = excluded.is_active,
        updated_at = excluded.updated_at;
END;

CREATE TRIGGER IF NOT EXISTS enrollments_after_update
AFTER UPDATE ON enrollments
BEGIN
    UPDATE current_students
    SET class_id = NEW.class_id,
        is_active = CASE WHEN NEW.status = 'active' THEN 1 ELSE 0 END,
        updated_at = NEW.updated_at
    WHERE org_id = NEW.org_id AND student_id = NEW.student_id AND period_id = NEW.period_id;
END;

CREATE TRIGGER IF NOT EXISTS enrollments_after_delete
AFTER DELETE ON enrollments
BEGIN
    DELETE FROM current_students
    WHERE org_id = OLD.org_id AND student_id = OLD.student_id AND period_id = OLD.period_id;
END;


-- ??????????????????????????????????????????????????????????????????????
-- ?? Outbox Pattern ??
-- ??????????????????????????????????????????????????????????????????????

CREATE TABLE IF NOT EXISTS outbox_events (
    id TEXT PRIMARY KEY,
  party_id TEXT REFERENCES parties(id),
    org_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_outbox_events_created ON outbox_events(created_at);


