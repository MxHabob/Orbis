use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub tenant_id: Uuid,
    pub actor_id: Uuid,
    pub request_id: String,
}
