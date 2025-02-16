use {
    super::m20241231_191343_first_one,
    sea_orm_migration::{MigrationTrait, MigratorTrait},
};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241231_191343_first_one::Migration),
            // Box::new(m20241231_194402_melony::Migration),
        ]
    }
}
