use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection()
            .execute_unprepared(
                "
                CREATE TABLE `public`.`workspace_user` (
                    `id` uuid NOT NULL PRIMARY KEY,
                    `user_id` uuid NOT NULL,
                    `workspace_id` uuid NOT NULL,
                    `invited_by` uuid,
                    `created_at` timestamp with time zone NOT NULL DEFAULT 'CURRENT_TIMESTAMP',
                    `deleted_at` timestamp with time zone,
                    CONSTRAINT `fk-workspace_user-user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
                    CONSTRAINT `fk-workspace_user-workspace_id` FOREIGN KEY (`workspace_id`) REFERENCES `workspace` (`id`)
                );
            "
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DELETE FROM public.`workspace_user`;")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
