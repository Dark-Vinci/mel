use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::others::messaging::{CreateResponse, UpdateResponse};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "responses", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub channel_id: Uuid,

    #[sea_orm(indexed)]
    pub message_id: Uuid,

    #[sea_orm(type="TEXT")]
    pub content: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateResponse> for Model {
    fn from(msg: CreateResponse) -> Self {
        todo!()
    }
}

impl From<UpdateResponse> for Model {
    fn from(msg: UpdateResponse) -> Self {
        todo!()
    }
}

pub enum Relation{}