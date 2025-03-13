use {
    crate::models::others::messaging::{CreateChat, UpdateChat},
    chrono::{DateTime, Utc},
    sea_orm::prelude::*,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "chats", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub user_a: Uuid,

    #[sea_orm(indexed)]
    pub user_b: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl From<CreateChat> for Model {
    fn from(_reaction: CreateChat) -> Self {
        todo!()
    }
}

impl From<UpdateChat> for Model {
    fn from(_reaction: UpdateChat) -> Self {
        todo!()
    }
}
