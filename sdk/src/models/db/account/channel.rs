use sea_orm::ActiveValue::Set;
use {
    chrono::Utc,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};
use crate::models::others::auth::channel::{UpdateChannel, CreateChannel};
use crate::models::others::auth::create::UpdateUserRequest;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "channel", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    pub description: Option<String>,

    pub created_by: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: chrono::DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CreateChannel> for ActiveModel {
    fn from(fro: CreateChannel) -> Self {
        let mut val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}

impl From<UpdateChannel> for ActiveModel {
    fn from(fro: UpdateChannel) -> Self {
        let mut val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl Related<super::user::Entity> for Entity {
    // The final relation is Channel -> ChannelUser -> User
    fn to() -> RelationDef {
        super::channel_user::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::channel_user::Relation::Channel.def().rev())
    }
}
