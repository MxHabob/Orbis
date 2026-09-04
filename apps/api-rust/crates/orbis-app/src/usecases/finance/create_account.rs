use crate::ports::repositories::AccountRepository;
use crate::context::RequestContext;
use orbis_domain::entities::account::Account;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;
use std::sync::Arc;

pub struct CreateAccountUseCase {
    pub account_repo: Arc<dyn AccountRepository>,
}

impl CreateAccountUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        code: String,
        name: String,
        account_type: String,
        parent_id: Option<Uuid>,
        currency: String,
    ) -> Result<Account, DomainError> {
        let account = Account {
            id: Uuid::new_v4(),
            org_id,
            code,
            name,
            account_type,
            parent_id,
            currency,
            is_active: true,
            balances: json!({"current": 0.0}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.account_repo.save(&account).await?;

        Ok(account)
    }
}
