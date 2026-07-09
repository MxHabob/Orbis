use crate::entities::user;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder, QuerySelect};

impl super::Repository {
    pub async fn get_users(
        &self,
        org_id: &str,
        params: crate::params::UserParams,
    ) -> Result<Vec<user::Model>, DbErr> {
        let mut query = user::Entity::find()
            .filter(user::Column::OrgId.eq(org_id))
            .filter(user::Column::IsDeleted.eq(0));

        if let Some(ref role) = params.role {
            if !role.trim().is_empty() {
                query = query.filter(user::Column::PrimaryRole.eq(role));
            }
        }

        if let Some(ref status) = params.status {
            if !status.trim().is_empty() {
                query = query.filter(user::Column::Status.eq(status));
            }
        }

        if let Some(ref search) = params.q {
            if !search.trim().is_empty() {
                let s = format!("%{}%", search);
                query = query.filter(
                    user::Column::FullName.like(&s)
                        .or(user::Column::Email.like(&s))
                        .or(user::Column::NationalId.like(&s))
                );
            }
        }

        query = query.order_by_asc(user::Column::FullName);

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        query.all(&self.db).await
    }

    pub async fn update_user(&self, id: &str, updates: UserUpdate) -> Result<user::Model, DbErr> {
        let u = user::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("user not found".to_string()))?;

        let mut active: user::ActiveModel = u.into();

        if let Some(full_name) = updates.full_name {
            active.full_name = Set(full_name);
        }
        if let Some(name_ar) = updates.name_ar {
            active.name_ar = Set(Some(name_ar));
        }
        if let Some(name_en) = updates.name_en {
            active.name_en = Set(Some(name_en));
        }
        if let Some(email) = updates.email {
            active.email = Set(Some(email));
        }
        if let Some(phone) = updates.phone {
            active.phone = Set(Some(phone));
        }
        if let Some(sex) = updates.sex {
            active.sex = Set(Some(sex));
        }
        if let Some(date_of_birth) = updates.date_of_birth {
            active.date_of_birth = Set(Some(date_of_birth));
        }
        if let Some(nationality) = updates.nationality {
            active.nationality = Set(Some(nationality));
        }
        if let Some(national_id) = updates.national_id {
            active.national_id = Set(Some(national_id));
        }
        if let Some(address) = updates.address {
            active.address = Set(Some(address));
        }
        if let Some(city) = updates.city {
            active.city = Set(Some(city));
        }
        if let Some(country) = updates.country {
            active.country = Set(Some(country));
        }
        if let Some(emergency_contact) = updates.emergency_contact {
            active.emergency_contact = Set(Some(emergency_contact));
        }
        if let Some(blood_type) = updates.blood_type {
            active.blood_type = Set(Some(blood_type));
        }
        if let Some(medical_conditions) = updates.medical_conditions {
            active.medical_conditions = Set(Some(medical_conditions));
        }
        if let Some(avatar) = updates.avatar {
            active.avatar = Set(Some(avatar));
        }
        if let Some(status) = updates.status {
            active.status = Set(status);
        }
        if let Some(primary_role) = updates.primary_role {
            active.primary_role = Set(primary_role);
        }
        if let Some(password_hash) = updates.password_hash {
            active.password_hash = Set(Some(password_hash));
            active.password_changed_at = Set(Some(chrono::Utc::now().to_rfc3339()));
        }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    pub async fn soft_delete_user(&self, id: &str, deleted_by: &str) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = user::Entity::update_many()
            .col_expr(user::Column::IsDeleted, Expr::value(1))
            .col_expr(user::Column::DeletedAt, Expr::value(now))
            .col_expr(user::Column::DeletedBy, Expr::value(deleted_by.to_string()))
            .col_expr(user::Column::Status, Expr::value("inactive"))
            .filter(user::Column::Id.eq(id))
            .filter(user::Column::IsDeleted.eq(0))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn get_users_by_org_role_count(&self, org_id: &str, role: &str) -> Result<u64, DbErr> {
        user::Entity::find()
            .filter(user::Column::OrgId.eq(org_id))
            .filter(user::Column::PrimaryRole.eq(role))
            .filter(user::Column::IsDeleted.eq(0))
            .count(&self.db)
            .await
    }
}

#[derive(Debug, Clone, Default)]
pub struct UserUpdate {
    pub full_name: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub national_id: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub emergency_contact: Option<String>,
    pub blood_type: Option<String>,
    pub medical_conditions: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub primary_role: Option<String>,
    pub password_hash: Option<String>,
}

