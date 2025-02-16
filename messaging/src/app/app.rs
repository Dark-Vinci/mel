use {
    crate::{
        app::interface::{Account, Auth, Settings},
        config::config::Config,
        connections::db::DB,
        repository::user::{UserRepo, UserRepository},
    },
    uuid::Uuid,
};
use crate::repository::message::{MessageRepo, MessageRepository};
use crate::repository::reaction::{ReactionRepo, ReactionRepository};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub user_repo: Box<dyn UserRepository + Sync + Send>,
    pub message_repo: Box<dyn MessageRepository + Sync + Send>,
    pub reaction_repo: Box<dyn ReactionRepository + Sync + Send>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

        let u = UserRepo::new(db.clone());
        let message_repo = MessageRepo::new(db.clone());
        let reaction_repo = ReactionRepo::new(db.clone());

        Self {
            db,
            user_repo: Box::new(u),
            config: Config::new(),
            message_repo: Box::new(message_repo),
            reaction_repo: Box::new(reaction_repo),
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
