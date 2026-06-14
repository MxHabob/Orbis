use std::sync::Arc;
use crate::config::Config;
use crate::repository::Repository;

pub mod auth;
pub mod class;
pub mod behavior;
pub mod notification;

#[derive(Clone)]
pub struct Service {
    pub repo: Arc<Repository>,
    pub cfg: Arc<Config>,
}

impl Service {
    pub fn new(repo: Repository, cfg: Config) -> Self {
        Self {
            repo: Arc::new(repo),
            cfg: Arc::new(cfg),
        }
    }
}
