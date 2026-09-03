//! IAM domain events.

pub mod event_types {
    pub const USER_CREATED: &str = "user.created";
    pub const USER_UPDATED: &str = "user.updated";
    pub const USER_DELETED: &str = "user.deleted";
    pub const USER_LOGIN: &str = "user.login";
    pub const USER_LOGOUT: &str = "user.logout";
    pub const ROLE_CHANGED: &str = "user.role_changed";
    pub const PASSWORD_CHANGED: &str = "user.password_changed";
}
