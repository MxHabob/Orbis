use async_trait::async_trait;
use orbis_domain::errors::DomainError;

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate_access_token(
        &self,
        user_id: &str,
        org_id: &str,
        role: &str,
    ) -> Result<String, DomainError>;
    async fn generate_refresh_token(
        &self,
        user_id: &str,
        org_id: &str,
        role: &str,
    ) -> Result<String, DomainError>;
    async fn generate_mfa_token(
        &self,
        user_id: &str,
        org_id: &str,
        role: &str,
    ) -> Result<String, DomainError>;
    async fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
    async fn validate_mfa_token(&self, token: &str) -> Result<TokenClaims, DomainError>;
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

#[async_trait]
pub trait TotpService: Send + Sync {
    fn generate_secret(&self) -> String;
    fn generate_qr_uri(&self, issuer: &str, account_name: &str, secret: &str) -> String;
    fn generate_recovery_codes(&self, count: usize) -> Vec<String>;
    fn encrypt_secret(&self, secret: &str) -> Result<String, DomainError>;
    fn decrypt_secret(&self, secret: &str) -> Result<String, DomainError>;
    fn verify_code(&self, secret: &str, code: &str) -> bool;
}

#[async_trait]
pub trait OidcService: Send + Sync {
    async fn verify_google_token(
        &self,
        id_token: &str,
    ) -> Result<orbis_domain::entities::identity::IdentityProfile, DomainError>;
}
