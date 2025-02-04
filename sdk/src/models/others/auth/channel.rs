use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannel {}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelUser {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelUser {}
