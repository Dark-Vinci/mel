use {
    crate::models::others::extras::CreateHistory,
    chrono::{DateTime, Utc},
    sea_orm::{
        prelude::*, ActiveModelBehavior, DeriveEntityModel, DeriveRelation,
        EnumIter,
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "history", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub user_id: Uuid,

    pub page: String,

    #[sea_orm(indexed)]
    pub workspace_user_id: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

impl From<CreateHistory> for ActiveModel {
    fn from(_value: CreateHistory) -> Self {
        todo!()
    }
}
