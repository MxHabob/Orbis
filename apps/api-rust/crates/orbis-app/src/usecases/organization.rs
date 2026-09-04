use orbis_domain::entities::organization::Organization;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::OrganizationRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateOrganizationUseCase {
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateOrganizationUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        name: String,
        slug: String,
        timezone: String,
        locale: String,
    ) -> Result<Organization, DomainError> {
        let existing = self.org_repo.find_by_slug(&slug).await?;
        if existing.is_some() {
            return Err(DomainError::Conflict(format!("Slug '{}' already exists", slug)));
        }

        let org = Organization::new(name, slug, timezone, locale)?;
        
        self.org_repo.save(&org).await?;
        
        let now = chrono::Utc::now();
        let event = DomainEvent::OrganizationCreated {
            org_id: org.id,
            name: org.name.clone(),
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;
        
        Ok(org)
    }
}

pub struct UpdateOrganizationUseCase {
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl UpdateOrganizationUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: Option<String>,
        timezone: Option<String>,
        locale: Option<String>,
        primary_color: Option<String>,
    ) -> Result<Organization, DomainError> {
        let mut org = self.org_repo.find_by_id(org_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Organization {} not found", org_id)))?;

        if let Some(n) = name { org.name = n; }
        if let Some(tz) = timezone { org.timezone = tz; }
        if let Some(loc) = locale { org.locale = loc; }
        if let Some(pc) = primary_color { org.primary_color = Some(pc); }

        org.updated_at = chrono::Utc::now();

        self.org_repo.save(&org).await?;

        let event = DomainEvent::OrganizationUpdated {
            org_id: org.id,
            timestamp: org.updated_at,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(org)
    }
}
