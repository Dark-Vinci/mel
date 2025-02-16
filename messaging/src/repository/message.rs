use async_trait::async_trait;
use uuid::Uuid;
use sdk::errors::RepoError;
use crate::connections::db::DB;

struct CreateMessage{}

struct UpdateMessage{}


#[async_trait]
pub trait MessageRepository {
    async fn create(&self, payload: CreateMessage, request_id: Uuid) -> Result<(), RepoError>;
    async fn update(&self, payload: UpdateMessage, request_id: Uuid) -> Result<(), RepoError>;
    async fn delete(&self, id: Uuid, request_id: Uuid) -> Result<(), RepoError>;
    async fn get_by_id(&self, id: Uuid, request_id: Uuid) -> Result<Option<>, RepoError>;
    async fn get_many(&self, channel_id: Vec<String>, request_id: Uuid) -> Result<Vec<Message>, RepoError>;
}

pub struct MessageRepo(DB);

impl MessageRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl MessageRepository for MessageRepo {

}