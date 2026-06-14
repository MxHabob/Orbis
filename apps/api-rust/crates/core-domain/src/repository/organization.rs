use crate::entities::organization;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait};

impl super::Repository {
    pub async fn get_organization_by_id(&self, id: &str) -> Result<Option<organization::Model>, DbErr> {
        organization::Entity::find()
            .filter(organization::Column::Id.eq(id))
            .filter(organization::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    pub async fn update_organization(&self, id: &str, updates: OrganizationUpdate) -> Result<organization::Model, DbErr> {
        let org = organization::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("organization not found".to_string()))?;

        let mut active: organization::ActiveModel = org.into();

        if let Some(name) = updates.name {
            active.name = Set(name);
        }
        if let Some(timezone) = updates.timezone {
            active.timezone = Set(timezone);
        }
        if let Some(locale) = updates.locale {
            active.locale = Set(locale);
        }
        if let Some(currency) = updates.currency {
            active.currency = Set(currency);
        }
        if let Some(date_format) = updates.date_format {
            active.date_format = Set(Some(date_format));
        }
        if let Some(hijri_enabled) = updates.hijri_enabled {
            active.hijri_enabled = Set(hijri_enabled);
        }
        if let Some(logo) = updates.logo {
            active.logo = Set(Some(logo));
        }
        if let Some(primary_color) = updates.primary_color {
            active.primary_color = Set(Some(primary_color));
        }
        if let Some(settings) = updates.settings {
            active.settings = Set(Some(settings));
        }
        if let Some(features) = updates.features {
            active.features = Set(Some(features));
        }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }
}

#[derive(Debug, Clone)]
pub struct OrganizationUpdate {
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
