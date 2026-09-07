use orbis_domain::iam::actor::Actor;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub tenant_id: Uuid,
    pub actor: Actor,
    pub request_id: String,
}

impl RequestContext {
    pub fn user_id(&self) -> Option<Uuid> {
        self.actor.id()
    }
}

