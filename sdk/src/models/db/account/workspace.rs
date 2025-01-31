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
#[sea_orm(table_name = "workspace", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub created_by: Uuid, // workspace owner

    pub description: Option<String>,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: DateTimeLocal,

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub updated_at: DateTimeLocal,

    pub deleted_at: Option<DateTimeLocal>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, EnumIter, DeriveRelation, Debug, Clone)]
pub enum Relation {}
