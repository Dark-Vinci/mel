use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


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
    pub created_at: DateTime<Utc>
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

struct CreateHistory {}

impl From<CreateHistory> for ActiveModel {
    fn from(_value: CreateHistory) -> Self {
        todo!()
    }
}