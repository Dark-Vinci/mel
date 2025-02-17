use {
    crate::{config::config::Config, migration::migrator::Migrator},
    sdk::{
        constants::{constant::DB_TEARDOWN_QUERY, Environment},
        errors::ConnectionError,
        utils::utility,
    },
    sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection},
    sea_orm_migration::MigratorTrait,
    std::time::Duration,
    tokio::runtime,
    tracing::{debug, error, log::LevelFilter},
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct DB {
    pub connection: DatabaseConnection,
}

impl DB {
    pub async fn new(e: &Config) -> Result<Self, ConnectionError> {
        let conn_string = if e.environment == Environment::Testing {
            let id = Uuid::new_v4();
            debug!("sqlite_extras_test_id: {}", id);
            utility::sqlite_test_document(id)
        } else {
            format!(
                "postgres://{0}:{1}@{2}:{3}/{4}",
                e.db.username, e.db.password, e.db.host, e.db.port, e.db.name
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
            .sqlx_logging_level(LevelFilter::Info);

        let db = Database::connect(opt).await;

        if let Err(e) = db {
            error!(error = %e.to_string(), "Error connecting to database");
            return Err(ConnectionError::DB(e.to_string()));
        }

        let db = db.unwrap();

        if e.environment != Environment::Production {
            // running for the first time;
            Migrator::install(&db).await.unwrap();
            Migrator::up(&db, None).await.unwrap();
        }

        debug!("Extras connected to the DB");

        Ok(Self { connection: db })
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        let db = self.connection.clone();
        let runtime = runtime::Handle::current();

        runtime.block_on(async {
            db.execute(sea_orm::Statement::from_string(
                db.get_database_backend(),
                DB_TEARDOWN_QUERY,
            ))
            .await
            .unwrap();
        });
    }
}
