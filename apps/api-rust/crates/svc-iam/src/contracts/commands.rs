//! IAM write commands.

use serde::{Deserialize, Serialize};

/// Create a new user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCmd {
    pub org_id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub primary_role: String,
}

/// Update user profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserCmd {
    pub user_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_active: Option<bool>,
}

/// Change user role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeUserRoleCmd {
    pub user_id: String,
    pub new_role: String,
}

/// Login command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCmd {
    pub username: String,
    pub password: String,
    pub org_slug: String,
}
