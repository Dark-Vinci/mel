pub const LAGOS_TIME: &'static str = "Africa/Lagos";
pub const TIME_ZONE: &'static str = "Timezone";
pub const LOCAL_HOST: &'static str = "0.0.0.0";

pub const LOG_DIR: &'static str = "./logs";
pub const LOG_FILE_NAME: &'static str = "file_name.log";
pub const LOG_WARNING_FILE_NAME: &'static str = "warning.log";

pub const DB_PASSWORD: &str = "DB_PASSWORD";
pub const DB_USERNAME: &str = "DB_USERNAME";
pub const DB_HOST: &str = "DB_HOST";
pub const DB_PORT: &str = "DB_PORT";
pub const DB_NAME: &str = "DB_NAME";

pub const REDIS_PASSWORD: &str = "REDIS_PASSWORD";
pub const REDIS_USERNAME: &str = "REDIS_USERNAME";
pub const REDIS_HOST: &str = "REDIS_HOST";
pub const REDIS_PORT: &str = "REDIS_PORT";
pub const REDIS_NAME: &str = "REDIS_NAME";

pub const KAFKA_GROUP_ID: &'static str = "KAFKA_GROUP_ID";
pub const KAFKA_USERNAME: &'static str = "KAFKA_USERNAME";
pub const KAFKA_PASSWORD: &'static str = "KAFKA_PASSWORD";
pub const KAFKA_HOST: &'static str = "KAFKA_HOST";
pub const KAFKA_PORT: &'static str = "KAFKA_PORT";
pub const KAFKA_TOPIC: &'static str = "KAFKA_TOPIC";

pub const OBJECT_STORE_URL: &'static str = "OBJECT_STORE_URL";
pub const OBJECT_STORE_ACCESS_KEY_ID: &'static str =
    "OBJECT_STORE_ACCESS_KEY_ID";
pub const OBJECT_STORE_SECRET_ACCESS_KEY: &'static str =
    "OBJECT_STORE_SECRET_ACCESS_KEY";
pub const OBJECT_STORE_PROVIDER_NAME: &'static str =
    "OBJECT_STORE_PROVIDER_NAME";

pub const ACCOUNT_PORT: &str = "ACCOUNT_PORT";
pub const PORT: &str = "PORT";
pub const ACCOUNT: &str = "ACCOUNT";
pub const WS_CHANNEL: &str = "WS_CHANNEL";

pub const ACCOUNT_URL: &str = "ACCOUNT_URL";
pub const MESSAGE_URL: &str = "MESSAGE_URL";

pub const ZERO_UUID: &'static str = "00000000-00000000-00000000-00000000";

pub const DB_TEARDOWN_QUERY: &str = "
                    SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE
                        pg_stat_activity.datname = current_database() AND
                        pid <> pg_backend_pid();
                ";
