use {
    crate::{
        app::interface::{Message, Reaction, Response},
        config::config::Config,
        connections::db::DB,
        repository::{
            message::{MessageRepo, MessageRepository},
            reaction::{ReactionRepo, ReactionRepository},
            response::{ResponseRepo, ResponseRepository},
            user::{UserRepo, UserRepository},
        },
    },
    uuid::Uuid,
};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub user_repo: Box<dyn UserRepository + Sync + Send>,
    pub message_repo: Box<dyn MessageRepository + Sync + Send>,
    pub response_repo: Box<dyn ResponseRepository + Sync + Send>,
    pub reaction_repo: Box<dyn ReactionRepository + Sync + Send>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

        let u = UserRepo::new(db.clone());
        let message_repo = MessageRepo::new(db.clone());
        let reaction_repo = ReactionRepo::new(db.clone());
        let response_repo = ResponseRepo::new(db.clone());

        Self {
            db,
            user_repo: Box::new(u),
            config: Config::new(),
            message_repo: Box::new(message_repo),
            reaction_repo: Box::new(reaction_repo),
            response_repo: Box::new(response_repo),
        }
    }
}

impl App {
    pub fn ping(&self, id: Uuid) -> String {
        format!("PING FROM ACCOUNT SERVICE: {}", id)
    }
}

pub trait MessagingInterface: Message + Response + Reaction {}

impl MessagingInterface for App {}
