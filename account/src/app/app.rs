use {
    crate::{
        app::interface::{Account, AccountInterface, Auth, Settings},
        config::config::Config,
        connections::db::DB,
        downstream::downstream::{Downstream, DownstreamImpl},
        repository::{
            channel::{ChannelRepo, ChannelRepository},
            channel_user::ChannelUserRepository,
            user::{UserRepo, UserRepository},
            workspace::{WorkspaceRepo, WorkspaceRepository},
            workspace_user::{WorkspaceUserRepo, WorkspaceUserRepository},
        },
    },
    std::fmt::Display,
    uuid::Uuid,
};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub downstream: Box<dyn Downstream + Sync + Send>,
    // pub redis: Box<dyn RedisInterface>,
    // pub kafka: Box<dyn KafkaInterface>,
    pub user_repo: Box<dyn UserRepository + Sync + Send>,
    pub workspace_repo: Box<dyn WorkspaceRepository + Sync + Send>,
    pub channel_repo: Box<dyn ChannelRepository + Sync + Send>,
    pub channel_user_repo: Box<dyn ChannelUserRepository + Sync + Send>,
    pub workspace_user_repo: Box<dyn WorkspaceUserRepository + Sync + Send>,
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, format!("DB: {:?}\n; Config: {:?}", self.db, self.config))
    }
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();
        let downstream = DownstreamImpl::new();

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

        let user = UserRepo::new(db.clone());
        let channel = ChannelRepo::new(db.clone());
        let channel_user = WorkspaceUserRepo::new(db.clone());
        let workspace = WorkspaceRepo::new(db.clone());
        let workspace_user = WorkspaceUserRepo::new(db.clone());

        Self {
            db,
            workspace_user_repo: Box::new(workspace_user),
            workspace_repo: Box::new(workspace),
            channel_repo: Box::new(channel),
            user_repo: Box::new(user),
            downstream: Box::new(downstream),
            config: Config::new(), //todo; update this
            channel_user_repo: Box::new(channel_user),
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

impl AccountInterface for App {}

#[cfg(test)]
mod test {
    use {
        super::*,
        mockall::{
            automock,
            predicate::{eq, *},
        },
        tracing_subscriber::layer::SubscriberExt,
    };

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
