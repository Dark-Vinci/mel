use {
    crate::models::others::extras::CreateChatMedia,
    chrono::Utc,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Clone, Debug, EnumIter, DeriveActiveEnum, PartialEq)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum MediaType {
    #[sea_orm(string_value = "pdf")]
    PDF,
    #[sea_orm(string_value = "jpg")]
    JPG,
    #[sea_orm(string_value = "png")]
    PNG,
    #[sea_orm(string_value = "mp3")]
    MP3,
    #[sea_orm(string_value = "mp4")]
    MP4,
}

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "chat_media", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    pub message_id: Uuid,

    #[sea_orm(indexed)]
    pub medium_id: Uuid, // channel <-> dm(medium)

    pub workspace_id: Uuid,

    #[sea_orm(type = "TEXT")]
    pub url: String,

    pub bucket: String,

    pub key: String,

    pub r#type: MediaType,

    pub file_name: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateChatMedia> for ActiveModel {
    fn from(_fro: CreateChatMedia) -> Self {
        let val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::super::messaging::message::Entity",
        from = "Column::MessageId",
        to = "super::super::messaging::message::Column::Id"
    )]
    Message,
}

impl Related<super::super::messaging::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}
