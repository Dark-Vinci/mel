use mockall::automock;
use {
    crate::utils::kafka::context::CustomContext,
    log::{info, warn},
    rdkafka::{
        config::RDKafkaLogLevel,
        consumer::{stream_consumer::StreamConsumer, CommitMode, Consumer},
        message::{BorrowedMessage, Header, Headers, OwnedHeaders},
        producer::{FutureProducer, FutureRecord},
        ClientConfig, Message,
    },
    std::time::Duration,
    tokio::sync::mpsc::{self, Receiver, Sender},
};

pub type SdkConsumer = StreamConsumer<CustomContext>;

pub struct Kafka {
    topic: Vec<String>,
    topic_name: String,
    host: String,
    password: String,
    username: String,
    producer: FutureProducer,
    consumer: SdkConsumer,
}

pub trait KafkaInterface {
    async fn produce(&self, payload: Vec<u8>, id: String) -> bool;
    async fn consume(&self, sender: Sender<&BorrowedMessage>);
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
        
        // client.

        let producer: &FutureProducer = client.create().clone().unwrap();
        let consumer: &SdkConsumer = client.create().unwrap();

        Self {
            topic,
            producer,
            consumer,
            topic_name: "".to_string(),
            host: "localhost".to_string(),
            password: "".to_string(),
            username: "".to_string(),
        }
    }
}

impl KafkaInterface for Kafka {
    async fn produce(&self, payload: Vec<u8>, id: String) -> bool {
        let delivery_status = self
            .producer
            .send(
                FutureRecord::to(&self.topic_name)
                    .payload(&format!("Message {}", payload.into()))
                    .key(&format!("Key {}", id))
                    .headers(OwnedHeaders::new().insert(Header {
                        key: "header_key",
                        value: Some("header_value"),
                    })),
                Duration::from_secs(0),
            )
            .await;

        delivery_status.is_ok()
    }

    async fn consume(&self, sender: Sender<&BorrowedMessage>) {
        self.consumer
            .subscribe(&[""])
            .expect("Can't subscribe to specified topics");

        loop {
            match self.consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),

                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        },
                    };

                    info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());

                    if let Some(headers) = m.headers() {
                        for header in headers.iter() {
                            info!(
                                "  Header {:#?}: {:?}",
                                header.key, header.value
                            );
                        }
                    }

                    self.consumer
                        .commit_message(&m, CommitMode::Async)
                        .unwrap();

                    sender.send(&m).await.unwrap();
                },
            };
        }
    }
}
