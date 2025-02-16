use {
    crate::models::others::messaging::{CreateMessage, UpdateMessage},
    chrono::{DateTime, Utc},
    sea_orm::{ActiveModelBehavior, DeriveEntityModel},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "messages", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub channel_id: Uuid,

    #[sea_orm(type = "TEXT")]
    pub content: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateMessage> for Model {
    fn from(msg: CreateMessage) -> Self {
        todo!()
    }
}

impl From<UpdateMessage> for Model {
    fn from(msg: UpdateMessage) -> Self {
        todo!()
    }
}

pub enum Relation {}
