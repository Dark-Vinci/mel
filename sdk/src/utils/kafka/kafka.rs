use rdkafka::producer::BaseRecord;
use {
    crate::utils::kafka::{
        context::CustomContext,
        util::{KafkaError, KafkaMessage},
    },
    log::{error, info, warn},
    mockall::automock,
    rdkafka::{
        config::RDKafkaLogLevel,
        consumer::{stream_consumer::StreamConsumer, CommitMode, Consumer},
        message::{
            BorrowedMessage, Header, Headers, OwnedHeaders, OwnedMessage,
        },
        producer::{FutureProducer, FutureRecord},
        ClientConfig, Message,
    },
    std::time::Duration,
    tokio::sync::mpsc::{self, Receiver, Sender},
};

pub type SdkConsumer = StreamConsumer<CustomContext>;

pub struct Kafka {
    topic: Vec<String>,
    // topic_name: String,
    // host: String,
    // password: String,
    // username: String,
    producer: FutureProducer,
    consumer: SdkConsumer,
}

pub trait KafkaInterface {
    async fn log_event<T: KafkaMessage>(
        &self,
        payload: T,
    ) -> Result<(), KafkaError>;
    async fn consume(&self, topic: &str, sender: Sender<OwnedMessage>);
}

impl Kafka {
    pub fn new(
        brokers: &str,
        topic: Vec<String>,
        group_id: &str,
        username: &str,
        password: &str,
        host: &str,
        port: &str,
    ) -> Self {
        let context = CustomContext;

        let client = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("bootstrap.servers", format!("{}:{}", host, port))
            .set("sasl.username", username)
            .set("sasl.password", password)
            .set("message.timeout.ms", "5000")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("statistics.interval.ms", "30000")
            .set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context);

        let producer: &FutureProducer = client.create().clone().unwrap();
        let consumer: &SdkConsumer = client.create().unwrap();

        Self {
            topic,
            producer,
            consumer,
            // topic_name: "".to_string(),
            // host: "localhost".to_string(),
            // password: "".to_string(),
            // username: "".to_string(),
        }
    }
}

impl KafkaInterface for Kafka {
    async fn log_event<T: KafkaMessage>(
        &self,
        payload: T,
    ) -> Result<(), KafkaError> {
        self.producer
            .send(
                BaseRecord::to(&payload.event_type().to_string())
                    .payload(&payload)
                    .key(&payload.get_key())
                    .payload(&payload.get_value())
                    .timestamp(payload.get_timestamp()),
                Duration::from_secs(0),
            )
            .await
            .map_err(|err| {
                error!("Failed to produce message: {:?}", err);

                return KafkaError::Generic;
            })
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

        loop {
            match self.consumer.recv().await {
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
    }
}
