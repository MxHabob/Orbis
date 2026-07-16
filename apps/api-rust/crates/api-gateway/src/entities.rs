//! Entity re-exports mapping modular domain model paths to the original gateway namespace.

pub mod user {
    pub use svc_iam::entities::user::*;
}

pub mod user_session {
    pub use svc_iam::entities::user_session::*;
}

pub mod audit_log {
    pub use svc_iam::entities::audit_log::*;
}

pub mod organization {
    pub use svc_organization::entities::organization::*;
}

pub mod academic_period {
    pub use svc_calendar::entities::academic_period::*;
}

pub mod academic_year {
    pub use svc_calendar::entities::academic_year::*;
}

pub mod academic_policy {
    pub use platform_policy::entities::academic_policy::*;
}

pub mod org_unit {
    pub use svc_organization::entities::org_unit::*;
}

pub mod class {
    pub use svc_academic::entities::class::*;
}

pub mod enrollment {
    pub use svc_academic::entities::enrollment::*;
}

pub mod student_guardian {
    pub use svc_academic::entities::student_guardian::*;
}

pub mod document {
    pub use svc_academic::entities::document::*;
}

pub mod subject {
    pub use svc_academic::entities::subject::*;
}

pub mod notification {
    pub use svc_academic::entities::notification::*;
}

pub mod time_slot {
    pub use svc_scheduling::entities::time_slot::*;
}

pub mod class_subject {
    pub use svc_academic::entities::class_subject::*;
}

pub mod scheduled_lesson {
    pub use svc_scheduling::entities::scheduled_lesson::*;
}

pub mod room {
    pub use svc_scheduling::entities::room::*;
}

pub mod solve_job {
    pub use svc_scheduling::entities::solve_job::*;
}


pub mod behavior_record {
    pub use svc_assessment::entities::behavior_record::*;
}

pub mod grade {
    pub use svc_assessment::entities::grade::*;
}

pub mod attendance {
    pub use svc_assessment::entities::attendance::*;
}

pub mod exam_session {
    pub use svc_exam::entities::exam_session::*;
}

pub mod exam_subject {
    pub use svc_exam::entities::exam_subject::*;
}

pub mod exam_schedule {
    pub use svc_exam::entities::exam_schedule::*;
}

pub mod seating_number {
    pub use svc_exam::entities::seating_number::*;
}

pub mod secret_number {
    pub use svc_exam::entities::secret_number::*;
}

pub mod exam_hall {
    pub use svc_exam::entities::exam_hall::*;
}

pub mod exam_hall_allocation {
    pub use svc_exam::entities::exam_hall_allocation::*;
}

pub mod exam_proctor {
    pub use svc_exam::entities::exam_proctor::*;
}

pub mod student_ordering {
    pub use svc_exam::entities::student_ordering::*;
}
