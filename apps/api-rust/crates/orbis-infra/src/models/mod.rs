// === Academic Structure (Unified) ===
pub mod academic_calendar;
pub mod academic_period;
pub mod period_type;
pub mod prerequisite;
pub mod section;
pub mod section_subject;
pub mod subject;
pub use section as class;
pub use subject as course;

// === Organization & IAM ===
pub mod mfa_factor;
pub mod mfa_recovery_code;
pub mod org_unit;
pub mod organization;
pub mod person;
pub mod role;
pub mod role_permission;
pub mod user;
pub mod user_device;
pub mod user_identity;
pub mod user_role_assignment;
pub mod user_session;

// === Employee & Groups ===
pub mod employee;
pub mod group;
pub mod group_permission;
pub mod user_group_membership;

// === Student Lifecycle ===
pub mod enrollment;
pub mod override_request;
pub mod progression_detail;
pub mod progression_job;
pub mod student;
pub mod student_guardian;

// === Assessment & Grading ===
pub mod assessment_scheme;
pub mod attendance;
pub mod behavior_record;
pub mod gpa_record;
pub mod grade;

// === Exams ===
pub mod exam_proctor;
pub mod exam_schedule;
pub mod exam_session;
pub mod seating_number;
pub mod secret_number;

// === Finance ===
pub mod account;
pub mod currency;
pub mod discount_template;
pub mod exchange_rate;
pub mod fee_template;
pub mod journal_entry;
pub mod journal_entry_line;
pub mod payment;
pub mod student_fee;

// === Documents & Storage ===
pub mod custom_field_definition;
pub mod document;
pub mod document_requirement;
pub mod folder;

// === Integration & Automation ===
pub mod autopilot_task;
pub mod integration;
pub mod workflow;

// === Reporting ===
pub mod report;

// === Scheduling & Rooms ===
pub mod room;
pub mod scheduled_lesson;
pub mod time_slot;

// === Infrastructure ===
pub mod sequence_counter;
