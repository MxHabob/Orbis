pub mod organization;
pub mod user;
pub mod user_session;
pub mod academic_cycle;
pub mod org_unit;
pub mod class;
pub mod enrollment;
pub mod behavior_record;
pub mod notification;
pub mod audit_log;
pub mod grade;
pub mod attendance;

// ── Newly Added Entities (v2 Schema Migration) ──
pub mod user_account;
pub mod user_role;
pub mod permission;
pub mod role_permission;
pub mod user_permission;
pub mod verification;
pub mod notification_preference;
pub mod document;
pub mod subject;
pub mod class_subject;
pub mod student_guardian;
pub mod time_slot;
pub mod scheduled_lesson;
pub mod solve_job;
pub mod room;
pub mod teacher_availability;
pub mod assessment;
pub mod grade_history;
pub mod attendance_daily;
pub mod attendance_summary;

// ── Finance Domain ──
pub mod currency;
pub mod ledger_account;
pub mod journal_entry;
pub mod fee_template;
pub mod discount_template;
pub mod student_fee;
pub mod payment;
pub mod treasury;
pub mod treasury_transaction;

// ── System & Reporting ──
pub mod system_log;
pub mod data_transfer;
pub mod report_definition;
pub mod document_template;

// ── Examination Management System ──
pub mod student_ordering;
pub mod exam_session;
pub mod exam_subject;
pub mod exam_schedule;
pub mod seating_number;
pub mod secret_number;
pub mod exam_hall;
pub mod exam_hall_allocation;
pub mod exam_proctor;
pub mod teacher_exam_availability;

// ── Academic Calendar & Policies ──
pub mod academic_calendar_event;
pub mod org_academic_policy;
pub mod promotion_policy;
pub mod academic_override_request;

// ── Academic Year Management & Promotion (Registry DB) ──
pub mod academic_year;
pub mod promotion_job;
pub mod promotion_detail;
pub mod student_registry_entry;
