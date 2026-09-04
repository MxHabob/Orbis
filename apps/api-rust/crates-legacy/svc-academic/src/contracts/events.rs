//! Academic domain events.

pub mod event_types {
    pub const CLASS_CREATED: &str = "class.created";
    pub const CLASS_UPDATED: &str = "class.updated";
    pub const CLASS_DELETED: &str = "class.deleted";
    pub const SUBJECT_CREATED: &str = "subject.created";
    pub const ORG_UNIT_CREATED: &str = "org_unit.created";
    pub const ENROLLMENT_CREATED: &str = "enrollment.created";
    pub const ENROLLMENT_WITHDRAWN: &str = "enrollment.withdrawn";
    pub const ENROLLMENT_TRANSFERRED: &str = "enrollment.transferred";
    pub const ENROLLMENT_PROMOTED: &str = "enrollment.promoted";
}
