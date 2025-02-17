use {
    crate::models::others::auth::channel::{
        CreateChannelUser,
    },
    chrono::Utc,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};
use crate::models::others::auth::channel::UpdateChannelUser;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "channel_user", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub user_id: Uuid,

    pub channel_id: Uuid,

    pub invited_by: Option<Uuid>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl From<CreateChannelUser> for ActiveModel {
    fn from(_fro: CreateChannelUser) -> Self {
        let val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}

impl From<UpdateChannelUser> for ActiveModel {
    fn from(_fro: UpdateChannelUser) -> Self {
        let val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    // User,
    Channel,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            // Self::User => {
            //     Entity::belongs_to(super::user::Entity)
            //         .from(Column::UserId)
            //         .to(super::user::Column::Id)
            //         .into()
            // },
            Self::Channel => {
                Entity::belongs_to(super::channel::Entity)
                    .from(Column::ChannelId)
                    .to(super::channel::Column::Id)
                    .into()
            },
        }
    }
}
