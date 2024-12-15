use {
    crate::{config::config::Config, migrations::migrator::Migrator},
    sdk::{constants::Environment, utils::helper},
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
    sea_orm_migration::MigratorTrait,
    std::{sync::Arc, time::Duration},
    tokio::{runtime, task},
    tracing::{debug, log},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct DB {
    conn: DatabaseConnection,
    pub connection: Arc<DatabaseConnection>,
}

impl DB {
    pub fn new(e: &Config) -> Self {
        let mut dbl: DatabaseConnection = Default::default();

        runtime::Handle::current().block_on(async {
            let conn_string = if e.environment == Environment::Testing {
                let id = Uuid::new_v4();
                debug!("sqlite_test_id: {}", id.to_string());
                helper::sqlite_test_document(id)
            } else {
                format!(
                    "postgres://{0}:{1}@{2}:{3}/{4}",
                    e.db.username,
                    e.db.password,
                    e.db.host,
                    e.db.port,
                    e.db.name
                )
            };

            let mut opt = ConnectOptions::new(conn_string);

            opt.max_connections(100)
                .min_connections(5)
                .connect_timeout(Duration::from_secs(8))
                .acquire_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .max_lifetime(Duration::from_secs(8))
                .sqlx_logging(true)
                .sqlx_logging_level(log::LevelFilter::Info);

            let db = Database::connect(opt).await;

            assert!(db.is_ok());

            let db = db.unwrap();

            if e.environment != Environment::Production {
                // running for the first time;
                Migrator::install(&db).await.unwrap();

                Migrator::up(&db, None).await.unwrap();
            }

            debug!("connected to the DB");

            dbl = db;
        });

        Self {
            connection: Arc::new(dbl.clone()),
            conn: dbl,
        }
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        let connection = self.conn.clone();

        task::block_in_place(move || {
            let runtime = runtime::Handle::current();

            runtime.block_on(async {
                if let Err(err) = connection.close().await {
                    eprintln!(
                        "Failed to close the database connection: {}",
                        err
                    );
                }
            });
        });
    }
}
