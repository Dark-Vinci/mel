use {
    sdk::models::db::account::channel_user::Entity as WorkspaceUser,
    sea_orm::{DbBackend, Schema},
};

fn main() {
    let db_postgres = DbBackend::Postgres;
    let schema = Schema::new(db_postgres);

    let a = db_postgres
        .build(&schema.create_table_from_entity(WorkspaceUser))
        .sql;

    // let  = db_postgres.build(&schema.(User)).sql;

    println!("{a}");
}
