use orbis_domain::entities::period_template::PeriodTemplate;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::PeriodTemplateRepository;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct CreatePeriodTemplateUseCase {
    pub template_repo: Arc<dyn PeriodTemplateRepository>,
}

impl CreatePeriodTemplateUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        institution_type: String,
        structure: serde_json::Value,
        is_default: bool,
    ) -> Result<PeriodTemplate, DomainError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let template = PeriodTemplate {
            id,
            org_id,
            name,
            name_ar: None,
            institution_type,
            structure,
            is_default,
            created_at: now,
            updated_at: now,
        };

        self.template_repo.save(&template).await?;

        Ok(template)
    }
}

pub struct SeedDefaultTemplatesUseCase {
    pub template_repo: Arc<dyn PeriodTemplateRepository>,
}

impl SeedDefaultTemplatesUseCase {
    pub async fn execute(&self, org_id: Uuid) -> Result<(), DomainError> {
        let structure = serde_json::json!([
            {
                "name": "الفصل الأول",
                "type": "semester",
                "weight": 50,
                "offset_start_days": 0,
                "offset_end_days": 90,
                "children": [
                    { "name": "الفترة الأولى", "type": "quarter", "weight": 25 },
                    { "name": "الفترة الثانية", "type": "quarter", "weight": 25 }
                ]
            },
            {
                "name": "الفصل الثاني",
                "type": "semester",
                "weight": 50,
                "offset_start_days": 100,
                "offset_end_days": 190,
                "children": [
                    { "name": "الفترة الثالثة", "type": "quarter", "weight": 25 },
                    { "name": "الفترة الرابعة", "type": "quarter", "weight": 25 }
                ]
            }
        ]);

        let template = PeriodTemplate {
            id: Uuid::new_v4(),
            org_id,
            name: "التقويم المدرسي الأساسي (فصلين)".to_string(),
            name_ar: Some("التقويم المدرسي الأساسي (فصلين)".to_string()),
            institution_type: "school".to_string(),
            structure,
            is_default: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.template_repo.save(&template).await?;

        Ok(())
    }
}
