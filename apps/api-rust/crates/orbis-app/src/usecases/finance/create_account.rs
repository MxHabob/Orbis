use crate::ports::repositories::AccountRepository;
use crate::context::RequestContext;
use orbis_domain::entities::account::{Account, AccountType};
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
        name_ar: Option<String>,
        account_type: AccountType,
        sub_type: Option<String>,
        parent_id: Option<Uuid>,
        currency: String,
    ) -> Result<Account, DomainError> {
        let (path, depth) = if let Some(p_id) = parent_id {
            if let Some(parent) = self.account_repo.find_by_id(p_id).await? {
                (format!("{}/{}", parent.path, code), parent.depth + 1)
            } else {
                (format!("/{}", code), 1)
            }
        } else {
            (format!("/{}", code), 1)
        };

        let account = Account {
            id: Uuid::new_v4(),
            org_id,
            code,
            name,
            name_ar,
            account_type,
            sub_type,
            parent_id,
            path,
            depth,
            currency,
            entity_type: None,
            entity_id: None,
            is_control_account: false,
            is_system_account: false,
            is_active: true,
            balances: json!({"current": "0"}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.account_repo.save(&account).await?;

        Ok(account)
    }
}
