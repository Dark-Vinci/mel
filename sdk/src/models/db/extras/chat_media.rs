use {
    crate::models::others::{
        auth::create::UpdateUserRequest,
        extras::{CreateChatMedia, CreateProfileMedia},
    },
    chrono::Utc,
    sea_orm::{entity::prelude::*, ActiveValue::Set},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

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
    pub channel_id: Uuid, // channel <-> user

    pub workspace_id: Uuid,

    #[sea_orm(type = "TEXT")]
    pub url: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl From<CreateChatMedia> for ActiveModel {
    fn from(fro: UpdateUserRequest) -> Self {
        let mut val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}
