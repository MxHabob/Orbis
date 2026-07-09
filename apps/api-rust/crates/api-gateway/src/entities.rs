//! Entity re-exports mapping modular domain model paths to the original gateway namespace.

pub mod user {
    pub use domain_iam::entities::user::*;
}

pub mod user_session {
    pub use domain_iam::entities::user_session::*;
}

pub mod audit_log {
    pub use domain_iam::entities::audit_log::*;
}

pub mod organization {
    pub use domain_lifecycle::entities::organization::*;
}

pub mod academic_cycle {
    pub use domain_lifecycle::entities::academic_cycle::*;
}

pub mod org_unit {
    pub use domain_academic::entities::org_unit::*;
}

pub mod class {
    pub use domain_academic::entities::class::*;
}

pub mod enrollment {
    pub use domain_academic::entities::enrollment::*;
}

pub mod student_guardian {
    pub use domain_academic::entities::student_guardian::*;
}

pub mod document {
    pub use domain_academic::entities::document::*;
}

pub mod subject {
    pub use domain_academic::entities::subject::*;
}

pub mod notification {
    pub use domain_academic::entities::notification::*;
}

pub mod time_slot {
    pub use domain_scheduling::entities::time_slot::*;
}

pub mod class_subject {
    pub use domain_academic::entities::class_subject::*;
}

pub mod scheduled_lesson {
    pub use domain_scheduling::entities::scheduled_lesson::*;
}

pub mod room {
    pub use domain_scheduling::entities::room::*;
}

pub mod solve_job {
    pub use domain_scheduling::entities::solve_job::*;
}

pub mod assessment {
    pub use domain_assessment::entities::assessment::*;
}

pub mod behavior_record {
    pub use domain_assessment::entities::behavior_record::*;
}

pub mod grade {
    pub use domain_assessment::entities::grade::*;
}

pub mod attendance {
    pub use domain_assessment::entities::attendance::*;
}

pub mod exam_session {
    pub use domain_exam::entities::exam_session::*;
}

pub mod exam_subject {
    pub use domain_exam::entities::exam_subject::*;
}

pub mod exam_schedule {
    pub use domain_exam::entities::exam_schedule::*;
}

pub mod seating_number {
    pub use domain_exam::entities::seating_number::*;
}

pub mod secret_number {
    pub use domain_exam::entities::secret_number::*;
}

pub mod exam_hall {
    pub use domain_exam::entities::exam_hall::*;
}

pub mod exam_hall_allocation {
    pub use domain_exam::entities::exam_hall_allocation::*;
}

pub mod exam_proctor {
    pub use domain_exam::entities::exam_proctor::*;
}

pub mod student_ordering {
    pub use domain_exam::entities::student_ordering::*;
}
