use crate::repository::organization::OrganizationUpdate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub currency: Option<String>,
    pub date_format: Option<String>,
    pub hijri_enabled: Option<i32>,
    pub logo: Option<String>,
    pub primary_color: Option<String>,
    pub settings: Option<String>,
    pub features: Option<String>,
}

impl super::Service {
    pub async fn get_organization(&self, org_id: &str) -> Result<crate::entities::organization::Model, String> {
        self.repo.get_organization_by_id(org_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "organization not found".to_string())
    }

    pub async fn update_organization(&self, org_id: &str, req: UpdateOrganizationRequest) -> Result<crate::entities::organization::Model, String> {
        let updates = OrganizationUpdate {
            name: req.name,
            timezone: req.timezone,
            locale: req.locale,
            currency: req.currency,
            date_format: req.date_format,
            hijri_enabled: req.hijri_enabled,
            logo: req.logo,
            primary_color: req.primary_color,
            settings: req.settings,
            features: req.features,
        };

        self.repo.update_organization(org_id, updates).await
            .map_err(|e| e.to_string())
    }
}
