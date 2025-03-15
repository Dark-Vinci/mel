use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("DB connection error")]
    DB(String),

    #[error("Unable to connect with kafka server")]
    Kafka,
}
