use sea_orm_migration::prelude::*;

/// Migration: Backfill wildcard permissions for super_admin and admin roles.
///
/// The initial bootstrap (`build_default_permissions`) omitted permission entries
/// for `super_admin` and `admin`, meaning these roles received zero effective
/// permissions from the `role_permission` table. This migration retroactively
/// inserts the wildcard grant for any existing organizations.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Insert wildcard permissions for super_admin and admin roles
        // in all existing organizations, only if they don't already exist.
        db.execute_unprepared(
            r#"
            INSERT INTO role_permissions (id, role_id, org_id, resource, action, created_at)
            SELECT
                lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),
                r.role_id,
                r.org_id,
                '*',
                '*',
                datetime('now')
            FROM (
                SELECT 'super_admin' as role_id, id as org_id FROM organizations
                UNION ALL
                SELECT 'admin' as role_id, id as org_id FROM organizations
            ) r
            WHERE NOT EXISTS (
                SELECT 1 FROM role_permissions rp
                WHERE rp.role_id = r.role_id
                  AND rp.org_id = r.org_id
                  AND rp.resource = '*'
                  AND rp.action = '*'
            )
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "DELETE FROM role_permissions WHERE role_id IN ('super_admin', 'admin') AND resource = '*' AND action = '*'",
        )
        .await?;

        Ok(())
    }
}
