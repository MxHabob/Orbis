use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use orbis_domain::entities::user::{User, Role};
use orbis_domain::entities::person::Person;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;

use crate::context::RequestContext;
use crate::ports::repositories::{UserRepository, PersonRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::security::PasswordHasher;

pub struct RegisterUserUseCase {
    pub user_repo: Arc<dyn UserRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterUserUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        full_name: String,
        email: Option<String>,
        password_plain: Option<String>,
        role: Role,
        org_id: Option<Uuid>,
    ) -> Result<User, DomainError> {
        let actual_org_id = org_id.unwrap_or(ctx.tenant_id);

        if let Some(ref e) = email {
            if self.user_repo.find_by_email(e).await?.is_some() {
                return Err(DomainError::Conflict(format!("Email '{}' already registered", e)));
            }
        }

        let now = Utc::now();
        let person_id = Uuid::new_v4();

        let name_parts: Vec<&str> = full_name.split_whitespace().collect();
        let first_name = name_parts.first().unwrap_or(&"").to_string();
        let last_name = if name_parts.len() > 1 {
            name_parts[1..].join(" ")
        } else {
            first_name.clone()
        };

        let person = Person {
            id: person_id,
            party_id: None,
            org_id: Some(actual_org_id),
            first_name,
            last_name,
            full_name: Some(full_name.clone()),
            national_id: None,
            gender: None,
            date_of_birth: None,
            nationality: None,
            created_at: now,
            updated_at: now,
        };
        self.person_repo.save(&person).await?;

        let password_hash = match password_plain {
            Some(p) => Some(self.password_hasher.hash(&p).await?),
            None => None,
        };

        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            person_id,
            party_id: None,
            org_id: actual_org_id,
            email,
            email_verified: false,
            phone: None,
            phone_verified: false,
            password_hash,
            mfa_enabled: false,
            full_name,
            primary_role: role,
            status: "active".to_string(),
            login_count: 0,
            failed_logins: 0,
            locked_until: None,
            is_deleted: false,
            created_at: now,
            updated_at: now,
        };
        
        self.user_repo.save(&user).await?;

        let event = DomainEvent::UserCreated {
            user_id,
            person_id,
            org_id: actual_org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(user)
    }
}
