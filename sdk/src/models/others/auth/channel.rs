use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannel {
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannel {
    pub id: Uuid,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelUser {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelUser {}
