use crate::ports::repositories::IntegrationRepository;
use crate::context::RequestContext;
use orbis_domain::entities::integration::{IntegrationConfig, IntegrationSecret};
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use std::collections::HashMap;

pub struct ConnectAppUseCase {
    pub integration_repo: Arc<dyn IntegrationRepository>,
}

impl ConnectAppUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        provider_name: String,
        category: String,
        env: String,
        settings: serde_json::Value,
        secrets: HashMap<String, String>,
    ) -> Result<IntegrationConfig, DomainError> {
        let config_id = Uuid::new_v4();
        
        let config = IntegrationConfig {
            id: config_id,
            org_id: _ctx.tenant_id,
            provider_name,
            category,
            is_enabled: true,
            env,
            settings: Some(settings),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // 1. Save Config
        self.integration_repo.save_config(&config).await?;

        // 2. Save Secrets (Iterate over hashmap)
        for (key_name, secret_value) in secrets {
            // In a real production system, this `secret_value` should be encrypted using AES-GCM
            // with a MASTER_KEY before storing it in the database. 
            // For now, we store it directly as it simulates the structure.
            
            let secret = IntegrationSecret {
                id: Uuid::new_v4(),
                config_id,
                secret_key: secret_value, // TODO: encrypt
                key_name,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            self.integration_repo.save_secret(&secret).await?;
        }

        Ok(config)
    }
}
