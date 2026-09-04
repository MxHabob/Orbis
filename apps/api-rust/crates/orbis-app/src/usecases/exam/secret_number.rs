use orbis_domain::entities::secret_number::SecretNumber;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::SecretNumberRepository;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct AssignSecretNumberUseCase {
    pub secret_repo: Arc<dyn SecretNumberRepository>,
}

impl AssignSecretNumberUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        exam_schedule_id: Uuid,
        student_id: Uuid,
        secret_number: String,
    ) -> Result<SecretNumber, DomainError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let secret = SecretNumber {
            id,
            exam_schedule_id,
            student_id,
            secret_number,
            created_at: now,
            updated_at: now,
        };

        self.secret_repo.save(&secret).await?;

        Ok(secret)
    }
}
