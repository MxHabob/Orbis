use async_trait::async_trait;
use orbis_domain::errors::DomainError;

#[async_trait]
pub trait TransactionPort: Send + Sync {
    async fn begin_transaction(&self) -> Result<(), DomainError>;
    async fn commit_transaction(&self) -> Result<(), DomainError>;
    async fn rollback_transaction(&self) -> Result<(), DomainError>;
}
