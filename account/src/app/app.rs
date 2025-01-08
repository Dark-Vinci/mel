use {
    crate::{
        app::interface::{Account, Auth, Settings},
        config::config::Config,
        connections::db::DB,
        // downstream::downstream::Downstream,
        // repository::user::UserRepository,
    },
    uuid::Uuid,
};
use crate::repository::user::{ UserRepo, UserRepository};

#[derive(Debug)]
pub struct App {
    pub db: DB,
    pub config: Config,
    // pub downstream: Box<dyn Downstream>,
    // pub redis: Box<dyn RedisInterface>,
    // pub kafka: Box<dyn KafkaInterface>,
    pub user_repo: Box<dyn UserRepository>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await;

        // let redis = MyRedis::new(
        //     &c.redis.username,
        //     &c.redis.password,
        //     &c.redis.host,
        //     &c.redis.port,
        //     "0",
        // );

        // let (db, redis) = join!(db, redis,);

        // let kafka = Kafka::new(
        //     &c.kafka.broker,
        //     vec![],
        //     &c.kafka.group_id,
        //     &c.kafka.username,
        //     &c.kafka.password,
        //     &c.kafka.host,
        //     &c.kafka.port,
        // );

        let db = db.unwrap();

        let u = UserRepo::new(&db);

        Self {
            db,
            user_repo: Box::new(u),
            config: Config::new(),
            // redis: Box::new(redis),
            // kafka: Box::new(kafka),
        }
    }
}

impl App {
    pub fn ping(&self, id: Uuid) -> String {
        format!("PING FROM ACCOUNT SERVICE: {}", id)
    }
}

pub trait AccountInterface: Auth + Account + Settings {}

impl AccountInterface for App {}
