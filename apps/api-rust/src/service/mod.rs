use std::sync::Arc;
use crate::config::Config;
use crate::repository::Repository;

pub mod auth;
pub mod authz;
pub mod class;
pub mod behavior;
pub mod notification;
pub mod organization;
pub mod user;
pub mod academic_cycle;
pub mod org_unit;
pub mod enrollment;
pub mod grade;
pub mod attendance_svc;
pub mod dashboard;
pub mod import_export;

#[derive(Clone)]
pub struct Service {
    pub repo: Arc<Repository>,
    pub cfg: Arc<Config>,
    pub authz: Arc<authz::AuthzService>,
}

impl Service {
    pub fn new(repo: Repository, cfg: Config) -> Self {
        Self {
            repo: Arc::new(repo),
            cfg: Arc::new(cfg),
            authz: Arc::new(authz::AuthzService::new()),
        }
    }
}
