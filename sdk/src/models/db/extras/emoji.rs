use {
    crate::models::others::extras::CreateChatMedia,
    chrono::Utc,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "emoji", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    value: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl From<CreateChatMedia> for ActiveModel {
    fn from(_fro: CreateChatMedia) -> Self {
        let val: ActiveModel = Self {
            ..Default::default()
        };

        // todo: fill other fields

        val
    }
}
