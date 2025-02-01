use {
    super::{
        m20250201_001938_create_user,
        m20250201_001957_create_workspace,
        m20250201_003049_create_workspace_user,
        m20250201_003108_create_channel_user,
        m20250201_003101_create_channel,
    },
    sea_orm_migration::{MigrationTrait, MigratorTrait},
};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250201_001938_create_user::Migration),
            Box::new(m20250201_001957_create_workspace::Migration),
            Box::new(m20250201_003049_create_workspace_user::Migration),
            Box::new(m20250201_003101_create_channel::Migration),
            Box::new(m20250201_003108_create_channel_user::Migration),
        ]
    }
}
