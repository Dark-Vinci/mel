use {
    crate::utils::kafka::util::{KafkaEventType, KafkaMessage},
    serde::Serialize,
    uuid::Uuid,
};

#[derive(Debug, Serialize)]
pub struct KafkaEvent<T: KafkaMessage> {
    request_id: Uuid,
    server_name: Uuid,
    timestamp: i64,
    #[serde(flatten)]
    value: T,
    event_type: KafkaEventType,
    id: Uuid,
}

impl<T: KafkaMessage> KafkaEvent<T> {
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

impl<T: KafkaMessage> KafkaMessage for KafkaEvent<T> {
    fn get_key(&self) -> String {
        self.event_type.clone().to_string()
    }

    fn event_type(&self) -> &str {
        "KafkaEvent"
    }

    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}
