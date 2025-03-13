use {
    chrono::{DateTime, Utc},
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct CreateMessage {
    pub workspace_id: Uuid,
    pub channel_id: Uuid,
    pub body: String,
    pub created_by: Uuid,
    pub is_private_message: bool,
    pub activate_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct UpdateMessage {
    pub id: Uuid,
    pub body: Option<String>,
    pub make_main_by: Option<Uuid>,
    pub make_main_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct CreateChannelUserMessage {
    pub is_private_message: bool,
    pub message_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdateChannelUserMessage {
    pub message_id: Uuid,
    pub id: Uuid,
    pub seen: bool,
}

#[derive(Debug, Clone)]
pub struct CreateReaction {}

#[derive(Debug, Clone)]
pub struct CreateResponse {}

#[derive(Debug, Clone)]
pub struct UpdateResponse {}

#[derive(Debug, Clone)]
pub struct CreateChat {}

#[derive(Debug, Clone)]
pub struct UpdateChat {}
