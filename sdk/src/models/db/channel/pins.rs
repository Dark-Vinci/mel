use {
    chrono::{DateTime, Utc},
    sea_orm::{entity::prelude::*, DeriveEntityModel},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "pins", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_incremet = false)]
    pub id: Uuid,

    pub message_id: Uuid,

    pub created_by: Uuid,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
