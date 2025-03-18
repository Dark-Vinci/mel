use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "search", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub content: String,

    pub user_id: Uuid,

    #[sea_orm(indexed)]
    pub workspace_user_id: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relations {}

impl From<CreateSearch> for ActiveModel {
    fn from(_value: CreateSearch) -> Self {
        todo!()
    }
}


struct CreateSearch {}
