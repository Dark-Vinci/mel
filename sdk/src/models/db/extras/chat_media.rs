use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Serialize,
    Deserialize,
    DeriveActiveModel,
)]
#[sea_orm(table_name = "chat_media", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    pub message_id: Uuid,

    #[sea_orm(indexed)]
    pub channel_id: Uuid,

    pub workspace_id: Uuid,

    #[sea_orm(type = "TEXT")]
    pub url: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTimeLocal,

    pub deleted_at: Option<DateTimeLocal>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}
