use sea_orm::Schema;
use sea_orm_migration::prelude::*;

use crate::models::{employee, group, user_group_membership, group_permission};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        // 1. Create employees table
        let mut emp_stmt = schema.create_table_from_entity(employee::Entity);
        manager.create_table(emp_stmt.if_not_exists().to_owned()).await?;

        // 2. Create groups table
        let mut grp_stmt = schema.create_table_from_entity(group::Entity);
        manager.create_table(grp_stmt.if_not_exists().to_owned()).await?;

        // 3. Create user_group_memberships table
        let mut ugm_stmt = schema.create_table_from_entity(user_group_membership::Entity);
        manager.create_table(ugm_stmt.if_not_exists().to_owned()).await?;

        // 4. Create group_permissions table
        let mut gp_stmt = schema.create_table_from_entity(group_permission::Entity);
        manager.create_table(gp_stmt.if_not_exists().to_owned()).await?;

        // 5. Indexes
        let idx_emp_num = Index::create()
            .name("idx_employees_org_number")
            .table(Alias::new("employees"))
            .col(Alias::new("org_id"))
            .col(Alias::new("employee_number"))
            .if_not_exists()
            .to_owned();
        manager.create_index(idx_emp_num).await?;

        let idx_emp_person = Index::create()
            .name("idx_employees_org_person")
            .table(Alias::new("employees"))
            .col(Alias::new("org_id"))
            .col(Alias::new("person_id"))
            .if_not_exists()
            .to_owned();
        manager.create_index(idx_emp_person).await?;

        let idx_ugm_user = Index::create()
            .name("idx_user_group_user")
            .table(Alias::new("user_group_memberships"))
            .col(Alias::new("user_id"))
            .if_not_exists()
            .to_owned();
        manager.create_index(idx_ugm_user).await?;

        let idx_gp_group = Index::create()
            .name("idx_group_permissions_group")
            .table(Alias::new("group_permissions"))
            .col(Alias::new("group_id"))
            .if_not_exists()
            .to_owned();
        manager.create_index(idx_gp_group).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(group_permission::Entity).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(user_group_membership::Entity).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(group::Entity).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(employee::Entity).if_exists().to_owned())
            .await?;
        Ok(())
    }
}
