use {
    crate::utils::kafka::util::{KafkaError, KafkaMessage},
    log::{error, info, warn},
    rdkafka::{
        config::RDKafkaLogLevel,
        consumer::{stream_consumer::StreamConsumer, CommitMode, Consumer},
        message::{Headers, OwnedMessage},
        producer::{FutureProducer, FutureRecord},
        ClientConfig, Message,
    },
    std::time::Duration,
    tokio::sync::mpsc::Sender,
    tonic::{async_trait, codegen::tokio_stream::StreamExt},
};

pub struct Kafka {
    producer: FutureProducer,
    consumer: StreamConsumer,
}

#[async_trait]
pub trait KafkaInterface {
    async fn log_event<T: KafkaMessage>(
        &self,
        payload: T,
    ) -> Result<(), KafkaError>;
    async fn consume(
        &self,
        topic: &str,
        sender: Sender<OwnedMessage>,
    ) -> Result<(), KafkaError>;
}

impl Kafka {
    pub fn new(
        group_id: &str,
        username: &str,
        password: &str,
        host: &str,
        port: &str,
    ) -> Result<Self, KafkaError> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", format!("{}:{}", host, port)) // Kafka broker address
            .set("group.id", group_id) // Consumer group ID
            .set("auto.offset.reset", "earliest")
            .set("sasl.username", username)
            .set("sasl.password", password)
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .map_err(|_err| KafkaError::Connection)?;

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", format!("{}:{}", host, port)) // Kafka broker address
            .set("message.timeout.ms", "5000")
            .set("sasl.username", username)
            .set("sasl.password", password)
            .set_log_level(RDKafkaLogLevel::Debug) // Message timeout
            .create()
            .map_err(|_err| KafkaError::Connection)?;

        Ok(Self { producer, consumer })
    }
}

#[async_trait]
impl KafkaInterface for Kafka {
    async fn log_event<T: KafkaMessage>(
        &self,
        payload: T,
    ) -> Result<(), KafkaError> {
        let load = &payload.get_value()?;
        let topic = payload.event_type();
        let key = &payload.get_key();

        let new_payload = FutureRecord::to(topic)
            .key(key)
            .payload(load)
            .timestamp(payload.get_timestamp());

        self.producer
            .send(new_payload, Duration::from_secs(1))
            .await
            .map_err(|err| {
                error!("Failed to produce message: {:?}", err);

                return KafkaError::Generic;
            })?;

        Ok(())
    }

    async fn consume(
        &self,
        topic: &str,
        sender: Sender<OwnedMessage>,
    ) -> Result<(), KafkaError> {
        let connection = self.consumer.subscribe(&[topic]).map_err(|err| {
            error!("Failed to subscribe to topic: {:?}", err);
            return KafkaError::Generic;
        });

        if let Err(_) = connection {
            return connection;
        }

        let mut message_stream = self.consumer.stream();

        while let Some(message) = message_stream.next().await {
            match message {
                Err(e) => warn!("Kafka error: {}", e),

                Ok(received_message) => {
                    let payload = match received_message.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        },
                    };

                    info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      received_message.key(), payload, received_message.topic(), received_message.partition(), received_message.offset(), received_message.timestamp());

                    if let Some(headers) = received_message.headers() {
                        for header in headers.iter() {
                            info!(
                                "  Header {:#?}: {:?}",
                                header.key, header.value
                            );
                        }
                    }

                    self.consumer
                        .commit_message(&received_message, CommitMode::Async)
                        .unwrap();

                    sender.send(received_message.detach()).await.unwrap();
                },
            };
        }

        Ok(())
    }
}
