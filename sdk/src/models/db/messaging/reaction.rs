use {
    crate::models::others::messaging::CreateReaction,
    chrono::{DateTime, Utc},
    sea_orm::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "reactions", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub message_id: Uuid,

    #[sea_orm(indexed)]
    pub emoji_id: Uuid,

    pub max_count: u32,

    pub workspace_user_id: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateReaction> for Model {
    fn from(_reaction: CreateReaction) -> Self {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::MessageId",
        to = "super::message::Column::Id"
    )]
    Message,

    #[sea_orm(
        belongs_to = "super::super::extras::emoji::Entity",
        from = "Column::EmojiId",
        to = "super::super::extras::emoji::Column::Id"
    )]
    Emoji,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}