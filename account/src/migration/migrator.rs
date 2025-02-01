use {
    super::{m20250201_001938_create_user, m20250201_001957_create_workspace},
    sea_orm_migration::{MigrationTrait, MigratorTrait},
};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250201_001938_create_user::Migration),
            Box::new(m20250201_001957_create_workspace::Migration),
        ]
    }
}
