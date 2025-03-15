use {
    crate::{
        app::interfaces::AppInterface,
        config::config::Config,
        downstream::downstream::{Downstream, DownstreamOperations},
    },
    rdkafka::message::OwnedMessage,
    sdk::{
        errors::{AppError, ConnectionError},
        utils::{
            kafka::kafka::{Kafka, KafkaInterface},
            objects::{ObjectStore, S3},
            redis::{MyRedis, RedisInterface},
        },
    },
    std::sync::Arc,
    tokio::sync::mpsc,
    tracing::error,
};

#[derive(Clone)]
pub struct App {
    pub config: Config,
    pub downstream: Arc<dyn DownstreamOperations + Sync + Send>,
    pub redis: Arc<dyn RedisInterface + Send + Sync>,
    pub object_store: Arc<dyn ObjectStore + Send + Sync>,
    pub kafka_receiver: Arc<mpsc::Receiver<OwnedMessage>>,
}

impl App {
    pub async fn new(c: Config) -> Result<Self, AppError> {
        let r = MyRedis::new(
            &c.redis.username,
            &c.redis.password,
            &c.redis.host,
            &c.redis.host,
            &c.redis.name,
        )
        .await;

        let object_store = S3::new(
            &c.object_store.url,
            &c.object_store.access_key_id,
            &c.object_store.access_key_id,
            &c.object_store.provider_name,
        );

        let downstream = Downstream::connect(c.clone()).await;

        // should be done last
        let kafka = Kafka::new(
            &c.kafka.group_id,
            &c.kafka.username,
            &c.kafka.password,
            &c.kafka.host,
            &c.kafka.port,
        )
        .map_err(|e| {
            error!(
                display = %e,
                debug = ?e,
                "error connecting to kafka server"
            );

            return ConnectionError::Kafka;
        })?;

        let (sender, receiver) = mpsc::channel(10); //todo: add right number

        let topic = c.kafka.topic.clone();

        tokio::spawn(async move {
            let _ = kafka.consume(&topic, sender).await;
        });

        tokio::spawn(async move {
            // consume the message received in the receiver channel
        });

        Ok(Self {
            config: c,
            downstream: Arc::new(downstream),
            redis: Arc::new(r),
            object_store: Arc::new(object_store),
            kafka_receiver: Arc::new(receiver),
        })
    }
}

impl AppInterface for App {}
