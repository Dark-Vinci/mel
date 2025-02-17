use {
    crate::{
        app::interface::{Chat, Message, Reaction, Response},
        config::config::Config,
        connections::db::DB,
        repository::{
            chat::{ChatRepo, ChatRepository},
            message::{MessageRepo, MessageRepository},
            reaction::{ReactionRepo, ReactionRepository},
            response::{ResponseRepo, ResponseRepository},
        },
    },
    uuid::Uuid,
};

pub struct App {
    pub db: DB,
    pub config: Config,
    pub message_repo: Box<dyn MessageRepository + Sync + Send>,
    pub response_repo: Box<dyn ResponseRepository + Sync + Send>,
    pub reaction_repo: Box<dyn ReactionRepository + Sync + Send>,
    pub chat_repo: Box<dyn ChatRepository + Sync + Send>,
}

impl App {
    pub async fn new(c: &Config) -> Self {
        let db = DB::new(&c).await.unwrap();

        let message_repo = MessageRepo::new(db.clone());
        let reaction_repo = ReactionRepo::new(db.clone());
        let response_repo = ResponseRepo::new(db.clone());
        let chat_repo = ChatRepo::new(db.clone());

        Self {
            db,
            config: Config::new(),
            message_repo: Box::new(message_repo),
            reaction_repo: Box::new(reaction_repo),
            response_repo: Box::new(response_repo),
            chat_repo: Box::new(chat_repo),
        }
    }
}

impl App {
    pub fn ping(&self, id: Uuid) -> String {
        format!("PING FROM ACCOUNT SERVICE: {}", id)
    }
}

pub trait MessagingTrait: Message + Response + Reaction + Chat {}

impl MessagingTrait for App {}
