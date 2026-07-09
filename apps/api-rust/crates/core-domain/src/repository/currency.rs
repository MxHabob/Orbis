use crate::entities::currency;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, DeleteResult};

impl super::Repository {
    pub async fn get_currencies(&self, org_id: &str) -> Result<Vec<currency::Model>, DbErr> {
        currency::Entity::find()
            .filter(currency::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
    }

    pub async fn get_currency(&self, org_id: &str, code: &str) -> Result<Option<currency::Model>, DbErr> {
        currency::Entity::find()
            .filter(currency::Column::OrgId.eq(org_id))
            .filter(currency::Column::Code.eq(code))
            .one(&self.db)
            .await
    }

    pub async fn create_currency(&self, c: currency::Model) -> Result<currency::Model, DbErr> {
        let active_model = currency::ActiveModel {
            code: Set(c.code),
            org_id: Set(c.org_id),
            name: Set(c.name),
            name_ar: Set(c.name_ar),
            symbol: Set(c.symbol),
            symbol_position: Set(c.symbol_position),
            decimal_places: Set(c.decimal_places),
            exchange_rate: Set(c.exchange_rate),
            is_base: Set(c.is_base),
            is_active: Set(c.is_active),
            updated_at: Set(chrono::Utc::now().to_rfc3339()),
        };
        active_model.insert(&self.db).await
    }

    pub async fn update_currency(&self, org_id: &str, code: &str, updates: CurrencyUpdate) -> Result<currency::Model, DbErr> {
        let c = currency::Entity::find()
            .filter(currency::Column::OrgId.eq(org_id))
            .filter(currency::Column::Code.eq(code))
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("currency not found".to_string()))?;

        let mut active: currency::ActiveModel = c.into();

        if let Some(name) = updates.name {
            active.name = Set(name);
        }
        if let Some(name_ar) = updates.name_ar {
            active.name_ar = Set(name_ar);
        }
        if let Some(symbol) = updates.symbol {
            active.symbol = Set(symbol);
        }
        if let Some(symbol_position) = updates.symbol_position {
            active.symbol_position = Set(symbol_position);
        }
        if let Some(decimal_places) = updates.decimal_places {
            active.decimal_places = Set(decimal_places);
        }
        if let Some(exchange_rate) = updates.exchange_rate {
            active.exchange_rate = Set(exchange_rate);
        }
        if let Some(is_base) = updates.is_base {
            active.is_base = Set(is_base);
        }
        if let Some(is_active) = updates.is_active {
            active.is_active = Set(is_active);
        }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    pub async fn delete_currency(&self, org_id: &str, code: &str) -> Result<DeleteResult, DbErr> {
        currency::Entity::delete_many()
            .filter(currency::Column::OrgId.eq(org_id))
            .filter(currency::Column::Code.eq(code))
            .exec(&self.db)
            .await
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct CurrencyUpdate {
    pub name: Option<String>,
    pub name_ar: Option<Option<String>>,
    pub symbol: Option<String>,
    pub symbol_position: Option<String>,
    pub decimal_places: Option<i32>,
    pub exchange_rate: Option<f64>,
    pub is_base: Option<i32>,
    pub is_active: Option<i32>,
}
