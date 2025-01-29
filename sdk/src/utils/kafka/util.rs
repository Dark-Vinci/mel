use {
    serde::Serialize,
    std::fmt::{Debug, Display},
};

#[derive(thiserror::Error, Debug, Clone)]
pub enum KafkaError {
    #[error("Kafka Generic error")]
    Generic,

    #[error("Kafka Connection error")]
    Connection,
}

#[derive(Debug, Serialize, Clone)]
pub enum KafkaEventType {}

impl Display for KafkaEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "KafkaEventType".to_string())
    }
}

pub type MQResult<T> = Result<T, KafkaError>;

pub trait KafkaMessage
where
    Self: Serialize + Debug + Send,
{
    fn get_key(&self) -> String;

    fn get_value(&self) -> MQResult<Vec<u8>> {
        serde_json::to_vec(&self).map_err(|_err| KafkaError::Generic)
    }

    fn event_type(&self) -> &str;

    fn get_timestamp(&self) -> i64;
}
