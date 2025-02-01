use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection()
            .execute_unprepared(
                "
                CREATE TABLE `public`.`channel_user` (
                    `id` uuid NOT NULL PRIMARY KEY,
                    `user_id` uuid NOT NULL,
                    `channel_id` uuid NOT NULL,
                    `invited_by` uuid,
                    `created_at` timestamp with time zone NOT NULL DEFAULT 'CURRENT_TIMESTAMP',
                    `deleted_at` timestamp with time zone,
                    CONSTRAINT `fk-channel_user-user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
                    CONSTRAINT `fk-channel_user-channel_id` FOREIGN KEY (`channel_id`) REFERENCES `channel` (`id`)
                );
            "
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DELETE FROM public.`workspace`;")
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
