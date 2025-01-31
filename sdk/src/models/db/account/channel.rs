

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
#[sea_orm(table_name = "channel", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub name: String,

    pub description: Option<String>,

    pub created_by: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTimeLocal,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTimeLocal,

    pub deleted_at: Option<DateTimeLocal>,
}

impl ActiveModelBehavior for ActiveModel {}

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
