use {channel::migration::migrator::Migrator, sea_orm_migration::prelude::*};

#[tokio::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
