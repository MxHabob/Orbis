use async_trait::async_trait;
use orbis_domain::errors::DomainError;

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate_access_token(&self, user_id: &str, org_id: &str, role: &str) -> Result<String, DomainError>;
    async fn generate_refresh_token(&self, user_id: &str, org_id: &str, role: &str) -> Result<String, DomainError>;
    async fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
}

pub struct TokenClaims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
}

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, DomainError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
}
