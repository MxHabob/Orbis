pub mod org_cmds;
pub mod iam_cmds;
pub mod calendar_cmds;
pub mod academic_cmds;
pub mod student_cmds;
pub mod scheduling_cmds;
pub mod assessment_cmds;
pub mod exam_cmds;
pub mod finance_cmds;

use tauri::generate_handler;

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    generate_handler![
        org_cmds::create_organization,
        org_cmds::create_org_unit,
        iam_cmds::register_user,
        iam_cmds::login_user,
        calendar_cmds::create_academic_year,
        calendar_cmds::create_academic_period,
        academic_cmds::create_subject,
        academic_cmds::create_class,
        academic_cmds::assign_class_subject,
        student_cmds::enroll_student,
        student_cmds::assign_guardian,
        scheduling_cmds::create_room,
        scheduling_cmds::create_time_slot,
        scheduling_cmds::schedule_lesson,
        assessment_cmds::create_assessment_scheme,
        assessment_cmds::record_grade,
        assessment_cmds::mark_attendance,
        exam_cmds::create_exam_session,
        exam_cmds::schedule_exam,
        exam_cmds::assign_seating_number,
        exam_cmds::assign_secret_number,
        exam_cmds::assign_proctor,
        finance_cmds::create_account,
        finance_cmds::create_fee_template,
        finance_cmds::generate_student_fee,
        finance_cmds::process_payment,
    ]
}
