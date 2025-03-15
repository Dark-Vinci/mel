use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
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
pub struct CreatePlatformUserMessage {
    pub is_private_message: bool,
    pub message_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct QueryUserMessagePayload {
    pub platform_id: Uuid,
    pub is_dm: bool,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    platform_user_message:
        crate::models::db::messaging::platform_user_message::Model,
    message: Option<crate::models::db::messaging::message::Model>,
}

impl
    From<(
        crate::models::db::messaging::platform_user_message::Model,
        Option<crate::models::db::messaging::message::Model>,
    )> for UserMessage
{
    fn from(
        payload: (
            crate::models::db::messaging::platform_user_message::Model,
            Option<crate::models::db::messaging::message::Model>,
        ),
    ) -> Self {
        Self {
            platform_user_message: payload.0,
            message: payload.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessages {
    value: Vec<UserMessage>,
}

impl
    From<
        Vec<(
            crate::models::db::messaging::platform_user_message::Model,
            Option<crate::models::db::messaging::message::Model>,
        )>,
    > for UserMessages
{
    fn from(
        payload: Vec<(
            crate::models::db::messaging::platform_user_message::Model,
            Option<crate::models::db::messaging::message::Model>,
        )>,
    ) -> Self {
        let mut result = Self { value: Vec::new() };

        for v in payload {
            result.value.push(v.into())
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct UpdatePlatformUserMessage {
    pub message_id: Uuid,
    pub id: Uuid,
    pub seen: bool,
}

#[derive(Debug, Clone)]
pub struct CreateReaction {
    pub emoji_id: Uuid,
    pub message_id: Uuid,
    pub workspace_user_id: Uuid,
    pub max_count: u32,
}

#[derive(Debug, Clone)]
pub struct CreateResponse {}

#[derive(Debug, Clone)]
pub struct UpdateResponse {}

#[derive(Debug, Clone)]
pub struct CreateChat {}

#[derive(Debug, Clone)]
pub struct UpdateChat {}
