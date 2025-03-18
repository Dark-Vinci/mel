use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "devices", schema_name="public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub user_id: Uuid,

    pub device_type: String,

    pub os_type: String,

    pub os_version: String,

    pub user_agent: String,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}

struct CreateDevice {}

impl From<CreateDevice> for ActiveModel {
    fn from(_value: CreateDevice) -> Self {
        todo!()
    }
}
