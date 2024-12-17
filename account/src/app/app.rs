use {
    crate::{
        app::interface::{Account, Auth, Settings},
        config::config::Config,
        connections::db::DB,
        repository::user::UserRepository,
    },
    sdk::utils::{
        kafka::kafka::{Kafka, KafkaInterface},
        redis::{MyRedis, RedisInterface},
    },
    uuid::Uuid,
};

pub trait AccountInterface: Auth + Account + Settings {}

#[derive(Debug)]
pub struct App {
    pub db: DB,
    pub redis: Box<dyn RedisInterface>,
    pub kafka: Box<dyn KafkaInterface>,
    pub user_repo: Box<dyn UserRepository>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c);

        let kafka = Kafka::new(
            &c.kafka.broker,
            vec![],
            &c.kafka.group_id,
            &c.kafka.username,
            &c.kafka.password,
            &c.kafka.host,
            &c.kafka.port,
        );

        let redis = MyRedis::new(
            &c.redis.username,
            &c.redis.password,
            &c.redis.host,
            &c.redis.port,
            "0",
        )
        .await;

        let u = Box::new(&db);

        Self {
            db,
            user_repo: u,
            redis: Box::new(redis),
            kafka: Box::new(kafka),
        }
    }
}

impl App {
    pub fn ping(&self, id: Uuid) -> String {
        format!("PING FROM ACCOUNT SERVICE: {}", id)
    }
}

impl AccountInterface for App {}
