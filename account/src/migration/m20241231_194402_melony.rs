use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection()
            .execute_unprepared(
            "
                CREATE TABLE `public`.`workspace` (
                    `id` uuid NOT NULL PRIMARY KEY,
                    `created_by` uuid NOT NULL,
                    `description` varchar,
                    `created_at` timestamp with time zone NOT NULL DEFAULT 'CURRENT_TIMESTAMP',
                    `updated_at` timestamp with time zone NOT NULL DEFAULT 'CURRENT_TIMESTAMP',
                    `deleted_at` timestamp with time zone
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
