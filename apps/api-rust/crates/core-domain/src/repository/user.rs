use crate::entities::{user, organization, user_session};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait};

impl super::Repository {
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .filter(user::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Id.eq(id))
            .filter(user::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    pub async fn create_user(&self, u: user::Model) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(u.id),
            org_id: Set(u.org_id),
            email: Set(u.email),
            email_verified: Set(u.email_verified),
            email_verified_at: Set(u.email_verified_at),
            phone: Set(u.phone),
            phone_verified: Set(u.phone_verified),
            phone_verified_at: Set(u.phone_verified_at),
            password_hash: Set(u.password_hash),
            password_changed_at: Set(u.password_changed_at),
            password_expires_at: Set(u.password_expires_at),
            mfa_enabled: Set(u.mfa_enabled),
            mfa_secret: Set(u.mfa_secret),
            mfa_backup_codes: Set(u.mfa_backup_codes),
            full_name: Set(u.full_name),
            name_ar: Set(u.name_ar),
            name_en: Set(u.name_en),
            avatar: Set(u.avatar),
            sex: Set(u.sex),
            date_of_birth: Set(u.date_of_birth),
            place_of_birth: Set(u.place_of_birth),
            nationality: Set(u.nationality),
            national_id: Set(u.national_id),
            passport_number: Set(u.passport_number),
            address: Set(u.address),
            city: Set(u.city),
            country: Set(u.country),
            postal_code: Set(u.postal_code),
            emergency_contact: Set(u.emergency_contact),
            blood_type: Set(u.blood_type),
            allergies: Set(u.allergies),
            medical_conditions: Set(u.medical_conditions),
            medical_notes: Set(u.medical_notes),
            social_links: Set(u.social_links),
            bio: Set(u.bio),
            primary_role: Set(u.primary_role),
            status: Set(u.status),
            last_login_at: Set(u.last_login_at),
            last_login_ip: Set(u.last_login_ip),
            login_count: Set(u.login_count),
            failed_logins: Set(u.failed_logins),
            locked_until: Set(u.locked_until),
            prefs: Set(u.prefs),
            meta: Set(u.meta),
            is_deleted: Set(u.is_deleted),
            deleted_at: Set(u.deleted_at),
            deleted_by: Set(u.deleted_by),
            created_at: Set(u.created_at),
            updated_at: Set(u.updated_at),
            created_by: Set(u.created_by),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_user_count(&self) -> Result<u64, DbErr> {
        user::Entity::find()
            .filter(user::Column::IsDeleted.eq(0))
            .count(&self.db)
            .await
    }

    // ── Session ───────────────────────────────────────────────────

    pub async fn create_session(&self, s: user_session::Model) -> Result<user_session::Model, DbErr> {
        let active_model = user_session::ActiveModel {
            id: Set(s.id),
            user_id: Set(s.user_id),
            token: Set(s.token),
            refresh_token: Set(s.refresh_token),
            device_id: Set(s.device_id),
            device_type: Set(s.device_type),
            device_name: Set(s.device_name),
            os: Set(s.os),
            browser: Set(s.browser),
            ip_address: Set(s.ip_address),
            user_agent: Set(s.user_agent),
            is_current: Set(s.is_current),
            is_trusted: Set(s.is_trusted),
            mfa_verified: Set(s.mfa_verified),
            expires_at: Set(s.expires_at),
            refresh_expires_at: Set(s.refresh_expires_at),
            last_active_at: Set(s.last_active_at),
            created_at: Set(s.created_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_session_by_token(&self, token: &str) -> Result<Option<user_session::Model>, DbErr> {
        user_session::Entity::find()
            .filter(user_session::Column::Token.eq(token))
            .filter(user_session::Column::IsCurrent.eq(1))
            .one(&self.db)
            .await
    }

    pub async fn invalidate_session(&self, token: &str) -> Result<u64, DbErr> {
        let update_res = user_session::Entity::update_many()
            .col_expr(user_session::Column::IsCurrent, Expr::value(0))
            .filter(user_session::Column::Token.eq(token))
            .exec(&self.db)
            .await?;
        Ok(update_res.rows_affected)
    }

    // ── Organization ───────────────────────────────────────────────

    pub async fn get_organization_by_slug(&self, slug: &str) -> Result<Option<organization::Model>, DbErr> {
        organization::Entity::find()
            .filter(organization::Column::Slug.eq(slug))
            .filter(organization::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    pub async fn create_organization(&self, o: organization::Model) -> Result<organization::Model, DbErr> {
        let active_model = organization::ActiveModel {
            id: Set(o.id),
            name: Set(o.name),
            name_ar: Set(o.name_ar),
            slug: Set(o.slug),
            code: Set(o.code),
            timezone: Set(o.timezone),
            locale: Set(o.locale),
            currency: Set(o.currency),
            date_format: Set(o.date_format),
            hijri_enabled: Set(o.hijri_enabled),
            max_users: Set(o.max_users),
            max_students: Set(o.max_students),
            max_storage_gb: Set(o.max_storage_gb),
            settings: Set(o.settings),
            features: Set(o.features),
            subscription_plan: Set(o.subscription_plan),
            subscription_expires_at: Set(o.subscription_expires_at),
            logo: Set(o.logo),
            favicon: Set(o.favicon),
            primary_color: Set(o.primary_color),
            is_active: Set(o.is_active),
            is_verified: Set(o.is_verified),
            is_deleted: Set(o.is_deleted),
            deleted_at: Set(o.deleted_at),
            deleted_by: Set(o.deleted_by),
            created_at: Set(o.created_at),
            updated_at: Set(o.updated_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_organization_count(&self) -> Result<u64, DbErr> {
        organization::Entity::find()
            .filter(organization::Column::IsDeleted.eq(0))
            .count(&self.db)
            .await
    }
}
