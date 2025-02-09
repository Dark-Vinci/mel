#[cfg(test)]
use mockall::automock;

use {
    crate::{
        app::interface::{Account, Auth, Settings},
        config::config::Config,
        connections::db::DB,
        repository::user::{UserRepo, UserRepository},
        // downstream::downstream::Downstream,
        // repository::user::UserRepository,
    },
    uuid::Uuid,
};

// #[derive(Debug)]
pub struct App {
    pub db: DB,
    pub config: Config,
    // pub downstream: Box<dyn Downstream>,
    // pub redis: Box<dyn RedisInterface>,
    // pub kafka: Box<dyn KafkaInterface>,
    pub user_repo: Box<dyn UserRepository + Sync + Send>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

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

        let u = UserRepo::new(db.clone());

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

#[cfg(test)]
mod test {
    use super::*;
    use mockall::predicate::eq;
    use mockall::{automock, predicate::*};
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    fn first() {
        #[automock]
        trait MyTrait {
            fn foo(&self, x: u32) -> u32;
        }

        fn call_with_four(x: &dyn MyTrait) -> u32 {
            x.foo(4)
        }

        let mut mock = MockMyTrait::new();

        mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);

        assert_eq!(5, call_with_four(&mock));
    }
}
