use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Actor {
    /// A regular user executing a request. Binds to IAM role permissions.
    User(Uuid),
    /// Background AI Agent acting on behalf of a user or system.
    Agent(Uuid),
    /// The AutoPilot daemon acting on its own (System-wide privileges).
    Autopilot,
    /// System-level operations (e.g., migrations, internal triggers).
    System,
}

impl Actor {
    pub fn is_system(&self) -> bool {
        matches!(self, Actor::System | Actor::Autopilot)
    }

    pub fn id(&self) -> Option<Uuid> {
        match self {
            Actor::User(id) | Actor::Agent(id) => Some(*id),
            Actor::Autopilot | Actor::System => None,
        }
    }
}
