//! IAM command repository — write operations.

use crate::entities::{user, user_session, user_role, audit_log};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};
use super::IamCommandRepo;

impl IamCommandRepo {
    // ── User Commands ───────────────────────────────────────────

    pub async fn create_user(&self, u: user::Model) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(u.id),
            person_id: Set(u.person_id),
            party_id: Set(u.party_id),
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

    pub async fn update_user(&self, model: user::ActiveModel) -> Result<user::Model, DbErr> {
        model.update(&self.db).await
    }

    // ── Session Commands ────────────────────────────────────────

    pub async fn create_session(&self, s: user_session::Model) -> Result<user_session::Model, DbErr> {
        let active = user_session::ActiveModel {
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
        active.insert(&self.db).await
    }

    // ── Role Commands ───────────────────────────────────────────

    pub async fn assign_role(&self, r: user_role::Model) -> Result<user_role::Model, DbErr> {
        let active = user_role::ActiveModel {
            id: Set(r.id),
            user_id: Set(r.user_id),
            role: Set(r.role),
            org_id: Set(r.org_id),
            scope_type: Set(r.scope_type),
            scope_id: Set(r.scope_id),
            valid_from: Set(r.valid_from),
            valid_until: Set(r.valid_until),
            is_primary: Set(r.is_primary),
            assigned_by: Set(r.assigned_by),
            assigned_at: Set(r.assigned_at),
        };
        active.insert(&self.db).await
    }

    // ── Audit Log Commands ──────────────────────────────────────

    pub async fn create_audit_log(&self, a: audit_log::Model) -> Result<audit_log::Model, DbErr> {
        let active = audit_log::ActiveModel {
            id: Set(a.id),
            org_id: Set(a.org_id),
            user_id: Set(a.user_id),
            action: Set(a.action),
            entity_type: Set(a.entity_type),
            entity_id: Set(a.entity_id),
            changes: Set(a.changes),
            ip_address: Set(a.ip_address),
            user_agent: Set(a.user_agent),
            actor_type: Set(a.actor_type),
            actor_session_id: Set(a.actor_session_id),
            correlation_id: Set(a.correlation_id),
            created_at: Set(a.created_at),
        };
        active.insert(&self.db).await
    }
}
