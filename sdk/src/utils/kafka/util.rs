use {serde::Serialize, std::fmt::Debug, uuid::Uuid};

#[derive(thiserror::Error, Debug, Clone)]
pub enum KafkaError {
    #[error("Kafka Generic error")]
    Generic,
}

#[derive(Debug, Serialize, Clone)]
enum KafkaEventType {}

pub type MQResult<T> = Result<T, KafkaError>;

pub trait KafkaMessage
where
    Self: Serialize + Debug,
{
    fn get_key(&self) -> String;

    fn get_value(&self) -> MQResult<Vec<u8>> {
        serde_json::to_vec(&self).map_err(|err| KafkaError::Generic)
    }

    fn event_type(&self) -> &KafkaEventType;

    fn get_timestamp(&self) -> i64;
}

#[derive(Debug, Serialize)]
struct KafkaEvent<'a, T: KafkaMessage> {
    request_id: Uuid,
    server_name: Uuid,
    timestamp: i64,
    #[serde(flatten)]
    value: T,
    event_type: KafkaEventType,
    id: Uuid,
}

impl<'a, T: KafkaMessage> KafkaEvent<'a, T> {
    pub fn new(
        request_id: Uuid,
        server_name: Uuid,
        value: T,
        typ: KafkaEventType,
    ) -> Self {
        Self {
            request_id,
            server_name,
            timestamp: 0,
            value,
            event_type: typ,
            id: Uuid::new_v4(),
        }
    }
}

impl<T: KafkaMessage> KafkaMessage for KafkaEvent<'_, T> {
    fn get_key(&self) -> String {
        self.event_type.to_string()
    }

    fn event_type(&self) -> KafkaEventType {
        self.event_type.clone()
    }

    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}
