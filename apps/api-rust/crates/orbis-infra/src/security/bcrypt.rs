use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};
use orbis_app::ports::security::PasswordHasher;
use orbis_domain::errors::DomainError;

pub struct BcryptHasher;

#[async_trait]
impl PasswordHasher for BcryptHasher {
    async fn hash(&self, password: &str) -> Result<String, DomainError> {
        let password = password.to_string();
        // Since bcrypt is CPU bound, use tokio::task::spawn_blocking
        tokio::task::spawn_blocking(move || {
            hash(password, DEFAULT_COST)
                .map_err(|e| DomainError::Internal(format!("Password hashing failed: {}", e)))
        })
        .await
        .map_err(|e| DomainError::Internal(format!("Task panic: {}", e)))?
    }

    async fn verify(&self, password: &str, hash_str: &str) -> Result<bool, DomainError> {
        let password = password.to_string();
        let hash_str = hash_str.to_string();

        tokio::task::spawn_blocking(move || {
            verify(&password, &hash_str)
                .map_err(|e| DomainError::Internal(format!("Password verification failed: {}", e)))
        })
        .await
        .map_err(|e| DomainError::Internal(format!("Task panic: {}", e)))?
    }
}
