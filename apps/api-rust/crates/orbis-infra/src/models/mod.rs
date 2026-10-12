// === Academic Structure (Unified) ===
pub mod academic_calendar;
pub mod academic_period;
pub mod period_type;
pub mod section;
pub mod section_subject;
pub mod subject;
pub mod prerequisite;

// === Organization & IAM ===
pub mod organization;
pub mod org_unit;
pub mod person;
pub mod user;
pub mod user_session;
pub mod role;
pub mod role_permission;
pub mod user_role_assignment;
pub mod user_identity;
pub mod mfa_factor;
pub mod mfa_recovery_code;
pub mod user_device;

// === Employee & Groups ===
pub mod employee;
pub mod group;
pub mod user_group_membership;
pub mod group_permission;

// === Student Lifecycle ===
pub mod student;
pub mod enrollment;
pub mod student_guardian;
pub mod progression_job;
pub mod progression_detail;
pub mod override_request;

// === Assessment & Grading ===
pub mod assessment_scheme;
pub mod grade;
pub mod gpa_record;
pub mod attendance;
pub mod behavior_record;

// === Exams ===
pub mod exam_session;
pub mod exam_schedule;
pub mod exam_proctor;
pub mod seating_number;
pub mod secret_number;

// === Finance ===
pub mod account;
pub mod currency;
pub mod exchange_rate;
pub mod journal_entry;
pub mod journal_entry_line;
pub mod fee_template;
pub mod discount_template;
pub mod student_fee;
pub mod payment;

// === Documents & Storage ===
pub mod document;
pub mod document_requirement;
pub mod folder;
pub mod custom_field_definition;

// === Integration & Automation ===
pub mod integration;
pub mod workflow;
pub mod autopilot_task;

// === Reporting ===
pub mod report;

// === Scheduling & Rooms ===
pub mod room;
pub mod time_slot;
pub mod scheduled_lesson;

// === Infrastructure ===
pub mod sequence_counter;
