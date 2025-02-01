use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE `public`.`users` (
                `id` uuid NOT NULL PRIMARY KEY,
                `first_name` varchar NOT NULL,
                `last_name` varchar NOT NULL,
                `date_of_birth` datetime NOT NULL,
                `email` varchar NOT NULL UNIQUE,
                `password` varchar NOT NULL,
                `created_at` timestamp_with_timezone NOT NULL DEFAULT 'CURRENT_TIMESTAMP',
                `updated_at` timestamp_with_timezone NOT NULL DEFAULT 'CURRENT_TIMESTAMP',
                `deleted_at` timestamp_with_timezone
                );
            "
        )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP TABLE public.users;").await?;

        Ok(())
    }
}
